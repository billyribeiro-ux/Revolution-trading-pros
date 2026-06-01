//! No-auth public read-only block access (block previews).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{utils::errors::ApiError, AppState};

use super::types::{ApiResult, CmsReusableBlock};

/// Public endpoint to get a global block by slug (no auth required)
pub(super) async fn get_reusable_block_by_slug_public(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<CmsReusableBlock> {
    let block: CmsReusableBlock = sqlx::query_as(
        r"
        SELECT id, name, slug, description, block_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE slug = $1 AND deleted_at IS NULL AND is_global = true
        ",
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(block))
}
