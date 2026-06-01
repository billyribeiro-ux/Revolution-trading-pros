//! Member review write endpoints (submit/delete).
//!
//! Note: the public review-listing summary (`GET /:slug/reviews`) lives in
//! [`super::public`] because it requires no authentication.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::models::User;
use crate::AppState;

/// Submit a review for a course
pub(super) async fn submit_review(
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
        r"INSERT INTO course_reviews (user_id, course_id, rating, title, content, is_verified_purchase, is_approved)
           VALUES ($1, $2, $3, $4, $5, $6, true) RETURNING id, rating, created_at"
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
pub(super) async fn delete_review(
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
