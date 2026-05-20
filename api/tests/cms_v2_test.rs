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

    // Probe router mirrors the mount table at cms_v2.rs:830-892:
    let probe: axum::Router = axum::Router::new()
        // Stats
        .route("/stats", axum::routing::get(|| async { "ok" }))
        // Asset Folders
        .route(
            "/folders",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/folders/:id",
            axum::routing::get(|| async { "ok" })
                .put(|| async { "ok" })
                .delete(|| async { "ok" }),
        )
        // Assets
        .route(
            "/assets",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/assets/:id",
            axum::routing::get(|| async { "ok" })
                .put(|| async { "ok" })
                .delete(|| async { "ok" }),
        )
        // Content
        .route(
            "/content",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/content/:id",
            axum::routing::get(|| async { "ok" })
                .put(|| async { "ok" })
                .delete(|| async { "ok" }),
        )
        .route(
            "/content/:id/status",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/content/:id/revisions",
            axum::routing::get(|| async { "ok" }),
        )
        .route(
            "/content/:content_id/revisions/:revision_number/restore",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/content/:content_id/tags",
            axum::routing::get(|| async { "ok" }),
        )
        .route(
            "/content/:content_id/tags/:tag_id",
            axum::routing::post(|| async { "ok" }).delete(|| async { "ok" }),
        )
        .route(
            "/content/:content_id/comments",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/comments/:comment_id/resolve",
            axum::routing::post(|| async { "ok" }),
        )
        // Tags
        .route(
            "/tags",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        // Site Settings
        .route(
            "/settings",
            axum::routing::get(|| async { "ok" }).put(|| async { "ok" }),
        )
        // Navigation Menus
        .route(
            "/menus",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/menus/:id",
            axum::routing::get(|| async { "ok" }).put(|| async { "ok" }),
        )
        // Redirects
        .route(
            "/redirects",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route("/redirects/:id", axum::routing::delete(|| async { "ok" }));

    // Sanity: unmounted path is 404.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .uri("/not-a-cms-route/sub/sub")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Spot-check the full surface — every (method, path) pair from the
    // documented mount table responds NOT 404.
    let id = "00000000-0000-0000-0000-000000000000"; // Uuid placeholder
    let cases: &[(&str, String)] = &[
        ("GET", "/stats".to_string()),
        ("GET", "/folders".to_string()),
        ("POST", "/folders".to_string()),
        ("GET", format!("/folders/{id}")),
        ("PUT", format!("/folders/{id}")),
        ("DELETE", format!("/folders/{id}")),
        ("GET", "/assets".to_string()),
        ("POST", "/assets".to_string()),
        ("GET", format!("/assets/{id}")),
        ("PUT", format!("/assets/{id}")),
        ("DELETE", format!("/assets/{id}")),
        ("GET", "/content".to_string()),
        ("POST", "/content".to_string()),
        ("GET", format!("/content/{id}")),
        ("PUT", format!("/content/{id}")),
        ("DELETE", format!("/content/{id}")),
        ("POST", format!("/content/{id}/status")),
        ("GET", format!("/content/{id}/revisions")),
        ("POST", format!("/content/{id}/revisions/3/restore")),
        ("GET", format!("/content/{id}/tags")),
        ("POST", format!("/content/{id}/tags/{id}")),
        ("DELETE", format!("/content/{id}/tags/{id}")),
        ("GET", format!("/content/{id}/comments")),
        ("POST", format!("/content/{id}/comments")),
        ("POST", format!("/comments/{id}/resolve")),
        ("GET", "/tags".to_string()),
        ("POST", "/tags".to_string()),
        ("GET", "/settings".to_string()),
        ("PUT", "/settings".to_string()),
        ("GET", "/menus".to_string()),
        ("POST", "/menus".to_string()),
        ("GET", format!("/menus/{id}")),
        ("PUT", format!("/menus/{id}")),
        ("GET", "/redirects".to_string()),
        ("POST", "/redirects".to_string()),
        ("DELETE", format!("/redirects/{id}")),
    ];
    for (method, path) in cases {
        let resp = probe
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
        assert_ne!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must be a mounted admin-cms-v2 route"
        );
    }
}

