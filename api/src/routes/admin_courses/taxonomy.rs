//! Categories + tags + course mapping handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::routes::admin_courses::slugify;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// CATEGORIES & TAGS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all categories
pub(super) async fn list_categories(
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
pub(super) async fn create_category(
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
pub(super) async fn delete_category(
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
pub(super) async fn list_tags(
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
pub(super) async fn create_tag(
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
pub(super) async fn delete_tag(
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
pub(super) async fn update_course_categories(
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
pub(super) async fn update_course_tags(
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
