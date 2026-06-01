//! Admin Orders — canonical mounting point for `/api/admin/orders/*`
//!
//! Apple ICT 11+ Principal Engineer Grade — 2026-04-26 (FIX)
//!
//! ═══════════════════════════════════════════════════════════════════════════
//! WHY THIS MODULE EXISTS
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! The frontend page at `routes/admin/orders/+page.svelte` calls:
//!   - GET  /api/admin/orders                  — paginated list + stats
//!   - GET  /api/admin/orders/:id              — single order detail
//!   - GET  /api/admin/orders/export?format=csv — CSV download
//!
//! Plus the `routes/admin/orders/[id]/+page.svelte`-style detail page is
//! served by additional management endpoints already implemented in
//! `routes/orders.rs::admin_*`:
//!   - POST /api/admin/orders/:id/status
//!   - POST /api/admin/orders/:id/refund
//!   - POST /api/admin/orders/:id/cancel
//!   - POST /api/admin/orders/:id/fulfill
//!   - POST /api/admin/orders/:id/resend-confirmation
//!
//! `orders.rs` already has the heavy-lifting admin handlers (admin_index,
//! admin_show, admin_update_status, admin_refund, admin_cancel,
//! admin_fulfill, admin_resend_confirmation) plus the response types
//! (AdminOrderResponse, AdminOrderStats). Per the FIX-2026-04-26 directive,
//! we REUSE those — this module is the **canonical mount point** that the
//! main `api_router()` nests at `/admin/orders`, plus it contributes:
//!
//!   1. CSV export endpoint that the page already calls (line 177).
//!   2. Stats-only endpoint for the admin dashboard widget.
//!
//! All handlers are AdminUser-gated and use parameterized SQL via sqlx.

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    middleware::admin::AdminUser,
    routes::orders::{
        admin_cancel, admin_fulfill, admin_index, admin_refund, admin_resend_confirmation,
        admin_show, admin_update_status,
    },
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════
// CSV EXPORT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub status: Option<String>,
    pub search: Option<String>,
    /// Reserved for future formats (xlsx, json). CSV is the only supported value today.
    pub format: Option<String>,
}

#[derive(sqlx::FromRow)]
struct ExportRow {
    id: i64,
    order_number: String,
    status: String,
    total_cents: i64,
    currency: String,
    user_email: Option<String>,
    user_name: Option<String>,
    payment_provider: Option<String>,
    item_count: Option<i64>,
    created_at: chrono::NaiveDateTime,
    completed_at: Option<chrono::NaiveDateTime>,
}

/// GET /api/admin/orders/export — stream filtered orders as CSV.
///
/// Mirrors the same status/search filters as `admin_index`. Returns a
/// downloadable `text/csv` payload with a `Content-Disposition: attachment`
/// header so the browser triggers a save dialog.
#[tracing::instrument(skip(state, _admin))]
pub async fn admin_export(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<ExportQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Build query with the same filter shape as admin_index.
    let rows: Vec<ExportRow> = match (&query.status, &query.search) {
        (Some(status), Some(search)) if !status.is_empty() && !search.is_empty() => {
            let pattern = format!("%{search}%");
            sqlx::query_as::<_, ExportRow>(
                r"SELECT o.id, o.order_number, o.status,
                          (o.total * 100)::BIGINT AS total_cents, o.currency,
                          u.email AS user_email, u.name AS user_name,
                          o.payment_provider, COUNT(oi.id) AS item_count,
                          o.created_at, o.completed_at
                   FROM orders o
                   LEFT JOIN users u ON o.user_id = u.id
                   LEFT JOIN order_items oi ON o.id = oi.order_id
                   WHERE o.status = $1
                     AND (o.order_number ILIKE $2 OR u.email ILIKE $2 OR u.name ILIKE $2)
                   GROUP BY o.id, u.email, u.name
                   ORDER BY o.created_at DESC
                   LIMIT 10000",
            )
            .bind(status)
            .bind(&pattern)
            .fetch_all(&state.db.pool)
            .await
        }
        (Some(status), _) if !status.is_empty() => {
            sqlx::query_as::<_, ExportRow>(
                r"SELECT o.id, o.order_number, o.status,
                          (o.total * 100)::BIGINT AS total_cents, o.currency,
                          u.email AS user_email, u.name AS user_name,
                          o.payment_provider, COUNT(oi.id) AS item_count,
                          o.created_at, o.completed_at
                   FROM orders o
                   LEFT JOIN users u ON o.user_id = u.id
                   LEFT JOIN order_items oi ON o.id = oi.order_id
                   WHERE o.status = $1
                   GROUP BY o.id, u.email, u.name
                   ORDER BY o.created_at DESC
                   LIMIT 10000",
            )
            .bind(status)
            .fetch_all(&state.db.pool)
            .await
        }
        (_, Some(search)) if !search.is_empty() => {
            let pattern = format!("%{search}%");
            sqlx::query_as::<_, ExportRow>(
                r"SELECT o.id, o.order_number, o.status,
                          (o.total * 100)::BIGINT AS total_cents, o.currency,
                          u.email AS user_email, u.name AS user_name,
                          o.payment_provider, COUNT(oi.id) AS item_count,
                          o.created_at, o.completed_at
                   FROM orders o
                   LEFT JOIN users u ON o.user_id = u.id
                   LEFT JOIN order_items oi ON o.id = oi.order_id
                   WHERE o.order_number ILIKE $1
                      OR u.email ILIKE $1
                      OR u.name ILIKE $1
                   GROUP BY o.id, u.email, u.name
                   ORDER BY o.created_at DESC
                   LIMIT 10000",
            )
            .bind(&pattern)
            .fetch_all(&state.db.pool)
            .await
        }
        _ => {
            sqlx::query_as::<_, ExportRow>(
                r"SELECT o.id, o.order_number, o.status,
                          (o.total * 100)::BIGINT AS total_cents, o.currency,
                          u.email AS user_email, u.name AS user_name,
                          o.payment_provider, COUNT(oi.id) AS item_count,
                          o.created_at, o.completed_at
                   FROM orders o
                   LEFT JOIN users u ON o.user_id = u.id
                   LEFT JOIN order_items oi ON o.id = oi.order_id
                   GROUP BY o.id, u.email, u.name
                   ORDER BY o.created_at DESC
                   LIMIT 10000",
            )
            .fetch_all(&state.db.pool)
            .await
        }
    }
    .map_err(|e| {
        tracing::error!(target: "admin_orders", error = %e, "CSV export query failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch orders for export"})),
        )
    })?;

    // Build CSV body. Fields are quoted via `csv_field` to escape any embedded
    // commas / quotes / newlines in user-supplied data (names, emails).
    let mut body = String::with_capacity(rows.len() * 160);
    body.push_str(
        "id,order_number,status,total,currency,user_email,user_name,payment_provider,item_count,created_at,completed_at\n",
    );
    for r in rows {
        // CSV is a spreadsheet artifact; format cents → dollars for display
        let total_dollars = r.total_cents as f64 / 100.0;
        body.push_str(&format!(
            "{},{},{},{:.2},{},{},{},{},{},{},{}\n",
            r.id,
            csv_field(&r.order_number),
            csv_field(&r.status),
            total_dollars,
            csv_field(&r.currency),
            csv_field(r.user_email.as_deref().unwrap_or("")),
            csv_field(r.user_name.as_deref().unwrap_or("")),
            csv_field(r.payment_provider.as_deref().unwrap_or("")),
            r.item_count.unwrap_or(0),
            r.created_at.and_utc().to_rfc3339(),
            r.completed_at
                .map(|d| d.and_utc().to_rfc3339())
                .unwrap_or_default(),
        ));
    }

    let filename = format!(
        "orders-export-{}.csv",
        chrono::Utc::now().format("%Y-%m-%d")
    );

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        body,
    ))
}

