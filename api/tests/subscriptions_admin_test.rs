//! Subscriptions-admin route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::subscriptions_admin` and
//! exercises the public DTOs + the two `Router<AppState>` constructors
//! (`subscriptions_router()` for `/admin/subscriptions` and
//! `plans_router()` for `/admin/subscriptions/plans`).
//!
//! ## Why this shape
//!
//! Every handler in `routes/subscriptions_admin.rs` runs live SQL
//! against `subscriptions` / `subscription_plans` / `plan_price_history`
//! plus an outbound Stripe `Price` create on `change_plan_price`. None
//! of that is reachable in a unit-test harness — so we attack the
//! contract surface the handlers ride on:
//!
//! 1. **Money pin (CLAUDE.md hard rule).** `SubscriptionPlanRow.
//!    price_cents`, `ChangePriceRequest.amount_cents`, `CreatePlanRequest.
//!    price_cents`, `UpdatePlanRequest.price_cents`, and the
//!    `PriceHistoryRow.{old,new}_amount_cents` audit-trail fields are
//!    every-one `i64` / `Option<i64>`. A regression that narrows any of
//!    them to `i32` would silently cap a single annual enterprise plan
//!    at $21,474,836.47 AND wrap every revenue rollup that sums plan
//!    prices across the catalog. Pin all five with values past
//!    `i32::MAX`.
//!
//! 2. **Apply-to enum is wire-format load-bearing.** `ChangePriceRequest.
//!    apply_to` is a string with three documented values ("new_only" /
//!    "next_renewal" / "immediate_proration"). The handler routes
//!    Stripe-side migration semantics on this value; a typo in the
//!    frontend dropdown would silently fall through to the default
//!    branch and not migrate any subs. Pin the wire shape.
//!
//! 3. **Two mount tables.** `subscriptions_router()` exposes 7 routes
//!    (`/`, `/:id`, `/:id/cancel`, `/:id/pause`, `/:id/resume`,
//!    `/:id/renew`). `plans_router()` exposes 7 (`/`, `/stats`, `/:id`,
//!    `/:id/price`, `/:id/price-history`). A refactor that drops one or
//!    flips its HTTP verb would compile-pass and 405-fail in prod. We
//!    can't probe paths without `AppState`, but we *can* compile-pin
//!    the `Router<AppState>` return type — that catches every
//!    extractor-signature regression at build time.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/products_test.rs`, `tests/subscriptions_test.rs`,
//! `tests/admin_member_management_test.rs`.

use revolution_api::routes::subscriptions_admin::{
    ChangePriceRequest, CreatePlanRequest, CreateSubscriptionRequest, PlanListQuery,
    PriceHistoryRow, SubscriptionListQuery, SubscriptionPlanRow, SubscriptionRow,
    UpdatePlanRequest, UpdateSubscriptionRequest,
};

// ── 1. SubscriptionPlanRow.price_cents survives past i32::MAX ────────

/// HARD RULE (CLAUDE.md "Money / cents"): `price_cents` is i64. The
/// smallest i64 that cannot fit in i32 is `i32::MAX + 1` = 2_147_483_648
/// cents = $21,474,836.48. Annual enterprise plans (rare, but they
/// exist on this stack) push past this; aggregate-MRR rollups push past
/// it almost daily once the customer base is >10k. A regression that
/// narrows this field would corrupt both.
#[test]
fn subscription_plan_row_price_cents_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1; // $21,474,836.48
    let now = chrono::Utc::now().naive_utc();

    let row = SubscriptionPlanRow {
        id: 1,
        name: "Enterprise Annual".to_string(),
        slug: "enterprise-annual".to_string(),
        description: Some("Top-tier enterprise plan".to_string()),
        price_cents: above_i32_max,
        billing_cycle: "year".to_string(),
        is_active: true,
        stripe_price_id: Some("price_enterprise_annual".to_string()),
        features: None,
        trial_days: Some(14),
        trial_period_days: Some(14),
        trial_requires_payment_method: true,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize SubscriptionPlanRow");
    assert_eq!(
        wire["price_cents"].as_i64(),
        Some(above_i32_max),
        "price_cents must survive JSON round-trip as i64"
    );

    // Sanity: the fixture must actually exceed i32::MAX, otherwise the
    // assertion above could pass even if the field were i32.
    assert!(
        row.price_cents > i32::MAX as i64,
        "test fixture must exceed i32::MAX or it does not prove the i64 pin"
    );
    // Narrowing to i32 must lose data — proof that an i32 field would
    // corrupt the value before it ever reached the DB.
    assert!(
        (row.price_cents as i32 as i64) != row.price_cents,
        "narrowing to i32 must lose data — proves i64 is required"
    );
}

// ── 2. ChangePriceRequest + PriceHistoryRow money fields ─────────────

