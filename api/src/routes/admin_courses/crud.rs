//! Course CRUD lifecycle handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::models::course::{
    Course, CourseDownload, CourseQueryParams, CreateCourseRequest, LessonListItem,
    ModuleWithLessons, UpdateCourseRequest,
};
use crate::routes::admin_courses::slugify;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn list_courses(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<CourseQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // ICT 7 Grade: SECURE parameterized query with proper SQL injection prevention
    // All user inputs are bound as parameters, never interpolated into SQL
    let safe_sort_by = match params.sort_by.as_deref() {
        Some("title") => "title",
        Some("level") => "level",
        Some("price_cents") => "price_cents",
        _ => "created_at",
    };
    let safe_sort_order = match params.sort_order.as_deref() {
        Some(s) if s.eq_ignore_ascii_case("ASC") => "ASC",
        _ => "DESC",
    };

    // Build safe query with parameterized bindings
    let query = format!(
        r"
        SELECT id, title, slug, description, price_cents, is_published,
               thumbnail, level, created_at
        FROM courses
        WHERE ($1::text IS NULL OR level = $1)
          AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR description ILIKE '%' || $2 || '%')
        ORDER BY {safe_sort_by} {safe_sort_order}
        LIMIT $3 OFFSET $4
        "
    );

    // Query base course data with parameterized values
    #[allow(clippy::type_complexity)]
    let rows: Vec<(
        Uuid,
        String,
        String,
        Option<String>,
        i32,
        bool,
        Option<String>,
        Option<String>,
        NaiveDateTime,
    )> = sqlx::query_as(&query)
        .bind(params.level.as_deref())
        .bind(params.search.as_deref())
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Admin courses list query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    // Build response with defaults for enhanced fields
    let courses: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            json!({
                "id": row.0,
                "title": row.1,
                "slug": row.2,
                "description": row.3,
                "card_image_url": row.6,  // thumbnail
                "card_description": row.3, // use description as fallback
                "card_badge": null,
                "card_badge_color": "#10b981",
                "instructor_name": null,
                "instructor_avatar_url": null,
                "level": row.7.as_deref().unwrap_or("beginner"),
                "price_cents": row.4,
                "is_free": row.4 == 0,
                "is_published": row.5,
                "status": if row.5 { "published" } else { "draft" },
                "module_count": 0,
                "lesson_count": 0,
                "total_duration_minutes": 0,
                "enrollment_count": 0,
                "avg_rating": null,
                "review_count": 0,
                "created_at": row.8.format("%Y-%m-%dT%H:%M:%S").to_string()
            })
        })
        .collect();

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM courses")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as i32;

    Ok(Json(json!({
        "success": true,
        "data": {
            "courses": courses,
            "total": total.0,
            "page": page,
            "per_page": per_page,
            "total_pages": total_pages
        }
    })))
}

pub(super) async fn get_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = if let Ok(uuid) = Uuid::parse_str(&id) {
        sqlx::query_as(
            r"
            SELECT * FROM courses WHERE id = $1
            ",
        )
        .bind(uuid)
        .fetch_optional(&state.db.pool)
        .await
    } else {
        sqlx::query_as(
            r"
            SELECT * FROM courses WHERE slug = $1
            ",
        )
        .bind(&id)
        .fetch_optional(&state.db.pool)
        .await
    }
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

    let modules: Vec<crate::models::course::CourseModule> = sqlx::query_as(
        r"
        SELECT * FROM course_modules_v2
        WHERE course_id = $1
        ORDER BY sort_order
        ",
    )
    .bind(course.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let lessons: Vec<LessonListItem> = sqlx::query_as(
        r"
        SELECT id, course_id, module_id, title, slug, description, duration_minutes,
               position, sort_order, is_free, is_preview, is_published,
               bunny_video_guid, thumbnail_url
        FROM lessons
        WHERE course_id = $1
        ORDER BY COALESCE(sort_order, position)
        ",
    )
    .bind(course.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r"
        SELECT * FROM course_downloads
        WHERE course_id = $1
        ORDER BY sort_order
        ",
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

    let unassigned_lessons: Vec<LessonListItem> = lessons
        .into_iter()
        .filter(|l| l.module_id.is_none())
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "course": course,
            "modules": modules_with_lessons,
            "unassigned_lessons": unassigned_lessons,
            "downloads": downloads
        }
    })))
}

pub(super) async fn create_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateCourseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.title));

    let existing: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM courses WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "A course with this slug already exists"})),
        ));
    }

    let course: Course = sqlx::query_as(
        r"
        INSERT INTO courses (
            id, title, slug, description, card_description, card_image_url,
            card_badge, card_badge_color, price_cents, is_free, level,
            instructor_name, instructor_title, instructor_avatar_url, instructor_bio,
            what_you_learn, requirements, target_audience,
            meta_title, meta_description, status, is_published
        ) VALUES (
            gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, 'draft', false
        )
        RETURNING *
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.card_description)
    .bind(&input.card_image_url)
    .bind(&input.card_badge)
    .bind(&input.card_badge_color)
    .bind(input.price_cents.unwrap_or(0))
    .bind(input.is_free.unwrap_or(false))
    .bind(&input.level)
    .bind(&input.instructor_name)
    .bind(&input.instructor_title)
    .bind(&input.instructor_avatar_url)
    .bind(&input.instructor_bio)
    .bind(json!(input.what_you_learn.unwrap_or_default()))
    .bind(json!(input.requirements.unwrap_or_default()))
    .bind(json!(input.target_audience.unwrap_or_default()))
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create course: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course created successfully",
        "data": course
    })))
}

