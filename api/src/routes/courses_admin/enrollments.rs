//! Course enrollment & per-lesson progress endpoints
//! Extracted from `courses_admin.rs` in R14-B split (2026-05-20).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde_json::json;
use tracing::{error, info};

use crate::middleware::admin::AdminUser;
use crate::models::course_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT & PROGRESS ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Enroll a user in a course
pub(super) async fn enroll_user(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = input
        .get("user_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "user_id required"})),
            )
        })?;

    info!(target: "security", event = "course_enroll", admin_id = %admin.id, course_id = %course_id, target_user_id = %user_id, "Admin enrolling user in course");

    let enrollment_source = input
        .get("enrollment_source")
        .and_then(|v| v.as_str())
        .unwrap_or("admin");

    // Get total lessons count
    let total_lessons: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM course_lessons WHERE course_id = $1 AND is_published = true",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let enrollment: UserCourseEnrollment = sqlx::query_as(
        r"INSERT INTO user_course_enrollments
           (user_id, course_id, enrollment_source, total_lessons)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (user_id, course_id) DO UPDATE SET is_active = true
           RETURNING *",
    )
    .bind(user_id)
    .bind(course_id)
    .bind(enrollment_source)
    .bind(total_lessons.0 as i32)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to enroll user: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "enrollment": {
            "id": enrollment.id,
            "user_id": enrollment.user_id,
            "course_id": enrollment.course_id,
            "enrolled_at": enrollment.enrolled_at.to_rfc3339()
        }
    })))
}

/// Get course enrollments
pub(super) async fn get_enrollments(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path(course_id): Path<i64>,
    Query(query): Query<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query
        .get("page")
        .and_then(|v| v.as_i64())
        .unwrap_or(1)
        .max(1);
    let per_page = query
        .get("per_page")
        .and_then(|v| v.as_i64())
        .unwrap_or(50)
        .min(100);
    let offset = (page - 1) * per_page;

    let enrollments: Vec<UserCourseEnrollment> = sqlx::query_as(
        r"SELECT * FROM user_course_enrollments
           WHERE course_id = $1 AND is_active = true
           ORDER BY enrolled_at DESC
           LIMIT $2 OFFSET $3",
    )
    .bind(course_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_course_enrollments WHERE course_id = $1 AND is_active = true",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let enrollment_responses: Vec<serde_json::Value> = enrollments
        .iter()
        .map(|e| {
            json!({
                "id": e.id,
                "user_id": e.user_id,
                "enrolled_at": e.enrolled_at.to_rfc3339(),
                "completed_lessons": e.completed_lessons,
                "total_lessons": e.total_lessons,
                "progress_percent": e.progress_percent,
                "is_completed": e.is_completed,
                "completed_at": e.completed_at.map(|d| d.to_rfc3339()),
                "certificate_issued": e.certificate_issued,
                "last_activity_at": e.last_activity_at.to_rfc3339()
            })
        })
        .collect();

    Ok(Json(json!({
        "enrollments": enrollment_responses,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}

/// Update lesson progress
pub(super) async fn update_lesson_progress(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path((course_id, lesson_id)): Path<(i64, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = input
        .get("user_id")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "user_id required"})),
            )
        })?;

    let watch_position = input
        .get("watch_position_seconds")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let watch_percent = input
        .get("watch_percent")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    // Get lesson's required watch percent
    let lesson: Option<(i32,)> =
        sqlx::query_as("SELECT required_watch_percent FROM course_lessons WHERE id = $1")
            .bind(lesson_id)
            .fetch_optional(&state.db.pool)
            .await
            .unwrap_or(None);

    let required_percent = lesson.map(|l| l.0).unwrap_or(80);
    let is_completed = watch_percent >= required_percent;

    sqlx::query(
        r"INSERT INTO user_lesson_progress
           (user_id, course_id, lesson_id, watch_position_seconds, watch_percent, is_completed, completed_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (user_id, lesson_id) DO UPDATE SET
             watch_position_seconds = GREATEST(user_lesson_progress.watch_position_seconds, $4),
             watch_time_total_seconds = user_lesson_progress.watch_time_total_seconds + $4,
             watch_percent = GREATEST(user_lesson_progress.watch_percent, $5),
             is_completed = user_lesson_progress.is_completed OR $6,
             completed_at = CASE WHEN NOT user_lesson_progress.is_completed AND $6 THEN NOW() ELSE user_lesson_progress.completed_at END,
             last_watched_at = NOW()"
    )
    .bind(user_id)
    .bind(course_id)
    .bind(lesson_id)
    .bind(watch_position)
    .bind(watch_percent)
    .bind(is_completed)
    .bind(if is_completed { Some(Utc::now()) } else { None })
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update progress: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Update enrollment last activity
    sqlx::query(
        "UPDATE user_course_enrollments SET last_lesson_id = $1, last_position_seconds = $2, last_activity_at = NOW() WHERE user_id = $3 AND course_id = $4"
    )
    .bind(lesson_id)
    .bind(watch_position)
    .bind(user_id)
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({
        "success": true,
        "is_completed": is_completed
    })))
}

/// Get user's course progress
pub(super) async fn get_user_progress(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path((course_id, user_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let enrollment: Option<UserCourseEnrollment> = sqlx::query_as(
        "SELECT * FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let enrollment = enrollment.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Enrollment not found"})),
        )
    })?;

    let lesson_progress: Vec<UserLessonProgress> =
        sqlx::query_as("SELECT * FROM user_lesson_progress WHERE user_id = $1 AND course_id = $2")
            .bind(user_id)
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

    let progress_map: Vec<serde_json::Value> = lesson_progress
        .iter()
        .map(|p| {
            json!({
                "lesson_id": p.lesson_id,
                "watch_position_seconds": p.watch_position_seconds,
                "watch_percent": p.watch_percent,
                "is_completed": p.is_completed,
                "completed_at": p.completed_at.map(|d| d.to_rfc3339())
            })
        })
        .collect();

    Ok(Json(json!({
        "enrollment": {
            "enrolled_at": enrollment.enrolled_at.to_rfc3339(),
            "completed_lessons": enrollment.completed_lessons,
            "total_lessons": enrollment.total_lessons,
            "progress_percent": enrollment.progress_percent,
            "is_completed": enrollment.is_completed,
            "completed_at": enrollment.completed_at.map(|d| d.to_rfc3339()),
            "certificate_url": enrollment.certificate_url,
            "last_lesson_id": enrollment.last_lesson_id,
            "last_position_seconds": enrollment.last_position_seconds
        },
        "lesson_progress": progress_map
    })))
}
