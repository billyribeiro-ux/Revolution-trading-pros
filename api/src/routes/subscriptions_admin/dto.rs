//! Subscriptions Admin DTOs
//!
//! All shared request / SQL-row types for the subscriptions admin surface.
//! Re-exported through `super::mod` so the integration test continues to
//! resolve them at `revolution_api::routes::subscriptions_admin::{...}`.
//!
//! Money / cents — i64 ONLY, BIGINT ONLY (CLAUDE.md hard rule). All
//! `*_cents` fields here are `i64` end-to-end; no narrowing casts are
//! introduced by this structural split.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// PRICE-CHANGE TYPES (admin-driven, Stripe-syncing)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ChangePriceRequest {
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub billing_interval: String, // "month" | "year" | "one_time"
    pub apply_to: String,         // "new_only" | "next_renewal" | "immediate_proration"
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PriceHistoryRow {
    pub id: i64,
    pub plan_id: i64,
    pub old_stripe_price_id: Option<String>,
    pub new_stripe_price_id: String,
    pub old_amount_cents: Option<i64>,
    pub new_amount_cents: i64,
    pub currency: String,
    pub billing_interval: String,
    pub apply_to: String,
    pub subscriptions_migrated: i32,
    pub subscriptions_failed: i32,
    pub failure_details: Option<serde_json::Value>,
    pub changed_by_user_id: Option<i64>,
    pub changed_at: chrono::NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════
// DATA TYPES - SUBSCRIPTIONS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriptionRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: Option<i64>,
    pub status: String,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriptionPlanRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub trial_period_days: Option<i32>,
    pub trial_requires_payment_method: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub user_id: Option<i64>,
    pub plan_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PlanListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: i64,
    pub plan_id: i64,
    pub status: Option<String>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub status: Option<String>,
    pub plan_id: Option<i64>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub is_active: Option<bool>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub trial_period_days: Option<i32>,
    pub trial_requires_payment_method: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_cents: Option<i64>,
    pub billing_cycle: Option<String>,
    pub is_active: Option<bool>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub trial_period_days: Option<i32>,
    pub trial_requires_payment_method: Option<bool>,
}
