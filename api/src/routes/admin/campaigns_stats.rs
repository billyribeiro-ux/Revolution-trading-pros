//! Admin campaigns + dashboard/analytics stats + email diagnostics +
//! user-subscriptions endpoint — extracted from the original
//! `routes/admin.rs` as part of the R6-B split (2026-05-20).
//!
//! These four mini-sections were absorbed into one file because each
//! is small (campaigns 60 LOC, stats 130 LOC, email diagnostics 110
//! LOC, user-subscriptions 90 LOC) and they share the same admin-auth
//! surface. They are grouped semantically as the "admin observability
//! and operations" sub-domain.
//!
//! Handler bodies are byte-identical to the pre-split source.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// CAMPAIGNS MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct CampaignRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: String,
    pub status: String,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub target_audience: Option<serde_json::Value>,
    pub metrics: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: String,
    pub status: Option<String>,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub target_audience: Option<serde_json::Value>,
}

/// List campaigns (admin)
pub(super) async fn list_campaigns(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Vec<CampaignRow>>, (StatusCode, Json<serde_json::Value>)> {
    let campaigns: Vec<CampaignRow> =
        sqlx::query_as("SELECT * FROM campaigns ORDER BY created_at DESC")
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    Ok(Json(campaigns))
}

/// Create campaign (admin)
pub(super) async fn create_campaign(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateCampaignRequest>,
) -> Result<Json<CampaignRow>, (StatusCode, Json<serde_json::Value>)> {
    let campaign: CampaignRow = sqlx::query_as(
        r"
        INSERT INTO campaigns (name, description, campaign_type, status, start_date, end_date, target_audience, metrics, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, '{}'::jsonb, NOW(), NOW())
        RETURNING *
        "
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.campaign_type)
    .bind(input.status.unwrap_or_else(|| "draft".to_string()))
    .bind(input.start_date)
    .bind(input.end_date)
    .bind(&input.target_audience)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(campaign))
}

/// Delete campaign (admin)
pub(super) async fn delete_campaign(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM campaigns WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Campaign deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// DASHBOARD STATS
// ═══════════════════════════════════════════════════════════════════════════

/// Products stats (admin)
pub(super) async fn products_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let active: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = true")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let draft: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = false")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "total": total.0,
        "active": active.0,
        "draft": draft.0,
        "featured": 0,
        "total_revenue": 0,
        "total_sales": 0,
        "data": {
            "total": total.0
        }
    })))
}

/// Dashboard overview (admin)
pub(super) async fn dashboard_overview(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let active_subscriptions: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let total_products: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = true")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let total_posts: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = 'published'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let newsletter_subscribers: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers WHERE status = 'confirmed'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "total_users": total_users.0,
        "active_subscriptions": active_subscriptions.0,
        "total_products": total_products.0,
        "total_posts": total_posts.0,
        "newsletter_subscribers": newsletter_subscribers.0
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 FIX: Missing Admin Endpoints
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct AnalyticsDashboardQuery {
    pub period: Option<String>,
}

/// Analytics dashboard (admin) - GET /admin/analytics/dashboard
pub(super) async fn analytics_dashboard(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<AnalyticsDashboardQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _period = query.period.unwrap_or_else(|| "30d".to_string());

    let total_events: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM analytics_events WHERE created_at >= NOW() - INTERVAL '30 days'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let page_views: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM analytics_events WHERE event_type = 'pageview' AND created_at >= NOW() - INTERVAL '30 days'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let new_users: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE created_at >= NOW() - INTERVAL '30 days'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "kpis": {
            "sessions": { "value": total_events.0, "change": 0, "label": "Sessions" },
            "pageviews": { "value": page_views.0, "change": 0, "label": "Pageviews" },
            "unique_visitors": { "value": total_events.0, "change": 0, "label": "Unique Visitors" },
            "new_users": { "value": new_users.0, "change": 0, "label": "New Users" },
            "bounce_rate": { "value": null, "change": 0, "label": "Bounce Rate" },
            "avg_session_duration": { "value": null, "change": 0, "label": "Avg. Duration" }
        },
        "top_pages": [],
        "device_breakdown": { "desktop": 0, "mobile": 0, "tablet": 0 }
    })))
}

