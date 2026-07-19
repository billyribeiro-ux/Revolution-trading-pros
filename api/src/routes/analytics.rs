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
    body::Bytes,
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

/// Parsed shape of a reading-analytics event coming from the browser.
///
/// The client (`readingAnalytics.ts`) ships these via `navigator.sendBeacon`,
/// which forces `Content-Type: text/plain` to dodge the CORS preflight. Axum's
/// `Json<T>` extractor would refuse that → 415. So `track_reading` reads the
/// raw bytes itself and runs them through this struct, which accepts both
/// snake_case and the camelCase the client sends today.
///
/// Only `post_id` (or its camelCase alias `postId`) is required so we can
/// associate the row with a post. Everything else is preserved into JSONB
/// so future analytics fields don't need another schema change.
/// `post_id` arrives as either a JSON number or a string (the client's TS
/// type is `string | number`; numbers are normal but defensive parsing keeps
/// us from rejecting the rare string variant).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FlexibleId {
    Number(i64),
    Stringy(String),
}

impl FlexibleId {
    fn as_i64(&self) -> Option<i64> {
        match self {
            FlexibleId::Number(n) => Some(*n),
            FlexibleId::Stringy(s) => s.parse().ok(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadingTrackRequest {
    #[serde(alias = "post_id")]
    pub post_id: FlexibleId,
    #[serde(default, alias = "scroll_depth")]
    pub scroll_depth: Option<i32>,
    #[serde(default, alias = "time_on_page")]
    pub time_on_page: Option<i32>,
    /// Anything else the client sends (event, slug, engagementScore, …) is
    /// captured here verbatim and round-tripped into properties JSONB.
    #[serde(flatten)]
    pub extras: std::collections::BTreeMap<String, serde_json::Value>,
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
        r"
        INSERT INTO analytics_events (event_type, event_name, page_url, referrer, properties, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "
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

/// Track reading analytics (public).
///
/// FIX-2026-04-27: previously used `Json<ReadingTrackRequest>`, which 415's on
/// `Content-Type: text/plain` — the only content-type `navigator.sendBeacon`
/// can ship without triggering a CORS preflight. The browser was getting
/// blanket 415s and zero `reading` rows ever landed in `analytics_events`.
///
/// Now reads raw bytes and parses JSON manually, so it works regardless of
/// content-type. The body shape is also widened (see `ReadingTrackRequest`)
/// to accept the client's actual camelCase payload.
async fn track_reading(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let input: ReadingTrackRequest = serde_json::from_slice(&body).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("invalid reading event payload: {e}")})),
        )
    })?;

    // Round-trip the full payload (including any extra client-supplied fields)
    // into properties JSONB so we can analyze whatever the client cares about
    // without round-tripping through another schema migration.
    let post_id_num = input.post_id.as_i64();
    let mut props = serde_json::Map::new();
    if let Some(n) = post_id_num {
        props.insert("post_id".into(), json!(n));
    }
    if let Some(v) = input.scroll_depth {
        props.insert("scroll_depth".into(), json!(v));
    }
    if let Some(v) = input.time_on_page {
        props.insert("time_on_page".into(), json!(v));
    }
    for (k, v) in input.extras {
        props.insert(k, v);
    }

    // Pull the event_name out of `props.event` if the client supplied one
    // (e.g. "reading_start", "reading_completion"). Fall back to "page_read".
    let event_name = props
        .get("event")
        .and_then(|v| v.as_str())
        .unwrap_or("page_read")
        .to_string();

    sqlx::query(
        r"
        INSERT INTO analytics_events (event_type, event_name, properties, created_at)
        VALUES ('reading', $1, $2, NOW())
        ",
    )
    .bind(&event_name)
    .bind(serde_json::Value::Object(props))
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
        r"
        INSERT INTO analytics_events (event_type, event_name, properties, created_at)
        VALUES ('performance', 'web_vitals', $1, NOW())
        ",
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
/// - `bounce_rate`: returns 0.0 — accurate computation requires per-session
///   pageview counts and time-on-page which are not stored in `analytics_events`.
///   A dedicated `analytics_sessions` table would be needed to compute this properly.
async fn get_realtime(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<RealtimeStats>, (StatusCode, Json<serde_json::Value>)> {
    // Active visitors: distinct sessions in the last 5 minutes.
    let active_visitors: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(COUNT(DISTINCT session_id), 0)::BIGINT
           FROM analytics_events
           WHERE created_at >= NOW() - INTERVAL '5 minutes'
             AND session_id IS NOT NULL",
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
        r"SELECT COALESCE(COUNT(DISTINCT session_id), 0)::BIGINT
           FROM analytics_events
           WHERE created_at >= DATE_TRUNC('day', NOW())
             AND session_id IS NOT NULL",
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
        r"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM analytics_events
           WHERE event_type = 'pageview'
             AND created_at >= DATE_TRUNC('day', NOW())",
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
        r"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM analytics_events
           WHERE created_at >= NOW() - INTERVAL '1 minute'",
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
        r"SELECT page_url, COUNT(*)::BIGINT AS views
           FROM analytics_events
           WHERE event_type = 'pageview'
             AND created_at >= DATE_TRUNC('day', NOW())
             AND page_url IS NOT NULL
           GROUP BY page_url
           ORDER BY views DESC
           LIMIT 5",
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
        bounce_rate: 0.0, // schema limitation: requires analytics_sessions table (see fn doc)
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
        // The /room/{slug}/* namespace is owned by room_analytics::router()
        // (nested at the same /analytics prefix). Only the drawdowns report,
        // which that module does not expose, is registered here.
        .route("/room/{slug}/drawdowns", get(get_drawdown_periods))
}