pub(super) async fn update_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCourseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates = Vec::new();
    let mut param_count = 1;

    macro_rules! add_update {
        ($field:expr, $value:expr) => {
            if let Some(ref v) = $value {
                updates.push(format!("{} = ${}", $field, param_count));
                param_count += 1;
                let _ = v;
            }
        };
    }

    add_update!("title", input.title);
    add_update!("slug", input.slug);
    add_update!("description", input.description);
    add_update!("card_description", input.card_description);
    add_update!("card_image_url", input.card_image_url);
    add_update!("card_badge", input.card_badge);
    add_update!("card_badge_color", input.card_badge_color);
    add_update!("level", input.level);
    add_update!("instructor_name", input.instructor_name);
    add_update!("instructor_title", input.instructor_title);
    add_update!("instructor_avatar_url", input.instructor_avatar_url);
    add_update!("instructor_bio", input.instructor_bio);
    add_update!("meta_title", input.meta_title);
    add_update!("meta_description", input.meta_description);
    add_update!("status", input.status);

    if input.price_cents.is_some() {
        updates.push(format!("price_cents = ${param_count}"));
        param_count += 1;
    }

    if input.is_free.is_some() {
        updates.push(format!("is_free = ${param_count}"));
        param_count += 1;
    }

    if input.is_published.is_some() {
        updates.push(format!("is_published = ${param_count}"));
        param_count += 1;
        if input.is_published == Some(true) {
            updates.push("published_at = NOW()".to_string());
            updates.push("status = 'published'".to_string());
        }
    }

    if input.bunny_library_id.is_some() {
        updates.push(format!("bunny_library_id = ${param_count}"));
        param_count += 1;
    }

    if input.what_you_learn.is_some() {
        updates.push(format!("what_you_learn = ${param_count}"));
        param_count += 1;
    }

    if input.requirements.is_some() {
        updates.push(format!("requirements = ${param_count}"));
        param_count += 1;
    }

    if input.target_audience.is_some() {
        updates.push(format!("target_audience = ${param_count}"));
        param_count += 1;
    }

    updates.push("updated_at = NOW()".to_string());

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let query = format!(
        "UPDATE courses SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );

    let mut q = sqlx::query_as::<_, Course>(&query);

    if let Some(ref v) = input.title {
        q = q.bind(v);
    }
    if let Some(ref v) = input.slug {
        q = q.bind(v);
    }
    if let Some(ref v) = input.description {
        q = q.bind(v);
    }
    if let Some(ref v) = input.card_description {
        q = q.bind(v);
    }
    if let Some(ref v) = input.card_image_url {
        q = q.bind(v);
    }
    if let Some(ref v) = input.card_badge {
        q = q.bind(v);
    }
    if let Some(ref v) = input.card_badge_color {
        q = q.bind(v);
    }
    if let Some(ref v) = input.level {
        q = q.bind(v);
    }
    if let Some(ref v) = input.instructor_name {
        q = q.bind(v);
    }
    if let Some(ref v) = input.instructor_title {
        q = q.bind(v);
    }
    if let Some(ref v) = input.instructor_avatar_url {
        q = q.bind(v);
    }
    if let Some(ref v) = input.instructor_bio {
        q = q.bind(v);
    }
    if let Some(ref v) = input.meta_title {
        q = q.bind(v);
    }
    if let Some(ref v) = input.meta_description {
        q = q.bind(v);
    }
    if let Some(ref v) = input.status {
        q = q.bind(v);
    }
    if let Some(v) = input.price_cents {
        q = q.bind(v);
    }
    if let Some(v) = input.is_free {
        q = q.bind(v);
    }
    if let Some(v) = input.is_published {
        q = q.bind(v);
    }
    if let Some(v) = input.bunny_library_id {
        q = q.bind(v);
    }
    if let Some(ref v) = input.what_you_learn {
        q = q.bind(json!(v));
    }
    if let Some(ref v) = input.requirements {
        q = q.bind(json!(v));
    }
    if let Some(ref v) = input.target_audience {
        q = q.bind(json!(v));
    }

    q = q.bind(id);

    let course = q.fetch_one(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update course: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course updated successfully",
        "data": course
    })))
}

pub(super) async fn delete_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM courses WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete course: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course deleted successfully"
    })))
}

pub(super) async fn publish_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r"
        UPDATE courses
        SET is_published = true, status = 'published', published_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to publish course: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course published successfully",
        "data": course
    })))
}

pub(super) async fn unpublish_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r"
        UPDATE courses
        SET is_published = false, status = 'draft', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to unpublish course: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course unpublished successfully",
        "data": course
    })))
}

/// Archive a course (soft delete - keeps data but removes from listings)
pub(super) async fn archive_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r"
        UPDATE courses
        SET is_published = false, status = 'archived', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to archive course: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course archived successfully",
        "data": course
    })))
}

/// Restore an archived course back to draft
pub(super) async fn restore_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r"
        UPDATE courses
        SET status = 'draft', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to restore course: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Course restored to draft successfully",
        "data": course
    })))
}
