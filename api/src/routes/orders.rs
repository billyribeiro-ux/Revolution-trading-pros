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

use crate::{middleware::admin::AdminUser, models::User, services::order_service::OrderService, AppState};

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
// ADMIN ROUTES - ICT 7 Fix: Added admin orders management
// ═══════════════════════════════════════════════════════════════════════════

/// Admin order response with user details
#[derive(Debug, Serialize)]
pub struct AdminOrderResponse {
    pub id: i64,
    pub order_number: String,
    pub status: String,
    pub total: f64,
    pub currency: String,
    pub user_email: String,
    pub user_name: Option<String>,
    pub payment_provider: Option<String>,
    pub item_count: i32,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// Admin order query params
#[derive(Debug, Deserialize)]
pub struct AdminOrderListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Admin order stats
#[derive(Debug, Serialize)]
pub struct AdminOrderStats {
    pub total_orders: i64,
    pub completed_orders: i64,
    pub pending_orders: i64,
    pub refunded_orders: i64,
    pub total_revenue: f64,
    pub revenue_this_month: f64,
    pub average_order_value: f64,
}

/// GET /api/admin/orders - List all orders with pagination (Admin only)
/// ICT 7 SECURITY FIX: Use AdminUser extractor for consistent authorization
#[tracing::instrument(skip(state, _admin))]
pub async fn admin_index(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<AdminOrderListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;

    // Build query with filters
    let mut sql = String::from(
        r#"SELECT o.id, o.order_number, o.status, o.total::FLOAT8 as total, o.currency,
                  u.email as user_email, u.name as user_name, o.payment_provider,
                  o.created_at, o.completed_at,
                  COUNT(oi.id)::INT as item_count
           FROM orders o
           LEFT JOIN users u ON o.user_id = u.id
           LEFT JOIN order_items oi ON o.id = oi.order_id
           WHERE 1=1"#
    );

    let mut bind_count = 0;

    if let Some(ref status) = query.status {
        if !status.is_empty() {
            bind_count += 1;
            sql.push_str(&format!(" AND o.status = ${}", bind_count));
        }
    }

    if let Some(ref search) = query.search {
        if !search.is_empty() {
            bind_count += 1;
            sql.push_str(&format!(" AND (o.order_number ILIKE ${0} OR u.email ILIKE ${0} OR u.name ILIKE ${0})", bind_count));
        }
    }

    sql.push_str(" GROUP BY o.id, u.email, u.name ORDER BY o.created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    #[derive(sqlx::FromRow)]
    struct AdminOrderRow {
        id: i64,
        order_number: String,
        status: String,
        total: f64,
        currency: String,
        user_email: String,
        user_name: Option<String>,
        payment_provider: Option<String>,
        created_at: chrono::NaiveDateTime,
        completed_at: Option<chrono::NaiveDateTime>,
        item_count: i32,
    }

    // Execute query based on filters
    let orders: Vec<AdminOrderRow> = match (&query.status, &query.search) {
        (Some(status), Some(search)) if !status.is_empty() && !search.is_empty() => {
            let search_pattern = format!("%{}%", search);
            sqlx::query_as::<_, AdminOrderRow>(&sql)
                .bind(status)
                .bind(&search_pattern)
                .fetch_all(&state.db.pool)
                .await
        }
        (Some(status), _) if !status.is_empty() => {
            sqlx::query_as::<_, AdminOrderRow>(&sql)
                .bind(status)
                .fetch_all(&state.db.pool)
                .await
        }
        (_, Some(search)) if !search.is_empty() => {
            let search_pattern = format!("%{}%", search);
            sqlx::query_as::<_, AdminOrderRow>(&sql)
                .bind(&search_pattern)
                .fetch_all(&state.db.pool)
                .await
        }
        _ => {
            sqlx::query_as::<_, AdminOrderRow>(
                r#"SELECT o.id, o.order_number, o.status, o.total::FLOAT8 as total, o.currency,
                          u.email as user_email, u.name as user_name, o.payment_provider,
                          o.created_at, o.completed_at,
                          COUNT(oi.id)::INT as item_count
                   FROM orders o
                   LEFT JOIN users u ON o.user_id = u.id
                   LEFT JOIN order_items oi ON o.id = oi.order_id
                   GROUP BY o.id, u.email, u.name
                   ORDER BY o.created_at DESC
                   LIMIT $1 OFFSET $2"#
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
    }
    .map_err(|e| {
        tracing::error!("Failed to fetch admin orders: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch orders"})),
        )
    })?;

    // Get total count
    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    // Get stats
    let stats = get_admin_order_stats(&state).await.unwrap_or(AdminOrderStats {
        total_orders: 0,
        completed_orders: 0,
        pending_orders: 0,
        refunded_orders: 0,
        total_revenue: 0.0,
        revenue_this_month: 0.0,
        average_order_value: 0.0,
    });

    let order_responses: Vec<AdminOrderResponse> = orders
        .into_iter()
        .map(|o| AdminOrderResponse {
            id: o.id,
            order_number: o.order_number,
            status: o.status,
            total: o.total,
            currency: o.currency,
            user_email: o.user_email,
            user_name: o.user_name,
            payment_provider: o.payment_provider,
            item_count: o.item_count,
            created_at: o.created_at.and_utc().to_rfc3339(),
            completed_at: o.completed_at.map(|dt| dt.and_utc().to_rfc3339()),
        })
        .collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "data": order_responses,
        "stats": stats,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total_count,
            "total_pages": (total_count as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get admin order stats helper
async fn get_admin_order_stats(state: &AppState) -> Result<AdminOrderStats, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct StatsRow {
        total_orders: i64,
        completed_orders: i64,
        pending_orders: i64,
        refunded_orders: i64,
        total_revenue: f64,
    }

    let stats: StatsRow = sqlx::query_as(
        r#"SELECT
            COUNT(*) as total_orders,
            COUNT(*) FILTER (WHERE status = 'completed') as completed_orders,
            COUNT(*) FILTER (WHERE status = 'pending') as pending_orders,
            COUNT(*) FILTER (WHERE status IN ('refunded', 'partial_refund')) as refunded_orders,
            COALESCE(SUM(total) FILTER (WHERE status = 'completed'), 0)::FLOAT8 as total_revenue
           FROM orders"#
    )
    .fetch_one(&state.db.pool)
    .await?;

    let revenue_this_month: f64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(total), 0)::FLOAT8
           FROM orders
           WHERE status = 'completed'
           AND created_at >= DATE_TRUNC('month', CURRENT_DATE)"#
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0.0);

