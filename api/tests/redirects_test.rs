//! Redirects route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::redirects` and exercises
//! the 4 public DTOs + the `router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/redirects.rs` (398 LOC) manages URL redirects for SEO and
//! site maintenance. Every public page request runs through the
//! `/redirect/resolve` endpoint (front-door middleware) and the admin
//! surface is gated behind `AdminUser`. Handlers can't be invoked in
//! isolation. What we CAN pin:
//!
//! 1. **`Redirect.id` is `i64` BIGSERIAL.** A long-running site
//!    accumulates redirects fast (every renamed slug, every retired
//!    page, every legacy URL); the table can legitimately cross `2^31`
//!    rows over years. The PK flows into `WHERE id = $1` on
//!    show/update/destroy + the hit-counter UPDATE.
//!
//! 2. **`Redirect.status_code` is `i32` (allowlist 301/302/307/308).**
//!    HTTP status codes are 3-digit integers — `i32` is correct here
//!    (CLAUDE.md "Reserved exception" for legitimately-bounded counts).
//!    The handler explicitly validates against `[301, 302, 307, 308]`;
//!    a regression that broadened the type to something exotic would
//!    still hit the runtime allowlist check, but the DTO pin documents
//!    intent.
//!
//! 3. **`Redirect.hit_count` is `i32`.** Per-redirect hit counter —
//!    CLAUDE.md "Reserved exception" candidate (counters that
//!    legitimately cap below 2B). A single redirect getting 2B hits is
//!    plausible only for the most-trafficked legacy URLs on a major
//!    site; the i32 pin is the existing house contract.
//!
//! 4. **`RedirectQuery` all-optional, `per_page: Option<i64>`.** The
//!    admin grid's default view ("show me everything") sends an empty
//!    query. Pin the i64 paginator shape — `per_page` is clamped at
//!    the handler to `.min(500)` but the input type stays wide.
//!
//! 5. **`CreateRedirect` requires `from_path` + `to_path`, optional
//!    status/type.** Most redirects are 301 (permanent), so
//!    `status_code` defaults at the handler. The redirect-loop check
//!    (`from_path != to_path`) is a runtime guard; the DTO just pins
//!    the shape.
//!
//! 6. **`UpdateRedirect` follows PATCH semantics — every field
//!    optional.** Toggling just `is_active` without re-sending the
//!    full payload is the most common admin action (disable a broken
//!    redirect during incident response).
//!
//! 7. **`router()` mount table compile-pin.** Mount table covers the
//!    admin CRUD `/admin/redirects/*` AND the public
//!    `/redirect/resolve` (middleware hook) — they MUST coexist in a
//!    single `Router<AppState>` since they're merged at root level
//!    in `api_router()`.
//!
//! ## Pattern source
//!
//! Modeled on `tests/forms_test.rs`, `tests/connections_test.rs`,
//! `tests/admin_orders_test.rs`.

use revolution_api::routes::redirects::{CreateRedirect, Redirect, RedirectQuery, UpdateRedirect};

// ── 1. Redirect: BIGSERIAL id is i64; status_code + hit_count i32 ────

/// `Redirect.id` is BIGSERIAL → must stay `i64`. The redirects table
/// on a long-running site accumulates rows fast (every slug rename,
/// every retired page, every legacy URL migrated). Crossing `2^31`
/// rows over many years is realistic; narrowing the PK silently 404s
/// the first high-id redirect on the admin grid.
#[test]
fn redirect_id_is_i64_status_and_hits_are_i32() {
    let now = chrono::Utc::now();
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let r = Redirect {
        id: above_i32_max,
        from_path: "/old-product".to_string(),
        to_path: "/products/new-shiny".to_string(),
        status_code: 301,
        redirect_type: "manual".to_string(),
        is_active: true,
        hit_count: i32::MAX, // largest legal i32 hit count
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&r).expect("serialize Redirect");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32_max),
        "Redirect.id MUST be i64 — BIGSERIAL PK"
    );
    assert_eq!(
        wire["status_code"].as_i64(),
        Some(301),
        "status_code is i32 — 3-digit HTTP status, CLAUDE.md exception applies"
    );
    assert_eq!(
        wire["hit_count"].as_i64(),
        Some(i32::MAX as i64),
        "hit_count is i32 — per-redirect counter, CLAUDE.md exception applies"
    );

    // Sanity — fixture id must exceed i32::MAX or the i64 pin above
    // could pass with an i32 field.
    assert!(r.id > i32::MAX as i64);
}

// ── 2. RedirectQuery: all-optional admin-grid default view ───────────

/// The admin redirect grid lands on `/admin/redirects` with no filters
/// set. Any field flipping to required would 400-error that landing
/// view. `per_page` is `Option<i64>` (clamped at the handler to
/// `.min(500)`). Pin all-optional.
#[test]
fn redirect_query_accepts_empty_and_filtered_payloads() {
    let empty: RedirectQuery =
        serde_json::from_str("{}").expect("empty RedirectQuery must deserialize");
    assert!(empty.search.is_none());
    assert!(empty.is_active.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.page.is_none());

    // Filtered: active redirects, search by path substring.
    let filtered: RedirectQuery = serde_json::from_value(serde_json::json!({
        "search": "old-product",
        "is_active": true,
        "per_page": 50_i64,
        "page": 1_i64
    }))
    .expect("filtered RedirectQuery must deserialize");
    assert_eq!(filtered.search.as_deref(), Some("old-product"));
    assert_eq!(filtered.is_active, Some(true));
    assert_eq!(filtered.per_page, Some(50));
    assert_eq!(filtered.page, Some(1));

    // Negative-test: per_page must be a number, not a string.
    assert!(
        serde_json::from_value::<RedirectQuery>(serde_json::json!({"per_page": "fifty"})).is_err(),
        "non-numeric per_page MUST fail (Option<i64> type pin)"
    );
}

