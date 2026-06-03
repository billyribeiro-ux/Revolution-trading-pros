//! Media CRUD — list, show, update, destroy, and library statistics.
//!
//! R27-B split: index/show/update/destroy mirror the standard REST shape;
//! `statistics` lives here because it's a read-only summary over the
//! same table and uses the shared `format_bytes` helper.

#![allow(clippy::useless_format)]
#![allow(clippy::double_ended_iterator_last)]

use axum::{
    extract::{Path, Query, State},
    response::Json,
};

use super::dto::{Media, MediaQuery, UpdateMedia};
use super::helpers::format_bytes;
use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

/// GET /admin/media - List all media with filtering and pagination
/// ICT 7 SECURITY: AdminUser authentication required, parameterized queries
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn index(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<MediaQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // ICT 7 SECURITY FIX: Use parameterized queries to prevent SQL injection
    let mut conditions = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut param_idx = 1;

    // Search filter - parameterized
    if let Some(search) = &params.search {
        // Sanitize search input - only allow alphanumeric, spaces, and common chars
        let sanitized: String = search
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.')
            .take(100) // Limit length
            .collect();
        if !sanitized.is_empty() {
            conditions.push(format!(
                "(filename ILIKE ${} OR title ILIKE ${} OR alt_text ILIKE ${})",
                param_idx,
                param_idx + 1,
                param_idx + 2
            ));
            let pattern = format!("%{sanitized}%");
            bind_values.push(pattern.clone());
            bind_values.push(pattern.clone());
            bind_values.push(pattern);
            param_idx += 3;
        }
    }

    // Type filter - whitelist validation
    if let Some(media_type) = &params.r#type {
        let allowed_types = ["image", "video", "audio", "application", "text"];
        if allowed_types.contains(&media_type.as_str()) {
            conditions.push(format!("mime_type LIKE ${param_idx}"));
            bind_values.push(format!("{media_type}%"));
            param_idx += 1;
        }
    }

    // Collection filter - parameterized
    if let Some(collection) = &params.collection {
        // Sanitize collection name
        let sanitized: String = collection
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .take(50)
            .collect();
        if !sanitized.is_empty() {
            conditions.push(format!("collection = ${param_idx}"));
            bind_values.push(sanitized);
            #[allow(unused_assignments)]
            {
                param_idx += 1;
            } // Keep for extensibility
        }
    }

    // Images only filter - no user input, safe
    if params.images_only.unwrap_or(false) {
        conditions.push("mime_type LIKE 'image/%'".to_string());
    }

    // Optimization filter - boolean, safe
    if let Some(is_optimized) = params.is_optimized {
        conditions.push(format!("is_optimized = {is_optimized}"));
    }

    // Sorting - whitelist validation (no user input in query)
    let allowed_columns = [
        "filename",
        "title",
        "size",
        "created_at",
        "updated_at",
        "id",
    ];
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");

    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "created_at"
    };
    let sort_direction = if sort_dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    // Pagination - validated integers
    let per_page = params.per_page.unwrap_or(24).clamp(1, 100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Build query with conditions
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    // Get total count with parameterized query
    let count_query = format!("SELECT COUNT(*) FROM media WHERE 1=1{where_clause}");

    let mut count_q = sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(count_query.as_str()));
    for val in &bind_values {
        count_q = count_q.bind(val);
    }

    let total: i64 = count_q
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Build main query.
    //
    // SAFETY: `sort_column` and `sort_direction` are checked against the
    // `allowed_columns` allow-list and a binary ASC/DESC choice above.
    // Format-string injection through them is therefore impossible. LIMIT
    // and OFFSET are bound as parameters below — never formatted in.
    let query = format!(
        "SELECT id, filename, original_filename, mime_type, size, path, url,
                title, alt_text, caption, description, collection, is_optimized,
                width, height, created_at, updated_at
         FROM media WHERE 1=1{} ORDER BY {} {} LIMIT ${} OFFSET ${}",
        where_clause,
        sort_column,
        sort_direction,
        bind_values.len() + 1,
        bind_values.len() + 2,
    );

    let mut main_q = sqlx::query_as::<_, Media>(sqlx::AssertSqlSafe(query.as_str()));
    for val in &bind_values {
        main_q = main_q.bind(val);
    }
    main_q = main_q.bind(per_page).bind(offset);

    let media: Vec<Media> = main_q
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let last_page = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": media,
        "meta": {
            "current_page": page,
            "last_page": last_page,
            "per_page": per_page,
            "total": total
        }
    })))
}

