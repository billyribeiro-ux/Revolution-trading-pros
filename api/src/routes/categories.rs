//! Categories Controller
//! ICT 11+ Principal Engineer - Blog category CRUD operations
//!
//! Manages blog categories with full CRUD, search, filtering, and sorting.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{utils::errors::ApiError, AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub search: Option<String>,
    pub is_visible: Option<bool>,
    pub parent_id: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub all: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
}

/// GET /admin/categories - List all categories
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut query = String::from("SELECT * FROM categories WHERE 1=1");
    let mut conditions = Vec::new();

    // Search
    if let Some(search) = &params.search {
        conditions.push(format!(
            "(name ILIKE '%{}%' OR description ILIKE '%{}%' OR slug ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    // Filter by visibility
    if let Some(is_visible) = params.is_visible {
        conditions.push(format!("is_visible = {}", is_visible));
    }

    // Filter by parent
    if let Some(parent_id) = params.parent_id {
        conditions.push(format!("parent_id = {}", parent_id));
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }

    // Sort with whitelist validation
    let allowed_columns = ["name", "slug", "created_at", "updated_at", "id"];
    let sort_by = params.sort_by.as_deref().unwrap_or("name");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("asc");

    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "name"
    };

    let sort_direction = if sort_dir.eq_ignore_ascii_case("desc") {
        "DESC"
    } else {
        "ASC"
    };

    query.push_str(&format!(" ORDER BY {} {}", sort_column, sort_direction));

    // Pagination
    if params.all.unwrap_or(false) {
        let categories: Vec<Category> = sqlx::query_as(&query)
            .fetch_all(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

        return Ok(Json(serde_json::json!({ "data": categories })));
    }

    let per_page = params.per_page.unwrap_or(50);
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * per_page;

    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let categories: Vec<Category> = sqlx::query_as(&query)
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": categories })))
}

/// GET /admin/categories/:id - Get a single category
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let category: Category = sqlx::query_as("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|_| ApiError::not_found("Category not found"))?;

    Ok(Json(serde_json::json!({ "data": category })))
}

/// POST /admin/categories - Create a new category
#[tracing::instrument(skip(state))]
pub async fn store(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategory>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate slug format
    if !payload
        .slug
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(ApiError::validation_error(
            "Slug must contain only lowercase letters, numbers, and hyphens",
        ));
    }

    // Check if slug already exists
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE slug = $1)")
            .bind(&payload.slug)
            .fetch_one(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if exists {
        return Err(ApiError::validation_error("Slug already exists"));
    }

    let category: Category = sqlx::query_as(
        "INSERT INTO categories (name, slug, description, parent_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), NOW())
         RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .bind(payload.parent_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": category })))
}

/// PUT /admin/categories/:id - Update a category
#[tracing::instrument(skip(state))]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCategory>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if category exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1)")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !exists {
        return Err(ApiError::not_found("Category not found"));
    }

    // Validate slug if provided
    if let Some(slug) = &payload.slug {
        if !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(ApiError::validation_error(
                "Slug must contain only lowercase letters, numbers, and hyphens",
            ));
        }

        // Check if slug already exists for another category
        let slug_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM categories WHERE slug = $1 AND id != $2)",
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

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.name.is_some() {
        updates.push(format!("name = ${}", param_count));
        param_count += 1;
    }
    if payload.slug.is_some() {
        updates.push(format!("slug = ${}", param_count));
        param_count += 1;
    }
    if payload.description.is_some() {
        updates.push(format!("description = ${}", param_count));
        param_count += 1;
    }
    if payload.parent_id.is_some() {
        updates.push(format!("parent_id = ${}", param_count));
        param_count += 1;
    }

    updates.push(format!("updated_at = ${}", param_count));

    let query_str = format!(
        "UPDATE categories SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count + 1
    );

    let mut query = sqlx::query_as::<_, Category>(&query_str);

    if let Some(name) = payload.name {
        query = query.bind(name);
    }
    if let Some(slug) = payload.slug {
        query = query.bind(slug);
    }
    if let Some(description) = payload.description {
        query = query.bind(description);
    }
    if let Some(parent_id) = payload.parent_id {
        query = query.bind(parent_id);
    }

    query = query.bind(chrono::Utc::now().naive_utc());
    query = query.bind(id);

    let category = query
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": category })))
}

/// DELETE /admin/categories/:id - Delete a category
#[tracing::instrument(skip(state))]
pub async fn destroy(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if category has posts
    let post_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE category_id = $1")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if post_count > 0 {
        return Err(ApiError::validation_error(
            "Cannot delete category with posts",
        ));
    }

    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Category not found"));
    }

    Ok(Json(
        serde_json::json!({ "message": "Category deleted successfully" }),
    ))
}

/// Build the categories router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/categories", get(index).post(store))
        .route(
            "/admin/categories/:id",
            get(show).put(update).delete(destroy),
        )
}
