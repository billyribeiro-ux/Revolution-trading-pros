//! Categories Controller
//! ICT Level 7 Principal Engineer - Blog category CRUD operations
//!
//! Manages blog categories with full CRUD, search, filtering, sorting, and hierarchy support.
//! All admin endpoints require AdminUser authentication.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

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

/// GET /admin/categories - List all categories (ICT 7: SQL injection safe)
#[tracing::instrument(skip(state, _admin))]
pub async fn index(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // ICT Level 7: Parameterized query to prevent SQL injection
    let search_pattern = params
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('%', "\\%").replace('_', "\\_")));

    // Sort with whitelist validation (ICT 7: prevent injection via sort columns)
    let allowed_columns = ["name", "slug", "created_at", "updated_at", "id"];
    let sort_by = params.sort_by.as_deref().unwrap_or("name");
    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "name"
    };
    let sort_direction = if params
        .sort_dir
        .as_deref()
        .unwrap_or("asc")
        .eq_ignore_ascii_case("desc")
    {
        "DESC"
    } else {
        "ASC"
    };

    // Build fully parameterized query
    let base_query = format!(
        "SELECT * FROM categories WHERE 1=1 {} {} {} ORDER BY {} {}",
        if search_pattern.is_some() {
            "AND (name ILIKE $1 OR description ILIKE $1 OR slug ILIKE $1)"
        } else {
            ""
        },
        if params.is_visible.is_some() && search_pattern.is_some() {
            "AND is_visible = $2"
        } else if params.is_visible.is_some() {
            "AND is_visible = $1"
        } else {
            ""
        },
        if params.parent_id.is_some() {
            let idx = 1 + params.search.is_some() as usize + params.is_visible.is_some() as usize;
            format!("AND parent_id = ${}", idx)
        } else {
            "".to_string()
        },
        sort_column,
        sort_direction
    );

    // Pagination
    let per_page = params.per_page.unwrap_or(50).min(500);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;
    let fetch_all = params.all.unwrap_or(false);

    // Execute with proper parameter binding
    let categories: Vec<Category> = match (
        &search_pattern,
        params.is_visible,
        params.parent_id,
        fetch_all,
    ) {
        (Some(pattern), Some(vis), Some(pid), false) => {
            sqlx::query_as(&format!("{} LIMIT $4 OFFSET $5", base_query))
                .bind(pattern)
                .bind(vis)
                .bind(pid)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), Some(vis), None, false) => {
            sqlx::query_as(&format!("{} LIMIT $3 OFFSET $4", base_query))
                .bind(pattern)
                .bind(vis)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), None, Some(pid), false) => {
            sqlx::query_as(&format!("{} LIMIT $3 OFFSET $4", base_query))
                .bind(pattern)
                .bind(pid)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), None, None, false) => {
            sqlx::query_as(&format!("{} LIMIT $2 OFFSET $3", base_query))
                .bind(pattern)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (None, Some(vis), Some(pid), false) => {
            sqlx::query_as(&format!("{} LIMIT $3 OFFSET $4", base_query))
                .bind(vis)
                .bind(pid)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (None, Some(vis), None, false) => {
            sqlx::query_as(&format!("{} LIMIT $2 OFFSET $3", base_query))
                .bind(vis)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (None, None, Some(pid), false) => {
            sqlx::query_as(&format!("{} LIMIT $2 OFFSET $3", base_query))
                .bind(pid)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (None, None, None, false) => {
            sqlx::query_as(&format!("{} LIMIT $1 OFFSET $2", base_query))
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        // All=true variants (no pagination)
        (Some(pattern), Some(vis), Some(pid), true) => {
            sqlx::query_as(&base_query)
                .bind(pattern)
                .bind(vis)
                .bind(pid)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), Some(vis), None, true) => {
            sqlx::query_as(&base_query)
                .bind(pattern)
                .bind(vis)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), None, Some(pid), true) => {
            sqlx::query_as(&base_query)
                .bind(pattern)
                .bind(pid)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), None, None, true) => {
            sqlx::query_as(&base_query)
                .bind(pattern)
                .fetch_all(state.db.pool())
                .await
        }
        (None, Some(vis), Some(pid), true) => {
            sqlx::query_as(&base_query)
                .bind(vis)
                .bind(pid)
                .fetch_all(state.db.pool())
                .await
        }
        (None, Some(vis), None, true) => {
            sqlx::query_as(&base_query)
                .bind(vis)
                .fetch_all(state.db.pool())
                .await
        }
        (None, None, Some(pid), true) => {
            sqlx::query_as(&base_query)
                .bind(pid)
                .fetch_all(state.db.pool())
                .await
        }
        (None, None, None, true) => sqlx::query_as(&base_query).fetch_all(state.db.pool()).await,
    }
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": categories })))
}

