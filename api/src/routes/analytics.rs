//! Analytics routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! Comprehensive analytics API for tracking and room performance analysis.
//! Includes:
//! - Event tracking (pageviews, interactions)
//! - Room analytics (Explosive Swings, etc.)
//! - Performance metrics with Sharpe ratio, profit factor, drawdown
//! - Equity curves and P&L analysis

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    middleware::admin::AdminUser,
    services::analytics::{AnalyticsService, DateRange},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct TrackRequest {
    pub event_type: String,
    pub event_name: String,
    pub page_url: Option<String>,
    pub referrer: Option<String>,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ReadingTrackRequest {
    pub post_id: i64,
    pub scroll_depth: Option<i32>,
    pub time_on_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub event_type: Option<String>,
}

/// Track analytics event (public)
async fn track_event(
    State(state): State<AppState>,
    Json(input): Json<TrackRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r#"
        INSERT INTO analytics_events (event_type, event_name, page_url, referrer, properties, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#
    )
    .bind(&input.event_type)
    .bind(&input.event_name)
    .bind(&input.page_url)
    .bind(&input.referrer)
    .bind(&input.properties)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"status": "ok"})))
}

/// Track reading analytics (public)
async fn track_reading(
    State(state): State<AppState>,
    Json(input): Json<ReadingTrackRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r#"
        INSERT INTO analytics_events (event_type, event_name, properties, created_at)
        VALUES ('reading', 'page_read', $1, NOW())
        "#,
    )
    .bind(json!({
        "post_id": input.post_id,
        "scroll_depth": input.scroll_depth,
        "time_on_page": input.time_on_page
    }))
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({"status": "ok"})))
}

/// Get analytics overview (admin only - sensitive dashboard data)
async fn get_overview(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let start_date = query.start_date.unwrap_or_else(|| {
        (chrono::Utc::now() - chrono::Duration::days(30))
            .format("%Y-%m-%d")
            .to_string()
    });

    let total_events: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM analytics_events WHERE created_at >= $1::date")
            .bind(&start_date)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let page_views: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM analytics_events WHERE event_type = 'pageview' AND created_at >= $1::date"
    )
    .bind(&start_date)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let unique_visitors: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT session_id) FROM analytics_events WHERE created_at >= $1::date",
    )
    .bind(&start_date)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "total_events": total_events.0,
        "page_views": page_views.0,
        "unique_visitors": unique_visitors.0,
        "start_date": start_date
    })))
}

/// Performance tracking (public - for frontend)
/// Accepts performance metrics from frontend monitoring
/// Supports both JSON and text/plain (from navigator.sendBeacon)
async fn track_performance(
    State(state): State<AppState>,
    body: axum::body::Bytes,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Parse body - could be JSON or text/plain from sendBeacon
    let input: serde_json::Value = if body.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&body).unwrap_or_else(|e| {
            tracing::warn!("Failed to parse performance metrics as JSON: {:?}", e);
            json!({})
        })
    };

    // Store performance metrics in analytics_events table - fail silently if table doesn't exist
    let _ = sqlx::query(
        r#"
        INSERT INTO analytics_events (event_type, event_name, properties, created_at)
        VALUES ('performance', 'web_vitals', $1, NOW())
        "#,
    )
    .bind(&input)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::debug!(
            "Performance metrics not stored (table may not exist yet): {:?}",
            e
        );
    });

    // Always return success to prevent frontend errors
    Ok(Json(json!({"status": "ok"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROOM ANALYTICS TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct RoomAnalyticsQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl RoomAnalyticsQuery {
    fn to_date_range(&self) -> DateRange {
        DateRange {
            from: self
                .from
                .as_ref()
                .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
            to: self
                .to
                .as_ref()
                .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ROOM ANALYTICS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// Get comprehensive analytics for a trading room (admin only)
/// GET /api/analytics/room/{room_slug}
async fn get_room_analytics(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<RoomAnalyticsQuery>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let service = AnalyticsService::new(state.db.pool.clone());
    let date_range = params.to_date_range();

    match service.get_room_analytics(&room_slug, date_range).await {
        Ok(analytics) => Ok(Json(json!({
            "success": true,
            "data": analytics
        }))),
        Err(e) => {
            tracing::error!("Failed to get room analytics: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to get analytics: {}", e)
                })),
            ))
        }
    }
}

/// Get equity curve data for a trading room (admin only)
/// GET /api/analytics/room/{room_slug}/equity-curve
async fn get_equity_curve(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<RoomAnalyticsQuery>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let service = AnalyticsService::new(state.db.pool.clone());
    let date_range = params.to_date_range();

    match service.get_equity_curve(&room_slug, date_range).await {
        Ok(equity_curve) => Ok(Json(json!({
            "success": true,
            "data": equity_curve
        }))),
        Err(e) => {
            tracing::error!("Failed to get equity curve: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to get equity curve: {}", e)
                })),
            ))
        }
    }
}

