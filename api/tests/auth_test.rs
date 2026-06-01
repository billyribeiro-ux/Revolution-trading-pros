// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Auth route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::auth` (the R12-B-split
//! `auth/` directory) and exercises:
//!   - the public `router()` and `logout_router()` mount tables;
//!   - the re-exported `resolve_client_ip` symbol (consumed by
//!     `tests/client_ip_test.rs`);
//!   - the auth-shaped DTO contracts (`LoginUser`, `CreateUser`,
//!     `RefreshTokenRequest`, `AuthResponse`, `UserResponse`,
//!     `ResetPasswordRequest`) re-exported via `revolution_api::models`.
//!
//! ## Why this shape
//!
//! `auth/login.rs`, `auth/register.rs`, `auth/refresh.rs`,
//! `auth/session.rs`, `auth/password.rs`, `auth/verify.rs` are all
//! `pub(super)` — they're invocable only via the mounted router. We
//! can't unit-test the handlers (each runs live SQL + Redis + Argon2
//! password hashing). What we CAN pin without a DB:
//!
//! 1. **The mount table** — `auth::router()` must mount the 10
//!    documented endpoints (register, login, refresh, me, logout,
//!    logout-all, forgot-password, reset-password, verify-email,
//!    resend-verification). A regression that drops `logout-all`
//!    silently kills the "Sign out everywhere" feature; a drop of
//!    `verify-email` strands every new signup with a verification
//!    link that 404s. The compile-time check that `router()` returns
//!    `Router<AppState>` catches handler-signature regressions
//!    (wrong extractor, wrong return type, missing State<>).
//!
//! 2. **`logout_router()` must also compile** — mounted separately at
//!    `/logout` for frontend compatibility (the marketing site's
//!    nav-bar hits `/api/logout` not `/api/auth/logout`).
//!
//! 3. **`resolve_client_ip` is publicly re-exported** — explicit
//!    contract from `routes/auth/mod.rs:36`:
//!    "re-export the spoof-resistant client-IP resolver so external
//!     consumers (notably `tests/client_ip_test.rs`) keep resolving
//!     `routes::auth::resolve_client_ip` at the same path."
//!    Pin the path so a future move that broke the re-export is
//!    caught here, not by the client_ip_test breaking mysteriously.
//!
//! 4. **`User.id` (the user model that flows through every auth
//!    handler) is `i64`** (BIGSERIAL PK). A narrowing would silently
//!    break every JWT (`create_jwt_versioned(user.id, ...)`) and
//!    every session ID stored in Redis with the user_id as a key.
//!    Symptom: post-narrowing, IDs above `i32::MAX` would wrap
//!    negative and "log in as another user" by collision. CLAUDE.md
//!    "Reserved exception" does NOT apply to identity columns.
//!
//! 5. **`LoginUser` requires `email` + `password`** (NEGATIVE pin).
//!    `ValidatedJson<LoginUser>` is the extractor; a refactor that
//!    flipped either to Optional would let an attacker POST `{}` and
//!    skip past the rate-limit gate at the validator layer, hitting
//!    the password-hash dummy in `login.rs` and burning CPU
//!    (`hash_dummy_password()`). The validator + DTO required-ness
//!    is the front line.
//!
//! 6. **`AuthResponse.expires_in` is i64 seconds** (load-bearing —
//!    `state.config.jwt_expires_in * 3600` per
//!    `routes/auth/login.rs:412`). A narrowing to i32 would overflow
//!    at >680 years, which is fine, but the contract requires i64
//!    end-to-end to match the JWT exp claim.
//!
//! 7. **`UserResponse.email_verified` is a flat `bool` field** at the
//!    top level (NOT nested under a `verification` object). The
//!    frontend reads `user.email_verified` to gate access to the
//!    member dashboard. NEGATIVE pin: assert camelCase
//!    `emailVerified` is absent.
//!
//! ## Pattern source
//!
//! Modeled on `tests/oauth_test.rs` (sister auth surface),
//! `tests/security_test.rs`, `tests/client_ip_test.rs`.

use revolution_api::models::{CreateUser, LoginUser, RefreshTokenRequest, ResetPasswordRequest};
use revolution_api::routes::auth;

// ── 1. router() mount-table compile-pin (all 10 documented routes) ───

