//! First-party behavior ingestion and summaries over durable, anonymous events.

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, Query, State},
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
    coordinate_space: Option<String>,
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
    for mut event in batch.events {
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
        if path.contains('@')
            || path.to_ascii_lowercase().contains("%40")
            || [
                "/admin",
                "/account",
                "/auth",
                "/checkout",
                "/login",
                "/register",
                "/dashboard",
                "/my",
                "/profile",
                "/settings",
                "/billing",
                "/orders",
                "/member",
                "/members",
                "/rooms",
                "/classroom",
                "/reset-password",
                "/verify-email",
                "/forgot-password",
                "/unsubscribe",
            ]
            .iter()
            .any(|prefix| path == *prefix || path.starts_with(&format!("{prefix}/")))
        {
            return Err(ApiError::validation_error(
                "Private page path is not tracked",
            ));
        }
        // Client-supplied CSS IDs and classes can encode names or account IDs.
        event.element_selector = event.element_selector.take().filter(|selector| {
            matches!(
                selector.as_str(),
                "a" | "button"
                    | "div"
                    | "span"
                    | "img"
                    | "svg"
                    | "path"
                    | "li"
                    | "nav"
                    | "header"
                    | "footer"
                    | "section"
                    | "article"
                    | "p"
                    | "h1"
                    | "h2"
                    | "h3"
                    | "h4"
                    | "h5"
                    | "h6"
                    | "label"
                    | "form"
                    | "main"
                    | "summary"
                    | "details"
            )
        });
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
            || event
                .coordinate_space
                .as_deref()
                .is_some_and(|space| space != "viewport_percent")
            || (event.coordinate_space.is_some()
                && event
                    .coordinates
                    .as_ref()
                    .is_some_and(|c| c.x > 100.0 || c.y > 100.0))
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
                coordinates,
                event.coordinate_space
            ]))
            .map_err(|_| ApiError::validation_error("Invalid event"))?,
        ));
        prepared.push((event, time, path, value, coordinates, hash));
    }
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let received = prepared.len();
    let mut persisted = 0;
    for (event, time, path, value, coordinates, hash) in prepared {
        persisted += sqlx::query("INSERT INTO behavior_events (event_hash, session_id, visitor_id, event_type, page_url, element_selector, event_value, coordinates, happened_at, coordinate_space) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT (event_hash) DO NOTHING")
            .bind(hash).bind(&batch.session_id).bind(&batch.visitor_id).bind(event.event_type).bind(path)
            .bind(event.element_selector).bind(value).bind(coordinates).bind(time).bind(event.coordinate_space)
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
    let engagement: (f64, i64, i64) = sqlx::query_as("SELECT COALESCE((SELECT AVG(COALESCE(depth,0))::float8 FROM (SELECT session_id,page_url FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' AND event_type='page_view' GROUP BY session_id,page_url) AS views LEFT JOIN (SELECT session_id,page_url,MAX(event_value) AS depth FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' AND event_type='scroll_depth' GROUP BY session_id,page_url) AS depths USING (session_id,page_url)),0)::float8, COUNT(*) FILTER (WHERE event_type IN ('click','cta_click','navigation_click'))::bigint, COUNT(*)::bigint FROM behavior_events WHERE happened_at >= NOW() - $1 * INTERVAL '1 day'")
        .bind(days).fetch_one(&state.db.pool).await.map_err(db_error)?;
    let clicks: Vec<(String, i64, String)> = sqlx::query_as("SELECT COALESCE(element_selector,'Unknown element'), COUNT(*)::bigint, page_url FROM behavior_events WHERE happened_at >= NOW() - $1 * INTERVAL '1 day' AND event_type IN ('click','cta_click','navigation_click') GROUP BY element_selector,page_url ORDER BY COUNT(*) DESC LIMIT 20")
        .bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let scrolls: Vec<(String, f64, i64)> = sqlx::query_as("WITH views AS (SELECT page_url,session_id FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' AND event_type='page_view' GROUP BY page_url,session_id), depths AS (SELECT page_url,session_id,MAX(event_value) AS depth FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' AND event_type='scroll_depth' GROUP BY page_url,session_id) SELECT views.page_url,COALESCE(AVG(COALESCE(depths.depth,0)),0)::float8,COUNT(*)::bigint FROM views LEFT JOIN depths USING (page_url,session_id) GROUP BY views.page_url ORDER BY COUNT(*) DESC LIMIT 20")
        .bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let recent_sessions: Vec<(String, f64, i64, chrono::DateTime<Utc>)> = sqlx::query_as("SELECT session_id,EXTRACT(EPOCH FROM MAX(happened_at)-MIN(happened_at))::float8,COUNT(DISTINCT page_url)::bigint,MIN(happened_at) FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' GROUP BY session_id ORDER BY MIN(happened_at) DESC LIMIT 5")
        .bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(json!({
        "total_sessions": totals.0, "avg_session_duration": format!("{}s", totals.1.round()),
        "pages_per_session": totals.2, "avg_scroll_depth": engagement.0,
        "click_rate": if engagement.2 > 0 { engagement.1 as f64 / engagement.2 as f64 * 100.0 } else { 0.0 },
        "click_events": engagement.1,
        "top_clicks": clicks.into_iter().map(|(element,count,page)| json!({"element":element,"count":count,"page":page})).collect::<Vec<_>>(),
        "top_scrolls": scrolls.into_iter().map(|(page,depth,views)| json!({"page":page,"avgDepth":depth,"views":views})).collect::<Vec<_>>(),
        "recent_sessions": recent_sessions.into_iter().map(|(id,duration,pages,date)|json!({"id":id,"duration":format!("{}s",duration.round()),"pages":pages,"date":date.to_rfc3339()})).collect::<Vec<_>>()
    })))
}

