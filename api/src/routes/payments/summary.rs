//! Aggregate payment metrics for the admin dashboard.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Every cents-cast,
//! billing-cycle MRR normalization arm, and SQL aggregation
//! preserved byte-for-byte.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use super::dtos::PaymentSummary;
use crate::{middleware::admin::AdminUser, AppState};

/// Get aggregate payment metrics for the admin dashboard.
/// GET /api/payments/summary
pub(super) async fn get_summary(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<PaymentSummary>, (StatusCode, Json<serde_json::Value>)> {
    // Revenue today (UTC) — integer cents.
    let revenue_today_cents: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
           FROM orders
           WHERE status = 'completed'
             AND completed_at >= DATE_TRUNC('day', NOW())",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "revenue_today query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("revenue_today query failed: {}", e)})),
        )
    })?;

    // Revenue last 7 days — integer cents.
    let revenue_week_cents: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
           FROM orders
           WHERE status = 'completed'
             AND completed_at >= NOW() - INTERVAL '7 days'",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "revenue_week query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("revenue_week query failed: {}", e)})),
        )
    })?;

    // Revenue current calendar month — integer cents.
    let revenue_month_cents: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
           FROM orders
           WHERE status = 'completed'
             AND completed_at >= DATE_TRUNC('month', NOW())",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "revenue_month query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("revenue_month query failed: {}", e)})),
        )
    })?;

    // MRR: sum active+trialing memberships' plan prices, normalized to monthly,
    // returned in integer cents.
    let mrr_cents: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(SUM(
                CASE LOWER(mp.billing_cycle)
                    WHEN 'monthly'   THEN (mp.price * 100)::BIGINT
                    WHEN 'quarterly' THEN ((mp.price * 100) / 3)::BIGINT
                    WHEN 'annual'    THEN ((mp.price * 100) / 12)::BIGINT
                    WHEN 'yearly'    THEN ((mp.price * 100) / 12)::BIGINT
                    ELSE (mp.price * 100)::BIGINT
                END
           ), 0)::BIGINT
           FROM user_memberships um
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status IN ('active', 'trialing')",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "mrr query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("mrr query failed: {}", e)})),
        )
    })?;

    // Pending and failed order counts.
    let pending_count: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM orders
           WHERE status = 'pending'",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "pending_count query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("pending_count query failed: {}", e)})),
        )
    })?;

    let failed_count: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM orders
           WHERE status = 'failed'",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "failed_count query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("failed_count query failed: {}", e)})),
        )
    })?;

    Ok(Json(PaymentSummary {
        revenue_cents: revenue_month_cents,
        revenue_today_cents,
        revenue_week_cents,
        revenue_month_cents,
        mrr_cents,
        pending_count,
        failed_count,
        last_updated: chrono::Utc::now(),
    }))
}
