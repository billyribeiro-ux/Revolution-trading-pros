//! Course lesson endpoints
//! Extracted from `courses_admin.rs` in R14-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use tracing::{error, info};

use crate::middleware::admin::AdminUser;
use crate::models::course_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// LESSON ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a lesson
pub(super) async fn create_lesson(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "lesson_create", admin_id = %admin.id, course_id = %course_id, "Admin creating lesson");
    // Get max sort order for section
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_lessons WHERE section_id = $1")
            .bind(input.section_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let lesson: CourseLesson = sqlx::query_as(
        r"INSERT INTO course_lessons
           (course_id, section_id, title, description, content_html, video_url,
            bunny_video_guid, thumbnail_url, sort_order, lesson_type, is_preview,
            is_published, completion_type, required_watch_percent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
           RETURNING *",
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.content_html)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(sort_order)
    .bind(input.lesson_type.as_deref().unwrap_or("video"))
    .bind(input.is_preview.unwrap_or(false))
    .bind(input.is_published.unwrap_or(true))
    .bind(input.completion_type.as_deref().unwrap_or("watch"))
    .bind(input.required_watch_percent.unwrap_or(80))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create lesson: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!("Created lesson: {} in course {}", lesson.title, course_id);

    Ok(Json(json!({
        "success": true,
        "lesson": {
            "id": lesson.id,
            "title": lesson.title,
            "section_id": lesson.section_id,
            "sort_order": lesson.sort_order
        }
    })))
}

/// Update a lesson
/// ICT 7+ SECURITY: Uses parameterized queries to prevent SQL injection
pub(super) async fn update_lesson(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, lesson_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "lesson_update", admin_id = %admin.id, course_id = %course_id, lesson_id = %lesson_id, "Admin updating lesson");

    // ICT 7+ SECURITY FIX: Parameterized query
    sqlx::query(
        r"UPDATE course_lessons SET
            section_id = COALESCE($3, section_id),
            title = COALESCE($4, title),
            description = COALESCE($5, description),
            content_html = COALESCE($6, content_html),
            video_url = COALESCE($7, video_url),
            bunny_video_guid = COALESCE($8, bunny_video_guid),
            thumbnail_url = COALESCE($9, thumbnail_url),
            duration_seconds = COALESCE($10, duration_seconds),
            lesson_type = COALESCE($11, lesson_type),
            is_preview = COALESCE($12, is_preview),
            is_published = COALESCE($13, is_published),
            completion_type = COALESCE($14, completion_type),
            required_watch_percent = COALESCE($15, required_watch_percent),
            updated_at = NOW()
        WHERE id = $1 AND course_id = $2",
    )
    .bind(lesson_id)
    .bind(course_id)
    .bind(input.section_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.content_html)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.duration_seconds)
    .bind(&input.lesson_type)
    .bind(input.is_preview)
    .bind(input.is_published)
    .bind(&input.completion_type)
    .bind(input.required_watch_percent)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update lesson: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({"success": true})))
}

/// Delete a lesson
pub(super) async fn delete_lesson(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, lesson_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "lesson_delete", admin_id = %admin.id, course_id = %course_id, lesson_id = %lesson_id, "Admin deleting lesson");
    // Delete associated resources first
    sqlx::query("DELETE FROM course_resources WHERE lesson_id = $1")
        .bind(lesson_id)
        .execute(&state.db.pool)
        .await
        .ok();

    let result = sqlx::query("DELETE FROM course_lessons WHERE id = $1 AND course_id = $2")
        .bind(lesson_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete lesson: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Lesson not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}

/// Reorder lessons within a section
pub(super) async fn reorder_lessons(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path((course_id, section_id)): Path<(i64, i64)>,
    Json(input): Json<ReorderItemsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.items {
        sqlx::query(
            "UPDATE course_lessons SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3 AND section_id = $4"
        )
        .bind(item.sort_order)
        .bind(item.id)
        .bind(course_id)
        .bind(section_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true})))
}
