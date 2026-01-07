//! Subscription Model
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub status: String,
    pub price: f64,
    pub currency: String,
    pub billing_period: String,
    pub start_date: DateTime<Utc>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub trial_end_date: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub paused_at: Option<DateTime<Utc>>,
    pub stripe_subscription_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SubscriptionWithPlan {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub plan_name: String,
    pub plan_description: Option<String>,
    pub status: String,
    pub price: f64,
    pub currency: String,
    pub billing_period: String,
    pub start_date: DateTime<Utc>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub currency: String,
    pub billing_period: String,
    pub trial_days: i32,
    pub is_active: bool,
}