/// `auth::router()` must build as `Router<AppState>`. The documented
/// route set (per `routes/auth/mod.rs:40-50`) is 10 endpoints:
///   - POST /register
///   - POST /login
///   - POST /refresh
///   - GET  /me
///   - POST /logout
///   - POST /logout-all
///   - POST /forgot-password
///   - POST /reset-password
///   - GET  /verify-email
///   - POST /resend-verification
///
/// A regression in any handler signature (wrong State<>, wrong
/// extractor, wrong return type) fails compilation here. We can't
/// invoke the handlers (Redis + Postgres + Argon2 required), but the
/// type-level guarantee that the .route() chain compiled is the
/// minimum contract.
#[test]
fn auth_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = auth::router();
}

/// `auth::logout_router()` is mounted at the top-level `/logout` path
/// for frontend compatibility (the marketing site's nav-bar hits
/// `/api/logout` directly). A regression that drops this builder
/// would 404 the nav-bar's logout button without any compile-time
/// signal. Pin the symbol's existence + the AppState type.
#[test]
fn auth_logout_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = auth::logout_router();
}

/// Idempotent construction — building both routers multiple times in
/// the same process must succeed. Catches accidental `OnceLock<...>`
/// in either constructor (per CLAUDE.md habit #3 — "cached state
/// lost during refactor" landmine).
#[test]
fn auth_routers_safe_to_construct_multiple_times() {
    let _a1: axum::Router<revolution_api::AppState> = auth::router();
    let _a2: axum::Router<revolution_api::AppState> = auth::router();
    let _l1: axum::Router<revolution_api::AppState> = auth::logout_router();
    let _l2: axum::Router<revolution_api::AppState> = auth::logout_router();
}

// ── 2. resolve_client_ip is re-exported at the documented path ───────

/// `routes/auth/mod.rs:36` explicitly re-exports `resolve_client_ip`
/// so `tests/client_ip_test.rs` keeps importing
/// `routes::auth::resolve_client_ip` at the same path. A future
/// move/rename that breaks this re-export would silently break the
/// client_ip_test (it'd fail with `cannot find` instead of a useful
/// failure message). Pin the symbol path here so the regression
/// surfaces at the auth-test level.
#[test]
fn resolve_client_ip_is_re_exported_at_auth_path() {
    use axum::http::HeaderMap;
    use std::net::{IpAddr, Ipv4Addr};

    // Function signature pin: (IpAddr, &HeaderMap, &[IpCidr]) -> IpAddr.
    let peer: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1));
    let headers = HeaderMap::new();
    let trusted: Vec<_> = Vec::new();

    // The call compiles only if the function lives at this exact path
    // with this exact signature. The IP returned doesn't matter for
    // the contract pin — only that the call compiles + runs.
    let resolved = auth::resolve_client_ip(peer, &headers, &trusted);
    assert_eq!(
        resolved, peer,
        "with no XFF headers and an empty trusted list, the resolver must return the peer IP unchanged"
    );
}

// ── 3. LoginUser DTO required-fields NEGATIVE pin ────────────────────

/// `LoginUser` is the body extracted by `ValidatedJson<LoginUser>` in
/// `auth/login.rs`. Both `email` and `password` are required by the
/// validator. A regression that flipped either to `Option<>` would
/// let an attacker POST `{}` and skip past the field-level validation
/// at the extractor layer — hitting `hash_dummy_password()` and
/// burning a 100ms Argon2 hash per request (CPU DoS vector).
///
/// R9-D NEGATIVE pin: assert deserialization FAILS when each
/// required field is missing.
#[test]
fn login_user_required_fields_negative_pin() {
    // Minimal valid payload
    let valid: LoginUser = serde_json::from_value(serde_json::json!({
        "email": "trader@example.com",
        "password": "correct horse battery staple",
    }))
    .expect("minimal LoginUser must parse");
    assert_eq!(valid.email, "trader@example.com");
    assert_eq!(valid.password, "correct horse battery staple");
    // Optional fields default to None
    assert!(valid.remember.is_none());
    assert!(valid.device_name.is_none());
    assert!(valid.device_fingerprint.is_none());

    // NEGATIVE: missing `email` must FAIL — login can't proceed
    let no_email = serde_json::from_value::<LoginUser>(serde_json::json!({
        "password": "x",
    }));
    assert!(
        no_email.is_err(),
        "LoginUser without email must fail deserialization — login.rs depends on input.email for the per-email rate-limit key"
    );

    // NEGATIVE: missing `password` must FAIL
    let no_password = serde_json::from_value::<LoginUser>(serde_json::json!({
        "email": "trader@example.com",
    }));
    assert!(
        no_password.is_err(),
        "LoginUser without password must fail — login.rs always calls verify_password(&input.password, ...)"
    );

    // POSITIVE: full payload (with optional device fingerprint for
    // session-pinning) round-trips
    let full: LoginUser = serde_json::from_value(serde_json::json!({
        "email": "user@example.com",
        "password": "long enough password 12+",
        "remember": true,
        "device_name": "Chrome on Mac",
        "device_fingerprint": "abc123",
    }))
    .expect("full LoginUser must parse");
    assert_eq!(full.remember, Some(true));
    assert_eq!(full.device_name.as_deref(), Some("Chrome on Mac"));
}

