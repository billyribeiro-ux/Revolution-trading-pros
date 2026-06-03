//! Public (unauthenticated) course endpoints.
//!
//! Course discovery — listing, detail page, and the public-facing review
//! summary. No paywall logic lives here: protected fields are stripped /
//! gated inside `super::player` instead.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::models::course::{
    Course, CourseListItem, CourseModule, CourseQueryParams, LessonListItem, ModuleWithLessons,
};
use crate::AppState;

pub(super) async fn list_published_courses(
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
        r"
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
        ORDER BY {safe_sort_by} {safe_sort_order}
        LIMIT $4 OFFSET $5
        "
    );

    let courses: Vec<CourseListItem> = sqlx::query_as(sqlx::AssertSqlSafe(query.as_str()))
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
    let count_query = r"
        SELECT COUNT(*) FROM courses
        WHERE is_published = true AND status = 'published'
          AND ($1::text IS NULL OR level = $1)
          AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR description ILIKE '%' || $2 || '%')
          AND ($3::boolean IS NULL OR is_free = $3)
    ";

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

pub(super) async fn get_course_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let course: Course = sqlx::query_as(
        r"
        SELECT * FROM courses
        WHERE slug = $1 AND is_published = true
        ",
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
        r"
        SELECT * FROM course_modules_v2
        WHERE course_id = $1 AND is_published = true
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
        WHERE course_id = $1 AND (is_published = true OR is_published IS NULL)
        ORDER BY COALESCE(sort_order, position)
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

    Ok(Json(json!({
        "success": true,
        "data": {
            "course": course,
            "modules": modules_with_lessons
        }
    })))
}

/// Get reviews for a course (public summary — no auth required)
pub(super) async fn get_course_reviews(
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
        r"SELECT r.id, r.user_id, r.rating, r.title, r.content, r.is_verified_purchase, r.created_at
           FROM course_reviews r WHERE r.course_id = $1 AND r.is_approved = true ORDER BY r.created_at DESC LIMIT 50",
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
