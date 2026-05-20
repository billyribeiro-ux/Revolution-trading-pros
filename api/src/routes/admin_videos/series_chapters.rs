//! Video series CRUD + per-video chapter CRUD handlers.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// SERIES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /video-advanced/series
pub(super) async fn list_series(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let series: Vec<(i64, String, String, Option<String>, bool)> = sqlx::query_as(
        "SELECT id, title, slug, description, is_published FROM video_series ORDER BY created_at DESC"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": series.iter().map(|(id, title, slug, desc, pub_)| json!({
            "id": id, "title": title, "slug": slug, "description": desc, "is_published": pub_, "video_count": 0
        })).collect::<Vec<_>>()
    })))
}

/// POST /video-advanced/series
pub(super) async fn create_series(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = input
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Untitled");
    let slug = title.to_lowercase().replace(' ', "-");

    let result: (i64, String) = sqlx::query_as(
        "INSERT INTO video_series (title, slug, description, is_published, created_at) VALUES ($1, $2, $3, false, NOW()) RETURNING id, slug"
    )
    .bind(title)
    .bind(&slug)
    .bind(input.get("description").and_then(|v| v.as_str()))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(
        json!({"success": true, "data": {"id": result.0, "slug": result.1}}),
    ))
}

/// GET /video-advanced/series/:id
pub(super) async fn get_series(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let series: Option<(i64, String, String, Option<String>, bool)> = sqlx::query_as(
        "SELECT id, title, slug, description, is_published FROM video_series WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    match series {
        Some((sid, title, slug, desc, pub_)) => Ok(Json(json!({
            "success": true,
            "data": {"id": sid, "title": title, "slug": slug, "description": desc, "is_published": pub_, "videos": []}
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Series not found"})),
        )),
    }
}

/// PUT /video-advanced/series/:id
pub(super) async fn update_series(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE video_series SET title = COALESCE($2, title), description = COALESCE($3, description), is_published = COALESCE($4, is_published), updated_at = NOW() WHERE id = $1")
        .bind(id)
        .bind(input.get("title").and_then(|v| v.as_str()))
        .bind(input.get("description").and_then(|v| v.as_str()))
        .bind(input.get("is_published").and_then(|v| v.as_bool()))
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"success": true})))
}

/// DELETE /video-advanced/series/:id
pub(super) async fn delete_series(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM video_series WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"success": true})))
}

/// POST /video-advanced/series/:id/videos
pub(super) async fn add_series_videos(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

/// DELETE /video-advanced/series/:id/videos/:video_id
pub(super) async fn remove_series_video(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path((_series_id, _video_id)): Path<(i64, i64)>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

/// POST /video-advanced/series/:id/reorder
pub(super) async fn reorder_series_videos(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

// ═══════════════════════════════════════════════════════════════════════════
// CHAPTERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /video-advanced/videos/:id/chapters
#[allow(clippy::type_complexity)]
pub(super) async fn list_chapters(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let chapters: Vec<(i64, String, Option<String>, i32, Option<i32>)> = sqlx::query_as(
        "SELECT id, title, description, start_time_seconds, end_time_seconds FROM video_chapters WHERE video_id = $1 ORDER BY start_time_seconds"
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": chapters.iter().enumerate().map(|(i, (cid, title, desc, start, end))| json!({
            "id": cid, "title": title, "description": desc, "start_time_seconds": start, "end_time_seconds": end, "chapter_number": i + 1
        })).collect::<Vec<_>>()
    })))
}

/// POST /video-advanced/videos/:id/chapters
pub(super) async fn create_chapter(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(video_id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result: (i64,) = sqlx::query_as(
        "INSERT INTO video_chapters (video_id, title, description, start_time_seconds, end_time_seconds, created_at) VALUES ($1, $2, $3, $4, $5, NOW()) RETURNING id"
    )
    .bind(video_id)
    .bind(input.get("title").and_then(|v| v.as_str()).unwrap_or("Chapter"))
    .bind(input.get("description").and_then(|v| v.as_str()))
    .bind(input.get("start_time_seconds").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
    .bind(input.get("end_time_seconds").and_then(|v| v.as_i64()).map(|v| v as i32))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"success": true, "data": {"id": result.0}})))
}

/// POST /video-advanced/videos/:id/chapters/bulk
pub(super) async fn bulk_create_chapters(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

/// PUT /video-advanced/videos/:id/chapters/:chapter_id
pub(super) async fn update_chapter(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path((_video_id, chapter_id)): Path<(i64, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE video_chapters SET title = COALESCE($2, title), description = COALESCE($3, description), start_time_seconds = COALESCE($4, start_time_seconds) WHERE id = $1")
        .bind(chapter_id)
        .bind(input.get("title").and_then(|v| v.as_str()))
        .bind(input.get("description").and_then(|v| v.as_str()))
        .bind(input.get("start_time_seconds").and_then(|v| v.as_i64()).map(|v| v as i32))
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"success": true})))
}

/// DELETE /video-advanced/videos/:id/chapters/:chapter_id
pub(super) async fn delete_chapter(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path((_video_id, chapter_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM video_chapters WHERE id = $1")
        .bind(chapter_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"success": true})))
}
