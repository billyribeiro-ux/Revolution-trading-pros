//! Course Management Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::models::course::{
    Course, CourseDownload, CourseListItem, CourseModule, CourseQueryParams, CreateCourseRequest,
    CreateDownloadRequest, CreateLessonRequest, CreateModuleRequest, Lesson, LessonListItem,
    ModuleWithLessons, PaginatedCourses, ReorderRequest, UpdateCourseRequest,
    UpdateDownloadRequest, UpdateLessonRequest, UpdateModuleRequest,
};
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_courses(
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
        r#"
        SELECT id, title, slug, description, price_cents, is_published,
               thumbnail, level, created_at
        FROM courses
        WHERE ($1::text IS NULL OR level = $1)
          AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR description ILIKE '%' || $2 || '%')
        ORDER BY {} {}
        LIMIT $3 OFFSET $4
        "#,
        safe_sort_by, safe_sort_order
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

async fn get_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = if let Ok(uuid) = Uuid::parse_str(&id) {
        sqlx::query_as(
            r#"
            SELECT * FROM courses WHERE id = $1
            "#,
        )
        .bind(uuid)
        .fetch_optional(&state.db.pool)
        .await
    } else {
        sqlx::query_as(
            r#"
            SELECT * FROM courses WHERE slug = $1
            "#,
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

    let modules: Vec<CourseModule> = sqlx::query_as(
        r#"
        SELECT * FROM course_modules_v2 
        WHERE course_id = $1 
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
        WHERE course_id = $1 
        ORDER BY COALESCE(sort_order, position)
        "#,
    )
    .bind(course.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r#"
        SELECT * FROM course_downloads 
        WHERE course_id = $1 
        ORDER BY sort_order
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

async fn create_course(
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
        r#"
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
        "#,
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

async fn update_course(
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
        updates.push(format!("price_cents = ${}", param_count));
        param_count += 1;
    }

    if input.is_free.is_some() {
        updates.push(format!("is_free = ${}", param_count));
        param_count += 1;
    }

    if input.is_published.is_some() {
        updates.push(format!("is_published = ${}", param_count));
        param_count += 1;
        if input.is_published == Some(true) {
            updates.push("published_at = NOW()".to_string());
            updates.push("status = 'published'".to_string());
        }
    }

    if input.bunny_library_id.is_some() {
        updates.push(format!("bunny_library_id = ${}", param_count));
        param_count += 1;
    }

    if input.what_you_learn.is_some() {
        updates.push(format!("what_you_learn = ${}", param_count));
        param_count += 1;
    }

    if input.requirements.is_some() {
        updates.push(format!("requirements = ${}", param_count));
        param_count += 1;
    }

    if input.target_audience.is_some() {
        updates.push(format!("target_audience = ${}", param_count));
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

async fn delete_course(
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

async fn publish_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r#"
        UPDATE courses 
        SET is_published = true, status = 'published', published_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
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

async fn unpublish_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r#"
        UPDATE courses 
        SET is_published = false, status = 'draft', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
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
async fn archive_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r#"
        UPDATE courses
        SET is_published = false, status = 'archived', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
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
async fn restore_course(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r#"
        UPDATE courses
        SET status = 'draft', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
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

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(serde::Deserialize)]
struct EnrollUserRequest {
    user_id: i64,
    is_lifetime_access: Option<bool>,
    access_days: Option<i32>,
    notes: Option<String>,
}

/// Enroll a user in a course (admin action)
async fn enroll_user(
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
        r#"
        INSERT INTO user_course_enrollments (
            user_id, course_id, status, progress_percent, is_lifetime_access,
            access_expires_at, enrolled_at
        ) VALUES ($1, $2, 'active', 0, $3, $4, NOW())
        RETURNING id, course_id, user_id, progress_percent, status, enrolled_at
        "#,
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
async fn list_enrollments(
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
        r#"
        SELECT e.id, e.user_id, e.progress_percent, e.status, e.enrolled_at,
               e.completed_at, e.certificate_issued
        FROM user_course_enrollments e
        WHERE e.course_id = $1
        ORDER BY e.enrolled_at DESC
        "#,
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
async fn remove_enrollment(
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
// MODULE CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_modules(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let modules: Vec<CourseModule> = sqlx::query_as(
        r#"
        SELECT * FROM course_modules_v2 
        WHERE course_id = $1 
        ORDER BY sort_order
        "#,
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": modules
    })))
}

async fn create_module(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<CreateModuleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_modules_v2 WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = input.sort_order.unwrap_or(max_order.0.unwrap_or(0) + 1);

    let module: CourseModule = sqlx::query_as(
        r#"
        INSERT INTO course_modules_v2 (course_id, title, description, sort_order, is_published, drip_enabled, drip_days)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(course_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(sort_order)
    .bind(input.is_published.unwrap_or(true))
    .bind(input.drip_enabled.unwrap_or(false))
    .bind(input.drip_days.unwrap_or(0))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create module: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Module created successfully",
        "data": module
    })))
}

async fn update_module(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, module_id)): Path<(Uuid, i64)>,
    Json(input): Json<UpdateModuleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let module: CourseModule = sqlx::query_as(
        r#"
        UPDATE course_modules_v2 SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            sort_order = COALESCE($3, sort_order),
            is_published = COALESCE($4, is_published),
            drip_enabled = COALESCE($5, drip_enabled),
            drip_days = COALESCE($6, drip_days),
            updated_at = NOW()
        WHERE id = $7 AND course_id = $8
        RETURNING *
        "#,
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.sort_order)
    .bind(input.is_published)
    .bind(input.drip_enabled)
    .bind(input.drip_days)
    .bind(module_id)
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update module: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Module updated successfully",
        "data": module
    })))
}

