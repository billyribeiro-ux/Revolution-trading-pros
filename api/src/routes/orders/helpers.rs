//! Shared helpers used by both the user-facing and admin order handlers.
//!
//! R23-B (2026-05-20): extracted from the original 1,382-LOC
//! `orders.rs` as a pure structural move. Every SQL statement,
//! product-grant transition, and money invariant (`i64` cents
//! end-to-end) is preserved byte-for-byte.

use axum::{http::StatusCode, Json};

use super::dtos::{
    OrderDetailResponse, OrderItemDetailResponse, OrderItemRow, OrderRow, ProductMeta,
};
use crate::AppState;

/// Valid order status transitions
pub(super) const VALID_STATUS_TRANSITIONS: &[(&str, &[&str])] = &[
    (
        "pending",
        &["processing", "completed", "cancelled", "failed"],
    ),
    (
        "processing",
        &["completed", "cancelled", "failed", "refunded"],
    ),
    ("completed", &["refunded", "partial_refund"]),
    ("failed", &["pending"]),          // Allow retry
    ("cancelled", &[]),                // Terminal state
    ("refunded", &[]),                 // Terminal state
    ("partial_refund", &["refunded"]), // Can fully refund
];

/// Check if status transition is valid
pub(super) fn is_valid_transition(from: &str, to: &str) -> bool {
    VALID_STATUS_TRANSITIONS
        .iter()
        .find(|(status, _)| *status == from)
        .map(|(_, valid_targets)| valid_targets.contains(&to))
        .unwrap_or(false)
}

/// Build complete order detail response with items and product metadata
pub(super) async fn build_order_detail(
    state: &AppState,
    order: OrderRow,
) -> Result<OrderDetailResponse, (StatusCode, Json<serde_json::Value>)> {
    // Fetch order items
    let items: Vec<OrderItemRow> = sqlx::query_as::<_, OrderItemRow>(
        r"SELECT id, product_id, plan_id, name, quantity, (unit_price * 100)::BIGINT AS unit_price_cents, (total * 100)::BIGINT AS total_cents
           FROM order_items
           WHERE order_id = $1
           ORDER BY id",
    )
    .bind(order.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch order items: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch order items"})),
        )
    })?;

    // Fetch product metadata for items with product_id
    let product_ids: Vec<i64> = items.iter().filter_map(|i| i.product_id).collect();
    let product_meta: std::collections::HashMap<i64, ProductMeta> = if !product_ids.is_empty() {
        sqlx::query_as::<_, (i64, Option<String>, Option<String>, Option<String>)>(
            "SELECT id, type, slug, thumbnail FROM products WHERE id = ANY($1)",
        )
        .bind(&product_ids)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch product metadata for order: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to fetch product metadata"})),
            )
        })?
        .into_iter()
        .map(|(id, product_type, slug, thumbnail)| {
            (
                id,
                ProductMeta {
                    product_type,
                    slug,
                    thumbnail,
                },
            )
        })
        .collect()
    } else {
        std::collections::HashMap::new()
    };

    // Build item responses
    let item_details: Vec<OrderItemDetailResponse> = items
        .into_iter()
        .map(|item| {
            let meta = item.product_id.and_then(|pid| product_meta.get(&pid));
            OrderItemDetailResponse {
                id: item.id,
                product_id: item.product_id,
                plan_id: item.plan_id,
                name: item.name,
                quantity: item.quantity,
                unit_price_cents: item.unit_price_cents,
                total_cents: item.total_cents,
                product_type: meta.and_then(|m| m.product_type.clone()),
                product_slug: meta.and_then(|m| m.slug.clone()),
                thumbnail: meta.and_then(|m| m.thumbnail.clone()),
            }
        })
        .collect();

    Ok(OrderDetailResponse {
        id: order.id,
        order_number: order.order_number,
        status: order.status,
        subtotal_cents: order.subtotal_cents,
        discount_cents: order.discount_cents,
        tax_cents: order.tax_cents,
        total_cents: order.total_cents,
        currency: order.currency,
        billing_name: order.billing_name,
        billing_email: order.billing_email,
        billing_address: order.billing_address,
        payment_provider: order.payment_provider,
        coupon_code: order.coupon_code,
        items: item_details,
        created_at: order.created_at.and_utc().to_rfc3339(),
        completed_at: order.completed_at.map(|dt| dt.and_utc().to_rfc3339()),
    })
}

/// Grant product access for completed order
pub(super) async fn grant_order_products(
    state: &AppState,
    order_id: i64,
) -> Result<(), sqlx::Error> {
    // Get user_id and order items
    let user_id: i64 = sqlx::query_scalar("SELECT user_id FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_one(&state.db.pool)
        .await?;

    #[derive(sqlx::FromRow)]
    struct OrderProductItem {
        product_id: Option<i64>,
    }

    let items: Vec<OrderProductItem> = sqlx::query_as(
        "SELECT product_id FROM order_items WHERE order_id = $1 AND product_id IS NOT NULL",
    )
    .bind(order_id)
    .fetch_all(&state.db.pool)
    .await?;

    for item in items {
        if let Some(product_id) = item.product_id {
            // Insert into user_products if not exists
            sqlx::query(
                r"INSERT INTO user_products (user_id, product_id, purchased_at, order_id, created_at, updated_at)
                   VALUES ($1, $2, NOW(), $3, NOW(), NOW())
                   ON CONFLICT (user_id, product_id) DO NOTHING"
            )
            .bind(user_id)
            .bind(product_id)
            .bind(order_id)
            .execute(&state.db.pool)
            .await?;
        }
    }

    tracing::info!(
        target: "orders",
        event = "products_granted",
        order_id = %order_id,
        user_id = %user_id,
        "Product access granted for completed order"
    );

    Ok(())
}
