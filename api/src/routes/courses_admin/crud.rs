//! Course CRUD lifecycle handlers
//! Extracted from `courses_admin.rs` in R14-B split (2026-05-20).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use slug::slugify;
use tracing::{error, info};

use crate::middleware::admin::AdminUser;
use crate::models::course_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE CRUD ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all courses with filters and pagination
pub(super) async fn list_courses(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<CourseListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // ICT 7+ SECURITY FIX: Parameterized queries to prevent SQL injection
    let courses: Vec<CourseEnhanced> = sqlx::query_as(
        r"SELECT * FROM courses_enhanced
           WHERE deleted_at IS NULL
             AND ($1::text IS NULL OR difficulty_level = $1)
             AND ($2::text IS NULL OR category = $2)
             AND ($3::bigint IS NULL OR instructor_id = $3)
             AND ($4::boolean IS NULL OR is_published = $4)
             AND ($5::boolean IS NULL OR is_featured = $5)
             AND ($6::text IS NULL OR title ILIKE '%' || $6 || '%' OR description ILIKE '%' || $6 || '%')
           ORDER BY is_featured DESC, created_at DESC
           LIMIT $7 OFFSET $8"
    )
    .bind(query.difficulty_level.as_deref())
    .bind(query.category.as_deref())
    .bind(query.instructor_id)
    .bind(query.is_published)
    .bind(query.is_featured)
    .bind(query.search.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to list courses: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        r"SELECT COUNT(*) FROM courses_enhanced
           WHERE deleted_at IS NULL
             AND ($1::text IS NULL OR difficulty_level = $1)
             AND ($2::text IS NULL OR category = $2)
             AND ($3::bigint IS NULL OR instructor_id = $3)
             AND ($4::boolean IS NULL OR is_published = $4)
             AND ($5::boolean IS NULL OR is_featured = $5)
             AND ($6::text IS NULL OR title ILIKE '%' || $6 || '%' OR description ILIKE '%' || $6 || '%')"
    )
    .bind(query.difficulty_level.as_deref())
    .bind(query.category.as_deref())
    .bind(query.instructor_id)
    .bind(query.is_published)
    .bind(query.is_featured)
    .bind(query.search.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let course_responses: Vec<serde_json::Value> = courses
        .iter()
        .map(|c| {
            json!({
                "id": c.id,
                "title": c.title,
                "slug": c.slug,
                "subtitle": c.subtitle,
                "thumbnail_url": c.thumbnail_url,
                "difficulty_level": c.difficulty_level,
                "category": c.category,
                "estimated_duration_minutes": c.estimated_duration_minutes,
                "formatted_duration": format_duration_minutes(c.estimated_duration_minutes),
                "total_lessons": c.total_lessons,
                "total_sections": c.total_sections,
                "is_published": c.is_published,
                "is_featured": c.is_featured,
                "is_free": c.is_free,
                "certificate_enabled": c.certificate_enabled,
                "created_at": c.created_at.to_rfc3339()
            })
        })
        .collect();

    Ok(Json(json!({
        "courses": course_responses,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get a single course with all sections, lessons, resources
pub(super) async fn get_course(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path(course_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: CourseEnhanced =
        sqlx::query_as("SELECT * FROM courses_enhanced WHERE id = $1 AND deleted_at IS NULL")
            .bind(course_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                error!("Failed to get course: {}", e);
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

    // Get sections
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

    // Get lessons grouped by section
    let lessons: Vec<CourseLesson> =
        sqlx::query_as("SELECT * FROM course_lessons WHERE course_id = $1 ORDER BY sort_order")
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

    // Get resources grouped by section/lesson
    let resources: Vec<CourseResource> =
        sqlx::query_as("SELECT * FROM course_resources WHERE course_id = $1 ORDER BY sort_order")
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

    // Get live sessions
    let live_sessions: Vec<CourseLiveSession> = sqlx::query_as(
        "SELECT * FROM course_live_sessions WHERE course_id = $1 ORDER BY session_date DESC, sort_order"
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))? ;

    // Build section responses with nested lessons and resources
    let section_responses: Vec<serde_json::Value> = sections
        .iter()
        .map(|s| {
            let section_lessons: Vec<serde_json::Value> = lessons
                .iter()
                .filter(|l| l.section_id == s.id)
                .map(|l| {
                    let lesson_resources: Vec<serde_json::Value> = resources
                        .iter()
                        .filter(|r| r.lesson_id == Some(l.id))
                        .map(|r| json!({
                            "id": r.id,
                            "title": r.title,
                            "description": r.description,
                            "file_url": r.file_url,
                            "file_name": r.file_name,
                            "file_type": r.file_type,
                            "file_size_bytes": r.file_size_bytes,
                            "formatted_size": format_file_size(r.file_size_bytes.unwrap_or(0)),
                            "version": r.version,
                            "download_count": r.download_count
                        }))
                        .collect();

                    json!({
                        "id": l.id,
                        "title": l.title,
                        "description": l.description,
                        "video_url": l.video_url,
                        "embed_url": get_embed_url(&l.video_url, &l.bunny_video_guid, l.bunny_library_id),
                        "thumbnail_url": l.thumbnail_url,
                        "duration_seconds": l.duration_seconds,
                        "formatted_duration": l.duration_seconds.map(format_duration_seconds),
                        "sort_order": l.sort_order,
                        "lesson_type": l.lesson_type,
                        "is_preview": l.is_preview,
                        "is_published": l.is_published,
                        "completion_type": l.completion_type,
                        "required_watch_percent": l.required_watch_percent,
                        "resources": lesson_resources
                    })
                })
                .collect();

            let section_resources: Vec<serde_json::Value> = resources
                .iter()
                .filter(|r| r.section_id == Some(s.id) && r.lesson_id.is_none())
                .map(|r| json!({
                    "id": r.id,
                    "title": r.title,
                    "description": r.description,
                    "file_url": r.file_url,
                    "file_name": r.file_name,
                    "file_type": r.file_type,
                    "file_size_bytes": r.file_size_bytes,
                    "formatted_size": format_file_size(r.file_size_bytes.unwrap_or(0)),
                    "version": r.version,
                    "download_count": r.download_count
                }))
                .collect();

            json!({
                "id": s.id,
                "title": s.title,
                "description": s.description,
                "sort_order": s.sort_order,
                "section_type": s.section_type,
                "unlock_type": s.unlock_type,
                "unlock_after_section_id": s.unlock_after_section_id,
                "unlock_date": s.unlock_date.map(|d| d.to_rfc3339()),
                "is_published": s.is_published,
                "lesson_count": s.lesson_count,
                "estimated_duration_minutes": s.estimated_duration_minutes,
                "lessons": section_lessons,
                "resources": section_resources
            })
        })
        .collect();

    // Course-level resources
    let course_resources: Vec<serde_json::Value> = resources
        .iter()
        .filter(|r| r.section_id.is_none() && r.lesson_id.is_none())
        .map(|r| {
            json!({
                "id": r.id,
                "title": r.title,
                "description": r.description,
                "file_url": r.file_url,
                "file_name": r.file_name,
                "file_type": r.file_type,
                "file_size_bytes": r.file_size_bytes,
                "formatted_size": format_file_size(r.file_size_bytes.unwrap_or(0)),
                "version": r.version,
                "download_count": r.download_count
            })
        })
        .collect();

    // Live sessions response
    let live_session_responses: Vec<serde_json::Value> = live_sessions
        .iter()
        .map(|ls| json!({
            "id": ls.id,
            "title": ls.title,
            "description": ls.description,
            "session_date": ls.session_date.to_string(),
            "session_time": ls.session_time.map(|t| t.to_string()),
            "formatted_date": ls.session_date.format("%B %d, %Y").to_string(),
            "video_url": ls.video_url,
            "embed_url": get_embed_url(&ls.video_url, &ls.bunny_video_guid, ls.bunny_library_id),
            "thumbnail_url": ls.thumbnail_url,
            "duration_seconds": ls.duration_seconds,
            "formatted_duration": ls.duration_seconds.map(format_duration_seconds),
            "is_published": ls.is_published,
            "sort_order": ls.sort_order
        }))
        .collect();

    // Parse tags
    let tags: Vec<String> = course
        .tags
        .as_ref()
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(Json(json!({
        "id": course.id,
        "title": course.title,
        "slug": course.slug,
        "subtitle": course.subtitle,
        "description": course.description,
        "description_html": course.description_html,
        "thumbnail_url": course.thumbnail_url,
        "trailer_video_url": course.trailer_video_url,
        "difficulty_level": course.difficulty_level,
        "category": course.category,
        "tags": tags,
        "instructor_id": course.instructor_id,
        "estimated_duration_minutes": course.estimated_duration_minutes,
        "formatted_duration": format_duration_minutes(course.estimated_duration_minutes),
        "total_lessons": course.total_lessons,
        "total_sections": course.total_sections,
        "is_published": course.is_published,
        "is_featured": course.is_featured,
        "is_free": course.is_free,
        "required_plan_id": course.required_plan_id,
        "price_cents": course.price_cents,
        "certificate_enabled": course.certificate_enabled,
        "certificate_template": course.certificate_template,
        "completion_threshold_percent": course.completion_threshold_percent,
        "sections": section_responses,
        "resources": course_resources,
        "live_sessions": live_session_responses,
        "created_at": course.created_at.to_rfc3339(),
        "updated_at": course.updated_at.to_rfc3339(),
        "published_at": course.published_at.map(|d| d.to_rfc3339())
    })))
}

/// Create a new course
pub(super) async fn create_course(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Json(input): Json<CreateCourseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        target: "security",
        event = "course_create",
        admin_id = %admin.id,
        course_title = %input.title,
        "Admin creating course"
    );
    let slug = slugify(&input.title);
    let tags_json = input
        .tags
        .as_ref()
        .map(|t| serde_json::to_value(t).unwrap_or_default());
    let prereq_json = input
        .prerequisite_course_ids
        .as_ref()
        .map(|p| serde_json::to_value(p).unwrap_or_default());

    let course: CourseEnhanced = sqlx::query_as(
        r"INSERT INTO courses_enhanced
           (title, slug, subtitle, description, description_html, thumbnail_url,
            trailer_video_url, difficulty_level, category, tags, instructor_id,
            is_published, is_featured, is_free, required_plan_id, price_cents,
            prerequisite_course_ids, certificate_enabled, completion_threshold_percent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
           RETURNING *"
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.subtitle)
    .bind(&input.description)
    .bind(&input.description_html)
    .bind(&input.thumbnail_url)
    .bind(&input.trailer_video_url)
    .bind(input.difficulty_level.as_deref().unwrap_or("beginner"))
    .bind(&input.category)
    .bind(&tags_json)
    .bind(input.instructor_id)
    .bind(input.is_published.unwrap_or(false))
    .bind(input.is_featured.unwrap_or(false))
    .bind(input.is_free.unwrap_or(false))
    .bind(input.required_plan_id)
    .bind(input.price_cents)
    .bind(&prereq_json)
    .bind(input.certificate_enabled.unwrap_or(false))
    .bind(input.completion_threshold_percent.unwrap_or(80))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create course: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    info!("Created course: {} ({})", course.title, course.id);

    Ok(Json(json!({
        "success": true,
        "course": {
            "id": course.id,
            "title": course.title,
            "slug": course.slug
        }
    })))
}

/// Update a course
/// ICT 7+ SECURITY: Uses parameterized queries to prevent SQL injection
pub(super) async fn update_course(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<UpdateCourseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        target: "security",
        event = "course_update",
        admin_id = %admin.id,
        course_id = %course_id,
        "Admin updating course"
    );

    // Generate new slug if title is being updated
    let new_slug = input.title.as_ref().map(slugify);
    let tags_json = input
        .tags
        .as_ref()
        .map(|t| serde_json::to_value(t).unwrap_or_default());
    let prereqs_json = input
        .prerequisite_course_ids
        .as_ref()
        .map(|p| serde_json::to_value(p).unwrap_or_default());
    let should_publish = input.is_published.unwrap_or(false);

    // ICT 7+ SECURITY FIX: Parameterized query
    let result: Option<(i64, String)> = sqlx::query_as(
        r"UPDATE courses_enhanced SET
            title = COALESCE($2, title),
            slug = COALESCE($3, slug),
            subtitle = COALESCE($4, subtitle),
            description = COALESCE($5, description),
            description_html = COALESCE($6, description_html),
            thumbnail_url = COALESCE($7, thumbnail_url),
            trailer_video_url = COALESCE($8, trailer_video_url),
            difficulty_level = COALESCE($9, difficulty_level),
            category = COALESCE($10, category),
            tags = COALESCE($11, tags),
            instructor_id = COALESCE($12, instructor_id),
            is_published = COALESCE($13, is_published),
            published_at = CASE WHEN $14 AND published_at IS NULL THEN NOW() ELSE published_at END,
            is_featured = COALESCE($15, is_featured),
            is_free = COALESCE($16, is_free),
            required_plan_id = COALESCE($17, required_plan_id),
            price_cents = COALESCE($18, price_cents),
            prerequisite_course_ids = COALESCE($19, prerequisite_course_ids),
            certificate_enabled = COALESCE($20, certificate_enabled),
            completion_threshold_percent = COALESCE($21, completion_threshold_percent),
            updated_at = NOW()
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, title",
    )
    .bind(course_id)
    .bind(&input.title)
    .bind(&new_slug)
    .bind(&input.subtitle)
    .bind(&input.description)
    .bind(&input.description_html)
    .bind(&input.thumbnail_url)
    .bind(&input.trailer_video_url)
    .bind(&input.difficulty_level)
    .bind(&input.category)
    .bind(&tags_json)
    .bind(input.instructor_id)
    .bind(input.is_published)
    .bind(should_publish)
    .bind(input.is_featured)
    .bind(input.is_free)
    .bind(input.required_plan_id)
    .bind(input.price_cents)
    .bind(&prereqs_json)
    .bind(input.certificate_enabled)
    .bind(input.completion_threshold_percent)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update course: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    match result {
        Some((id, title)) => {
            info!("Updated course: {} ({})", title, id);
            Ok(Json(json!({"success": true, "course_id": id})))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Course not found"})),
        )),
    }
}

/// Delete a course (soft delete)
pub(super) async fn delete_course(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "course_delete", admin_id = %admin.id, course_id = %course_id, "Admin deleting course");
    let result = sqlx::query(
        "UPDATE courses_enhanced SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to delete course: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Course not found"})),
        ));
    }

    info!("Soft deleted course: {}", course_id);
    Ok(Json(json!({"success": true})))
}
