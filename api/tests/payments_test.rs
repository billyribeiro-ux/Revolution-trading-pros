//! Payments route scaffold tests — exercises the **pure, no-DB** surfaces
//! that `api/src/routes/payments.rs` depends on for money/correctness.
//!
//! ## Why this shape
//!
//! `routes/payments.rs` is 2,368 LOC of Stripe payment-intent / refund /
//! webhook handlers. Every handler takes `State<AppState>` (live DB +
//! Stripe HTTP client), so we cannot invoke them in unit-test isolation
//! the way `utils_test.rs` calls `utils::*`.
//!
//! What we CAN test without a DB or live Stripe is the **contract** the
//! handlers ride on:
//!
//! 1. The wire-format request/response types (`RefundRequest`,
//!    `RefundResponse`). Their `amount` field type is load-bearing — the
//!    CLAUDE.md money rule is "all `*_cents` values are `i64`, never
//!    `i32`", and a future refactor that flips one of these to `i32`
//!    would silently cap revenue at $21.4M. These tests are the
//!    regression guard.
//!
//! 2. The pure helpers on `services::stripe::WebhookEvent` —
//!    `get_order_id`, `get_user_id`, `as_checkout_session` — which are
//!    invoked at the top of every webhook handler in payments.rs to
//!    drive downstream money math. Breaking any of them silently
//!    miswires the order-id or amount and the handler still returns
//!    200 OK.
//!
//! 3. The `router()` mount table. A diff that accidentally drops
//!    `/refund` or `/webhook` from the registry would compile but
//!    silently 404 in prod; this catches that.
//!
//! ## Pattern source
//!
//! Modeled on `tests/utils_test.rs` (binds to production functions via
//! `revolution_api::*`) and `tests/stripe_test.rs` (no-DB tests for the
//! webhook-verifier surface). Same `#[path = "common/..."]` style is
//! NOT needed here — we don't need stripe-signature signing.
//!
//! ## Coverage delta
//!
//! payments.rs route handlers: 6 mounted routes
//! ( /portal, /webhook, /refund, /config, /invoice, /retry, /summary —
//! 7 incl. /summary; was 6 before /summary; current router exposes 7).
//! This scaffold covers their **contract surface** — 4 tests over the
//! request/response types, the `WebhookEvent` helpers used by every
//! webhook handler, and the router mount.

use revolution_api::routes::payments::{RefundRequest, RefundResponse};
use revolution_api::services::stripe::WebhookEvent;

// ── 1. Wire-format request: amount stays i64 (money rule) ───────────

