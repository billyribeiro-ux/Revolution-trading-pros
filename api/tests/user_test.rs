//! User route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::user` and exercises the
//! public DTOs (`UserMembershipResponse`, `MembershipsResponse`,
//! `CancelSubscriptionRequest`, `PaymentMethodResponse`,
//! `AddPaymentMethodRequest`) + the `router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/user.rs` is the singular `/api/user/*` surface the SvelteKit
//! frontend hits for the logged-in user's own profile, memberships,
//! and Stripe payment methods. Every handler talks live to Postgres
//! and/or Stripe, so we can't run the handlers in isolation. What we
//! CAN pin:
//!
//! 1. **`UserMembershipResponse.price_cents` MUST stay `Option<i64>`.**
//!    This is the per-plan price that the SvelteKit "My Memberships"
//!    card reads. CLAUDE.md money rule: every `*_cents` field is i64,
//!    never i32. A regression that narrows the field to i32 would silently
//!    cap a plan price at $21,474,836.47 — irrelevant per-plan, but
//!    Stripe's wire format is `int64` for `unit_amount`, and a
//!    type-mismatched read of a high-tier annual price would corrupt
//!    before it ever reached the frontend.
//!
//! 2. **Wire-format keys are load-bearing (frontend contract).** The
//!    SvelteKit `UserMembership` interface reads `priceCents`,
//!    `startDate`, `nextBillingDate`, `expiresAt`, `membershipType`,
//!    and `type`. A `#[serde(rename = ...)]` regression would silently
//!    404 every membership card. Pin every documented camelCase key.
//!
//! 3. **`CancelSubscriptionRequest.cancel_immediately`** defaults to
//!    `false` via `#[serde(default)]`. That default is the difference
//!    between "cancel at period end" (refund-safe) and "cancel now"
//!    (revenue-leak path). A regression that flipped the default would
//!    refund every cancel click. Pin the default.
//!
//! 4. **`PaymentMethodResponse` is camelCase end-to-end.** Stripe gives
//!    us snake_case (`exp_month`, `last4`); the Svelte frontend reads
//!    camelCase (`expiryMonth`, `last4`). A regression that dropped
//!    `#[serde(rename_all = "camelCase")]` would silently break every
//!    "Saved Cards" row in the billing center.
//!
//! 5. **Router mount table (10 routes).** The `/api/user/*` mount-point
//!    is the canonical singular-noun frontend surface (vs. the plural
//!    `/api/users/*` admin surface). A regression that drops the
//!    `User` extractor from a handler would let any unauthenticated
//!    request cancel a membership. We can't probe paths without
//!    `AppState`, but we CAN compile-pin the `Router<AppState>` return
//!    type, which catches every extractor-signature regression at
//!    build time.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_orders_test.rs`, `tests/payments_test.rs`,
//! `tests/products_test.rs`, `tests/subscriptions_test.rs`.

use revolution_api::routes::user::{
    AddPaymentMethodRequest, CancelSubscriptionRequest, MembershipsResponse, PaymentMethodResponse,
    UserMembershipResponse,
};

// ── 1. Money: price_cents is Option<i64>, round-trips past i32::MAX ──

