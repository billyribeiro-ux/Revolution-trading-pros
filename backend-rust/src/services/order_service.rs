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
        let orders = sqlx::query_as!(
            OrderSummary,
            r#"
            SELECT 
                o.id,
                o.order_number,
                o.status,
                o.total,
                o.currency,
                o.created_at,
                COUNT(oi.id)::int as item_count
            FROM orders o
            LEFT JOIN order_items oi ON o.id = oi.order_id
            WHERE o.user_id = $1
            GROUP BY o.id, o.order_number, o.status, o.total, o.currency, o.created_at
            ORDER BY o.created_at DESC
            "#,
            user_id
        )
        .fetch_all(self.db)
        .await?;

        Ok(orders)
    }

    pub async fn get_user_order(&self, user_id: Uuid, order_id: Uuid) -> Result<Option<OrderWithItems>, AppError> {
        let order = sqlx::query!(
            r#"
            SELECT 
                id,
                order_number,
                status,
                subtotal,
                tax,
                discount,
                total,
                currency,
                payment_method,
                billing_address,
                created_at
            FROM orders
            WHERE id = $1 AND user_id = $2
            "#,
            order_id,
            user_id
        )
        .fetch_optional(self.db)
        .await?;

        let Some(order) = order else {
            return Ok(None);
        };

        let items = sqlx::query!(
            r#"
            SELECT 
                id,
                name,
                quantity,
                price,
                total
            FROM order_items
            WHERE order_id = $1
            ORDER BY created_at
            "#,
            order_id
        )
        .fetch_all(self.db)
        .await?;

        Ok(Some(OrderWithItems {
            id: order.id,
            order_number: order.order_number,
            status: order.status,
            subtotal: order.subtotal,
            tax: order.tax,
            discount: order.discount,
            total: order.total,
            currency: order.currency,
            payment_method: order.payment_method,
            billing_address: order.billing_address,
            created_at: order.created_at,
            items: items.into_iter().map(|i| crate::models::OrderItem {
                id: i.id,
                name: i.name,
                quantity: i.quantity,
                price: i.price,
                total: i.total,
            }).collect(),
        }))
    }
}