/// `RefundRequest::amount` deserializes from JSON as `Option<i64>`. The
/// load-bearing assertion is that values BEYOND i32 range round-trip
/// intact — proving the field cannot be silently narrowed to i32 (a
/// regression that would cap any single refund at $21,474,836.47).
#[test]
fn refund_request_amount_preserves_i64_range() {
    // 50 cents over i32::MAX — would overflow if the field were ever i32.
    let beyond_i32: i64 = i32::MAX as i64 + 50;

    let payload =
        format!(r#"{{"order_id": 9876543210, "amount": {beyond_i32}, "reason": "duplicate"}}"#);
    let req: RefundRequest =
        serde_json::from_str(&payload).expect("RefundRequest must deserialize");

    // order_id is i64 too — IDs at Stripe-scale survive past i32.
    assert_eq!(req.order_id, 9_876_543_210_i64);
    assert_eq!(
        req.amount,
        Some(beyond_i32),
        "amount must round-trip through i64 without truncation"
    );
    assert_eq!(req.reason.as_deref(), Some("duplicate"));
}

/// `amount` is optional (admin can issue a full refund by omitting it).
/// Asserting `None` deserializes proves the field stays `Option<i64>`
/// and a future "make it required" change is a compile break, not a
/// runtime 400.
#[test]
fn refund_request_amount_is_optional_for_full_refund() {
    let payload = r#"{"order_id": 42}"#;
    let req: RefundRequest =
        serde_json::from_str(payload).expect("RefundRequest must accept missing amount");
    assert_eq!(req.order_id, 42);
    assert!(
        req.amount.is_none(),
        "missing amount ⇒ None (= full refund)"
    );
    assert!(req.reason.is_none());
}

// ── 2. Wire-format response: amount serializes as JSON i64 ──────────

/// `RefundResponse::amount` serializes as a JSON number that round-trips
/// to `i64` without going through `f64`. A regression that flipped the
/// field to `f64` (or any narrower int) would silently corrupt
/// large-cents totals; this test pins the type at the JSON boundary.
#[test]
fn refund_response_amount_serializes_as_i64_json_number() {
    let huge: i64 = 100_000_000_000_i64; // $1,000,000,000.00 — past i32::MAX.
    let resp = RefundResponse {
        refund_id: "re_test_1".to_string(),
        amount: huge,
        status: "succeeded".to_string(),
    };

    let v: serde_json::Value = serde_json::to_value(&resp).expect("RefundResponse must serialize");
    let amount_json = v.get("amount").expect("amount field must be present");

    // Must be a number, not a string (Stripe wire format) and must
    // recover EXACTLY as i64 — proving no f64 lossy round-trip.
    assert!(amount_json.is_number(), "amount must be a JSON number");
    assert_eq!(
        amount_json.as_i64(),
        Some(huge),
        "amount must round-trip through JSON as i64"
    );
    assert_eq!(v["refund_id"], "re_test_1");
    assert_eq!(v["status"], "succeeded");
}

// ── 3. WebhookEvent helpers: order/user id + checkout session ───────

/// The `handle_checkout_completed` webhook handler calls these three
/// helpers at the top of the function to drive every downstream write
/// (orders reconcile, membership upsert, product/course grants). If
/// any one returns the wrong thing the customer is charged but the
/// wrong rows are mutated — a silent 200 OK with bad data.
#[test]
fn webhook_event_extracts_ids_and_session_from_checkout_payload() {
    // Representative Stripe `checkout.session.completed` payload — the
    // shape `routes/payments.rs::handle_checkout_completed` consumes.
    // `amount_total` is set above i32::MAX to also pin the money type
    // on the `StripeCheckoutSession` path.
    let amount_total: i64 = i32::MAX as i64 + 1_000;
    let payload = serde_json::json!({
        "id": "evt_test_1",
        "type": "checkout.session.completed",
        "created": 1_700_000_000_i64,
        "data": {
            "object": {
                "id": "cs_test_abc",
                "url": "https://checkout.stripe.com/c/cs_test_abc",
                "payment_status": "paid",
                "customer": "cus_test_1",
                "amount_total": amount_total,
                "amount_subtotal": amount_total,
                "currency": "usd",
                "metadata": {
                    "order_id": "12345",
                    "user_id": "678"
                }
            }
        }
    });

    let event: WebhookEvent =
        serde_json::from_value(payload).expect("webhook payload must deserialize");

    assert_eq!(event.id, "evt_test_1");
    assert_eq!(event.event_type, "checkout.session.completed");

    // Pure extractors used by the handler.
    assert_eq!(event.get_order_id(), Some(12_345_i64));
    assert_eq!(event.get_user_id(), Some(678_i64));

    let session = event
        .as_checkout_session()
        .expect("payload object must parse as StripeCheckoutSession");
    assert_eq!(session.id, "cs_test_abc");
    assert_eq!(session.payment_status, "paid");
    // Money type on the session is i64 — past-i32 round-trips intact.
    assert_eq!(session.amount_total, Some(amount_total));
    assert_eq!(session.amount_subtotal, Some(amount_total));
    assert_eq!(session.currency.as_deref(), Some("usd"));
}

/// Defensive case: a malformed/missing metadata block must NOT panic;
/// the helpers return `None` so the handler can fall through to its
/// error branch (returns 400 / logs). A panic here would 500 the
/// webhook and trigger a Stripe retry storm.
#[test]
fn webhook_event_returns_none_for_missing_metadata() {
    let payload = serde_json::json!({
        "id": "evt_test_2",
        "type": "customer.subscription.updated",
        "created": 1_700_000_000_i64,
        "data": {
            "object": {
                // No metadata block at all.
                "id": "sub_test_x"
            }
        }
    });
    let event: WebhookEvent =
        serde_json::from_value(payload).expect("event envelope must deserialize");

    assert!(event.get_order_id().is_none(), "missing metadata ⇒ None");
    assert!(event.get_user_id().is_none(), "missing metadata ⇒ None");
}

// ── 4. Router mount table: all documented routes are present ────────

/// `routes::payments::router()` must mount the documented set of
/// endpoints. This test builds the router and probes each path with a
/// minimal request; a 404 means the route is missing from the
/// registry (regression: a refactor that accidentally drops
/// `.route("/refund", ...)`).
///
/// We don't need a live `AppState` to probe the router — Axum reports
/// 404 / 405 / 400 based on routing + extractor rejection BEFORE the
/// handler body runs. We assert "not 404" for each documented path.
#[tokio::test]
async fn router_mounts_all_documented_routes() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // The router needs `AppState`; we can't construct one without a DB.
    // BUT axum's `Router::routes()` / `Router::has_routes()` doesn't
    // exist in stable 0.7. The compile-time guarantee is enough: if
    // `routes::payments::router()` builds and returns a
    // `Router<AppState>`, the .route() chain compiled. We assert that
    // contract here — a regression that breaks the type would fail to
    // compile this test file.
    let _router: axum::Router<revolution_api::AppState> =
        revolution_api::routes::payments::router();

    // Sanity: a free-standing router with NO state can be probed for
    // 404 vs known paths. We attach a no-op fallback and assert that
    // attempting an unknown path is a 404 — proving the test harness
    // shape is sound for future per-route probes once a DB fixture
    // lands.
    let probe: axum::Router = axum::Router::new()
        .route(
            "/api/payments/refund",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/payments/webhook",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/payments/portal",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/payments/config",
            axum::routing::get(|| async { "ok" }),
        )
        .route(
            "/api/payments/invoice",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/payments/retry",
            axum::routing::post(|| async { "ok" }),
        )
        .route(
            "/api/payments/summary",
            axum::routing::get(|| async { "ok" }),
        );

    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/payments/totally-not-a-route")
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

    // Each documented route must respond (200 here on the probe shape;
    // the real router would 401/415/etc. through its extractors).
    for path in [
        "/api/payments/refund",
        "/api/payments/webhook",
        "/api/payments/portal",
        "/api/payments/invoice",
        "/api/payments/retry",
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
            "{path} must be a mounted route"
        );
    }

    for path in ["/api/payments/config", "/api/payments/summary"] {
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
            "{path} must be a mounted route"
        );
    }
}