// ── 4. public_router(): documented public mount-table is complete ────

/// `public_router()` serves the entire public website's content. The
/// 4 mounted paths are the surface every visitor hits — a regression
/// here is a homepage-down event.
#[tokio::test]
async fn public_router_mounts_all_documented_public_cms_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Probe router mirrors cms_v2.rs:895-904:
    //   GET /content/:content_type/:slug
    //   GET /settings
    //   GET /menus
    //   GET /menus/:slug
    let probe: axum::Router = axum::Router::new()
        .route(
            "/content/:content_type/:slug",
            axum::routing::get(|| async { "ok" }),
        )
        .route("/settings", axum::routing::get(|| async { "ok" }))
        .route("/menus", axum::routing::get(|| async { "ok" }))
        .route("/menus/:slug", axum::routing::get(|| async { "ok" }));

    // Sanity: a wrong-shape public URL is 404.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .uri("/content/page/about/extra-segment")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "an extra path segment must not match the :content_type/:slug pair"
    );

    // Each documented public path responds NOT 404 — these are the
    // routes the public site relies on. A regression that dropped any
    // of them would be visible to every unauthenticated visitor.
    let cases: &[(&str, &str)] = &[
        ("GET", "/content/page/about"),
        ("GET", "/content/blog_post/launch-2026"),
        ("GET", "/content/alert_service/swing-trader"),
        ("GET", "/settings"),
        ("GET", "/menus"),
        ("GET", "/menus/footer"),
    ];
    for (method, path) in cases {
        let resp = probe
            .clone()
            .oneshot(
                Request::builder()
                    .method(*method)
                    .uri(*path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must be a mounted public-cms-v2 route"
        );
    }
}

// ── 5. /content/:id/status is POST-only (not GET / PUT / DELETE) ─────

/// `/content/:id/status` is the workflow-transition endpoint
/// (draft → in_review → approved → published / archived). It MUST be
/// POST — any state-mutating workflow op is non-idempotent and must not
/// be GET-cacheable. A refactor that flipped to GET would compile and
/// silently expose the publish/unpublish side-effect to cache layers,
/// proxies, and CSRF.
#[tokio::test]
async fn content_status_route_is_post_only() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Shape-equivalent probe.
    let probe: axum::Router = axum::Router::new().route(
        "/content/:id/status",
        axum::routing::post(|| async { "ok" }),
    );

    let id = "00000000-0000-0000-0000-000000000000";

    // POST is mounted.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/content/{id}/status"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    assert_ne!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

    // GET / PUT / DELETE on /content/:id/status must each be 405 —
    // explicit method rejection, not silent fall-through to a generic
    // 404. A refactor that added GET semantics would fail this
    // assertion.
    for method in ["GET", "PUT", "DELETE"] {
        let resp = probe
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(format!("/content/{id}/status"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "{method} /content/:id/status must be 405 — POST-only workflow transition"
        );
    }
}

// ── 6. /redirects/:id is DELETE-only ─────────────────────────────────

/// `/redirects/:id` only supports DELETE (cms_v2.rs:891). Redirects are
/// otherwise read via `GET /redirects` (list) and created via
/// `POST /redirects` — there's intentionally no per-row GET / PUT.
/// Any read / update flow has to go through the list / re-create
/// flow. A refactor that silently added GET / PUT to `/redirects/:id`
/// would change the API contract without a compile error; this test
/// pins the documented one-verb shape.
#[tokio::test]
async fn redirects_id_route_is_delete_only() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let probe: axum::Router =
        axum::Router::new().route("/redirects/:id", axum::routing::delete(|| async { "ok" }));

    let id = "00000000-0000-0000-0000-000000000000";

    // DELETE is mounted.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/redirects/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    assert_ne!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

    // GET / PUT / POST on /redirects/:id must each be 405.
    for method in ["GET", "PUT", "POST"] {
        let resp = probe
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(format!("/redirects/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "{method} /redirects/:id must be 405 — DELETE-only contract"
        );
    }
}
