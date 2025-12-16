//! Subscription model and related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiError;

/// Subscription plan entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub product_id: Uuid,
    pub price: i64,  // cents
    pub currency: String,
    pub interval: BillingInterval,
    pub interval_count: i32,
    pub trial_days: Option<i32>,
    pub features: Option<Vec<String>>,
    pub stripe_price_id: Option<String>,
    pub is_active: bool,
    pub is_featured: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingInterval {
    Day,
    Week,
    Month,
    Year,
}

impl Default for BillingInterval {
    fn default() -> Self {
        Self::Month
    }
}

/// User subscription entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub product_id: Uuid,
    pub status: SubscriptionStatus,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub paused_at: Option<DateTime<Utc>>,
    pub resume_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Trialing,
    PastDue,
    Paused,
    Cancelled,
    Unpaid,
    Incomplete,
    IncompleteExpired,
}

impl SubscriptionStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active | Self::Trialing)
    }

    pub fn can_access_content(&self) -> bool {
        matches!(self, Self::Active | Self::Trialing | Self::PastDue)
    }
}

/// Subscription with plan details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionWithPlan {
    #[serde(flatten)]
    pub subscription: UserSubscription,
    pub plan: SubscriptionPlan,
    pub product_name: String,
}

/// Create subscription request
#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: Uuid,
    pub payment_method_id: Option<String>,
    pub coupon_code: Option<String>,
}

/// Update subscription request
#[derive(Debug, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub plan_id: Option<Uuid>,
    pub payment_method_id: Option<String>,
}

/// Cancel subscription request
#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    pub reason: Option<String>,
    pub feedback: Option<String>,
    pub cancel_immediately: Option<bool>,
}

/// Subscription invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInvoice {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub stripe_invoice_id: Option<String>,
    pub number: String,
    pub status: InvoiceStatus,
    pub subtotal: i64,
    pub discount: i64,
    pub tax: i64,
    pub total: i64,
    pub currency: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Void,
    Uncollectible,
}

/// Subscription payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPayment {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub stripe_payment_intent_id: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub status: PaymentStatus,
    pub payment_method: Option<String>,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Succeeded,
    Failed,
    Refunded,
    Cancelled,
}

/// Subscription metrics for dashboard
#[derive(Debug, Serialize)]
pub struct SubscriptionMetrics {
    pub total_active: i64,
    pub total_trialing: i64,
    pub total_cancelled: i64,
    pub mrr: i64,  // Monthly Recurring Revenue in cents
    pub arr: i64,  // Annual Recurring Revenue in cents
    pub churn_rate: f32,
    pub average_lifetime_value: i64,
    pub new_this_month: i64,
    pub cancelled_this_month: i64,
}

/// Subscription list query
#[derive(Debug, Deserialize)]
pub struct SubscriptionListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<SubscriptionStatus>,
    pub plan_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Create subscription plan request
#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub product_id: Uuid,
    pub price: i64,
    pub currency: Option<String>,
    pub interval: BillingInterval,
    pub interval_count: Option<i32>,
    pub trial_days: Option<i32>,
    pub features: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
}

impl CreatePlanRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        if self.price < 0 {
            return Err(ApiError::Validation("Price must be positive".to_string()));
        }
        Ok(())
    }
}

/// Update subscription plan request
#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub trial_days: Option<i32>,
    pub features: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
}