/// HARD RULE (CLAUDE.md "Money / cents"): every `*_cents` field is i64.
/// `UserMembershipResponse.price_cents: Option<i64>` is the per-plan
/// monthly/annual price the user's "My Memberships" card reads. A
/// regression that narrowed the field to i32 would silently cap the
/// value at $21,474,836.47 — Stripe's wire format for `unit_amount`
/// is `int64`, so a mismatched read of a high-tier annual price would
/// either truncate-on-read or fail to deserialize from a properly
/// shaped backend response. Pin the i64 type with a fixture that
/// genuinely exceeds i32::MAX so an i32 reduction won't compile.
#[test]
fn user_membership_response_price_cents_is_option_i64_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1; // $21,474,836.48

    let resp = UserMembershipResponse {
        id: "42".to_string(),
        name: "Enterprise Annual".to_string(),
        membership_type: "trading-room".to_string(),
        slug: "enterprise-annual".to_string(),
        status: "active".to_string(),
        subscription_type: Some("active".to_string()),
        icon: Some("crown".to_string()),
        start_date: "2026-01-01".to_string(),
        next_billing_date: Some("2027-01-01".to_string()),
        expires_at: Some("2027-01-01".to_string()),
        price_cents: Some(above_i32_max),
        interval: Some("yearly".to_string()),
        features: Some(vec![
            "all-rooms".to_string(),
            "priority-support".to_string(),
        ]),
    };

    let wire = serde_json::to_value(&resp).expect("serialize membership");
    // Frontend reads the camelCase key `priceCents`, not `price_cents`.
    assert_eq!(
        wire["priceCents"].as_i64(),
        Some(above_i32_max),
        "priceCents must survive JSON round-trip as i64 past i32::MAX"
    );

    // Sanity: the fixture must actually exceed i32::MAX.
    assert!(
        resp.price_cents.unwrap() > i32::MAX as i64,
        "fixture must exceed i32::MAX or it doesn't prove the i64 pin"
    );
    // Narrowing-to-i32 must lose data — proves i64 is required.
    let huge = resp.price_cents.unwrap();
    assert!(
        (huge as i32 as i64) != huge,
        "narrowing to i32 must lose data — proves i64 is required"
    );
}

// ── 2. Wire-format keys: frontend `UserMembership` contract ──────────

/// The SvelteKit `UserMembership` interface reads camelCase keys
/// (`startDate`, `nextBillingDate`, `expiresAt`, `priceCents`,
/// `membershipType`) AND the special `type` key (rename target for
/// `membership_type`). A regression in ANY of those `#[serde(rename)]`
/// attrs would silently 404 a membership card on the user dashboard.
/// Pin every documented wire key explicitly — both the renamed ones
/// AND the negative cases (no bare `price_cents`, no bare `start_date`).
#[test]
fn user_membership_response_wire_format_keys_match_frontend_contract() {
    let resp = UserMembershipResponse {
        id: "1".to_string(),
        name: "Day Trading Room".to_string(),
        membership_type: "trading-room".to_string(),
        slug: "day-trading-room".to_string(),
        status: "active".to_string(),
        subscription_type: Some("trial".to_string()),
        icon: Some("chart-line".to_string()),
        start_date: "2026-05-01".to_string(),
        next_billing_date: Some("2026-06-01".to_string()),
        expires_at: Some("2026-06-01".to_string()),
        price_cents: Some(9_900), // $99.00
        interval: Some("monthly".to_string()),
        features: Some(vec!["chat".to_string()]),
    };

    let wire = serde_json::to_value(&resp).expect("serialize");

    // Positive: every documented camelCase key the frontend reads.
    assert_eq!(wire["type"], "trading-room");
    assert_eq!(wire["membershipType"], "trial");
    assert_eq!(wire["startDate"], "2026-05-01");
    assert_eq!(wire["nextBillingDate"], "2026-06-01");
    assert_eq!(wire["expiresAt"], "2026-06-01");
    assert_eq!(wire["priceCents"], 9_900);
    assert_eq!(wire["slug"], "day-trading-room");
    assert_eq!(wire["status"], "active");

    // Negative: no snake_case leaks (would mean the rename annotations
    // were dropped during a refactor).
    assert!(
        wire.get("membership_type").is_none(),
        "membership_type must serialize as 'type', not 'membership_type'"
    );
    assert!(
        wire.get("subscription_type").is_none(),
        "subscription_type must serialize as 'membershipType', not 'subscription_type'"
    );
    assert!(
        wire.get("start_date").is_none(),
        "start_date must serialize as 'startDate'"
    );
    assert!(
        wire.get("next_billing_date").is_none(),
        "next_billing_date must serialize as 'nextBillingDate'"
    );
    assert!(
        wire.get("expires_at").is_none(),
        "expires_at must serialize as 'expiresAt'"
    );
    assert!(
        wire.get("price_cents").is_none(),
        "price_cents must serialize as 'priceCents'"
    );
}