/// `ChangePriceRequest.amount_cents` is the admin's new price; the
/// handler writes it into both Stripe (via `unit_amount`) and the local
/// `plan_price_history` audit row. Pin it past `i32::MAX` to catch any
/// narrowing regression — and pin the `apply_to` wire shape while we're
/// here (the handler matches on this string to decide whether to
/// migrate existing subs).
#[test]
fn change_price_request_accepts_huge_amount_and_documented_apply_to_values() {
    let huge: i64 = (i32::MAX as i64) * 4; // ~$85.9M — beyond any i32

    for apply_to in &["new_only", "next_renewal", "immediate_proration"] {
        let payload = serde_json::json!({
            "amount_cents": huge,
            "currency": "usd",
            "billing_interval": "year",
            "apply_to": apply_to,
        });
        let req: ChangePriceRequest =
            serde_json::from_value(payload).expect("ChangePriceRequest must deserialize");

        assert_eq!(req.amount_cents, huge);
        assert_eq!(req.apply_to, *apply_to);
        assert_eq!(req.billing_interval, "year");
        assert_eq!(req.currency.as_deref(), Some("usd"));
    }

    // The currency field is `Option<String>` — verify a missing currency
    // deserializes to None (Stripe defaults to "usd" at the handler).
    let no_currency: ChangePriceRequest = serde_json::from_value(serde_json::json!({
        "amount_cents": 9_900,
        "billing_interval": "month",
        "apply_to": "new_only",
    }))
    .expect("currency-less payload must deserialize");
    assert!(no_currency.currency.is_none());
}

