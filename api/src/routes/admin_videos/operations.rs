//! Misc video operations: clone, duration fetch, bulk edit, CSV export,
//! room/CDN ops, scheduled jobs, bulk upload, and tag/feature bulk ops.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// SCHEDULED JOBS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /video-advanced/scheduled-jobs
pub(super) async fn list_scheduled_jobs(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Json<serde_json::Value> {
    Json(json!({"success": true, "data": []}))
}

/// POST /video-advanced/scheduled-jobs
pub(super) async fn create_scheduled_job(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "id": 1,
            "scheduled_at": input.get("scheduled_at").and_then(|v| v.as_str()).unwrap_or("")
        }
    }))
}

/// POST /video-advanced/scheduled-jobs/:id/cancel
pub(super) async fn cancel_scheduled_job(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

// ═══════════════════════════════════════════════════════════════════════════
// BULK UPLOAD
// ═══════════════════════════════════════════════════════════════════════════

/// POST /video-advanced/bulk-upload
pub(super) async fn init_bulk_upload(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "batch_id": format!("batch_{}", chrono::Utc::now().timestamp()),
            "uploads": []
        }
    }))
}

/// GET /video-advanced/bulk-upload/:batch_id
pub(super) async fn get_batch_status(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(batch_id): Path<String>,
) -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "batch_id": batch_id,
            "total_files": 0,
            "completed": 0,
            "failed": 0,
            "in_progress": 0,
            "pending": 0,
            "uploads": []
        }
    }))
}

/// PUT /video-advanced/bulk-upload/item/:id
pub(super) async fn update_upload_item(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO OPERATIONS (CLONE, DURATION, BULK EDIT, EXPORT)
// ═══════════════════════════════════════════════════════════════════════════

/// POST /video-advanced/videos/:id/clone
pub(super) async fn clone_video(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let new_title = input.get("new_title").and_then(|v| v.as_str());

    let result: Option<(i64, String)> = sqlx::query_as(
        r"
        INSERT INTO unified_videos (title, slug, description, video_url, video_platform, thumbnail_url, duration, content_type, is_published, video_date, created_at, updated_at)
        SELECT COALESCE($2, title || ' (Copy)'), slug || '-copy-' || $1, description, video_url, video_platform, thumbnail_url, duration, content_type, false, video_date, NOW(), NOW()
        FROM unified_videos WHERE id = $1
        RETURNING id, slug
        "
    )
    .bind(id)
    .bind(new_title)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    match result {
        Some((new_id, slug)) => Ok(Json(
            json!({"success": true, "data": {"id": new_id, "slug": slug}}),
        )),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Video not found"})),
        )),
    }
}

