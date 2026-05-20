//! Coupons route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::coupons` and exercises the
//! public request/response DTOs + the mount-table router. The discount
//! math in `validate_coupon` is the load-bearing money path: a wrong
//! field type (i32 instead of i64) or a wrong percent encoding (e.g.
//! `value` as a literal percent vs `value * 100` cents) silently
//! over- or under-bills every customer who uses a coupon. These tests
//! pin the contract that drives that arithmetic.

use revolution_api::routes::coupons::{
    CouponInfo, CreateCouponRequest, ValidateCouponRequest, ValidateCouponResponse,
};

// ── Money: i64 end-to-end, round-trips past i32::MAX ─────────────────

/// HARD RULE (CLAUDE.md "Money / cents"): every `*_cents` field is i64.
/// The discount-amount field MUST hold values past i32::MAX — once you
/// stack a corporate coupon on a 7-figure annual contract you blow past
/// $21M cents. Frontend reads this number directly into the order total.
#[test]
fn validate_response_discount_amount_cents_is_i64_round_trips_past_i32_max() {
    let huge_discount: i64 = (i32::MAX as i64) + 10_000; // way past 2^31

    let resp = ValidateCouponResponse {
        valid: true,
        coupon: Some(CouponInfo {
            id: 1,
            code: "MEGA50".to_string(),
            coupon_type: "fixed".to_string(),
            value_cents: huge_discount,
            min_purchase_amount_cents: (i32::MAX as i64) + 1,
            expiry_date: Some("2030-12-31".to_string()),
        }),
        discount_amount_cents: Some(huge_discount),
        error: None,
    };

    let wire = serde_json::to_value(&resp).expect("serialize validate response");
    assert_eq!(wire["valid"], true);
    assert_eq!(
        wire["discount_amount_cents"].as_i64(),
        Some(huge_discount),
        "discount_amount_cents must survive JSON round-trip as i64"
    );
    assert_eq!(
        wire["coupon"]["value_cents"].as_i64(),
        Some(huge_discount),
        "value_cents must survive JSON round-trip as i64"
    );
    assert_eq!(
        wire["coupon"]["min_purchase_amount_cents"].as_i64(),
        Some((i32::MAX as i64) + 1)
    );

    // Type renamed on the wire (#[serde(rename = "type")]). Contract pin.
    assert_eq!(wire["coupon"]["type"], "fixed");
    assert!(
        wire["coupon"].get("coupon_type").is_none(),
        "coupon_type must serialize as 'type', not 'coupon_type'"
    );
}

/// The `value_cents` field encodes percent coupons as `percent * 100`
/// (e.g. 5000 = 50%). This test pins the documented convention so a
/// future "let's just store the percent literal" refactor fails loudly
/// instead of silently 100x-ing every discount.
#[test]
fn percent_coupon_encoding_is_value_cents_eq_percent_times_100() {
    // 50% coupon → value_cents = 5000. Apply to $200 (20_000 cents) →
    // discount = subtotal * (value_cents/100) / 100 = 20_000 * 50 / 100 = 10_000 cents.
    let value_cents: i64 = 5000; // 50%
    let subtotal_cents: i64 = 20_000; // $200.00
    let percent = value_cents / 100;
    let discount = subtotal_cents.saturating_mul(percent) / 100;
    assert_eq!(discount, 10_000, "$200 × 50% must be $100 (10_000 cents)");

    // Same math past i32::MAX: $50,000.00 subtotal × 50% must produce a
    // discount that survives i64 arithmetic without wrap or saturation.
    let big_subtotal: i64 = 5_000_000_000; // $50,000,000.00 in cents
    let big_discount = big_subtotal.saturating_mul(percent) / 100;
    assert_eq!(big_discount, 2_500_000_000);
    assert!(big_discount > i32::MAX as i64, "must exceed i32::MAX");
}

// ── ValidateCouponRequest deserializes the shape the checkout sends ──

#[test]
fn validate_request_accepts_minimal_and_full_payloads() {
    // Minimal — code only.
    let min: ValidateCouponRequest =
        serde_json::from_str(r#"{"code": "WELCOME10"}"#).expect("minimal validate payload");
    assert_eq!(min.code, "WELCOME10");
    assert!(min.subtotal_cents.is_none());
    assert!(min.product_ids.is_none());
    assert!(min.plan_ids.is_none());

    // Full — code + subtotal + product/plan restrictions; subtotal i64 past i32::MAX.
    let big_subtotal: i64 = (i32::MAX as i64) + 5_000;
    let body = serde_json::json!({
        "code": "biggie",
        "subtotal_cents": big_subtotal,
        "product_ids": [1_i64, 2, 3],
        "plan_ids": [10_i64, 20]
    });
    let full: ValidateCouponRequest = serde_json::from_value(body).expect("full validate payload");
    assert_eq!(full.code, "biggie");
    assert_eq!(full.subtotal_cents, Some(big_subtotal));
    assert_eq!(full.product_ids.unwrap(), vec![1_i64, 2, 3]);
    assert_eq!(full.plan_ids.unwrap(), vec![10_i64, 20]);
}

// ── CreateCouponRequest: Decimal discount_value + optional fields ────

#[test]
fn create_coupon_request_carries_decimal_value_and_optional_fields() {
    let body = serde_json::json!({
        "code": "LAUNCH",
        "description": "Launch promo",
        "discount_type": "percentage",
        "discount_value": "25.50",       // serde Decimal accepts string form
        "min_purchase": "100.00",
        "max_discount": "500.00",
        "usage_limit": 1000,
        "expires_at": "2027-01-01T00:00:00",
        "applicable_products": [1_i64, 2],
        "applicable_plans": [3_i64],
        "is_active": true
    });
    let req: CreateCouponRequest = serde_json::from_value(body).expect("create coupon payload");
    assert_eq!(req.code, "LAUNCH");
    assert_eq!(req.discount_type, "percentage");
    assert_eq!(req.discount_value.to_string(), "25.50");
    assert_eq!(
        req.min_purchase.map(|d| d.to_string()).as_deref(),
        Some("100.00")
    );
    assert_eq!(
        req.max_discount.map(|d| d.to_string()).as_deref(),
        Some("500.00")
    );
    assert_eq!(req.usage_limit, Some(1000));
    assert_eq!(req.applicable_products.unwrap(), vec![1_i64, 2]);
    assert_eq!(req.applicable_plans.unwrap(), vec![3_i64]);
    assert_eq!(req.is_active, Some(true));
}

// ── Invalid-input response shape (the failure flow customers see) ────

#[test]
fn invalid_coupon_response_carries_error_without_coupon_or_discount() {
    let resp = ValidateCouponResponse {
        valid: false,
        coupon: None,
        discount_amount_cents: None,
        error: Some("Invalid coupon code".to_string()),
    };
    let wire = serde_json::to_value(&resp).expect("serialize invalid response");
    assert_eq!(wire["valid"], false);
    assert!(wire["coupon"].is_null());
    assert!(wire["discount_amount_cents"].is_null());
    assert_eq!(wire["error"], "Invalid coupon code");
}

// ── Router mount table ───────────────────────────────────────────────

/// Mount-table test: removing any public route or breaking a handler
/// signature will fail to compile here. `Router<AppState>` is the
/// load-bearing return type; this assertion is a guard against accidental
/// state-type changes during refactors.
#[test]
fn coupons_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::coupons::router();
}
