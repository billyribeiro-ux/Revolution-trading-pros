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
// Minimal schema - production tags only has id, name, slug

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct TagQuery {
    pub search: Option<String>,
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
}

#[derive(Debug, Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub slug: Option<String>,
}

/// GET /admin/tags - List all tags (ICT 7: SQL injection safe)
#[tracing::instrument(skip(state, _admin))]
pub async fn index(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<TagQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // ICT Level 7: Parameterized query to prevent SQL injection
    let search_pattern = params
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('%', "\\%").replace('_', "\\_")));

    // Sort with whitelist validation (ICT 7: prevent injection via sort columns)
    let allowed_columns = ["name", "slug", "id"];
    let sort_by = params.sort_by.as_deref().unwrap_or("id");
    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "id"
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

    // Pagination
    let per_page = params.per_page.unwrap_or(100).min(500);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;
    let fetch_all = params.all.unwrap_or(false);

    // Execute with proper parameter binding
    let tags: Vec<Tag> = match (&search_pattern, fetch_all) {
        (Some(pattern), false) => {
            let query = format!(
                "SELECT id, name, slug FROM tags WHERE (name ILIKE $1 OR slug ILIKE $1) ORDER BY {} {} LIMIT $2 OFFSET $3",
                sort_column, sort_direction
            );
            sqlx::query_as(&query)
                .bind(pattern)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (Some(pattern), true) => {
            let query = format!(
                "SELECT id, name, slug FROM tags WHERE (name ILIKE $1 OR slug ILIKE $1) ORDER BY {} {}",
                sort_column, sort_direction
            );
            sqlx::query_as(&query)
                .bind(pattern)
                .fetch_all(state.db.pool())
                .await
        }
        (None, false) => {
            let query = format!(
                "SELECT id, name, slug FROM tags ORDER BY {} {} LIMIT $1 OFFSET $2",
                sort_column, sort_direction
            );
            sqlx::query_as(&query)
                .bind(per_page)
                .bind(offset)
                .fetch_all(state.db.pool())
                .await
        }
        (None, true) => {
            let query = format!(
                "SELECT id, name, slug FROM tags ORDER BY {} {}",
                sort_column, sort_direction
            );
            sqlx::query_as(&query)
                .fetch_all(state.db.pool())
                .await
        }
    }.map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tags })))
}

/// GET /admin/tags/:id - Get a single tag
#[tracing::instrument(skip(state, _admin))]
pub async fn show(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let tag: Tag = sqlx::query_as("SELECT * FROM tags WHERE id = $1")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|_| ApiError::not_found("Tag not found"))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// POST /admin/tags - Create a new tag
#[tracing::instrument(skip(state, _admin))]
pub async fn store(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateTag>,
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
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE slug = $1)")
        .bind(&payload.slug)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if exists {
        return Err(ApiError::validation_error("Slug already exists"));
    }

    let tag: Tag = sqlx::query_as(
        "INSERT INTO tags (name, slug)
         VALUES ($1, $2)
         RETURNING id, name, slug",
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// PUT /admin/tags/:id - Update a tag
#[tracing::instrument(skip(state, _admin))]
pub async fn update(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTag>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if tag exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1)")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !exists {
        return Err(ApiError::not_found("Tag not found"));
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

        let slug_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE slug = $1 AND id != $2)")
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

    if updates.is_empty() {
        return Err(ApiError::validation_error("No fields to update"));
    }

    let query_str = format!(
        "UPDATE tags SET {} WHERE id = ${} RETURNING id, name, slug",
        updates.join(", "),
        param_count
    );

    let mut query = sqlx::query_as::<_, Tag>(&query_str);

    if let Some(name) = payload.name {
        query = query.bind(name);
    }
    if let Some(slug) = payload.slug {
        query = query.bind(slug);
    }

    query = query.bind(id);

    let tag = query
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// DELETE /admin/tags/:id - Delete a tag
#[tracing::instrument(skip(state, _admin))]
pub async fn destroy(
    _admin: AdminUser,
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

    Ok(Json(
        serde_json::json!({ "message": "Tag deleted successfully" }),
    ))
}

/// Merge tag request (ICT 7: tag merge/rename functionality)
#[derive(Debug, Deserialize)]
pub struct MergeTagRequest {
    /// The tag ID to merge into (target - will be kept)
    pub target_id: i64,
    /// The tag ID to merge from (source - will be deleted)
    pub source_id: i64,
}

/// POST /admin/tags/merge - Merge two tags (ICT 7: merge functionality)
/// Moves all posts from source tag to target tag, then deletes source tag
#[tracing::instrument(skip(state, _admin))]
pub async fn merge_tags(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(payload): Json<MergeTagRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.target_id == payload.source_id {
        return Err(ApiError::validation_error("Cannot merge a tag with itself"));
    }

    // Verify both tags exist
    let target_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1)")
        .bind(payload.target_id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !target_exists {
        return Err(ApiError::not_found("Target tag not found"));
    }

    let source_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1)")
        .bind(payload.source_id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !source_exists {
        return Err(ApiError::not_found("Source tag not found"));
    }

    // Move all post_tags from source to target (ignoring duplicates)
    let moved_count = sqlx::query(
        "UPDATE post_tags SET tag_id = $1
         WHERE tag_id = $2
         AND post_id NOT IN (SELECT post_id FROM post_tags WHERE tag_id = $1)",
    )
    .bind(payload.target_id)
    .bind(payload.source_id)
    .execute(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?
    .rows_affected();

    // Delete remaining post_tags for source (duplicates that couldn't be moved)
    sqlx::query("DELETE FROM post_tags WHERE tag_id = $1")
        .bind(payload.source_id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Delete the source tag
    sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(payload.source_id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Return the target tag
    let tag: Tag = sqlx::query_as("SELECT id, name, slug FROM tags WHERE id = $1")
        .bind(payload.target_id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({
        "data": tag,
        "merged_posts": moved_count,
        "message": "Tags merged successfully"
    })))
}

/// Tag with usage count
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TagWithCount {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub post_count: i64,
}

/// GET /admin/tags/:id/usage - Get tag usage count (ICT 7: analytics)
#[tracing::instrument(skip(state, _admin))]
pub async fn get_usage(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let tag: TagWithCount = sqlx::query_as(
        "SELECT t.id, t.name, t.slug, COALESCE(COUNT(pt.post_id), 0) as post_count
         FROM tags t
         LEFT JOIN post_tags pt ON t.id = pt.tag_id
         WHERE t.id = $1
         GROUP BY t.id, t.name, t.slug",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?
    .ok_or_else(|| ApiError::not_found("Tag not found"))?;

    Ok(Json(serde_json::json!({ "data": tag })))
}

/// GET /admin/tags/with-counts - List all tags with usage counts (ICT 7: analytics)
#[tracing::instrument(skip(state, _admin))]
pub async fn list_with_counts(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let tags: Vec<TagWithCount> = sqlx::query_as(
        "SELECT t.id, t.name, t.slug, COALESCE(COUNT(pt.post_id), 0) as post_count
         FROM tags t
         LEFT JOIN post_tags pt ON t.id = pt.tag_id
         GROUP BY t.id, t.name, t.slug
         ORDER BY post_count DESC, t.name ASC",
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": tags })))
}

/// Build the tags router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/tags", get(index).post(store))
        .route("/admin/tags/merge", post(merge_tags))
        .route("/admin/tags/with-counts", get(list_with_counts))
        .route("/admin/tags/:id", get(show).put(update).delete(destroy))
        .route("/admin/tags/:id/usage", get(get_usage))
}
