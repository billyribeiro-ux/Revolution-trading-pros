//! CMS Assets — single-asset CRUD
//!
//! Endpoints (mounted under `/cms/assets` in `mod.rs::router()`):
//!   POST   /upload          — confirm upload, create asset record
//!   PUT    /:id             — patch asset metadata
//!   DELETE /:id             — soft delete (or `?permanent=true` for hard)
//!   POST   /:id/restore     — restore soft-deleted asset
//!   POST   /:id/replace     — replace asset file, keep metadata

use axum::{
    extract::{Path, Query, State},
    Json,
};
use rust_decimal::Decimal;
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

use super::dto::{Asset, ReplaceAssetRequest, UpdateAssetRequest, UploadConfirmRequest};
use super::helpers::get_cms_user_id;

/// POST /cms/assets/upload - Confirm upload and create asset record
#[tracing::instrument(skip(state, admin))]
pub(super) async fn create_asset(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<UploadConfirmRequest>,
) -> Result<Json<Asset>, ApiError> {
    // Validate file key
    if payload.file_key.contains("..") || payload.file_key.contains("//") {
        return Err(ApiError::validation_error("Invalid file key"));
    }

    let extension = payload
        .original_filename
        .rsplit('.')
        .next()
        .unwrap_or("bin")
        .to_lowercase();

    let public_url = format!("{}/{}", state.config.r2_public_url, payload.file_key);

    // Calculate aspect ratio
    let aspect_ratio = match (payload.width, payload.height) {
        (Some(w), Some(h)) if h > 0 => Some(Decimal::from(w) / Decimal::from(h)),
        _ => None,
    };

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;

    let asset: Asset = sqlx::query_as(
        r"
        INSERT INTO cms_assets (
            id, folder_id, filename, original_filename, mime_type, file_size,
            file_extension, storage_provider, storage_key, cdn_url,
            width, height, aspect_ratio, blurhash, dominant_color,
            title, alt_text, tags, version,
            created_at, updated_at, created_by
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, 'r2', $8, $9,
            $10, $11, $12, $13, $14,
            $15, $16, $17, 1,
            NOW(), NOW(), $18
        )
        RETURNING *
        ",
    )
    .bind(Uuid::new_v4())
    .bind(payload.folder_id)
    .bind(
        payload
            .file_key
            .split('/')
            .next_back()
            .unwrap_or(&payload.file_key),
    )
    .bind(&payload.original_filename)
    .bind(&payload.mime_type)
    .bind(payload.file_size)
    .bind(&extension)
    .bind(&payload.file_key)
    .bind(&public_url)
    .bind(payload.width)
    .bind(payload.height)
    .bind(aspect_ratio)
    .bind(&payload.blurhash)
    .bind(&payload.dominant_color)
    .bind(&payload.title)
    .bind(&payload.alt_text)
    .bind(&payload.tags)
    .bind(cms_user_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(asset))
}

