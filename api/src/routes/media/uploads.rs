//! Media uploads — presigned R2 URLs, confirm upload, multipart direct upload.
//!
//! R27-B split: every endpoint that materially writes a NEW row into
//! `media` lives here. The CRUD module only mutates existing rows.

// The pre-split file had `#![allow(clippy::double_ended_iterator_last)]`
// at module scope; the `.split('/').last()` filename extractions below
// rely on it. Preserve verbatim to keep the split structural-only.
#![allow(clippy::double_ended_iterator_last)]

use axum::{
    extract::{Multipart, State},
    response::Json,
};
use uuid::Uuid;

use super::dto::{ConfirmUploadRequest, Media, PresignedUploadRequest};
use super::helpers::validate_file_signature;
use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

/// POST /admin/media/presigned-upload - Get presigned URL for direct upload
/// Client uploads directly to R2, then confirms with /admin/media/confirm-upload
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn presigned_upload(
    State(state): State<AppState>,
    _admin: AdminUser,
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
        .map_err(|e| ApiError::internal_error(&format!("Failed to generate upload URL: {e}")))?;

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

/// POST /admin/media/confirm-upload - Confirm upload and save to database
/// ICT 7 SECURITY: AdminUser authentication required, path traversal protection
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn confirm_upload(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<ConfirmUploadRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // ICT 7 SECURITY: Path traversal protection
    if payload.file_key.contains("..")
        || payload.file_key.contains("//")
        || payload.file_key.starts_with('/')
        || payload.file_key.contains('\0')
    {
        return Err(ApiError::validation_error(
            "Invalid file key: path traversal detected",
        ));
    }

    // Validate file_key format (should be folder/uuid.ext)
    let parts: Vec<&str> = payload.file_key.split('/').collect();
    if parts.len() < 2 {
        return Err(ApiError::validation_error("Invalid file key format"));
    }

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
        r"
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
        ",
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
/// ICT 7 SECURITY: AdminUser authentication, file type whitelist, size limits
#[tracing::instrument(skip(state, _admin, multipart))]
pub(super) async fn direct_upload(
    State(state): State<AppState>,
    _admin: AdminUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let storage = &state.services.storage;

    // ICT 7 SECURITY: Whitelist of allowed content types
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

    let mut uploaded_files: Vec<Media> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::validation_error(&format!("Failed to read multipart: {e}")))?
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

        // ICT 7 SECURITY: Path traversal protection on filename
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            return Err(ApiError::validation_error(
                "Invalid filename: path traversal characters not allowed",
            ));
        }

        let content_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // ICT 7 SECURITY: Validate content type against whitelist
        if !allowed_types.contains(&content_type.as_str()) {
            return Err(ApiError::validation_error(&format!(
                "File type '{content_type}' not allowed. Allowed types: images, videos, PDFs, documents"
            )));
        }

        // Read file data
        let data = field
            .bytes()
            .await
            .map_err(|e| ApiError::validation_error(&format!("Failed to read file: {e}")))?;

        let size = data.len() as i64;

        // Max file size: 50MB
        if size > 50 * 1024 * 1024 {
            return Err(ApiError::validation_error(
                "File too large. Maximum size is 50MB",
            ));
        }

        // ICT 7 SECURITY: Basic file signature validation (magic bytes)
        if !validate_file_signature(&data, &content_type) {
            return Err(ApiError::validation_error(
                "File content does not match declared type",
            ));
        }

        // Upload to R2
        let folder = "uploads";
        let public_url = storage
            .upload(data.to_vec(), &content_type, folder)
            .await
            .map_err(|e| ApiError::internal_error(&format!("Upload failed: {e}")))?;

        // Extract key from URL
        let file_key = public_url
            .replace(&state.config.r2_public_url, "")
            .trim_start_matches('/')
            .to_string();
        let stored_filename = file_key.split('/').last().unwrap_or(&file_key).to_string();

        // Insert into database
        let media: Media = sqlx::query_as(
            r"
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
            ",
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