/// Get drawdown periods for a trading room (admin only)
/// GET /api/analytics/room/{room_slug}/drawdowns
async fn get_drawdown_periods(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<RoomAnalyticsQuery>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let service = AnalyticsService::new(state.db.pool.clone());
    let date_range = params.to_date_range();

    match service.get_drawdown_periods(&room_slug, date_range).await {
        Ok(drawdowns) => Ok(Json(json!({
            "success": true,
            "data": drawdowns
        }))),
        Err(e) => {
            tracing::error!("Failed to get drawdown periods: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to get drawdown periods: {}", e)
                })),
            ))
        }
    }
}

/// Get ticker-specific analytics (admin only)
/// GET /api/analytics/room/{room_slug}/ticker/{ticker}
async fn get_ticker_analytics(
    State(state): State<AppState>,
    Path((room_slug, ticker)): Path<(String, String)>,
    Query(params): Query<RoomAnalyticsQuery>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let service = AnalyticsService::new(state.db.pool.clone());
    let date_range = params.to_date_range();

    match service.get_room_analytics(&room_slug, date_range).await {
        Ok(analytics) => {
            // Find the specific ticker performance
            let ticker_upper = ticker.to_uppercase();
            let ticker_perf = analytics
                .performance_by_ticker
                .iter()
                .find(|t| t.ticker == ticker_upper);

            match ticker_perf {
                Some(perf) => Ok(Json(json!({
                    "success": true,
                    "data": perf
                }))),
                None => Ok(Json(json!({
                    "success": true,
                    "data": null,
                    "message": format!("No trades found for ticker {}", ticker_upper)
                }))),
            }
        }
        Err(e) => {
            tracing::error!("Failed to get ticker analytics: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to get ticker analytics: {}", e)
                })),
            ))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REALTIME DASHBOARD METRICS
// ═══════════════════════════════════════════════════════════════════════════

/// Top page entry returned by `/api/analytics/realtime`.
#[derive(Debug, Serialize)]
struct TopPage {
    path: String,
    views: i64,
}

/// Realtime analytics summary consumed by the admin dashboard widgets.
///
/// FIX-2026-04-26: shape is hand-tuned to satisfy the `DashboardMetrics`
/// fields the page mixes into its state (visitors, sessions, pageviews,
/// bounce_rate). Extra fields (`top_pages`, `events_per_minute`,
/// `last_updated`, `active_visitors`) are additive — the page's `...data`
/// spread tolerates them.
#[derive(Debug, Serialize)]
struct RealtimeStats {
    active_visitors: i64,
    visitors: i64,
    sessions: i64,
    pageviews: i64,
    bounce_rate: f64,
    events_per_minute: i64,
    top_pages: Vec<TopPage>,
    last_updated: chrono::DateTime<chrono::Utc>,
}

