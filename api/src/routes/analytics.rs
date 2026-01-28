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
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::User,
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

/// Get analytics overview (admin)
async fn get_overview(
    State(state): State<AppState>,
    _user: User,
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

/// Get comprehensive analytics for a trading room
/// GET /api/analytics/room/{room_slug}
async fn get_room_analytics(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<RoomAnalyticsQuery>,
    _user: User,
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

/// Get equity curve data for a trading room
/// GET /api/analytics/room/{room_slug}/equity-curve
async fn get_equity_curve(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<RoomAnalyticsQuery>,
    _user: User,
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

/// Get drawdown periods for a trading room
/// GET /api/analytics/room/{room_slug}/drawdowns
async fn get_drawdown_periods(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<RoomAnalyticsQuery>,
    _user: User,
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

/// Get ticker-specific analytics
/// GET /api/analytics/room/{room_slug}/ticker/{ticker}
async fn get_ticker_analytics(
    State(state): State<AppState>,
    Path((room_slug, ticker)): Path<(String, String)>,
    Query(params): Query<RoomAnalyticsQuery>,
    _user: User,
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

pub fn router() -> Router<AppState> {
    Router::new()
        // Event tracking (public)
        .route("/track", post(track_event))
        .route("/reading", post(track_reading))
        .route("/performance", post(track_performance))
        // Overview (admin)
        .route("/overview", get(get_overview))
        // Room analytics (authenticated users)
        .route("/room/:room_slug", get(get_room_analytics))
        .route("/room/:room_slug/equity-curve", get(get_equity_curve))
        .route("/room/:room_slug/drawdowns", get(get_drawdown_periods))
        .route("/room/:room_slug/ticker/:ticker", get(get_ticker_analytics))
}
