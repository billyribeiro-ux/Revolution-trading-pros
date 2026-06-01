//! Stripe Payment Service - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Enterprise-grade Stripe integration with:
//! - Checkout sessions (one-time + subscription)
//! - Customer portal management
//! - Webhook signature verification
//! - Payment intent handling
//! - Subscription lifecycle management
//! - Refund processing

use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;

/// Stripe API version - December 2025
const STRIPE_API_VERSION: &str = "2024-12-18.acacia";
const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";

/// Stripe checkout session response.
///
/// `amount_total` and `amount_subtotal` are the cent values actually charged to
/// the customer after Stripe applies any promotion code. `total_details.amount_discount`
/// is what Stripe knocked off; `total_details.breakdown.discounts[]` lists the
/// applied promotion codes (we read the first one for `coupon_code` reconciliation).
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCheckoutSession {
    pub id: String,
    pub url: Option<String>,
    pub payment_status: String,
    pub customer: Option<String>,
    pub subscription: Option<String>,
    pub payment_intent: Option<String>,
    pub amount_total: Option<i64>,
    pub amount_subtotal: Option<i64>,
    pub currency: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub total_details: Option<StripeSessionTotalDetails>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StripeSessionTotalDetails {
    pub amount_discount: Option<i64>,
    pub amount_shipping: Option<i64>,
    pub amount_tax: Option<i64>,
    pub breakdown: Option<StripeSessionTotalBreakdown>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StripeSessionTotalBreakdown {
    #[serde(default)]
    pub discounts: Vec<StripeSessionDiscount>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StripeSessionDiscount {
    pub amount: Option<i64>,
    pub discount: Option<StripeSessionDiscountInner>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StripeSessionDiscountInner {
    pub id: Option<String>,
    pub coupon: Option<StripeCouponRef>,
    pub promotion_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StripeCouponRef {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// Stripe subscription response
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeSubscription {
    pub id: String,
    pub status: String,
    pub customer: String,
    /// Stripe 2026 API: period timestamps moved to items.data[0].
    /// These top-level fields are kept as #[serde(default)] fallback for older fixtures/mocks.
    #[serde(default)]
    pub current_period_start: i64,
    #[serde(default)]
    pub current_period_end: i64,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
    /// Items list — present in real API responses and live webhook payloads.
    pub items: Option<StripeSubscriptionItemList>,
}

impl StripeSubscription {
    /// Returns (current_period_start, current_period_end) as Unix timestamps.
    /// Reads from items.data[0] first (Stripe 2026+), falls back to top-level fields.
    pub fn get_current_period(&self) -> (i64, i64) {
        if let Some(item) = self.items.as_ref().and_then(|l| l.data.first()) {
            if item.current_period_start != 0 || item.current_period_end != 0 {
                return (item.current_period_start, item.current_period_end);
            }
        }
        (self.current_period_start, self.current_period_end)
    }
}

/// Stripe customer response
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCustomer {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Stripe refund response
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeRefund {
    pub id: String,
    pub amount: i64,
    pub status: String,
    pub payment_intent: Option<String>,
    pub reason: Option<String>,
}

/// Stripe Coupon object (subset of fields we use). Created via the
/// /v1/coupons endpoint and attached to a Checkout Session via `discounts[]`.
///
/// Stripe Coupons are immutable after creation. To "edit" a DB coupon we
/// create a new Stripe coupon, store its id, and (optionally) delete the
/// old one. Deletion is supported but does not invalidate already-redeemed
/// coupons; it only stops future redemptions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripeCoupon {
    pub id: String,
    /// One of "once", "forever", "repeating".
    pub duration: String,
    /// Required when `duration == "repeating"`: number of billing periods
    /// the coupon applies to.
    pub duration_in_months: Option<i64>,
    /// Set for percent-off coupons (1-100). Mutually exclusive with `amount_off`.
    pub percent_off: Option<f64>,
    /// Set for fixed-amount coupons (cents). Mutually exclusive with `percent_off`.
    pub amount_off: Option<i64>,
    /// Currency for `amount_off`. Required when `amount_off` is set.
    pub currency: Option<String>,
    pub valid: Option<bool>,
    pub name: Option<String>,
    pub max_redemptions: Option<i64>,
    pub times_redeemed: Option<i64>,
    pub redeem_by: Option<i64>,
    pub created: Option<i64>,
}

/// Stripe Price object (subset of fields we use)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripePrice {
    pub id: String,
    pub product: Option<String>,
    pub unit_amount: Option<i64>,
    pub currency: String,
    pub active: bool,
    pub recurring: Option<StripePriceRecurring>,
}

/// Recurring portion of a Stripe Price
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripePriceRecurring {
    pub interval: String,
    pub interval_count: i64,
}

/// Single subscription item — includes period timestamps (Stripe moved them here in 2026 API)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripeSubscriptionItem {
    pub id: String,
    pub price: StripePrice,
    #[serde(default)]
    pub current_period_start: i64,
    #[serde(default)]
    pub current_period_end: i64,
}

/// Paginated list of subscription items (Stripe wraps them in a list object)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripeSubscriptionItemList {
    pub data: Vec<StripeSubscriptionItem>,
}

/// Stripe payment method response - ICT 7 Fix: Payment Methods Management
#[derive(Debug, Serialize, Deserialize)]
pub struct StripePaymentMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,
    pub card: Option<StripeCardDetails>,
    pub billing_details: Option<StripeBillingDetails>,
    pub created: i64,
    pub customer: Option<String>,
}

/// Card details from payment method
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCardDetails {
    pub brand: String,
    pub last4: String,
    pub exp_month: u32,
    pub exp_year: u32,
    pub funding: Option<String>,
    pub country: Option<String>,
}

/// Billing details from payment method
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeBillingDetails {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<StripeAddress>,
}

/// Address from billing details
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeAddress {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

/// Payment methods list response
#[derive(Debug, Deserialize)]
pub struct PaymentMethodsList {
    pub data: Vec<StripePaymentMethod>,
    pub has_more: bool,
}

/// Stripe billing portal session
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeBillingPortal {
    pub id: String,
    pub url: String,
}

/// Payment retry result - ICT 7 Fix
#[derive(Debug)]
pub struct RetryPaymentResult {
    pub success: bool,
    pub error: Option<String>,
}

/// Line item for checkout
#[derive(Debug, Clone)]
pub struct LineItem {
    pub price_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub amount: i64, // In cents
    pub currency: String,
    pub quantity: i64,
    pub is_subscription: bool,
    pub interval: Option<String>,
    pub interval_count: Option<i64>,
}

/// Inputs for `StripeService::create_coupon`. Exactly one of `percent_off`
/// or `amount_off_cents` must be Some. `duration` is one of "once" /
/// "forever" / "repeating"; when "repeating", `duration_in_months` MUST be
/// Some(n) with n >= 1.
#[derive(Debug, Clone)]
pub struct CreateStripeCouponRequest {
    pub percent_off: Option<f64>,
    pub amount_off_cents: Option<i64>,
    pub currency: String,
    pub duration: String,
    pub duration_in_months: Option<i64>,
    pub name: Option<String>,
    pub max_redemptions: Option<i64>,
    pub redeem_by_unix: Option<i64>,
}

/// A single Stripe discount entry to attach to a Checkout Session via
/// `discounts[]`. Either `coupon` or `promotion_code` must be set; we use
/// `coupon` for the server-applied path (DB coupon mirrored into Stripe).
#[derive(Debug, Clone)]
pub struct DiscountSpec {
    pub coupon: Option<String>,
    pub promotion_code: Option<String>,
}

/// Checkout session configuration
#[derive(Debug)]
pub struct CheckoutConfig {
    pub customer_email: String,
    pub customer_name: Option<String>,
    pub line_items: Vec<LineItem>,
    pub success_url: String,
    pub cancel_url: String,
    pub metadata: HashMap<String, String>,
    /// When true, Stripe shows a "promotion code" field on its hosted page
    /// so the customer can enter a Stripe Promotion Code. Mutually
    /// exclusive at runtime with `discounts` — Stripe rejects sessions that
    /// have both.
    pub allow_promotion_codes: bool,
    /// Server-applied discounts (DB coupons mirrored into Stripe). When
    /// non-empty, `allow_promotion_codes` MUST be false.
    pub discounts: Vec<DiscountSpec>,
    pub billing_address_collection: bool,
    /// If set, Stripe will start the subscription in trial mode for this many days.
    pub trial_period_days: Option<i64>,
    /// If false, card collection is deferred until trial converts (payment_method_collection=if_required).
    pub trial_requires_payment_method: bool,
}

#[derive(Clone)]
pub struct StripeService {
    secret_key: String,
    webhook_secret: Option<String>,
    client: reqwest::Client,
}

impl StripeService {
    /// Create new Stripe service with API key
    /// ICT 11+ Fix: Use unwrap_or_else with default client instead of expect()
    pub fn new(secret_key: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            webhook_secret: None,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|e| {
                    tracing::error!(
                        "Failed to create HTTP client with timeout: {}, using default",
                        e
                    );
                    reqwest::Client::new()
                }),
        }
    }

    /// Set webhook secret for signature verification
    pub fn with_webhook_secret(mut self, secret: &str) -> Self {
        self.webhook_secret = Some(secret.to_string());
        self
    }

    /// Create or retrieve Stripe customer
    pub async fn get_or_create_customer(
        &self,
        email: &str,
        name: Option<&str>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<StripeCustomer> {
        // First, try to find existing customer
        let search_params = [("email", email), ("limit", "1")];

        let response: serde_json::Value = self
            .client
            .get(format!("{STRIPE_API_BASE}/customers"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&search_params)
            .send()
            .await?
            .json()
            .await?;

        if let Some(customers) = response["data"].as_array() {
            if let Some(customer) = customers.first() {
                return Ok(serde_json::from_value(customer.clone())?);
            }
        }

        // Create new customer
        let mut params: Vec<(String, String)> = vec![("email".to_string(), email.to_string())];

        if let Some(n) = name {
            params.push(("name".to_string(), n.to_string()));
        }

        if let Some(meta) = metadata {
            for (k, v) in meta {
                params.push((format!("metadata[{k}]"), v));
            }
        }

        let response: StripeCustomer = self
            .client
            .post(format!("{STRIPE_API_BASE}/customers"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(
                &params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Create a checkout session for one-time or subscription payments
    pub async fn create_checkout_session(
        &self,
        config: CheckoutConfig,
    ) -> Result<StripeCheckoutSession> {
        let mut form_params: Vec<(String, String)> = vec![];

        // Customer email
        form_params.push(("customer_email".to_string(), config.customer_email));

        // Determine mode based on items
        let has_subscription = config.line_items.iter().any(|item| item.is_subscription);
        let mode = if has_subscription {
            "subscription"
        } else {
            "payment"
        };
        form_params.push(("mode".to_string(), mode.to_string()));

        // URLs
        form_params.push(("success_url".to_string(), config.success_url));
        form_params.push(("cancel_url".to_string(), config.cancel_url));

        // Line items
        for (i, item) in config.line_items.iter().enumerate() {
            if let Some(ref price_id) = item.price_id {
                // Use existing Stripe price
                form_params.push((format!("line_items[{i}][price]"), price_id.clone()));
                form_params.push((
                    format!("line_items[{i}][quantity]"),
                    item.quantity.to_string(),
                ));
            } else {
                // Create ad-hoc price
                form_params.push((
                    format!("line_items[{i}][price_data][currency]"),
                    item.currency.clone(),
                ));
                form_params.push((
                    format!("line_items[{i}][price_data][unit_amount]"),
                    item.amount.to_string(),
                ));
                form_params.push((
                    format!("line_items[{i}][price_data][product_data][name]"),
                    item.name.clone(),
                ));

                if let Some(ref desc) = item.description {
                    form_params.push((
                        format!("line_items[{i}][price_data][product_data][description]"),
                        desc.clone(),
                    ));
                }

                // Subscription pricing
                if item.is_subscription {
                    let interval = item.interval.as_deref().unwrap_or("month");
                    let count = item.interval_count.unwrap_or(1);
                    form_params.push((
                        format!("line_items[{i}][price_data][recurring][interval]"),
                        interval.to_string(),
                    ));
                    form_params.push((
                        format!("line_items[{i}][price_data][recurring][interval_count]"),
                        count.to_string(),
                    ));
                }

                form_params.push((
                    format!("line_items[{i}][quantity]"),
                    item.quantity.to_string(),
                ));
            }
        }

        // Metadata
        for (k, v) in config.metadata {
            form_params.push((format!("metadata[{k}]"), v));
        }

        // Discounts (server-applied via Stripe Coupon mirror) and
        // promotion-code field (Stripe-side promo code on the hosted page).
        // Stripe rejects sessions that have BOTH set, so the caller must pick
        // one: server-applied (when our DB coupon was supplied) or hosted-page
        // (when no DB coupon was supplied so the customer can type a Stripe
        // Promotion Code).
        if !config.discounts.is_empty() {
            for (i, d) in config.discounts.iter().enumerate() {
                if let Some(ref c) = d.coupon {
                    form_params.push((format!("discounts[{i}][coupon]"), c.clone()));
                }
                if let Some(ref pc) = d.promotion_code {
                    form_params.push((format!("discounts[{i}][promotion_code]"), pc.clone()));
                }
            }
        } else if config.allow_promotion_codes {
            form_params.push(("allow_promotion_codes".to_string(), "true".to_string()));
        }

        if config.billing_address_collection {
            form_params.push((
                "billing_address_collection".to_string(),
                "required".to_string(),
            ));
        }

        // Trial support
        if let Some(trial_days) = config.trial_period_days {
            if trial_days > 0 {
                form_params.push((
                    "subscription_data[trial_period_days]".to_string(),
                    trial_days.to_string(),
                ));
                if !config.trial_requires_payment_method {
                    form_params.push((
                        "payment_method_collection".to_string(),
                        "if_required".to_string(),
                    ));
                    form_params.push((
                        "subscription_data[trial_settings][end_behavior][missing_payment_method]"
                            .to_string(),
                        "cancel".to_string(),
                    ));
                }
            }
        }

        // Payment method types
        form_params.push(("payment_method_types[0]".to_string(), "card".to_string()));

        let response = self
            .client
            .post(format!("{STRIPE_API_BASE}/checkout/sessions"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(
                &form_params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;

        if !response.status().is_success() {
            let error: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe error: {}",
                error["error"]["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
            ));
        }

        Ok(response.json().await?)
    }

    /// Create a checkout session for subscription (simplified interface)
    pub async fn create_subscription_checkout(
        &self,
        customer_email: &str,
        price_id: &str,
        success_url: &str,
        cancel_url: &str,
    ) -> Result<String> {
        let config = CheckoutConfig {
            customer_email: customer_email.to_string(),
            customer_name: None,
            line_items: vec![LineItem {
                price_id: Some(price_id.to_string()),
                name: "Subscription".to_string(),
                description: None,
                amount: 0,
                currency: "usd".to_string(),
                quantity: 1,
                is_subscription: true,
                interval: None,
                interval_count: None,
            }],
            success_url: success_url.to_string(),
            cancel_url: cancel_url.to_string(),
            metadata: HashMap::new(),
            allow_promotion_codes: true,
            discounts: Vec::new(),
            billing_address_collection: false,
            trial_period_days: None,
            trial_requires_payment_method: true,
        };

        let session = self.create_checkout_session(config).await?;
        session
            .url
            .ok_or_else(|| anyhow!("No checkout URL returned"))
    }

    /// Retrieve checkout session details
    pub async fn get_checkout_session(&self, session_id: &str) -> Result<StripeCheckoutSession> {
        let response: StripeCheckoutSession = self
            .client
            .get(format!("{STRIPE_API_BASE}/checkout/sessions/{session_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&[("expand[]", "subscription"), ("expand[]", "payment_intent")])
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Create customer portal session
    pub async fn create_portal_session(
        &self,
        customer_id: &str,
        return_url: &str,
    ) -> Result<String> {
        let params = [("customer", customer_id), ("return_url", return_url)];

        let response: StripeBillingPortal = self
            .client
            .post(format!("{STRIPE_API_BASE}/billing_portal/sessions"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.url)
    }

    /// Create a Stripe Coupon. The DB-side admin UI calls this when an
    /// operator creates a new coupon; the returned id is stored as
    /// `coupons.stripe_coupon_id`.
    ///
    /// Stripe Coupons are immutable. To "edit" you create a new one and
    /// flip the pointer; the old one can be deleted to stop future
    /// redemptions (already-redeemed subscriptions keep their discount).
    pub async fn create_coupon(&self, req: CreateStripeCouponRequest) -> Result<StripeCoupon> {
        if req.percent_off.is_some() == req.amount_off_cents.is_some() {
            return Err(anyhow!(
                "Stripe coupon: must set exactly one of percent_off or amount_off_cents"
            ));
        }
        if req.duration == "repeating" && req.duration_in_months.is_none() {
            return Err(anyhow!(
                "Stripe coupon with duration='repeating' requires duration_in_months"
            ));
        }
        if req.duration != "repeating" && req.duration_in_months.is_some() {
            return Err(anyhow!(
                "Stripe coupon: duration_in_months only valid when duration='repeating'"
            ));
        }
        if !["once", "forever", "repeating"].contains(&req.duration.as_str()) {
            return Err(anyhow!(
                "Stripe coupon: duration must be once|forever|repeating, got '{}'",
                req.duration
            ));
        }

        let mut form: Vec<(String, String)> = vec![("duration".into(), req.duration.clone())];
        if let Some(p) = req.percent_off {
            form.push(("percent_off".into(), format!("{p}")));
        }
        if let Some(a) = req.amount_off_cents {
            form.push(("amount_off".into(), a.to_string()));
            form.push(("currency".into(), req.currency.to_lowercase()));
        }
        if let Some(n) = req.duration_in_months {
            form.push(("duration_in_months".into(), n.to_string()));
        }
        if let Some(n) = req.name.as_ref() {
            form.push(("name".into(), n.clone()));
        }
        if let Some(m) = req.max_redemptions {
            form.push(("max_redemptions".into(), m.to_string()));
        }
        if let Some(t) = req.redeem_by_unix {
            form.push(("redeem_by".into(), t.to_string()));
        }

        let response = self
            .client
            .post(format!("{STRIPE_API_BASE}/coupons"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(
                &form
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;
        if !response.status().is_success() {
            let err: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe coupon create failed: {}",
                err["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }
        Ok(response.json().await?)
    }

    /// Retrieve a Stripe Coupon by id (admin diagnostic; also used at
    /// checkout time to verify a DB-mirrored coupon still exists in Stripe
    /// before attaching it).
    pub async fn retrieve_coupon(&self, coupon_id: &str) -> Result<StripeCoupon> {
        let response = self
            .client
            .get(format!("{STRIPE_API_BASE}/coupons/{coupon_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .send()
            .await?;
        if !response.status().is_success() {
            let err: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe coupon retrieve failed: {}",
                err["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }
        Ok(response.json().await?)
    }

    /// Delete a Stripe Coupon. Stops future redemptions; existing
    /// subscriptions that already have the discount keep it for the
    /// remainder of its `duration`.
    pub async fn delete_coupon(&self, coupon_id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{STRIPE_API_BASE}/coupons/{coupon_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .send()
            .await?;
        if !response.status().is_success() {
            let err: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe coupon delete failed: {}",
                err["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }
        Ok(())
    }

    /// Retrieve a Checkout Session with discount details fully expanded.
    /// Used by `handle_checkout_completed` to reconcile `orders` with the
    /// actual Stripe-applied amounts after promotion codes (per arch §10).
    pub async fn retrieve_checkout_session(
        &self,
        session_id: &str,
    ) -> Result<StripeCheckoutSession> {
        let response: StripeCheckoutSession = self
            .client
            .get(format!("{STRIPE_API_BASE}/checkout/sessions/{session_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&[(
                "expand[]",
                "total_details.breakdown.discounts.discount.coupon",
            )])
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    /// Get subscription details.
    ///
    /// P1-8 FIX (audit FULL_REPO_AUDIT_2026-05-17): the Stripe 2026 API moved
    /// the billing-period timestamps off the top-level Subscription object and
    /// into `items.data[0].current_period_start/end`. A plain GET no longer
    /// returns those item fields populated, so `get_current_period()` fell back
    /// to `(0, 0)` and memberships were persisted with
    /// `current_period_start/end = 1970-01-01`. We now explicitly expand
    /// `items.data` (matching how `get_checkout_session` /
    /// `retrieve_checkout_session` already expand nested objects) so the period
    /// timestamps are present in the response.
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<StripeSubscription> {
        let response: StripeSubscription = self
            .client
            .get(format!("{STRIPE_API_BASE}/subscriptions/{subscription_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&[("expand[]", "items.data")])
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Cancel subscription
    pub async fn cancel_subscription(
        &self,
        subscription_id: &str,
        immediately: bool,
    ) -> Result<StripeSubscription> {
        if immediately {
            let response: StripeSubscription = self
                .client
                .delete(format!("{STRIPE_API_BASE}/subscriptions/{subscription_id}"))
                .basic_auth(&self.secret_key, None::<&str>)
                .header("Stripe-Version", STRIPE_API_VERSION)
                .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
                .send()
                .await?
                .json()
                .await?;

            Ok(response)
        } else {
            let response: StripeSubscription = self
                .client
                .post(format!("{STRIPE_API_BASE}/subscriptions/{subscription_id}"))
                .basic_auth(&self.secret_key, None::<&str>)
                .header("Stripe-Version", STRIPE_API_VERSION)
                .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
                .form(&[("cancel_at_period_end", "true")])
                .send()
                .await?
                .json()
                .await?;

            Ok(response)
        }
    }

    /// Update subscription fields via Stripe PATCH
    pub async fn update_subscription(
        &self,
        subscription_id: &str,
        params: &[(&str, &str)],
    ) -> Result<StripeSubscription> {
        let response: StripeSubscription = self
            .client
            .post(format!("{STRIPE_API_BASE}/subscriptions/{subscription_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Create refund.
    ///
    /// P2-A FIX (audit FULL_REPO_AUDIT_2026-05-17): previously every call sent a
    /// fresh `uuid::Uuid::new_v4()` Idempotency-Key, so a webhook retry or a
    /// user double-submit that re-invoked this method issued a *second* refund
    /// against the same payment intent. The Idempotency-Key is now derived
    /// deterministically from the logical refund operation
    /// (`payment_intent + amount`) so a retry of the SAME logical refund
    /// reuses the SAME key and Stripe collapses it to one refund.
    ///
    /// For callers that own a higher-level logical operation id (e.g. the
    /// originating order/refund-request id) prefer
    /// [`StripeService::create_refund_with_key`] and thread that id through;
    /// this 3-arg form keeps its public signature for existing callers
    /// (`routes/orders.rs`) and derives the key from the refund parameters.
    pub async fn create_refund(
        &self,
        payment_intent_id: &str,
        amount: Option<i64>,
        reason: Option<&str>,
    ) -> Result<StripeRefund> {
        // Deterministic per (payment_intent, amount). A full refund and a
        // distinct partial-refund amount get distinct keys (they are distinct
        // logical operations); a retry of the same logical refund reuses it.
        let logical_op = format!(
            "refund:{}:{}",
            payment_intent_id,
            amount
                .map(|a| a.to_string())
                .unwrap_or_else(|| "full".to_string())
        );
        self.create_refund_with_key(payment_intent_id, amount, reason, &logical_op)
            .await
    }

    /// Create refund with an explicit caller-supplied logical-operation key.
    ///
    /// `logical_op` MUST be stable across retries of the SAME logical refund
    /// (e.g. derived from the originating order id / refund-request id). It is
    /// hashed into a deterministic Stripe Idempotency-Key so that Stripe
    /// dedupes a replayed refund instead of issuing a second one.
    pub async fn create_refund_with_key(
        &self,
        payment_intent_id: &str,
        amount: Option<i64>,
        reason: Option<&str>,
        logical_op: &str,
    ) -> Result<StripeRefund> {
        let mut params: Vec<(&str, String)> =
            vec![("payment_intent", payment_intent_id.to_string())];

        if let Some(amt) = amount {
            params.push(("amount", amt.to_string()));
        }

        if let Some(r) = reason {
            params.push(("reason", r.to_string()));
        }

        let response: StripeRefund = self
            .client
            .post(format!("{STRIPE_API_BASE}/refunds"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header(
                "Idempotency-Key",
                deterministic_idempotency_key("refund", logical_op),
            )
            .form(
                &params
                    .iter()
                    .map(|(k, v)| (*k, v.as_str()))
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Verify Stripe webhook signature using HMAC-SHA256
    ///
    /// Stripe signature header format: t=timestamp,v1=signature
    /// The signed payload is: {timestamp}.{payload}
    pub fn verify_webhook(&self, payload: &[u8], signature_header: &str) -> Result<bool> {
        let webhook_secret = self
            .webhook_secret
            .as_ref()
            .ok_or_else(|| anyhow!("Webhook secret not configured"))?;

        // Parse the signature header
        let mut timestamp: Option<&str> = None;
        let mut signatures: Vec<&str> = Vec::new();

        for part in signature_header.split(',') {
            let mut kv = part.splitn(2, '=');
            if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                match key {
                    "t" => timestamp = Some(value),
                    "v1" => signatures.push(value),
                    _ => {} // Ignore unknown keys
                }
            }
        }

        let timestamp =
            timestamp.ok_or_else(|| anyhow!("Missing timestamp in signature header"))?;

        if signatures.is_empty() {
            return Err(anyhow!("No v1 signature found in header"));
        }

        // Verify timestamp is within tolerance (5 minutes)
        let timestamp_i64: i64 = timestamp
            .parse()
            .map_err(|_| anyhow!("Invalid timestamp format"))?;
        let now = chrono::Utc::now().timestamp();
        let tolerance = 300; // 5 minutes

        if (now - timestamp_i64).abs() > tolerance {
            return Err(anyhow!("Timestamp outside tolerance window"));
        }

        // Compute expected signature — payload must be exact bytes; lossy conversion breaks HMAC
        let payload_str = std::str::from_utf8(payload)
            .map_err(|_| anyhow!("Webhook payload contains invalid UTF-8"))?;
        let signed_payload = format!("{timestamp}.{payload_str}");

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
            .map_err(|_| anyhow!("Invalid webhook secret"))?;
        mac.update(signed_payload.as_bytes());
        let expected = hex::encode(mac.finalize().into_bytes());

        // Compare with provided signatures
        for sig in signatures {
            if constant_time_compare(sig, &expected) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Parse webhook event from JSON payload
    pub fn parse_webhook_event(&self, payload: &str) -> Result<WebhookEvent> {
        Ok(serde_json::from_str(payload)?)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PAYMENT RETRY - ICT 7 Fix
    // ═══════════════════════════════════════════════════════════════════════════

    /// Update default payment method for a customer
    pub async fn update_default_payment_method(
        &self,
        customer_id: &str,
        payment_method_id: &str,
    ) -> Result<()> {
        // Attach payment method to customer
        self.client
            .post(format!(
                "{STRIPE_API_BASE}/payment_methods/{payment_method_id}/attach"
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(&[("customer", customer_id)])
            .send()
            .await?
            .error_for_status()?;

        // Set as default invoice payment method
        self.client
            .post(format!("{STRIPE_API_BASE}/customers/{customer_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(&[(
                "invoice_settings[default_payment_method]",
                payment_method_id,
            )])
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Retry a failed subscription payment
    pub async fn retry_subscription_payment(
        &self,
        subscription_id: &str,
    ) -> Result<RetryPaymentResult> {
        // Get the latest open invoice for this subscription
        #[derive(Deserialize)]
        struct InvoiceList {
            data: Vec<StripeInvoice>,
        }

        #[derive(Deserialize)]
        struct StripeInvoice {
            id: String,
            status: String,
        }

        let invoices: InvoiceList = self
            .client
            .get(format!("{STRIPE_API_BASE}/invoices"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&[
                ("subscription", subscription_id),
                ("status", "open"),
                ("limit", "1"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let invoice = invoices
            .data
            .first()
            .ok_or_else(|| anyhow::anyhow!("No open invoice found for subscription"))?;

        // Pay the invoice
        let response = self
            .client
            .post(format!("{}/invoices/{}/pay", STRIPE_API_BASE, invoice.id))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(RetryPaymentResult {
                success: true,
                error: None,
            })
        } else {
            let error_body: serde_json::Value = response.json().await?;
            let error_message = error_body["error"]["message"]
                .as_str()
                .unwrap_or("Payment failed")
                .to_string();

            Ok(RetryPaymentResult {
                success: false,
                error: Some(error_message),
            })
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PAYMENT METHODS MANAGEMENT - ICT 7 Fix
    // ═══════════════════════════════════════════════════════════════════════════

    /// List payment methods for a customer
    /// GET /v1/payment_methods?customer={customer_id}&type=card
    pub async fn list_payment_methods(
        &self,
        customer_id: &str,
    ) -> Result<Vec<StripePaymentMethod>> {
        let response: PaymentMethodsList = self
            .client
            .get(format!("{STRIPE_API_BASE}/payment_methods"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&[
                ("customer", customer_id),
                ("type", "card"),
                ("limit", "100"),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(response.data)
    }

    /// Attach a payment method to a customer
    /// POST /v1/payment_methods/{id}/attach
    pub async fn attach_payment_method(
        &self,
        payment_method_id: &str,
        customer_id: &str,
    ) -> Result<StripePaymentMethod> {
        let response: StripePaymentMethod = self
            .client
            .post(format!(
                "{STRIPE_API_BASE}/payment_methods/{payment_method_id}/attach"
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(&[("customer", customer_id)])
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Detach a payment method from a customer
    /// POST /v1/payment_methods/{id}/detach
    pub async fn detach_payment_method(
        &self,
        payment_method_id: &str,
    ) -> Result<StripePaymentMethod> {
        let response: StripePaymentMethod = self
            .client
            .post(format!(
                "{STRIPE_API_BASE}/payment_methods/{payment_method_id}/detach"
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Retrieve a specific payment method
    /// GET /v1/payment_methods/{id}
    pub async fn get_payment_method(&self, payment_method_id: &str) -> Result<StripePaymentMethod> {
        let response: StripePaymentMethod = self
            .client
            .get(format!(
                "{STRIPE_API_BASE}/payment_methods/{payment_method_id}"
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PRICE & PRODUCT MANAGEMENT (Admin Price Changes)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Create a new Stripe Product. Returns the product ID.
    pub async fn create_product(&self, name: &str) -> Result<String> {
        let response = self
            .client
            .post(format!("{STRIPE_API_BASE}/products"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(&[("name", name)])
            .send()
            .await?;

        if !response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe error creating product: {}",
                body["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }

        let body: serde_json::Value = response.json().await?;
        body["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Stripe product response missing id"))
    }

    /// Create a new Stripe Price.
    ///
    /// `billing_interval` accepts:
    ///   - "month" | "year" → recurring price
    ///   - "one_time"        → non-recurring price
    pub async fn create_price(
        &self,
        product_id: &str,
        unit_amount_cents: i64,
        currency: &str,
        billing_interval: &str,
    ) -> Result<StripePrice> {
        let mut params: Vec<(String, String)> = vec![
            ("product".to_string(), product_id.to_string()),
            ("unit_amount".to_string(), unit_amount_cents.to_string()),
            ("currency".to_string(), currency.to_lowercase()),
        ];

        match billing_interval {
            "month" | "year" => {
                params.push((
                    "recurring[interval]".to_string(),
                    billing_interval.to_string(),
                ));
                params.push(("recurring[interval_count]".to_string(), "1".to_string()));
            }
            "one_time" => {} // No recurring stanza
            other => {
                return Err(anyhow!(
                    "Unsupported billing_interval '{other}': expected month|year|one_time"
                ));
            }
        }

        let response = self
            .client
            .post(format!("{STRIPE_API_BASE}/prices"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(
                &params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;

        if !response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe error creating price: {}",
                body["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }

        Ok(response.json().await?)
    }

    /// Retrieve a Stripe Price by ID.
    pub async fn get_price(&self, price_id: &str) -> Result<StripePrice> {
        let response = self
            .client
            .get(format!("{STRIPE_API_BASE}/prices/{price_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .send()
            .await?;

        if !response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe error retrieving price: {}",
                body["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }

        Ok(response.json().await?)
    }

    /// Mark a Stripe Price as inactive. Existing subscriptions on this price
    /// are unaffected.
    pub async fn deactivate_price(&self, price_id: &str) -> Result<()> {
        self.client
            .post(format!("{STRIPE_API_BASE}/prices/{price_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(&[("active", "false")])
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// List subscription items (so we know which item-id to PATCH when
    /// migrating to a new price).
    pub async fn list_subscription_items(
        &self,
        subscription_id: &str,
    ) -> Result<Vec<StripeSubscriptionItem>> {
        #[derive(Deserialize)]
        struct ItemsResp {
            data: Vec<StripeSubscriptionItem>,
        }

        let response = self
            .client
            .get(format!("{STRIPE_API_BASE}/subscription_items"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .query(&[("subscription", subscription_id), ("limit", "100")])
            .send()
            .await?;

        if !response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe error listing subscription items: {}",
                body["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }

        let parsed: ItemsResp = response.json().await?;
        Ok(parsed.data)
    }

    /// Migrate an existing subscription onto a new Stripe Price.
    ///
    /// `proration_behavior`:
    ///   - "none"               → take effect at next billing cycle (no proration)
    ///   - "create_prorations"  → take effect immediately, prorated
    ///
    /// For the no-proration path we additionally pin
    /// `billing_cycle_anchor=unchanged` so the renewal date does not jump.
    pub async fn migrate_subscription_to_price(
        &self,
        subscription_id: &str,
        new_price_id: &str,
        proration_behavior: &str,
    ) -> Result<()> {
        let items = self.list_subscription_items(subscription_id).await?;
        let item = items.first().ok_or_else(|| {
            anyhow!("Subscription {subscription_id} has no items; cannot migrate price")
        })?;

        let mut params: Vec<(String, String)> = vec![
            ("items[0][id]".to_string(), item.id.clone()),
            ("items[0][price]".to_string(), new_price_id.to_string()),
            (
                "proration_behavior".to_string(),
                proration_behavior.to_string(),
            ),
        ];

        if proration_behavior == "none" {
            params.push(("billing_cycle_anchor".to_string(), "unchanged".to_string()));
        }

        let response = self
            .client
            .post(format!("{STRIPE_API_BASE}/subscriptions/{subscription_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .form(
                &params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;

        if !response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            return Err(anyhow!(
                "Stripe error migrating subscription {}: {}",
                subscription_id,
                body["error"]["message"].as_str().unwrap_or("unknown")
            ));
        }

        Ok(())
    }

    /// List all subscriptions with the given status, paginated.
    /// Returns every subscription page until Stripe says has_more=false.
    pub async fn list_subscriptions(&self, status: &str) -> Result<Vec<StripeSubscription>> {
        #[derive(Deserialize)]
        struct Page {
            data: Vec<StripeSubscription>,
            has_more: bool,
        }

        let mut all: Vec<StripeSubscription> = Vec::new();
        let mut starting_after: Option<String> = None;

        loop {
            let mut params = vec![
                ("status".to_string(), status.to_string()),
                ("limit".to_string(), "100".to_string()),
            ];
            if let Some(ref cursor) = starting_after {
                params.push(("starting_after".to_string(), cursor.clone()));
            }

            let page: Page = self
                .client
                .get(format!("{STRIPE_API_BASE}/subscriptions"))
                .basic_auth(&self.secret_key, None::<&str>)
                .header("Stripe-Version", STRIPE_API_VERSION)
                .query(&params)
                .send()
                .await?
                .json()
                .await?;

            let has_more = page.has_more;
            starting_after = page.data.last().map(|s| s.id.clone());
            all.extend(page.data);

            if !has_more {
                break;
            }
        }

        Ok(all)
    }

    /// Get customer's default payment method
    /// Returns the default payment method ID if set
    pub async fn get_customer_default_payment_method(
        &self,
        customer_id: &str,
    ) -> Result<Option<String>> {
        let response: serde_json::Value = self
            .client
            .get(format!("{STRIPE_API_BASE}/customers/{customer_id}"))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .send()
            .await?
            .json()
            .await?;

        // Check invoice_settings.default_payment_method first, then default_source
        let default_pm = response["invoice_settings"]["default_payment_method"]
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| response["default_source"].as_str().map(|s| s.to_string()));

        Ok(default_pm)
    }
}

/// Stripe webhook event
#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: WebhookEventData,
    pub created: i64,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEventData {
    pub object: serde_json::Value,
}

impl WebhookEvent {
    /// Extract checkout session from event
    pub fn as_checkout_session(&self) -> Option<StripeCheckoutSession> {
        serde_json::from_value(self.data.object.clone()).ok()
    }

    /// Extract subscription from event
    pub fn as_subscription(&self) -> Option<StripeSubscription> {
        serde_json::from_value(self.data.object.clone()).ok()
    }

    /// Get order ID from metadata
    pub fn get_order_id(&self) -> Option<i64> {
        self.data.object["metadata"]["order_id"]
            .as_str()
            .and_then(|s| s.parse().ok())
    }

    /// Get user ID from metadata
    pub fn get_user_id(&self) -> Option<i64> {
        self.data.object["metadata"]["user_id"]
            .as_str()
            .and_then(|s| s.parse().ok())
    }
}

/// Derive a deterministic Stripe Idempotency-Key from a logical operation.
///
/// P2-A FIX (audit FULL_REPO_AUDIT_2026-05-17): mutating Stripe calls must
/// reuse the SAME Idempotency-Key when the SAME logical operation is retried
/// (webhook redelivery, user double-submit) so Stripe collapses the duplicate
/// instead of charging/refunding twice. `domain` namespaces the operation
/// class (e.g. "refund") and `logical_op` is the stable identity of the
/// operation (order id, subscription id, payment-intent, …). We SHA-256 the
/// `domain:logical_op` tuple so arbitrary-length identifiers map to a
/// fixed-length, collision-resistant key well under Stripe's 255-char limit.
///
/// This is the compile-verifiable, no-DB form. The ideal hardening (G0.3,
/// owner-gated, NOT verifiable here) is to additionally persist the resulting
/// key on the originating DB row so the SAME key is reused even across process
/// restarts where the caller no longer holds the logical id in memory; that
/// requires a schema column and is documented as a follow-up.
fn deterministic_idempotency_key(domain: &str, logical_op: &str) -> String {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    hasher.update(b":");
    hasher.update(logical_op.as_bytes());
    format!("{}-{}", domain, hex::encode(hasher.finalize()))
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare("abc", "abc"));
        assert!(!constant_time_compare("abc", "abd"));
        assert!(!constant_time_compare("abc", "ab"));
    }
}