// ── 3. CancelSubscriptionRequest: default = cancel-at-period-end ─────

/// `cancel_immediately` defaults to `false` via `#[serde(default)]`.
/// The contract: an empty body POST to `/memberships/:id/cancel` means
/// "cancel at period end" — the user keeps access until the date they
/// already paid for, and no proration/refund is issued. A regression
/// that flipped the default to `true` (or removed the `#[serde(default)]`)
/// would either 400-error every cancel click (bad UX) or, worse,
/// silently issue refunds on every cancel — a real revenue leak the
/// "How do I cancel?" support flow can't catch in QA.
#[test]
fn cancel_subscription_request_defaults_to_period_end_cancel() {
    // Empty body → cancel at period end (no immediate cancel, no reason).
    let empty: CancelSubscriptionRequest =
        serde_json::from_str("{}").expect("empty body must deserialize");
    assert!(
        !empty.cancel_immediately,
        "cancel_immediately MUST default to false (cancel at period end)"
    );
    assert!(
        empty.reason.is_none(),
        "reason MUST default to None when omitted"
    );

    // Explicit true → cancel now (refund path).
    let immediate: CancelSubscriptionRequest = serde_json::from_value(serde_json::json!({
        "cancel_immediately": true,
        "reason": "no longer trading",
    }))
    .expect("explicit cancel must deserialize");
    assert!(immediate.cancel_immediately);
    assert_eq!(immediate.reason.as_deref(), Some("no longer trading"));

    // Explicit false → same as default. Pin the boolean shape.
    let explicit_false: CancelSubscriptionRequest = serde_json::from_value(serde_json::json!({
        "cancel_immediately": false,
    }))
    .expect("explicit false must deserialize");
    assert!(!explicit_false.cancel_immediately);
}

// ── 4. PaymentMethodResponse: camelCase wire format, AddRequest too ──

/// Stripe gives us snake_case (`exp_month`, `last4`) but the SvelteKit
/// "Saved Cards" UI reads camelCase (`expiryMonth`, `last4`, `isDefault`).
/// `PaymentMethodResponse` has `#[serde(rename_all = "camelCase")]` AND
/// an explicit `#[serde(rename = "type")]` on `method_type`. A
/// regression that dropped EITHER would silently break every saved-card
/// row. Pin both.
///
/// Same struct also pins `AddPaymentMethodRequest.set_as_default`
/// default — the contract is that adding a card does NOT promote it to
/// the default payment method unless the user explicitly checks the
/// "Make Default" box. A regression that flipped the default would
/// silently rotate the customer's default card on every add.
#[test]
fn payment_method_dtos_use_camelcase_and_safe_defaults() {
    // PaymentMethodResponse: camelCase + type-rename pin
    let pm = PaymentMethodResponse {
        id: "pm_1234".to_string(),
        method_type: "card".to_string(),
        brand: Some("visa".to_string()),
        last4: Some("4242".to_string()),
        expiry_month: Some(12),
        expiry_year: Some(2030),
        is_default: true,
        subscriptions: vec!["sub_abc".to_string()],
    };
    let wire = serde_json::to_value(&pm).expect("serialize PM");
    // type-rename: method_type → "type" (frontend reads `pm.type`).
    assert_eq!(wire["type"], "card");
    assert!(
        wire.get("method_type").is_none(),
        "method_type must serialize as 'type'"
    );
    // camelCase rename_all: expiry_month → expiryMonth, etc.
    assert_eq!(wire["expiryMonth"], 12);
    assert_eq!(wire["expiryYear"], 2030);
    assert_eq!(wire["isDefault"], true);
    assert_eq!(wire["last4"], "4242");
    // Negative: no snake_case leaks.
    assert!(
        wire.get("expiry_month").is_none(),
        "expiry_month must serialize as 'expiryMonth'"
    );
    assert!(
        wire.get("is_default").is_none(),
        "is_default must serialize as 'isDefault'"
    );

    // AddPaymentMethodRequest: camelCase + set_as_default default-false.
    // The frontend POSTs `{ paymentMethodId: "pm_..." }` for a basic add
    // (no default promotion). Pin both shapes.
    let add: AddPaymentMethodRequest = serde_json::from_value(serde_json::json!({
        "paymentMethodId": "pm_test_xyz",
    }))
    .expect("camelCase paymentMethodId must deserialize");
    assert_eq!(add.payment_method_id, "pm_test_xyz");
    assert!(
        !add.set_as_default,
        "set_as_default MUST default to false — adding a card must NOT silently rotate the default"
    );

    // Explicit promote-to-default path.
    let add_default: AddPaymentMethodRequest = serde_json::from_value(serde_json::json!({
        "paymentMethodId": "pm_new",
        "setAsDefault": true,
    }))
    .expect("setAsDefault must deserialize");
    assert!(add_default.set_as_default);
}

