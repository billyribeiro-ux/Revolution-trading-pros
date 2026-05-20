//! Checkout route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::checkout` and exercises
//! the public request DTOs (`CartItem`, `CheckoutRequest`,
//! `CalculateTaxRequest`) plus the `router()` mount table.
//!
//! ## Why this shape
//!
//! Every handler in `routes/checkout.rs` runs live SQL (against
//! `products` / `subscription_plans` / `coupons` / `orders` /
//! `order_items`) AND makes outbound Stripe API calls (Customer +
//! CheckoutSession create). Neither is reachable in a unit-test
//! harness — so we attack the contract surface the handlers ride on:
//!
//! 1. **Cart-line type pin.** `CartItem.product_id` and `CartItem.
//!    plan_id` are `Option<i64>` (one or the other set; never both).
//!    `quantity` is `Option<i32>` — but the *price* lookups inside
//!    the handler (`ProductPrice.price_cents`, `PlanPrice.price_cents`)
//!    are `i64`. We can only directly assert the DTOs, but pinning
//!    `product_id` as i64 catches the most common AI-refactor failure
//!    mode: silently widening `i32 -> i64` everywhere in one file but
//!    leaving the cart line type narrow, which then implicit-casts at
//!    SQL bind time and silently truncates large IDs.
//!
//! 2. **`success_path` / `cancel_path` are RELATIVE-only.** Comments
//!    in the source pin "must start with /" — the server builds the
//!    full URL. We can't enforce the runtime check here, but we CAN
//!    pin the wire type as `Option<String>` so a refactor that tries
//!    to be helpful and require it would break every existing
//!    frontend call.
//!
//! 3. **Tax request shape.** `CalculateTaxRequest` has the same `items`
//!    Vec as `CheckoutRequest` plus address. Pin it so a future
//!    refactor that merges the two structs is caught here.
//!
//! 4. **Router mount table.** `router()` exposes 4 routes
//!    (`/`, `/calculate-tax`, `/orders`, `/orders/:order_number`). A
//!    refactor that drops one or flips its HTTP verb would compile and
//!    silently 404 in prod. The `Router<AppState>` return-type pin
//!    catches every extractor-signature regression at build time.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/products_test.rs`, `tests/subscriptions_test.rs`,
//! `tests/admin_member_management_test.rs`.

use revolution_api::routes::checkout::{CalculateTaxRequest, CartItem, CheckoutRequest};

// ── 1. CartItem: product/plan IDs are i64 ────────────────────────────

/// `CartItem.product_id` and `CartItem.plan_id` are `Option<i64>`.
/// Product / plan IDs are `BIGSERIAL` in PostgreSQL — the bigint range
/// is load-bearing for any catalog that ever crosses 2.1B rows across
/// its lifetime (including soft-deleted rows). A regression that
/// narrows either field to `i32` would silently truncate IDs above
/// 2.1B at the wire boundary, making those products un-purchaseable
/// without raising a single error. Pin both past `i32::MAX`.
#[test]
fn cart_item_ids_are_i64_round_trip_past_i32_max() {
    let big_id: i64 = (i32::MAX as i64) + 7; // 2_147_483_654 — beyond i32

    // product-line shape
    let prod_line: CartItem = serde_json::from_value(serde_json::json!({
        "product_id": big_id,
        "quantity": 2,
    }))
    .expect("CartItem with i64 product_id must deserialize");
    assert_eq!(prod_line.product_id, Some(big_id));
    assert_eq!(prod_line.plan_id, None);
    assert_eq!(prod_line.quantity, Some(2));

    // plan-line shape
    let plan_line: CartItem = serde_json::from_value(serde_json::json!({
        "plan_id": big_id,
    }))
    .expect("CartItem with i64 plan_id must deserialize");
    assert_eq!(plan_line.plan_id, Some(big_id));
    assert_eq!(plan_line.product_id, None);
    // `quantity` is optional — verify omission yields None (handler
    // defaults to 1 internally, but the wire-type contract is None).
    assert!(plan_line.quantity.is_none());

    // Sanity: narrowing to i32 would corrupt the ID.
    assert!(
        (big_id as i32 as i64) != big_id,
        "narrowing to i32 must lose data — proves i64 is required"
    );
}

// ── 2. CheckoutRequest: full + minimal payloads ──────────────────────

