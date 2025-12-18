//! Subscription & Membership models - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Billing cycle enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annual,
}

/// Subscription status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
    Expired,
    Pending,
    PastDue,
    Paused,
}

/// Membership plan entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MembershipPlan {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: f64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create membership plan request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateMembershipPlan {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 0.0))]
    pub price: f64,
    #[validate(length(min = 1, max = 20))]
    pub billing_cycle: String,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
}

/// Update membership plan request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMembershipPlan {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub billing_cycle: Option<String>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
}

/// User subscription/membership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSubscription {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
    pub starts_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub current_period_start: Option<NaiveDateTime>,
    pub current_period_end: Option<NaiveDateTime>,
    pub cancel_at_period_end: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// User subscription with plan details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscriptionWithPlan {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
    pub plan_name: String,
    pub plan_price: f64,
    pub billing_cycle: String,
    pub starts_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub current_period_start: Option<NaiveDateTime>,
    pub current_period_end: Option<NaiveDateTime>,
    pub cancel_at_period_end: bool,
    pub created_at: NaiveDateTime,
}

/// Create subscription request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSubscription {
    pub plan_id: i64,
    pub payment_method_id: Option<String>,
    pub coupon_code: Option<String>,
}

/// Cancel subscription request
#[derive(Debug, Deserialize)]
pub struct CancelSubscription {
    pub cancel_immediately: Option<bool>,
    pub reason: Option<String>,
}

/// Subscription metrics response
#[derive(Debug, Serialize)]
pub struct SubscriptionMetrics {
    pub total_active: i64,
    pub total_cancelled: i64,
    pub total_expired: i64,
    pub mrr: f64, // Monthly Recurring Revenue
    pub arr: f64, // Annual Recurring Revenue
    pub churn_rate: f64,
    pub new_this_month: i64,
    pub cancelled_this_month: i64,
}
