//! Release bundle CRUD + item operations.
//!
//! Nine handlers covering the full lifecycle of a release bundle —
//! a coordinated multi-content publish/unpublish/archive/update job:
//! `create_release`, `list_releases`, `get_release`, `update_release`,
//! `add_release_item`, `remove_release_item`, `schedule_release`,
//! `cancel_release`, `delete_release`.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value as JsonValue};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{models::User, AppState};

use super::{
    api_error, default_timezone, require_cms_admin, require_cms_editor, AddReleaseItemRequest,
    ApiResult, CmsRelease, CmsReleaseItem, CmsReleaseItemExtended, CreateReleaseRequest, ListQuery,
    ReleaseDetailResponse, ReleaseListResponse, ReleaseStatus, UpdateReleaseRequest,
};

/// Create a new release
#[utoipa::path(
    post,
    path = "/api/cms/releases",
    request_body = CreateReleaseRequest,
    responses(
        (status = 201, description = "Release created", body = CmsRelease),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn create_release(
    State(state): State<AppState>,
    user: User,
    Json(req): Json<CreateReleaseRequest>,
) -> ApiResult<CmsRelease> {
    require_cms_editor(&user)?;

    let status = if req.scheduled_at.is_some() {
        ReleaseStatus::Scheduled
    } else {
        ReleaseStatus::Draft
    };

    let release: CmsRelease = sqlx::query_as(
        r"
        INSERT INTO cms_releases (
            name, description, scheduled_at, timezone, status,
            stop_on_error, notify_on_complete, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        ",
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.scheduled_at)
    .bind(&req.timezone)
    .bind(&status)
    .bind(req.stop_on_error)
    .bind(req.notify_on_complete)
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Log to history
    let _ = sqlx::query(
        r"
        INSERT INTO cms_schedule_history (
            release_id, event_type, new_status, user_id, event_data
        ) VALUES ($1, 'release_created', $2, $3, $4)
        ",
    )
    .bind(release.id)
    .bind(format!("{status:?}").to_lowercase())
    .bind(user.id)
    .bind(json!({
        "name": req.name,
        "scheduled_at": req.scheduled_at
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(release))
}

/// List releases
#[utoipa::path(
    get,
    path = "/api/cms/releases",
    params(ListQuery),
    responses(
        (status = 200, description = "Release list", body = ReleaseListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn list_releases(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListQuery>,
) -> ApiResult<ReleaseListResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.min(100);

    let mut sql = String::from("SELECT * FROM cms_releases WHERE 1=1");

    // Build parameterized query to prevent SQL injection
    if let Some(ref status) = query.status {
        sql.push_str(" AND status = $1");
    }

    sql.push_str(" ORDER BY created_at DESC");
    if query.status.is_some() {
        sql.push_str(" LIMIT $2 OFFSET $3");
    } else {
        sql.push_str(" LIMIT $1 OFFSET $2");
    }

    let releases: Vec<CmsRelease> = if let Some(ref status) = query.status {
        sqlx::query_as(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(status)
            .bind(limit)
            .bind(query.offset)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    } else {
        sqlx::query_as(sqlx::AssertSqlSafe(sql.as_str()))
            .bind(limit)
            .bind(query.offset)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    };

    // Count query with parameterized binding
    let total: (i64,) = if let Some(ref status) = query.status {
        sqlx::query_as("SELECT COUNT(*) FROM cms_releases WHERE status = $1")
            .bind(status)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,))
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM cms_releases WHERE 1=1")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,))
    };

    Ok(Json(ReleaseListResponse {
        releases,
        total: total.0,
        limit,
        offset: query.offset,
        has_more: query.offset + limit < total.0,
    }))
}

/// Get release details with items
#[utoipa::path(
    get,
    path = "/api/cms/releases/{id}",
    params(("id" = Uuid, Path, description = "Release ID")),
    responses(
        (status = 200, description = "Release details", body = ReleaseDetailResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Release not found")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn get_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<ReleaseDetailResponse> {
    require_cms_editor(&user)?;

    let release: CmsRelease = sqlx::query_as("SELECT * FROM cms_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Release not found"))?;

    let items: Vec<CmsReleaseItemExtended> = sqlx::query_as(
        r"
        SELECT
            ri.id, ri.release_id, ri.content_id, ri.action, ri.order_index,
            ri.status, ri.executed_at, ri.error_message,
            c.title as content_title, c.slug as content_slug,
            ct.name as content_type_name
        FROM cms_release_items ri
        LEFT JOIN cms_content c ON ri.content_id = c.id
        LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
        WHERE ri.release_id = $1
        ORDER BY ri.order_index ASC
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get creator name
    let created_by_name: Option<(String,)> = if let Some(created_by) = release.created_by {
        sqlx::query_as("SELECT display_name FROM users WHERE id = $1")
            .bind(created_by)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    Ok(Json(ReleaseDetailResponse {
        release,
        items,
        created_by_name: created_by_name.map(|n| n.0),
    }))
}

/// Update a release
#[utoipa::path(
    put,
    path = "/api/cms/releases/{id}",
    params(("id" = Uuid, Path, description = "Release ID")),
    request_body = UpdateReleaseRequest,
    responses(
        (status = 200, description = "Release updated", body = CmsRelease),
        (status = 400, description = "Cannot update non-draft/scheduled release"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Release not found")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn update_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateReleaseRequest>,
) -> ApiResult<CmsRelease> {
    require_cms_editor(&user)?;

    // Determine new status based on scheduled_at
    let new_status = if req.scheduled_at.is_some() {
        Some(ReleaseStatus::Scheduled)
    } else {
        None
    };

    let release: CmsRelease = sqlx::query_as(
        r"
        UPDATE cms_releases SET
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            scheduled_at = COALESCE($4, scheduled_at),
            timezone = COALESCE($5, timezone),
            stop_on_error = COALESCE($6, stop_on_error),
            notify_on_complete = COALESCE($7, notify_on_complete),
            status = COALESCE($8, status)
        WHERE id = $1 AND status IN ('draft', 'scheduled')
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.scheduled_at)
    .bind(&req.timezone)
    .bind(req.stop_on_error)
    .bind(req.notify_on_complete)
    .bind(&new_status)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::BAD_REQUEST,
            "Release not found or cannot be updated",
        )
    })?;

    Ok(Json(release))
}

/// Add item to release
#[utoipa::path(
    post,
    path = "/api/cms/releases/{id}/items",
    params(("id" = Uuid, Path, description = "Release ID")),
    request_body = AddReleaseItemRequest,
    responses(
        (status = 201, description = "Item added", body = CmsReleaseItem),
        (status = 400, description = "Cannot add to non-draft release"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Release or content not found"),
        (status = 409, description = "Content already in release")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn add_release_item(
    State(state): State<AppState>,
    user: User,
    Path(release_id): Path<Uuid>,
    Json(req): Json<AddReleaseItemRequest>,
) -> ApiResult<CmsReleaseItem> {
    require_cms_editor(&user)?;

    // Verify release exists and is draft/scheduled
    let release: Option<CmsRelease> = sqlx::query_as("SELECT * FROM cms_releases WHERE id = $1")
        .bind(release_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let release = release.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Release not found"))?;

    if !matches!(
        release.status,
        ReleaseStatus::Draft | ReleaseStatus::Scheduled
    ) {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Can only add items to draft or scheduled releases",
        ));
    }

    // Verify content exists
    let content_exists: (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1)")
            .bind(req.content_id)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    if !content_exists.0 {
        return Err(api_error(StatusCode::NOT_FOUND, "Content not found"));
    }

    // Get next order index if not provided
    let order_index = if let Some(idx) = req.order_index {
        idx
    } else {
        let max_order: (Option<i32>,) =
            sqlx::query_as("SELECT MAX(order_index) FROM cms_release_items WHERE release_id = $1")
                .bind(release_id)
                .fetch_one(&state.db.pool)
                .await
                .unwrap_or((None,));
        max_order.0.unwrap_or(-1) + 1
    };

    let item: CmsReleaseItem = sqlx::query_as(
        r"
        INSERT INTO cms_release_items (
            release_id, content_id, action, order_index, staged_data
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        ",
    )
    .bind(release_id)
    .bind(req.content_id)
    .bind(&req.action)
    .bind(order_index)
    .bind(&req.staged_data)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
            api_error(StatusCode::CONFLICT, "Content already in this release")
        } else {
            api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string())
        }
    })?;

    Ok(Json(item))
}

/// Remove item from release
#[utoipa::path(
    delete,
    path = "/api/cms/releases/{release_id}/items/{item_id}",
    params(
        ("release_id" = Uuid, Path, description = "Release ID"),
        ("item_id" = Uuid, Path, description = "Item ID")
    ),
    responses(
        (status = 204, description = "Item removed"),
        (status = 400, description = "Cannot remove from non-draft release"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Item not found")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn remove_release_item(
    State(state): State<AppState>,
    user: User,
    Path((release_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, Json<JsonValue>)> {
    require_cms_editor(&user)?;

    let result = sqlx::query(
        r"
        DELETE FROM cms_release_items ri
        USING cms_releases r
        WHERE ri.id = $1
          AND ri.release_id = $2
          AND r.id = ri.release_id
          AND r.status IN ('draft', 'scheduled')
        ",
    )
    .bind(item_id)
    .bind(release_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "Item not found or release cannot be modified",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Schedule a draft release
#[utoipa::path(
    post,
    path = "/api/cms/releases/{id}/schedule",
    params(("id" = Uuid, Path, description = "Release ID")),
    request_body = inline(ScheduleReleaseRequest),
    responses(
        (status = 200, description = "Release scheduled", body = CmsRelease),
        (status = 400, description = "Cannot schedule empty or non-draft release"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Release not found")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn schedule_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(req): Json<ScheduleReleaseRequest>,
) -> ApiResult<CmsRelease> {
    require_cms_editor(&user)?;

    // Verify release has items
    let item_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cms_release_items WHERE release_id = $1")
            .bind(id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    if item_count.0 == 0 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Cannot schedule an empty release",
        ));
    }

    let release: CmsRelease = sqlx::query_as(
        r"
        UPDATE cms_releases SET
            scheduled_at = $2,
            timezone = $3,
            status = 'scheduled'
        WHERE id = $1 AND status = 'draft'
        RETURNING *
        ",
    )
    .bind(id)
    .bind(req.scheduled_at)
    .bind(&req.timezone)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::BAD_REQUEST,
            "Release not found or not in draft status",
        )
    })?;

    // Log to history
    let _ = sqlx::query(
        r"
        INSERT INTO cms_schedule_history (
            release_id, event_type, previous_status, new_status, user_id, event_data
        ) VALUES ($1, 'release_scheduled', 'draft', 'scheduled', $2, $3)
        ",
    )
    .bind(id)
    .bind(user.id)
    .bind(json!({
        "scheduled_at": req.scheduled_at,
        "timezone": req.timezone
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(release))
}

#[derive(Debug, Deserialize, ToSchema)]
pub(super) struct ScheduleReleaseRequest {
    pub(super) scheduled_at: DateTime<Utc>,
    #[serde(default = "default_timezone")]
    pub(super) timezone: String,
}

/// Cancel a release
#[utoipa::path(
    post,
    path = "/api/cms/releases/{id}/cancel",
    params(("id" = Uuid, Path, description = "Release ID")),
    responses(
        (status = 200, description = "Release cancelled", body = CmsRelease),
        (status = 400, description = "Cannot cancel non-scheduled release"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Release not found")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn cancel_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsRelease> {
    require_cms_editor(&user)?;

    let release: CmsRelease = sqlx::query_as(
        r"
        UPDATE cms_releases SET
            status = 'cancelled',
            cancelled_by = $2,
            cancelled_at = NOW()
        WHERE id = $1 AND status IN ('draft', 'scheduled')
        RETURNING *
        ",
    )
    .bind(id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::BAD_REQUEST,
            "Release not found or cannot be cancelled",
        )
    })?;

    Ok(Json(release))
}

/// Delete a release (only cancelled/completed/failed)
#[utoipa::path(
    delete,
    path = "/api/cms/releases/{id}",
    params(("id" = Uuid, Path, description = "Release ID")),
    responses(
        (status = 204, description = "Release deleted"),
        (status = 400, description = "Cannot delete active release"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Release not found")
    ),
    tag = "CMS Releases",
    security(("bearer_auth" = []))
)]
pub(super) async fn delete_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<JsonValue>)> {
    require_cms_admin(&user)?;

    let result = sqlx::query(
        r"
        DELETE FROM cms_releases
        WHERE id = $1 AND status IN ('cancelled', 'completed', 'failed')
        ",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Release not found or cannot be deleted",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