async fn delete_module(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, module_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE lessons SET module_id = NULL WHERE module_id = $1")
        .bind(module_id)
        .execute(&state.db.pool)
        .await
        .ok();

    sqlx::query("DELETE FROM course_modules_v2 WHERE id = $1 AND course_id = $2")
        .bind(module_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete module: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Module deleted successfully"
    })))
}

async fn reorder_modules(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<ReorderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in input.items {
        let module_id: i64 = item.id.parse().unwrap_or(0);
        sqlx::query(
            "UPDATE course_modules_v2 SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3",
        )
        .bind(item.sort_order)
        .bind(module_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({
        "success": true,
        "message": "Modules reordered successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LESSON CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_lessons(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lessons: Vec<LessonListItem> = sqlx::query_as(
        r#"
        SELECT id, course_id, module_id, title, slug, description, duration_minutes,
               position, sort_order, is_free, is_preview, is_published, 
               bunny_video_guid, thumbnail_url
        FROM lessons 
        WHERE course_id = $1 
        ORDER BY COALESCE(sort_order, position)
        "#,
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": lessons
    })))
}

async fn get_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lesson: Lesson = sqlx::query_as(
        r#"
        SELECT * FROM lessons WHERE id = $1 AND course_id = $2
        "#,
    )
    .bind(lesson_id)
    .bind(course_id)
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
            Json(json!({"error": "Lesson not found"})),
        )
    })?;

    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r#"
        SELECT * FROM course_downloads WHERE lesson_id = $1 ORDER BY sort_order
        "#,
    )
    .bind(lesson_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "lesson": lesson,
            "downloads": downloads
        }
    })))
}

async fn create_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<CreateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.title));

    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(COALESCE(sort_order, position)) FROM lessons WHERE course_id = $1",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = input.sort_order.unwrap_or(max_order.0.unwrap_or(0) + 1);

    let lesson: Lesson = sqlx::query_as(
        r#"
        INSERT INTO lessons (
            id, course_id, module_id, title, slug, description, video_url,
            bunny_video_guid, thumbnail_url, duration_minutes, content_html,
            is_free, is_preview, is_published, sort_order, position, drip_days
        ) VALUES (
            gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $14, $15
        )
        RETURNING *
        "#,
    )
    .bind(course_id)
    .bind(input.module_id)
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.duration_minutes.unwrap_or(0))
    .bind(&input.content_html)
    .bind(input.is_free.unwrap_or(false))
    .bind(input.is_preview.unwrap_or(false))
    .bind(input.is_published.unwrap_or(true))
    .bind(sort_order)
    .bind(input.drip_days.unwrap_or(0))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create lesson: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Lesson created successfully",
        "data": lesson
    })))
}

