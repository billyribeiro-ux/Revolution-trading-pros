//! Authenticated member endpoints: course player, progress, downloads.
//!
//! These handlers serve the in-progress viewing experience for an enrolled
//! member: the list of "my courses", the per-course player payload (with
//! the P0-7 paywall gate), per-lesson progress upserts, and the downloads
//! tray.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use super::entitlement::{is_course_admin, is_lesson_entitled};
use crate::models::course::{
    Course, CourseDownload, CourseListItem, CourseModule, LessonListItem, UpdateProgressRequest,
    UserCourseEnrollment, UserLessonProgress,
};
use crate::models::User;
use crate::AppState;

pub(super) async fn get_my_courses(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Grade: User extracted from JWT auth middleware
    let user_id = user.id;

    let enrollments: Vec<UserCourseEnrollment> = sqlx::query_as(
        r"
        SELECT * FROM user_course_enrollments
        WHERE user_id = $1
        ORDER BY last_accessed_at DESC NULLS LAST
        ",
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let mut result = Vec::new();
    for enrollment in enrollments {
        let course: Option<CourseListItem> = sqlx::query_as(
            r"
            SELECT id, title, slug, description, card_image_url, card_description,
                   card_badge, card_badge_color, instructor_name, instructor_avatar_url,
                   level, price_cents, is_free, is_published, status, module_count,
                   lesson_count, total_duration_minutes, enrollment_count, avg_rating,
                   review_count, created_at
            FROM courses WHERE id = $1
            ",
        )
        .bind(enrollment.course_id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

        if let Some(c) = course {
            result.push(json!({
                "id": enrollment.id,
                "course_id": enrollment.course_id,
                "progress_percent": enrollment.progress_percent.unwrap_or(0),
                "status": enrollment.status.unwrap_or_else(|| "active".to_string()),
                "enrolled_at": enrollment.enrolled_at,
                "last_accessed_at": enrollment.last_accessed_at,
                "current_lesson_id": enrollment.current_lesson_id,
                "course": c
            }));
        }
    }

    Ok(Json(json!({
        "success": true,
        "data": result
    })))
}

pub(super) async fn get_course_player(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Grade: User extracted from JWT auth middleware
    let user_id = user.id;

    let course: Course = sqlx::query_as("SELECT * FROM courses WHERE slug = $1")
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

    let enrollment: Option<UserCourseEnrollment> = sqlx::query_as(
        "SELECT * FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    // ═══════════════════════════════════════════════════════════════════════
    // PAYWALL GATE (P0-7) — ICT 7 Grade
    //
    // A paid course's player payload (Bunny video GUIDs + downloads) must
    // NOT be served to a caller who is neither enrolled nor an admin. Prior
    // to this fix any authenticated free account could pull every
    // `bunny_video_guid` and download link for a paid course.
    // ═══════════════════════════════════════════════════════════════════════
    let course_is_free = course.is_free.unwrap_or(false);
    let is_enrolled = enrollment.is_some();
    let is_admin = is_course_admin(&user);

    if !course_is_free && !is_enrolled && !is_admin {
        tracing::warn!(
            target: "security",
            event = "paywall_bypass_blocked",
            user_id = %user_id,
            course_id = %course.id,
            course_slug = %slug,
            "Non-entitled user blocked from paid course player"
        );
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        ));
    }

    let modules: Vec<CourseModule> =
        sqlx::query_as("SELECT * FROM course_modules_v2 WHERE course_id = $1 ORDER BY sort_order")
            .bind(course.id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    let lessons: Vec<LessonListItem> = sqlx::query_as(
        r"
        SELECT id, course_id, module_id, title, slug, description, duration_minutes,
               position, sort_order, is_free, is_preview, is_published,
               bunny_video_guid, thumbnail_url
        FROM lessons WHERE course_id = $1
        ORDER BY COALESCE(sort_order, position)
        ",
    )
    .bind(course.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let downloads: Vec<CourseDownload> =
        sqlx::query_as("SELECT * FROM course_downloads WHERE course_id = $1 ORDER BY sort_order")
            .bind(course.id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    let progress: Vec<UserLessonProgress> = if let Some(ref e) = enrollment {
        sqlx::query_as("SELECT * FROM user_lesson_progress WHERE enrollment_id = $1")
            .bind(e.id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    // Defense-in-depth: serialize each lesson but strip the protected video
    // payload (`bunny_video_guid`) for any lesson the caller is not entitled
    // to. Enrolled users and admins are entitled to every lesson, so their
    // experience is unchanged; a viewer on a free course only ever sees
    // GUIDs for lessons that are themselves free/preview (or, for a free
    // course, all of them). The response shape/keys are preserved — the
    // protected field is merely nulled rather than leaked.
    let modules_with_lessons: Vec<serde_json::Value> = modules
        .into_iter()
        .map(|m| {
            let module_lessons: Vec<serde_json::Value> = lessons
                .iter()
                .filter(|l| l.module_id == Some(m.id))
                .map(|l| {
                    let entitled = is_lesson_entitled(
                        course_is_free,
                        is_enrolled,
                        is_admin,
                        l.is_free,
                        l.is_preview.unwrap_or(false),
                    );
                    let mut value = json!(l);
                    if !entitled {
                        if let Some(obj) = value.as_object_mut() {
                            obj.insert("bunny_video_guid".to_string(), json!(null));
                        }
                    }
                    value
                })
                .collect();
            json!({
                "module": m,
                "lessons": module_lessons
            })
        })
        .collect();

    // Downloads (PDFs, indicator files, etc.) are an enrollment benefit —
    // mirror `get_course_downloads`, which only serves them to enrolled
    // users. A non-enrolled, non-admin viewer of a free course can still
    // open the player, but must not receive download links.
    let visible_downloads: Vec<CourseDownload> = if is_enrolled || is_admin {
        downloads
    } else {
        Vec::new()
    };

    Ok(Json(json!({
        "success": true,
        "data": {
            "course": course,
            "modules": modules_with_lessons,
            "downloads": visible_downloads,
            "enrollment": enrollment,
            "progress": progress,
            "is_enrolled": is_enrolled
        }
    })))
}

pub(super) async fn update_lesson_progress(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<UpdateProgressRequest>,
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

    let enrollment: UserCourseEnrollment = sqlx::query_as(
        "SELECT * FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.0)
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
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        )
    })?;

    // Upsert progress
    let progress: UserLessonProgress = sqlx::query_as(
        r"
        INSERT INTO user_lesson_progress (
            user_id, lesson_id, enrollment_id, video_position_seconds,
            video_duration_seconds, is_completed
        ) VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, lesson_id) DO UPDATE SET
            video_position_seconds = COALESCE($4, user_lesson_progress.video_position_seconds),
            video_duration_seconds = COALESCE($5, user_lesson_progress.video_duration_seconds),
            is_completed = COALESCE($6, user_lesson_progress.is_completed),
            last_accessed_at = NOW(),
            view_count = user_lesson_progress.view_count + 1
        RETURNING *
        ",
    )
    .bind(user_id)
    .bind(input.lesson_id)
    .bind(enrollment.id)
    .bind(input.video_position_seconds)
    .bind(input.video_duration_seconds)
    .bind(input.is_completed)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update progress: {}", e)})),
        )
    })?;

    // Update enrollment last accessed
    sqlx::query(
        r"
        UPDATE user_course_enrollments
        SET current_lesson_id = $1, last_accessed_at = NOW()
        WHERE id = $2
        ",
    )
    .bind(input.lesson_id)
    .bind(enrollment.id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({
        "success": true,
        "data": progress
    })))
}

pub(super) async fn get_course_downloads(
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

    // Check enrollment
    let enrolled: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r"
        SELECT * FROM course_downloads
        WHERE course_id = $1 AND (is_public = true OR $2 = true)
        ORDER BY sort_order
        ",
    )
    .bind(course.0)
    .bind(enrolled.is_some())
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "downloads": downloads,
            "is_enrolled": enrolled.is_some()
        }
    })))
}
