//! Member Course API Routes
//! Apple Principal Engineer ICT 7 Grade - February 2026
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
use crate::models::User;
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

    // ICT 7 Grade: SECURE parameterized query with search and filter
    let safe_sort_by = match params.sort_by.as_deref() {
        Some("title") => "title",
        Some("enrollment_count") => "enrollment_count",
        Some("avg_rating") => "avg_rating",
        Some("price_cents") => "price_cents",
        _ => "created_at",
    };
    let safe_sort_order = match params.sort_order.as_deref() {
        Some(s) if s.eq_ignore_ascii_case("ASC") => "ASC",
        _ => "DESC",
    };

    let query = format!(
        r#"
        SELECT id, title, slug, description, card_image_url, card_description,
               card_badge, card_badge_color, instructor_name, instructor_avatar_url,
               level, price_cents, is_free, is_published, status, module_count,
               lesson_count, total_duration_minutes, enrollment_count, avg_rating,
               review_count, created_at
        FROM courses
        WHERE is_published = true AND status = 'published'
          AND ($1::text IS NULL OR level = $1)
          AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR description ILIKE '%' || $2 || '%')
          AND ($3::boolean IS NULL OR is_free = $3)
        ORDER BY {} {}
        LIMIT $4 OFFSET $5
        "#,
        safe_sort_by, safe_sort_order
    );

    let courses: Vec<CourseListItem> = sqlx::query_as(&query)
        .bind(params.level.as_deref())
        .bind(params.search.as_deref())
        .bind(params.is_free)
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

    // Get total with same filters
    let count_query = r#"
        SELECT COUNT(*) FROM courses
        WHERE is_published = true AND status = 'published'
          AND ($1::text IS NULL OR level = $1)
          AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR description ILIKE '%' || $2 || '%')
          AND ($3::boolean IS NULL OR is_free = $3)
    "#;

    let total: (i64,) = sqlx::query_as(count_query)
        .bind(params.level.as_deref())
        .bind(params.search.as_deref())
        .bind(params.is_free)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as i32;

    Ok(Json(json!({
        "success": true,
        "data": {
            "courses": courses,
            "total": total.0,
            "page": page,
            "per_page": per_page,
            "total_pages": total_pages,
            "filters": {
                "level": params.level,
                "search": params.search,
                "is_free": params.is_free,
                "sort_by": safe_sort_by,
                "sort_order": safe_sort_order
            }
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
        SELECT * FROM course_modules_v2 
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
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Grade: User extracted from JWT auth middleware
    let user_id = user.id;

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

    let modules: Vec<CourseModule> =
        sqlx::query_as("SELECT * FROM course_modules_v2 WHERE course_id = $1 ORDER BY sort_order")
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
// CERTIFICATE GENERATION
// ═══════════════════════════════════════════════════════════════════════════════════

async fn get_certificate(
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
        r#"
        SELECT id, progress_percent, certificate_issued, certificate_url
        FROM user_course_enrollments
        WHERE user_id = $1 AND course_id = $2
        "#,
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
    let certificate_url = format!(
        "https://revolution-trading-pros.com/certificates/verify/{}",
        cert_id
    );

    // Update enrollment with certificate info
    sqlx::query(
        r#"
        UPDATE user_course_enrollments
        SET certificate_issued = true,
            certificate_url = $1,
            certificate_issued_at = NOW(),
            completed_at = COALESCE(completed_at, NOW())
        WHERE id = $2
        "#,
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
async fn resume_course(
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
        r#"
        SELECT id, current_lesson_id, progress_percent
        FROM user_course_enrollments
        WHERE user_id = $1 AND course_id = $2
        "#,
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
            r#"
            SELECT l.id, l.title, l.slug, p.video_position_seconds, l.duration_minutes
            FROM lessons l
            LEFT JOIN user_lesson_progress p ON p.lesson_id = l.id AND p.user_id = $1
            WHERE l.id = $2
            "#,
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
            r#"
            SELECT l.id, l.title, l.slug, p.video_position_seconds, l.duration_minutes
            FROM lessons l
            LEFT JOIN user_lesson_progress p ON p.lesson_id = l.id AND p.user_id = $1 AND p.enrollment_id = $2
            WHERE l.course_id = $3 AND (p.is_completed IS NULL OR p.is_completed = false)
            ORDER BY COALESCE(l.sort_order, l.position)
            LIMIT 1
            "#,
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

// ═══════════════════════════════════════════════════════════════════════════════════
// REVIEWS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

use chrono::NaiveDateTime;

/// Get reviews for a course
async fn get_course_reviews(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
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

    let reviews: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, i64, i16, Option<String>, Option<String>, bool, NaiveDateTime)>(
        r#"SELECT r.id, r.user_id, r.rating, r.title, r.content, r.is_verified_purchase, r.created_at
           FROM course_reviews r WHERE r.course_id = $1 AND r.is_approved = true ORDER BY r.created_at DESC LIMIT 50"#,
    )
    .bind(course.0)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| json!({"id": r.0, "user_id": r.1, "rating": r.2, "title": r.3, "content": r.4, "is_verified_purchase": r.5, "created_at": r.6.format("%Y-%m-%dT%H:%M:%S").to_string()}))
    .collect();

    // Get rating summary
    let summary: (Option<f64>, i64) = sqlx::query_as(
        "SELECT AVG(rating::float), COUNT(*) FROM course_reviews WHERE course_id = $1 AND is_approved = true"
    )
    .bind(course.0)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None, 0));

    // Get rating distribution
    let distribution: Vec<(i16, i64)> = sqlx::query_as(
        "SELECT rating, COUNT(*) FROM course_reviews WHERE course_id = $1 AND is_approved = true GROUP BY rating ORDER BY rating DESC"
    )
    .bind(course.0)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let mut dist_map = std::collections::HashMap::new();
    for (rating, count) in distribution {
        dist_map.insert(rating, count);
    }

    Ok(Json(json!({
        "success": true,
        "data": {
            "reviews": reviews,
            "summary": {
                "avg_rating": summary.0.unwrap_or(0.0),
                "total_reviews": summary.1,
                "rating_distribution": {
                    "5": dist_map.get(&5).unwrap_or(&0),
                    "4": dist_map.get(&4).unwrap_or(&0),
                    "3": dist_map.get(&3).unwrap_or(&0),
                    "2": dist_map.get(&2).unwrap_or(&0),
                    "1": dist_map.get(&1).unwrap_or(&0)
                }
            }
        }
    })))
}

/// Submit a review for a course
async fn submit_review(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;
    let rating = input["rating"].as_i64().unwrap_or(5).clamp(1, 5) as i16;
    let title = input["title"].as_str();
    let content = input["content"].as_str();

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

    // Check if user is enrolled (verified purchase)
    let enrollment: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let is_verified = enrollment.is_some();

    // Check if already reviewed
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM course_reviews WHERE user_id = $1 AND course_id = $2")
            .bind(user_id)
            .bind(course.0)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();

    if existing.is_some() {
        // Update existing review
        sqlx::query("UPDATE course_reviews SET rating = $1, title = $2, content = $3, updated_at = NOW() WHERE user_id = $4 AND course_id = $5")
            .bind(rating)
            .bind(title)
            .bind(content)
            .bind(user_id)
            .bind(course.0)
            .execute(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to update review: {}", e)}))))?;

        return Ok(Json(
            json!({"success": true, "message": "Review updated successfully"}),
        ));
    }

    // Create new review
    let review = sqlx::query_as::<_, (i64, i16, NaiveDateTime)>(
        r#"INSERT INTO course_reviews (user_id, course_id, rating, title, content, is_verified_purchase, is_approved)
           VALUES ($1, $2, $3, $4, $5, $6, true) RETURNING id, rating, created_at"#
    )
    .bind(user_id)
    .bind(course.0)
    .bind(rating)
    .bind(title)
    .bind(content)
    .bind(is_verified)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to submit review: {}", e)}))))?;

    // Update course review count and average
    sqlx::query("UPDATE courses SET review_count = COALESCE(review_count, 0) + 1, avg_rating = (SELECT AVG(rating::float) FROM course_reviews WHERE course_id = $1 AND is_approved = true) WHERE id = $1")
        .bind(course.0)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(json!({
        "success": true,
        "message": "Review submitted successfully",
        "data": {"id": review.0, "rating": review.1, "is_verified_purchase": is_verified, "created_at": review.2.format("%Y-%m-%dT%H:%M:%S").to_string()}
    })))
}

/// Delete user's own review
async fn delete_review(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
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

    sqlx::query("DELETE FROM course_reviews WHERE user_id = $1 AND course_id = $2")
        .bind(user_id)
        .bind(course.0)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete review: {}", e)})),
            )
        })?;

    // Update course review count
    sqlx::query("UPDATE courses SET review_count = GREATEST(COALESCE(review_count, 1) - 1, 0) WHERE id = $1")
        .bind(course.0)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(
        json!({"success": true, "message": "Review deleted successfully"}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// QUIZ SUBMISSION - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get available quizzes for a course
async fn get_course_quizzes(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
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

    // Check enrollment
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

    let quizzes: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, Option<String>, String, Option<i32>, Option<i32>, Option<i32>, bool)>(
        r#"SELECT id, title, description, quiz_type, passing_score, time_limit_minutes, max_attempts, is_required
           FROM course_quizzes WHERE course_id = $1 AND is_published = true ORDER BY sort_order"#,
    )
    .bind(course.0)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| json!({"id": q.0, "title": q.1, "description": q.2, "quiz_type": q.3, "passing_score": q.4, "time_limit_minutes": q.5, "max_attempts": q.6, "is_required": q.7}))
    .collect();

    Ok(Json(json!({"success": true, "data": quizzes})))
}

/// Start a quiz attempt
async fn start_quiz(
    user: User,
    State(state): State<AppState>,
    Path((slug, quiz_id)): Path<(String, i64)>,
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
    let enrollment: (i64,) = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
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

    // Get quiz details
    let quiz: (i64, String, Option<i32>, Option<i32>, bool, bool) = sqlx::query_as(
        "SELECT id, title, max_attempts, time_limit_minutes, shuffle_questions, shuffle_answers FROM course_quizzes WHERE id = $1 AND course_id = $2 AND is_published = true"
    )
    .bind(quiz_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Database error: {}", e)}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Quiz not found"}))))?;

    // Check attempt count
    let attempt_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_quiz_attempts WHERE user_id = $1 AND quiz_id = $2",
    )
    .bind(user_id)
    .bind(quiz_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    if let Some(max) = quiz.2 {
        if attempt_count.0 >= max as i64 {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error": "Maximum attempts reached", "max_attempts": max})),
            ));
        }
    }

    // Get questions
    let order_clause = if quiz.4 { "RANDOM()" } else { "sort_order" };
    let questions: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, String, i32)>(
        &format!("SELECT id, question_type, question_text, points FROM quiz_questions WHERE quiz_id = $1 ORDER BY {}", order_clause)
    )
    .bind(quiz_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| {
        let question_id = q.0;
        json!({"id": question_id, "question_type": q.1, "question_text": q.2, "points": q.3})
    })
    .collect();

    // Get answers for each question (without is_correct for students)
    let mut questions_with_answers = Vec::new();
    for mut question in questions {
        let question_id = question["id"].as_i64().unwrap_or(0);
        let answer_order = if quiz.5 { "RANDOM()" } else { "sort_order" };
        let answers: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String)>(&format!(
            "SELECT id, answer_text FROM quiz_answers WHERE question_id = $1 ORDER BY {}",
            answer_order
        ))
        .bind(question_id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| json!({"id": a.0, "answer_text": a.1}))
        .collect();
        question["answers"] = json!(answers);
        questions_with_answers.push(question);
    }

    // Calculate max score
    let max_score: (Option<i64>,) =
        sqlx::query_as("SELECT SUM(points) FROM quiz_questions WHERE quiz_id = $1")
            .bind(quiz_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    // Create attempt
    let attempt = sqlx::query_as::<_, (i64, i32, NaiveDateTime)>(
        r#"INSERT INTO user_quiz_attempts (user_id, quiz_id, enrollment_id, max_score, attempt_number, started_at)
           VALUES ($1, $2, $3, $4, $5, NOW()) RETURNING id, attempt_number, started_at"#
    )
    .bind(user_id)
    .bind(quiz_id)
    .bind(enrollment.0)
    .bind(max_score.0.unwrap_or(0) as i32)
    .bind(attempt_count.0 as i32 + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to start quiz: {}", e)}))))?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "attempt_id": attempt.0,
            "attempt_number": attempt.1,
            "started_at": attempt.2.format("%Y-%m-%dT%H:%M:%S").to_string(),
            "quiz_title": quiz.1,
            "time_limit_minutes": quiz.3,
            "max_score": max_score.0.unwrap_or(0),
            "questions": questions_with_answers
        }
    })))
}

