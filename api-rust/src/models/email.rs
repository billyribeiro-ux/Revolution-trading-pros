//! Email model and related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiError;

/// Email template entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub subject: String,
    pub html_content: String,
    pub text_content: Option<String>,
    pub template_type: EmailTemplateType,
    pub variables: Option<Vec<String>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailTemplateType {
    Transactional,
    Marketing,
    System,
    Custom,
}

/// Email campaign entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub preview_text: Option<String>,
    pub from_name: String,
    pub from_email: String,
    pub reply_to: Option<String>,
    pub template_id: Option<Uuid>,
    pub html_content: String,
    pub text_content: Option<String>,
    pub status: CampaignStatus,
    pub segment_type: SegmentType,
    pub segment_filters: Option<serde_json::Value>,
    pub recipient_count: i64,
    pub sent_count: i64,
    pub opened_count: i64,
    pub clicked_count: i64,
    pub bounced_count: i64,
    pub unsubscribed_count: i64,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Sent,
    Paused,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentType {
    All,
    Subscribers,
    Members,
    Custom,
    Tag,
}

/// Email subscriber entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSubscriber {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub user_id: Option<Uuid>,
    pub status: SubscriberStatus,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub custom_fields: Option<serde_json::Value>,
    pub subscribed_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub unsubscribed_at: Option<DateTime<Utc>>,
    pub bounced_at: Option<DateTime<Utc>>,
    pub last_email_at: Option<DateTime<Utc>>,
    pub last_opened_at: Option<DateTime<Utc>>,
    pub last_clicked_at: Option<DateTime<Utc>>,
    pub email_count: i64,
    pub open_count: i64,
    pub click_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberStatus {
    Pending,
    Subscribed,
    Unsubscribed,
    Bounced,
    Complained,
}

/// Email domain entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDomain {
    pub id: Uuid,
    pub domain: String,
    pub status: DomainStatus,
    pub dkim_verified: bool,
    pub spf_verified: bool,
    pub dmarc_verified: bool,
    pub return_path_verified: bool,
    pub dns_records: Option<serde_json::Value>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainStatus {
    Pending,
    Verified,
    Failed,
}

/// Email webhook entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailWebhook {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEvent>,
    pub is_active: bool,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub failure_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    Delivered,
    Opened,
    Clicked,
    Bounced,
    Complained,
    Unsubscribed,
}

/// Email conversation (inbound email CRM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConversation {
    pub id: Uuid,
    pub subject: String,
    pub from_email: String,
    pub from_name: Option<String>,
    pub user_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub status: ConversationStatus,
    pub priority: ConversationPriority,
    pub tags: Vec<String>,
    pub is_starred: bool,
    pub is_spam: bool,
    pub message_count: i32,
    pub last_message_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    Open,
    Pending,
    Resolved,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Email message in conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub direction: MessageDirection,
    pub from_email: String,
    pub from_name: Option<String>,
    pub to_email: String,
    pub subject: String,
    pub html_body: Option<String>,
    pub text_body: Option<String>,
    pub attachments: Option<Vec<EmailAttachment>>,
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub url: String,
}

/// Newsletter category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterCategory {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub subscriber_count: i64,
    pub is_default: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Newsletter subscription request
#[derive(Debug, Deserialize)]
pub struct NewsletterSubscribeRequest {
    pub email: String,
    pub name: Option<String>,
    pub categories: Option<Vec<Uuid>>,
    pub source: Option<String>,
}

impl NewsletterSubscribeRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if !is_valid_email(&self.email) {
            return Err(ApiError::Validation("Invalid email address".to_string()));
        }
        Ok(())
    }
}

fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    if email.is_empty() { return false; }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 { return false; }
    !parts[0].is_empty() && !parts[1].is_empty() && parts[1].contains('.')
}

/// Create campaign request
#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub subject: String,
    pub preview_text: Option<String>,
    pub from_name: Option<String>,
    pub from_email: Option<String>,
    pub reply_to: Option<String>,
    pub template_id: Option<Uuid>,
    pub html_content: Option<String>,
    pub text_content: Option<String>,
    pub segment_type: Option<SegmentType>,
    pub segment_filters: Option<serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

impl CreateCampaignRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        if self.subject.trim().is_empty() {
            return Err(ApiError::Validation("Subject is required".to_string()));
        }
        Ok(())
    }
}

/// Create template request
#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub slug: Option<String>,
    pub subject: String,
    pub html_content: String,
    pub text_content: Option<String>,
    pub template_type: Option<EmailTemplateType>,
    pub variables: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

impl CreateTemplateRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        if self.subject.trim().is_empty() {
            return Err(ApiError::Validation("Subject is required".to_string()));
        }
        if self.html_content.trim().is_empty() {
            return Err(ApiError::Validation("HTML content is required".to_string()));
        }
        Ok(())
    }
}

/// Email metrics
#[derive(Debug, Serialize)]
pub struct EmailMetrics {
    pub total_subscribers: i64,
    pub active_subscribers: i64,
    pub total_campaigns: i64,
    pub emails_sent_today: i64,
    pub emails_sent_this_month: i64,
    pub open_rate: f32,
    pub click_rate: f32,
    pub bounce_rate: f32,
    pub unsubscribe_rate: f32,
}

/// Email audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAuditLog {
    pub id: Uuid,
    pub event_type: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
