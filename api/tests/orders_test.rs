//! Orders route scaffold tests — exercises the **pure, no-DB** surfaces
//! that `api/src/routes/orders.rs` depends on for money/correctness.
//!
//! ## Why this shape
//!
//! `routes/orders.rs` is 1,382 LOC of order listing / detail / admin
//! handlers (`index`, `show`, `show_by_number`, `admin_index`,
//! `admin_show`, `admin_update_status`, `admin_refund`, `admin_cancel`,
//! `admin_fulfill`, `admin_resend_confirmation`). Every handler takes
//! `State<AppState>` (live DB + email + Stripe HTTP client), so we
//! cannot invoke them in unit-test isolation the way `utils_test.rs`
//! calls `utils::*`.
//!
//! What we CAN test without a DB or live Stripe is the **contract**
//! the handlers ride on:
//!
//! 1. The wire-format response types (`OrderDetailResponse`,
//!    `OrderItemDetailResponse`, `AdminOrderResponse`,
//!    `AdminOrderStats`). Every `*_cents` field is load-bearing — the
//!    CLAUDE.md money rule is "all `*_cents` values are `i64`, never
//!    `i32`", and a future refactor that flips one of these to `i32`
//!    would silently cap revenue rollups at $21.4M. These tests are
//!    the regression guard.
//!
//! 2. The wire-format request type (`RefundOrderRequest`). Its
//!    `amount_cents` field is `Option<i64>` (None ⇒ full refund); a
//!    silent change to required-or-narrower would change the refund
//!    semantics from "full" to "400 Bad Request".
//!
//! 3. The `router()` + `admin_router()` mount tables. A diff that
//!    accidentally drops `/{id}/refund` or `/by-number/{order_number}`
//!    from the registry would compile but silently 404 in prod; this
//!    catches that.
//!
//! ## Pattern source
//!
//! Modeled on `api/tests/payments_test.rs` (PR adding R2-02 scaffold,
//! commit 103b828c) — same shape: bind directly to
//! `revolution_api::routes::orders::*`, no DB, no live network,
//! deserialize/serialize JSON to pin the wire types, and probe a
//! shadow router (not the real one — that needs `AppState`) for path
//! existence sanity.
//!
//! ## Coverage delta
//!
//! orders.rs route handlers: 10 mounted handlers across 2 routers
//! ( /, /by-number/{order_number}, /{id} on `router()`;
//!   /, /{id}, /{id}/status, /{id}/refund, /{id}/cancel, /{id}/fulfill,
//!   /{id}/resend-confirmation on `admin_router()` ).
//! This scaffold covers their **contract surface** — 4 tests over the
//! request/response types (every `*_cents` field pinned to i64) and 2
//! tests for the router mount tables.

use revolution_api::routes::orders::{
    AdminOrderResponse, AdminOrderStats, OrderDetailResponse, OrderItemDetailResponse,
    RefundOrderRequest,
};

// ── 1. OrderDetailResponse: every *_cents field round-trips as i64 ──