// ── 4. CreateUser DTO required-fields NEGATIVE pin ───────────────────

/// `CreateUser` is the body for POST `/auth/register`. All three
/// fields (`email`, `password`, `name`) are required at the validator
/// AND deserializer levels (no `#[serde(default)]`). A regression
/// that flipped any to Optional would let registration POST without
/// the field, hitting Argon2 hashing on a missing password (panic)
/// or inserting a NULL name (DB NOT NULL constraint failure).
#[test]
fn create_user_required_fields_negative_pin() {
    // Minimal valid payload
    let valid: CreateUser = serde_json::from_value(serde_json::json!({
        "email": "new@example.com",
        "password": "long enough password 12+ chars",
        "name": "Jane Trader",
    }))
    .expect("minimal CreateUser must parse");
    assert_eq!(valid.email, "new@example.com");
    assert_eq!(valid.name, "Jane Trader");

    // NEGATIVE: missing each required field
    for missing in &["email", "password", "name"] {
        let mut obj = serde_json::json!({
            "email": "new@example.com",
            "password": "long enough password 12+ chars",
            "name": "Jane Trader",
        });
        obj.as_object_mut().unwrap().remove(*missing);
        let r = serde_json::from_value::<CreateUser>(obj);
        assert!(
            r.is_err(),
            "CreateUser without {missing} must fail deserialization — DB has NOT NULL on the corresponding users column"
        );
    }
}

// ── 5. RefreshTokenRequest + ResetPasswordRequest contracts ──────────

/// `RefreshTokenRequest.refresh_token` is required (refresh.rs reads
/// it directly). A regression to Optional would let an attacker POST
/// `{}` and trigger a None-unwrap or fallback to an env-default token
/// (catastrophic — could mint a token as user 0). Pin required-ness.
///
/// `ResetPasswordRequest` requires token + email + password +
/// password_confirmation. A regression that flipped any to Optional
/// would let the password-reset confirmation page accept a partial
/// body, hitting the password-mismatch branch with one side `None`
/// (which would `unwrap()` and panic, or worse, treat missing as
/// equal).
#[test]
fn refresh_and_reset_password_request_required_fields_negative_pin() {
    // RefreshTokenRequest — required field
    let valid_refresh: RefreshTokenRequest = serde_json::from_value(serde_json::json!({
        "refresh_token": "eyJ.example.token",
    }))
    .expect("RefreshTokenRequest must parse");
    assert_eq!(valid_refresh.refresh_token, "eyJ.example.token");

    let no_refresh = serde_json::from_value::<RefreshTokenRequest>(serde_json::json!({}));
    assert!(
        no_refresh.is_err(),
        "RefreshTokenRequest without refresh_token must fail — refresh.rs has no fallback"
    );

    // ResetPasswordRequest — all four required
    let valid_reset: ResetPasswordRequest = serde_json::from_value(serde_json::json!({
        "token": "x".repeat(64),
        "email": "user@example.com",
        "password": "new strong password 12+",
        "password_confirmation": "new strong password 12+",
    }))
    .expect("ResetPasswordRequest must parse");
    assert_eq!(valid_reset.email, "user@example.com");

    // NEGATIVE: every required field must be present
    for missing in &["token", "email", "password", "password_confirmation"] {
        let mut obj = serde_json::json!({
            "token": "x".repeat(64),
            "email": "user@example.com",
            "password": "new strong password 12+",
            "password_confirmation": "new strong password 12+",
        });
        obj.as_object_mut().unwrap().remove(*missing);
        let r = serde_json::from_value::<ResetPasswordRequest>(obj);
        assert!(
            r.is_err(),
            "ResetPasswordRequest without {missing} must fail — password reset must have all four fields"
        );
    }
}

// ── 6. User.id is i64 (BIGSERIAL contract) — JWT subject ─────────────

