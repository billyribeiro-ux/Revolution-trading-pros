//! Public, no-auth read-only endpoints for global components.
//!
//! Returns only components that are marked `is_global = true` so the
//! delivery side can render them without leaking unpublished work.

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;

use super::types::{not_found_component, sqlx_err, ApiResult, GlobalComponent};

/// Public endpoint to get a global component by slug (no auth required)
pub(super) async fn get_global_component_public(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<GlobalComponent> {
    let component: GlobalComponent = sqlx::query_as(
        r"
        SELECT id, name, slug, description, component_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE slug = $1 AND deleted_at IS NULL AND is_global = true
        ",
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    Ok(Json(component))
}

/// Public endpoint to get a global component by ID (no auth required)
pub(super) async fn get_global_component_by_id_public(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponent> {
    let component: GlobalComponent = sqlx::query_as(
        r"
        SELECT id, name, slug, description, component_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE id = $1 AND deleted_at IS NULL AND is_global = true
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    Ok(Json(component))
}