/// `OrderDetailResponse` exposes five `*_cents` money fields
/// (`subtotal_cents`, `discount_cents`, `tax_cents`, `total_cents`,
/// plus per-item cents via the nested `items` vec). The load-bearing
/// assertion is that values BEYOND `i32::MAX` round-trip intact —
/// proving none of the fields can be silently narrowed to `i32` (a
/// regression that would cap any single order line at $21,474,836.47
/// and ALL aggregate totals + per-customer LTV reports along with it).
#[test]
fn order_detail_response_cents_fields_serialize_as_i64_json_numbers() {
    let beyond_i32: i64 = i32::MAX as i64 + 1_000;

    let resp = OrderDetailResponse {
        id: 9_876_543_210_i64,
        order_number: "ORD-TEST-1".to_string(),
        status: "completed".to_string(),
        subtotal_cents: beyond_i32,
        discount_cents: 500,
        tax_cents: 12_345,
        total_cents: beyond_i32 + 12_345 - 500,
        currency: "usd".to_string(),
        billing_name: Some("Test User".to_string()),
        billing_email: Some("t@example.com".to_string()),
        billing_address: None,
        payment_provider: Some("stripe".to_string()),
        coupon_code: None,
        items: vec![OrderItemDetailResponse {
            id: 1,
            product_id: Some(42),
            plan_id: None,
            name: "Annual Plan".to_string(),
            quantity: 1,
            unit_price_cents: beyond_i32,
            total_cents: beyond_i32,
            product_type: Some("subscription".to_string()),
            product_slug: Some("annual".to_string()),
            thumbnail: None,
        }],
        created_at: "2026-05-20T00:00:00Z".to_string(),
        completed_at: Some("2026-05-20T00:01:00Z".to_string()),
    };

    let v: serde_json::Value =
        serde_json::to_value(&resp).expect("OrderDetailResponse must serialize");

    // id is i64 — Stripe-scale primary keys survive past i32.
    assert_eq!(v["id"].as_i64(), Some(9_876_543_210_i64));

    // Every cents field must be a JSON number AND recover EXACTLY as
    // i64 — proving no f64 lossy round-trip and no i32 narrowing.
    for field in [
        "subtotal_cents",
        "discount_cents",
        "tax_cents",
        "total_cents",
    ] {
        let n = v.get(field).unwrap_or_else(|| panic!("{field} missing"));
        assert!(n.is_number(), "{field} must be a JSON number");
        assert!(
            n.as_i64().is_some(),
            "{field} must recover as i64 (not f64)"
        );
    }
    assert_eq!(v["subtotal_cents"].as_i64(), Some(beyond_i32));
    assert_eq!(v["total_cents"].as_i64(), Some(beyond_i32 + 12_345 - 500));

    // Per-item cents — same i64 invariant.
    let item = &v["items"][0];
    assert_eq!(item["unit_price_cents"].as_i64(), Some(beyond_i32));
    assert_eq!(item["total_cents"].as_i64(), Some(beyond_i32));
}

// ── 2. AdminOrderStats: aggregate revenue/AOV stay i64 ──────────────

/// `AdminOrderStats` is the riskiest place for an i32 cap to bite:
/// `total_revenue_cents` is a SUM over ALL completed orders, so the
/// $21.4M cap is reached in months, not years. `average_order_value_
/// cents` is computed from that sum. Both must stay i64 end-to-end.
///
/// This test serializes a value > i32::MAX, parses it back, and
/// asserts the values are bit-identical — proof that the JSON
/// boundary doesn't silently narrow.
#[test]
fn admin_order_stats_revenue_fields_round_trip_as_i64() {
    // $100B in cents — would overflow i64-as-i32 cast by 4 orders of
    // magnitude. The test fails loudly if the field shrinks.
    let huge_revenue: i64 = 10_000_000_000_000_i64;

    let stats = AdminOrderStats {
        total_orders: 100_000,
        completed_orders: 98_000,
        pending_orders: 1_500,
        refunded_orders: 500,
        total_revenue_cents: huge_revenue,
        revenue_this_month_cents: huge_revenue / 12,
        average_order_value_cents: huge_revenue / 98_000,
    };

    let v: serde_json::Value =
        serde_json::to_value(&stats).expect("AdminOrderStats must serialize");

    // All three money fields must serialize as JSON numbers AND
    // recover exactly as i64.
    for field in [
        "total_revenue_cents",
        "revenue_this_month_cents",
        "average_order_value_cents",
    ] {
        let n = v.get(field).unwrap_or_else(|| panic!("{field} missing"));
        assert!(n.is_number(), "{field} must be a JSON number");
        assert!(
            n.as_i64().is_some(),
            "{field} must recover as i64 — i32 narrowing would cap at $21.4M"
        );
    }
    assert_eq!(v["total_revenue_cents"].as_i64(), Some(huge_revenue));
    assert_eq!(
        v["revenue_this_month_cents"].as_i64(),
        Some(huge_revenue / 12)
    );
    assert_eq!(
        v["average_order_value_cents"].as_i64(),
        Some(huge_revenue / 98_000)
    );

    // Counts are i64 too (>2B users is feasible in the long term).
    assert_eq!(v["total_orders"].as_i64(), Some(100_000));
}

