//! Stripe payment service
//!
//! WASM-compatible Stripe API client using fetch

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// Stripe service for payment processing
#[derive(Clone)]
pub struct StripeService {
    secret_key: String,
    webhook_secret: String,
    base_url: String,
}

impl StripeService {
    pub fn new(secret_key: &str, webhook_secret: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            webhook_secret: webhook_secret.to_string(),
            base_url: "https://api.stripe.com/v1".to_string(),
        }
    }

    /// Create a payment intent
    pub async fn create_payment_intent(
        &self,
        amount: i64,
        currency: &str,
        customer_id: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> Result<PaymentIntent, ApiError> {
        let mut params: Vec<(String, String)> = vec![
            ("amount".to_string(), amount.to_string()),
            ("currency".to_string(), currency.to_string()),
        ];

        if let Some(cid) = customer_id {
            params.push(("customer".to_string(), cid.to_string()));
        }

        if let Some(meta) = metadata {
            if let Some(obj) = meta.as_object() {
                for (k, v) in obj {
                    params.push((format!("metadata[{}]", k), v.to_string()));
                }
            }
        }

        self.post_owned("/payment_intents", params).await
    }

    /// Create a customer
    pub async fn create_customer(
        &self,
        email: &str,
        name: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> Result<Customer, ApiError> {
        let mut params: Vec<(String, String)> = vec![("email".to_string(), email.to_string())];

        if let Some(n) = name {
            params.push(("name".to_string(), n.to_string()));
        }

        if let Some(meta) = metadata {
            if let Some(obj) = meta.as_object() {
                for (k, v) in obj {
                    params.push((format!("metadata[{}]", k), v.to_string()));
                }
            }
        }

        self.post_owned("/customers", params).await
    }

    /// Create a subscription
    pub async fn create_subscription(
        &self,
        customer_id: &str,
        price_id: &str,
        trial_days: Option<i32>,
    ) -> Result<Subscription, ApiError> {
        let mut params = vec![
            ("customer", customer_id.to_string()),
            ("items[0][price]", price_id.to_string()),
        ];

        if let Some(days) = trial_days {
            params.push(("trial_period_days", days.to_string()));
        }

        self.post("/subscriptions", &params).await
    }

    /// Cancel a subscription
    pub async fn cancel_subscription(
        &self,
        subscription_id: &str,
        cancel_at_period_end: bool,
    ) -> Result<Subscription, ApiError> {
        if cancel_at_period_end {
            self.post(
                &format!("/subscriptions/{}", subscription_id),
                &[("cancel_at_period_end", "true".to_string())],
            )
            .await
        } else {
            self.delete(&format!("/subscriptions/{}", subscription_id)).await
        }
    }

    /// Retrieve a subscription
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<Subscription, ApiError> {
        self.get(&format!("/subscriptions/{}", subscription_id)).await
    }

    /// Create a checkout session
    pub async fn create_checkout_session(
        &self,
        customer_id: Option<&str>,
        price_id: &str,
        success_url: &str,
        cancel_url: &str,
        mode: &str,
    ) -> Result<CheckoutSession, ApiError> {
        let mut params = vec![
            ("line_items[0][price]", price_id.to_string()),
            ("line_items[0][quantity]", "1".to_string()),
            ("mode", mode.to_string()),
            ("success_url", success_url.to_string()),
            ("cancel_url", cancel_url.to_string()),
        ];

        if let Some(cid) = customer_id {
            params.push(("customer", cid.to_string()));
        }

        self.post("/checkout/sessions", &params).await
    }

    /// Verify webhook signature
    pub fn verify_webhook(&self, payload: &str, signature: &str) -> Result<WebhookEvent, ApiError> {
        // Parse the signature header
        let parts: std::collections::HashMap<&str, &str> = signature
            .split(',')
            .filter_map(|part| {
                let mut split = part.split('=');
                Some((split.next()?, split.next()?))
            })
            .collect();

        let timestamp = parts
            .get("t")
            .ok_or_else(|| ApiError::BadRequest("Missing timestamp".to_string()))?;
        let sig = parts
            .get("v1")
            .ok_or_else(|| ApiError::BadRequest("Missing signature".to_string()))?;

        // Compute expected signature
        let signed_payload = format!("{}.{}", timestamp, payload);
        let expected = Self::compute_hmac(&self.webhook_secret, &signed_payload);

        // Constant-time comparison
        if !Self::secure_compare(sig, &expected) {
            return Err(ApiError::Unauthorized("Invalid webhook signature".to_string()));
        }

        // Parse the event
        serde_json::from_str(payload)
            .map_err(|e| ApiError::BadRequest(format!("Invalid webhook payload: {}", e)))
    }

    fn compute_hmac(secret: &str, payload: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    fn secure_compare(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.bytes().zip(b.bytes()).fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
    }

    /// Make a GET request to Stripe API using worker fetch
    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.secret_key)).ok();
        headers.set("Content-Type", "application/x-www-form-urlencoded").ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Get);
        init.with_headers(headers);
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Stripe request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Make a POST request to Stripe API using worker fetch (owned params)
    async fn post_owned<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        params: Vec<(String, String)>,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        let body = serde_urlencoded::to_string(&params_ref)
            .map_err(|e| ApiError::Internal(format!("Failed to encode params: {}", e)))?;
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.secret_key)).ok();
        headers.set("Content-Type", "application/x-www-form-urlencoded").ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Post);
        init.with_headers(headers);
        init.with_body(Some(wasm_bindgen::JsValue::from_str(&body)));
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Stripe request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Make a POST request to Stripe API using worker fetch
    async fn post<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let body = serde_urlencoded::to_string(params)
            .map_err(|e| ApiError::Internal(format!("Failed to encode params: {}", e)))?;
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.secret_key)).ok();
        headers.set("Content-Type", "application/x-www-form-urlencoded").ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Post);
        init.with_headers(headers);
        init.with_body(Some(wasm_bindgen::JsValue::from_str(&body)));
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Stripe request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Make a DELETE request to Stripe API using worker fetch
    async fn delete<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.secret_key)).ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Delete);
        init.with_headers(headers);
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Stripe request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Handle Stripe API response
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: &mut worker::Response,
    ) -> Result<T, ApiError> {
        let status = response.status_code();
        let text = response.text().await
            .map_err(|e| ApiError::ExternalService(format!("Failed to read response: {:?}", e)))?;
        
        if status >= 400 {
            let error: StripeError = serde_json::from_str(&text)
                .unwrap_or(StripeError {
                    error: StripeErrorDetail {
                        message: text.clone(),
                        code: None,
                    },
                });
            return Err(ApiError::ExternalService(format!(
                "Stripe error: {}",
                error.error.message
            )));
        }

        serde_json::from_str(&text)
            .map_err(|e| ApiError::ExternalService(format!("Failed to parse Stripe response: {}", e)))
    }
}

#[derive(Debug, Deserialize)]
struct StripeError {
    error: StripeErrorDetail,
}

#[derive(Debug, Deserialize)]
struct StripeErrorDetail {
    message: String,
    code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub client_secret: Option<String>,
    pub customer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub customer: String,
    pub status: String,
    pub current_period_start: i64,
    pub current_period_end: i64,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub id: String,
    pub url: Option<String>,
    pub payment_status: String,
    pub customer: Option<String>,
    pub subscription: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: WebhookEventData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEventData {
    pub object: serde_json::Value,
}