#[derive(Deserialize)]
struct HeatmapFilters {
    period: Option<String>,
    page_url: Option<String>,
}

async fn heatmaps(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<HeatmapFilters>,
) -> Result<Json<Value>, ApiError> {
    let days = period_days(filters.period.as_deref())?;
    if let Some(path) = filters.page_url {
        if !path.starts_with('/') || path.len() > 500 || path.contains('?') || path.contains('#') {
            return Err(ApiError::validation_error("Invalid page path"));
        }
        // Only normalized viewport coordinates are plotted. Older absolute
        // coordinates cannot be compared across device sizes reliably.
        let points: Vec<(f64, f64, i64)> = sqlx::query_as(
            "SELECT (FLOOR((coordinates->>'x')::float8 / 5) * 5)::float8, (FLOOR((coordinates->>'y')::float8 / 5) * 5)::float8, COUNT(*)::bigint FROM behavior_events WHERE page_url=$1 AND happened_at>=NOW()-$2*INTERVAL '1 day' AND event_type='click' AND coordinate_space='viewport_percent' AND coordinates IS NOT NULL GROUP BY 1,2 ORDER BY 3 DESC LIMIT 400"
        ).bind(&path).bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
        let scrolls: Vec<(i32, i64)> = sqlx::query_as(
            "WITH depths AS (SELECT session_id, MAX(event_value) AS depth FROM behavior_events WHERE page_url=$1 AND happened_at>=NOW()-$2*INTERVAL '1 day' AND event_type='scroll_depth' GROUP BY session_id) SELECT threshold, COUNT(*) FILTER (WHERE depth>=threshold)::bigint FROM depths CROSS JOIN (VALUES (25),(50),(75),(90),(100)) AS thresholds(threshold) GROUP BY threshold ORDER BY threshold"
        ).bind(&path).bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
        let sessions: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT session_id)::bigint FROM behavior_events WHERE page_url=$1 AND happened_at>=NOW()-$2*INTERVAL '1 day' AND event_type='page_view'"
        ).bind(&path).bind(days).fetch_one(&state.db.pool).await.map_err(db_error)?;
        return Ok(Json(
            json!({"page_url":path,"points":points.into_iter().map(|(x,y,count)|json!({"x":x,"y":y,"count":count})).collect::<Vec<_>>(),"scroll_depth":scrolls.into_iter().map(|(depth,count)|json!({"depth":depth,"sessions":count})).collect::<Vec<_>>(),"sessions":sessions}),
        ));
    }
    let pages: Vec<(String, i64, i64, f64, chrono::DateTime<Utc>)> = sqlx::query_as(
        "WITH per_session AS (SELECT page_url,session_id,COUNT(*) FILTER (WHERE event_type='click')::bigint AS clicks,MAX(event_value) FILTER (WHERE event_type='scroll_depth') AS scroll_depth,MAX(happened_at) AS last_event FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' GROUP BY page_url,session_id) SELECT page_url,SUM(clicks)::bigint,COUNT(*)::bigint,COALESCE(AVG(COALESCE(scroll_depth,0)),0)::float8,MAX(last_event) FROM per_session GROUP BY page_url ORDER BY SUM(clicks) DESC,MAX(last_event) DESC LIMIT 100"
    ).bind(days).fetch_all(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(
        json!({"pages":pages.into_iter().map(|(url,clicks,sessions,depth,updated)|json!({"id":url,"url":url,"title":url,"clicks":clicks,"sessions":sessions,"scroll_depth":depth.round(),"last_updated":updated})).collect::<Vec<_>>()}),
    ))
}

#[derive(Deserialize)]
struct SessionFilters {
    period: Option<String>,
    filter: Option<String>,
    page: Option<i64>,
}

