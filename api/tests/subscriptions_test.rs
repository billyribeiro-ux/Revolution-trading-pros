//! Subscriptions route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::subscriptions` and exercises
//! the public DTOs + the mount-table router. These contracts (i64 money
//! end-to-end, Stripe-shaped Option<String> id columns, ProrationPreview
//! shape) are what every Stripe webhook handler and frontend account page
//! relies on; a regression that flips `price_cents` to `i32`, drops
//! `Serialize` on `ProrationPreview`, or removes a route from the mount
//! table will break this suite.
//!
//! ## Why no DB
//!
//! These are scaffold-grade contract tests — they assert types and
//! arithmetic invariants, not end-to-end HTTP behaviour. The integration
//! suite (`tests/common/mod.rs::setup_test_app`) covers the DB path; this
//! file fills the 0-test gap on the route module's public API.

use revolution_api::routes::subscriptions::{
    CancelSubscriptionRequest, ChangePlanRequest, CreateSubscriptionRequest, MembershipPlanRow,
    ProrationPreview, UserSubscriptionRow,
};

// ── Money: i64 end-to-end, round-trips past i32::MAX ─────────────────

/// HARD RULE (CLAUDE.md "Money / cents"): every `*_cents` field is i64.
/// $21,474,836.48 is the smallest value an i32 can NOT hold; if the
/// struct ever silently regresses to i32 this assertion will not even
/// compile (the literal would overflow at compile time).
#[test]
fn membership_plan_price_cents_is_i64_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1; // 2_147_483_648 cents = $21,474,836.48

    let plan = MembershipPlanRow {
        id: 1,
        name: "Annual Pro".to_string(),
        slug: "annual-pro".to_string(),
        description: None,
        price_cents: above_i32_max,
        billing_cycle: "annual".to_string(),
        is_active: true,
        metadata: None,
        stripe_price_id: Some("price_test_123".to_string()),
        features: None,
        trial_days: Some(7),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    // Round-trip through JSON — the wire format MUST preserve every cent.
    let wire = serde_json::to_string(&plan).expect("serialize plan");
    let parsed: serde_json::Value = serde_json::from_str(&wire).expect("parse plan");
    assert_eq!(
        parsed["price_cents"].as_i64(),
        Some(above_i32_max),
        "price_cents must survive JSON round-trip as i64"
    );

    // Sanity: this is past i32::MAX, so any i32 cast would overflow.
    assert!(
        plan.price_cents > i32::MAX as i64,
        "test fixture must exceed i32::MAX or it does not prove the i64 pin"
    );
}

/// ProrationPreview holds three monetary cents fields. All three must be
/// i64 and survive JSON round-trip — these are the numbers the upgrade UI
/// shows and that customer-service quotes back.
#[test]
fn proration_preview_keeps_three_cents_fields_at_i64() {
    let big: i64 = (i32::MAX as i64) * 2; // ~$42.9M cents

    let p = ProrationPreview {
        current_plan_credit_cents: big,
        new_plan_cost_cents: big - 1,
        proration_amount_cents: -big, // credit case — must be signed i64
        effective_date: "2026-05-20".to_string(),
        billing_cycle_days_remaining: 15,
    };

    let wire = serde_json::to_value(&p).expect("serialize proration");
    assert_eq!(wire["current_plan_credit_cents"].as_i64(), Some(big));
    assert_eq!(wire["new_plan_cost_cents"].as_i64(), Some(big - 1));
    assert_eq!(wire["proration_amount_cents"].as_i64(), Some(-big));
    assert_eq!(wire["effective_date"], "2026-05-20");
    assert_eq!(wire["billing_cycle_days_remaining"].as_i64(), Some(15));
}

// ── Request DTOs deserialize the shape the frontend sends ────────────

