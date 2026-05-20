//! Usage-tracking handlers: list usages for a block, track a new usage,
//! remove a usage record, and detach a usage instance from sync.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::require_cms_editor;
use super::types::{
    ApiResult, ApiResultEmpty, BlockUsageResponse, CmsReusableBlockUsage,
    CmsReusableBlockUsageWithContent, TrackBlockUsageRequest,
};

/// Get usage locations for a reusable block
#[utoipa::path(
    get,
    path = "/api/cms/reusable-blocks/{id}/usage",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Block UUID")
    ),
    responses(
        (status = 200, description = "Block usage locations", body = BlockUsageResponse),
        (status = 404, description = "Block not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_block_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<BlockUsageResponse> {
    require_cms_editor(&user)?;

    // Verify block exists
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM cms_reusable_blocks WHERE id = $1 AND deleted_at IS NULL) as \"exists!\"",
        id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if !exists {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND"),
        );
    }

    // Fetch usage with content metadata
    let usages: Vec<CmsReusableBlockUsageWithContent> = sqlx::query_as!(
        CmsReusableBlockUsageWithContent,
        r#"
        SELECT
            u.id,
            u.reusable_block_id,
            u.content_id,
            c.title as content_title,
            c.content_type::text as "content_type!",
            c.slug as content_slug,
            u.block_instance_id,
            u.is_synced,
            u.detached_at,
            u.created_at
        FROM cms_reusable_block_usage u
        JOIN cms_content c ON c.id = u.content_id
        WHERE u.reusable_block_id = $1
        ORDER BY u.created_at DESC
        "#,
        id
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_count = usages.len() as i64;

    Ok(Json(BlockUsageResponse {
        usages,
        total_count,
    }))
}

/// Track usage when a reusable block is inserted into content
#[utoipa::path(
    post,
    path = "/api/cms/reusable-blocks/usage",
    tag = "CMS Reusable Blocks",
    request_body = TrackBlockUsageRequest,
    responses(
        (status = 201, description = "Usage tracked successfully", body = CmsReusableBlockUsage),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Block or content not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn track_block_usage(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<TrackBlockUsageRequest>,
) -> Result<(StatusCode, Json<CmsReusableBlockUsage>), ApiError> {
    require_cms_editor(&user)?;

    // Verify block exists
    let block_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM cms_reusable_blocks WHERE id = $1 AND deleted_at IS NULL) as \"exists!\"",
        request.reusable_block_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !block_exists {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND"),
        );
    }

    // Verify content exists
    let content_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1 AND deleted_at IS NULL) as \"exists!\"",
        request.content_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !content_exists {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Content not found").with_code("NOT_FOUND")
        );
    }

    // Create usage record
    let usage = sqlx::query_as!(
        CmsReusableBlockUsage,
        r#"
        INSERT INTO cms_reusable_block_usage (
            id, reusable_block_id, content_id, block_instance_id, is_synced, created_at
        )
        VALUES ($1, $2, $3, $4, $5, NOW())
        RETURNING id, reusable_block_id, content_id, block_instance_id, is_synced,
                  detached_at, detached_by, created_at
        "#,
        Uuid::new_v4(),
        request.reusable_block_id,
        request.content_id,
        request.block_instance_id,
        request.is_synced
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Increment usage count on the block
    sqlx::query!(
        "UPDATE cms_reusable_blocks SET usage_count = usage_count + 1 WHERE id = $1",
        request.reusable_block_id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Block usage tracked: block {} in content {} (instance {})",
        request.reusable_block_id,
        request.content_id,
        request.block_instance_id
    );

    Ok((StatusCode::CREATED, Json(usage)))
}

/// Remove usage tracking when a block is removed from content
#[utoipa::path(
    delete,
    path = "/api/cms/reusable-blocks/usage/{id}",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Usage record UUID")
    ),
    responses(
        (status = 200, description = "Usage removed successfully"),
        (status = 404, description = "Usage record not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn remove_block_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Get usage record to find the block ID
    let usage = sqlx::query!(
        "SELECT reusable_block_id FROM cms_reusable_block_usage WHERE id = $1",
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    // Delete the usage record
    sqlx::query!("DELETE FROM cms_reusable_block_usage WHERE id = $1", id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    // Decrement usage count on the block
    sqlx::query!(
        "UPDATE cms_reusable_blocks SET usage_count = GREATEST(0, usage_count - 1) WHERE id = $1",
        usage.reusable_block_id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Block usage removed: usage {} from block {}",
        id,
        usage.reusable_block_id
    );

    Ok(Json(json!({
        "message": "Usage removed successfully",
        "id": id
    })))
}

/// Detach a block instance from sync (keeps content but stops updates)
#[utoipa::path(
    post,
    path = "/api/cms/reusable-blocks/usage/{id}/detach",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Usage record UUID")
    ),
    responses(
        (status = 200, description = "Block instance detached from sync", body = CmsReusableBlockUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Already detached")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn detach_block_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsReusableBlockUsage> {
    require_cms_editor(&user)?;

    // Check if already detached
    let existing = sqlx::query!(
        "SELECT is_synced FROM cms_reusable_block_usage WHERE id = $1",
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if !existing.is_synced {
        return Err(
            ApiError::new(StatusCode::CONFLICT, "Block instance is already detached")
                .with_code("ALREADY_DETACHED"),
        );
    }

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar!("SELECT id FROM cms_users WHERE user_id = $1", user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    // Detach the usage
    let usage = sqlx::query_as!(
        CmsReusableBlockUsage,
        r#"
        UPDATE cms_reusable_block_usage
        SET is_synced = false, detached_at = NOW(), detached_by = $2
        WHERE id = $1
        RETURNING id, reusable_block_id, content_id, block_instance_id, is_synced,
                  detached_at, detached_by, created_at
        "#,
        id,
        cms_user_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Block usage detached: {} from block {}",
        id,
        usage.reusable_block_id
    );

    Ok(Json(usage))
}