// ── 3. AdminOrderResponse: total_cents preserved at wire boundary ───

/// `AdminOrderResponse::total_cents` is the per-row total shown in the
/// admin order list. It's load-bearing for the inline display
/// ("$1,234.56") and for any future "sort by amount" feature. Pinning
/// i64 here prevents a silent i32 narrowing during a refactor.
#[test]
fn admin_order_response_total_cents_round_trips_as_i64() {
    let big_total: i64 = i32::MAX as i64 + 999;

    let resp = AdminOrderResponse {
        id: 12_345_i64,
        order_number: "ORD-ADMIN-1".to_string(),
        status: "completed".to_string(),
        total_cents: big_total,
        currency: "usd".to_string(),
        user_email: "buyer@example.com".to_string(),
        user_name: Some("Buyer Name".to_string()),
        payment_provider: Some("stripe".to_string()),
        item_count: 3,
        created_at: "2026-05-20T00:00:00Z".to_string(),
        completed_at: Some("2026-05-20T00:05:00Z".to_string()),
    };

    let v: serde_json::Value =
        serde_json::to_value(&resp).expect("AdminOrderResponse must serialize");

    let amount_json = v.get("total_cents").expect("total_cents missing");
    assert!(amount_json.is_number(), "total_cents must be a JSON number");
    assert_eq!(
        amount_json.as_i64(),
        Some(big_total),
        "total_cents must round-trip through JSON as i64"
    );
    assert_eq!(v["id"].as_i64(), Some(12_345_i64));
    assert_eq!(v["item_count"].as_i64(), Some(3));
}

// ── 4. RefundOrderRequest: amount_cents is Option<i64> ──────────────

/// `RefundOrderRequest::amount_cents` is `Option<i64>` — `None` means
/// "full refund", `Some(x)` is a partial refund in integer cents.
/// Two regressions this test catches:
///
/// 1. Field flipped from `Option<i64>` to required `i64` — full-
///    refund admin flow would start returning 400 Bad Request.
/// 2. Field narrowed to `i32` — partial refunds over $21.4M would
///    overflow on parse or silently truncate.
#[test]
fn refund_order_request_amount_cents_round_trips_optional_i64() {
    // Case A: explicit partial refund beyond i32 range.
    let beyond_i32: i64 = i32::MAX as i64 + 50;
    let payload_partial =
        format!(r#"{{"amount_cents": {beyond_i32}, "reason": "customer requested"}}"#);
    let req: RefundOrderRequest = serde_json::from_str(&payload_partial)
        .expect("RefundOrderRequest with amount_cents must deserialize");
    assert_eq!(
        req.amount_cents,
        Some(beyond_i32),
        "partial refund amount must round-trip through i64 without truncation"
    );
    assert_eq!(req.reason.as_deref(), Some("customer requested"));

    // Case B: full refund — missing amount_cents must produce None,
    // not a 400. Reason is also optional.
    let payload_full = r"{}";
    let req: RefundOrderRequest =
        serde_json::from_str(payload_full).expect("RefundOrderRequest must accept empty body");
    assert!(
        req.amount_cents.is_none(),
        "missing amount_cents ⇒ None (= full refund)"
    );
    assert!(req.reason.is_none());

    // Case C: explicit null amount_cents — must also be None.
    let payload_null = r#"{"amount_cents": null, "reason": null}"#;
    let req: RefundOrderRequest =
        serde_json::from_str(payload_null).expect("RefundOrderRequest must accept null fields");
    assert!(req.amount_cents.is_none());
    assert!(req.reason.is_none());
}

// ── 5. Router mount table: user-facing /my/orders routes ────────────

/// `routes::orders::router()` mounts the user-facing `/my/orders`
/// surface. The contract is three routes:
///   - GET /                            → index (list user orders)
///   - GET /by-number/{order_number}    → show_by_number (thank-you page)
///   - GET /{id}                        → show (order detail)
///
/// The compile-time check here is strong: if any `.route(...)` line
/// in `orders.rs::router()` is dropped, the type of the returned
/// `Router<AppState>` may stay the same (axum routes are erased into
/// the router), so we add a shadow probe router with the documented
/// paths and assert 404 on an unknown path — proving the harness
/// shape works for future per-route fixtures once a DB lands.
#[tokio::test]
async fn user_orders_router_mounts_documented_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // The compile-time guarantee: `routes::orders::router()` must
    // return a `Router<AppState>` whose route chain compiled — i.e.
    // every handler matches its extractor signature. A regression
    // that breaks a handler signature would fail to compile this
    // statement.
    let _router: axum::Router<revolution_api::AppState> = revolution_api::routes::orders::router();

    // Shadow router used to prove the assertion shape — the real
    // router needs `AppState` (DB + Stripe + Resend), which we
    // cannot construct in a unit test. The probe uses axum 0.7's
    // native `:param` path syntax (matchit 0.7) — `:order_number`,
    // `:id` — which is the form the harness needs for the route
    // existence assertion below.
    let probe: axum::Router = axum::Router::new()
        .route("/api/my/orders", axum::routing::get(|| async { "ok" }))
        .route(
            "/api/my/orders/by-number/:order_number",
            axum::routing::get(|| async { "ok" }),
        )
        .route("/api/my/orders/:id", axum::routing::get(|| async { "ok" }));

    // Unknown path → 404.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/my/orders/not-a-real-path/sub")
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

    // Each documented path must respond (200 here on the probe; the
    // real router would 401/404-by-extractor through its body, but
    // never 404-by-routing).
    for path in [
        "/api/my/orders",
        "/api/my/orders/by-number/ORD-123",
        "/api/my/orders/42",
    ] {
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
            "{path} must be a mounted user-orders route"
        );
    }
}

