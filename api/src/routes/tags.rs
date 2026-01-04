//! Tags Controller
//! ICT 11+ Principal Engineer - Blog tag CRUD operations
//!
//! Manages blog tags with full CRUD, search, filtering, and sorting.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

use crate::{
    utils::errors::ApiError,
    AppState,
};

#[derive(Debug, Serialize, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub color: String,
    pub is_visible: bool,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct TagQuery {
    pub search: Option<String>,
    pub is_visible: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub all: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub slug: String,
    pub color: String,
    pub is_visible: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub is_visible: Option<bool>,
    pub order: Option<i32>,
}

/// GET /admin/tags - List all tags
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<TagQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut query = String::from("SELECT * FROM tags WHERE 1=1");
    let mut conditions = Vec::new();

    // Search
    if let Some(search) = &params.search {
        conditions.push(format!(
            "(name ILIKE '%{}%' OR slug ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    // Filter by visibility
    if let Some(is_visible) = params.is_visible {
        conditions.push(format!("is_visible = {}", is_visible));
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }

    // Sort
    let allowed_columns = ["order", "name", "slug", "created_at", "updated_at", "id"];
    let sort_by = params.sort_by.as_deref().unwrap_or("order");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("asc");

    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "order"
    };

    let sort_direction = if sort_dir.eq_ignore_ascii_case("desc") {
        "DESC"
    } else {
        "ASC"
    };

    query.push_str(&format!(" ORDER BY {} {}", sort_column, sort_direction));

    // Pagination
    if params.all.unwrap_or(false) {
        let tags: Vec<Tag> = sqlx::query_as(&query)
            .fetch_all(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

        return Ok(Json(serde_json::json!({ "data": tags })));
    }

    let per_page = params.per_page.unwrap_or(100);
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * per_page;

    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let tags: Vec<Tag> = sqlx::query_as(&query)
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tags })))
}

/// GET /admin/tags/:id - Get a single tag
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let tag: Tag = sqlx::query_as(
        "SELECT * FROM tags WHERE id = $1"
    )
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|_| ApiError::not_found("Tag not found"))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// POST /admin/tags - Create a new tag
#[tracing::instrument(skip(state))]
pub async fn store(
    State(state): State<AppState>,
    Json(payload): Json<CreateTag>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate slug format
    if !payload.slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Err(ApiError::validation_error("Slug must contain only lowercase letters, numbers, and hyphens"));
    }

    // Validate color format
    if !payload.color.starts_with('#') || payload.color.len() != 7 {
        return Err(ApiError::validation_error("Color must be in #RRGGBB format"));
    }

    // Check if slug already exists
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM tags WHERE slug = $1)"
    )
    .bind(&payload.slug)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if exists {
        return Err(ApiError::validation_error("Slug already exists"));
    }

    // Get max order for new tag
    let max_order: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(\"order\") FROM tags"
    )
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let new_order = max_order.unwrap_or(0) + 1;

    let tag: Tag = sqlx::query_as(
        "INSERT INTO tags (name, slug, color, is_visible, \"order\", created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
         RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.color)
    .bind(payload.is_visible.unwrap_or(true))
    .bind(new_order)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// PUT /admin/tags/:id - Update a tag
#[tracing::instrument(skip(state))]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTag>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if tag exists
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !exists {
        return Err(ApiError::not_found("Tag not found"));
    }

    // Validate slug if provided
    if let Some(slug) = &payload.slug {
        if !slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(ApiError::validation_error("Slug must contain only lowercase letters, numbers, and hyphens"));
        }

        let slug_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM tags WHERE slug = $1 AND id != $2)"
        )
        .bind(slug)
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if slug_exists {
            return Err(ApiError::validation_error("Slug already exists"));
        }
    }

    // Validate color if provided
    if let Some(color) = &payload.color {
        if !color.starts_with('#') || color.len() != 7 {
            return Err(ApiError::validation_error("Color must be in #RRGGBB format"));
        }
    }

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.name.is_some() { updates.push(format!("name = ${}", param_count)); param_count += 1; }
    if payload.slug.is_some() { updates.push(format!("slug = ${}", param_count)); param_count += 1; }
    if payload.color.is_some() { updates.push(format!("color = ${}", param_count)); param_count += 1; }
    if payload.is_visible.is_some() { updates.push(format!("is_visible = ${}", param_count)); param_count += 1; }
    if payload.order.is_some() { updates.push(format!("\"order\" = ${}", param_count)); param_count += 1; }

    updates.push(format!("updated_at = ${}", param_count));

    let query_str = format!(
        "UPDATE tags SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count + 1
    );

    let mut query = sqlx::query_as::<_, Tag>(&query_str);

    if let Some(name) = payload.name { query = query.bind(name); }
    if let Some(slug) = payload.slug { query = query.bind(slug); }
    if let Some(color) = payload.color { query = query.bind(color); }
    if let Some(is_visible) = payload.is_visible { query = query.bind(is_visible); }
    if let Some(order) = payload.order { query = query.bind(order); }

    query = query.bind(Utc::now());
    query = query.bind(id);

    let tag = query
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// DELETE /admin/tags/:id - Delete a tag
#[tracing::instrument(skip(state))]
pub async fn destroy(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let result = sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Tag not found"));
    }

    Ok(Json(serde_json::json!({ "message": "Tag deleted successfully" })))
}

/// Build the tags router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/tags", get(index).post(store))
        .route("/admin/tags/:id", get(show).put(update).delete(destroy))
}