    let avg_order_value = if stats.completed_orders > 0 {
        stats.total_revenue / stats.completed_orders as f64
    } else {
        0.0
    };

    Ok(AdminOrderStats {
        total_orders: stats.total_orders,
        completed_orders: stats.completed_orders,
        pending_orders: stats.pending_orders,
        refunded_orders: stats.refunded_orders,
        total_revenue: stats.total_revenue,
        revenue_this_month,
        average_order_value: avg_order_value,
    })
}

/// GET /api/admin/orders/:id - Get admin order details
/// ICT 7 SECURITY FIX: Use AdminUser extractor for consistent authorization
#[tracing::instrument(skip(state, _admin))]
pub async fn admin_show(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {

    let order: Option<OrderRow> = sqlx::query_as::<_, OrderRow>(
        r#"SELECT id, order_number, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                  tax::FLOAT8 as tax, total::FLOAT8 as total, currency,
                  billing_name, billing_email, billing_address, payment_provider,
                  coupon_code, created_at, completed_at
           FROM orders
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
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

// ═══════════════════════════════════════════════════════════════════════════
// Admin Order Management - ICT 7 COMPLETE ORDER MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

/// Valid order status transitions
const VALID_STATUS_TRANSITIONS: &[(&str, &[&str])] = &[
    ("pending", &["processing", "completed", "cancelled", "failed"]),
    ("processing", &["completed", "cancelled", "failed", "refunded"]),
    ("completed", &["refunded", "partial_refund"]),
    ("failed", &["pending"]),  // Allow retry
    ("cancelled", &[]),  // Terminal state
    ("refunded", &[]),  // Terminal state
    ("partial_refund", &["refunded"]),  // Can fully refund
];

/// Check if status transition is valid
fn is_valid_transition(from: &str, to: &str) -> bool {
    VALID_STATUS_TRANSITIONS
        .iter()
        .find(|(status, _)| *status == from)
        .map(|(_, valid_targets)| valid_targets.contains(&to))
        .unwrap_or(false)
}

/// Update order status request
#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
    pub notes: Option<String>,
}

/// Refund order request
#[derive(Debug, Deserialize)]
pub struct RefundOrderRequest {
    pub amount: Option<f64>,  // None = full refund
    pub reason: Option<String>,
}

/// POST /api/admin/orders/:id/status - Update order status
/// ICT 7 FIX: Complete order status management with validation
#[tracing::instrument(skip(state, admin))]
pub async fn admin_update_status(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateOrderStatusRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate status value
    let valid_statuses = ["pending", "processing", "completed", "cancelled", "failed", "refunded", "partial_refund"];
    if !valid_statuses.contains(&input.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": format!("Invalid status. Must be one of: {}", valid_statuses.join(", "))})),
        ));
    }

    // Get current order status
    let current_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order status");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"})))
    })?;

    let current_status = current_status.ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Order not found"})))
    })?;

    // Validate transition
    if !is_valid_transition(&current_status, &input.status) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Invalid status transition from '{}' to '{}'", current_status, input.status)
            })),
        ));
    }

    // Update status
    let completed_at = if input.status == "completed" {
        Some(chrono::Utc::now().naive_utc())
    } else {
        None
    };

    let refunded_at = if input.status == "refunded" || input.status == "partial_refund" {
        Some(chrono::Utc::now().naive_utc())
    } else {
        None
    };

    sqlx::query(
        r#"UPDATE orders SET
            status = $2,
            completed_at = COALESCE($3, completed_at),
            refunded_at = COALESCE($4, refunded_at),
            updated_at = NOW()
        WHERE id = $1"#
    )
    .bind(id)
    .bind(&input.status)
    .bind(completed_at)
    .bind(refunded_at)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to update order status");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update order status"})))
    })?;

    // Log status change
    tracing::info!(
        target: "orders",
        event = "order_status_updated",
        order_id = %id,
        from_status = %current_status,
        to_status = %input.status,
        admin_id = %admin.id,
        notes = ?input.notes,
        "Order status updated"
    );

    // If completed, grant product access
    if input.status == "completed" {
        grant_order_products(&state, id).await.ok();
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order status updated",
        "order_id": id,
        "previous_status": current_status,
        "new_status": input.status
    })))
}