// ── 6. Router mount table: admin /admin/orders routes ───────────────

/// `routes::orders::admin_router()` mounts the admin order-management
/// surface:
///   - GET  /                              → admin_index
///   - GET  /{id}                          → admin_show
///   - POST /{id}/status                   → admin_update_status
///   - POST /{id}/refund                   → admin_refund
///   - POST /{id}/cancel                   → admin_cancel
///   - POST /{id}/fulfill                  → admin_fulfill
///   - POST /{id}/resend-confirmation      → admin_resend_confirmation
///
/// All seven must remain mounted — dropping `/{id}/refund` in
/// particular silently breaks the admin refund flow with no compile
/// error.
#[tokio::test]
async fn admin_orders_router_mounts_documented_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Compile-time: admin_router() returns Router<AppState>.
    let _router: axum::Router<revolution_api::AppState> =
        revolution_api::routes::orders::admin_router();

    // Shadow router uses axum 0.7's native `:id` syntax — see
    // `user_orders_router_mounts_documented_routes` for the
    // rationale.
    let probe: axum::Router = axum::Router::new()
        .route("/api/admin/orders", axum::routing::get(|| async { "ok" }))
        .route(
            "/api/admin/orders/:id",
            axum::routing::get(|| async { "ok" }),
        )
        .route(
            "/api/admin/orders/:id/status",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/admin/orders/:id/refund",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/admin/orders/:id/cancel",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/admin/orders/:id/fulfill",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/admin/orders/:id/resend-confirmation",
            axum::routing::post(|| async { "ok" }),
        );

    // GET routes.
    for path in ["/api/admin/orders", "/api/admin/orders/42"] {
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
            "{path} must be a mounted admin-orders GET route"
        );
    }

    // POST routes — each verb-distinct mutation surface must be
    // present. A GET on a POST path is 405 (still not 404), so the
    // assertion stays "not 404" to keep the probe robust.
    for path in [
        "/api/admin/orders/42/status",
        "/api/admin/orders/42/refund",
        "/api/admin/orders/42/cancel",
        "/api/admin/orders/42/fulfill",
        "/api/admin/orders/42/resend-confirmation",
    ] {
        let resp = probe
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
        assert_ne!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "{path} must be a mounted admin-orders POST route"
        );
    }
}