async fn update_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UpdateLessonRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lesson: Lesson = sqlx::query_as(
        r#"
        UPDATE lessons SET
            title = COALESCE($1, title),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            module_id = COALESCE($4, module_id),
            video_url = COALESCE($5, video_url),
            bunny_video_guid = COALESCE($6, bunny_video_guid),
            thumbnail_url = COALESCE($7, thumbnail_url),
            duration_minutes = COALESCE($8, duration_minutes),
            content_html = COALESCE($9, content_html),
            is_free = COALESCE($10, is_free),
            is_preview = COALESCE($11, is_preview),
            is_published = COALESCE($12, is_published),
            sort_order = COALESCE($13, sort_order),
            drip_days = COALESCE($14, drip_days),
            updated_at = NOW()
        WHERE id = $15 AND course_id = $16
        RETURNING *
        "#,
    )
    .bind(&input.title)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(input.module_id)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.duration_minutes)
    .bind(&input.content_html)
    .bind(input.is_free)
    .bind(input.is_preview)
    .bind(input.is_published)
    .bind(input.sort_order)
    .bind(input.drip_days)
    .bind(lesson_id)
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update lesson: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Lesson updated successfully",
        "data": lesson
    })))
}

async fn delete_lesson(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM lessons WHERE id = $1 AND course_id = $2")
        .bind(lesson_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete lesson: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Lesson deleted successfully"
    })))
}

async fn reorder_lessons(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<ReorderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in input.items {
        let lesson_id = Uuid::parse_str(&item.id).ok();
        if let Some(lid) = lesson_id {
            sqlx::query(
                "UPDATE lessons SET sort_order = $1, position = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3",
            )
            .bind(item.sort_order)
            .bind(lid)
            .bind(course_id)
            .execute(&state.db.pool)
            .await
            .ok();
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "Lessons reordered successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DOWNLOAD CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_downloads(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r#"
        SELECT * FROM course_downloads 
        WHERE course_id = $1 OR lesson_id IN (SELECT id FROM lessons WHERE course_id = $1)
        ORDER BY sort_order
        "#,
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": downloads
    })))
}

async fn create_download(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<CreateDownloadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let download: CourseDownload = sqlx::query_as(
        r#"
        INSERT INTO course_downloads (
            course_id, module_id, lesson_id, title, description, file_name, file_path,
            file_size_bytes, file_type, mime_type, download_url, category, sort_order, is_public
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING *
        "#,
    )
    .bind(input.course_id.or(Some(course_id)))
    .bind(input.module_id)
    .bind(input.lesson_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.file_name)
    .bind(&input.file_path)
    .bind(input.file_size_bytes)
    .bind(&input.file_type)
    .bind(&input.mime_type)
    .bind(&input.download_url)
    .bind(input.category.as_deref().unwrap_or("resource"))
    .bind(input.sort_order.unwrap_or(0))
    .bind(input.is_public.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create download: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Download created successfully",
        "data": download
    })))
}

async fn update_download(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, download_id)): Path<(Uuid, i64)>,
    Json(input): Json<UpdateDownloadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let download: CourseDownload = sqlx::query_as(
        r#"
        UPDATE course_downloads SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            category = COALESCE($3, category),
            sort_order = COALESCE($4, sort_order),
            is_public = COALESCE($5, is_public),
            updated_at = NOW()
        WHERE id = $6 AND (course_id = $7 OR lesson_id IN (SELECT id FROM lessons WHERE course_id = $7))
        RETURNING *
        "#,
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.category)
    .bind(input.sort_order)
    .bind(input.is_public)
    .bind(download_id)
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update download: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Download updated successfully",
        "data": download
    })))
}