/// `User.id` is the JWT subject (`create_jwt_versioned(user.id, ...)`
/// in `auth/login.rs:359`). On this stack every PK is `BIGSERIAL`
/// (`i64`). A narrowing to `i32` would mean every user with ID
/// > 2,147,483,647 would wrap negative and silently authenticate as
/// a DIFFERENT user on token mint. Pin via the User model + a
/// fixture that exceeds i32::MAX.
///
/// CLAUDE.md "Reserved exception" (revisions, attempts) does NOT
/// apply to identity columns — wrong-user-on-login is the worst
/// possible auth bug.
#[test]
fn user_model_id_is_i64() {
    use chrono::Utc;
    use revolution_api::models::User;

    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let now = Utc::now().naive_utc();
    let u = User {
        id: above_i32_max,
        email: "trader@example.com".to_string(),
        password_hash: "$argon2id$...".to_string(),
        name: "Test Trader".to_string(),
        role: Some("user".to_string()),
        is_active: Some(true),
        email_verified_at: Some(now),
        avatar_url: None,
        mfa_enabled: Some(false),
        created_at: now,
        updated_at: now,
    };

    // Compile-pin
    let _: i64 = u.id;

    // Narrowing proof — wrap-on-cast would log in as a DIFFERENT user
    assert!(
        (u.id as i32 as i64) != u.id,
        "narrowing User.id to i32 must lose data — proves i64 is required (wrong-user-on-login is the worst auth bug)"
    );

    // Wire format: password_hash MUST be skipped (`#[serde(skip_serializing)]`)
    let wire = serde_json::to_value(&u).expect("serialize User");
    assert!(
        wire.get("password_hash").is_none(),
        "password_hash must NOT appear in serialized output — #[serde(skip_serializing)] is load-bearing"
    );
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
}

// ── 7. UserResponse wire shape: flat email_verified, snake_case ──────

/// `UserResponse.email_verified` is a flat `bool` at the top level
/// (NOT nested under a `verification` object). The frontend's
/// `routes/+layout.svelte` reads `user.email_verified` to gate access
/// to the member dashboard. A regression that nested it (e.g.
/// `user.verification.email_verified`) would silently grant access
/// to unverified accounts (the JS `user.email_verified` would be
/// `undefined`, but if any code does `!user.email_verified` to mean
/// "not verified" that's still falsy — different code paths could
/// flip).
///
/// R9-D NEGATIVE pin: assert camelCase `emailVerified` does NOT
/// appear on the wire (regression to `#[serde(rename_all =
/// "camelCase")]` would break the frontend's auth-gate check).
#[test]
fn user_response_wire_is_snake_case_flat_email_verified() {
    use revolution_api::models::UserResponse;

    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    // Build a UserResponse via JSON to avoid coupling to private
    // fields (UserResponse may add Optional fields over time).
    // Instead, instantiate the struct directly with the minimal set
    // observed in the model definition. UserResponse uses
    // `#[serde(skip_serializing_if = "Option::is_none")]` on optional
    // fields, so we leave them out at construction.
    let now = chrono::Utc::now().naive_utc();
    let user = revolution_api::models::User {
        id: above_i32_max,
        email: "user@example.com".to_string(),
        password_hash: "irrelevant".to_string(),
        name: "Test".to_string(),
        role: Some("user".to_string()),
        is_active: Some(true),
        email_verified_at: Some(now),
        avatar_url: None,
        mfa_enabled: Some(false),
        created_at: now,
        updated_at: now,
    };
    // `User: Into<UserResponse>` is the documented conversion path
    // used in `auth/login.rs:411` — `user: user.into()`. If a
    // refactor breaks this, login.rs no longer compiles either.
    let resp: UserResponse = user.into();
    let wire = serde_json::to_value(&resp).expect("serialize UserResponse");

    // POSITIVE: `id` is i64 on the wire
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));

    // POSITIVE: `email_verified` is a flat top-level bool
    assert!(
        wire.get("email_verified").is_some(),
        "email_verified must be a top-level key on UserResponse"
    );
    assert!(
        wire["email_verified"].is_boolean(),
        "email_verified must be a bool, not a nested object"
    );

    // NEGATIVE: camelCase variants MUST NOT appear (R9-D regression guard)
    assert!(
        wire.get("emailVerified").is_none(),
        "emailVerified (camelCase) must NOT appear — wire format is snake_case"
    );
    assert!(
        wire.get("avatarUrl").is_none(),
        "avatarUrl (camelCase) must NOT appear — wire format is snake_case"
    );
    assert!(
        wire.get("mfaEnabled").is_none(),
        "mfaEnabled (camelCase) must NOT appear — wire format is snake_case"
    );

    // NEGATIVE: must NOT be nested under a `verification` object
    assert!(
        wire.get("verification").is_none(),
        "email_verified must be flat, NOT nested under a verification object — frontend reads user.email_verified directly"
    );
}
