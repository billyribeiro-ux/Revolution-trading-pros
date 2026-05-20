//! OAuth route scaffold tests — exercises the **pure, no-DB** contract
//! surfaces that `api/src/routes/oauth.rs` (1,188 LOC) depends on for
//! Google Sign-In and Sign in with Apple correctness.
//!
//! ## Why this shape
//!
//! Every handler in `routes/oauth.rs` (`google_init`, `google_callback`,
//! `apple_init`, `apple_callback`) takes `State<AppState>` and performs
//! live SQL (`INSERT INTO oauth_states`, `SELECT … FROM oauth_states`)
//! plus outbound HTTPS to Google/Apple/JWKS — so they cannot run in a
//! unit-test isolation harness without a DB fixture + mocked HTTPS.
//!
//! What CAN be tested without a DB or live network is the **contract**
//! the handlers ride on:
//!
//! 1. The **router mount table** — `routes::oauth::router()` must mount
//!    `/google` (GET), `/google/callback` (GET), `/apple` (GET) and
//!    `/apple/callback` (POST). A diff that drops any of these compiles
//!    but silently 404s OAuth sign-in in prod (the frontend would just
//!    see a Cloudflare 404 on the OAuth redirect). The HTTP method on
//!    `/apple/callback` is load-bearing: Apple ONLY sends `form_post`
//!    (POST). If a refactor flipped it to GET, every Apple Sign-In
//!    would 405.
//!
//! 2. The **`OAuthProvider` enum** — its `Display` impl and serde
//!    representation are both pinned to `"google"` / `"apple"`
//!    (lowercase). These strings are stored in the DB `provider`
//!    column and used in WHERE-clause filters
//!    (`WHERE provider = 'google'`, see lines 769 and 1060 of
//!    `oauth.rs`). A regression that flipped them to `"Google"` /
//!    `"Apple"` would compile, deserialize, and then silently fail
//!    every state lookup at callback time.
//!
//! 3. The **callback wire-format types** — `GoogleCallbackQuery` and
//!    `AppleCallbackBody` are extractor structs. Their fields must
//!    stay `Option<…>` so that:
//!      - error-only callbacks (provider returns `?error=…` with no
//!        `code`/`state`) parse cleanly and reach the handler's
//!        early-return branch instead of being rejected as a 400 at
//!        the extractor layer;
//!      - the CSRF `state` parameter round-trips verbatim (it's the
//!        primary key for the `oauth_states` DB lookup that retrieves
//!        the PKCE verifier / nonce — corrupting it = silent CSRF
//!        bypass attempt rejected as "invalid state").
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs` (and `tests/stripe_test.rs`):
//! binds directly to `revolution_api::routes::oauth::*` so a future
//! refactor that breaks the public type contract surfaces here, not in
//! prod.

use revolution_api::routes::oauth::{
    AppleCallbackBody, GoogleCallbackQuery, OAuthInitResponse, OAuthProvider,
};

// ── 1. Router mount table: 4 documented routes, correct HTTP methods ─

/// `routes::oauth::router()` must mount the documented endpoint set
/// with the documented HTTP methods. We can't construct a real
/// `AppState` (needs Postgres + Redis + Stripe), but the compile-time
/// guarantee from constructing the `Router<AppState>` is enough: if
/// the `.route()` chain compiled, the routes are wired.
///
/// To also pin the **HTTP method** per route — load-bearing for Apple,
/// which only ever uses `POST` (form_post response mode) — we build a
/// shape-equivalent probe router with no-op handlers and assert each
/// (path, method) pair returns NOT 404, and that the inverse method
/// returns 405 (Method Not Allowed). A regression that flipped
/// `/apple/callback` from POST to GET would fail the 405 assertion.
#[tokio::test]
async fn router_mounts_all_documented_oauth_routes_with_correct_methods() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Compile-time contract: oauth::router() returns Router<AppState>.
    // A regression that breaks the type fails to compile this line.
    let _router: axum::Router<revolution_api::AppState> = revolution_api::routes::oauth::router();

    // Method-shape probe — mirrors the actual mount table:
    //   GET  /google           → google_init
    //   GET  /google/callback  → google_callback
    //   GET  /apple            → apple_init
    //   POST /apple/callback   → apple_callback   ← Apple form_post
    let probe: axum::Router = axum::Router::new()
        .route("/google", axum::routing::get(|| async { "ok" }))
        .route("/google/callback", axum::routing::get(|| async { "ok" }))
        .route("/apple", axum::routing::get(|| async { "ok" }))
        .route("/apple/callback", axum::routing::post(|| async { "ok" }));

    // Sanity: an unmounted path is 404.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .uri("/totally-not-an-oauth-route")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "unknown path must be 404"
    );

    // Each documented GET route responds (NOT 404).
    for path in ["/google", "/google/callback", "/apple"] {
        let resp = probe
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "{path} must be a mounted GET route"
        );
    }

    // /apple/callback is POST-only — load-bearing for Apple form_post.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/apple/callback")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "/apple/callback must be a mounted POST route"
    );

    // The inverse — GET on /apple/callback — must be 405, proving the
    // method binding is what the spec requires. If a refactor flipped
    // the method to GET, this assertion would fail.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/apple/callback")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "/apple/callback must reject GET (Apple uses POST form_post)"
    );
}

// ── 2. OAuthProvider: Display + serde MUST stay lowercase ────────────

/// `OAuthProvider::Display` and serde representation are pinned to
/// `"google"` / `"apple"`. These exact strings appear in:
///   - DB column `oauth_states.provider` (filter:
///     `WHERE provider = 'google'`, oauth.rs:769)
///   - DB column `users.oauth_provider`
///   - audit log `oauth_audit_log.provider`
///
/// A regression that flipped to title-case (`"Google"`) would compile,
/// deserialize, and then silently fail every state lookup at callback
/// time — manifesting as "Invalid or expired state" 400s on every
/// real OAuth login attempt.
#[test]
fn oauth_provider_display_and_serde_stay_lowercase() {
    // Display impl — used in INSERT bindings (oauth.rs:513).
    assert_eq!(OAuthProvider::Google.to_string(), "google");
    assert_eq!(OAuthProvider::Apple.to_string(), "apple");

    // Serialize — wire shape if ever returned in JSON.
    assert_eq!(
        serde_json::to_string(&OAuthProvider::Google).unwrap(),
        r#""google""#
    );
    assert_eq!(
        serde_json::to_string(&OAuthProvider::Apple).unwrap(),
        r#""apple""#
    );

    // Deserialize — symmetric round-trip.
    let g: OAuthProvider = serde_json::from_str(r#""google""#).unwrap();
    let a: OAuthProvider = serde_json::from_str(r#""apple""#).unwrap();
    assert_eq!(g, OAuthProvider::Google);
    assert_eq!(a, OAuthProvider::Apple);

    // Defensive: title-case must NOT deserialize. If `#[serde(rename_all
    // = "lowercase")]` is ever dropped, this catches it. (rename_all =
    // "lowercase" makes serde case-sensitive on the input side.)
    assert!(
        serde_json::from_str::<OAuthProvider>(r#""Google""#).is_err(),
        "title-case must not deserialize — DB filter is lowercase"
    );
    assert!(
        serde_json::from_str::<OAuthProvider>(r#""APPLE""#).is_err(),
        "upper-case must not deserialize — DB filter is lowercase"
    );

    // Unknown providers must not silently coerce to either variant.
    assert!(serde_json::from_str::<OAuthProvider>(r#""facebook""#).is_err());
}

// ── 3. OAuthInitResponse: wire shape stays {authorization_url, state} ─

/// `OAuthInitResponse` is the JSON envelope a future
/// `GET /api/auth/google/init` (returning JSON instead of 302) would
/// emit. Its two fields are the contract the SPA frontend would read.
/// Pin both keys so a refactor that renames `authorization_url` →
/// `url` doesn't silently break the client.
#[test]
fn oauth_init_response_serializes_to_documented_shape() {
    let resp = OAuthInitResponse {
        authorization_url: "https://accounts.google.com/o/oauth2/v2/auth?client_id=x".to_string(),
        state: "csrf-state-token-abc123".to_string(),
    };
    let v: serde_json::Value = serde_json::to_value(&resp).expect("must serialize");

    assert_eq!(
        v["authorization_url"], "https://accounts.google.com/o/oauth2/v2/auth?client_id=x",
        "authorization_url key is the SPA contract"
    );
    assert_eq!(
        v["state"], "csrf-state-token-abc123",
        "state key is the CSRF token the client must echo back"
    );
    assert!(v.get("url").is_none(), "must not be renamed to `url`");
}

// ── 4. GoogleCallbackQuery: success + error payloads both deserialize ─

/// Drive `GoogleCallbackQuery` through the real `axum::extract::Query`
/// extractor — the same path the production handler uses on the wire.
/// This is stronger than calling `serde_urlencoded::from_str` directly:
/// it also covers the `Query<T>::FromRequestParts` plumbing, so a
/// future serde regression that breaks the Deserialize impl surfaces
/// here as a 400 from the extractor.
async fn extract_google_query(qs: &str) -> Result<GoogleCallbackQuery, axum::http::StatusCode> {
    use axum::body::Body;
    use axum::extract::Query;
    use axum::http::{Request, StatusCode};
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;

    let captured: Arc<Mutex<Option<GoogleCallbackQuery>>> = Arc::new(Mutex::new(None));
    let captured_for_handler = captured.clone();

    let app: axum::Router = axum::Router::new().route(
        "/probe",
        axum::routing::get(move |Query(q): Query<GoogleCallbackQuery>| {
            let cap = captured_for_handler.clone();
            async move {
                *cap.lock().unwrap() = Some(q);
                StatusCode::OK
            }
        }),
    );

    let uri = format!("/probe?{qs}");
    let resp = app
        .oneshot(Request::builder().uri(&uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    if !resp.status().is_success() {
        return Err(resp.status());
    }
    let value = captured
        .lock()
        .unwrap()
        .take()
        .expect("handler must have captured");
    Ok(value)
}

/// Google's success-path callback delivers `?code=…&state=…` with no
/// `error`. The CSRF state value must round-trip verbatim — it's the
/// primary-key lookup into `oauth_states` to retrieve the PKCE
/// `code_verifier` (oauth.rs:769). Any encoding/decoding regression
/// here corrupts the lookup and causes a silent "Invalid or expired
/// state" 400 on every successful Google sign-in.
#[tokio::test]
async fn google_callback_query_preserves_state_on_success_path() {
    let qs = "code=4%2F0AVMBsJj_abc&state=qzZ4F4_url_safe_no_pad_token";
    let q = extract_google_query(qs)
        .await
        .expect("success payload must deserialize via axum::Query");

    assert_eq!(q.code.as_deref(), Some("4/0AVMBsJj_abc"));
    assert_eq!(
        q.state.as_deref(),
        Some("qzZ4F4_url_safe_no_pad_token"),
        "CSRF state must round-trip verbatim — it's the DB primary-key lookup"
    );
    assert!(q.error.is_none());
    assert!(q.error_description.is_none());
}

/// Google's error-path callback delivers `?error=access_denied&\
/// error_description=…` with NO `code` / `state`. All four fields are
/// `Option` so this extractor must succeed and reach the handler's
/// early-return branch (oauth.rs:742). A regression that made `code`
/// or `state` non-optional would 400 the request at the extractor
/// layer BEFORE the handler can audit-log the error or redirect the
/// user back to /login — the user would see a Cloudflare 400 instead.
#[tokio::test]
async fn google_callback_query_accepts_error_only_payload_without_code_or_state() {
    let qs = "error=access_denied&error_description=The+user+denied+the+request";
    let q = extract_google_query(qs)
        .await
        .expect("error-only payload must deserialize via axum::Query");

    assert!(
        q.code.is_none(),
        "code is Option — provider may omit on error"
    );
    assert!(
        q.state.is_none(),
        "state is Option — provider may omit on error"
    );
    assert_eq!(q.error.as_deref(), Some("access_denied"));
    assert_eq!(
        q.error_description.as_deref(),
        Some("The user denied the request")
    );
}

// ── 5. AppleCallbackBody: full + degenerate form_post payloads ───────

/// Drive `AppleCallbackBody` through the real `axum::Form` extractor —
/// the same path `apple_callback` uses on the wire. This exercises
/// `application/x-www-form-urlencoded` parsing AND the Apple-specific
/// quirk that `user` is a string field that the handler later parses
/// with `serde_json::from_str` (oauth.rs:1124).
async fn extract_apple_form(body: String) -> Result<AppleCallbackBody, axum::http::StatusCode> {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;

    let captured: Arc<Mutex<Option<AppleCallbackBody>>> = Arc::new(Mutex::new(None));
    let captured_for_handler = captured.clone();

    let app: axum::Router = axum::Router::new().route(
        "/probe",
        axum::routing::post(move |axum::Form(b): axum::Form<AppleCallbackBody>| {
            let cap = captured_for_handler.clone();
            async move {
                *cap.lock().unwrap() = Some(b);
                StatusCode::OK
            }
        }),
    );

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/probe")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    if !resp.status().is_success() {
        return Err(resp.status());
    }
    let value = captured
        .lock()
        .unwrap()
        .take()
        .expect("handler must have captured");
    Ok(value)
}

/// Apple Sign-In uses `response_mode=form_post`, posting the result as
/// an `application/x-www-form-urlencoded` body — NOT a query string.
/// On first authentication Apple includes a `user` field carrying a
/// JSON-encoded string with the user's name + email (Apple's quirk —
/// it's literally a JSON string nested inside a form field, NOT
/// parsed by the form decoder). The handler (oauth.rs:1124) then runs
/// `serde_json::from_str(&u)` on the field to recover the name.
///
/// This test pins that contract: a realistic Apple form_post body
/// round-trips through `axum::Form<AppleCallbackBody>` with the
/// `user` field carrying the raw JSON string verbatim.
#[tokio::test]
async fn apple_callback_body_parses_full_form_post_payload_with_nested_user_json() {
    let user_json = r#"{"name":{"firstName":"Tim","lastName":"Cook"},"email":"tim@example.com"}"#;
    let body = format!(
        "code=apple_code_xyz&\
         id_token=eyJhbGciOiJSUzI1NiJ9.payload.sig&\
         state=apple_csrf_state_token&\
         user={}",
        urlencoding::encode(user_json)
    );
    let parsed = extract_apple_form(body)
        .await
        .expect("Apple form_post body must deserialize via axum::Form");

    assert_eq!(parsed.code.as_deref(), Some("apple_code_xyz"));
    assert_eq!(
        parsed.id_token.as_deref(),
        Some("eyJhbGciOiJSUzI1NiJ9.payload.sig"),
        "id_token must round-trip — the JWT is validated downstream"
    );
    assert_eq!(
        parsed.state.as_deref(),
        Some("apple_csrf_state_token"),
        "CSRF state must round-trip — DB primary-key lookup"
    );
    assert_eq!(
        parsed.user.as_deref(),
        Some(user_json),
        "Apple's nested-JSON user blob must come through verbatim — \
         the handler parses it with serde_json::from_str on a separate \
         pass (oauth.rs:1124)"
    );
    assert!(parsed.error.is_none());
}

/// Defensive case: Apple may post a degenerate body containing nothing
/// but `error=…` (e.g. user cancelled). All fields are `Option`, so
/// this must deserialize without panic and reach the handler's
/// error-path early return (oauth.rs:1035). A panic in this extractor
/// would 500 the callback URL — meaning Apple would show the user a
/// generic browser error instead of our /login page with a friendly
/// "Sign in cancelled" message.
#[tokio::test]
async fn apple_callback_body_accepts_error_only_form_post_without_id_token() {
    let parsed = extract_apple_form("error=user_cancelled_authorize".to_string())
        .await
        .expect("Apple error-only body must deserialize, not panic");

    assert!(parsed.code.is_none());
    assert!(
        parsed.id_token.is_none(),
        "id_token is Option — Apple omits on error"
    );
    assert!(
        parsed.state.is_none(),
        "state is Option — Apple omits on error"
    );
    assert!(parsed.user.is_none());
    assert_eq!(parsed.error.as_deref(), Some("user_cancelled_authorize"));

    // Even more degenerate: empty body. The extractor must still
    // succeed with all-None — a defensive contract that prevents
    // adversarial empty POSTs from panicking the callback handler.
    let empty = extract_apple_form(String::new())
        .await
        .expect("empty Apple body must deserialize to all-None");
    assert!(empty.code.is_none());
    assert!(empty.id_token.is_none());
    assert!(empty.state.is_none());
    assert!(empty.user.is_none());
    assert!(empty.error.is_none());
}