async fn delete_download(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, download_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        "DELETE FROM course_downloads WHERE id = $1 AND (course_id = $2 OR lesson_id IN (SELECT id FROM lessons WHERE course_id = $2))",
    )
    .bind(download_id)
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to delete download: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Download deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO UPLOAD (Bunny Stream TUS)
// ═══════════════════════════════════════════════════════════════════════════════════

async fn create_video_upload(
    _admin: AdminUser,
    State(_state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = input["title"].as_str().unwrap_or("Untitled Video");

    let library_id = std::env::var("BUNNY_STREAM_LIBRARY_ID")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let api_key = std::env::var("BUNNY_STREAM_API_KEY")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if library_id.is_empty() || api_key.is_empty() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Bunny.net Stream not configured"})),
        ));
    }

    // Create video in Bunny Stream
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://video.bunnycdn.com/library/{}/videos",
            library_id
        ))
        .header("AccessKey", &api_key)
        .header("Content-Type", "application/json")
        .json(&json!({ "title": title }))
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create video: {}", e)})),
            )
        })?;

    if !response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": format!("Bunny API error: {}", text)})),
        ));
    }

    let video: serde_json::Value = response.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to parse response: {}", e)})),
        )
    })?;

    let video_guid = video["guid"].as_str().unwrap_or("");
    let expiry = chrono::Utc::now().timestamp() + 7200; // 2 hour expiry

    // Generate TUS upload URL with authentication
    let tus_url = format!(
        "https://video.bunnycdn.com/tusupload?libraryId={}&videoId={}&expirationTime={}",
        library_id, video_guid, expiry
    );

    // Generate authorization signature for TUS
    let signature_string = format!("{}{}{}", library_id, api_key, expiry);
    let signature = format!("{:x}", md5::compute(signature_string.as_bytes()));

    Ok(Json(json!({
        "success": true,
        "data": {
            "video_guid": video_guid,
            "library_id": library_id,
            "tus_upload_url": tus_url,
            "auth_signature": signature,
            "auth_expiry": expiry,
            "embed_url": format!("https://iframe.mediadelivery.net/embed/{}/{}", library_id, video_guid),
            "play_url": format!("https://iframe.mediadelivery.net/play/{}/{}", library_id, video_guid),
            "thumbnail_url": format!("https://vz-{}.b-cdn.net/{}/thumbnail.jpg", library_id, video_guid)
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// FILE UPLOAD URL (Bunny Storage)
// ═══════════════════════════════════════════════════════════════════════════════════

async fn get_upload_url(
    _admin: AdminUser,
    State(_state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let file_name = input["file_name"].as_str().unwrap_or("file");
    let file_type = input["file_type"]
        .as_str()
        .unwrap_or("application/octet-stream");

    let storage_zone =
        std::env::var("BUNNY_STORAGE_ZONE").unwrap_or_else(|_| "revolution-downloads".to_string());
    let cdn_url = std::env::var("BUNNY_CDN_URL")
        .unwrap_or_else(|_| "https://revolution-downloads.b-cdn.net".to_string());

    let file_path = format!("courses/{}/{}", course_id, file_name);
    let upload_url = format!(
        "https://storage.bunnycdn.com/{}/{}",
        storage_zone, file_path
    );
    let download_url = format!("{}/{}", cdn_url, file_path);

    // SECURITY: Do NOT return BUNNY_STORAGE_API_KEY to the client.
    // File uploads should go through a backend proxy endpoint instead of
    // exposing storage credentials to the frontend.
    Ok(Json(json!({
        "success": true,
        "data": {
            "upload_url": upload_url,
            "download_url": download_url,
            "file_path": file_path,
            "storage_zone": storage_zone,
            "content_type": file_type
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// QUIZ CRUD - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all quizzes for a course
async fn list_quizzes(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let quizzes: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, Option<String>, Option<Uuid>, Option<i64>, String, Option<i32>, bool, i32, NaiveDateTime)>(
        r#"
        SELECT id, title, description, lesson_id, module_id, quiz_type, passing_score, is_published, sort_order, created_at
        FROM course_quizzes WHERE course_id = $1 ORDER BY sort_order
        "#,
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| json!({
        "id": q.0,
        "title": q.1,
        "description": q.2,
        "lesson_id": q.3,
        "module_id": q.4,
        "quiz_type": q.5,
        "passing_score": q.6,
        "is_published": q.7,
        "sort_order": q.8,
        "created_at": q.9.format("%Y-%m-%dT%H:%M:%S").to_string()
    }))
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": quizzes
    })))
}

/// Create a new quiz
async fn create_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = input["title"].as_str().unwrap_or("Untitled Quiz");
    let description = input["description"].as_str();
    let lesson_id = input["lesson_id"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok());
    let module_id = input["module_id"].as_i64();
    let quiz_type = input["quiz_type"].as_str().unwrap_or("graded");
    let passing_score = input["passing_score"]
        .as_i64()
        .map(|v| v as i32)
        .unwrap_or(70);
    let time_limit = input["time_limit_minutes"].as_i64().map(|v| v as i32);
    let max_attempts = input["max_attempts"].as_i64().map(|v| v as i32);
    let shuffle_questions = input["shuffle_questions"].as_bool().unwrap_or(false);
    let shuffle_answers = input["shuffle_answers"].as_bool().unwrap_or(false);
    let show_correct = input["show_correct_answers"].as_bool().unwrap_or(true);
    let is_required = input["is_required"].as_bool().unwrap_or(false);

    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_quizzes WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let quiz = sqlx::query_as::<_, (i64, String, String, bool, NaiveDateTime)>(
        r#"
        INSERT INTO course_quizzes (
            course_id, lesson_id, module_id, title, description, quiz_type,
            passing_score, time_limit_minutes, max_attempts, shuffle_questions,
            shuffle_answers, show_correct_answers, is_required, is_published, sort_order
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, false, $14)
        RETURNING id, title, quiz_type, is_published, created_at
        "#,
    )
    .bind(course_id)
    .bind(lesson_id)
    .bind(module_id)
    .bind(title)
    .bind(description)
    .bind(quiz_type)
    .bind(passing_score)
    .bind(time_limit)
    .bind(max_attempts)
    .bind(shuffle_questions)
    .bind(shuffle_answers)
    .bind(show_correct)
    .bind(is_required)
    .bind(max_order.0.unwrap_or(0) + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create quiz: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Quiz created successfully",
        "data": {
            "id": quiz.0,
            "title": quiz.1,
            "quiz_type": quiz.2,
            "is_published": quiz.3,
            "created_at": quiz.4.format("%Y-%m-%dT%H:%M:%S").to_string()
        }
    })))
}

/// Get quiz with questions
async fn get_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let quiz: serde_json::Value = sqlx::query_as::<
        _,
        (
            i64,
            Uuid,
            String,
            Option<String>,
            String,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            bool,
            bool,
            bool,
            bool,
            bool,
            i32,
        ),
    >(
        r#"
        SELECT id, course_id, title, description, quiz_type, passing_score, time_limit_minutes,
               max_attempts, shuffle_questions, shuffle_answers, show_correct_answers, is_required,
               is_published, sort_order
        FROM course_quizzes WHERE id = $1 AND course_id = $2
        "#,
    )
    .bind(quiz_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .map(|q| {
        json!({
            "id": q.0, "course_id": q.1, "title": q.2, "description": q.3, "quiz_type": q.4,
            "passing_score": q.5, "time_limit_minutes": q.6, "max_attempts": q.7,
            "shuffle_questions": q.8, "shuffle_answers": q.9, "show_correct_answers": q.10,
            "is_required": q.11, "is_published": q.12, "sort_order": q.13
        })
    })
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Quiz not found"})),
        )
    })?;

    let questions: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, String, Option<String>, i32, i32)>(
        r#"SELECT id, question_type, question_text, explanation, points, sort_order
           FROM quiz_questions WHERE quiz_id = $1 ORDER BY sort_order"#,
    )
    .bind(quiz_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| json!({"id": q.0, "question_type": q.1, "question_text": q.2, "explanation": q.3, "points": q.4, "sort_order": q.5}))
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "quiz": quiz,
            "questions": questions
        }
    })))
}

