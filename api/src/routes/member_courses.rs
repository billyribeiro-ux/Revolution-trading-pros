//! Member Course API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//! Public and authenticated member endpoints for courses

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::course::{
    Course, CourseDownload, CourseListItem, CourseModule, CourseQueryParams, LessonListItem,
    ModuleWithLessons, UpdateProgressRequest, UserCourseEnrollment, UserLessonProgress,
};
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// PUBLIC COURSE ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_published_courses(
    State(state): State<AppState>,
    Query(params): Query<CourseQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(50);
    let offset = (page - 1) * per_page;

    let courses: Vec<CourseListItem> = sqlx::query_as(
        r#"
        SELECT id, title, slug, description, card_image_url, card_description,
               card_badge, card_badge_color, instructor_name, instructor_avatar_url,
               level, price_cents, is_free, is_published, status, module_count,
               lesson_count, total_duration_minutes, enrollment_count, avg_rating,
               review_count, created_at
        FROM courses
        WHERE is_published = true AND status = 'published'
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM courses WHERE is_published = true AND status = 'published'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": {
            "courses": courses,
            "total": total.0,
            "page": page,
            "per_page": per_page
        }
    })))
}

async fn get_course_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r#"
        SELECT * FROM courses 
        WHERE slug = $1 AND is_published = true
        "#,
    )
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

    let modules: Vec<CourseModule> = sqlx::query_as(
        r#"
        SELECT * FROM course_modules 
        WHERE course_id = $1 AND is_published = true
        ORDER BY sort_order
        "#,
    )
    .bind(course.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let lessons: Vec<LessonListItem> = sqlx::query_as(
        r#"
        SELECT id, course_id, module_id, title, slug, description, duration_minutes,
               position, sort_order, is_free, is_preview, is_published, 
               bunny_video_guid, thumbnail_url
        FROM lessons 
        WHERE course_id = $1 AND (is_published = true OR is_published IS NULL)
        ORDER BY COALESCE(sort_order, position)
        "#,
    )
    .bind(course.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let modules_with_lessons: Vec<ModuleWithLessons> = modules
        .into_iter()
        .map(|m| {
            let module_lessons: Vec<LessonListItem> = lessons
                .iter()
                .filter(|l| l.module_id == Some(m.id))
                .cloned()
                .collect();
            ModuleWithLessons {
                module: m,
                lessons: module_lessons,
            }
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "course": course,
            "modules": modules_with_lessons
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// AUTHENTICATED MEMBER ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

async fn get_my_courses(
    State(state): State<AppState>,
    // In production, extract user_id from auth middleware
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // For now, return empty - in production, get user_id from auth context
    let user_id: i64 = 1; // Placeholder

    let enrollments: Vec<UserCourseEnrollment> = sqlx::query_as(
        r#"
        SELECT * FROM user_course_enrollments
        WHERE user_id = $1
        ORDER BY last_accessed_at DESC NULLS LAST
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let mut result = Vec::new();
    for enrollment in enrollments {
        let course: Option<CourseListItem> = sqlx::query_as(
            r#"
            SELECT id, title, slug, description, card_image_url, card_description,
                   card_badge, card_badge_color, instructor_name, instructor_avatar_url,
                   level, price_cents, is_free, is_published, status, module_count,
                   lesson_count, total_duration_minutes, enrollment_count, avg_rating,
                   review_count, created_at
            FROM courses WHERE id = $1
            "#,
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

async fn get_course_player(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = 1; // Placeholder - get from auth

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

    let modules: Vec<CourseModule> =
        sqlx::query_as("SELECT * FROM course_modules WHERE course_id = $1 ORDER BY sort_order")
            .bind(course.id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    let lessons: Vec<LessonListItem> = sqlx::query_as(
        r#"
        SELECT id, course_id, module_id, title, slug, description, duration_minutes,
               position, sort_order, is_free, is_preview, is_published, 
               bunny_video_guid, thumbnail_url
        FROM lessons WHERE course_id = $1
        ORDER BY COALESCE(sort_order, position)
        "#,
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

    let modules_with_lessons: Vec<ModuleWithLessons> = modules
        .into_iter()
        .map(|m| {
            let module_lessons = lessons
                .iter()
                .filter(|l| l.module_id == Some(m.id))
                .cloned()
                .collect();
            ModuleWithLessons {
                module: m,
                lessons: module_lessons,
            }
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "course": course,
            "modules": modules_with_lessons,
            "downloads": downloads,
            "enrollment": enrollment,
            "progress": progress,
            "is_enrolled": enrollment.is_some()
        }
    })))
}

async fn update_lesson_progress(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<UpdateProgressRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = 1; // Placeholder

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
        r#"
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
        "#,
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
        r#"
        UPDATE user_course_enrollments 
        SET current_lesson_id = $1, last_accessed_at = NOW()
        WHERE id = $2
        "#,
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

async fn get_course_downloads(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = 1; // Placeholder

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
        r#"
        SELECT * FROM course_downloads 
        WHERE course_id = $1 AND (is_public = true OR $2 = true)
        ORDER BY sort_order
        "#,
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

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_published_courses))
        .route("/:slug", get(get_course_detail))
}

pub fn member_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_courses))
        .route("/:slug/player", get(get_course_player))
        .route("/:slug/progress", put(update_lesson_progress))
        .route("/:slug/downloads", get(get_course_downloads))
}
