//! Admin list / show / stats handlers (mounted under `/admin/orders`).
//!
//! R23-B (2026-05-20): extracted from the original 1,382-LOC
//! `orders.rs` as a pure structural move. Every SQL statement, error
//! mapping, and money invariant is preserved byte-for-byte.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use super::dtos::{AdminOrderListQuery, AdminOrderResponse, AdminOrderStats, OrderRow};
use super::helpers::build_order_detail;
use crate::{middleware::admin::AdminUser, AppState};

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
        r"SELECT o.id, o.order_number, o.status, (o.total * 100)::BIGINT AS total_cents, o.currency,
                  u.email as user_email, u.name as user_name, o.payment_provider,
                  o.created_at, o.completed_at,
                  COUNT(oi.id)::INT as item_count
           FROM orders o
           LEFT JOIN users u ON o.user_id = u.id
           LEFT JOIN order_items oi ON o.id = oi.order_id
           WHERE 1=1",
    );

    let mut bind_count = 0;

    if let Some(ref status) = query.status {
        if !status.is_empty() {
            bind_count += 1;
            sql.push_str(&format!(" AND o.status = ${bind_count}"));
        }
    }

    if let Some(ref search) = query.search {
        if !search.is_empty() {
            bind_count += 1;
            sql.push_str(&format!(
                " AND (o.order_number ILIKE ${bind_count} OR u.email ILIKE ${bind_count} OR u.name ILIKE ${bind_count})"
            ));
        }
    }

    sql.push_str(" GROUP BY o.id, u.email, u.name ORDER BY o.created_at DESC");
    sql.push_str(&format!(" LIMIT {per_page} OFFSET {offset}"));

    #[derive(sqlx::FromRow)]
    struct AdminOrderRow {
        id: i64,
        order_number: String,
        status: String,
        total_cents: i64,
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
            let search_pattern = format!("%{search}%");
            sqlx::query_as::<_, AdminOrderRow>(sqlx::AssertSqlSafe(sql.as_str()))
                .bind(status)
                .bind(&search_pattern)
                .fetch_all(&state.db.pool)
                .await
        }
        (Some(status), _) if !status.is_empty() => {
            sqlx::query_as::<_, AdminOrderRow>(sqlx::AssertSqlSafe(sql.as_str()))
                .bind(status)
                .fetch_all(&state.db.pool)
                .await
        }
        (_, Some(search)) if !search.is_empty() => {
            let search_pattern = format!("%{search}%");
            sqlx::query_as::<_, AdminOrderRow>(sqlx::AssertSqlSafe(sql.as_str()))
                .bind(&search_pattern)
                .fetch_all(&state.db.pool)
                .await
        }
        _ => {
            sqlx::query_as::<_, AdminOrderRow>(
                r"SELECT o.id, o.order_number, o.status, (o.total * 100)::BIGINT AS total_cents, o.currency,
                          u.email as user_email, u.name as user_name, o.payment_provider,
                          o.created_at, o.completed_at,
                          COUNT(oi.id)::INT as item_count
                   FROM orders o
                   LEFT JOIN users u ON o.user_id = u.id
                   LEFT JOIN order_items oi ON o.id = oi.order_id
                   GROUP BY o.id, u.email, u.name
                   ORDER BY o.created_at DESC
                   LIMIT $1 OFFSET $2",
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

    // FIX-2026-04-26 (P1-2): the unfiltered `SELECT COUNT(*) FROM orders`
    // produced a paginator that let admins click into empty pages whenever
    // a `status` or `search` filter was active. Mirror the same WHERE
    // clauses used in the list query so the paginator math matches the
    // visible result set. Errors propagate via `?` instead of being
    // swallowed by `unwrap_or(0)` (per CLAUDE.md error-handling rule).
    let mut count_sql = String::from(
        r"SELECT COUNT(DISTINCT o.id)
           FROM orders o
           LEFT JOIN users u ON o.user_id = u.id
           WHERE 1=1",
    );
    let mut count_bind_count = 0;
    if let Some(ref status) = query.status {
        if !status.is_empty() {
            count_bind_count += 1;
            count_sql.push_str(&format!(" AND o.status = ${count_bind_count}"));
        }
    }
    if let Some(ref search) = query.search {
        if !search.is_empty() {
            count_bind_count += 1;
            count_sql.push_str(&format!(
                " AND (o.order_number ILIKE ${count_bind_count} OR u.email ILIKE ${count_bind_count} OR u.name ILIKE ${count_bind_count})"
            ));
        }
    }

    let total_count: i64 = match (&query.status, &query.search) {
        (Some(status), Some(search)) if !status.is_empty() && !search.is_empty() => {
            let search_pattern = format!("%{search}%");
            sqlx::query_scalar(sqlx::AssertSqlSafe(count_sql.as_str()))
                .bind(status)
                .bind(&search_pattern)
                .fetch_one(&state.db.pool)
                .await
        }
        (Some(status), _) if !status.is_empty() => {
            sqlx::query_scalar(sqlx::AssertSqlSafe(count_sql.as_str()))
                .bind(status)
                .fetch_one(&state.db.pool)
                .await
        }
        (_, Some(search)) if !search.is_empty() => {
            let search_pattern = format!("%{search}%");
            sqlx::query_scalar(sqlx::AssertSqlSafe(count_sql.as_str()))
                .bind(&search_pattern)
                .fetch_one(&state.db.pool)
                .await
        }
        _ => {
            sqlx::query_scalar("SELECT COUNT(*) FROM orders")
                .fetch_one(&state.db.pool)
                .await
        }
    }
    .map_err(|e| {
        tracing::error!("Failed to count admin orders: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to count orders"})),
        )
    })?;

    // Get stats
    let stats = get_admin_order_stats(&state)
        .await
        .unwrap_or(AdminOrderStats {
            total_orders: 0,
            completed_orders: 0,
            pending_orders: 0,
            refunded_orders: 0,
            total_revenue_cents: 0,
            revenue_this_month_cents: 0,
            average_order_value_cents: 0,
        });

    let order_responses: Vec<AdminOrderResponse> = orders
        .into_iter()
        .map(|o| AdminOrderResponse {
            id: o.id,
            order_number: o.order_number,
            status: o.status,
            total_cents: o.total_cents,
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

/// Get admin order stats helper (revenue figures in integer cents)
async fn get_admin_order_stats(state: &AppState) -> Result<AdminOrderStats, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct StatsRow {
        total_orders: i64,
        completed_orders: i64,
        pending_orders: i64,
        refunded_orders: i64,
        total_revenue_cents: i64,
    }

    let stats: StatsRow = sqlx::query_as(
        r"SELECT
            COUNT(*) as total_orders,
            COUNT(*) FILTER (WHERE status = 'completed') as completed_orders,
            COUNT(*) FILTER (WHERE status = 'pending') as pending_orders,
            COUNT(*) FILTER (WHERE status IN ('refunded', 'partial_refund')) as refunded_orders,
            COALESCE(SUM((total * 100)::BIGINT) FILTER (WHERE status = 'completed'), 0)::BIGINT as total_revenue_cents
           FROM orders",
    )
    .fetch_one(&state.db.pool)
    .await?;

    let revenue_this_month_cents: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
           FROM orders
           WHERE status = 'completed'
           AND created_at >= DATE_TRUNC('month', CURRENT_DATE)",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    let avg_order_value_cents: i64 = if stats.completed_orders > 0 {
        stats.total_revenue_cents / stats.completed_orders
    } else {
        0
    };

    Ok(AdminOrderStats {
        total_orders: stats.total_orders,
        completed_orders: stats.completed_orders,
        pending_orders: stats.pending_orders,
        refunded_orders: stats.refunded_orders,
        total_revenue_cents: stats.total_revenue_cents,
        revenue_this_month_cents,
        average_order_value_cents: avg_order_value_cents,
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
        r"SELECT id, order_number, status, (subtotal * 100)::BIGINT AS subtotal_cents, (discount * 100)::BIGINT AS discount_cents,
                  (tax * 100)::BIGINT AS tax_cents, (total * 100)::BIGINT AS total_cents, currency,
                  billing_name, billing_email, billing_address, payment_provider,
                  coupon_code, created_at, completed_at
           FROM orders
           WHERE id = $1",
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