/// Update quiz
async fn update_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r#"
        UPDATE course_quizzes SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            quiz_type = COALESCE($3, quiz_type),
            passing_score = COALESCE($4, passing_score),
            time_limit_minutes = COALESCE($5, time_limit_minutes),
            max_attempts = COALESCE($6, max_attempts),
            shuffle_questions = COALESCE($7, shuffle_questions),
            shuffle_answers = COALESCE($8, shuffle_answers),
            show_correct_answers = COALESCE($9, show_correct_answers),
            is_required = COALESCE($10, is_required),
            is_published = COALESCE($11, is_published),
            updated_at = NOW()
        WHERE id = $12 AND course_id = $13
        "#,
    )
    .bind(input["title"].as_str())
    .bind(input["description"].as_str())
    .bind(input["quiz_type"].as_str())
    .bind(input["passing_score"].as_i64().map(|v| v as i32))
    .bind(input["time_limit_minutes"].as_i64().map(|v| v as i32))
    .bind(input["max_attempts"].as_i64().map(|v| v as i32))
    .bind(input["shuffle_questions"].as_bool())
    .bind(input["shuffle_answers"].as_bool())
    .bind(input["show_correct_answers"].as_bool())
    .bind(input["is_required"].as_bool())
    .bind(input["is_published"].as_bool())
    .bind(quiz_id)
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update quiz: {}", e)})),
        )
    })?;

    Ok(Json(
        json!({"success": true, "message": "Quiz updated successfully"}),
    ))
}

