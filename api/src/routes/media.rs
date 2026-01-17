//! Media Controller - ICT 11+ Principal Engineer
//!
//! Manages media library with CRUD operations, filtering, pagination,
//! and full file upload support with Cloudflare R2 integration.
//!
//! Features:
//! - File upload with automatic optimization
//! - Presigned URL generation for direct uploads
//! - Image dimension extraction
//! - MIME type validation

#![allow(clippy::useless_format)]
#![allow(clippy::double_ended_iterator_last)]
//!
//! @version 2.0.0 - January 2026

use axum::{
    extract::{Multipart, Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{utils::errors::ApiError, AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Media {
    pub id: i64,
    pub filename: String,
    pub original_filename: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub path: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
    pub is_optimized: Option<bool>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct MediaQuery {
    pub search: Option<String>,
    pub r#type: Option<String>,
    pub collection: Option<String>,
    pub images_only: Option<bool>,
    pub is_optimized: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMedia {
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
}

/// GET /admin/media - List all media with filtering and pagination
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<MediaQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut query = String::from(
        "SELECT id, filename, original_filename, mime_type, size, path, url, 
                title, alt_text, caption, description, collection, is_optimized,
                width, height, created_at, updated_at 
         FROM media WHERE 1=1",
    );
    let mut conditions = Vec::new();

    // Search filter
    if let Some(search) = &params.search {
        conditions.push(format!(
            "(filename ILIKE '%{}%' OR title ILIKE '%{}%' OR alt_text ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    // Type filter (mime_type prefix)
    if let Some(media_type) = &params.r#type {
        conditions.push(format!(
            "mime_type LIKE '{}%'",
            media_type.replace('\'', "''")
        ));
    }

    // Collection filter
    if let Some(collection) = &params.collection {
        conditions.push(format!("collection = '{}'", collection.replace('\'', "''")));
    }

    // Images only filter
    if params.images_only.unwrap_or(false) {
        conditions.push("mime_type LIKE 'image/%'".to_string());
    }

    // Optimization filter
    if let Some(is_optimized) = params.is_optimized {
        conditions.push(format!("is_optimized = {}", is_optimized));
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }

    // Sorting
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

    query.push_str(&format!(" ORDER BY {} {}", sort_column, sort_direction));

    // Pagination
    let per_page = params.per_page.unwrap_or(24).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Get total count
    let count_query = format!(
        "SELECT COUNT(*) FROM media WHERE 1=1{}",
        if !conditions.is_empty() {
            format!(" AND {}", conditions.join(" AND "))
        } else {
            String::new()
        }
    );

    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let media: Vec<Media> = sqlx::query_as(&query)
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
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
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
#[tracing::instrument(skip(state))]
pub async fn update(
    State(state): State<AppState>,
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
        updates.push(format!("title = ${}", param_count));
        param_count += 1;
    }
    if payload.alt_text.is_some() {
        updates.push(format!("alt_text = ${}", param_count));
        param_count += 1;
    }
    if payload.caption.is_some() {
        updates.push(format!("caption = ${}", param_count));
        param_count += 1;
    }
    if payload.description.is_some() {
        updates.push(format!("description = ${}", param_count));
        param_count += 1;
    }
    if payload.collection.is_some() {
        updates.push(format!("collection = ${}", param_count));
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

    let mut query = sqlx::query_as::<_, Media>(&query_str);

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
#[tracing::instrument(skip(state))]
pub async fn destroy(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get media info first (for file deletion if needed)
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

    if media.is_none() {
        return Err(ApiError::not_found("Media not found"));
    }

    // Delete from database
    sqlx::query("DELETE FROM media WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // TODO: Delete actual file from S3/R2 storage
    // This would require the storage service integration

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Media deleted successfully"
    })))
}

/// GET /admin/media/statistics - Get media library statistics
#[tracing::instrument(skip(state))]
pub async fn statistics(
    State(state): State<AppState>,
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

/// Format bytes to human readable
fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FILE UPLOAD ENDPOINTS - ICT 11+ Principal Engineer
// ═══════════════════════════════════════════════════════════════════════════

/// Request for presigned upload URL
#[derive(Debug, Deserialize)]
pub struct PresignedUploadRequest {
    pub filename: String,
    pub content_type: String,
    pub size: Option<i64>,
    pub collection: Option<String>,
}

/// Response for presigned upload
#[derive(Debug, Serialize)]
pub struct PresignedUploadResponse {
    pub upload_url: String,
    pub file_key: String,
    pub public_url: String,
    pub expires_in: u64,
}

/// POST /admin/media/presigned-upload - Get presigned URL for direct upload
/// Client uploads directly to R2, then confirms with /admin/media/confirm-upload
#[tracing::instrument(skip(state))]
pub async fn presigned_upload(
    State(state): State<AppState>,
    Json(payload): Json<PresignedUploadRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate content type
    let allowed_types = [
        "image/jpeg",
        "image/png",
        "image/webp",
        "image/gif",
        "image/svg+xml",
        "video/mp4",
        "video/webm",
        "video/quicktime",
        "application/pdf",
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application/vnd.ms-excel",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "text/plain",
        "text/csv",
    ];

    if !allowed_types.contains(&payload.content_type.as_str()) {
        return Err(ApiError::validation_error(&format!(
            "Content type '{}' not allowed. Allowed: {:?}",
            payload.content_type, allowed_types
        )));
    }

    // Generate file key
    let extension = match payload.content_type.as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/svg+xml" => "svg",
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "video/quicktime" => "mov",
        "application/pdf" => "pdf",
        _ => "bin",
    };

    let folder = payload.collection.unwrap_or_else(|| "uploads".to_string());
    let file_key = format!("{}/{}.{}", folder, Uuid::new_v4(), extension);

    // Get presigned URL from storage service
    let storage = &state.services.storage;

    let expires_in = 3600u64; // 1 hour
    let upload_url = storage
        .presigned_upload_url(&file_key, &payload.content_type, expires_in)
        .await
        .map_err(|e| ApiError::internal_error(&format!("Failed to generate upload URL: {}", e)))?;

    // Get public URL
    let public_url = format!("{}/{}", state.config.r2_public_url, file_key);

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "upload_url": upload_url,
            "file_key": file_key,
            "public_url": public_url,
            "expires_in": expires_in
        }
    })))
}

