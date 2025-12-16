//! Stripe payment service

use anyhow::Result;

#[derive(Clone)]
pub struct StripeService {
    secret_key: String,
}

impl StripeService {
    pub fn new(secret_key: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
        }
    }

    /// Create a checkout session for subscription
    pub async fn create_checkout_session(
        &self,
        customer_email: &str,
        price_id: &str,
        success_url: &str,
        cancel_url: &str,
    ) -> Result<String> {
        let client = reqwest::Client::new();

        let params = [
            ("customer_email", customer_email),
            ("mode", "subscription"),
            ("success_url", success_url),
            ("cancel_url", cancel_url),
            ("line_items[0][price]", price_id),
            ("line_items[0][quantity]", "1"),
        ];

        let response: serde_json::Value = client
            .post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth(&self.secret_key, None::<&str>)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        let url = response["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No checkout URL returned"))?;

        Ok(url.to_string())
    }

    /// Create a customer portal session
    pub async fn create_portal_session(
        &self,
        customer_id: &str,
        return_url: &str,
    ) -> Result<String> {
        let client = reqwest::Client::new();

        let params = [
            ("customer", customer_id),
            ("return_url", return_url),
        ];

        let response: serde_json::Value = client
            .post("https://api.stripe.com/v1/billing_portal/sessions")
            .basic_auth(&self.secret_key, None::<&str>)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        let url = response["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No portal URL returned"))?;

        Ok(url.to_string())
    }

    /// Verify webhook signature
    pub fn verify_webhook(&self, _payload: &[u8], _signature: &str, _secret: &str) -> Result<bool> {
        // TODO: Implement proper webhook verification using stripe-rust crate
        // This is a placeholder that should use HMAC-SHA256 to verify the signature
        Ok(true)
    }
}
