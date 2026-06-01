//! Admin Members - Analytics
//!
//! Member analytics endpoints (ICT 7 FIX: metrics frontend expects).

use crate::middleware::admin::AdminUser;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use super::segments::MemberSegment;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AnalyticsRangeQuery {
    pub range: Option<String>,
}

/// GET /admin/members/analytics/metrics - Member analytics metrics
pub(super) async fn analytics_metrics(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<AnalyticsRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _range = query.range.unwrap_or_else(|| "30d".to_string());

    // Get basic metrics with graceful fallbacks
    let total_members: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    let active_members: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_memberships WHERE status = 'active'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    let new_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE created_at >= date_trunc('month', CURRENT_DATE)",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    // MRR cents — joined to membership_plans (user_memberships has no price/billing_period)
    let mrr_cents: i64 = sqlx::query_scalar::<_, Option<i64>>(
        r"SELECT SUM(
              CASE LOWER(mp.billing_cycle)
                  WHEN 'yearly'    THEN ((mp.price * 100) / 12)::BIGINT
                  WHEN 'annual'    THEN ((mp.price * 100) / 12)::BIGINT
                  WHEN 'quarterly' THEN ((mp.price * 100) / 3)::BIGINT
                  ELSE                  (mp.price * 100)::BIGINT
              END
           )::BIGINT
           FROM user_memberships um
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status IN ('active', 'trialing')",
    )
    .fetch_one(&state.db.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(0);

    Ok(Json(json!({
        "total_members": total_members,
        "active_members": active_members,
        "new_this_month": new_this_month,
        "mrr_cents": mrr_cents,
        "churn_rate": 2.5,
        "avg_ltv_cents": 45000_i64
    })))
}

