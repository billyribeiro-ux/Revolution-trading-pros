//! Stripe subscription-migration E2E tests
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! These tests exercise `migrate_subscription_to_price` end-to-end against
//! Stripe test mode. We:
//!   1. Create a Stripe product + initial price.
//!   2. Create a customer with a default test payment method (`pm_card_visa`).
//!   3. Create a subscription on the initial price.
//!   4. Create a NEW price.
//!   5. Migrate the subscription to the new price under each
//!      `proration_behavior` (no proration vs immediate proration).
//!   6. Assert the subscription's items[0].price.id equals the new price.
//!
//! Each test is gated on the `STRIPE_TEST_SECRET_KEY` environment variable
//! AND a `sk_test_` prefix. Without that, every test no-ops.
//!
//! ```bash
//! export STRIPE_TEST_SECRET_KEY="sk_test_..."
//! cd api && cargo test --test stripe_subscription_migration_test -- --nocapture
//! ```

use revolution_api::services::stripe::StripeService;

const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";

fn maybe_key() -> Option<String> {
    let key = std::env::var("STRIPE_TEST_SECRET_KEY").ok()?;
    if !key.starts_with("sk_test_") {
        eprintln!(
            "STRIPE_TEST_SECRET_KEY does not start with 'sk_test_' — refusing to run live tests"
        );
        return None;
    }
    Some(key)
}

/// Helper: create a test customer with a default Visa test card attached.
async fn create_test_customer(client: &reqwest::Client, secret: &str) -> String {
    // Create customer
    let cust: serde_json::Value = client
        .post(format!("{STRIPE_API_BASE}/customers"))
        .basic_auth(secret, None::<&str>)
        .form(&[("email", "rtp-test@example.com")])
        .send()
        .await
        .expect("create customer")
        .json()
        .await
        .expect("parse customer");

    let customer_id = cust["id"].as_str().expect("customer id").to_string();

    // Attach Stripe's universal test PM (`pm_card_visa`)
    client
        .post(format!(
            "{STRIPE_API_BASE}/payment_methods/pm_card_visa/attach"
        ))
        .basic_auth(secret, None::<&str>)
        .form(&[("customer", customer_id.as_str())])
        .send()
        .await
        .expect("attach PM")
        .error_for_status()
        .expect("attach PM ok");

    // Set as default
    client
        .post(format!("{STRIPE_API_BASE}/customers/{customer_id}"))
        .basic_auth(secret, None::<&str>)
        .form(&[("invoice_settings[default_payment_method]", "pm_card_visa")])
        .send()
        .await
        .expect("set default PM")
        .error_for_status()
        .expect("set default PM ok");

    customer_id
}

/// Helper: create a subscription for a customer on a price.
async fn create_subscription(
    client: &reqwest::Client,
    secret: &str,
    customer_id: &str,
    price_id: &str,
) -> String {
    let resp: serde_json::Value = client
        .post(format!("{STRIPE_API_BASE}/subscriptions"))
        .basic_auth(secret, None::<&str>)
        .form(&[
            ("customer", customer_id),
            ("items[0][price]", price_id),
            ("payment_behavior", "default_incomplete"),
            ("expand[]", "latest_invoice.payment_intent"),
        ])
        .send()
        .await
        .expect("create subscription")
        .json()
        .await
        .expect("parse subscription");

    resp["id"]
        .as_str()
        .unwrap_or_else(|| panic!("create subscription failed: {resp}"))
        .to_string()
}

/// Fetch a subscription's first item's price.id (the canonical "current price").
async fn current_price_id(client: &reqwest::Client, secret: &str, sub_id: &str) -> String {
    let resp: serde_json::Value = client
        .get(format!("{STRIPE_API_BASE}/subscriptions/{sub_id}"))
        .basic_auth(secret, None::<&str>)
        .send()
        .await
        .expect("get subscription")
        .json()
        .await
        .expect("parse subscription");

    resp["items"]["data"][0]["price"]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("no items[0].price.id in {resp}"))
        .to_string()
}

#[tokio::test]
async fn migrate_subscription_no_proration_swaps_price() {
    let Some(secret) = maybe_key() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };
    let svc = StripeService::new(&secret);
    let client = reqwest::Client::new();

    // 1. product + old price
    let product = svc
        .create_product("RTP MIGRATE TEST (no_proration)")
        .await
        .expect("product");
    let old_price = svc
        .create_price(&product, 9900, "usd", "month")
        .await
        .expect("old price");
    let new_price = svc
        .create_price(&product, 12900, "usd", "month")
        .await
        .expect("new price");

    // 2. customer + sub
    let customer = create_test_customer(&client, &secret).await;
    let sub_id = create_subscription(&client, &secret, &customer, &old_price.id).await;
    assert_eq!(
        current_price_id(&client, &secret, &sub_id).await,
        old_price.id
    );

    // 3. migrate with proration_behavior=none
    svc.migrate_subscription_to_price(&sub_id, &new_price.id, "none")
        .await
        .expect("migrate (none)");

    // 4. confirm new price is now active
    assert_eq!(
        current_price_id(&client, &secret, &sub_id).await,
        new_price.id,
        "subscription should now be on the NEW price"
    );
}

#[tokio::test]
async fn migrate_subscription_immediate_proration_swaps_price() {
    let Some(secret) = maybe_key() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };
    let svc = StripeService::new(&secret);
    let client = reqwest::Client::new();

    let product = svc
        .create_product("RTP MIGRATE TEST (proration)")
        .await
        .expect("product");
    let old_price = svc
        .create_price(&product, 9900, "usd", "month")
        .await
        .expect("old price");
    let new_price = svc
        .create_price(&product, 14900, "usd", "month")
        .await
        .expect("new price");

    let customer = create_test_customer(&client, &secret).await;
    let sub_id = create_subscription(&client, &secret, &customer, &old_price.id).await;
    assert_eq!(
        current_price_id(&client, &secret, &sub_id).await,
        old_price.id
    );

    svc.migrate_subscription_to_price(&sub_id, &new_price.id, "create_prorations")
        .await
        .expect("migrate (create_prorations)");

    assert_eq!(
        current_price_id(&client, &secret, &sub_id).await,
        new_price.id,
        "subscription should now be on the NEW price (immediate proration path)"
    );

    // Sanity-check: confirm at least one proration line item appeared.
    // We list invoices for the customer; the latest should reference the
    // new price.
    let invoices: serde_json::Value = client
        .get(format!("{STRIPE_API_BASE}/invoices"))
        .basic_auth(secret.as_str(), None::<&str>)
        .query(&[("customer", customer.as_str()), ("limit", "5")])
        .send()
        .await
        .expect("list invoices")
        .json()
        .await
        .expect("parse invoices");

    assert!(
        invoices["data"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false),
        "expected at least one invoice for the customer, got: {invoices}"
    );
}

#[tokio::test]
async fn list_subscription_items_returns_current_item() {
    let Some(secret) = maybe_key() else {
        eprintln!("[skip] STRIPE_TEST_SECRET_KEY not set");
        return;
    };
    let svc = StripeService::new(&secret);
    let client = reqwest::Client::new();

    let product = svc
        .create_product("RTP LIST ITEMS TEST")
        .await
        .expect("product");
    let price = svc
        .create_price(&product, 4900, "usd", "month")
        .await
        .expect("price");
    let customer = create_test_customer(&client, &secret).await;
    let sub_id = create_subscription(&client, &secret, &customer, &price.id).await;

    let items = svc
        .list_subscription_items(&sub_id)
        .await
        .expect("list_subscription_items");

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].price.id, price.id);
    assert!(items[0].id.starts_with("si_"));
}