/// Submit quiz answers
async fn submit_quiz(
    user: User,
    State(state): State<AppState>,
    Path((slug, quiz_id, attempt_id)): Path<(String, i64, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    // Verify attempt belongs to user
    let attempt: (i64, i32, NaiveDateTime) = sqlx::query_as(
        "SELECT id, max_score, started_at FROM user_quiz_attempts WHERE id = $1 AND user_id = $2 AND quiz_id = $3 AND completed_at IS NULL"
    )
    .bind(attempt_id)
    .bind(user_id)
    .bind(quiz_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Database error: {}", e)}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Quiz attempt not found or already submitted"}))))?;

    // Get quiz passing score
    let quiz: (Option<i32>, Option<bool>) = sqlx::query_as(
        "SELECT passing_score, show_correct_answers FROM course_quizzes WHERE id = $1",
    )
    .bind(quiz_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((Some(70), Some(true)));

    // Calculate score
    let mut total_score = 0i32;
    let empty_vec = Vec::new();
    let answers = input["answers"].as_array().unwrap_or(&empty_vec);
    let mut results = Vec::new();

    for answer in answers {
        let question_id = answer["question_id"].as_i64().unwrap_or(0);
        let answer_id = answer["answer_id"].as_i64();

        // Get correct answer and points
        let question: Option<(i32,)> =
            sqlx::query_as("SELECT points FROM quiz_questions WHERE id = $1")
                .bind(question_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

        if let Some(q) = question {
            if let Some(aid) = answer_id {
                let is_correct: Option<(bool,)> = sqlx::query_as(
                    "SELECT is_correct FROM quiz_answers WHERE id = $1 AND question_id = $2",
                )
                .bind(aid)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

                if is_correct.map(|c| c.0).unwrap_or(false) {
                    total_score += q.0;
                    results
                        .push(json!({"question_id": question_id, "correct": true, "points": q.0}));
                } else {
                    results
                        .push(json!({"question_id": question_id, "correct": false, "points": 0}));
                }
            }
        }
    }

    let score_percent = if attempt.1 > 0 {
        total_score as f64 / attempt.1 as f64 * 100.0
    } else {
        0.0
    };
    let passed = score_percent >= quiz.0.unwrap_or(70) as f64;
    let time_spent = (chrono::Utc::now().naive_utc() - attempt.2).num_seconds() as i32;

    // Update attempt
    sqlx::query(
        r#"UPDATE user_quiz_attempts SET score = $1, score_percent = $2, passed = $3, completed_at = NOW(), time_spent_seconds = $4, answers = $5 WHERE id = $6"#
    )
    .bind(total_score)
    .bind(score_percent)
    .bind(passed)
    .bind(time_spent)
    .bind(json!(answers))
    .bind(attempt_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to submit quiz: {}", e)}))))?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "score": total_score,
            "max_score": attempt.1,
            "score_percent": score_percent,
            "passed": passed,
            "passing_score": quiz.0.unwrap_or(70),
            "time_spent_seconds": time_spent,
            "results": if quiz.1.unwrap_or(true) { json!(results) } else { json!(null) }
        }
    })))
}

