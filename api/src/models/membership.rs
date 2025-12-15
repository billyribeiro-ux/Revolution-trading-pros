//! Membership/Subscription model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Membership {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan: String, // "free", "basic", "pro", "enterprise"
    pub status: String, // "active", "cancelled", "past_due"
    pub stripe_subscription_id: Option<String>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseEnrollment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub enrolled_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress_percent: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckoutSession {
    pub price_id: String,
    pub success_url: String,
    pub cancel_url: String,
}