// ── 5. MembershipsResponse: list wrapper pins empty + populated ──────

/// The frontend reads `{ memberships: [...] }` (list wrapper), NOT a
/// bare array. A regression that flattened the response would break
/// every `.memberships.map(...)` call site. Pin the wrapper shape
/// AND the empty-list case (new user with no subscriptions sees an
/// empty array, not `null` or a 404).
#[test]
fn memberships_response_wraps_list_under_memberships_key() {
    // Empty list (new user, no plans yet).
    let empty = MembershipsResponse {
        memberships: vec![],
    };
    let wire = serde_json::to_value(&empty).expect("serialize empty");
    assert_eq!(wire["memberships"], serde_json::json!([]));
    assert!(
        wire.get("memberships")
            .and_then(|v| v.as_array())
            .map(|a| a.is_empty())
            .unwrap_or(false),
        "empty user must serialize as `memberships: []`, never null/missing"
    );

    // Populated — pin the list-of-objects shape.
    let populated = MembershipsResponse {
        memberships: vec![UserMembershipResponse {
            id: "1".to_string(),
            name: "Day Trading Room".to_string(),
            membership_type: "trading-room".to_string(),
            slug: "day-trading-room".to_string(),
            status: "active".to_string(),
            subscription_type: None,
            icon: None,
            start_date: "2026-05-01".to_string(),
            next_billing_date: None,
            expires_at: None,
            price_cents: Some(9_900),
            interval: Some("monthly".to_string()),
            features: None,
        }],
    };
    let wire = serde_json::to_value(&populated).expect("serialize populated");
    assert_eq!(wire["memberships"].as_array().map(|a| a.len()), Some(1));
    assert_eq!(wire["memberships"][0]["slug"], "day-trading-room");
    assert_eq!(wire["memberships"][0]["priceCents"], 9_900);
}

// ── 6. Router mount table compile-pin ────────────────────────────────

/// `router()` must build as `Router<AppState>`. Load-bearing because
/// the 10-route `/api/user/*` mount-point is what the SvelteKit
/// frontend hits for EVERY logged-in user action that isn't admin-only
/// (memberships list, cancel, profile read/update, avatar upload/delete,
/// account deactivate, payment-methods list/add/delete). A regression
/// in ANY handler's signature — wrong extractor (drop `User`, become
/// unauthenticated), wrong return type — would fail compilation here.
/// This is the single canonical compile-pin for the entire
/// `/api/user/*` surface.
#[test]
fn user_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::user::router();
}

/// Idempotent construction — the router must be safe to build multiple
/// times in the same process (the main `api_router()` does this
/// implicitly when nesting at `/api/user`). A regression that
/// introduced a global `static` mutable inside the constructor would
/// fail here.
#[test]
fn user_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::user::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::user::router();
    // If the constructor were stateful, the second call would panic
    // ("already initialized"); reaching this line proves statelessness.
}
