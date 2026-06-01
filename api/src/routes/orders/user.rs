//! User-facing order endpoints (mounted under `/my/orders`).
//!
//! R23-B (2026-05-20): extracted from the original 1,382-LOC
//! `orders.rs` as a pure structural move. Every SQL statement, error
//! mapping, and money invariant is preserved byte-for-byte.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use super::dtos::{OrderListQuery, OrderResponse, OrderRow};
use super::helpers::build_order_detail;
use crate::{models::User, AppState};

/// GET /api/my/orders - List user's orders with pagination
#[tracing::instrument(skip(state))]
pub(super) async fn index(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<OrderListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build query with optional status filter
    let orders: Vec<OrderRow> = if let Some(ref status) = query.status {
        sqlx::query_as::<_, OrderRow>(
            r"SELECT id, order_number, status, (subtotal * 100)::BIGINT AS subtotal_cents, (discount * 100)::BIGINT AS discount_cents,
                      (tax * 100)::BIGINT AS tax_cents, (total * 100)::BIGINT AS total_cents, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE user_id = $1 AND status = $2
               ORDER BY created_at DESC
               LIMIT $3 OFFSET $4",
        )
        .bind(user.id)
        .bind(status)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    } else {
        sqlx::query_as::<_, OrderRow>(
            r"SELECT id, order_number, status, (subtotal * 100)::BIGINT AS subtotal_cents, (discount * 100)::BIGINT AS discount_cents,
                      (tax * 100)::BIGINT AS tax_cents, (total * 100)::BIGINT AS total_cents, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE user_id = $1
               ORDER BY created_at DESC
               LIMIT $2 OFFSET $3",
        )
        .bind(user.id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Failed to fetch orders: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch orders"})),
        )
    })?;

    // Get total count
    let total_count: i64 = if let Some(ref status) = query.status {
        sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE user_id = $1 AND status = $2")
            .bind(user.id)
            .bind(status)
            .fetch_one(&state.db.pool)
            .await
    } else {
        sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE user_id = $1")
            .bind(user.id)
            .fetch_one(&state.db.pool)
            .await
    }
    .unwrap_or(0);

    // Get item counts for each order
    let order_ids: Vec<i64> = orders.iter().map(|o| o.id).collect();
    let item_counts: std::collections::HashMap<i64, i64> = if !order_ids.is_empty() {
        sqlx::query_as::<_, (i64, i64)>(
            "SELECT order_id, COUNT(*) as count FROM order_items WHERE order_id = ANY($1) GROUP BY order_id"
        )
        .bind(&order_ids)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch order item counts: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to fetch order item counts"})),
            )
        })?
        .into_iter()
        .collect()
    } else {
        std::collections::HashMap::new()
    };

    let order_responses: Vec<OrderResponse> = orders
        .into_iter()
        .map(|o| {
            let item_count = *item_counts.get(&o.id).unwrap_or(&0) as i32;
            OrderResponse {
                id: o.id.to_string(),
                number: o.order_number,
                date: o.created_at.and_utc().to_rfc3339(),
                status: o.status,
                total: format!("{:.2}", o.total_cents as f64 / 100.0), // display only — OrderResponse.total is a String
                currency: o.currency,
                item_count,
            }
        })
        .collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "data": order_responses,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total_count,
            "total_pages": (total_count as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /api/my/orders/:id - Get order details by ID
#[tracing::instrument(skip(state))]
pub(super) async fn show(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Try to parse as i64 ID first, then as UUID
    let order: Option<OrderRow> = if let Ok(order_id) = id.parse::<i64>() {
        sqlx::query_as::<_, OrderRow>(
            r"SELECT id, order_number, status, (subtotal * 100)::BIGINT AS subtotal_cents, (discount * 100)::BIGINT AS discount_cents,
                      (tax * 100)::BIGINT AS tax_cents, (total * 100)::BIGINT AS total_cents, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE id = $1 AND user_id = $2",
        )
        .bind(order_id)
        .bind(user.id)
        .fetch_optional(&state.db.pool)
        .await
    } else if let Ok(uuid) = Uuid::parse_str(&id) {
        // Legacy UUID support
        sqlx::query_as::<_, OrderRow>(
            r"SELECT id, order_number, status, (subtotal * 100)::BIGINT AS subtotal_cents, (discount * 100)::BIGINT AS discount_cents,
                      (tax * 100)::BIGINT AS tax_cents, (total * 100)::BIGINT AS total_cents, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE uuid = $1 AND user_id = $2",
        )
        .bind(uuid)
        .bind(user.id)
        .fetch_optional(&state.db.pool)
        .await
    } else {
        Ok(None)
    }
    .map_err(|e| {
        tracing::error!("Failed to fetch order: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch order"})),
        )
    })?;

    let order = order.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    let detail = build_order_detail(&state, order).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": detail
    })))
}

/// GET /api/my/orders/by-number/:order_number - Get order by order number
/// Used by thank-you page to fetch order details after checkout
#[tracing::instrument(skip(state))]
pub(super) async fn show_by_number(
    State(state): State<AppState>,
    user: User,
    Path(order_number): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let order: Option<OrderRow> = sqlx::query_as::<_, OrderRow>(
        r"SELECT id, order_number, status, (subtotal * 100)::BIGINT AS subtotal_cents, (discount * 100)::BIGINT AS discount_cents,
                  (tax * 100)::BIGINT AS tax_cents, (total * 100)::BIGINT AS total_cents, currency,
                  billing_name, billing_email, billing_address, payment_provider,
                  coupon_code, created_at, completed_at
           FROM orders
           WHERE order_number = $1 AND user_id = $2",
    )
    .bind(&order_number)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch order by number: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch order"})),
        )
    })?;

    let order = order.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    let detail = build_order_detail(&state, order).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": detail
    })))
}