async fn sessions(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<SessionFilters>,
) -> Result<Json<Value>, ApiError> {
    let days = period_days(filters.period.as_deref())?;
    let filter = filters.filter.as_deref().unwrap_or("all");
    if !matches!(filter, "all" | "with_errors" | "rage_clicks" | "long") {
        return Err(ApiError::validation_error("Invalid session filter"));
    }
    let page = filters.page.unwrap_or(1);
    if !(1..=10_000).contains(&page) {
        return Err(ApiError::validation_error("Page out of range"));
    }
    let total: i64 = sqlx::query_scalar(
        "WITH sessions AS (SELECT session_id,BOOL_OR(event_type='form_error') AS has_errors,BOOL_OR(event_type='rage_click') AS has_rage_clicks,EXTRACT(EPOCH FROM MAX(happened_at)-MIN(happened_at)) AS duration FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' GROUP BY session_id) SELECT COUNT(*)::bigint FROM sessions WHERE ($2='all' OR $2='with_errors' AND has_errors OR $2='rage_clicks' AND has_rage_clicks OR $2='long' AND duration>=60)"
    ).bind(days).bind(filter).fetch_one(&state.db.pool).await.map_err(db_error)?;
    let rows: Vec<(String, f64, i64, i64, chrono::DateTime<Utc>, bool, bool)> = sqlx::query_as(
        "WITH sessions AS (SELECT session_id,EXTRACT(EPOCH FROM MAX(happened_at)-MIN(happened_at))::float8 AS duration,COUNT(DISTINCT page_url)::bigint AS pages_viewed,COUNT(*)::bigint AS events_count,MIN(happened_at) AS started_at,BOOL_OR(event_type='form_error') AS has_errors,BOOL_OR(event_type='rage_click') AS has_rage_clicks FROM behavior_events WHERE happened_at>=NOW()-$1*INTERVAL '1 day' GROUP BY session_id) SELECT session_id,duration,pages_viewed,events_count,started_at,has_errors,has_rage_clicks FROM sessions WHERE ($2='all' OR $2='with_errors' AND has_errors OR $2='rage_clicks' AND has_rage_clicks OR $2='long' AND duration>=60) ORDER BY started_at DESC LIMIT 20 OFFSET $3"
    ).bind(days).bind(filter).bind((page-1)*20).fetch_all(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(
        json!({"sessions":rows.into_iter().map(|(id,duration,pages,events,started,errors,rage)|json!({"session_id":id,"duration":duration.round() as i64,"pages_viewed":pages,"events_count":events,"started_at":started,"has_errors":errors,"has_rage_clicks":rage})).collect::<Vec<_>>(),"pagination":{"page":page,"total_pages":((total+19)/20).max(1),"total":total}}),
    ))
}

async fn session_events(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(session_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    if !valid_identifier(&session_id) {
        return Err(ApiError::validation_error("Invalid session ID"));
    }
    let rows: Vec<(String, String, chrono::DateTime<Utc>, Option<f64>)> = sqlx::query_as(
        "SELECT event_type,page_url,happened_at,event_value FROM behavior_events WHERE session_id=$1 AND received_at>=NOW()-INTERVAL '90 days' ORDER BY happened_at,id LIMIT 501"
    ).bind(&session_id).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let truncated = rows.len() > 500;
    Ok(Json(
        json!({"session_id":session_id,"truncated":truncated,"events":rows.into_iter().take(500).map(|(kind,path,time,value)|json!({"event_type":kind,"page_url":path,"timestamp":time,"event_value":value})).collect::<Vec<_>>()}),
    ))
}

/// Delete in small batches so retention does not hold a long table lock.
pub async fn prune_expired(pool: &sqlx::PgPool) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM behavior_events WHERE id IN (SELECT id FROM behavior_events WHERE received_at<NOW()-INTERVAL '90 days' ORDER BY received_at LIMIT 1000 FOR UPDATE SKIP LOCKED)")
        .execute(pool).await.map(|result| result.rows_affected())
}

pub async fn run_retention(pool: sqlx::PgPool) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
    loop {
        interval.tick().await;
        loop {
            let result = prune_expired(&pool).await;
            match result {
                Ok(1000) => {
                    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
                    continue;
                }
                Ok(_) => break,
                Err(error) => {
                    tracing::error!(%error, "Behavior retention cleanup failed");
                    break;
                }
            }
        }
    }
}

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/events", post(ingest))
        .layer(DefaultBodyLimit::max(64 * 1024))
}

pub fn analytics_router() -> Router<AppState> {
    Router::new()
        .route("/behavior", get(summary))
        .route("/behavior/heatmaps", get(heatmaps))
        .route("/behavior/sessions", get(sessions))
        .route("/behavior/sessions/{session_id}", get(session_events))
}
