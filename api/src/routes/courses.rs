//! Course routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{Course, CreateCourse, Lesson, User},
    middleware::admin::AdminUser,
    AppState,
};

/// List all published courses
async fn list_courses(
    State(state): State<AppState>,
) -> Result<Json<Vec<Course>>, (StatusCode, Json<serde_json::Value>)> {
    let courses: Vec<Course> = sqlx::query_as(
        "SELECT * FROM courses WHERE is_published = true ORDER BY created_at DESC"
    )
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(courses))
}

/// Get course by slug
async fn get_course(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Course>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as("SELECT * FROM courses WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Course not found"}))))?;

    Ok(Json(course))
}

/// Get lessons for a course
async fn get_lessons(
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<Vec<Lesson>>, (StatusCode, Json<serde_json::Value>)> {
    let lessons: Vec<Lesson> = sqlx::query_as(
        "SELECT * FROM lessons WHERE course_id = $1 ORDER BY position ASC"
    )
        .bind(course_id)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(lessons))
}

/// Create a new course (admin only)
/// ICT 11+ Security: Admin authorization enforced via AdminUser extractor
async fn create_course(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateCourse>,
) -> Result<Json<Course>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "course_create",
        user_id = %user.id,
        email = %user.email,
        "Admin creating course"
    );

    let slug = slug::slugify(&input.title);
    let id = Uuid::new_v4();
    let instructor_id = user.id;

    let course: Course = sqlx::query_as(
        r#"
        INSERT INTO courses (id, title, slug, description, price_cents, instructor_id, is_published, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, false, NOW(), NOW())
        RETURNING *
        "#,
    )
        .bind(id)
        .bind(&input.title)
        .bind(&slug)
        .bind(&input.description)
        .bind(input.price_cents)
        .bind(instructor_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(course))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_courses).post(create_course))
        .route("/:slug", get(get_course))
        .route("/:id/lessons", get(get_lessons))
}
