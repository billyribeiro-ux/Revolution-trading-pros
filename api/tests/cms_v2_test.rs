//! CMS v2 route scaffold tests — exercises the **pure, no-DB** contract
//! surfaces that `api/src/routes/cms_v2.rs` (904 LOC) depends on for
//! content / asset / navigation correctness.
//!
//! ## Why this shape
//!
//! Every handler in `routes/cms_v2.rs` takes `State<AppState>` (live
//! Postgres pool against the `cms_content`, `cms_assets`,
//! `cms_navigation_menus`, etc. tables) plus the `User` extractor —
//! none can be invoked in unit-test isolation without a DB fixture.
//!
//! Even more aggressively, the module deliberately keeps *every*
//! handler, every internal helper (`api_error`, `require_cms_admin`,
//! `require_cms_editor`, `parse_content_type`), and every internal
//! query struct (`DeleteFolderQuery`, `ContentBySlugQuery`,
//! `RevisionListQuery`) **private**. Only `admin_router()` and
//! `public_router()` are `pub`. That's intentional: the wire shape is
//! the contract, not the helpers.
//!
//! So what we can pin without a DB is the **router contract**:
//!
//! 1. The `admin_router()` mount table — 22 mounted paths covering
//!    stats, folders, assets, content, revisions, tags, comments,
//!    settings, menus, redirects. Each (path, method) pair is the
//!    surface the SPA admin client speaks to. A diff that drops
//!    `/content/:id/status` (POST → publish/archive/etc.) silently
//!    breaks the workflow-transition button with no compile error.
//!
//! 2. The `public_router()` mount table — 4 unauthenticated public
//!    paths (`/content/:content_type/:slug`, `/settings`, `/menus`,
//!    `/menus/:slug`). This is what serves the **public website**:
//!    every blog post, alert-service page, FAQ, etc., is fetched
//!    through `/content/:content_type/:slug`. A regression that
//!    dropped that route would 404 the entire public site without a
//!    Rust error.
//!
//! 3. The HTTP method binding per path — `content/:id` accepts
//!    GET (read), PUT (update), DELETE (delete-soft) but NOT POST.
//!    `/content/:id/status` accepts POST only (state-mutating).
//!    `/redirects/:id` accepts DELETE only. Flipping any of these
//!    methods would be a silent breakage detected only by manual QA.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs` (R2-02) and `tests/orders_test.rs`
//! / `tests/oauth_test.rs` (R3-A/B): bind directly to
//! `revolution_api::routes::cms_v2::*` so a future refactor that breaks
//! the public router contract surfaces here, not in prod.
//!
//! ## Money contract
//!
//! cms_v2 has **zero `*_cents` fields** at this layer — content,
//! folders, tags, assets, comments, menus and redirects are all money-
//! free domain types. The CLAUDE.md i64-only money rule has no surface
//! to test here. Documented for the auditor's sake.

mod common;

// ── 1. admin_router(): compiles to Router<AppState> and is mountable ─

/// `routes::cms_v2::admin_router()` must return a `Router<AppState>`
/// that the top-level api router can `.nest("/admin/cms-v2", _)` onto.
/// A regression in any handler signature (extractor mismatch, missing
/// `AppState`, wrong return type) would fail to compile this test.
///
/// This is the cheapest possible regression guard — it costs nothing
/// to compile, runs in microseconds, and catches every signature drift
/// across all 22 admin endpoints without needing a live DB.
#[test]
fn admin_router_compiles_with_expected_state_binding() {
    // Construct the real router. If any handler signature regressed
    // (e.g. someone added a State<OtherType> by mistake, or returned
    // an incompatible Result type), this expression fails to type-
    // check at compile time.
    let _router: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2::admin_router();
}

// ── 2. public_router(): compiles to Router<AppState> ─────────────────

/// `routes::cms_v2::public_router()` serves the unauthenticated public
/// site. Same compile-time guarantee as admin_router(): if any handler
/// signature drifts, this won't type-check.
///
/// Critically, the public router intentionally does NOT take a `User`
/// extractor on its handlers — `get_content_by_slug`,
/// `get_site_settings`, `list_navigation_menus`, `get_navigation_menu`
/// are all anonymous. A refactor that accidentally added `User` to one
/// of them would compile here (the State binding stays the same) but
/// fail at runtime with 401 because no Authorization header is sent
/// to public routes. The integration-test layer catches that runtime
/// shape; this test guards the type-level binding.
#[test]
fn public_router_compiles_with_expected_state_binding() {
    let _router: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2::public_router();
}

// ── 3. admin_router(): documented mount-table is complete ────────────