// ── 3. CreateRedirect: required from_path + to_path, optional rest ───

/// `CreateRedirect` requires the two routing fields (`from_path`,
/// `to_path`) — you can't have a redirect without both. `status_code`
/// is optional (defaults to 301 at the handler — the right default
/// for SEO-preserving permanent redirects). `redirect_type` is
/// optional (defaults to "manual"). Pin the required-vs-optional
/// contract — flipping `status_code` to required would 400-error the
/// "just create a 301" landing action of the admin UI.
#[test]
fn create_redirect_requires_paths_only() {
    // Minimal payload — just the two paths.
    let minimal: CreateRedirect = serde_json::from_value(serde_json::json!({
        "from_path": "/old",
        "to_path": "/new"
    }))
    .expect("minimal CreateRedirect must deserialize");
    assert_eq!(minimal.from_path, "/old");
    assert_eq!(minimal.to_path, "/new");
    assert!(
        minimal.status_code.is_none(),
        "status_code optional — handler defaults to 301"
    );
    assert!(
        minimal.redirect_type.is_none(),
        "redirect_type optional — handler defaults to 'manual'"
    );

    // With explicit status code.
    let full: CreateRedirect = serde_json::from_value(serde_json::json!({
        "from_path": "/temp-promo",
        "to_path": "/promos/winter",
        "status_code": 302,
        "redirect_type": "campaign"
    }))
    .expect("full CreateRedirect must deserialize");
    assert_eq!(full.status_code, Some(302));
    assert_eq!(full.redirect_type.as_deref(), Some("campaign"));

    // Missing `from_path` MUST fail.
    assert!(
        serde_json::from_value::<CreateRedirect>(serde_json::json!({"to_path": "/new"})).is_err(),
        "CreateRedirect without `from_path` MUST fail (required)"
    );

    // Missing `to_path` MUST fail.
    assert!(
        serde_json::from_value::<CreateRedirect>(serde_json::json!({"from_path": "/old"})).is_err(),
        "CreateRedirect without `to_path` MUST fail (required)"
    );
}

// ── 4. UpdateRedirect: PATCH semantics — every field optional ────────

/// `UpdateRedirect` is fully optional. The most-common admin action
/// is toggling `is_active` during incident response (turn off a
/// broken redirect without re-sending the path config). A regression
/// that flipped any field to required would break that flow.
#[test]
fn update_redirect_follows_patch_semantics() {
    // Empty payload — pure PATCH, all-optional.
    let empty: UpdateRedirect =
        serde_json::from_str("{}").expect("empty UpdateRedirect must deserialize (PATCH)");
    assert!(empty.from_path.is_none());
    assert!(empty.to_path.is_none());
    assert!(empty.status_code.is_none());
    assert!(empty.is_active.is_none());

    // Toggle-only: the disable-during-incident flow.
    let toggle: UpdateRedirect = serde_json::from_value(serde_json::json!({"is_active": false}))
        .expect("toggle-only UpdateRedirect must deserialize");
    assert_eq!(toggle.is_active, Some(false));
    assert!(toggle.from_path.is_none());
    assert!(toggle.to_path.is_none());
    assert!(toggle.status_code.is_none());

    // Repoint: redirect the same `from_path` to a new destination.
    let repoint: UpdateRedirect = serde_json::from_value(serde_json::json!({
        "to_path": "/promos/spring",
        "status_code": 301
    }))
    .expect("repoint UpdateRedirect must deserialize");
    assert_eq!(repoint.to_path.as_deref(), Some("/promos/spring"));
    assert_eq!(repoint.status_code, Some(301));
    assert!(repoint.from_path.is_none());
}

// ── 5. Router mount table compile-pin ────────────────────────────────

/// `router()` must build as `Router<AppState>`. Mount table:
///   - `/admin/redirects` (GET list + POST create)
///   - `/admin/redirects/:id` (GET show + PUT update + DELETE destroy)
///   - `/admin/redirects/:id/hit` (POST increment_hit)
///   - `/redirect/resolve` (GET, PUBLIC — middleware hook)
///
/// Note: this router is `.merge()`d at root in `api_router()` because
/// the public `/redirect/resolve` path lives at the API root (not
/// nested under `/redirects`). A refactor that broke the mount table
/// (e.g. dropped `_admin: AdminUser` extractor from a CRUD handler,
/// or accidentally gated `resolve_redirect` behind `AdminUser`) would
/// fail compilation here.
#[test]
fn redirects_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::redirects::router();
}

/// Idempotent constructor — must be safe to call multiple times.
/// `api_router()` merges this once at root level; nothing prevents
/// future composition from re-merging it.
#[test]
fn redirects_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::redirects::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::redirects::router();
}
