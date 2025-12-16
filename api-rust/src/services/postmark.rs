//! Postmark email service

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// Postmark service for sending emails
pub struct PostmarkService {
    api_key: String,
    http_client: reqwest::Client,
    base_url: String,
    from_email: String,
    from_name: String,
}

impl PostmarkService {
    pub fn new(api_key: &str, from_email: &str, from_name: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            http_client: reqwest::Client::new(),
            base_url: "https://api.postmarkapp.com".to_string(),
            from_email: from_email.to_string(),
            from_name: from_name.to_string(),
        }
    }

    /// Send a single email
    pub async fn send_email(&self, request: SendEmailRequest) -> Result<SendEmailResponse, ApiError> {
        let payload = PostmarkEmail {
            from: format!("{} <{}>", self.from_name, self.from_email),
            to: request.to,
            subject: request.subject,
            html_body: request.html_body,
            text_body: request.text_body,
            reply_to: request.reply_to,
            tag: request.tag,
            track_opens: request.track_opens.unwrap_or(true),
            track_links: request.track_links.unwrap_or_else(|| "HtmlAndText".to_string()),
            metadata: request.metadata,
        };

        self.post("/email", &payload).await
    }

    /// Send email using a template
    pub async fn send_template_email(
        &self,
        to: &str,
        template_alias: &str,
        template_model: serde_json::Value,
    ) -> Result<SendEmailResponse, ApiError> {
        let payload = serde_json::json!({
            "From": format!("{} <{}>", self.from_name, self.from_email),
            "To": to,
            "TemplateAlias": template_alias,
            "TemplateModel": template_model,
            "TrackOpens": true,
            "TrackLinks": "HtmlAndText"
        });

        self.post("/email/withTemplate", &payload).await
    }

    /// Send batch emails
    pub async fn send_batch(&self, emails: Vec<SendEmailRequest>) -> Result<Vec<SendEmailResponse>, ApiError> {
        let payloads: Vec<PostmarkEmail> = emails
            .into_iter()
            .map(|req| PostmarkEmail {
                from: format!("{} <{}>", self.from_name, self.from_email),
                to: req.to,
                subject: req.subject,
                html_body: req.html_body,
                text_body: req.text_body,
                reply_to: req.reply_to,
                tag: req.tag,
                track_opens: req.track_opens.unwrap_or(true),
                track_links: req.track_links.unwrap_or_else(|| "HtmlAndText".to_string()),
                metadata: req.metadata,
            })
            .collect();

        self.post("/email/batch", &payloads).await
    }

    /// Get email delivery stats
    pub async fn get_delivery_stats(&self) -> Result<DeliveryStats, ApiError> {
        self.get("/deliverystats").await
    }

    /// Get bounces
    pub async fn get_bounces(&self, count: i32, offset: i32) -> Result<BouncesResponse, ApiError> {
        self.get(&format!("/bounces?count={}&offset={}", count, offset)).await
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, ApiError> {
        let response = self.http_client
            .get(format!("{}{}", self.base_url, path))
            .header("X-Postmark-Server-Token", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Postmark request failed: {}", e)))?;

        self.handle_response(response).await
    }

    async fn post<T: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let response = self.http_client
            .post(format!("{}{}", self.base_url, path))
            .header("X-Postmark-Server-Token", &self.api_key)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Postmark request failed: {}", e)))?;

        self.handle_response(response).await
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ApiError> {
        if !response.status().is_success() {
            let error: PostmarkError = response.json().await
                .unwrap_or(PostmarkError {
                    error_code: 0,
                    message: "Unknown Postmark error".to_string(),
                });
            return Err(ApiError::ExternalService(format!(
                "Postmark error {}: {}",
                error.error_code, error.message
            )));
        }

        response.json().await
            .map_err(|e| ApiError::ExternalService(format!("Failed to parse Postmark response: {}", e)))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PostmarkEmail {
    from: String,
    to: String,
    subject: String,
    html_body: Option<String>,
    text_body: Option<String>,
    reply_to: Option<String>,
    tag: Option<String>,
    track_opens: bool,
    track_links: String,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PostmarkError {
    error_code: i32,
    message: String,
}

/// Request to send an email
#[derive(Debug)]
pub struct SendEmailRequest {
    pub to: String,
    pub subject: String,
    pub html_body: Option<String>,
    pub text_body: Option<String>,
    pub reply_to: Option<String>,
    pub tag: Option<String>,
    pub track_opens: Option<bool>,
    pub track_links: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Response from sending an email
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailResponse {
    pub message_id: String,
    pub to: String,
    pub submitted_at: String,
    pub error_code: i32,
    pub message: String,
}

/// Delivery statistics
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeliveryStats {
    pub inactive_mails: i64,
    pub bounces: BounceStats,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BounceStats {
    pub hard_bounce: i64,
    pub soft_bounce: i64,
    pub spam_complaint: i64,
    pub unsubscribe: i64,
}

/// Bounces response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BouncesResponse {
    pub total_count: i64,
    pub bounces: Vec<Bounce>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bounce {
    pub id: i64,
    pub email: String,
    pub bounced_at: String,
    pub bounce_type: String,
    pub description: String,
}
