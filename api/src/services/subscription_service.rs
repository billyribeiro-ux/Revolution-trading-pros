//! Subscription Service
//! ICT 11+ Principal Engineer Implementation
use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::{models::UserSubscriptionWithPlan, utils::errors::ApiError};

#[derive(sqlx::FromRow)]
struct SubscriptionQueryRow {
    id: i64,
    user_id: i64,
    plan_id: i64,
    plan_name: String,
    plan_price: f64,
    billing_cycle: String,
    starts_at: NaiveDateTime,
    expires_at: Option<NaiveDateTime>,
    cancelled_at: Option<NaiveDateTime>,
    status: String,
    payment_provider: Option<String>,
    stripe_subscription_id: Option<String>,
    current_period_start: Option<NaiveDateTime>,
    current_period_end: Option<NaiveDateTime>,
    cancel_at_period_end: bool,
    created_at: NaiveDateTime,
}

pub struct SubscriptionService<'a> {
    db: &'a PgPool,
}

impl<'a> SubscriptionService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_subscriptions(
        &self,
        user_id: i64,
    ) -> Result<Vec<UserSubscriptionWithPlan>, ApiError> {
        let subscriptions = sqlx::query_as::<_, SubscriptionQueryRow>(
            r#"
            SELECT 
                um.id,
                um.user_id,
                um.plan_id,
                mp.name as plan_name,
                mp.price as plan_price,
                mp.billing_cycle,
                um.starts_at,
                um.expires_at,
                um.cancelled_at,
                um.status,
                um.payment_provider,
                um.stripe_subscription_id,
                um.current_period_start,
                um.current_period_end,
                um.cancel_at_period_end,
                um.created_at
            FROM user_memberships um
            JOIN membership_plans mp ON um.plan_id = mp.id
            WHERE um.user_id = $1
            ORDER BY um.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.db)
        .await
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

        Ok(subscriptions
            .into_iter()
            .map(|s| UserSubscriptionWithPlan {
                id: s.id,
                user_id: s.user_id,
                plan_id: s.plan_id,
                plan_name: s.plan_name,
                plan_price: s.plan_price,
                billing_cycle: s.billing_cycle,
                starts_at: s.starts_at,
                expires_at: s.expires_at,
                cancelled_at: s.cancelled_at,
                status: s.status,
                payment_provider: s.payment_provider,
                stripe_subscription_id: s.stripe_subscription_id,
                current_period_start: s.current_period_start,
                current_period_end: s.current_period_end,
                cancel_at_period_end: s.cancel_at_period_end,
                created_at: s.created_at,
            })
            .collect())
    }

    pub async fn get_user_subscription(
        &self,
        user_id: i64,
        subscription_id: i64,
    ) -> Result<Option<UserSubscriptionWithPlan>, ApiError> {

        let subscription = sqlx::query_as::<_, SubscriptionQueryRow>(
            r#"
            SELECT 
                um.id,
                um.user_id,
                um.plan_id,
                mp.name as plan_name,
                mp.price as plan_price,
                mp.billing_cycle,
                um.starts_at,
                um.expires_at,
                um.cancelled_at,
                um.status,
                um.payment_provider,
                um.stripe_subscription_id,
                um.current_period_start,
                um.current_period_end,
                um.cancel_at_period_end,
                um.created_at
            FROM user_memberships um
            JOIN membership_plans mp ON um.plan_id = mp.id
            WHERE um.id = $1 AND um.user_id = $2
            "#,
        )
        .bind(subscription_id)
        .bind(user_id)
        .fetch_optional(self.db)
        .await
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

        Ok(subscription.map(|s| UserSubscriptionWithPlan {
            id: s.id,
            user_id: s.user_id,
            plan_id: s.plan_id,
            plan_name: s.plan_name,
            plan_price: s.plan_price,
            billing_cycle: s.billing_cycle,
            starts_at: s.starts_at,
            expires_at: s.expires_at,
            cancelled_at: s.cancelled_at,
            status: s.status,
            payment_provider: s.payment_provider,
            stripe_subscription_id: s.stripe_subscription_id,
            current_period_start: s.current_period_start,
            current_period_end: s.current_period_end,
            cancel_at_period_end: s.cancel_at_period_end,
            created_at: s.created_at,
        }))
    }
}