/// Get quiz results history
async fn get_quiz_results(
    user: User,
    State(state): State<AppState>,
    Path((slug, quiz_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    let attempts: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, Option<i32>, Option<f64>, Option<bool>, i32, NaiveDateTime, Option<NaiveDateTime>)>(
        "SELECT id, score, score_percent, passed, attempt_number, started_at, completed_at FROM user_quiz_attempts WHERE user_id = $1 AND quiz_id = $2 ORDER BY attempt_number DESC"
    )
    .bind(user_id)
    .bind(quiz_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|a| json!({
        "attempt_id": a.0, "score": a.1, "score_percent": a.2, "passed": a.3,
        "attempt_number": a.4, "started_at": a.5.format("%Y-%m-%dT%H:%M:%S").to_string(),
        "completed_at": a.6.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string())
    }))
    .collect();

    Ok(Json(json!({"success": true, "data": attempts})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PREREQUISITES - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// Check if user can access a lesson (prerequisites satisfied)
async fn check_lesson_access(
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

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_published_courses))
        .route("/:slug", get(get_course_detail))
        .route("/:slug/reviews", get(get_course_reviews))
}

pub fn member_router() -> Router<AppState> {
    use axum::routing::{delete, post};

    Router::new()
        .route("/", get(get_my_courses))
        .route("/:slug/player", get(get_course_player))
        .route("/:slug/progress", put(update_lesson_progress))
        .route("/:slug/downloads", get(get_course_downloads))
        .route("/:slug/certificate", get(get_certificate))
        .route("/:slug/resume", get(resume_course))
        // Reviews
        .route("/:slug/reviews", post(submit_review).delete(delete_review))
        // Quizzes
        .route("/:slug/quizzes", get(get_course_quizzes))
        .route("/:slug/quizzes/:quiz_id/start", post(start_quiz))
        .route(
            "/:slug/quizzes/:quiz_id/attempts/:attempt_id/submit",
            post(submit_quiz),
        )
        .route("/:slug/quizzes/:quiz_id/results", get(get_quiz_results))
        // Prerequisites
        .route("/:slug/lessons/:lesson_id/access", get(check_lesson_access))
}
