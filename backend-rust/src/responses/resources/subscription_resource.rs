//! Subscription Resource
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubscriptionResource {
    pub id: String,
    pub plan_name: String,
    pub status: String,
    pub price: String,
    pub currency: String,
    pub billing_period: String,
    pub start_date: String,
    pub next_payment_date: Option<String>,
    pub trial_end_date: Option<String>,
    pub cancelled_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubscriptionDetailResource {
    pub id: String,
    pub plan_name: String,
    pub plan_description: Option<String>,
    pub status: String,
    pub price: String,
    pub currency: String,
    pub billing_period: String,
    pub start_date: String,
    pub next_payment_date: Option<String>,
    pub trial_end_date: Option<String>,
    pub cancelled_at: Option<String>,
    pub paused_at: Option<String>,
    pub features: Vec<String>,
}
