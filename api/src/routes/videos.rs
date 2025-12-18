//! Video routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::User,
    AppState,
};

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct VideoRow {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: String,
    pub thumbnail: Option<String>,
    pub duration_seconds: i32,
    pub is_public: bool,
    pub views_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct VideoListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event_type: String, // play, pause, complete, seek
    pub timestamp_seconds: Option<i32>,
}

/// List videos (public)
async fn list_videos(
    State(state): State<AppState>,
    Query(query): Query<VideoListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let videos: Vec<VideoRow> = sqlx::query_as(
        "SELECT * FROM videos WHERE is_public = true ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM videos WHERE is_public = true")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "data": videos,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}

/// Get video by ID
async fn get_video(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<VideoRow>, (StatusCode, Json<serde_json::Value>)> {
    let video: VideoRow = sqlx::query_as("SELECT * FROM videos WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))?;

    // Increment view count
    let _ = sqlx::query("UPDATE videos SET views_count = views_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await;

    Ok(Json(video))
}

/// Track video event
async fn track_event(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<TrackEventRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Log analytics event
    sqlx::query(
        "INSERT INTO analytics_events (event_type, event_name, properties, created_at) VALUES ('video', $1, $2, NOW())"
    )
    .bind(&input.event_type)
    .bind(json!({
        "video_id": id,
        "timestamp_seconds": input.timestamp_seconds
    }))
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"status": "ok"})))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_videos))
        .route("/:id", get(get_video))
        .route("/:id/track", post(track_event))
}
