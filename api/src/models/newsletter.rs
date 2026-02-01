//! Newsletter & Email models - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! GDPR Compliant data structures with consent tracking

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Newsletter subscriber with GDPR compliance fields
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewsletterSubscriber {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub status: String, // pending, confirmed, unsubscribed

    // Tracking fields
    pub source: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    // GDPR Compliance fields
    pub gdpr_consent: bool,
    pub consent_ip: Option<String>,
    pub consent_source: Option<String>,
    pub consent_timestamp: Option<NaiveDateTime>,

    // Status timestamps
    pub confirmed_at: Option<NaiveDateTime>,
    pub unsubscribed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Subscribe to newsletter request with GDPR consent
#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeRequest {
    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 254, message = "Email too long"))]
    pub email: String,

    #[validate(length(max = 255, message = "Name too long"))]
    pub name: Option<String>,

    #[validate(length(max = 100, message = "Source identifier too long"))]
    pub source: Option<String>,

    pub tags: Option<Vec<String>>,

    /// GDPR: Explicit consent is REQUIRED
    pub gdpr_consent: bool,
}

/// Unsubscribe request with reason tracking
#[derive(Debug, Deserialize)]
pub struct UnsubscribeRequest {
    pub token: String,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Email template
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailTemplate {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub subject: String,
    pub html_content: String,
    pub text_content: Option<String>,
    pub template_type: String, // transactional, marketing, automated
    pub variables: Option<serde_json::Value>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create email template request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmailTemplate {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    pub html_content: String,
    pub text_content: Option<String>,
    pub template_type: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

/// Email campaign
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailCampaign {
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub template_id: Option<i64>,
    pub html_content: Option<String>,
    pub text_content: Option<String>,
    pub status: String, // draft, scheduled, sending, sent, cancelled
    pub segment_id: Option<i64>,
    pub recipient_count: i32,
    pub sent_count: i32,
    pub open_count: i32,
    pub click_count: i32,
    pub bounce_count: i32,
    pub unsubscribe_count: i32,
    pub scheduled_at: Option<NaiveDateTime>,
    pub sent_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create email campaign request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmailCampaign {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    pub template_id: Option<i64>,
    pub html_content: Option<String>,
    pub text_content: Option<String>,
    pub segment_id: Option<i64>,
    pub scheduled_at: Option<NaiveDateTime>,
}

/// Email send log
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailLog {
    pub id: i64,
    pub campaign_id: Option<i64>,
    pub subscriber_id: Option<i64>,
    pub user_id: Option<i64>,
    pub email: String,
    pub subject: String,
    pub template_type: String,
    pub status: String, // sent, delivered, bounced, opened, clicked
    pub provider: String,
    pub provider_message_id: Option<String>,
    pub opened_at: Option<NaiveDateTime>,
    pub clicked_at: Option<NaiveDateTime>,
    pub bounced_at: Option<NaiveDateTime>,
    pub error_message: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
}

/// Email metrics
#[derive(Debug, Serialize)]
pub struct EmailMetrics {
    pub total_sent: i64,
    pub total_delivered: i64,
    pub total_opened: i64,
    pub total_clicked: i64,
    pub total_bounced: i64,
    pub total_unsubscribed: i64,
    pub open_rate: f64,
    pub click_rate: f64,
    pub bounce_rate: f64,
    pub unsubscribe_rate: f64,
}