/// Delete quiz
async fn delete_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM course_quizzes WHERE id = $1 AND course_id = $2")
        .bind(quiz_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete quiz: {}", e)})),
            )
        })?;

    Ok(Json(
        json!({"success": true, "message": "Quiz deleted successfully"}),
    ))
}

/// Add question to quiz
async fn add_quiz_question(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let question_type = input["question_type"].as_str().unwrap_or("multiple_choice");
    let question_text = input["question_text"].as_str().unwrap_or("");
    let explanation = input["explanation"].as_str();
    let points = input["points"].as_i64().unwrap_or(1) as i32;

    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM quiz_questions WHERE quiz_id = $1")
            .bind(quiz_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let question = sqlx::query_as::<_, (i64, String, String, i32)>(
        r#"
        INSERT INTO quiz_questions (quiz_id, question_type, question_text, explanation, points, sort_order)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, question_type, question_text, points
        "#,
    )
    .bind(quiz_id)
    .bind(question_type)
    .bind(question_text)
    .bind(explanation)
    .bind(points)
    .bind(max_order.0.unwrap_or(0) + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to add question: {}", e)}))))?;

    // Add answers if provided
    if let Some(answers) = input["answers"].as_array() {
        for (i, answer) in answers.iter().enumerate() {
            let _ = sqlx::query(
                "INSERT INTO quiz_answers (question_id, answer_text, is_correct, sort_order, feedback) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(question.0)
            .bind(answer["answer_text"].as_str().unwrap_or(""))
            .bind(answer["is_correct"].as_bool().unwrap_or(false))
            .bind(i as i32)
            .bind(answer["feedback"].as_str())
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "Question added successfully",
        "data": {"id": question.0, "question_type": question.1, "question_text": question.2, "points": question.3}
    })))
}

/// Delete question from quiz
async fn delete_quiz_question(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((_course_id, _quiz_id, question_id)): Path<(Uuid, i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM quiz_questions WHERE id = $1")
        .bind(question_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete question: {}", e)})),
            )
        })?;

    Ok(Json(
        json!({"success": true, "message": "Question deleted successfully"}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// CATEGORIES & TAGS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all categories
async fn list_categories(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let categories: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, String, Option<String>, Option<String>, Option<i64>, i32, Option<bool>, Option<i32>)>(
        "SELECT id, name, slug, description, color, parent_id, sort_order, is_featured, course_count FROM course_categories ORDER BY sort_order"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|c| json!({"id": c.0, "name": c.1, "slug": c.2, "description": c.3, "color": c.4, "parent_id": c.5, "sort_order": c.6, "is_featured": c.7, "course_count": c.8}))
    .collect();

    Ok(Json(json!({"success": true, "data": categories})))
}

/// Create category
async fn create_category(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let name = input["name"].as_str().ok_or((
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "Name is required"})),
    ))?;
    let slug = input["slug"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| slugify(name));

    let max_order: (Option<i32>,) = sqlx::query_as("SELECT MAX(sort_order) FROM course_categories")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((None,));

    let category = sqlx::query_as::<_, (i64, String, String)>(
        "INSERT INTO course_categories (name, slug, description, color, parent_id, is_featured, sort_order) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, name, slug"
    )
    .bind(name)
    .bind(&slug)
    .bind(input["description"].as_str())
    .bind(input["color"].as_str())
    .bind(input["parent_id"].as_i64())
    .bind(input["is_featured"].as_bool().unwrap_or(false))
    .bind(max_order.0.unwrap_or(0) + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to create category: {}", e)}))))?;

    Ok(Json(
        json!({"success": true, "message": "Category created", "data": {"id": category.0, "name": category.1, "slug": category.2}}),
    ))
}

/// Delete category
async fn delete_category(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(category_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM course_categories WHERE id = $1")
        .bind(category_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete category: {}", e)})),
            )
        })?;

    Ok(Json(
        json!({"success": true, "message": "Category deleted"}),
    ))
}

