//! Action endpoints on the CMS presets API:
//!
//! - `duplicate_preset` — clone an existing preset with a fresh slug
//! - `apply_preset` — increment the usage counter when a preset is used
//! - `save_block_as_preset` — promote a live block to a reusable preset
//!
//! Split from the original `cms_presets.rs` (R25-B). Pure structural
//! move — handler bodies are unchanged.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::dto::{CmsPreset, CmsPresetCategory, SaveBlockAsPresetRequest};
use super::helpers::{ensure_unique_slug, generate_slug, require_cms_editor, ApiResult};

/// Duplicate a preset
#[utoipa::path(
    post,
    path = "/api/cms/presets/{id}/duplicate",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID to duplicate")
    ),
    responses(
        (status = 201, description = "Preset duplicated successfully", body = CmsPreset),
        (status = 404, description = "Preset not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn duplicate_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<CmsPreset>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch the original preset
    let original: CmsPreset = sqlx::query_as(
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

    // Generate new name and slug
    let new_name = format!("{} (Copy)", original.name);
    let base_slug = generate_slug(&new_name);
    let new_slug =
        ensure_unique_slug(&state.db.pool, &original.block_type, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    let dup_id = Uuid::new_v4();
    let duplicated: CmsPreset = sqlx::query_as(
        r"
        INSERT INTO cms_presets (
            id, block_type, name, slug, description, preset_data,
            thumbnail_url, thumbnail_blurhash, category, tags,
            is_default, is_global, is_locked, usage_count, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, false, $11, false, 0, 1, NOW(), NOW(), $12)
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        ",
    )
    .bind(dup_id)
    .bind(&original.block_type)
    .bind(&new_name)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.preset_data)
    .bind(&original.thumbnail_url)
    .bind(&original.thumbnail_blurhash)
    .bind(original.category)
    .bind(&original.tags)
    .bind(original.is_global)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Preset duplicated: {} -> {} ({})",
        original.name,
        duplicated.name,
        duplicated.id
    );

    Ok((StatusCode::CREATED, Json(duplicated)))
}

/// Apply a preset (increment usage count)
#[utoipa::path(
    post,
    path = "/api/cms/presets/{id}/apply",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    responses(
        (status = 200, description = "Preset applied successfully", body = CmsPreset),
        (status = 404, description = "Preset not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn apply_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    // Increment usage count and return updated preset
    let preset: CmsPreset = sqlx::query_as(
        r"
        UPDATE cms_presets
        SET usage_count = usage_count + 1
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    tracing::debug!(
        "Preset applied: {} ({}) - usage count: {}",
        preset.name,
        preset.id,
        preset.usage_count
    );

    Ok(Json(preset))
}

#[utoipa::path(
    post,
    path = "/api/cms/presets/save-from-block",
    tag = "CMS Presets",
    request_body = SaveBlockAsPresetRequest,
    responses(
        (status = 201, description = "Preset created from block", body = CmsPreset),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn save_block_as_preset(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<SaveBlockAsPresetRequest>,
) -> Result<(StatusCode, Json<CmsPreset>), ApiError> {
    require_cms_editor(&user)?;

    // Validate required fields
    if request.name.trim().is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Name is required")
            .with_code("VALIDATION_ERROR"));
    }

    if request.block_type.trim().is_empty() {
        return Err(
            ApiError::new(StatusCode::BAD_REQUEST, "Block type is required")
                .with_code("VALIDATION_ERROR"),
        );
    }

    // Combine content and settings into preset_data
    let preset_data = json!({
        "content": request.content,
        "settings": request.settings
    });

    // Generate slug
    let base_slug = generate_slug(&request.name);
    let slug = ensure_unique_slug(&state.db.pool, &request.block_type, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    let new_id = Uuid::new_v4();
    let category = request.category.unwrap_or(CmsPresetCategory::Custom);

    let preset: CmsPreset = sqlx::query_as(
        r"
        INSERT INTO cms_presets (
            id, block_type, name, slug, description, preset_data,
            thumbnail_url, category, tags,
            is_default, is_global, is_locked, usage_count, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, false, true, false, 0, 1, NOW(), NOW(), $10)
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        ",
    )
    .bind(new_id)
    .bind(request.block_type.trim())
    .bind(request.name.trim())
    .bind(&slug)
    .bind(&request.description)
    .bind(&preset_data)
    .bind(&request.thumbnail_url)
    .bind(category)
    .bind(&request.tags)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Preset saved from block: {} ({}) for block type {}",
        preset.name,
        preset.id,
        preset.block_type
    );

    Ok((StatusCode::CREATED, Json(preset)))
}