/// `admin_router()` mounts the documented endpoint set. We can't drive
/// the real router because every handler needs a live Postgres pool +
/// the `User` extractor, but we CAN build a shape-equivalent probe
/// router with the same paths and HTTP methods and assert "not 404"
/// per (path, method) pair. A regression that drops a `.route(...)`
/// line in cms_v2.rs:830-892 would compile but the corresponding
/// integration-test would 404 — this scaffold catches it sooner.
#[tokio::test]
async fn admin_router_mounts_all_documented_admin_cms_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let app = common::setup_test_app().await;

    // Sanity: unmounted path on the real router is 404.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/admin/cms-v2/not-a-cms-route/sub/sub")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Every (method, path) pair on the real /api/admin/cms-v2 router must reach
    // the auth extractor (401 Unauthorized without a token), never 404.
    let id = "00000000-0000-0000-0000-000000000000";
    let cases: &[(&str, String)] = &[
        ("GET", "/api/admin/cms-v2/stats".to_string()),
        ("GET", "/api/admin/cms-v2/folders".to_string()),
        ("POST", "/api/admin/cms-v2/folders".to_string()),
        ("GET", format!("/api/admin/cms-v2/folders/{id}")),
        ("PUT", format!("/api/admin/cms-v2/folders/{id}")),
        ("DELETE", format!("/api/admin/cms-v2/folders/{id}")),
        ("GET", "/api/admin/cms-v2/assets".to_string()),
        ("POST", "/api/admin/cms-v2/assets".to_string()),
        ("GET", format!("/api/admin/cms-v2/assets/{id}")),
        ("PUT", format!("/api/admin/cms-v2/assets/{id}")),
        ("DELETE", format!("/api/admin/cms-v2/assets/{id}")),
        ("GET", "/api/admin/cms-v2/content".to_string()),
        ("POST", "/api/admin/cms-v2/content".to_string()),
        ("GET", format!("/api/admin/cms-v2/content/{id}")),
        ("PUT", format!("/api/admin/cms-v2/content/{id}")),
        ("DELETE", format!("/api/admin/cms-v2/content/{id}")),
        ("POST", format!("/api/admin/cms-v2/content/{id}/status")),
        ("GET", format!("/api/admin/cms-v2/content/{id}/revisions")),
        (
            "POST",
            format!("/api/admin/cms-v2/content/{id}/revisions/3/restore"),
        ),
        ("GET", format!("/api/admin/cms-v2/content/{id}/tags")),
        ("POST", format!("/api/admin/cms-v2/content/{id}/tags/{id}")),
        (
            "DELETE",
            format!("/api/admin/cms-v2/content/{id}/tags/{id}"),
        ),
        ("GET", format!("/api/admin/cms-v2/content/{id}/comments")),
        ("POST", format!("/api/admin/cms-v2/content/{id}/comments")),
        ("POST", format!("/api/admin/cms-v2/comments/{id}/resolve")),
        ("GET", "/api/admin/cms-v2/tags".to_string()),
        ("POST", "/api/admin/cms-v2/tags".to_string()),
        ("GET", "/api/admin/cms-v2/settings".to_string()),
        ("PUT", "/api/admin/cms-v2/settings".to_string()),
        ("GET", "/api/admin/cms-v2/menus".to_string()),
        ("POST", "/api/admin/cms-v2/menus".to_string()),
        ("GET", format!("/api/admin/cms-v2/menus/{id}")),
        ("PUT", format!("/api/admin/cms-v2/menus/{id}")),
        ("GET", "/api/admin/cms-v2/redirects".to_string()),
        ("POST", "/api/admin/cms-v2/redirects".to_string()),
        ("DELETE", format!("/api/admin/cms-v2/redirects/{id}")),
    ];
    for (method, path) in cases {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(*method)
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::UNAUTHORIZED,
            "{method} {path} must be mounted on the real router and require auth (401)"
        );
    }
}

// ── 4. public_router(): documented public mount-table is complete ────

#[tokio::test]
async fn public_router_mounts_all_documented_public_cms_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let app = common::setup_test_app().await;

    // Sanity: an extra path segment must not match :content_type/:slug and returns an empty 404.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/cms/content/page/about/extra-segment")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // GET /api/cms/menus returns 200 OK anonymously on the real router.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/cms/menus")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "GET /api/cms/menus must be mounted on the real public router and return 200 OK"
    );

    // GET /api/cms/content/:content_type/:slug reaches the real handler (400 on unknown content_type).
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/cms/content/invalid_type/about")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "GET /api/cms/content/:content_type/:slug must reach handler on the real router"
    );

    // All 4 public routes are mounted as GET-only (POST returns 405 Method Not Allowed on the real router).
    for path in [
        "/api/cms/content/page/about",
        "/api/cms/settings",
        "/api/cms/menus",
        "/api/cms/menus/footer",
    ] {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "POST {path} must return 405 on the real router (proving GET route is mounted)"
        );
    }
}

// ── 5. /content/:id/status is POST-only (not GET / PUT / DELETE) ─────

#[tokio::test]
async fn content_status_route_is_post_only() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let app = common::setup_test_app().await;
    let id = "00000000-0000-0000-0000-000000000000";

    // POST is mounted on the real router (returns 401 without token).
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/admin/cms-v2/content/{id}/status"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // GET / PUT / DELETE on /api/admin/cms-v2/content/:id/status must each be 405.
    for method in ["GET", "PUT", "DELETE"] {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(format!("/api/admin/cms-v2/content/{id}/status"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "{method} /api/admin/cms-v2/content/:id/status must be 405 — POST-only workflow transition"
        );
    }
}

// ── 6. /redirects/:id is DELETE-only ─────────────────────────────────

#[tokio::test]
async fn redirects_id_route_is_delete_only() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let app = common::setup_test_app().await;
    let id = "00000000-0000-0000-0000-000000000000";

    // DELETE is mounted on the real router (returns 401 without token).
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/cms-v2/redirects/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // GET / PUT / POST on /api/admin/cms-v2/redirects/:id must each be 405.
    for method in ["GET", "PUT", "POST"] {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(format!("/api/admin/cms-v2/redirects/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "{method} /api/admin/cms-v2/redirects/:id must be 405 — DELETE-only contract"
        );
    }
}
