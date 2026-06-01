//! Certificate generation + resume helpers.
//!
//! Two completion-adjacent endpoints: the certificate issuance flow (gated
//! at 100% progress) and the "resume where you left off" lookup that
//! powers the continue-watching CTA.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::User;
use crate::AppState;

pub(super) async fn get_certificate(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Grade: User extracted from JWT auth middleware
    let user_id = user.id;

    // Get course info
    let course: (Uuid, String, Option<String>) =
        sqlx::query_as("SELECT id, title, instructor_name FROM courses WHERE slug = $1")
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

    // Check enrollment and completion
    #[allow(clippy::type_complexity)]
    let enrollment: Option<(i64, Option<i32>, Option<bool>, Option<String>)> = sqlx::query_as(
        r"
        SELECT id, progress_percent, certificate_issued, certificate_url
        FROM user_course_enrollments
        WHERE user_id = $1 AND course_id = $2
        ",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let enrollment = enrollment.ok_or_else(|| {
        (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        )
    })?;

    let progress = enrollment.1.unwrap_or(0);
    if progress < 100 {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Course not completed",
                "progress_percent": progress,
                "message": "Complete all lessons to receive your certificate"
            })),
        ));
    }

    // If certificate already exists, return it
    if enrollment.2.unwrap_or(false) {
        if let Some(cert_url) = enrollment.3 {
            return Ok(Json(json!({
                "success": true,
                "data": {
                    "certificate_url": cert_url,
                    "already_issued": true
                }
            })));
        }
    }

    // Get user info for certificate (name and email)
    let user_info: (String, String) =
        sqlx::query_as("SELECT COALESCE(name, username), email FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to get user info: {}", e)})),
                )
            })?;

    // Generate certificate ID
    let cert_id = format!(
        "CERT-{}-{}-{}",
        course
            .0
            .simple()
            .to_string()
            .chars()
            .take(8)
            .collect::<String>(),
        user_id,
        chrono::Utc::now().timestamp()
    );

    // In production, this would generate a PDF using a template
    // For now, we store certificate data and return a verification URL
    let certificate_url =
        format!("https://revolution-trading-pros.com/certificates/verify/{cert_id}");

    // Update enrollment with certificate info
    sqlx::query(
        r"
        UPDATE user_course_enrollments
        SET certificate_issued = true,
            certificate_url = $1,
            certificate_issued_at = NOW(),
            completed_at = COALESCE(completed_at, NOW())
        WHERE id = $2
        ",
    )
    .bind(&certificate_url)
    .bind(enrollment.0)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to issue certificate: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Certificate issued successfully",
        "data": {
            "certificate_id": cert_id,
            "certificate_url": certificate_url,
            "course_title": course.1,
            "instructor_name": course.2,
            "student_name": user_info.0,
            "issued_at": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            "verification_url": certificate_url
        }
    })))
}

/// Resume course - returns the last viewed lesson for continue watching
pub(super) async fn resume_course(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Grade: User extracted from JWT auth middleware
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

    // Get enrollment with current lesson
    let enrollment: Option<(i64, Option<Uuid>, Option<i32>)> = sqlx::query_as(
        r"
        SELECT id, current_lesson_id, progress_percent
        FROM user_course_enrollments
        WHERE user_id = $1 AND course_id = $2
        ",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let enrollment = enrollment.ok_or_else(|| {
        (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        )
    })?;

    // Get last accessed lesson with progress
    #[allow(clippy::type_complexity)]
    let resume_lesson: Option<(Uuid, String, String, Option<i32>, Option<i32>)> = if let Some(
        lesson_id,
    ) =
        enrollment.1
    {
        sqlx::query_as(
            r"
            SELECT l.id, l.title, l.slug, p.video_position_seconds, l.duration_minutes
            FROM lessons l
            LEFT JOIN user_lesson_progress p ON p.lesson_id = l.id AND p.user_id = $1
            WHERE l.id = $2
            ",
        )
        .bind(user_id)
        .bind(lesson_id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten()
    } else {
        // Get first uncompleted lesson
        sqlx::query_as(
            r"
            SELECT l.id, l.title, l.slug, p.video_position_seconds, l.duration_minutes
            FROM lessons l
            LEFT JOIN user_lesson_progress p ON p.lesson_id = l.id AND p.user_id = $1 AND p.enrollment_id = $2
            WHERE l.course_id = $3 AND (p.is_completed IS NULL OR p.is_completed = false)
            ORDER BY COALESCE(l.sort_order, l.position)
            LIMIT 1
            ",
        )
        .bind(user_id)
        .bind(enrollment.0)
        .bind(course.0)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten()
    };

    match resume_lesson {
        Some(lesson) => Ok(Json(json!({
            "success": true,
            "data": {
                "lesson_id": lesson.0,
                "lesson_title": lesson.1,
                "lesson_slug": lesson.2,
                "video_position_seconds": lesson.3.unwrap_or(0),
                "duration_minutes": lesson.4,
                "course_progress_percent": enrollment.2.unwrap_or(0),
                "resume_url": format!("/classes/{}/lesson/{}", slug, lesson.2)
            }
        }))),
        None => Ok(Json(json!({
            "success": true,
            "data": {
                "course_completed": true,
                "course_progress_percent": 100,
                "message": "Course completed! View your certificate."
            }
        }))),
    }
}
