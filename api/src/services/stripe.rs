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

/// Stripe checkout session response
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCheckoutSession {
    pub id: String,
    pub url: Option<String>,
    pub payment_status: String,
    pub customer: Option<String>,
    pub subscription: Option<String>,
    pub payment_intent: Option<String>,
    pub amount_total: Option<i64>,
    pub currency: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Stripe subscription response
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeSubscription {
    pub id: String,
    pub status: String,
    pub customer: String,
    pub current_period_start: i64,
    pub current_period_end: i64,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
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

/// Checkout session configuration
#[derive(Debug)]
pub struct CheckoutConfig {
    pub customer_email: String,
    pub customer_name: Option<String>,
    pub line_items: Vec<LineItem>,
    pub success_url: String,
    pub cancel_url: String,
    pub metadata: HashMap<String, String>,
    pub allow_promotion_codes: bool,
    pub billing_address_collection: bool,
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
            .get(format!("{}/customers", STRIPE_API_BASE))
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
        let mut params: Vec<(&str, String)> = vec![("email", email.to_string())];

        if let Some(n) = name {
            params.push(("name", n.to_string()));
        }

        if let Some(meta) = metadata {
            for (k, v) in meta {
                params.push(("metadata", format!("[{}]={}", k, v)));
            }
        }

        let response: StripeCustomer = self
            .client
            .post(format!("{}/customers", STRIPE_API_BASE))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
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
                form_params.push((format!("line_items[{}][price]", i), price_id.clone()));
                form_params.push((
                    format!("line_items[{}][quantity]", i),
                    item.quantity.to_string(),
                ));
            } else {
                // Create ad-hoc price
                form_params.push((
                    format!("line_items[{}][price_data][currency]", i),
                    item.currency.clone(),
                ));
                form_params.push((
                    format!("line_items[{}][price_data][unit_amount]", i),
                    item.amount.to_string(),
                ));
                form_params.push((
                    format!("line_items[{}][price_data][product_data][name]", i),
                    item.name.clone(),
                ));

                if let Some(ref desc) = item.description {
                    form_params.push((
                        format!("line_items[{}][price_data][product_data][description]", i),
                        desc.clone(),
                    ));
                }

                // Subscription pricing
                if item.is_subscription {
                    let interval = item.interval.as_deref().unwrap_or("month");
                    let count = item.interval_count.unwrap_or(1);
                    form_params.push((
                        format!("line_items[{}][price_data][recurring][interval]", i),
                        interval.to_string(),
                    ));
                    form_params.push((
                        format!("line_items[{}][price_data][recurring][interval_count]", i),
                        count.to_string(),
                    ));
                }

                form_params.push((
                    format!("line_items[{}][quantity]", i),
                    item.quantity.to_string(),
                ));
            }
        }

        // Metadata
        for (k, v) in config.metadata {
            form_params.push((format!("metadata[{}]", k), v));
        }

        // Options
        if config.allow_promotion_codes {
            form_params.push(("allow_promotion_codes".to_string(), "true".to_string()));
        }

        if config.billing_address_collection {
            form_params.push((
                "billing_address_collection".to_string(),
                "required".to_string(),
            ));
        }

        // Payment method types
        form_params.push(("payment_method_types[0]".to_string(), "card".to_string()));

        let response = self
            .client
            .post(format!("{}/checkout/sessions", STRIPE_API_BASE))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
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
            billing_address_collection: false,
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
            .get(format!(
                "{}/checkout/sessions/{}",
                STRIPE_API_BASE, session_id
            ))
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
            .post(format!("{}/billing_portal/sessions", STRIPE_API_BASE))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.url)
    }

    /// Get subscription details
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<StripeSubscription> {
        let response: StripeSubscription = self
            .client
            .get(format!(
                "{}/subscriptions/{}",
                STRIPE_API_BASE, subscription_id
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
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
                .delete(format!(
                    "{}/subscriptions/{}",
                    STRIPE_API_BASE, subscription_id
                ))
                .basic_auth(&self.secret_key, None::<&str>)
                .header("Stripe-Version", STRIPE_API_VERSION)
                .send()
                .await?
                .json()
                .await?;

            Ok(response)
        } else {
            let response: StripeSubscription = self
                .client
                .post(format!(
                    "{}/subscriptions/{}",
                    STRIPE_API_BASE, subscription_id
                ))
                .basic_auth(&self.secret_key, None::<&str>)
                .header("Stripe-Version", STRIPE_API_VERSION)
                .form(&[("cancel_at_period_end", "true")])
                .send()
                .await?
                .json()
                .await?;

            Ok(response)
        }
    }

    /// Create refund
    pub async fn create_refund(
        &self,
        payment_intent_id: &str,
        amount: Option<i64>,
        reason: Option<&str>,
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
            .post(format!("{}/refunds", STRIPE_API_BASE))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
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

        // Compute expected signature
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));

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
            .post(&format!("payment_methods/{}/attach", payment_method_id))
            .form(&[("customer", customer_id)])
            .send()
            .await?
            .error_for_status()?;

        // Set as default invoice payment method
        self.client
            .post(&format!("customers/{}", customer_id))
            .form(&[("invoice_settings[default_payment_method]", payment_method_id)])
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Retry a failed subscription payment
    pub async fn retry_subscription_payment(&self, subscription_id: &str) -> Result<RetryPaymentResult> {
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
            .get("invoices")
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

        let invoice = invoices.data.first().ok_or_else(|| {
            anyhow::anyhow!("No open invoice found for subscription")
        })?;

        // Pay the invoice
        let response = self
            .client
            .post(&format!("invoices/{}/pay", invoice.id))
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
    pub async fn list_payment_methods(&self, customer_id: &str) -> Result<Vec<StripePaymentMethod>> {
        let response: PaymentMethodsList = self
            .client
            .get(format!("{}/payment_methods", STRIPE_API_BASE))
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
                "{}/payment_methods/{}/attach",
                STRIPE_API_BASE, payment_method_id
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .form(&[("customer", customer_id)])
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Detach a payment method from a customer
    /// POST /v1/payment_methods/{id}/detach
    pub async fn detach_payment_method(&self, payment_method_id: &str) -> Result<StripePaymentMethod> {
        let response: StripePaymentMethod = self
            .client
            .post(format!(
                "{}/payment_methods/{}/detach",
                STRIPE_API_BASE, payment_method_id
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
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
                "{}/payment_methods/{}",
                STRIPE_API_BASE, payment_method_id
            ))
            .basic_auth(&self.secret_key, None::<&str>)
            .header("Stripe-Version", STRIPE_API_VERSION)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Get customer's default payment method
    /// Returns the default payment method ID if set
    pub async fn get_customer_default_payment_method(&self, customer_id: &str) -> Result<Option<String>> {
        let response: serde_json::Value = self
            .client
            .get(format!("{}/customers/{}", STRIPE_API_BASE, customer_id))
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
