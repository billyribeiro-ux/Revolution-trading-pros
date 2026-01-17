//! Analytics routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{models::User, AppState};

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
async fn track_performance(
    State(state): State<AppState>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Store performance metrics in analytics_events table
    sqlx::query(
        r#"
        INSERT INTO analytics_events (event_type, event_name, properties, created_at)
        VALUES ('performance', 'web_vitals', $1, NOW())
        "#,
    )
    .bind(&input)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to store performance metrics: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to store metrics", "details": e.to_string()})),
        )
    })?;

    Ok(Json(json!({"status": "ok"})))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/track", post(track_event))
        .route("/reading", post(track_reading))
        .route("/performance", post(track_performance))
        .route("/overview", get(get_overview))
}
