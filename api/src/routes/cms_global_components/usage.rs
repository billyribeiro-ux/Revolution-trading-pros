//! Usage-tracking handlers for global component instances.
//!
//! - `get_component_usage`    — list all instances of a component across content
//! - `track_component_usage`  — register that a component is now placed in content
//! - `remove_component_usage` — drop a tracking row when the instance is deleted

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::require_cms_editor;
use super::types::{
    not_found_component, sqlx_err, ApiResult, ApiResultEmpty, ComponentUsageResponse,
    GlobalComponentUsage, GlobalComponentUsageWithContent, TrackComponentUsageRequest,
};

/// Get usage locations for a global component
#[utoipa::path(
    get,
    path = "/api/cms/global-components/{id}/usage",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    responses(
        (status = 200, description = "Component usage locations", body = ComponentUsageResponse),
        (status = 404, description = "Component not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<ComponentUsageResponse> {
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

    // Fetch usage with content metadata
    #[derive(Debug, sqlx::FromRow)]
    struct UsageRow {
        id: Uuid,
        component_id: Uuid,
        content_id: Uuid,
        content_title: String,
        content_type: String,
        content_slug: String,
        instance_id: String,
        is_synced: bool,
        synced_version: i32,
        needs_update: bool,
        detached_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    }
    let usage_rows: Vec<UsageRow> = sqlx::query_as(
        r"
        SELECT
            u.id,
            u.component_id,
            u.content_id,
            c.title as content_title,
            c.content_type::text as content_type,
            c.slug as content_slug,
            u.instance_id,
            u.is_synced,
            u.synced_version,
            (u.is_synced AND u.synced_version < $2) as needs_update,
            u.detached_at,
            u.created_at
        FROM cms_global_component_usage u
        JOIN cms_content c ON c.id = u.content_id
        WHERE u.component_id = $1
        ORDER BY u.created_at DESC
        ",
    )
    .bind(id)
    .bind(current.version)
    .fetch_all(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    let usages: Vec<GlobalComponentUsageWithContent> = usage_rows
        .into_iter()
        .map(|r| GlobalComponentUsageWithContent {
            id: r.id,
            component_id: r.component_id,
            content_id: r.content_id,
            content_title: r.content_title,
            content_type: r.content_type,
            content_slug: r.content_slug,
            instance_id: r.instance_id,
            is_synced: r.is_synced,
            synced_version: r.synced_version,
            needs_update: r.needs_update,
            detached_at: r.detached_at,
            created_at: r.created_at,
        })
        .collect();

    let total_count = usages.len() as i64;
    let synced_count = usages.iter().filter(|u| u.is_synced).count() as i64;
    let outdated_count = usages.iter().filter(|u| u.needs_update).count() as i64;

    Ok(Json(ComponentUsageResponse {
        usages,
        total_count,
        synced_count,
        outdated_count,
    }))
}

/// Track usage when a global component is inserted into content
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage",
    tag = "CMS Global Components",
    request_body = TrackComponentUsageRequest,
    responses(
        (status = 201, description = "Usage tracked successfully", body = GlobalComponentUsage),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Component or content not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn track_component_usage(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<TrackComponentUsageRequest>,
) -> Result<(StatusCode, Json<GlobalComponentUsage>), ApiError> {
    require_cms_editor(&user)?;

    // Get component and its current version
    #[derive(Debug, sqlx::FromRow)]
    struct ComponentVersion {
        version: i32,
    }
    let component: ComponentVersion = sqlx::query_as(
        "SELECT version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(request.component_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    // Verify content exists
    let content_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(request.content_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    if !content_exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Content not found").with_code("NOT_FOUND")
        );
    }

    // Create usage record
    let usage: GlobalComponentUsage = sqlx::query_as(
        r"
        INSERT INTO cms_global_component_usage (
            id, component_id, content_id, instance_id, is_synced, synced_version, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        ",
    )
    .bind(Uuid::new_v4())
    .bind(request.component_id)
    .bind(request.content_id)
    .bind(&request.instance_id)
    .bind(request.is_synced)
    .bind(component.version)
    .fetch_one(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    // Increment usage count on the component
    sqlx::query("UPDATE cms_global_components SET usage_count = usage_count + 1 WHERE id = $1")
        .bind(request.component_id)
        .execute(&state.db.pool)
        .await
        .map_err(sqlx_err)?;

    tracing::info!(
        "Component usage tracked: component {} in content {} (instance {}) by user {}",
        request.component_id,
        request.content_id,
        request.instance_id,
        user.id
    );

    Ok((StatusCode::CREATED, Json(usage)))
}

/// Remove usage tracking when a component is removed from content
#[utoipa::path(
    delete,
    path = "/api/cms/global-components/usage/{id}",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    responses(
        (status = 200, description = "Usage removed successfully"),
        (status = 404, description = "Usage record not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn remove_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Get usage record to find the component ID
    #[derive(Debug, sqlx::FromRow)]
    struct UsageRecord {
        component_id: Uuid,
    }
    let usage: UsageRecord =
        sqlx::query_as("SELECT component_id FROM cms_global_component_usage WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(sqlx_err)?
            .ok_or_else(|| {
                ApiError::new(StatusCode::NOT_FOUND, "Usage record not found")
                    .with_code("NOT_FOUND")
            })?;

    // Delete the usage record
    sqlx::query("DELETE FROM cms_global_component_usage WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(sqlx_err)?;

    // Decrement usage count on the component
    sqlx::query(
        "UPDATE cms_global_components SET usage_count = GREATEST(0, usage_count - 1) WHERE id = $1",
    )
    .bind(usage.component_id)
    .execute(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

    tracing::info!(
        "Component usage removed: usage {} from component {} by user {}",
        id,
        usage.component_id,
        user.id
    );

    Ok(Json(json!({
        "message": "Usage removed successfully",
        "id": id
    })))
}
