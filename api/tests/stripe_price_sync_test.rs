//! Stripe price-sync tests
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! These tests exercise the StripeService price/product/subscription
//! migration methods against Stripe test mode using a real test secret key.
//!
//! Each test is gated on the `STRIPE_TEST_SECRET_KEY` environment variable
//! AND a key prefix of `sk_test_`. Without that, every test is a no-op and
//! `assert!(true)` to keep CI green when keys aren't injected. To run them
//! locally:
//!
//! ```bash
//! export STRIPE_TEST_SECRET_KEY="sk_test_..."
//! cd api && cargo test --test stripe_price_sync_test -- --nocapture
//! ```
//!
//! Note: tests run sequentially (not parallel) to avoid Stripe rate limits.

use revolution_api::services::stripe::StripeService;

/// Returns Some(StripeService) if a sk_test_ key is available, else None.
fn maybe_service() -> Option<StripeService> {
    let key = std::env::var("STRIPE_TEST_SECRET_KEY").ok()?;
    if !key.starts_with("sk_test_") {
        eprintln!(
            "STRIPE_TEST_SECRET_KEY does not start with 'sk_test_' — refusing to run live tests"
        );
        return None;
    }
    Some(StripeService::new(&key))
}

#[tokio::test]
async fn create_product_returns_id() {
    let Some(svc) = maybe_service() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };

    let result = svc.create_product("RTP Test Product").await;
    assert!(result.is_ok(), "create_product failed: {:?}", result.err());
    let product_id = result.unwrap();
    assert!(
        product_id.starts_with("prod_"),
        "Stripe product id should start with prod_, got {product_id}"
    );
}

#[tokio::test]
async fn create_recurring_monthly_price_then_retrieve() {
    let Some(svc) = maybe_service() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };

    let product_id = svc
        .create_product("RTP Test Plan A")
        .await
        .expect("create_product");

    let price = svc
        .create_price(&product_id, 9900, "usd", "month")
        .await
        .expect("create_price");

    assert!(price.id.starts_with("price_"));
    assert_eq!(price.unit_amount, Some(9900));
    assert_eq!(price.currency, "usd");
    assert!(price.active);
    assert!(price.recurring.is_some());
    let recurring = price.recurring.as_ref().unwrap();
    assert_eq!(recurring.interval, "month");
    assert_eq!(recurring.interval_count, 1);

    // Round-trip
    let fetched = svc.get_price(&price.id).await.expect("get_price");
    assert_eq!(fetched.id, price.id);
    assert_eq!(fetched.unit_amount, Some(9900));
}

#[tokio::test]
async fn create_yearly_price() {
    let Some(svc) = maybe_service() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };

    let product_id = svc
        .create_product("RTP Test Plan Yearly")
        .await
        .expect("create_product");

    let price = svc
        .create_price(&product_id, 99000, "usd", "year")
        .await
        .expect("create_price");

    assert_eq!(price.unit_amount, Some(99000));
    let recurring = price
        .recurring
        .as_ref()
        .expect("yearly price must have recurring");
    assert_eq!(recurring.interval, "year");
}

#[tokio::test]
async fn create_one_time_price_has_no_recurring() {
    let Some(svc) = maybe_service() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };

    let product_id = svc
        .create_product("RTP Test One-Time")
        .await
        .expect("create_product");

    let price = svc
        .create_price(&product_id, 4900, "usd", "one_time")
        .await
        .expect("create_price");

    assert!(price.recurring.is_none(), "one_time price must not recur");
}

#[tokio::test]
async fn create_price_rejects_bad_interval() {
    let Some(svc) = maybe_service() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };

    let product_id = svc
        .create_product("RTP Test Bad Interval")
        .await
        .expect("create_product");

    let result = svc
        .create_price(&product_id, 1000, "usd", "fortnight")
        .await;

    assert!(result.is_err(), "fortnight should be rejected");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("Unsupported billing_interval"),
        "Expected validation error, got: {msg}"
    );
}

#[tokio::test]
async fn deactivate_price_marks_inactive() {
    let Some(svc) = maybe_service() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };

    let product_id = svc
        .create_product("RTP Test Deactivate")
        .await
        .expect("create_product");
    let price = svc
        .create_price(&product_id, 1500, "usd", "month")
        .await
        .expect("create_price");
    assert!(price.active);

    svc.deactivate_price(&price.id)
        .await
        .expect("deactivate_price");

    let after = svc.get_price(&price.id).await.expect("get_price");
    assert!(!after.active, "price should be inactive after deactivate");
}
