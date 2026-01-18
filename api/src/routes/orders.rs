//! Orders Handlers
//!
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//! Complete user order management with:
//! - Order listing with pagination
//! - Order details by ID or order number
//! - Order item details with product metadata
//! - Support for thank-you page order fetching

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::User, services::order_service::OrderService, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// Response Types - ICT 11+ Grade
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub number: String,
    pub date: String,
    pub status: String,
    pub total: String,
    pub currency: String,
    pub item_count: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResponse {
    pub id: i64,
    pub order_number: String,
    pub status: String,
    pub subtotal: f64,
    pub discount: f64,
    pub tax: f64,
    pub total: f64,
    pub currency: String,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub payment_provider: Option<String>,
    pub coupon_code: Option<String>,
    pub items: Vec<OrderItemDetailResponse>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemDetailResponse {
    pub id: i64,
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total: f64,
    pub product_type: Option<String>,
    pub product_slug: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Database Row Types
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, sqlx::FromRow)]
struct OrderRow {
    id: i64,
    order_number: String,
    status: String,
    subtotal: f64,
    discount: f64,
    tax: f64,
    total: f64,
    currency: String,
    billing_name: Option<String>,
    billing_email: Option<String>,
    billing_address: Option<serde_json::Value>,
    payment_provider: Option<String>,
    coupon_code: Option<String>,
    created_at: chrono::NaiveDateTime,
    completed_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, sqlx::FromRow)]
struct OrderItemRow {
    id: i64,
    product_id: Option<i64>,
    plan_id: Option<i64>,
    name: String,
    quantity: i32,
    unit_price: f64,
    total: f64,
}

#[derive(Debug, sqlx::FromRow)]
struct ProductMeta {
    product_type: Option<String>,
    slug: Option<String>,
    thumbnail: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Route Handlers - ICT 11+ Grade
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/my/orders - List user's orders with pagination
#[tracing::instrument(skip(state))]
pub async fn index(
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
            r#"SELECT id, order_number, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                      tax::FLOAT8 as tax, total::FLOAT8 as total, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE user_id = $1 AND status = $2
               ORDER BY created_at DESC
               LIMIT $3 OFFSET $4"#,
        )
        .bind(user.id)
        .bind(status)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    } else {
        sqlx::query_as::<_, OrderRow>(
            r#"SELECT id, order_number, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                      tax::FLOAT8 as tax, total::FLOAT8 as total, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE user_id = $1
               ORDER BY created_at DESC
               LIMIT $2 OFFSET $3"#,
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
        .unwrap_or_default()
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
                total: format!("{:.2}", o.total),
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
pub async fn show(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Try to parse as i64 ID first, then as UUID
    let order: Option<OrderRow> = if let Ok(order_id) = id.parse::<i64>() {
        sqlx::query_as::<_, OrderRow>(
            r#"SELECT id, order_number, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                      tax::FLOAT8 as tax, total::FLOAT8 as total, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE id = $1 AND user_id = $2"#,
        )
        .bind(order_id)
        .bind(user.id)
        .fetch_optional(&state.db.pool)
        .await
    } else if let Ok(uuid) = Uuid::parse_str(&id) {
        // Legacy UUID support
        sqlx::query_as::<_, OrderRow>(
            r#"SELECT id, order_number, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                      tax::FLOAT8 as tax, total::FLOAT8 as total, currency,
                      billing_name, billing_email, billing_address, payment_provider,
                      coupon_code, created_at, completed_at
               FROM orders
               WHERE uuid = $1 AND user_id = $2"#,
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
pub async fn show_by_number(
    State(state): State<AppState>,
    user: User,
    Path(order_number): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let order: Option<OrderRow> = sqlx::query_as::<_, OrderRow>(
        r#"SELECT id, order_number, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                  tax::FLOAT8 as tax, total::FLOAT8 as total, currency,
                  billing_name, billing_email, billing_address, payment_provider,
                  coupon_code, created_at, completed_at
           FROM orders
           WHERE order_number = $1 AND user_id = $2"#,
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

// ═══════════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════════

/// Build complete order detail response with items and product metadata
async fn build_order_detail(
    state: &AppState,
    order: OrderRow,
) -> Result<OrderDetailResponse, (StatusCode, Json<serde_json::Value>)> {
    // Fetch order items
    let items: Vec<OrderItemRow> = sqlx::query_as::<_, OrderItemRow>(
        r#"SELECT id, product_id, plan_id, name, quantity, unit_price::FLOAT8 as unit_price, total::FLOAT8 as total
           FROM order_items
           WHERE order_id = $1
           ORDER BY id"#,
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
        .unwrap_or_default()
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
                unit_price: item.unit_price,
                total: item.total,
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
        subtotal: order.subtotal,
        discount: order.discount,
        tax: order.tax,
        total: order.total,
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

// ═══════════════════════════════════════════════════════════════════════════
// Router
// ═══════════════════════════════════════════════════════════════════════════

/// Build orders router with all endpoints
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/by-number/{order_number}", get(show_by_number))
        .route("/{id}", get(show))
}