/// List all tags
async fn list_tags(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tags: Vec<serde_json::Value> =
        sqlx::query_as::<_, (i64, String, String, Option<String>, Option<i32>)>(
            "SELECT id, name, slug, color, course_count FROM course_tags ORDER BY name",
        )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|t| json!({"id": t.0, "name": t.1, "slug": t.2, "color": t.3, "course_count": t.4}))
        .collect();

    Ok(Json(json!({"success": true, "data": tags})))
}

/// Create tag
async fn create_tag(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let name = input["name"].as_str().ok_or((
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "Name is required"})),
    ))?;
    let slug = input["slug"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| slugify(name));

    let tag = sqlx::query_as::<_, (i64, String, String)>(
        "INSERT INTO course_tags (name, slug, color) VALUES ($1, $2, $3) RETURNING id, name, slug",
    )
    .bind(name)
    .bind(&slug)
    .bind(input["color"].as_str())
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create tag: {}", e)})),
        )
    })?;

    Ok(Json(
        json!({"success": true, "message": "Tag created", "data": {"id": tag.0, "name": tag.1, "slug": tag.2}}),
    ))
}

/// Delete tag
async fn delete_tag(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(tag_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM course_tags WHERE id = $1")
        .bind(tag_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete tag: {}", e)})),
            )
        })?;

    Ok(Json(json!({"success": true, "message": "Tag deleted"})))
}

/// Update course categories
async fn update_course_categories(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Clear existing
    sqlx::query("DELETE FROM course_category_mappings WHERE course_id = $1")
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();

    // Add new
    if let Some(category_ids) = input["category_ids"].as_array() {
        for cid in category_ids {
            if let Some(id) = cid.as_i64() {
                let _ = sqlx::query(
                    "INSERT INTO course_category_mappings (course_id, category_id) VALUES ($1, $2)",
                )
                .bind(course_id)
                .bind(id)
                .execute(&state.db.pool)
                .await;
            }
        }
    }

    Ok(Json(
        json!({"success": true, "message": "Course categories updated"}),
    ))
}

