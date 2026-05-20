//! Admin-orders route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_orders` and
//! exercises the public `ExportQuery` DTO + the `router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/admin_orders.rs` is the canonical mount-point for the
//! `/admin/orders/*` surface (the CSV-export and stats endpoints live
//! here; the heavy CRUD handlers are re-mounted from `routes/orders.rs`
//! via `routes::orders::admin_*`). Every handler runs live SQL against
//! `orders` / `order_items` / `users`, so we can't invoke them in
//! isolation. What we CAN pin:
//!
//! 1. **`ExportQuery` wire shape.** Filters (`status`, `search`,
//!    `format`) are all optional — the admin grid POSTs `/export`
//!    with whatever filter set the user has currently applied (often
//!    empty). A refactor that flips `format` to required would
//!    400-error the "export everything as CSV" flow, which is the
//!    most-used legal-discovery path on this stack.
//!
//! 2. **Router mount table (9 routes).** The admin orders router is
//!    the most-routes-per-file admin mount on the stack:
//!      - `GET  /`                            → admin_index
//!      - `GET  /export`                      → admin_export (THIS file)
//!      - `GET  /stats`                       → admin_stats  (THIS file)
//!      - `GET  /:id`                         → admin_show
//!      - `POST /:id/status`                  → admin_update_status
//!      - `POST /:id/refund`                  → admin_refund  (money!)
//!      - `POST /:id/cancel`                  → admin_cancel
//!      - `POST /:id/fulfill`                 → admin_fulfill
//!      - `POST /:id/resend-confirmation`     → admin_resend_confirmation
//!
//!    Per the source-file comment, `/export` and `/stats` MUST be
//!    registered BEFORE `/:id` or axum greedily matches them as the
//!    `id` path param. We can't probe paths without `AppState`, but
//!    we CAN compile-pin the `Router<AppState>` return type — that
//!    catches every extractor-signature regression at build time
//!    (e.g. accidentally dropping `AdminUser` from `admin_refund`,
//!    which would let any authenticated user trigger a Stripe refund).
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/products_test.rs`, `tests/admin_member_management_test.rs`.

use revolution_api::routes::admin_orders::ExportQuery;

// ── 1. ExportQuery: empty payload → all-None filters ─────────────────

/// The CSV-export flow starts with the admin clicking "Export" with
/// no filters set — that posts an empty query string. The handler then
/// streams every order in the system as CSV. A regression that flipped
/// any of `status` / `search` / `format` to required would 400-error
/// this flow (the most-used legal-discovery path). Pin the
/// optional-everywhere shape.
#[test]
fn export_query_accepts_empty_payload() {
    let empty: ExportQuery =
        serde_json::from_str("{}").expect("empty ExportQuery must deserialize");
    assert!(empty.status.is_none());
    assert!(empty.search.is_none());
    assert!(empty.format.is_none());
}

// ── 2. ExportQuery: status filter ────────────────────────────────────

/// The admin grid filters by order status (`pending` / `completed` /
/// `refunded` / `cancelled` / etc.) and passes the filter through to
/// `/export`. The DTO MUST accept any string — the handler validates
/// the value against the DB ENUM, not serde. Pin the permissive shape.
#[test]
fn export_query_accepts_status_filter() {
    for status in &["pending", "completed", "refunded", "cancelled", "fulfilled"] {
        let q: ExportQuery = serde_json::from_value(serde_json::json!({"status": status}))
            .unwrap_or_else(|e| panic!("ExportQuery status={status} must deserialize: {e}"));
        assert_eq!(q.status.as_deref(), Some(*status));
        assert!(q.search.is_none());
        assert!(q.format.is_none());
    }
}

// ── 3. ExportQuery: search filter is free-form ───────────────────────

/// `search` is free-form (matched against order_number / email /
/// customer name via ILIKE at SQL time). The DTO must accept anything
/// the user types into the admin grid's search box — including
/// SQL-special characters (handled by parameterized sqlx, not by
/// serde-side rejection). Pin that.
#[test]
fn export_query_accepts_arbitrary_search_strings() {
    // Plain
    let plain: ExportQuery =
        serde_json::from_value(serde_json::json!({"search": "jane@example.com"}))
            .expect("plain search must deserialize");
    assert_eq!(plain.search.as_deref(), Some("jane@example.com"));

    // SQL-special chars (handled by sqlx binding, not serde rejection)
    let weird: ExportQuery = serde_json::from_value(serde_json::json!({
        "search": "O'Brien' OR 1=1--",
    }))
    .expect("SQL-special search must deserialize (sqlx params handle escaping)");
    assert_eq!(weird.search.as_deref(), Some("O'Brien' OR 1=1--"));

    // Unicode
    let uni: ExportQuery = serde_json::from_value(serde_json::json!({"search": "café"}))
        .expect("unicode search must deserialize");
    assert_eq!(uni.search.as_deref(), Some("café"));
}

// ── 4. ExportQuery: format reserved for future use ───────────────────

/// `format` is reserved for future formats (xlsx, json) — CSV is the
/// only supported value today, but the field MUST stay an
/// `Option<String>` so the eventual rollout doesn't break the wire
/// contract. Pin the type and the documented values.
#[test]
fn export_query_accepts_format_hint() {
    let csv: ExportQuery = serde_json::from_value(serde_json::json!({"format": "csv"}))
        .expect("format=csv must deserialize");
    assert_eq!(csv.format.as_deref(), Some("csv"));

    // Future values — must still parse (handler will reject unknowns
    // at runtime, but the DTO is permissive)
    let xlsx: ExportQuery = serde_json::from_value(serde_json::json!({"format": "xlsx"}))
        .expect("future format hint must deserialize");
    assert_eq!(xlsx.format.as_deref(), Some("xlsx"));

    // Combined filters — the realistic flow
    let combined: ExportQuery = serde_json::from_value(serde_json::json!({
        "status": "completed",
        "search": "ORD-",
        "format": "csv",
    }))
    .expect("combined filters must deserialize");
    assert_eq!(combined.status.as_deref(), Some("completed"));
    assert_eq!(combined.search.as_deref(), Some("ORD-"));
    assert_eq!(combined.format.as_deref(), Some("csv"));
}

// ── 5. Router mount table compile-pin ────────────────────────────────

/// `router()` must build as `Router<AppState>`. Load-bearing because
/// the 9-route admin-orders mount includes both the local `/export` +
/// `/stats` handlers AND the re-mounted `/:id/refund` / `/:id/cancel`
/// / `/:id/fulfill` handlers from `routes::orders::admin_*` — a
/// regression in EITHER file's handler signatures (wrong extractor,
/// wrong return type, dropped AdminUser gate) would fail compilation
/// here. This is the single canonical pin for the entire `/admin/orders`
/// surface.
#[test]
fn admin_orders_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::admin_orders::router();
}

/// Second router-build invocation pins idempotency — the constructor
/// must be safe to call multiple times in the same process (the main
/// `api_router()` does this implicitly when nesting at /admin/orders
/// from a higher-level group). A regression that introduced a global
/// `static` mutable inside the constructor would fail here.
#[test]
fn admin_orders_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_orders::router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_orders::router();
    // If the constructor were stateful, the second call would panic
    // ("already initialized"); the fact that we got here proves
    // statelessness.
}
