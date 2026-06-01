//! Single-preset retrieval endpoints — fetch by ID or by
//! (block_type, slug) tuple.
//!
//! Split from the original `cms_presets.rs` (R25-B). Pure structural
//! move — handler bodies are unchanged.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::dto::CmsPreset;
use super::helpers::{require_cms_editor, ApiResult};

/// Get a preset by ID
#[utoipa::path(
    get,
    path = "/api/cms/presets/{id}",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    responses(
        (status = 200, description = "Preset details", body = CmsPreset),
        (status = 404, description = "Preset not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    let preset: CmsPreset = sqlx::query_as(
        r"
        SELECT id, block_type, name, slug, description, preset_data,
               thumbnail_url, thumbnail_blurhash, category,
               tags, is_default, is_locked, is_global, usage_count, version,
               created_at, updated_at, created_by, updated_by, deleted_at
        FROM cms_presets
        WHERE id = $1 AND deleted_at IS NULL
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(preset))
}

/// Get a preset by block type and slug
#[utoipa::path(
    get,
    path = "/api/cms/presets/block/{block_type}/slug/{slug}",
    tag = "CMS Presets",
    params(
        ("block_type" = String, Path, description = "Block type"),
        ("slug" = String, Path, description = "Preset slug")
    ),
    responses(
        (status = 200, description = "Preset details", body = CmsPreset),
        (status = 404, description = "Preset not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_preset_by_slug(
    State(state): State<AppState>,
    user: User,
    Path((block_type, slug)): Path<(String, String)>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    let preset: CmsPreset = sqlx::query_as(
        r"
        SELECT id, block_type, name, slug, description, preset_data,
               thumbnail_url, thumbnail_blurhash, category,
               tags, is_default, is_locked, is_global, usage_count, version,
               created_at, updated_at, created_by, updated_by, deleted_at
        FROM cms_presets
        WHERE block_type = $1 AND slug = $2 AND deleted_at IS NULL
        ",
    )
    .bind(&block_type)
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(preset))
}