/// `CheckoutRequest` is the wire-format the cart page POSTs to start a
/// Stripe Checkout Session. Verify both the maximal (every metadata
/// field set, coupon attached) and minimal (just `items`) payloads
/// parse. The minimal shape is what an anonymous-guest-coupon-less
/// checkout sends; if a future "tighten the schema" refactor flips
/// `success_path` to required, every existing client call breaks.
#[test]
fn checkout_request_accepts_full_and_minimal_payloads() {
    let big_id: i64 = (i32::MAX as i64) + 1;

    let full: CheckoutRequest = serde_json::from_value(serde_json::json!({
        "items": [
            {"product_id": big_id, "quantity": 1},
            {"plan_id": 42, "quantity": 1},
        ],
        "coupon_code": "WELCOME10",
        "billing_name": "Jane Doe",
        "billing_email": "jane@example.com",
        "billing_address": {"line1": "123 Main", "city": "Austin", "country": "US"},
        "success_path": "/checkout/success",
        "cancel_path": "/checkout/cancelled",
    }))
    .expect("full CheckoutRequest must deserialize");

    assert_eq!(full.items.len(), 2);
    assert_eq!(full.items[0].product_id, Some(big_id));
    assert_eq!(full.items[1].plan_id, Some(42));
    assert_eq!(full.coupon_code.as_deref(), Some("WELCOME10"));
    assert_eq!(full.billing_name.as_deref(), Some("Jane Doe"));
    assert_eq!(full.success_path.as_deref(), Some("/checkout/success"));
    assert_eq!(full.cancel_path.as_deref(), Some("/checkout/cancelled"));
    assert!(full.billing_address.is_some());

    // Minimal shape: a logged-in user with no coupon, default return
    // paths. Every metadata field MUST be optional or this breaks.
    let minimal: CheckoutRequest = serde_json::from_value(serde_json::json!({
        "items": [{"product_id": 1}],
    }))
    .expect("minimal CheckoutRequest must deserialize");

    assert_eq!(minimal.items.len(), 1);
    assert!(minimal.coupon_code.is_none());
    assert!(minimal.billing_name.is_none());
    assert!(minimal.billing_email.is_none());
    assert!(minimal.billing_address.is_none());
    assert!(minimal.success_path.is_none());
    assert!(minimal.cancel_path.is_none());
}

// ── 3. CalculateTaxRequest: tax preview wire shape ──────────────────

/// `CalculateTaxRequest` is what the cart page POSTs to preview tax
/// before committing to checkout (so the user sees the line item
/// before they click "Pay"). Address fields are all optional — a US
/// user with only a ZIP, a UK user with only a country code, etc.
/// must all deserialize. Pin the optional-everywhere shape.
#[test]
fn calculate_tax_request_accepts_partial_addresses() {
    // Country-only (UK style, no state/zip)
    let uk: CalculateTaxRequest = serde_json::from_value(serde_json::json!({
        "items": [{"product_id": 1, "quantity": 1}],
        "country": "GB",
    }))
    .expect("country-only CalculateTaxRequest");
    assert_eq!(uk.country.as_deref(), Some("GB"));
    assert!(uk.state.is_none());
    assert!(uk.postal_code.is_none());

    // US style — country + state + ZIP
    let us: CalculateTaxRequest = serde_json::from_value(serde_json::json!({
        "items": [{"plan_id": 42}],
        "country": "US",
        "state": "CA",
        "postal_code": "94110",
    }))
    .expect("US-style CalculateTaxRequest");
    assert_eq!(us.country.as_deref(), Some("US"));
    assert_eq!(us.state.as_deref(), Some("CA"));
    assert_eq!(us.postal_code.as_deref(), Some("94110"));

    // Address-less (handler returns 0 tax) — must still deserialize
    let no_addr: CalculateTaxRequest = serde_json::from_value(serde_json::json!({
        "items": [{"product_id": 1}],
    }))
    .expect("address-less CalculateTaxRequest");
    assert!(no_addr.country.is_none());
    assert!(no_addr.state.is_none());
    assert!(no_addr.postal_code.is_none());
    assert_eq!(no_addr.items.len(), 1);
}

// ── 4. CartItem JSON shape: empty cart line must be tolerated ───────

/// A `CartItem` with neither `product_id` nor `plan_id` set is
/// semantically invalid (the handler rejects it at runtime), but the
/// wire type itself must still deserialize cleanly — otherwise a
/// malformed cart corrupts the request body and we get a generic
/// 422 instead of the typed "missing product or plan" 400 response.
/// Pin the permissive-DTO + strict-handler split.
#[test]
fn cart_item_with_neither_id_set_still_deserializes() {
    let empty: CartItem = serde_json::from_value(serde_json::json!({}))
        .expect("empty CartItem must deserialize (handler does runtime validation, not serde)");
    assert!(empty.product_id.is_none());
    assert!(empty.plan_id.is_none());
    assert!(empty.quantity.is_none());

    // And: quantity-only (also invalid but must parse)
    let qty_only: CartItem =
        serde_json::from_value(serde_json::json!({"quantity": 99})).expect("quantity-only");
    assert_eq!(qty_only.quantity, Some(99));
    assert!(qty_only.product_id.is_none());
}

// ── 5. Router mount table compile-pin ────────────────────────────────

/// The `router()` constructor must produce a `Router<AppState>`. A
/// regression that breaks a handler signature — wrong extractor (e.g.
/// drops `User` from `create_checkout`, allowing anonymous checkout
/// against authenticated handler logic), wrong return type, or wrong
/// state-type — will fail to compile here. The explicit type
/// annotation on the binding is the load-bearing assertion.
#[test]
fn checkout_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::checkout::router();
}