/// Update course tags
async fn update_course_tags(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Clear existing
    sqlx::query("DELETE FROM course_tag_mappings WHERE course_id = $1")
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();

    // Add new
    if let Some(tag_ids) = input["tag_ids"].as_array() {
        for tid in tag_ids {
            if let Some(id) = tid.as_i64() {
                let _ = sqlx::query(
                    "INSERT INTO course_tag_mappings (course_id, tag_id) VALUES ($1, $2)",
                )
                .bind(course_id)
                .bind(id)
                .execute(&state.db.pool)
                .await;
            }
        }
    }

    Ok(Json(
        json!({"success": true, "message": "Course tags updated"}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PREREQUISITES - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// Update lesson prerequisites
async fn update_lesson_prerequisites(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let prereq_ids: Vec<String> = input["prerequisite_lesson_ids"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    sqlx::query("UPDATE lessons SET prerequisite_lesson_ids = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3")
        .bind(json!(prereq_ids))
        .bind(lesson_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to update prerequisites: {}", e)}))))?;

    Ok(Json(
        json!({"success": true, "message": "Prerequisites updated"}),
    ))
}

/// Get lesson prerequisites
async fn get_lesson_prerequisites(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, lesson_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let prereqs: Option<(Option<serde_json::Value>,)> = sqlx::query_as(
        "SELECT prerequisite_lesson_ids FROM lessons WHERE id = $1 AND course_id = $2",
    )
    .bind(lesson_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let prerequisite_ids = prereqs.and_then(|p| p.0).unwrap_or(json!([]));

    Ok(Json(
        json!({"success": true, "data": {"lesson_id": lesson_id, "prerequisite_lesson_ids": prerequisite_ids}}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT STATS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get course enrollment statistics
async fn get_enrollment_stats(
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

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Courses
        .route("/", get(list_courses).post(create_course))
        .route(
            "/:id",
            get(get_course).put(update_course).delete(delete_course),
        )
        .route("/:id/publish", post(publish_course))
        .route("/:id/unpublish", post(unpublish_course))
        .route("/:id/archive", post(archive_course))
        .route("/:id/restore", post(restore_course))
        // Enrollments
        .route(
            "/:course_id/enrollments",
            get(list_enrollments).post(enroll_user),
        )
        .route(
            "/:course_id/enrollments/:enrollment_id",
            axum::routing::delete(remove_enrollment),
        )
        .route("/:course_id/stats", get(get_enrollment_stats))
        // Modules
        .route("/:course_id/modules", get(list_modules).post(create_module))
        .route(
            "/:course_id/modules/:module_id",
            put(update_module).delete(delete_module),
        )
        .route("/:course_id/modules/reorder", put(reorder_modules))
        // Lessons
        .route("/:course_id/lessons", get(list_lessons).post(create_lesson))
        .route(
            "/:course_id/lessons/:lesson_id",
            get(get_lesson).put(update_lesson).delete(delete_lesson),
        )
        .route("/:course_id/lessons/reorder", put(reorder_lessons))
        .route(
            "/:course_id/lessons/:lesson_id/prerequisites",
            get(get_lesson_prerequisites).put(update_lesson_prerequisites),
        )
        // Quizzes - ICT 7 Grade
        .route("/:course_id/quizzes", get(list_quizzes).post(create_quiz))
        .route(
            "/:course_id/quizzes/:quiz_id",
            get(get_quiz).put(update_quiz).delete(delete_quiz),
        )
        .route(
            "/:course_id/quizzes/:quiz_id/questions",
            post(add_quiz_question),
        )
        .route(
            "/:course_id/quizzes/:quiz_id/questions/:question_id",
            axum::routing::delete(delete_quiz_question),
        )
        // Categories & Tags - ICT 7 Grade
        .route("/:course_id/categories", put(update_course_categories))
        .route("/:course_id/tags", put(update_course_tags))
        // Downloads
        .route(
            "/:course_id/downloads",
            get(list_downloads).post(create_download),
        )
        .route(
            "/:course_id/downloads/:download_id",
            put(update_download).delete(delete_download),
        )
        .route("/:course_id/upload-url", post(get_upload_url))
        // Video Upload (TUS)
        .route("/:course_id/video-upload", post(create_video_upload))
}

/// Separate router for global category/tag management
pub fn taxonomy_router() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories).post(create_category))
        .route("/categories/:id", axum::routing::delete(delete_category))
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:id", axum::routing::delete(delete_tag))
}
