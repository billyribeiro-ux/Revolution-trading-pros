//! Write-side mutations for the CMS presets API — create, update,
//! and soft-delete a preset.
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

use super::dto::{CmsPreset, CmsPresetCategory, CreatePresetRequest, UpdatePresetRequest};
use super::helpers::{
    ensure_unique_slug, generate_slug, require_cms_admin, require_cms_editor, ApiResult,
    ApiResultEmpty,
};

/// Create a new preset
#[utoipa::path(
    post,
    path = "/api/cms/presets",
    tag = "CMS Presets",
    request_body = CreatePresetRequest,
    responses(
        (status = 201, description = "Preset created successfully", body = CmsPreset),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Slug already exists")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn create_preset(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreatePresetRequest>,
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

    // Generate or validate slug
    let base_slug = request
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&request.name));
    let slug = ensure_unique_slug(&state.db.pool, &request.block_type, &base_slug, None).await?;

    // Get CMS user ID from platform user
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    // If setting as default, clear other defaults for this block type
    if request.is_default {
        sqlx::query(
            "UPDATE cms_presets SET is_default = false WHERE block_type = $1 AND is_default = true",
        )
        .bind(&request.block_type)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    }

    let new_id = Uuid::new_v4();
    let category = request.category.unwrap_or(CmsPresetCategory::Custom);

    let preset: CmsPreset = sqlx::query_as(
        r"
        INSERT INTO cms_presets (
            id, block_type, name, slug, description, preset_data,
            thumbnail_url, thumbnail_blurhash, category, tags,
            is_default, is_global, is_locked, usage_count, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, false, 0, 1, NOW(), NOW(), $13)
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
    .bind(&request.preset_data)
    .bind(&request.thumbnail_url)
    .bind(&request.thumbnail_blurhash)
    .bind(category)
    .bind(&request.tags)
    .bind(request.is_default)
    .bind(request.is_global)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Preset created: {} ({}) for block type {}",
        preset.name,
        preset.id,
        preset.block_type
    );

    Ok((StatusCode::CREATED, Json(preset)))
}

/// Update a preset
#[utoipa::path(
    put,
    path = "/api/cms/presets/{id}",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    request_body = UpdatePresetRequest,
    responses(
        (status = 200, description = "Preset updated successfully", body = CmsPreset),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Preset not found"),
        (status = 409, description = "Slug already exists"),
        (status = 423, description = "Preset is locked")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn update_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePresetRequest>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    // Check if preset exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingPreset {
        is_locked: bool,
        block_type: String,
    }

    let existing: ExistingPreset = sqlx::query_as(
        "SELECT is_locked, block_type FROM cms_presets WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    // Only admins can update locked presets
    if existing.is_locked && request.is_locked != Some(false) {
        require_cms_admin(&user)?;
    }

    // Handle slug update
    let final_slug = if let Some(ref new_slug) = request.slug {
        Some(ensure_unique_slug(&state.db.pool, &existing.block_type, new_slug, Some(id)).await?)
    } else {
        None
    };

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    // If setting as default, clear other defaults for this block type
    if request.is_default == Some(true) {
        sqlx::query(
            "UPDATE cms_presets SET is_default = false WHERE block_type = $1 AND is_default = true AND id != $2"
        )
        .bind(&existing.block_type)
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    }

    let preset: CmsPreset = sqlx::query_as(
        r"
        UPDATE cms_presets
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            preset_data = COALESCE($5, preset_data),
            thumbnail_url = COALESCE($6, thumbnail_url),
            thumbnail_blurhash = COALESCE($7, thumbnail_blurhash),
            category = COALESCE($8, category),
            tags = COALESCE($9, tags),
            is_default = COALESCE($10, is_default),
            is_global = COALESCE($11, is_global),
            is_locked = COALESCE($12, is_locked),
            version = version + 1,
            updated_at = NOW(),
            updated_by = $13
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        ",
    )
    .bind(id)
    .bind(&request.name)
    .bind(&final_slug)
    .bind(&request.description)
    .bind(&request.preset_data)
    .bind(&request.thumbnail_url)
    .bind(&request.thumbnail_blurhash)
    .bind(request.category)
    .bind(&request.tags)
    .bind(request.is_default)
    .bind(request.is_global)
    .bind(request.is_locked)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!("Preset updated: {} ({})", preset.name, preset.id);

    Ok(Json(preset))
}

/// Delete a preset (soft delete)
#[utoipa::path(
    delete,
    path = "/api/cms/presets/{id}",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    responses(
        (status = 200, description = "Preset deleted successfully"),
        (status = 404, description = "Preset not found"),
        (status = 423, description = "Preset is locked")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn delete_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    // Check if preset exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct DeletePreset {
        is_locked: bool,
        name: String,
    }

    let existing: DeletePreset = sqlx::query_as(
        "SELECT is_locked, name FROM cms_presets WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    if existing.is_locked {
        return Err(ApiError::new(
            StatusCode::LOCKED,
            "Cannot delete a locked preset. Unlock it first.",
        )
        .with_code("PRESET_LOCKED"));
    }

    // Soft delete
    sqlx::query("UPDATE cms_presets SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    tracing::info!("Preset deleted: {} ({})", existing.name, id);

    Ok(Json(json!({
        "message": "Preset deleted successfully",
        "id": id
    })))
}
