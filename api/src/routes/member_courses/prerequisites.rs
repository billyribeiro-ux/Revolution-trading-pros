//! Lesson prerequisite gate.
//!
//! Determines whether a given lesson may be opened by an enrolled member.
//! Free / preview lessons short-circuit to "accessible"; otherwise every
//! `prerequisite_lesson_ids` entry must have a corresponding completed
//! `user_lesson_progress` row.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::User;
use crate::AppState;

/// Check if user can access a lesson (prerequisites satisfied)
pub(super) async fn check_lesson_access(
    user: User,
    State(state): State<AppState>,
    Path((slug, lesson_id)): Path<(String, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    let course: (Uuid,) = sqlx::query_as("SELECT id FROM courses WHERE slug = $1")
        .bind(&slug)
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
                Json(json!({"error": "Course not found"})),
            )
        })?;

    // Get enrollment
    let enrollment: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    if enrollment.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        ));
    }

    // Get lesson prerequisites
    let lesson: (Uuid, Option<serde_json::Value>, Option<bool>, Option<bool>) = sqlx::query_as(
        "SELECT id, prerequisite_lesson_ids, is_free, is_preview FROM lessons WHERE id = $1 AND course_id = $2"
    )
    .bind(lesson_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Database error: {}", e)}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Lesson not found"}))))?;

    // Free or preview lessons always accessible
    if lesson.2.unwrap_or(false) || lesson.3.unwrap_or(false) {
        return Ok(Json(
            json!({"success": true, "data": {"can_access": true, "reason": "Free/preview lesson"}}),
        ));
    }

    // Check prerequisites
    let prereq_ids = lesson
        .1
        .clone()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();
    if prereq_ids.is_empty() {
        return Ok(Json(
            json!({"success": true, "data": {"can_access": true, "reason": "No prerequisites"}}),
        ));
    }

    let mut missing_prereqs = Vec::new();
    for prereq_id in &prereq_ids {
        if let Some(pid) = prereq_id.as_str() {
            if let Ok(prereq_uuid) = Uuid::parse_str(pid) {
                // Check if completed
                let completed: Option<(bool,)> = sqlx::query_as(
                    "SELECT is_completed FROM user_lesson_progress WHERE user_id = $1 AND lesson_id = $2"
                )
                .bind(user_id)
                .bind(prereq_uuid)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

                if !completed.map(|c| c.0).unwrap_or(false) {
                    // Get lesson title
                    let prereq_title: Option<(String,)> =
                        sqlx::query_as("SELECT title FROM lessons WHERE id = $1")
                            .bind(prereq_uuid)
                            .fetch_optional(&state.db.pool)
                            .await
                            .ok()
                            .flatten();
                    missing_prereqs.push(json!({"lesson_id": pid, "title": prereq_title.map(|t| t.0).unwrap_or_default()}));
                }
            }
        }
    }

    if missing_prereqs.is_empty() {
        Ok(Json(
            json!({"success": true, "data": {"can_access": true, "reason": "Prerequisites completed"}}),
        ))
    } else {
        Ok(Json(json!({
            "success": true,
            "data": {
                "can_access": false,
                "reason": "Missing prerequisites",
                "missing_prerequisites": missing_prereqs
            }
        })))
    }
}
