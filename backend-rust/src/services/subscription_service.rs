//! Subscription Service
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, models::SubscriptionWithPlan};

pub struct SubscriptionService<'a> {
    db: &'a PgPool,
}

impl<'a> SubscriptionService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_subscriptions(&self, user_id: Uuid) -> Result<Vec<SubscriptionWithPlan>, AppError> {
        // TODO: Implement actual subscriptions query
        let _ = user_id;
        Ok(vec![])
    }

    pub async fn get_user_subscription(&self, user_id: Uuid, subscription_id: Uuid) -> Result<Option<SubscriptionWithPlan>, AppError> {
        // TODO: Implement actual subscription query
        let _ = (user_id, subscription_id);
        Ok(None)
    }
}
