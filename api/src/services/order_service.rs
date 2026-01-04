//! Order Service
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    utils::errors::ApiError, 
    models::order_service_types::{OrderSummary, OrderWithItems, OrderItemData}
};

pub struct OrderService<'a> {
    db: &'a PgPool,
}

impl<'a> OrderService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_orders(&self, user_id: Uuid) -> Result<Vec<OrderSummary>, ApiError> {
        let orders = sqlx::query_as::<_, OrderSummary>(
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
            "#
        )
        .bind(user_id)
        .fetch_all(self.db)
        .await
        .map_err(|e| ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;

        Ok(orders)
    }

    pub async fn get_user_order(&self, user_id: Uuid, order_id: Uuid) -> Result<Option<OrderWithItems>, ApiError> {
        #[derive(sqlx::FromRow)]
        struct OrderQueryRow {
            id: Uuid,
            order_number: String,
            status: String,
            subtotal: f64,
            tax: f64,
            discount: f64,
            total: f64,
            currency: String,
            payment_method: Option<String>,
            billing_address: Option<serde_json::Value>,
            created_at: chrono::DateTime<chrono::Utc>,
        }

        #[derive(sqlx::FromRow)]
        struct OrderItemQueryRow {
            id: Uuid,
            name: String,
            quantity: i32,
            unit_price: f64,
            total: f64,
        }

        let order = sqlx::query_as::<_, OrderQueryRow>(
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
            "#
        )
        .bind(order_id)
        .bind(user_id)
        .fetch_optional(self.db)
        .await
        .map_err(|e| ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;

        let Some(order) = order else {
            return Ok(None);
        };

        let items = sqlx::query_as::<_, OrderItemQueryRow>(
            r#"
            SELECT 
                id,
                name,
                quantity,
                unit_price,
                total
            FROM order_items
            WHERE order_id = $1
            ORDER BY created_at
            "#
        )
        .bind(order_id)
        .fetch_all(self.db)
        .await
        .map_err(|e| ApiError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;

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
            items: items.into_iter().map(|i| OrderItemData {
                id: i.id,
                name: i.name,
                quantity: i.quantity,
                price: i.unit_price,
                total: i.total,
            }).collect(),
        }))
    }
}