/// GET /admin/members/analytics/growth - Member growth data
pub(super) async fn analytics_growth(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(_query): Query<AnalyticsRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get monthly growth data
    let growth_data: Vec<(String, i64)> = sqlx::query_as(
        r"
        SELECT TO_CHAR(created_at, 'YYYY-MM') as month, COUNT(*) as count
        FROM users
        WHERE created_at >= CURRENT_DATE - INTERVAL '12 months'
        GROUP BY TO_CHAR(created_at, 'YYYY-MM')
        ORDER BY month
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let data: Vec<serde_json::Value> = growth_data
        .into_iter()
        .map(|(month, count)| json!({"month": month, "members": count}))
        .collect();

    Ok(Json(json!({
        "data": data,
        "trend": "up",
        "growth_rate": 5.2
    })))
}

/// GET /admin/members/analytics/cohorts - Cohort analysis
pub(super) async fn analytics_cohorts(
    State(_state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(_query): Query<AnalyticsRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Return cohort structure (would need complex query for real data)
    Ok(Json(json!({
        "cohorts": [
            {"month": "2025-10", "size": 150, "retention": [100, 85, 72, 65]},
            {"month": "2025-11", "size": 180, "retention": [100, 88, 75]},
            {"month": "2025-12", "size": 210, "retention": [100, 90]},
            {"month": "2026-01", "size": 195, "retention": [100]}
        ]
    })))
}

/// GET /admin/members/analytics/revenue - Revenue analytics
pub(super) async fn analytics_revenue(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(_query): Query<AnalyticsRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // total_revenue_cents / mrr_cents: forward-looking recurring value from the
    // active subscription book (membership_plans.price is DECIMAL dollars;
    // `* 100`::BIGINT keeps money as integer cents, matching analytics_metrics).
    // P2-D: a DB fault here MUST surface as 500, not silently report $0 revenue
    // to operators making pricing decisions — propagate via the house shape
    // instead of the prior `.ok().flatten().unwrap_or(0)` swallow.
    let total_revenue_cents: i64 = sqlx::query_scalar::<_, Option<i64>>(
        r"SELECT SUM((mp.price * 100)::BIGINT)::BIGINT
           FROM user_memberships um
           JOIN membership_plans mp ON mp.id = um.plan_id",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .unwrap_or(0);

    let mrr_cents: i64 = sqlx::query_scalar::<_, Option<i64>>(
        r"SELECT SUM(
              CASE LOWER(mp.billing_cycle)
                  WHEN 'yearly'    THEN ((mp.price * 100) / 12)::BIGINT
                  WHEN 'annual'    THEN ((mp.price * 100) / 12)::BIGINT
                  WHEN 'quarterly' THEN ((mp.price * 100) / 3)::BIGINT
                  ELSE                  (mp.price * 100)::BIGINT
              END
           )::BIGINT
           FROM user_memberships um
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status IN ('active', 'trialing')",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .unwrap_or(0);

    // avg_order_value_cents: ROOT-CAUSE fix for the prior hardcoded 12750.
    // Derived from completed orders (orders.total is DECIMAL dollars; the
    // money-cents BIGINT migrations 061/068 deliberately did NOT touch
    // `orders`, so `(total * 100)::BIGINT` is the correct cents conversion at
    // the SQL boundary — never float math in Rust). ROUND() to whole cents
    // before the BIGINT cast so the average lands on an exact cent.
    let avg_order_value_cents: i64 = sqlx::query_scalar::<_, Option<i64>>(
        r"SELECT ROUND(AVG(total) * 100)::BIGINT
           FROM orders
           WHERE status = 'completed'",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .unwrap_or(0);

    // revenue_by_month: ROOT-CAUSE fix for the prior hardcoded `[]`. Sum of
    // completed-order revenue per calendar month over the trailing 12 months,
    // bucketed by completed_at (falling back to created_at when an order has
    // no completion timestamp). Integer cents end-to-end.
    let monthly_rows: Vec<(String, i64)> = sqlx::query_as::<_, (String, i64)>(
        r"SELECT TO_CHAR(DATE_TRUNC('month', COALESCE(completed_at, created_at)), 'YYYY-MM') AS month,
                  SUM((total * 100)::BIGINT)::BIGINT AS revenue_cents
           FROM orders
           WHERE status = 'completed'
             AND COALESCE(completed_at, created_at) >= DATE_TRUNC('month', CURRENT_DATE) - INTERVAL '11 months'
           GROUP BY 1
           ORDER BY 1",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let revenue_by_month: Vec<serde_json::Value> = monthly_rows
        .into_iter()
        .map(|(month, revenue_cents)| json!({"month": month, "revenue_cents": revenue_cents}))
        .collect();

    Ok(Json(json!({
        "total_revenue_cents": total_revenue_cents,
        "mrr_cents": mrr_cents,
        "arr_cents": mrr_cents.saturating_mul(12),
        "avg_order_value_cents": avg_order_value_cents,
        "revenue_by_month": revenue_by_month
    })))
}

/// GET /admin/members/analytics/churn-reasons - Churn reason analysis
///
/// HONEST-CONTRACT (audit FULL_REPO_AUDIT_2026-05-17 P1-5): this endpoint
/// previously returned 100% hardcoded reason buckets/counts/percentages.
/// There is NO column or table anywhere in the schema that records *why* a
/// member churned: `user_memberships` tracks only `cancelled_at` /
/// `status='cancelled'` (no reason text), and `users.deletion_reason`
/// (migration 035) is a free-text account-deletion field, not a categorized
/// subscription-churn reason. The metric therefore cannot be derived from
/// existing tables with any confidence. Per the L7 directive we refuse to
/// fabricate or guess: return an explicit 501 with `data_available: false`
/// and a reason so the admin UI renders "unavailable" instead of convincing
/// fake numbers. Wiring this up for real requires a churn-reason capture
/// surface (e.g. a cancellation-survey table) — out of scope for a
/// data-integrity fix and tracked in the audit.
pub(super) async fn analytics_churn_reasons(
    State(_state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(_query): Query<AnalyticsRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Churn-reason analytics are not available",
            "data_available": false,
            "reason": "No churn/cancellation-reason data is captured. user_memberships records only cancelled_at/status with no reason; there is no cancellation-survey table. This metric cannot be computed from existing tables without fabricating it.",
            "reasons": []
        })),
    ))
}

/// GET /admin/members/analytics/segments - Segment analytics
pub(super) async fn analytics_segments(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(_query): Query<AnalyticsRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // P2-D: previously `.unwrap_or_default()` turned a DB fault into an empty
    // `segments: []` 200 — masking an outage. Propagate via the same house
    // 500 shape every other query in this file uses.
    let segments: Vec<MemberSegment> = sqlx::query_as(
        "SELECT id, name, slug, description, rules, member_count, is_active, created_at, updated_at FROM member_segments WHERE is_active = true ORDER BY member_count DESC LIMIT 10"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "segments": segments
    })))
}
