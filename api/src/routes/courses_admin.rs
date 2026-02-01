//! Enhanced Course Admin Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Complete course management: CRUD, sections, lessons, resources, live sessions, progress
//!
//! SECURITY: All endpoints require AdminUser authentication

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
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
async fn list_courses(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Query(query): Query<CourseListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // ICT 7+ SECURITY FIX: Parameterized queries to prevent SQL injection
    let courses: Vec<CourseEnhanced> = sqlx::query_as(
        r#"SELECT * FROM courses_enhanced
           WHERE deleted_at IS NULL
             AND ($1::text IS NULL OR difficulty_level = $1)
             AND ($2::text IS NULL OR category = $2)
             AND ($3::bigint IS NULL OR instructor_id = $3)
             AND ($4::boolean IS NULL OR is_published = $4)
             AND ($5::boolean IS NULL OR is_featured = $5)
             AND ($6::text IS NULL OR title ILIKE '%' || $6 || '%' OR description ILIKE '%' || $6 || '%')
           ORDER BY is_featured DESC, created_at DESC
           LIMIT $7 OFFSET $8"#
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
        r#"SELECT COUNT(*) FROM courses_enhanced
           WHERE deleted_at IS NULL
             AND ($1::text IS NULL OR difficulty_level = $1)
             AND ($2::text IS NULL OR category = $2)
             AND ($3::bigint IS NULL OR instructor_id = $3)
             AND ($4::boolean IS NULL OR is_published = $4)
             AND ($5::boolean IS NULL OR is_featured = $5)
             AND ($6::text IS NULL OR title ILIKE '%' || $6 || '%' OR description ILIKE '%' || $6 || '%')"#
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
async fn get_course(
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
            .unwrap_or_default();

    // Get lessons grouped by section
    let lessons: Vec<CourseLesson> =
        sqlx::query_as("SELECT * FROM course_lessons WHERE course_id = $1 ORDER BY sort_order")
            .bind(course_id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    // Get resources grouped by section/lesson
    let resources: Vec<CourseResource> =
        sqlx::query_as("SELECT * FROM course_resources WHERE course_id = $1 ORDER BY sort_order")
            .bind(course_id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    // Get live sessions
    let live_sessions: Vec<CourseLiveSession> = sqlx::query_as(
        "SELECT * FROM course_live_sessions WHERE course_id = $1 ORDER BY session_date DESC, sort_order"
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

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
async fn create_course(
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
        r#"INSERT INTO courses_enhanced
           (title, slug, subtitle, description, description_html, thumbnail_url,
            trailer_video_url, difficulty_level, category, tags, instructor_id,
            is_published, is_featured, is_free, required_plan_id, price_cents,
            prerequisite_course_ids, certificate_enabled, completion_threshold_percent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
           RETURNING *"#
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
async fn update_course(
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
    let new_slug = input.title.as_ref().map(|t| slugify(t));
    let tags_json = input.tags.as_ref().map(|t| serde_json::to_value(t).unwrap_or_default());
    let prereqs_json = input.prerequisite_course_ids.as_ref().map(|p| serde_json::to_value(p).unwrap_or_default());
    let should_publish = input.is_published.unwrap_or(false);

    // ICT 7+ SECURITY FIX: Parameterized query
    let result: Option<(i64, String)> = sqlx::query_as(
        r#"UPDATE courses_enhanced SET
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
        RETURNING id, title"#
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
async fn delete_course(
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

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a section in a course
async fn create_section(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateSectionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "section_create", admin_id = %admin.id, course_id = %course_id, "Admin creating section");
    // Get max sort order
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_sections WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let unlock_date: Option<DateTime<Utc>> = input.unlock_date.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    let section: CourseSection = sqlx::query_as(
        r#"INSERT INTO course_sections
           (course_id, title, description, sort_order, section_type, unlock_type,
            unlock_after_section_id, unlock_date, is_published)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING *"#,
    )
    .bind(course_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(sort_order)
    .bind(input.section_type.as_deref().unwrap_or("standard"))
    .bind(input.unlock_type.as_deref().unwrap_or("immediate"))
    .bind(input.unlock_after_section_id)
    .bind(unlock_date)
    .bind(input.is_published.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create section: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!("Created section: {} in course {}", section.title, course_id);

    Ok(Json(json!({
        "success": true,
        "section": {
            "id": section.id,
            "title": section.title,
            "sort_order": section.sort_order
        }
    })))
}

/// Update a section
/// ICT 7+ SECURITY: Uses parameterized queries to prevent SQL injection
async fn update_section(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, section_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateSectionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "section_update", admin_id = %admin.id, course_id = %course_id, section_id = %section_id, "Admin updating section");

    // Parse unlock_date if provided
    let unlock_date: Option<DateTime<Utc>> = input.unlock_date.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    // ICT 7+ SECURITY FIX: Parameterized query
    sqlx::query(
        r#"UPDATE course_sections SET
            title = COALESCE($3, title),
            description = COALESCE($4, description),
            section_type = COALESCE($5, section_type),
            unlock_type = COALESCE($6, unlock_type),
            unlock_after_section_id = COALESCE($7, unlock_after_section_id),
            unlock_date = COALESCE($8, unlock_date),
            is_published = COALESCE($9, is_published),
            updated_at = NOW()
        WHERE id = $1 AND course_id = $2"#
    )
    .bind(section_id)
    .bind(course_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.section_type)
    .bind(&input.unlock_type)
    .bind(input.unlock_after_section_id)
    .bind(unlock_date)
    .bind(input.is_published)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update section: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({"success": true})))
}

/// Delete a section
async fn delete_section(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, section_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "section_delete", admin_id = %admin.id, course_id = %course_id, section_id = %section_id, "Admin deleting section");
    // Delete associated lessons and resources first
    sqlx::query("DELETE FROM course_resources WHERE section_id = $1")
        .bind(section_id)
        .execute(&state.db.pool)
        .await
        .ok();

    sqlx::query("DELETE FROM course_lessons WHERE section_id = $1")
        .bind(section_id)
        .execute(&state.db.pool)
        .await
        .ok();

    let result = sqlx::query("DELETE FROM course_sections WHERE id = $1 AND course_id = $2")
        .bind(section_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete section: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Section not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}

/// Reorder sections
async fn reorder_sections(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<ReorderItemsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.items {
        sqlx::query(
            "UPDATE course_sections SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3"
        )
        .bind(item.sort_order)
        .bind(item.id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LESSON ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a lesson
async fn create_lesson(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "lesson_create", admin_id = %admin.id, course_id = %course_id, "Admin creating lesson");
    // Get max sort order for section
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_lessons WHERE section_id = $1")
            .bind(input.section_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let lesson: CourseLesson = sqlx::query_as(
        r#"INSERT INTO course_lessons
           (course_id, section_id, title, description, content_html, video_url,
            bunny_video_guid, thumbnail_url, sort_order, lesson_type, is_preview,
            is_published, completion_type, required_watch_percent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
           RETURNING *"#,
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.content_html)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(sort_order)
    .bind(input.lesson_type.as_deref().unwrap_or("video"))
    .bind(input.is_preview.unwrap_or(false))
    .bind(input.is_published.unwrap_or(true))
    .bind(input.completion_type.as_deref().unwrap_or("watch"))
    .bind(input.required_watch_percent.unwrap_or(80))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create lesson: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!("Created lesson: {} in course {}", lesson.title, course_id);

    Ok(Json(json!({
        "success": true,
        "lesson": {
            "id": lesson.id,
            "title": lesson.title,
            "section_id": lesson.section_id,
            "sort_order": lesson.sort_order
        }
    })))
}

/// Update a lesson
/// ICT 7+ SECURITY: Uses parameterized queries to prevent SQL injection
async fn update_lesson(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, lesson_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "lesson_update", admin_id = %admin.id, course_id = %course_id, lesson_id = %lesson_id, "Admin updating lesson");

    // ICT 7+ SECURITY FIX: Parameterized query
    sqlx::query(
        r#"UPDATE course_lessons SET
            section_id = COALESCE($3, section_id),
            title = COALESCE($4, title),
            description = COALESCE($5, description),
            content_html = COALESCE($6, content_html),
            video_url = COALESCE($7, video_url),
            bunny_video_guid = COALESCE($8, bunny_video_guid),
            thumbnail_url = COALESCE($9, thumbnail_url),
            duration_seconds = COALESCE($10, duration_seconds),
            lesson_type = COALESCE($11, lesson_type),
            is_preview = COALESCE($12, is_preview),
            is_published = COALESCE($13, is_published),
            completion_type = COALESCE($14, completion_type),
            required_watch_percent = COALESCE($15, required_watch_percent),
            updated_at = NOW()
        WHERE id = $1 AND course_id = $2"#
    )
    .bind(lesson_id)
    .bind(course_id)
    .bind(input.section_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.content_html)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.duration_seconds)
    .bind(&input.lesson_type)
    .bind(input.is_preview)
    .bind(input.is_published)
    .bind(&input.completion_type)
    .bind(input.required_watch_percent)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update lesson: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({"success": true})))
}

/// Delete a lesson
async fn delete_lesson(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, lesson_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "lesson_delete", admin_id = %admin.id, course_id = %course_id, lesson_id = %lesson_id, "Admin deleting lesson");
    // Delete associated resources first
    sqlx::query("DELETE FROM course_resources WHERE lesson_id = $1")
        .bind(lesson_id)
        .execute(&state.db.pool)
        .await
        .ok();

    let result = sqlx::query("DELETE FROM course_lessons WHERE id = $1 AND course_id = $2")
        .bind(lesson_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete lesson: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Lesson not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}

/// Reorder lessons within a section
async fn reorder_lessons(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path((course_id, section_id)): Path<(i64, i64)>,
    Json(input): Json<ReorderItemsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.items {
        sqlx::query(
            "UPDATE course_lessons SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3 AND section_id = $4"
        )
        .bind(item.sort_order)
        .bind(item.id)
        .bind(course_id)
        .bind(section_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RESOURCE ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a resource (PDF, document, etc.)
async fn create_resource(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateResourceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "resource_create", admin_id = %admin.id, course_id = %course_id, "Admin creating resource");
    // Get max sort order
    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sort_order) FROM course_resources WHERE course_id = $1 AND section_id IS NOT DISTINCT FROM $2 AND lesson_id IS NOT DISTINCT FROM $3"
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(input.lesson_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let resource: CourseResource = sqlx::query_as(
        r#"INSERT INTO course_resources
           (course_id, section_id, lesson_id, title, description, file_url, file_name,
            file_type, file_size_bytes, sort_order, version)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
           RETURNING *"#,
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(input.lesson_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.file_url)
    .bind(&input.file_name)
    .bind(&input.file_type)
    .bind(input.file_size_bytes)
    .bind(sort_order)
    .bind(input.version.as_deref().unwrap_or("1.0"))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create resource: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "Created resource: {} for course {}",
        resource.title, course_id
    );

    Ok(Json(json!({
        "success": true,
        "resource": {
            "id": resource.id,
            "title": resource.title,
            "file_name": resource.file_name,
            "formatted_size": format_file_size(resource.file_size_bytes.unwrap_or(0))
        }
    })))
}

/// Delete a resource
async fn delete_resource(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, resource_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "resource_delete", admin_id = %admin.id, course_id = %course_id, resource_id = %resource_id, "Admin deleting resource");
    let result = sqlx::query("DELETE FROM course_resources WHERE id = $1 AND course_id = $2")
        .bind(resource_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete resource: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resource not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LIVE SESSION ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a live session
async fn create_live_session(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateLiveSessionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "live_session_create", admin_id = %admin.id, course_id = %course_id, "Admin creating live session");
    let session_date =
        NaiveDate::parse_from_str(&input.session_date, "%Y-%m-%d").map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid date format. Use YYYY-MM-DD"})),
            )
        })?;

    let session_time: Option<NaiveTime> = input.session_time.as_ref().and_then(|t| {
        NaiveTime::parse_from_str(t, "%H:%M:%S")
            .ok()
            .or_else(|| NaiveTime::parse_from_str(t, "%H:%M").ok())
    });

    let replay_until: Option<DateTime<Utc>> = input.replay_available_until.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    // Get max sort order
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_live_sessions WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let session: CourseLiveSession = sqlx::query_as(
        r#"INSERT INTO course_live_sessions
           (course_id, section_id, title, description, session_date, session_time,
            video_url, bunny_video_guid, thumbnail_url, replay_available_until,
            is_published, sort_order)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
           RETURNING *"#,
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(session_date)
    .bind(session_time)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(replay_until)
    .bind(input.is_published.unwrap_or(true))
    .bind(sort_order)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create live session: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "Created live session: {} for course {}",
        session.title, course_id
    );

    Ok(Json(json!({
        "success": true,
        "session": {
            "id": session.id,
            "title": session.title,
            "session_date": session.session_date.to_string()
        }
    })))
}

/// Bulk create live sessions
async fn bulk_create_live_sessions(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<BulkLiveSessionsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "live_sessions_bulk_create", admin_id = %admin.id, course_id = %course_id, count = %input.sessions.len(), "Admin bulk creating live sessions");
    let mut created = 0;

    for (i, session) in input.sessions.iter().enumerate() {
        let session_date = match NaiveDate::parse_from_str(&session.session_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };

        let result = sqlx::query(
            r#"INSERT INTO course_live_sessions
               (course_id, section_id, title, session_date, video_url, bunny_video_guid, sort_order)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(course_id)
        .bind(input.section_id)
        .bind(&session.title)
        .bind(session_date)
        .bind(&session.video_url)
        .bind(&session.bunny_video_guid)
        .bind(i as i32 + 1)
        .execute(&state.db.pool)
        .await;

        if result.is_ok() {
            created += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "created": created,
        "total": input.sessions.len()
    })))
}

/// Delete a live session
async fn delete_live_session(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, session_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "live_session_delete", admin_id = %admin.id, course_id = %course_id, session_id = %session_id, "Admin deleting live session");
    let result = sqlx::query("DELETE FROM course_live_sessions WHERE id = $1 AND course_id = $2")
        .bind(session_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete live session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Live session not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT & PROGRESS ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Enroll a user in a course
async fn enroll_user(
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
        r#"INSERT INTO user_course_enrollments
           (user_id, course_id, enrollment_source, total_lessons)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (user_id, course_id) DO UPDATE SET is_active = true
           RETURNING *"#,
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
async fn get_enrollments(
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
        r#"SELECT * FROM user_course_enrollments
           WHERE course_id = $1 AND is_active = true
           ORDER BY enrolled_at DESC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(course_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

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
async fn update_lesson_progress(
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
        r#"INSERT INTO user_lesson_progress
           (user_id, course_id, lesson_id, watch_position_seconds, watch_percent, is_completed, completed_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (user_id, lesson_id) DO UPDATE SET
             watch_position_seconds = GREATEST(user_lesson_progress.watch_position_seconds, $4),
             watch_time_total_seconds = user_lesson_progress.watch_time_total_seconds + $4,
             watch_percent = GREATEST(user_lesson_progress.watch_percent, $5),
             is_completed = user_lesson_progress.is_completed OR $6,
             completed_at = CASE WHEN NOT user_lesson_progress.is_completed AND $6 THEN NOW() ELSE user_lesson_progress.completed_at END,
             last_watched_at = NOW()"#
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
async fn get_user_progress(
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
            .unwrap_or_default();

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

// ═══════════════════════════════════════════════════════════════════════════════════
// UTILITY ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get course categories
async fn get_categories(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let categories: Vec<(String, i64)> = sqlx::query_as(
        "SELECT category, COUNT(*) FROM courses_enhanced WHERE category IS NOT NULL AND deleted_at IS NULL GROUP BY category ORDER BY category"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let cat_list: Vec<serde_json::Value> = categories
        .iter()
        .map(|(cat, count)| json!({"name": cat, "count": count}))
        .collect();

    Ok(Json(json!({"categories": cat_list})))
}

/// Get course stats
async fn get_course_stats(
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
async fn clone_course(
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
        r#"INSERT INTO courses_enhanced
           (title, slug, subtitle, description, description_html, thumbnail_url,
            trailer_video_url, difficulty_level, category, tags, instructor_id,
            is_published, is_free, required_plan_id, price_cents, certificate_enabled,
            completion_threshold_percent)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, false, $12, $13, $14, $15, $16)
           RETURNING *"#,
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
            .unwrap_or_default();

    let mut section_map: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    for section in &sections {
        let new_section: (i64,) = sqlx::query_as(
            r#"INSERT INTO course_sections
               (course_id, title, description, sort_order, section_type, unlock_type, is_published)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id"#,
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
            .unwrap_or_default();

    for lesson in &lessons {
        if let Some(&new_section_id) = section_map.get(&lesson.section_id) {
            sqlx::query(
                r#"INSERT INTO course_lessons
                   (course_id, section_id, title, description, content_html, video_url,
                    bunny_video_guid, bunny_library_id, duration_seconds, thumbnail_url,
                    sort_order, lesson_type, is_preview, is_published, completion_type, required_watch_percent)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"#
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

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER SETUP
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Courses
        .route("/", get(list_courses).post(create_course))
        .route("/stats", get(get_course_stats))
        .route("/categories", get(get_categories))
        .route(
            "/:course_id",
            get(get_course).put(update_course).delete(delete_course),
        )
        .route("/:course_id/clone", post(clone_course))
        // Sections
        .route("/:course_id/sections", post(create_section))
        .route("/:course_id/sections/reorder", put(reorder_sections))
        .route(
            "/:course_id/sections/:section_id",
            put(update_section).delete(delete_section),
        )
        // Lessons
        .route("/:course_id/lessons", post(create_lesson))
        .route(
            "/:course_id/lessons/:lesson_id",
            put(update_lesson).delete(delete_lesson),
        )
        .route(
            "/:course_id/sections/:section_id/lessons/reorder",
            put(reorder_lessons),
        )
        // Resources
        .route("/:course_id/resources", post(create_resource))
        .route(
            "/:course_id/resources/:resource_id",
            delete(delete_resource),
        )
        // Live Sessions
        .route("/:course_id/live-sessions", post(create_live_session))
        .route(
            "/:course_id/live-sessions/bulk",
            post(bulk_create_live_sessions),
        )
        .route(
            "/:course_id/live-sessions/:session_id",
            delete(delete_live_session),
        )
        // Enrollments & Progress
        .route("/:course_id/enroll", post(enroll_user))
        .route("/:course_id/enrollments", get(get_enrollments))
        .route(
            "/:course_id/lessons/:lesson_id/progress",
            post(update_lesson_progress),
        )
        .route("/:course_id/progress/:user_id", get(get_user_progress))
}
