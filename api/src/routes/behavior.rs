//! First-party behavior ingestion and summaries over durable, anonymous events.

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Query, State},
    routing::{get, post},
};
use chrono::{TimeDelta, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};

#[derive(Deserialize)]
struct Batch {
    session_id: String,
    visitor_id: String,
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Event {
    event_type: String,
    timestamp: i64,
    page_url: String,
    element_selector: Option<String>,
    event_value: Option<Value>,
    coordinates: Option<Coordinates>,
}

#[derive(Deserialize, serde::Serialize)]
struct Coordinates {
    x: f64,
    y: f64,
}

const EVENT_TYPES: &[&str] = &[
    "page_view",
    "page_exit",
    "navigation_click",
    "back_button",
    "scroll_depth",
    "speed_scroll",
    "scroll_backtrack",
    "scroll_pause",
    "click",
    "rage_click",
    "dead_click",
    "cta_click",
    "cta_hesitation",
    "hover_intent",
    "cursor_thrashing",
    "cursor_idle",
    "exit_intent",
    "form_focus",
    "form_input",
    "form_blur",
    "form_abandon",
    "form_submit",
    "form_error",
    "idle_start",
    "idle_end",
    "tab_blur",
    "tab_focus",
    "copy_text",
    "video_play",
    "video_pause",
    "friction_detected",
    "dead_zone_hover",
    "unexpected_scroll",
    "repeated_action",
];

fn valid_identifier(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

async fn ingest(
    State(state): State<AppState>,
    Json(batch): Json<Batch>,
) -> Result<Json<Value>, ApiError> {
    if !valid_identifier(&batch.session_id)
        || !valid_identifier(&batch.visitor_id)
        || batch.events.is_empty()
        || batch.events.len() > 100
    {
        return Err(ApiError::validation_error(
            "Provide valid session/visitor IDs and 1–100 events",
        ));
    }
    let mut prepared = Vec::new();
    for event in batch.events {
        if !EVENT_TYPES.contains(&event.event_type.as_str()) {
            return Err(ApiError::validation_error("Unknown behavior event type"));
        }
        let time = chrono::DateTime::from_timestamp_millis(event.timestamp)
            .filter(|t| {
                *t >= Utc::now() - TimeDelta::days(7) && *t <= Utc::now() + TimeDelta::minutes(5)
            })
            .ok_or_else(|| ApiError::validation_error("Invalid event timestamp"))?;
        let url = reqwest::Url::parse(&event.page_url)
            .ok()
            .filter(|u| matches!(u.scheme(), "http" | "https") && u.host_str().is_some())
            .ok_or_else(|| ApiError::validation_error("Invalid page URL"))?;
        // Keep only pathname: query strings/fragments can contain email,
        // authentication tokens and other PII. Never persist form values,
        // selected text, arbitrary metadata or client-supplied user identities.
        let path = url.path().to_string();
        if path.len() > 500
            || event
                .element_selector
                .as_ref()
                .is_some_and(|s| s.len() > 500)
            || event.coordinates.as_ref().is_some_and(|c| {
                !c.x.is_finite()
                    || !c.y.is_finite()
                    || !(0.0..=100_000.0).contains(&c.x)
                    || !(0.0..=100_000.0).contains(&c.y)
            })
        {
            return Err(ApiError::validation_error(
                "Behavior event exceeds field limits",
            ));
        }
        let value = event
            .event_value
            .as_ref()
            .and_then(Value::as_f64)
            .filter(|n| n.is_finite());
        if event.event_type == "scroll_depth"
            && !value.is_some_and(|depth| (0.0..=100.0).contains(&depth))
        {
            return Err(ApiError::validation_error(
                "Scroll depth must be between 0 and 100",
            ));
        }
        let coordinates = event.coordinates.as_ref().map(|c| json!(c));
        let hash = hex::encode(Sha256::digest(
            serde_json::to_vec(&json!([
                batch.session_id,
                batch.visitor_id,
                event.event_type,
                event.timestamp,
                path,
                event.element_selector,
                value,
                coordinates
            ]))
            .map_err(|_| ApiError::validation_error("Invalid event"))?,
        ));
        prepared.push((event, time, path, value, coordinates, hash));
    }
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let received = prepared.len();
    let mut persisted = 0;
    for (event, time, path, value, coordinates, hash) in prepared {
        persisted += sqlx::query("INSERT INTO behavior_events (event_hash, session_id, visitor_id, event_type, page_url, element_selector, event_value, coordinates, happened_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT (event_hash) DO NOTHING")
            .bind(hash).bind(&batch.session_id).bind(&batch.visitor_id).bind(event.event_type).bind(path)
            .bind(event.element_selector).bind(value).bind(coordinates).bind(time)
            .execute(&mut *tx).await.map_err(db_error)?.rows_affected();
    }
    tx.commit().await.map_err(db_error)?;
    Ok(Json(
        json!({"success": true, "received": received, "persisted": persisted}),
    ))
}

fn db_error(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}

#[derive(Deserialize)]
struct Filters {
    period: Option<String>,
}

fn period_days(period: Option<&str>) -> Result<i64, ApiError> {
    match period.unwrap_or("7d") {
        "24h" | "1d" => Ok(1),
        "7d" => Ok(7),
        "30d" => Ok(30),
        "90d" => Ok(90),
        _ => Err(ApiError::validation_error(
            "Period must be 24h, 7d, 30d or 90d",
        )),
    }
}

async fn summary(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let days = period_days(filters.period.as_deref())?;
    let totals: (i64, f64, f64) = sqlx::query_as("WITH sessions AS (SELECT session_id, EXTRACT(EPOCH FROM MAX(happened_at)-MIN(happened_at))::float8 AS duration, COUNT(*) FILTER (WHERE event_type='page_view')::float8 AS pages FROM behavior_events WHERE happened_at >= NOW() - $1 * INTERVAL '1 day' GROUP BY session_id) SELECT COUNT(*)::bigint, COALESCE(AVG(duration),0)::float8, COALESCE(AVG(pages),0)::float8 FROM sessions")
        .bind(days).fetch_one(&state.db.pool).await.map_err(db_error)?;
    let engagement: (f64, i64, i64) = sqlx::query_as("SELECT COALESCE(AVG(event_value) FILTER (WHERE event_type='scroll_depth'),0)::float8, COUNT(*) FILTER (WHERE event_type IN ('click','cta_click','navigation_click'))::bigint, COUNT(*)::bigint FROM behavior_events WHERE happened_at >= NOW() - $1 * INTERVAL '1 day'")
        .bind(days).fetch_one(&state.db.pool).await.map_err(db_error)?;
    let clicks: Vec<(String, i64, String)> = sqlx::query_as("SELECT COALESCE(element_selector,'Unknown element'), COUNT(*)::bigint, page_url FROM behavior_events WHERE happened_at >= NOW() - $1 * INTERVAL '1 day' AND event_type IN ('click','cta_click','navigation_click') GROUP BY element_selector,page_url ORDER BY COUNT(*) DESC LIMIT 20")
        .bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let scrolls: Vec<(String, f64, i64)> = sqlx::query_as("SELECT page_url, COALESCE(AVG(event_value),0)::float8, COUNT(*)::bigint FROM behavior_events WHERE happened_at >= NOW() - $1 * INTERVAL '1 day' AND event_type='scroll_depth' GROUP BY page_url ORDER BY COUNT(*) DESC LIMIT 20")
        .bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let heatmap_views: i64 = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM analytics_events WHERE event_type='heatmap_view' AND created_at >= NOW() - $1 * INTERVAL '1 day'")
        .bind(days).fetch_one(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(json!({
        "total_sessions": totals.0, "avg_session_duration": format!("{}s", totals.1.round()),
        "pages_per_session": totals.2, "avg_scroll_depth": engagement.0,
        "click_rate": if engagement.2 > 0 { engagement.1 as f64 / engagement.2 as f64 * 100.0 } else { 0.0 },
        "heatmap_views": heatmap_views,
        "top_clicks": clicks.into_iter().map(|(element,count,page)| json!({"element":element,"count":count,"page":page})).collect::<Vec<_>>(),
        "top_scrolls": scrolls.into_iter().map(|(page,depth,views)| json!({"page":page,"avgDepth":depth,"views":views})).collect::<Vec<_>>(),
        "recordings": []
    })))
}

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/events", post(ingest))
        .layer(DefaultBodyLimit::max(64 * 1024))
}

pub fn analytics_router() -> Router<AppState> {
    Router::new().route("/behavior", get(summary))
}
