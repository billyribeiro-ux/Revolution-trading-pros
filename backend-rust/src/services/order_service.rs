//! Order Service
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, models::{OrderSummary, OrderWithItems}};

pub struct OrderService<'a> {
    db: &'a PgPool,
}

impl<'a> OrderService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_orders(&self, user_id: Uuid) -> Result<Vec<OrderSummary>, AppError> {
        // TODO: Implement actual orders query
        let _ = user_id;
        Ok(vec![])
    }

    pub async fn get_user_order(&self, user_id: Uuid, order_id: Uuid) -> Result<Option<OrderWithItems>, AppError> {
        // TODO: Implement actual order query with items
        let _ = (user_id, order_id);
        Ok(None)
    }
}