/// GET /admin/media/:id - Get single media item
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn show(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let media: Option<Media> = sqlx::query_as(
        "SELECT id, filename, original_filename, mime_type, size, path, url,
                title, alt_text, caption, description, collection, is_optimized,
                width, height, created_at, updated_at
         FROM media WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    match media {
        Some(m) => Ok(Json(serde_json::json!({ "success": true, "data": m }))),
        None => Err(ApiError::not_found("Media not found")),
    }
}

/// PUT /admin/media/:id - Update media metadata (SEO fields)
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn update(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMedia>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM media WHERE id = $1)")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !exists {
        return Err(ApiError::not_found("Media not found"));
    }

    // Build dynamic update
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.title.is_some() {
        updates.push(format!("title = ${param_count}"));
        param_count += 1;
    }
    if payload.alt_text.is_some() {
        updates.push(format!("alt_text = ${param_count}"));
        param_count += 1;
    }
    if payload.caption.is_some() {
        updates.push(format!("caption = ${param_count}"));
        param_count += 1;
    }
    if payload.description.is_some() {
        updates.push(format!("description = ${param_count}"));
        param_count += 1;
    }
    if payload.collection.is_some() {
        updates.push(format!("collection = ${param_count}"));
        param_count += 1;
    }

    if updates.is_empty() {
        return Err(ApiError::validation_error("No fields to update"));
    }

    updates.push(format!("updated_at = NOW()"));

    let query_str = format!(
        "UPDATE media SET {} WHERE id = ${}
         RETURNING id, filename, original_filename, mime_type, size, path, url,
                   title, alt_text, caption, description, collection, is_optimized,
                   width, height, created_at, updated_at",
        updates.join(", "),
        param_count
    );

    let mut query = sqlx::query_as::<_, Media>(sqlx::AssertSqlSafe(query_str.as_str()));

    if let Some(title) = payload.title {
        query = query.bind(title);
    }
    if let Some(alt_text) = payload.alt_text {
        query = query.bind(alt_text);
    }
    if let Some(caption) = payload.caption {
        query = query.bind(caption);
    }
    if let Some(description) = payload.description {
        query = query.bind(description);
    }
    if let Some(collection) = payload.collection {
        query = query.bind(collection);
    }

    query = query.bind(id);

    let media = query
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": media,
        "message": "Media updated successfully"
    })))
}

/// DELETE /admin/media/:id - Delete media item
/// ICT 7+ ENHANCEMENT: Complete file deletion from R2 storage
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn destroy(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get media info first (for file deletion)
    let media: Option<Media> = sqlx::query_as(
        "SELECT id, filename, original_filename, mime_type, size, path, url,
                title, alt_text, caption, description, collection, is_optimized,
                width, height, created_at, updated_at
         FROM media WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let media = match media {
        Some(m) => m,
        None => return Err(ApiError::not_found("Media not found")),
    };

    // Delete file from R2 storage if path exists
    if let Some(file_key) = &media.path {
        let storage = &state.services.storage;
        if let Err(e) = storage.delete(file_key).await {
            tracing::warn!(
                "Failed to delete file from R2 storage: {} - {}",
                file_key,
                e
            );
            // Continue with database deletion even if R2 deletion fails
            // The file might have already been deleted or the path is invalid
        } else {
            tracing::info!("Deleted file from R2 storage: {}", file_key);
        }
    }

    // Delete from database
    sqlx::query("DELETE FROM media WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Media deleted successfully",
        "deleted": {
            "id": media.id,
            "filename": media.filename,
            "path": media.path
        }
    })))
}

/// GET /admin/media/statistics - Get media library statistics
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn statistics(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Total count
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM media")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // Total size
    let total_size: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(size), 0) FROM media")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // By type
    let images: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM media WHERE mime_type LIKE 'image/%'")
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    let videos: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM media WHERE mime_type LIKE 'video/%'")
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    let documents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media WHERE mime_type LIKE 'application/%' OR mime_type LIKE 'text/%'"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // Optimized count
    let optimized: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM media WHERE is_optimized = true")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "total_count": total,
            "total_size": total_size,
            "total_size_formatted": format_bytes(total_size),
            "by_type": {
                "images": images,
                "videos": videos,
                "documents": documents
            },
            "optimization": {
                "optimized": optimized,
                "unoptimized": total - optimized,
                "percentage": if total > 0 { (optimized * 100) / total } else { 0 }
            }
        }
    })))
}
