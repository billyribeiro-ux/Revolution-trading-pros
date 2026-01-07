//! Subscription Service
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{errors::AppError, models::SubscriptionWithPlan};

/// Row struct for query results
#[derive(FromRow)]
struct SubscriptionRow {
    id: Uuid,
    status: String,
    price: f64,
    currency: String,
    billing_period: String,
    start_date: DateTime<Utc>,
    next_payment_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    plan_id: Uuid,
    plan_name: String,
    plan_description: Option<String>,
}

pub struct SubscriptionService<'a> {
    db: &'a PgPool,
}

impl<'a> SubscriptionService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_subscriptions(&self, user_id: Uuid) -> Result<Vec<SubscriptionWithPlan>, AppError> {
        let subscriptions: Vec<SubscriptionRow> = sqlx::query_as(
            r#"
            SELECT 
                us.id,
                us.status,
                us.price,
                us.currency,
                us.billing_period,
                us.start_date,
                us.next_payment_date,
                us.created_at,
                sp.id as plan_id,
                sp.name as plan_name,
                sp.description as plan_description
            FROM user_subscriptions us
            JOIN subscription_plans sp ON us.plan_id = sp.id
            WHERE us.user_id = $1
            ORDER BY us.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(self.db)
        .await?;

        Ok(subscriptions.into_iter().map(|s| SubscriptionWithPlan {
            id: s.id,
            plan_id: s.plan_id,
            plan_name: s.plan_name,
            plan_description: s.plan_description,
            status: s.status,
            price: s.price,
            currency: s.currency,
            billing_period: s.billing_period,
            start_date: s.start_date,
            next_payment_date: s.next_payment_date,
            created_at: s.created_at,
        }).collect())
    }

    pub async fn get_user_subscription(&self, user_id: Uuid, subscription_id: Uuid) -> Result<Option<SubscriptionWithPlan>, AppError> {
        let subscription: Option<SubscriptionRow> = sqlx::query_as(
            r#"
            SELECT 
                us.id,
                us.status,
                us.price,
                us.currency,
                us.billing_period,
                us.start_date,
                us.next_payment_date,
                us.created_at,
                sp.id as plan_id,
                sp.name as plan_name,
                sp.description as plan_description
            FROM user_subscriptions us
            JOIN subscription_plans sp ON us.plan_id = sp.id
            WHERE us.id = $1 AND us.user_id = $2
            "#
        )
        .bind(subscription_id)
        .bind(user_id)
        .fetch_optional(self.db)
        .await?;

        Ok(subscription.map(|s| SubscriptionWithPlan {
            id: s.id,
            plan_id: s.plan_id,
            plan_name: s.plan_name,
            plan_description: s.plan_description,
            status: s.status,
            price: s.price,
            currency: s.currency,
            billing_period: s.billing_period,
            start_date: s.start_date,
            next_payment_date: s.next_payment_date,
            created_at: s.created_at,
        }))
    }
}
