//! CMS Assets — bulk operations
//!
//! Endpoints (mounted under `/cms/assets` in `mod.rs::router()`):
//!   POST /bulk/move    — move up to 100 assets to a folder
//!   POST /bulk/delete  — soft/hard delete up to 100 assets
//!   POST /bulk/tag     — add/remove tags on up to 100 assets

use axum::{extract::State, Json};
use serde_json::{json, Value as JsonValue};

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

use super::dto::{BulkDeleteRequest, BulkMoveRequest, BulkTagRequest};

/// POST /cms/assets/bulk/move - Move multiple assets to folder
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn bulk_move(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkMoveRequest>,
) -> Result<Json<JsonValue>, ApiError> {
    if payload.asset_ids.is_empty() {
        return Err(ApiError::validation_error("No assets specified"));
    }
    if payload.asset_ids.len() > 100 {
        return Err(ApiError::validation_error(
            "Maximum 100 assets per operation",
        ));
    }

    let mut moved = 0;
    for asset_id in &payload.asset_ids {
        let result =
            sqlx::query("UPDATE cms_assets SET folder_id = $1, updated_at = NOW() WHERE id = $2")
                .bind(payload.target_folder_id)
                .bind(asset_id)
                .execute(state.db.pool())
                .await;
        if result.is_ok() {
            moved += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "moved": moved,
        "message": format!("{} asset(s) moved successfully", moved)
    })))
}

/// POST /cms/assets/bulk/delete - Delete multiple assets
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn bulk_delete(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkDeleteRequest>,
) -> Result<Json<JsonValue>, ApiError> {
    if payload.asset_ids.is_empty() {
        return Err(ApiError::validation_error("No assets specified"));
    }
    if payload.asset_ids.len() > 100 {
        return Err(ApiError::validation_error(
            "Maximum 100 assets per operation",
        ));
    }

    let permanent = payload.permanent.unwrap_or(false);
    let mut deleted = 0;

    for asset_id in &payload.asset_ids {
        if permanent {
            // Get storage key
            let key: Option<(String,)> =
                sqlx::query_as("SELECT storage_key FROM cms_assets WHERE id = $1")
                    .bind(asset_id)
                    .fetch_optional(state.db.pool())
                    .await
                    .ok()
                    .flatten();

            if let Some((storage_key,)) = key {
                let storage = &state.services.storage;
                let _ = storage.delete(&storage_key).await;
            }

            if sqlx::query("DELETE FROM cms_assets WHERE id = $1")
                .bind(asset_id)
                .execute(state.db.pool())
                .await
                .is_ok()
            {
                deleted += 1;
            }
        } else if sqlx::query("UPDATE cms_assets SET deleted_at = NOW() WHERE id = $1")
            .bind(asset_id)
            .execute(state.db.pool())
            .await
            .is_ok()
        {
            deleted += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "deleted": deleted,
        "message": format!("{} asset(s) deleted successfully", deleted)
    })))
}

/// POST /cms/assets/bulk/tag - Add/remove tags from multiple assets
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn bulk_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkTagRequest>,
) -> Result<Json<JsonValue>, ApiError> {
    if payload.asset_ids.is_empty() {
        return Err(ApiError::validation_error("No assets specified"));
    }
    if payload.asset_ids.len() > 100 {
        return Err(ApiError::validation_error(
            "Maximum 100 assets per operation",
        ));
    }

    let mut updated = 0;

    for asset_id in &payload.asset_ids {
        // Get current tags
        let current: Option<(Option<Vec<String>>,)> =
            sqlx::query_as("SELECT tags FROM cms_assets WHERE id = $1")
                .bind(asset_id)
                .fetch_optional(state.db.pool())
                .await
                .ok()
                .flatten();

        let mut tags: Vec<String> = current.and_then(|(t,)| t).unwrap_or_default();

        // Add new tags
        if let Some(ref to_add) = payload.tags_to_add {
            for tag in to_add {
                if !tags.contains(tag) {
                    tags.push(tag.clone());
                }
            }
        }

        // Remove tags
        if let Some(ref to_remove) = payload.tags_to_remove {
            tags.retain(|t| !to_remove.contains(t));
        }

        // Update
        if sqlx::query("UPDATE cms_assets SET tags = $1, updated_at = NOW() WHERE id = $2")
            .bind(&tags)
            .bind(asset_id)
            .execute(state.db.pool())
            .await
            .is_ok()
        {
            updated += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "updated": updated,
        "message": format!("{} asset(s) updated successfully", updated)
    })))
}