/// Get realtime analytics snapshot for the admin dashboard.
/// GET /api/analytics/realtime
///
/// Aggregates from the `analytics_events` table:
/// - `active_visitors`: distinct sessions in the last 5 minutes
/// - `visitors`: distinct sessions today (UTC)
/// - `sessions`: same denominator (kept for the dashboard's
///   `DashboardMetrics.sessions` field)
/// - `pageviews`: count of `event_type = 'pageview'` today
/// - `events_per_minute`: events in the last minute
/// - `top_pages`: top 5 page paths today by view count
/// - `bounce_rate`: defaulted to 0.0 — accurate computation requires
///   session-duration aggregation that the current schema does not
///   capture. TODO: integrate with full analytics service for bounce rate.
async fn get_realtime(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<RealtimeStats>, (StatusCode, Json<serde_json::Value>)> {
    // Active visitors: distinct sessions in the last 5 minutes.
    let active_visitors: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(DISTINCT session_id), 0)::BIGINT
           FROM analytics_events
           WHERE created_at >= NOW() - INTERVAL '5 minutes'
             AND session_id IS NOT NULL"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "analytics", "active_visitors query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("active_visitors query failed: {}", e)})),
        )
    })?;

    // Distinct sessions today (UTC) — used for both `visitors` and `sessions`.
    let visitors: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(DISTINCT session_id), 0)::BIGINT
           FROM analytics_events
           WHERE created_at >= DATE_TRUNC('day', NOW())
             AND session_id IS NOT NULL"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "analytics", "visitors query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("visitors query failed: {}", e)})),
        )
    })?;

    // Pageviews today.
    let pageviews: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM analytics_events
           WHERE event_type = 'pageview'
             AND created_at >= DATE_TRUNC('day', NOW())"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "analytics", "pageviews query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("pageviews query failed: {}", e)})),
        )
    })?;

    // Events per minute over the last minute.
    let events_per_minute: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM analytics_events
           WHERE created_at >= NOW() - INTERVAL '1 minute'"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "analytics", "events_per_minute query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("events_per_minute query failed: {}", e)})),
        )
    })?;

    // Top 5 pages today by view count.
    let top_page_rows: Vec<(Option<String>, i64)> = sqlx::query_as(
        r#"SELECT page_url, COUNT(*)::BIGINT AS views
           FROM analytics_events
           WHERE event_type = 'pageview'
             AND created_at >= DATE_TRUNC('day', NOW())
             AND page_url IS NOT NULL
           GROUP BY page_url
           ORDER BY views DESC
           LIMIT 5"#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "analytics", "top_pages query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("top_pages query failed: {}", e)})),
        )
    })?;

    let top_pages: Vec<TopPage> = top_page_rows
        .into_iter()
        .filter_map(|(path, views)| path.map(|p| TopPage { path: p, views }))
        .collect();

    Ok(Json(RealtimeStats {
        active_visitors,
        visitors,
        sessions: visitors,
        pageviews,
        // TODO: integrate with full analytics service to compute true bounce rate
        // (requires per-session pageview counts and time-on-page).
        bounce_rate: 0.0,
        events_per_minute,
        top_pages,
        last_updated: chrono::Utc::now(),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Event tracking (public)
        .route("/track", post(track_event))
        .route("/reading", post(track_reading))
        .route("/performance", post(track_performance))
        // Overview (admin)
        .route("/overview", get(get_overview))
        // FIX-2026-04-26: realtime dashboard metrics — replaces 404 from
        // /admin/dashboard's `fetchMetrics` call.
        .route("/realtime", get(get_realtime))
        // Room analytics (authenticated users)
        .route("/room/:room_slug", get(get_room_analytics))
        .route("/room/:room_slug/equity-curve", get(get_equity_curve))
        .route("/room/:room_slug/drawdowns", get(get_drawdown_periods))
        .route("/room/:room_slug/ticker/:ticker", get(get_ticker_analytics))
}
