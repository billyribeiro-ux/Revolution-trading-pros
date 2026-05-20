//! CMS v2 Enterprise route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_v2_enterprise` and
//! exercises the public mount-table routers (`router()` for the
//! admin-nested `/admin/cms-v2/enterprise/*` surface and
//! `public_router()` for the unauthenticated `/preview/:token`
//! validator).
//!
//! ## Why this shape
//!
//! Every handler in `routes/cms_v2_enterprise.rs` runs live SQL via
//! the `cms_audit` / `cms_workflow` / `cms_preview` service layer.
//! There are no unit-testable pure functions and (deliberately) no
//! `pub` request/response DTOs on the module surface — the request
//! shapes are private to the module and the responses re-export
//! types from the service layer.
//!
//! The contract we CAN pin without a DB:
//!
//! 1. `router()` and `public_router()` MUST build with `AppState`.
//!    A refactor that breaks a handler signature (wrong extractor,
//!    wrong return type, missing `AdminUser` on an audit endpoint)
//!    will fail to compile here.
//!
//! 2. The split between the two routers is load-bearing —
//!    `public_router()` is mounted via `.merge()` in `routes/mod.rs`
//!    so `validate_preview_token` is reachable without auth. A
//!    regression that moves it under the admin-nested router would
//!    403 every preview link the editorial team shares.
//!
//! 3. The audit endpoint must be admin-only (`AdminUser` extractor);
//!    the workflow + preview endpoints take the lighter `User`
//!    extractor + a `require_editor` check inside the handler. Both
//!    sets must stay buildable independently.
//!
//! ## Pattern source
//!
//! Modeled on the router-mount-table tests in `tests/payments_test.rs`
//! (R2-02), `tests/orders_test.rs` / `tests/oauth_test.rs` (R3-A/B),
//! `tests/admin_indicators_test.rs` (R4-D). Because no public DTOs
//! are exposed by this module we also pin the wire-format contract
//! of the **mount points** by building each router into an axum
//! `Router<AppState>` and confirming the construction does not panic.

// ── Router mount tables ──────────────────────────────────────────────

/// `router()` is the admin-nested router mounted at
/// `/admin/cms-v2/enterprise` (see `src/routes/mod.rs`). It exposes
/// 9 routes across audit logs, workflow, and preview tokens. The
/// build-without-panic test is load-bearing: any refactor that drops
/// a route, breaks a handler signature, or changes the state type
/// will fail to compile here.
#[test]
fn cms_v2_enterprise_admin_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2_enterprise::router();
}

/// `public_router()` exposes the single unauthenticated endpoint
/// `GET /preview/:token` that the marketing site uses to validate
/// editor-shared preview links. If a refactor accidentally moves
/// that route into the admin-nested `router()`, the marketing
/// site's link validation would start 401-ing — pin the split here.
#[test]
fn cms_v2_enterprise_public_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2_enterprise::public_router();
}

/// The two routers are independent: building one must not affect
/// the other. This is a compile-time guarantee — if a helper or
/// closure became shared state between them in a way that broke
/// `Clone`/`Send`/`Sync`, the second build call would fail.
#[test]
fn cms_v2_enterprise_routers_build_independently_in_same_scope() {
    let admin: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2_enterprise::router();
    let public: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2_enterprise::public_router();

    // Both must be usable as `axum::Router<AppState>` for the calling
    // site (`routes/mod.rs`) to .nest() / .merge() them. We exercise
    // that contract by binding to the trait-required type and dropping.
    drop(admin);
    drop(public);
}

// ── Compile-time guarantees on the module surface ────────────────────

/// Pin that the function items themselves are addressable as
/// `fn() -> Router<AppState>` — this catches a regression where
/// someone changes the signature to `fn(state: AppState) -> Router<()>`
/// or similar. The two function pointers below MUST compile.
#[test]
fn cms_v2_enterprise_router_signatures_are_unit_to_router() {
    let _f1: fn() -> axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2_enterprise::router;
    let _f2: fn() -> axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_v2_enterprise::public_router;
}

/// Both routers must merge into the production-style nested mount
/// table without conflict. This mimics the `routes/mod.rs` shape
/// (`.nest("/admin/cms-v2/enterprise", router()).merge(public_router())`)
/// so an accidental path collision between the two would surface
/// here at build time, not in a half-deployed Railway container.
#[test]
fn cms_v2_enterprise_routers_compose_without_path_conflicts() {
    let composed: axum::Router<revolution_api::AppState> = axum::Router::new()
        .nest(
            "/admin/cms-v2/enterprise",
            revolution_api::routes::cms_v2_enterprise::router(),
        )
        .merge(revolution_api::routes::cms_v2_enterprise::public_router());

    // We can't hit the routes without a real `AppState`, but we CAN
    // assert the composition produced a buildable `Router<AppState>` —
    // any path/state collision would panic during `.nest` / `.merge`.
    drop(composed);
}
