//! Order Service
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::order_service_types::{OrderItemData, OrderSummary, OrderWithItems},
    utils::errors::ApiError,
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
            r"
            SELECT
                o.id,
                o.order_number,
                o.status,
                (o.total * 100)::BIGINT AS total_cents,
                o.currency,
                o.created_at,
                COUNT(oi.id)::int as item_count
            FROM orders o
            LEFT JOIN order_items oi ON o.id = oi.order_id
            WHERE o.user_id = $1
            GROUP BY o.id, o.order_number, o.status, o.total, o.currency, o.created_at
            ORDER BY o.created_at DESC
            ",
        )
        .bind(user_id)
        .fetch_all(self.db)
        .await
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
        })?;

        Ok(orders)
    }

    pub async fn get_user_order(
        &self,
        user_id: Uuid,
        order_id: Uuid,
    ) -> Result<Option<OrderWithItems>, ApiError> {
        // Money is integer cents per arch standard §1.2; SQL converts NUMERIC at the boundary.
        #[derive(sqlx::FromRow)]
        struct OrderQueryRow {
            id: Uuid,
            order_number: String,
            status: String,
            subtotal_cents: i64,
            tax_cents: i64,
            discount_cents: i64,
            total_cents: i64,
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
            unit_price_cents: i64,
            total_cents: i64,
        }

        let order = sqlx::query_as::<_, OrderQueryRow>(
            r"
            SELECT
                id,
                order_number,
                status,
                (subtotal * 100)::BIGINT AS subtotal_cents,
                (tax * 100)::BIGINT      AS tax_cents,
                (discount * 100)::BIGINT AS discount_cents,
                (total * 100)::BIGINT    AS total_cents,
                currency,
                payment_method,
                billing_address,
                created_at
            FROM orders
            WHERE id = $1 AND user_id = $2
            ",
        )
        .bind(order_id)
        .bind(user_id)
        .fetch_optional(self.db)
        .await
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
        })?;

        let Some(order) = order else {
            return Ok(None);
        };

        let items = sqlx::query_as::<_, OrderItemQueryRow>(
            r"
            SELECT
                id,
                name,
                quantity,
                (unit_price * 100)::BIGINT AS unit_price_cents,
                (total * 100)::BIGINT      AS total_cents
            FROM order_items
            WHERE order_id = $1
            ORDER BY created_at
            ",
        )
        .bind(order_id)
        .fetch_all(self.db)
        .await
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
        })?;

        Ok(Some(OrderWithItems {
            id: order.id,
            order_number: order.order_number,
            status: order.status,
            subtotal_cents: order.subtotal_cents,
            tax_cents: order.tax_cents,
            discount_cents: order.discount_cents,
            total_cents: order.total_cents,
            currency: order.currency,
            payment_method: order.payment_method,
            billing_address: order.billing_address,
            created_at: order.created_at,
            items: items
                .into_iter()
                .map(|i| OrderItemData {
                    id: i.id,
                    name: i.name,
                    quantity: i.quantity,
                    unit_price_cents: i.unit_price_cents,
                    total_cents: i.total_cents,
                })
                .collect(),
        }))
    }
}