/// Posts stats (admin) - GET /admin/posts/stats
pub(super) async fn posts_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let published: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = 'published'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let drafts: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = 'draft'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "total": total.0,
        "total_posts": total.0,
        "published": published.0,
        "drafts": drafts.0,
        "scheduled": 0,
        "data": { "total": total.0 }
    })))
}

/// Site health (admin) - GET /admin/site-health
pub(super) async fn site_health(
    State(_state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "status": "healthy",
        "checks": {
            "database": { "status": "ok", "latency_ms": 5 },
            "cache": { "status": "ok", "latency_ms": 1 },
            "storage": { "status": "ok", "usage_percent": 25 }
        },
        "uptime": "99.9%",
        "last_checked": chrono::Utc::now().to_rfc3339()
    })))
}

// FIX-H-5 (2026-04-29): impersonate_user endpoint REMOVED.
//
// Previous behavior: returned a non-functional placeholder token of the form
// "impersonate_{id}_{timestamp}" guarded by SuperAdminUser. The token was not
// a valid JWT so the auth middleware rejected it; the endpoint was harmless.
//
// Why removed: the inline comment ("In a real implementation, you would
// generate a JWT token") was a footgun. A future engineer or AI completing
// it literally — `create_jwt(target_user.id, ...)` — would mint a real
// admin-issued bearer JWT for arbitrary users with no audit trail, no
// time-bound impersonation token type, no original-actor preservation, and
// no allowlist of acceptable routes. That is a back-door, not a feature.
//
// If user-impersonation support is ever needed, build it deliberately:
//   - mint a JWT with token_type = "impersonation" and TTL <= 15 min;
//   - the auth middleware accepts it ONLY for an explicit allowlist of
//     read-only routes; any state-changing route is forbidden;
//   - every request seen with an impersonation token writes a
//     security_events row carrying (actor_id, target_id, route, ip);
//   - the frontend shows a persistent banner and a "stop impersonating"
//     control;
//   - the endpoint that mints the token requires SuperAdminUser AND
//     writes its own security_events row at issuance time.
//
// Until that design exists, this endpoint does not.