/// PUT /cms/assets/:id - Update asset metadata
#[tracing::instrument(skip(state, admin))]
pub(super) async fn update_asset(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, ApiError> {
    // Build dynamic update
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.folder_id.is_some() {
        updates.push(format!("folder_id = ${param_count}"));
        param_count += 1;
    }
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
    if payload.credits.is_some() {
        updates.push(format!("credits = ${param_count}"));
        param_count += 1;
    }
    if payload.seo_title.is_some() {
        updates.push(format!("seo_title = ${param_count}"));
        param_count += 1;
    }
    if payload.seo_description.is_some() {
        updates.push(format!("seo_description = ${param_count}"));
        param_count += 1;
    }
    if payload.tags.is_some() {
        updates.push(format!("tags = ${param_count}"));
        param_count += 1;
    }

    if updates.is_empty() {
        return Err(ApiError::validation_error("No fields to update"));
    }

    updates.push("updated_at = NOW()".to_string());
    updates.push(format!("updated_by = ${param_count}"));
    param_count += 1;
    updates.push("version = version + 1".to_string());

    let query = format!(
        "UPDATE cms_assets SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;
    let mut q = sqlx::query_as::<_, Asset>(&query);

    if let Some(folder_id) = payload.folder_id {
        q = q.bind(folder_id);
    }
    if let Some(ref title) = payload.title {
        q = q.bind(title);
    }
    if let Some(ref alt_text) = payload.alt_text {
        q = q.bind(alt_text);
    }
    if let Some(ref caption) = payload.caption {
        q = q.bind(caption);
    }
    if let Some(ref description) = payload.description {
        q = q.bind(description);
    }
    if let Some(ref credits) = payload.credits {
        q = q.bind(credits);
    }
    if let Some(ref seo_title) = payload.seo_title {
        q = q.bind(seo_title);
    }
    if let Some(ref seo_description) = payload.seo_description {
        q = q.bind(seo_description);
    }
    if let Some(ref tags) = payload.tags {
        q = q.bind(tags);
    }
    q = q.bind(cms_user_id);
    q = q.bind(id);

    let asset = q
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(asset))
}

/// DELETE /cms/assets/:id - Soft delete asset
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn delete_asset(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<JsonValue>, ApiError> {
    let permanent = params
        .get("permanent")
        .map(|v| v == "true")
        .unwrap_or(false);

    if permanent {
        // Get storage key first
        let asset: Option<(String,)> =
            sqlx::query_as("SELECT storage_key FROM cms_assets WHERE id = $1")
                .bind(id)
                .fetch_optional(state.db.pool())
                .await
                .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if let Some((storage_key,)) = asset {
            // Delete from storage
            let storage = &state.services.storage;
            if let Err(e) = storage.delete(&storage_key).await {
                tracing::warn!("Failed to delete from storage: {}", e);
            }
        }

        // Hard delete
        sqlx::query("DELETE FROM cms_assets WHERE id = $1")
            .bind(id)
            .execute(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    } else {
        // Soft delete
        sqlx::query("UPDATE cms_assets SET deleted_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    }

    Ok(Json(json!({
        "success": true,
        "message": "Asset deleted successfully"
    })))
}

/// POST /cms/assets/:id/restore - Restore soft-deleted asset
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn restore_asset(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JsonValue>, ApiError> {
    sqlx::query("UPDATE cms_assets SET deleted_at = NULL WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "Asset restored successfully"
    })))
}

/// POST /cms/assets/:id/replace - Replace asset file (keeps metadata)
#[tracing::instrument(skip(state, admin))]
pub(super) async fn replace_asset(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<ReplaceAssetRequest>,
) -> Result<Json<Asset>, ApiError> {
    // Get old storage key
    let old_key: Option<(String,)> =
        sqlx::query_as("SELECT storage_key FROM cms_assets WHERE id = $1")
            .bind(id)
            .fetch_optional(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let public_url = format!("{}/{}", state.config.r2_public_url, payload.file_key);
    let extension = payload
        .original_filename
        .rsplit('.')
        .next()
        .unwrap_or("bin")
        .to_lowercase();

    let aspect_ratio = match (payload.width, payload.height) {
        (Some(w), Some(h)) if h > 0 => Some(Decimal::from(w) / Decimal::from(h)),
        _ => None,
    };

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;

    // Update asset
    let asset: Asset = sqlx::query_as(
        r"
        UPDATE cms_assets SET
            original_filename = $1, mime_type = $2, file_size = $3,
            file_extension = $4, storage_key = $5, cdn_url = $6,
            width = $7, height = $8, aspect_ratio = $9,
            version = version + 1, updated_at = NOW(), updated_by = $10
        WHERE id = $11
        RETURNING *
        ",
    )
    .bind(&payload.original_filename)
    .bind(&payload.mime_type)
    .bind(payload.file_size)
    .bind(&extension)
    .bind(&payload.file_key)
    .bind(&public_url)
    .bind(payload.width)
    .bind(payload.height)
    .bind(aspect_ratio)
    .bind(cms_user_id)
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Delete old file from storage
    if let Some((old_storage_key,)) = old_key {
        let storage = &state.services.storage;
        if let Err(e) = storage.delete(&old_storage_key).await {
            tracing::warn!("Failed to delete old file: {}", e);
        }
    }

    Ok(Json(asset))
}
