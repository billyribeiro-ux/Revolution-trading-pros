//! Version history handlers for global components.
//!
//! - `get_version_history` — list all versions of a component
//! - `restore_version`     — restore a prior version as a new current version

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::{
    create_version_record, get_cms_user_id, require_cms_admin, require_cms_editor,
};
use super::types::{
    not_found_component, sqlx_err, ApiResult, GlobalComponent, GlobalComponentVersion,
    VersionHistoryResponse,
};

/// Get version history for a global component
#[utoipa::path(
    get,
    path = "/api/cms/global-components/{id}/versions",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    responses(
        (status = 200, description = "Version history", body = VersionHistoryResponse),
        (status = 404, description = "Component not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_version_history(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<VersionHistoryResponse> {
    require_cms_editor(&user)?;

    // Get current version
    #[derive(Debug, sqlx::FromRow)]
    struct CurrentVersion {
        version: i32,
    }
    let current: CurrentVersion = sqlx::query_as(
        "SELECT version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    // Get all versions
    let versions: Vec<GlobalComponentVersion> = sqlx::query_as(
        r"
        SELECT v.id, v.component_id, v.version, v.component_data,
               v.change_message, v.created_by, u.name as created_by_name, v.created_at
        FROM cms_global_component_versions v
        LEFT JOIN cms_users u ON u.id = v.created_by
        WHERE v.component_id = $1
        ORDER BY v.version DESC
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    Ok(Json(VersionHistoryResponse {
        total_count: versions.len() as i64,
        versions,
        current_version: current.version,
    }))
}

/// Restore a specific version of a global component
#[utoipa::path(
    post,
    path = "/api/cms/global-components/{id}/versions/{version}/restore",
    tag = "CMS Global Components",
    params(
        ("id" = Uuid, Path, description = "Component UUID"),
        ("version" = i32, Path, description = "Version number to restore")
    ),
    responses(
        (status = 200, description = "Version restored successfully", body = GlobalComponent),
        (status = 404, description = "Component or version not found"),
        (status = 423, description = "Component is locked")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn restore_version(
    State(state): State<AppState>,
    user: User,
    Path((id, version)): Path<(Uuid, i32)>,
) -> ApiResult<GlobalComponent> {
    require_cms_editor(&user)?;

    // Check if component exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingComponent {
        is_locked: bool,
        current_version: i32,
    }
    let existing: ExistingComponent = sqlx::query_as(
        "SELECT is_locked, version as current_version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    if existing.is_locked {
        require_cms_admin(&user)?;
    }

    // Get the version to restore
    #[derive(Debug, sqlx::FromRow)]
    struct VersionRecord {
        component_data: JsonValue,
    }
    let version_record: VersionRecord = sqlx::query_as(
        "SELECT component_data FROM cms_global_component_versions WHERE component_id = $1 AND version = $2",
    )
    .bind(id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Version not found").with_code("VERSION_NOT_FOUND")
    })?;

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;
    let new_version = existing.current_version + 1;

    // Update component with restored data
    let component: GlobalComponent = sqlx::query_as(
        r"
        UPDATE cms_global_components
        SET component_data = $2,
            version = $3,
            updated_at = NOW(),
            updated_by = $4
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        ",
    )
    .bind(id)
    .bind(&version_record.component_data)
    .bind(new_version)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    // Create version record for restoration
    create_version_record(
        &state.db.pool,
        id,
        new_version,
        &version_record.component_data,
        Some(&format!("Restored from version {version}")),
        cms_user_id,
    )
    .await?;

    tracing::info!(
        "Global component version restored: {} ({}) v{} -> v{} by user {}",
        component.name,
        id,
        version,
        new_version,
        user.id
    );

    Ok(Json(component))
}