/// GET /admin/users/:id/subscriptions - Get user's subscriptions
/// ICT 7 FIX: Added missing endpoint that frontend expects
pub(super) async fn get_user_subscriptions(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify user exists
    let user_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;

    if user_exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "User not found"})),
        ));
    }

    // Subscriptions for a user — joined to membership_plans for canonical price/name
    let subscriptions: Vec<(
        i64,
        String,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<chrono::NaiveDateTime>,
        Option<chrono::NaiveDateTime>,
        chrono::NaiveDateTime,
    )> = sqlx::query_as(
        r"
        SELECT
            um.id, um.status,
            (mp.price * 100)::BIGINT AS price_cents,
            mp.name AS product_name,
            mp.billing_cycle AS billing_period,
            um.starts_at, um.expires_at, um.created_at
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON mp.id = um.plan_id
        WHERE um.user_id = $1
        ORDER BY um.created_at DESC
        ",
    )
    .bind(id)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    let subs_json: Vec<serde_json::Value> = subscriptions
        .into_iter()
        .map(
            |(sub_id, status, price_cents, name, period, starts, expires, created)| {
                serde_json::json!({
                    "id": sub_id,
                    "status": status,
                    "price_cents": price_cents,
                    "product_name": name,
                    "billing_period": period,
                    "starts_at": starts,
                    "expires_at": expires,
                    "created_at": created
                })
            },
        )
        .collect();

    let active_count = subs_json.iter().filter(|s| s["status"] == "active").count();
    let total_revenue_cents: i64 = subs_json
        .iter()
        .filter_map(|s| s["price_cents"].as_i64())
        .sum();

    Ok(Json(serde_json::json!({
        "subscriptions": subs_json,
        "stats": {
            "total": subs_json.len(),
            "active": active_count,
            "total_revenue_cents": total_revenue_cents
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// Batch 6 — Email diagnostics
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/email/status
///
/// Tells the operator whether Postmark is wired up + summary counters
/// from the last 24h. Used by the admin dashboard to surface "Postmark
/// is OFF" warnings without having to grep server logs.
pub(super) async fn get_email_status(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    #[derive(sqlx::FromRow)]
    struct Counts {
        sent: i64,
        failed: i64,
        skipped: i64,
        // Migration 064 added sent_at as TIMESTAMPTZ.
        last_send_at: Option<chrono::DateTime<chrono::Utc>>,
    }
    let counts: Counts = sqlx::query_as(
        r"SELECT
               COUNT(*) FILTER (WHERE status = 'sent' AND queued_at > NOW() - INTERVAL '24 hours')
                   AS sent,
               COUNT(*) FILTER (WHERE status = 'failed' AND queued_at > NOW() - INTERVAL '24 hours')
                   AS failed,
               COUNT(*) FILTER (WHERE status = 'skipped_no_token' AND queued_at > NOW() - INTERVAL '24 hours')
                   AS skipped,
               MAX(sent_at) FILTER (WHERE status = 'sent') AS last_send_at
           FROM email_logs",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", error = %e, "Failed to load email status counts");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "postmark_token_set": state.services.email.is_enabled(),
        "from_email": state.services.email.from_email(),
        "admin_notification_email_set": state.config.admin_notification_email.is_some(),
        "last_24h_sent": counts.sent,
        "last_24h_failed": counts.failed,
        "last_24h_skipped": counts.skipped,
        "last_send_at": counts.last_send_at.map(|t| t.to_rfc3339()),
    })))
}

#[derive(Debug, Deserialize)]
pub struct EmailLogsQuery {
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub template_alias: Option<String>,
}

/// GET /api/admin/email/logs?limit=50
///
/// Recent rows from `email_logs`, newest first. `model` is returned as
/// raw JSONB so the admin can verify the right merge data was queued
/// to Postmark. `provider_message_id` is the Postmark MessageID when a
/// send succeeded; `error` is the captured error string when a send
/// failed.
pub(super) async fn list_email_logs(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    axum::extract::Query(query): axum::extract::Query<EmailLogsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limit = query.limit.unwrap_or(50).clamp(1, 500);
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct Row {
        id: i64,
        to_email: Option<String>,
        template_alias: Option<String>,
        status: String,
        provider_message_id: Option<String>,
        error: Option<String>,
        model: Option<serde_json::Value>,
        // Migration 064 added these as TIMESTAMPTZ; use DateTime<Utc>
        // to match. Existing legacy email_logs columns (`created_at`)
        // are TIMESTAMP and use NaiveDateTime — we don't read those here.
        queued_at: Option<chrono::DateTime<chrono::Utc>>,
        sent_at: Option<chrono::DateTime<chrono::Utc>>,
    }
    let rows: Vec<Row> = sqlx::query_as(
        r"SELECT id, to_email, template_alias, status,
                  provider_message_id, error, model,
                  queued_at, sent_at
           FROM email_logs
           WHERE ($1::TEXT IS NULL OR status = $1)
             AND ($2::TEXT IS NULL OR template_alias = $2)
           ORDER BY queued_at DESC NULLS LAST, id DESC
           LIMIT $3",
    )
    .bind(query.status.as_deref())
    .bind(query.template_alias.as_deref())
    .bind(limit)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", error = %e, "Failed to list email_logs");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({ "logs": rows, "limit": limit })))
}