/// GET /admin/categories/:id - Get a single category
#[tracing::instrument(skip(state, _admin))]
pub async fn show(
    _admin: AdminUser,
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
#[tracing::instrument(skip(state, _admin))]
pub async fn store(
    _admin: AdminUser,
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

    // ICT 7: Validate parent exists if specified
    if let Some(parent_id) = payload.parent_id {
        let parent_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1)")
                .bind(parent_id)
                .fetch_one(state.db.pool())
                .await
                .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if !parent_exists {
            return Err(ApiError::validation_error("Parent category does not exist"));
        }
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
#[tracing::instrument(skip(state, _admin))]
pub async fn update(
    _admin: AdminUser,
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

    // ICT 7: Prevent circular reference (parent cannot be self or descendant)
    if let Some(parent_id) = payload.parent_id {
        if parent_id == id {
            return Err(ApiError::validation_error(
                "Category cannot be its own parent",
            ));
        }

        // Check parent exists
        let parent_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1)")
                .bind(parent_id)
                .fetch_one(state.db.pool())
                .await
                .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if !parent_exists {
            return Err(ApiError::validation_error("Parent category does not exist"));
        }

        // Check for circular reference (parent cannot be a descendant)
        let is_descendant: bool = sqlx::query_scalar(
            "WITH RECURSIVE descendants AS (
                SELECT id FROM categories WHERE parent_id = $1
                UNION ALL
                SELECT c.id FROM categories c
                JOIN descendants d ON c.parent_id = d.id
            )
            SELECT EXISTS(SELECT 1 FROM descendants WHERE id = $2)",
        )
        .bind(id)
        .bind(parent_id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if is_descendant {
            return Err(ApiError::validation_error(
                "Cannot set a descendant category as parent (circular reference)",
            ));
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
#[tracing::instrument(skip(state, _admin))]
pub async fn destroy(
    _admin: AdminUser,
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

    // ICT 7: Check if category has child categories
    let child_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE parent_id = $1")
            .bind(id)
            .fetch_one(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if child_count > 0 {
        return Err(ApiError::validation_error(
            "Cannot delete category with subcategories. Delete or reassign children first.",
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

/// GET /admin/categories/:id/children - Get child categories (ICT 7: hierarchy support)
#[tracing::instrument(skip(state, _admin))]
pub async fn get_children(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let children: Vec<Category> =
        sqlx::query_as("SELECT * FROM categories WHERE parent_id = $1 ORDER BY name ASC")
            .bind(id)
            .fetch_all(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": children })))
}

/// Category tree node for hierarchical response
#[derive(Debug, Serialize)]
pub struct CategoryTreeNode {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub children: Vec<CategoryTreeNode>,
}

/// GET /admin/categories/tree - Get full category hierarchy tree (ICT 7: hierarchy support)
#[tracing::instrument(skip(state, _admin))]
pub async fn get_tree(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Fetch all categories
    let categories: Vec<Category> = sqlx::query_as("SELECT * FROM categories ORDER BY name ASC")
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Build tree structure
    fn build_tree(categories: &[Category], parent_id: Option<i64>) -> Vec<CategoryTreeNode> {
        categories
            .iter()
            .filter(|c| c.parent_id == parent_id)
            .map(|c| CategoryTreeNode {
                id: c.id,
                name: c.name.clone(),
                slug: c.slug.clone(),
                description: c.description.clone(),
                parent_id: c.parent_id,
                children: build_tree(categories, Some(c.id)),
            })
            .collect()
    }

    let tree = build_tree(&categories, None);

    Ok(Json(serde_json::json!({ "data": tree })))
}

/// Build the categories router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/categories", get(index).post(store))
        .route("/admin/categories/tree", get(get_tree))
        .route(
            "/admin/categories/:id",
            get(show).put(update).delete(destroy),
        )
        .route("/admin/categories/:id/children", get(get_children))
}