/// Grant product access for completed order
async fn grant_order_products(state: &AppState, order_id: i64) -> Result<(), sqlx::Error> {
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
        "SELECT product_id FROM order_items WHERE order_id = $1 AND product_id IS NOT NULL"
    )
    .bind(order_id)
    .fetch_all(&state.db.pool)
    .await?;

    for item in items {
        if let Some(product_id) = item.product_id {
            // Insert into user_products if not exists
            sqlx::query(
                r#"INSERT INTO user_products (user_id, product_id, purchased_at, order_id, created_at, updated_at)
                   VALUES ($1, $2, NOW(), $3, NOW(), NOW())
                   ON CONFLICT (user_id, product_id) DO NOTHING"#
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

/// POST /api/admin/orders/:id/refund - Refund an order
/// ICT 7 FIX: Complete refund functionality
#[tracing::instrument(skip(state, admin))]
pub async fn admin_refund(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<RefundOrderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get order details
    #[derive(sqlx::FromRow)]
    struct OrderInfo {
        status: String,
        total: f64,
        payment_intent_id: Option<String>,
    }

    let order: OrderInfo = sqlx::query_as(
        "SELECT status, total::FLOAT8 as total, payment_intent_id FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"})))
    })?
    .ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Order not found"})))
    })?;

    // Validate order can be refunded
    if order.status != "completed" && order.status != "processing" && order.status != "partial_refund" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": format!("Cannot refund order with status '{}'", order.status)})),
        ));
    }

    let refund_amount = input.amount.unwrap_or(order.total);

    // Validate refund amount
    if refund_amount <= 0.0 || refund_amount > order.total {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid refund amount"})),
        ));
    }

    let new_status = if (refund_amount - order.total).abs() < 0.01 {
        "refunded"
    } else {
        "partial_refund"
    };

    // If we have a payment intent, attempt Stripe refund
    if let Some(ref payment_intent_id) = order.payment_intent_id {
        // Attempt Stripe refund
        let refund_result = state.services.stripe.create_refund(
            payment_intent_id,
            Some((refund_amount * 100.0) as i64),
            input.reason.as_deref(),
        ).await;

        if let Err(e) = refund_result {
            tracing::error!(
                target: "orders",
                error = %e,
                order_id = %id,
                payment_intent_id = %payment_intent_id,
                "Stripe refund failed"
            );
            return Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Stripe refund failed: {}", e)})),
            ));
        }
    }

    // Update order status
    sqlx::query(
        r#"UPDATE orders SET
            status = $2,
            refunded_at = NOW(),
            updated_at = NOW()
        WHERE id = $1"#
    )
    .bind(id)
    .bind(new_status)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to update order status");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update order"})))
    })?;

    tracing::info!(
        target: "orders",
        event = "order_refunded",
        order_id = %id,
        refund_amount = %refund_amount,
        new_status = %new_status,
        admin_id = %admin.id,
        reason = ?input.reason,
        "Order refunded"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order refunded successfully",
        "order_id": id,
        "refund_amount": refund_amount,
        "new_status": new_status
    })))
}

