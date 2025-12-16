//! Stripe payment service

use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;

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

    /// Verify Stripe webhook signature using HMAC-SHA256
    ///
    /// Stripe signature header format: t=timestamp,v1=signature
    /// The signed payload is: {timestamp}.{payload}
    pub fn verify_webhook(&self, payload: &[u8], signature_header: &str, webhook_secret: &str) -> Result<bool> {
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

        let timestamp = timestamp
            .ok_or_else(|| anyhow::anyhow!("Missing timestamp in signature header"))?;

        if signatures.is_empty() {
            return Err(anyhow::anyhow!("No v1 signature found in header"));
        }

        // Verify timestamp is within tolerance (5 minutes)
        let timestamp_i64: i64 = timestamp.parse()
            .map_err(|_| anyhow::anyhow!("Invalid timestamp format"))?;
        let now = chrono::Utc::now().timestamp();
        let tolerance = 300; // 5 minutes

        if (now - timestamp_i64).abs() > tolerance {
            return Err(anyhow::anyhow!("Timestamp outside tolerance window"));
        }

        // Compute expected signature
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
            .map_err(|_| anyhow::anyhow!("Invalid webhook secret"))?;
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