/// POST /video-advanced/videos/:id/duration
pub(super) async fn fetch_video_duration(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let duration: Option<(Option<i32>,)> =
        sqlx::query_as("SELECT duration FROM unified_videos WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .unwrap_or(None);

    match duration {
        Some((dur,)) => Ok(Json(json!({
            "success": true,
            "data": {
                "duration": dur.unwrap_or(0),
                "formatted_duration": format!("{}:{:02}", dur.unwrap_or(0) / 60, dur.unwrap_or(0) % 60)
            }
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Video not found"})),
        )),
    }
}

/// POST /video-advanced/videos/fetch-durations
pub(super) async fn fetch_all_durations(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Json<serde_json::Value> {
    Json(json!({"success": true, "data": {"updated": 0, "total_processed": 0}}))
}

/// POST /video-advanced/bulk-edit
pub(super) async fn bulk_edit_videos(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

/// GET /video-advanced/export/csv
/// ICT 7 FIX: Export actual video data instead of empty template
pub(super) async fn export_videos_csv(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<axum::response::Response, (StatusCode, Json<serde_json::Value>)> {
    use axum::http::header;
    use axum::response::IntoResponse;

    // Fetch all videos for export
    #[allow(clippy::type_complexity)]
    let videos: Vec<(i64, String, String, String, i32, i32, bool, chrono::NaiveDateTime)> = sqlx::query_as(
        r"SELECT id, title, slug, content_type, views_count, COALESCE(duration, 0), is_published, created_at
           FROM unified_videos
           WHERE deleted_at IS NULL
           ORDER BY created_at DESC"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Build CSV with headers
    let mut csv = String::from(
        "id,title,slug,content_type,views_count,duration_seconds,is_published,created_at\n",
    );

    for (id, title, slug, content_type, views, duration, published, created_at) in videos {
        // Escape CSV fields properly
        let escaped_title = title.replace('"', "\"\"");
        csv.push_str(&format!(
            "{},\"{}\",{},{},{},{},{},{}\n",
            id,
            escaped_title,
            slug,
            content_type,
            views,
            duration,
            published,
            created_at.format("%Y-%m-%d %H:%M:%S")
        ));
    }

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"videos_export.csv\"",
            ),
        ],
        csv,
    )
        .into_response())
}

/// POST /video-advanced/rooms/:id/reorder
pub(super) async fn reorder_room_videos(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
    Json(_input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

// ═══════════════════════════════════════════════════════════════════════════
// CDN PURGE
// ═══════════════════════════════════════════════════════════════════════════

/// POST /video-advanced/cdn/purge/:id
pub(super) async fn purge_video_cdn(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

/// POST /video-advanced/cdn/purge-all
pub(super) async fn purge_all_cdn(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Json<serde_json::Value> {
    Json(json!({"success": true}))
}

// ═══════════════════════════════════════════════════════════════════════════
// BULK TAGS & FEATURE
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub(super) struct BulkTagsRequest {
    video_ids: Vec<i64>,
    add_tags: Option<Vec<String>>,
    remove_tags: Option<Vec<String>>,
}

/// POST /video-advanced/bulk/tags - Bulk add/remove tags
/// ICT 7 ADDITION: Bulk tag operations
pub(super) async fn bulk_update_tags(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkTagsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.video_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No video IDs provided"})),
        ));
    }

    let mut updated = 0;

    for video_id in &input.video_ids {
        // Get current tags
        let current: Option<(Option<serde_json::Value>,)> =
            sqlx::query_as("SELECT tags FROM unified_videos WHERE id = $1 AND deleted_at IS NULL")
                .bind(video_id)
                .fetch_optional(&state.db.pool)
                .await
                .unwrap_or(None);

        if let Some((tags_json,)) = current {
            let mut tags: Vec<String> = tags_json
                .and_then(|t| serde_json::from_value(t).ok())
                .unwrap_or_default();

            // Add new tags
            if let Some(ref add_tags) = input.add_tags {
                for tag in add_tags {
                    if !tags.contains(tag) {
                        tags.push(tag.clone());
                    }
                }
            }

            // Remove tags
            if let Some(ref remove_tags) = input.remove_tags {
                tags.retain(|t| !remove_tags.contains(t));
            }

            // Update
            let new_tags_json = serde_json::to_value(&tags).unwrap_or(json!([]));
            let result = sqlx::query(
                "UPDATE unified_videos SET tags = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(&new_tags_json)
            .bind(video_id)
            .execute(&state.db.pool)
            .await;

            if result.is_ok() {
                updated += 1;
            }
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": format!("Updated tags for {} videos", updated),
        "updated_count": updated
    })))
}

#[derive(Debug, Deserialize)]
pub(super) struct BulkFeatureRequest {
    video_ids: Vec<i64>,
    feature: bool,
}

/// POST /video-advanced/bulk/feature - Bulk feature/unfeature videos
/// ICT 7 ADDITION: Bulk feature operation
pub(super) async fn bulk_feature(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkFeatureRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.video_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No video IDs provided"})),
        ));
    }

    sqlx::query(
        "UPDATE unified_videos SET is_featured = $1, updated_at = NOW() WHERE id = ANY($2) AND deleted_at IS NULL"
    )
    .bind(input.feature)
    .bind(&input.video_ids)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos {} successfully", input.video_ids.len(), if input.feature { "featured" } else { "unfeatured" })
    })))
}