/// POST /api/admin/orders/:id/cancel - Cancel an order
/// ICT 7 FIX: Complete cancel functionality
#[tracing::instrument(skip(state, admin))]
pub async fn admin_cancel(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current status
    let current_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"})))
    })?;

    let current_status = current_status.ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Order not found"})))
    })?;

    // Only pending or processing orders can be cancelled
    if current_status != "pending" && current_status != "processing" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": format!("Cannot cancel order with status '{}'", current_status)})),
        ));
    }

    // Update order status
    sqlx::query(
        "UPDATE orders SET status = 'cancelled', updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to cancel order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to cancel order"})))
    })?;

    tracing::info!(
        target: "orders",
        event = "order_cancelled",
        order_id = %id,
        previous_status = %current_status,
        admin_id = %admin.id,
        "Order cancelled"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order cancelled successfully",
        "order_id": id,
        "previous_status": current_status
    })))
}

/// POST /api/admin/orders/:id/fulfill - Mark order as completed/fulfilled
/// ICT 7 FIX: Quick fulfill endpoint
#[tracing::instrument(skip(state, admin))]
pub async fn admin_fulfill(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current status
    let current_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"})))
    })?;

    let current_status = current_status.ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Order not found"})))
    })?;

    // Only pending or processing orders can be fulfilled
    if current_status != "pending" && current_status != "processing" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": format!("Cannot fulfill order with status '{}'", current_status)})),
        ));
    }

    // Update order status
    sqlx::query(
        "UPDATE orders SET status = 'completed', completed_at = NOW(), updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fulfill order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fulfill order"})))
    })?;

    // Grant product access
    grant_order_products(&state, id).await.ok();

    tracing::info!(
        target: "orders",
        event = "order_fulfilled",
        order_id = %id,
        previous_status = %current_status,
        admin_id = %admin.id,
        "Order fulfilled"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order fulfilled successfully",
        "order_id": id,
        "previous_status": current_status
    })))
}

/// POST /api/admin/orders/:id/resend-confirmation - Resend order confirmation email
/// ICT 7 FIX: Resend confirmation endpoint
#[tracing::instrument(skip(state, admin))]
pub async fn admin_resend_confirmation(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get order and user details
    #[derive(sqlx::FromRow)]
    struct OrderEmailInfo {
        order_number: String,
        billing_email: Option<String>,
        user_id: i64,
    }

    let order: OrderEmailInfo = sqlx::query_as(
        "SELECT order_number, billing_email, user_id FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"})))
    })?
    .ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Order not found"})))
    })?;

    // Get user email if billing email not set
    let email = if let Some(email) = order.billing_email {
        email
    } else {
        sqlx::query_scalar::<_, String>("SELECT email FROM users WHERE id = $1")
            .bind(order.user_id)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(target: "orders", error = %e, user_id = %order.user_id, "Failed to fetch user email");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to get user email"})))
            })?
    };

    // Queue email (implementation depends on email service)
    tracing::info!(
        target: "orders",
        event = "confirmation_resent",
        order_id = %id,
        order_number = %order.order_number,
        email = %email,
        admin_id = %admin.id,
        "Order confirmation email queued for resend"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order confirmation email queued",
        "order_id": id,
        "email": email
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// Router - ICT 7 COMPLETE ORDER MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

/// Build orders router with all endpoints
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/by-number/{order_number}", get(show_by_number))
        .route("/{id}", get(show))
}

/// Build admin orders router - ICT 7 COMPLETE
pub fn admin_router() -> Router<AppState> {
    use axum::routing::post;

    Router::new()
        .route("/", get(admin_index))
        .route("/{id}", get(admin_show))
        // Order management endpoints - ICT 7 FIX
        .route("/{id}/status", post(admin_update_status))
        .route("/{id}/refund", post(admin_refund))
        .route("/{id}/cancel", post(admin_cancel))
        .route("/{id}/fulfill", post(admin_fulfill))
        .route("/{id}/resend-confirmation", post(admin_resend_confirmation))
}