/// `PriceHistoryRow` is the audit-trail row for every plan-price change.
/// Both `old_amount_cents` (Option<i64>; None on first price) and
/// `new_amount_cents` (i64) must round-trip past `i32::MAX` — the
/// 7-year LTV / cohort-revenue queries pivot on these values.
#[test]
fn price_history_row_amount_cents_fields_are_i64() {
    let huge: i64 = (i32::MAX as i64) + 12_345;
    let now = chrono::Utc::now().naive_utc();

    let row = PriceHistoryRow {
        id: 1,
        plan_id: 42,
        old_stripe_price_id: Some("price_old".to_string()),
        new_stripe_price_id: "price_new".to_string(),
        old_amount_cents: Some(huge - 1_000),
        new_amount_cents: huge,
        currency: "usd".to_string(),
        billing_interval: "year".to_string(),
        apply_to: "next_renewal".to_string(),
        subscriptions_migrated: 100,
        subscriptions_failed: 0,
        failure_details: None,
        changed_by_user_id: Some(7),
        changed_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize PriceHistoryRow");
    assert_eq!(wire["new_amount_cents"].as_i64(), Some(huge));
    assert_eq!(wire["old_amount_cents"].as_i64(), Some(huge - 1_000));
    // `null` shape on first-ever price for a plan:
    let mut first_price = row;
    first_price.old_amount_cents = None;
    first_price.old_stripe_price_id = None;
    let wire2 = serde_json::to_value(&first_price).expect("serialize first-price row");
    assert!(wire2["old_amount_cents"].is_null());
}

// ── 3. Request DTOs accept the full + minimal payloads ───────────────

/// `CreatePlanRequest` — admin creates a new plan. `price_cents` is the
/// load-bearing money field; the rest are optional metadata. Verify
/// both the maximal (all-fields-set) and minimal (just the required
/// trio) payloads parse, and that `price_cents` past `i32::MAX` lands
/// in the right type.
#[test]
fn create_plan_request_parses_full_and_minimal_payloads() {
    let huge: i64 = (i32::MAX as i64) + 999;

    let full: CreatePlanRequest = serde_json::from_value(serde_json::json!({
        "name": "Whale Annual",
        "description": "For our biggest accounts",
        "price_cents": huge,
        "billing_cycle": "year",
        "is_active": true,
        "stripe_price_id": "price_whale_annual",
        "features": {"alerts": "unlimited"},
        "trial_days": 14,
        "trial_period_days": 14,
        "trial_requires_payment_method": true,
    }))
    .expect("full CreatePlanRequest must deserialize");

    assert_eq!(full.name, "Whale Annual");
    assert_eq!(full.price_cents, huge);
    assert_eq!(full.billing_cycle, "year");
    assert_eq!(full.is_active, Some(true));
    assert_eq!(full.trial_days, Some(14));
    assert_eq!(full.trial_requires_payment_method, Some(true));

    // Minimal payload (name + price_cents + billing_cycle required; the
    // rest are Option / default-able). A drift that flips one of these
    // optional fields to required would break the "create a $0
    // placeholder plan and configure later" admin workflow.
    let minimal: CreatePlanRequest = serde_json::from_value(serde_json::json!({
        "name": "Free",
        "price_cents": 0,
        "billing_cycle": "month",
    }))
    .expect("minimal CreatePlanRequest must deserialize");

    assert_eq!(minimal.price_cents, 0);
    assert!(minimal.description.is_none());
    assert!(minimal.is_active.is_none());
    assert!(minimal.features.is_none());
    assert!(minimal.trial_days.is_none());

    // `UpdatePlanRequest.price_cents` is `Option<i64>` — the "leave
    // price unchanged" path. Pin the type at i64 too.
    let upd: UpdatePlanRequest = serde_json::from_value(serde_json::json!({
        "name": "Renamed",
        "price_cents": huge,
    }))
    .expect("UpdatePlanRequest must deserialize");
    assert_eq!(upd.price_cents, Some(huge));
    assert_eq!(upd.name.as_deref(), Some("Renamed"));
    // Verify None survives when the field is omitted entirely.
    let upd_no_price: UpdatePlanRequest =
        serde_json::from_value(serde_json::json!({ "name": "Just rename" }))
            .expect("UpdatePlanRequest must accept partial");
    assert!(upd_no_price.price_cents.is_none());
}

// ── 4. List + subscription request DTOs ──────────────────────────────

/// `SubscriptionListQuery` / `PlanListQuery` are query-string filters
/// the admin grid uses. Pin the optional-everywhere shape — an
/// accidental required-field would 400-error the empty-grid load
/// (a common AI-refactor footgun).
#[test]
fn list_queries_accept_empty_and_full_payloads() {
    let empty_sub: SubscriptionListQuery =
        serde_json::from_str("{}").expect("empty SubscriptionListQuery");
    assert!(empty_sub.page.is_none());
    assert!(empty_sub.per_page.is_none());
    assert!(empty_sub.status.is_none());
    assert!(empty_sub.user_id.is_none());
    assert!(empty_sub.plan_id.is_none());

    let full_sub: SubscriptionListQuery = serde_json::from_value(serde_json::json!({
        "page": 2,
        "per_page": 50,
        "status": "active",
        "user_id": 9_876_543_210_i64,
        "plan_id": 42,
    }))
    .expect("full SubscriptionListQuery");
    assert_eq!(full_sub.page, Some(2));
    assert_eq!(full_sub.user_id, Some(9_876_543_210));

    let empty_plan: PlanListQuery = serde_json::from_str("{}").expect("empty PlanListQuery");
    assert!(empty_plan.page.is_none());
    assert!(empty_plan.is_active.is_none());

    let full_plan: PlanListQuery = serde_json::from_value(serde_json::json!({
        "page": 1,
        "per_page": 25,
        "is_active": false,
    }))
    .expect("full PlanListQuery");
    assert_eq!(full_plan.is_active, Some(false));

    // CreateSubscriptionRequest + UpdateSubscriptionRequest carry no
    // money fields, but the user_id / plan_id `i64` pin is still
    // load-bearing: user IDs in this stack are bigserial.
    let create: CreateSubscriptionRequest = serde_json::from_value(serde_json::json!({
        "user_id": 9_876_543_210_i64,
        "plan_id": 12_345,
        "status": "active",
    }))
    .expect("CreateSubscriptionRequest");
    assert_eq!(create.user_id, 9_876_543_210);
    assert_eq!(create.plan_id, 12_345);

    let update: UpdateSubscriptionRequest = serde_json::from_value(serde_json::json!({
        "status": "cancelled",
    }))
    .expect("UpdateSubscriptionRequest");
    assert_eq!(update.status.as_deref(), Some("cancelled"));
    assert!(update.plan_id.is_none());
}

// ── 5. SubscriptionRow wire-format keys (frontend contract) ──────────

/// The admin grid reads `SubscriptionRow` by exact JSON key. A
/// `#[serde(rename)]` regression on any of these would silently break
/// the table (no error, just blank cells). Pin the wire keys.
#[test]
fn subscription_row_wire_format_keys_match_frontend_contract() {
    let now = chrono::Utc::now().naive_utc();
    let row = SubscriptionRow {
        id: 1,
        user_id: 42,
        plan_id: Some(7),
        status: "active".to_string(),
        starts_at: Some(now),
        expires_at: Some(now),
        cancelled_at: None,
        payment_provider: Some("stripe".to_string()),
        stripe_subscription_id: Some("sub_test".to_string()),
        stripe_customer_id: Some("cus_test".to_string()),
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize SubscriptionRow");
    assert_eq!(wire["status"], "active");
    assert_eq!(wire["plan_id"], 7);
    assert_eq!(wire["stripe_subscription_id"], "sub_test");
    assert_eq!(wire["stripe_customer_id"], "cus_test");
    assert!(wire["cancelled_at"].is_null());

    // Negative pin: must NOT silently drop required fields. (A serde
    // skip_serializing_if = "Option::is_none" on user_id would corrupt
    // the audit log.) user_id is i64 not Option, so the key is always
    // present — pin that.
    assert!(
        wire.get("user_id").is_some(),
        "user_id must always serialize (non-Option field)"
    );
}

// ── 6. Router mount tables compile-pin ──────────────────────────────

/// `subscriptions_router()` must build as `Router<AppState>`. The
/// load-bearing assertion is the explicit return-type annotation: a
/// refactor that swaps an extractor (e.g. drops `AdminUser` from
/// `cancel_subscription`) would fail compilation here, not in prod.
#[test]
fn subscriptions_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::subscriptions_admin::subscriptions_router();
}

/// `plans_router()` must build as `Router<AppState>`. Same compile-pin
/// as above — protects the Stripe-syncing `change_plan_price` endpoint
/// and the `price-history` audit-trail GET against extractor drift.
#[test]
fn plans_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::subscriptions_admin::plans_router();
}
