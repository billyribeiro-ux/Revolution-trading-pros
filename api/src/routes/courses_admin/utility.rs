//! Course utility endpoints — categories, stats, clone
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
// UTILITY ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get course categories
pub(super) async fn get_categories(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let categories: Vec<(String, i64)> = sqlx::query_as(
        "SELECT category, COUNT(*) FROM courses_enhanced WHERE category IS NOT NULL AND deleted_at IS NULL GROUP BY category ORDER BY category"
    )
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let cat_list: Vec<serde_json::Value> = categories
        .iter()
        .map(|(cat, count)| json!({"name": cat, "count": count}))
        .collect();

    Ok(Json(json!({"categories": cat_list})))
}

/// Get course stats
pub(super) async fn get_course_stats(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_courses: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM courses_enhanced WHERE deleted_at IS NULL")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let published_courses: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM courses_enhanced WHERE is_published = true AND deleted_at IS NULL",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let total_lessons: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM course_lessons")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let total_enrollments: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_course_enrollments WHERE is_active = true")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let completed_enrollments: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_course_enrollments WHERE is_completed = true")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "total_courses": total_courses.0,
        "published_courses": published_courses.0,
        "total_lessons": total_lessons.0,
        "total_enrollments": total_enrollments.0,
        "completed_enrollments": completed_enrollments.0,
        "completion_rate": if total_enrollments.0 > 0 {
            (completed_enrollments.0 as f64 / total_enrollments.0 as f64 * 100.0).round()
        } else {
            0.0
        }
    })))
}

/// Clone a course
pub(super) async fn clone_course(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "course_clone", admin_id = %admin.id, course_id = %course_id, "Admin cloning course");
    // Get original course
    let original: CourseEnhanced =
        sqlx::query_as("SELECT * FROM courses_enhanced WHERE id = $1 AND deleted_at IS NULL")
            .bind(course_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Course not found"})),
                )
            })?;

    let new_title = format!("{} (Copy)", original.title);
    let new_slug = format!("{}-copy-{}", original.slug, chrono::Utc::now().timestamp());

    // Create new course
    let new_course: CourseEnhanced = sqlx::query_as(
        r"INSERT INTO courses_enhanced
           (title, slug, subtitle, description, description_html, thumbnail_url,
            trailer_video_url, difficulty_level, category, tags, instructor_id,
            is_published, is_free, required_plan_id, price_cents, certificate_enabled,
            completion_threshold_percent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, false, $12, $13, $14, $15, $16)
           RETURNING *",
    )
    .bind(&new_title)
    .bind(&new_slug)
    .bind(&original.subtitle)
    .bind(&original.description)
    .bind(&original.description_html)
    .bind(&original.thumbnail_url)
    .bind(&original.trailer_video_url)
    .bind(&original.difficulty_level)
    .bind(&original.category)
    .bind(&original.tags)
    .bind(original.instructor_id)
    .bind(original.is_free)
    .bind(original.required_plan_id)
    .bind(original.price_cents)
    .bind(original.certificate_enabled)
    .bind(original.completion_threshold_percent)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to clone course: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Clone sections
    let sections: Vec<CourseSection> =
        sqlx::query_as("SELECT * FROM course_sections WHERE course_id = $1 ORDER BY sort_order")
            .bind(course_id)
            .fetch_all(&state.db.pool)
            .await
            // FIX-2026-04-26: .unwrap_or_default();
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let mut section_map: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    for section in &sections {
        let new_section: (i64,) = sqlx::query_as(
            r"INSERT INTO course_sections
               (course_id, title, description, sort_order, section_type, unlock_type, is_published)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id",
        )
        .bind(new_course.id)
        .bind(&section.title)
        .bind(&section.description)
        .bind(section.sort_order)
        .bind(&section.section_type)
        .bind(&section.unlock_type)
        .bind(section.is_published)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        section_map.insert(section.id, new_section.0);
    }

    // Clone lessons
    let lessons: Vec<CourseLesson> =
        sqlx::query_as("SELECT * FROM course_lessons WHERE course_id = $1")
            .bind(course_id)
            .fetch_all(&state.db.pool)
            .await
            // FIX-2026-04-26: .unwrap_or_default();
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    for lesson in &lessons {
        if let Some(&new_section_id) = section_map.get(&lesson.section_id) {
            sqlx::query(
                r"INSERT INTO course_lessons
                   (course_id, section_id, title, description, content_html, video_url,
                    bunny_video_guid, bunny_library_id, duration_seconds, thumbnail_url,
                    sort_order, lesson_type, is_preview, is_published, completion_type, required_watch_percent)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"
            )
            .bind(new_course.id)
            .bind(new_section_id)
            .bind(&lesson.title)
            .bind(&lesson.description)
            .bind(&lesson.content_html)
            .bind(&lesson.video_url)
            .bind(&lesson.bunny_video_guid)
            .bind(lesson.bunny_library_id)
            .bind(lesson.duration_seconds)
            .bind(&lesson.thumbnail_url)
            .bind(lesson.sort_order)
            .bind(&lesson.lesson_type)
            .bind(lesson.is_preview)
            .bind(lesson.is_published)
            .bind(&lesson.completion_type)
            .bind(lesson.required_watch_percent)
            .execute(&state.db.pool)
            .await
            .ok();
        }
    }

    info!("Cloned course {} to {}", course_id, new_course.id);

    Ok(Json(json!({
        "success": true,
        "new_course": {
            "id": new_course.id,
            "title": new_course.title,
            "slug": new_course.slug
        }
    })))
}
