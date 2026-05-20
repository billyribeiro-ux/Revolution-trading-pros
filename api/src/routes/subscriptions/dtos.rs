//! Subscription DTOs — shared request/response types.
//!
//! R15-B (2026-05-20): extracted from the original 1,690-LOC
//! `subscriptions.rs`. These structs are re-exported from `mod.rs`
//! so the public path `routes::subscriptions::{MembershipPlanRow,
//! UserSubscriptionRow, CreateSubscriptionRequest, ...}` is preserved
//! byte-for-byte (the integration tests in `tests/subscriptions_test.rs`
//! depend on it).

use serde::Deserialize;

/// Membership plan row (price in integer cents per architecture standard §1.2)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct MembershipPlanRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Extended membership plan with room info (price in integer cents)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct MembershipPlanExtended {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub interval_count: Option<i32>,
    pub savings_percent: Option<i32>,
    pub is_popular: Option<bool>,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub stripe_product_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub sort_order: Option<i32>,
    pub room_id: Option<i64>,
    pub room_name: Option<String>,
    pub room_slug: Option<String>,
}

/// User subscription row - flexible schema for production compatibility
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct UserSubscriptionRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: Option<i64>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub current_period_start: Option<chrono::NaiveDateTime>,
    pub current_period_end: Option<chrono::NaiveDateTime>,
    pub cancel_at_period_end: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Create subscription request
#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: i64,
    pub payment_method_id: Option<String>,
    pub coupon_code: Option<String>,
}

/// Cancel subscription request
#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    pub cancel_immediately: Option<bool>,
    pub reason: Option<String>,
}

/// Upgrade/Downgrade subscription request (ICT 7+ Enterprise)
#[derive(Debug, Deserialize)]
pub struct ChangePlanRequest {
    pub new_plan_id: i64,
    pub prorate: Option<bool>,
}

/// Proration calculation result (all monetary in integer cents)
#[derive(Debug, serde::Serialize)]
pub struct ProrationPreview {
    pub current_plan_credit_cents: i64,
    pub new_plan_cost_cents: i64,
    pub proration_amount_cents: i64,
    pub effective_date: String,
    pub billing_cycle_days_remaining: i64,
}

/// Extended subscription row with plan details (ICT 7+ Enterprise)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct UserSubscriptionWithPlanRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: Option<i64>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub trial_ends_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub current_period_start: Option<chrono::NaiveDateTime>,
    pub current_period_end: Option<chrono::NaiveDateTime>,
    pub cancel_at_period_end: Option<bool>,
    pub grace_period_end: Option<chrono::NaiveDateTime>,
    pub failed_payment_count: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    // Plan details (joined)
    pub plan_name: Option<String>,
    pub plan_price_cents: Option<i64>,
    pub plan_billing_cycle: Option<String>,
    pub plan_features: Option<serde_json::Value>,
    pub plan_trial_days: Option<i32>,
}
