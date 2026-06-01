//! Module + lesson + prerequisite handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::models::course::{
    CourseDownload, CourseModule, CreateLessonRequest, CreateModuleRequest, Lesson, LessonListItem,
    ReorderRequest, UpdateLessonRequest, UpdateModuleRequest,
};
use crate::routes::admin_courses::slugify;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// MODULE CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn list_modules(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let modules: Vec<CourseModule> = sqlx::query_as(
        r"
        SELECT * FROM course_modules_v2
        WHERE course_id = $1
        ORDER BY sort_order
        ",
    )
    .bind(course_id)
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
        "data": modules
    })))
}

pub(super) async fn create_module(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<CreateModuleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_modules_v2 WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = input.sort_order.unwrap_or(max_order.0.unwrap_or(0) + 1);

    let module: CourseModule = sqlx::query_as(
        r"
        INSERT INTO course_modules_v2 (course_id, title, description, sort_order, is_published, drip_enabled, drip_days)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        ",
    )
    .bind(course_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(sort_order)
    .bind(input.is_published.unwrap_or(true))
    .bind(input.drip_enabled.unwrap_or(false))
    .bind(input.drip_days.unwrap_or(0))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create module: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Module created successfully",
        "data": module
    })))
}

pub(super) async fn update_module(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, module_id)): Path<(Uuid, i64)>,
    Json(input): Json<UpdateModuleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let module: CourseModule = sqlx::query_as(
        r"
        UPDATE course_modules_v2 SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            sort_order = COALESCE($3, sort_order),
            is_published = COALESCE($4, is_published),
            drip_enabled = COALESCE($5, drip_enabled),
            drip_days = COALESCE($6, drip_days),
            updated_at = NOW()
        WHERE id = $7 AND course_id = $8
        RETURNING *
        ",
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.sort_order)
    .bind(input.is_published)
    .bind(input.drip_enabled)
    .bind(input.drip_days)
    .bind(module_id)
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update module: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Module updated successfully",
        "data": module
    })))
}

pub(super) async fn delete_module(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, module_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE lessons SET module_id = NULL WHERE module_id = $1")
        .bind(module_id)
        .execute(&state.db.pool)
        .await
        .ok();

    sqlx::query("DELETE FROM course_modules_v2 WHERE id = $1 AND course_id = $2")
        .bind(module_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete module: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Module deleted successfully"
    })))
}

pub(super) async fn reorder_modules(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<ReorderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in input.items {
        let module_id: i64 = item.id.parse().unwrap_or(0);
        sqlx::query(
            "UPDATE course_modules_v2 SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3",
        )
        .bind(item.sort_order)
        .bind(module_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({
        "success": true,
        "message": "Modules reordered successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LESSON CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn list_lessons(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lessons: Vec<LessonListItem> = sqlx::query_as(
        r"
        SELECT id, course_id, module_id, title, slug, description, duration_minutes,
               position, sort_order, is_free, is_preview, is_published,
               bunny_video_guid, thumbnail_url
        FROM lessons
        WHERE course_id = $1
        ORDER BY COALESCE(sort_order, position)
        ",
    )
    .bind(course_id)
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
        "data": lessons
    })))
}

pub(super) async fn get_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lesson: Lesson = sqlx::query_as(
        r"
        SELECT * FROM lessons WHERE id = $1 AND course_id = $2
        ",
    )
    .bind(lesson_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Lesson not found"})),
        )
    })?;

    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r"
        SELECT * FROM course_downloads WHERE lesson_id = $1 ORDER BY sort_order
        ",
    )
    .bind(lesson_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "lesson": lesson,
            "downloads": downloads
        }
    })))
}

pub(super) async fn create_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<CreateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.title));

    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(COALESCE(sort_order, position)) FROM lessons WHERE course_id = $1",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = input.sort_order.unwrap_or(max_order.0.unwrap_or(0) + 1);

    let lesson: Lesson = sqlx::query_as(
        r"
        INSERT INTO lessons (
            id, course_id, module_id, title, slug, description, video_url,
            bunny_video_guid, thumbnail_url, duration_minutes, content_html,
            is_free, is_preview, is_published, sort_order, position, drip_days
        ) VALUES (
            gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $14, $15
        )
        RETURNING *
        ",
    )
    .bind(course_id)
    .bind(input.module_id)
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.duration_minutes.unwrap_or(0))
    .bind(&input.content_html)
    .bind(input.is_free.unwrap_or(false))
    .bind(input.is_preview.unwrap_or(false))
    .bind(input.is_published.unwrap_or(true))
    .bind(sort_order)
    .bind(input.drip_days.unwrap_or(0))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create lesson: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Lesson created successfully",
        "data": lesson
    })))
}

pub(super) async fn update_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UpdateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lesson: Lesson = sqlx::query_as(
        r"
        UPDATE lessons SET
            title = COALESCE($1, title),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            module_id = COALESCE($4, module_id),
            video_url = COALESCE($5, video_url),
            bunny_video_guid = COALESCE($6, bunny_video_guid),
            thumbnail_url = COALESCE($7, thumbnail_url),
            duration_minutes = COALESCE($8, duration_minutes),
            content_html = COALESCE($9, content_html),
            is_free = COALESCE($10, is_free),
            is_preview = COALESCE($11, is_preview),
            is_published = COALESCE($12, is_published),
            sort_order = COALESCE($13, sort_order),
            drip_days = COALESCE($14, drip_days),
            updated_at = NOW()
        WHERE id = $15 AND course_id = $16
        RETURNING *
        ",
    )
    .bind(&input.title)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(input.module_id)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.duration_minutes)
    .bind(&input.content_html)
    .bind(input.is_free)
    .bind(input.is_preview)
    .bind(input.is_published)
    .bind(input.sort_order)
    .bind(input.drip_days)
    .bind(lesson_id)
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update lesson: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Lesson updated successfully",
        "data": lesson
    })))
}

pub(super) async fn delete_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM lessons WHERE id = $1 AND course_id = $2")
        .bind(lesson_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete lesson: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Lesson deleted successfully"
    })))
}

pub(super) async fn reorder_lessons(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<ReorderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in input.items {
        let lesson_id = Uuid::parse_str(&item.id).ok();
        if let Some(lid) = lesson_id {
            sqlx::query(
                "UPDATE lessons SET sort_order = $1, position = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3",
            )
            .bind(item.sort_order)
            .bind(lid)
            .bind(course_id)
            .execute(&state.db.pool)
            .await
            .ok();
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "Lessons reordered successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PREREQUISITES - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// Update lesson prerequisites
pub(super) async fn update_lesson_prerequisites(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let prereq_ids: Vec<String> = input["prerequisite_lesson_ids"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    sqlx::query("UPDATE lessons SET prerequisite_lesson_ids = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3")
        .bind(json!(prereq_ids))
        .bind(lesson_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to update prerequisites: {}", e)}))))?;

    Ok(Json(
        json!({"success": true, "message": "Prerequisites updated"}),
    ))
}

/// Get lesson prerequisites
pub(super) async fn get_lesson_prerequisites(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let prereqs: Option<(Option<serde_json::Value>,)> = sqlx::query_as(
        "SELECT prerequisite_lesson_ids FROM lessons WHERE id = $1 AND course_id = $2",
    )
    .bind(lesson_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let prerequisite_ids = prereqs.and_then(|p| p.0).unwrap_or(json!([]));

    Ok(Json(
        json!({"success": true, "data": {"lesson_id": lesson_id, "prerequisite_lesson_ids": prerequisite_ids}}),
    ))
}
