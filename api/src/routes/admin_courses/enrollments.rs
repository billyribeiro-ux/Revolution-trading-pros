//! Enrollment management handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::AppState;

#[derive(serde::Deserialize)]
pub(super) struct EnrollUserRequest {
    user_id: i64,
    is_lifetime_access: Option<bool>,
    access_days: Option<i32>,
    notes: Option<String>,
}

/// Enroll a user in a course (admin action)
pub(super) async fn enroll_user(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<EnrollUserRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Check if already enrolled
    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(input.user_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "User is already enrolled in this course"})),
        ));
    }

    // Calculate access expiry if not lifetime
    let access_expires = if input.is_lifetime_access.unwrap_or(true) {
        None
    } else {
        input
            .access_days
            .map(|days| chrono::Utc::now() + chrono::Duration::days(days as i64))
    };

    let enrollment = sqlx::query_as::<_, (i64, Uuid, i64, Option<i32>, String, NaiveDateTime)>(
        r"
        INSERT INTO user_course_enrollments (
            user_id, course_id, status, progress_percent, is_lifetime_access,
            access_expires_at, enrolled_at
        ) VALUES ($1, $2, 'active', 0, $3, $4, NOW())
        RETURNING id, course_id, user_id, progress_percent, status, enrolled_at
        ",
    )
    .bind(input.user_id)
    .bind(course_id)
    .bind(input.is_lifetime_access.unwrap_or(true))
    .bind(access_expires)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to enroll user: {}", e)})),
        )
    })?;

    // Update course enrollment count
    sqlx::query(
        "UPDATE courses SET enrollment_count = COALESCE(enrollment_count, 0) + 1 WHERE id = $1",
    )
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({
        "success": true,
        "message": "User enrolled successfully",
        "data": {
            "id": enrollment.0,
            "course_id": enrollment.1,
            "user_id": enrollment.2,
            "progress_percent": enrollment.3.unwrap_or(0),
            "status": enrollment.4,
            "enrolled_at": enrollment.5.format("%Y-%m-%dT%H:%M:%S").to_string()
        }
    })))
}

/// List all enrollments for a course
pub(super) async fn list_enrollments(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let enrollments: Vec<serde_json::Value> = sqlx::query_as::<
        _,
        (
            i64,
            i64,
            Option<i32>,
            String,
            NaiveDateTime,
            Option<NaiveDateTime>,
            Option<bool>,
        ),
    >(
        r"
        SELECT e.id, e.user_id, e.progress_percent, e.status, e.enrolled_at,
               e.completed_at, e.certificate_issued
        FROM user_course_enrollments e
        WHERE e.course_id = $1
        ORDER BY e.enrolled_at DESC
        ",
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|e| {
        json!({
            "id": e.0,
            "user_id": e.1,
            "progress_percent": e.2.unwrap_or(0),
            "status": e.3,
            "enrolled_at": e.4.format("%Y-%m-%dT%H:%M:%S").to_string(),
            "completed_at": e.5.map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
            "certificate_issued": e.6.unwrap_or(false)
        })
    })
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": enrollments
    })))
}

/// Remove a user's enrollment from a course
pub(super) async fn remove_enrollment(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, enrollment_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM user_course_enrollments WHERE id = $1 AND course_id = $2")
        .bind(enrollment_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to remove enrollment: {}", e)})),
            )
        })?;

    // Update course enrollment count
    sqlx::query("UPDATE courses SET enrollment_count = GREATEST(COALESCE(enrollment_count, 1) - 1, 0) WHERE id = $1")
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(json!({
        "success": true,
        "message": "Enrollment removed successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT STATS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get course enrollment statistics
pub(super) async fn get_enrollment_stats(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_course_enrollments WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let active: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_course_enrollments WHERE course_id = $1 AND status = 'active'",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let completed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_course_enrollments WHERE course_id = $1 AND completed_at IS NOT NULL")
        .bind(course_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let avg_progress: (Option<f64>,) = sqlx::query_as(
        "SELECT AVG(progress_percent::float) FROM user_course_enrollments WHERE course_id = $1",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let certificates: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_course_enrollments WHERE course_id = $1 AND certificate_issued = true")
        .bind(course_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": {
            "course_id": course_id,
            "total_enrollments": total.0,
            "active_enrollments": active.0,
            "completed_enrollments": completed.0,
            "avg_progress": avg_progress.0.unwrap_or(0.0),
            "certificates_issued": certificates.0
        }
    })))
}
