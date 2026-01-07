//! Order Service
//! ICT 11+ Principal Engineer Grade - Runtime SQLx queries with FromRow
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{errors::AppError, models::{OrderItem, OrderSummary, OrderWithItems}};

/// Row struct for order query results
#[derive(FromRow)]
struct OrderRow {
    id: Uuid,
    user_id: Uuid,
    order_number: String,
    status: String,
    subtotal: f64,
    tax: f64,
    discount: f64,
    total: f64,
    currency: String,
    payment_method: Option<String>,
    billing_address: Option<serde_json::Value>,
    created_at: DateTime<Utc>,
}

/// Row struct for order item query results
#[derive(FromRow)]
struct OrderItemRow {
    id: Uuid,
    order_id: Uuid,
    product_id: Option<Uuid>,
    name: String,
    quantity: i32,
    price: f64,
    total: f64,
}

pub struct OrderService<'a> {
    db: &'a PgPool,
}

impl<'a> OrderService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_orders(&self, user_id: Uuid) -> Result<Vec<OrderSummary>, AppError> {
        let orders: Vec<OrderSummary> = sqlx::query_as(
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
        .await?;

        Ok(orders)
    }

    pub async fn get_user_order(&self, user_id: Uuid, order_id: Uuid) -> Result<Option<OrderWithItems>, AppError> {
        let order: Option<OrderRow> = sqlx::query_as(
            r#"
            SELECT 
                id,
                user_id,
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
        .await?;

        let Some(order) = order else {
            return Ok(None);
        };

        let items: Vec<OrderItemRow> = sqlx::query_as(
            r#"
            SELECT 
                id,
                order_id,
                product_id,
                name,
                quantity,
                price,
                total
            FROM order_items
            WHERE order_id = $1
            ORDER BY created_at
            "#
        )
        .bind(order_id)
        .fetch_all(self.db)
        .await?;

        Ok(Some(OrderWithItems {
            id: order.id,
            user_id: order.user_id,
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
            items: items.into_iter().map(|i| OrderItem {
                id: i.id,
                order_id: i.order_id,
                product_id: i.product_id,
                name: i.name,
                quantity: i.quantity,
                price: i.price,
                total: i.total,
            }).collect(),
        }))
    }
}
