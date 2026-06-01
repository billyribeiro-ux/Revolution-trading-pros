//! Sync / detach / reattach handlers for component instances.
//!
//! These operate on `cms_global_component_usage` rows to control whether the
//! per-content instance follows the library component or has been detached
//! to allow local edits.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::{get_cms_user_id, require_cms_editor};
use super::types::{sqlx_err, ApiResult, GlobalComponentUsage, SyncComponentRequest};

/// Sync a component instance to the latest version
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage/{id}/sync",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    request_body = SyncComponentRequest,
    responses(
        (status = 200, description = "Component synced to latest version", body = GlobalComponentUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Component is detached")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn sync_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(_request): Json<SyncComponentRequest>,
) -> ApiResult<GlobalComponentUsage> {
    require_cms_editor(&user)?;

    // Check if usage exists and is synced
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingUsage {
        is_synced: bool,
        component_id: Uuid,
        current_version: i32,
    }
    let existing: ExistingUsage = sqlx::query_as(
        r"
        SELECT u.is_synced, u.component_id, c.version as current_version
        FROM cms_global_component_usage u
        JOIN cms_global_components c ON c.id = u.component_id
        WHERE u.id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if !existing.is_synced {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "Component instance is detached. Re-attach it first to sync.",
        )
        .with_code("COMPONENT_DETACHED"));
    }

    // Update synced version
    let usage: GlobalComponentUsage = sqlx::query_as(
        r"
        UPDATE cms_global_component_usage
        SET synced_version = $2
        WHERE id = $1
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        ",
    )
    .bind(id)
    .bind(existing.current_version)
    .fetch_one(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    tracing::info!(
        "Component usage synced: {} to v{} by user {}",
        id,
        existing.current_version,
        user.id
    );

    Ok(Json(usage))
}

/// Detach a component instance from sync (copy as local)
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage/{id}/detach",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    responses(
        (status = 200, description = "Component instance detached from sync", body = GlobalComponentUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Already detached")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn detach_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponentUsage> {
    require_cms_editor(&user)?;

    // Check if already detached
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingUsage {
        is_synced: bool,
    }
    let existing: ExistingUsage =
        sqlx::query_as("SELECT is_synced FROM cms_global_component_usage WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(sqlx_err)?
            .ok_or_else(|| {
                ApiError::new(StatusCode::NOT_FOUND, "Usage record not found")
                    .with_code("NOT_FOUND")
            })?;

    if !existing.is_synced {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "Component instance is already detached",
        )
        .with_code("ALREADY_DETACHED"));
    }

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    // Detach the usage
    let usage: GlobalComponentUsage = sqlx::query_as(
        r"
        UPDATE cms_global_component_usage
        SET is_synced = false, detached_at = NOW(), detached_by = $2
        WHERE id = $1
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        ",
    )
    .bind(id)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    tracing::info!("Component usage detached: {} by user {}", id, user.id);

    Ok(Json(usage))
}

/// Re-attach a detached component instance to sync
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage/{id}/reattach",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    responses(
        (status = 200, description = "Component instance re-attached to sync", body = GlobalComponentUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Already synced")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn reattach_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponentUsage> {
    require_cms_editor(&user)?;

    // Check if already synced
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingUsage {
        is_synced: bool,
        current_version: i32,
    }
    let existing: ExistingUsage = sqlx::query_as(
        r"
        SELECT u.is_synced, c.version as current_version
        FROM cms_global_component_usage u
        JOIN cms_global_components c ON c.id = u.component_id
        WHERE u.id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if existing.is_synced {
        return Err(
            ApiError::new(StatusCode::CONFLICT, "Component instance is already synced")
                .with_code("ALREADY_SYNCED"),
        );
    }

    // Re-attach and sync to latest version
    let usage: GlobalComponentUsage = sqlx::query_as(
        r"
        UPDATE cms_global_component_usage
        SET is_synced = true, synced_version = $2, detached_at = NULL, detached_by = NULL
        WHERE id = $1
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        ",
    )
    .bind(id)
    .bind(existing.current_version)
    .fetch_one(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    tracing::info!(
        "Component usage re-attached: {} to v{} by user {}",
        id,
        existing.current_version,
        user.id
    );

    Ok(Json(usage))
}