/// Quote a CSV field if it contains commas, quotes, or newlines.
/// Embedded `"` characters are doubled per RFC 4180.
fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STATS-ONLY ENDPOINT
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/orders/stats — aggregate stats only (no list).
///
/// Lightweight endpoint for dashboard widgets that need just the headline
/// numbers without paginating through orders.
#[tracing::instrument(skip(state, _admin))]
pub async fn admin_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
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
            COUNT(*) AS total_orders,
            COUNT(*) FILTER (WHERE status = 'completed') AS completed_orders,
            COUNT(*) FILTER (WHERE status = 'pending') AS pending_orders,
            COUNT(*) FILTER (WHERE status IN ('refunded', 'partial_refund')) AS refunded_orders,
            COALESCE(SUM((total * 100)::BIGINT) FILTER (WHERE status = 'completed'), 0)::BIGINT AS total_revenue_cents
           FROM orders",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin_orders", error = %e, "stats aggregate query failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to fetch order stats"})),
        )
    })?;

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

    Ok(Json(serde_json::json!({
        "success": true,
        "stats": {
            "total_orders": stats.total_orders,
            "completed_orders": stats.completed_orders,
            "pending_orders": stats.pending_orders,
            "refunded_orders": stats.refunded_orders,
            "total_revenue_cents": stats.total_revenue_cents,
            "revenue_this_month_cents": revenue_this_month_cents,
            "average_order_value_cents": avg_order_value_cents
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Build the canonical `/admin/orders` router.
///
/// Composes:
///   - `orders::admin_index`             → GET  /
///   - `admin_orders::admin_export`      → GET  /export
///   - `admin_orders::admin_stats`       → GET  /stats
///   - `orders::admin_show`              → GET  /:id
///   - `orders::admin_update_status`     → POST /:id/status
///   - `orders::admin_refund`            → POST /:id/refund
///   - `orders::admin_cancel`            → POST /:id/cancel
///   - `orders::admin_fulfill`           → POST /:id/fulfill
///   - `orders::admin_resend_confirmation` → POST /:id/resend-confirmation
///
/// NOTE: `/export` and `/stats` are registered **before** `/:id` so axum
/// matches them as literal paths rather than greedily binding them as `id`.
pub fn router() -> Router<AppState> {
    use axum::routing::post;

    Router::new()
        .route("/", get(admin_index))
        .route("/export", get(admin_export))
        .route("/stats", get(admin_stats))
        .route("/{id}", get(admin_show))
        .route("/{id}/status", post(admin_update_status))
        .route("/{id}/refund", post(admin_refund))
        .route("/{id}/cancel", post(admin_cancel))
        .route("/{id}/fulfill", post(admin_fulfill))
        .route("/{id}/resend-confirmation", post(admin_resend_confirmation))
}