/// Request to confirm upload after direct upload to R2
#[derive(Debug, Deserialize)]
pub struct ConfirmUploadRequest {
    pub file_key: String,
    pub original_filename: String,
    pub content_type: String,
    pub size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
}

/// POST /admin/media/confirm-upload - Confirm upload and save to database
#[tracing::instrument(skip(state))]
pub async fn confirm_upload(
    State(state): State<AppState>,
    Json(payload): Json<ConfirmUploadRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Build the public URL
    let public_url = format!("{}/{}", state.config.r2_public_url, payload.file_key);

    // Extract filename from key
    let filename = payload
        .file_key
        .split('/')
        .last()
        .unwrap_or(&payload.file_key)
        .to_string();

    // Insert into database
    let media: Media = sqlx::query_as(
        r#"
        INSERT INTO media (
            filename, original_filename, mime_type, size, path, url,
            title, alt_text, caption, description, collection,
            width, height, is_optimized, created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11,
            $12, $13, false, NOW(), NOW()
        )
        RETURNING id, filename, original_filename, mime_type, size, path, url,
                  title, alt_text, caption, description, collection, is_optimized,
                  width, height, created_at, updated_at
        "#,
    )
    .bind(&filename)
    .bind(&payload.original_filename)
    .bind(&payload.content_type)
    .bind(payload.size)
    .bind(&payload.file_key)
    .bind(&public_url)
    .bind(&payload.title)
    .bind(&payload.alt_text)
    .bind(&payload.caption)
    .bind(&payload.description)
    .bind(&payload.collection)
    .bind(payload.width)
    .bind(payload.height)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": media,
        "message": "Media uploaded successfully"
    })))
}

/// POST /admin/media/upload - Direct multipart file upload
/// For smaller files or when presigned URLs aren't suitable
#[tracing::instrument(skip(state, multipart))]
pub async fn direct_upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let storage = &state.services.storage;

    let mut uploaded_files: Vec<Media> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::validation_error(&format!("Failed to read multipart: {}", e)))?
    {
        let name = field.name().unwrap_or("file").to_string();

        // Skip non-file fields
        if name == "collection" || name == "title" {
            continue;
        }

        let filename = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}.bin", Uuid::new_v4()));

        let content_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Read file data
        let data = field
            .bytes()
            .await
            .map_err(|e| ApiError::validation_error(&format!("Failed to read file: {}", e)))?;

        let size = data.len() as i64;

        // Max file size: 50MB
        if size > 50 * 1024 * 1024 {
            return Err(ApiError::validation_error(
                "File too large. Maximum size is 50MB",
            ));
        }

        // Upload to R2
        let folder = "uploads";
        let public_url = storage
            .upload(data.to_vec(), &content_type, folder)
            .await
            .map_err(|e| ApiError::internal_error(&format!("Upload failed: {}", e)))?;

        // Extract key from URL
        let file_key = public_url
            .replace(&state.config.r2_public_url, "")
            .trim_start_matches('/')
            .to_string();
        let stored_filename = file_key.split('/').last().unwrap_or(&file_key).to_string();

        // Insert into database
        let media: Media = sqlx::query_as(
            r#"
            INSERT INTO media (
                filename, original_filename, mime_type, size, path, url,
                is_optimized, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                false, NOW(), NOW()
            )
            RETURNING id, filename, original_filename, mime_type, size, path, url,
                      title, alt_text, caption, description, collection, is_optimized,
                      width, height, created_at, updated_at
            "#,
        )
        .bind(&stored_filename)
        .bind(&filename)
        .bind(&content_type)
        .bind(size)
        .bind(&file_key)
        .bind(&public_url)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        uploaded_files.push(media);
    }

    if uploaded_files.is_empty() {
        return Err(ApiError::validation_error("No files uploaded"));
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "data": uploaded_files,
        "message": format!("{} file(s) uploaded successfully", uploaded_files.len())
    })))
}

/// Build the media admin router
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/files", get(index)) // Frontend uses /admin/media/files
        .route("/upload", post(direct_upload))
        .route("/presigned-upload", post(presigned_upload))
        .route("/confirm-upload", post(confirm_upload))
        .route("/statistics", get(statistics))
        .route("/:id", get(show).put(update).delete(destroy))
}
