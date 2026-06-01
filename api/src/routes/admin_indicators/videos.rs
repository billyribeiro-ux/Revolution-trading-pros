//! ICT 7 Grade — Video Management endpoints for indicators.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::types::{CreateVideoRequest, IndicatorVideoRow};

/// ICT 7: List all videos for an indicator
pub(super) async fn list_indicator_videos(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let videos: Vec<IndicatorVideoRow> = sqlx::query_as(
        r"
        SELECT * FROM indicator_videos
        WHERE indicator_id = $1
        ORDER BY display_order, id
        ",
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": videos
    })))
}

/// ICT 7: Add a new video to an indicator
pub(super) async fn create_indicator_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: IndicatorVideoRow = sqlx::query_as(
        r"
        INSERT INTO indicator_videos (
            indicator_id, title, description, bunny_video_guid, bunny_library_id,
            embed_url, play_url, thumbnail_url, duration_seconds, display_order,
            is_featured, is_preview
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        ",
    )
    .bind(indicator_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.bunny_video_guid)
    .bind(&input.bunny_library_id)
    .bind(&input.embed_url)
    .bind(&input.play_url)
    .bind(&input.thumbnail_url)
    .bind(input.duration_seconds)
    .bind(input.display_order.unwrap_or(0))
    .bind(input.is_featured.unwrap_or(false))
    .bind(input.is_preview.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create video: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Video added successfully",
        "data": video
    })))
}

/// ICT 7: Delete an indicator video
pub(super) async fn delete_indicator_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((indicator_id, video_id)): Path<(i64, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM indicator_videos WHERE id = $1 AND indicator_id = $2")
        .bind(video_id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete video: {}", e)})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Video not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Video deleted successfully"
    })))
}