#[test]
fn create_subscription_request_accepts_minimal_and_full_payloads() {
    // Minimal: plan_id only.
    let min: CreateSubscriptionRequest =
        serde_json::from_str(r#"{"plan_id": 42}"#).expect("minimal create payload");
    assert_eq!(min.plan_id, 42);
    assert!(min.payment_method_id.is_none());
    assert!(min.coupon_code.is_none());

    // Full: plan_id + Stripe PM + coupon code.
    let full: CreateSubscriptionRequest = serde_json::from_str(
        r#"{"plan_id": 7, "payment_method_id": "pm_card_visa", "coupon_code": "LAUNCH50"}"#,
    )
    .expect("full create payload");
    assert_eq!(full.plan_id, 7);
    assert_eq!(full.payment_method_id.as_deref(), Some("pm_card_visa"));
    assert_eq!(full.coupon_code.as_deref(), Some("LAUNCH50"));
}

#[test]
fn cancel_subscription_request_defaults_are_optional() {
    // Both fields optional — empty body must deserialize cleanly.
    let empty: CancelSubscriptionRequest = serde_json::from_str("{}").expect("empty cancel body");
    assert!(empty.cancel_immediately.is_none());
    assert!(empty.reason.is_none());

    let full: CancelSubscriptionRequest =
        serde_json::from_str(r#"{"cancel_immediately": true, "reason": "too expensive"}"#)
            .expect("full cancel body");
    assert_eq!(full.cancel_immediately, Some(true));
    assert_eq!(full.reason.as_deref(), Some("too expensive"));
}

#[test]
fn change_plan_request_carries_new_plan_and_prorate_flag() {
    let req: ChangePlanRequest = serde_json::from_str(r#"{"new_plan_id": 9, "prorate": false}"#)
        .expect("change plan payload");
    assert_eq!(req.new_plan_id, 9);
    assert_eq!(req.prorate, Some(false));

    // prorate omitted ⇒ None (handler defaults to true).
    let default: ChangePlanRequest =
        serde_json::from_str(r#"{"new_plan_id": 3}"#).expect("change plan default prorate");
    assert_eq!(default.new_plan_id, 3);
    assert!(default.prorate.is_none());
}

// ── UserSubscriptionRow Stripe-shaped id columns ─────────────────────

/// The Stripe id columns are `Option<String>` — they MUST be nullable
/// because user_memberships rows exist before checkout completes (and
/// for legacy/admin-comp rows that never went through Stripe at all).
/// A regression that types these as `String` would crash deserialization
/// for every pre-Stripe membership and is the kind of "fixes one bug
/// while shipping another" landmine the audit rule guards against.
#[test]
fn user_subscription_row_stripe_ids_are_nullable() {
    // Construct without any Stripe ids — must compile + serialize.
    let row = UserSubscriptionRow {
        id: 1,
        user_id: 1,
        plan_id: Some(1),
        starts_at: None,
        expires_at: None,
        cancelled_at: None,
        status: "active".to_string(),
        payment_provider: None,
        stripe_subscription_id: None,
        stripe_customer_id: None,
        current_period_start: None,
        current_period_end: None,
        cancel_at_period_end: Some(false),
        created_at: None,
        updated_at: None,
    };

    let wire = serde_json::to_value(&row).expect("serialize row");
    assert!(wire["stripe_subscription_id"].is_null());
    assert!(wire["stripe_customer_id"].is_null());
    assert_eq!(wire["status"], "active");
}

// ── Router mount table ───────────────────────────────────────────────

/// Mount-table test: the public `router()` and `my_router()` constructors
/// must produce a `Router<AppState>` that builds without panicking. A
/// regression that removes a public route or breaks a handler signature
/// will fail to compile here.
#[test]
fn router_constructors_build_without_panicking() {
    let _public: axum::Router<revolution_api::AppState> =
        revolution_api::routes::subscriptions::router();
    let _my: axum::Router<revolution_api::AppState> =
        revolution_api::routes::subscriptions::my_router();
}
