//! Single-content schedule CRUD handlers.
//!
//! Six handlers covering the full lifecycle of an individual scheduled
//! content action (publish / unpublish / archive / update):
//! `create_schedule`, `list_schedules`, `get_schedule`,
//! `update_schedule`, `cancel_schedule`, `delete_schedule`.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{models::User, AppState};

use super::{
    api_error, require_cms_admin, require_cms_editor, ApiResult, CmsSchedule, CmsScheduleExtended,
    CreateScheduleRequest, ListQuery, ScheduleListResponse, ScheduleStatus, UpdateScheduleRequest,
};

/// Create a new content schedule
#[utoipa::path(
    post,
    path = "/api/cms/schedules",
    request_body = CreateScheduleRequest,
    responses(
        (status = 201, description = "Schedule created", body = CmsSchedule),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Content not found")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn create_schedule(
    State(state): State<AppState>,
    user: User,
    Json(req): Json<CreateScheduleRequest>,
) -> ApiResult<CmsSchedule> {
    require_cms_editor(&user)?;

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

    // Validate scheduled time is in the future
    if req.scheduled_at <= Utc::now() {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Scheduled time must be in the future",
        ));
    }

    // Create the schedule
    let schedule: CmsSchedule = sqlx::query_as(
        r"
        INSERT INTO cms_schedules (
            content_id, action, scheduled_at, timezone, staged_data, notes, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        ",
    )
    .bind(req.content_id)
    .bind(&req.action)
    .bind(req.scheduled_at)
    .bind(&req.timezone)
    .bind(&req.staged_data)
    .bind(&req.notes)
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Log to history
    let _ = sqlx::query(
        r"
        INSERT INTO cms_schedule_history (
            schedule_id, content_id, event_type, new_status, user_id, event_data
        ) VALUES ($1, $2, 'schedule_created', 'pending', $3, $4)
        ",
    )
    .bind(schedule.id)
    .bind(req.content_id)
    .bind(user.id)
    .bind(json!({
        "action": req.action,
        "scheduled_at": req.scheduled_at,
        "timezone": req.timezone
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(schedule))
}

/// List schedules with filters
#[utoipa::path(
    get,
    path = "/api/cms/schedules",
    params(ListQuery),
    responses(
        (status = 200, description = "Schedule list", body = ScheduleListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn list_schedules(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListQuery>,
) -> ApiResult<ScheduleListResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.min(100);

    // Build dynamic query
    let mut sql = String::from(
        r"
        SELECT
            s.id, s.content_id, s.action, s.scheduled_at, s.timezone,
            s.status, s.executed_at, s.error_message, s.notes,
            s.created_by, s.created_at,
            c.title as content_title, c.slug as content_slug,
            ct.name as content_type_name,
            u.display_name as created_by_name
        FROM cms_schedules s
        LEFT JOIN cms_content c ON s.content_id = c.id
        LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
        LEFT JOIN users u ON s.created_by = u.id
        WHERE 1=1
        ",
    );

    // Build parameterized query to prevent SQL injection
    let mut param_idx = 1;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref status) = query.status {
        sql.push_str(&format!(" AND s.status = ${param_idx}"));
        bind_values.push(status.clone());
        param_idx += 1;
    }
    if let Some(content_id) = query.content_id {
        sql.push_str(&format!(" AND s.content_id = ${param_idx}"));
        bind_values.push(content_id.to_string());
        param_idx += 1;
    }

    sql.push_str(&format!(
        " ORDER BY s.scheduled_at DESC LIMIT ${} OFFSET ${}",
        param_idx,
        param_idx + 1
    ));

    // Build and execute parameterized query
    let mut query_builder = sqlx::query_as::<_, CmsScheduleExtended>(&sql);
    for val in &bind_values {
        query_builder = query_builder.bind(val);
    }
    query_builder = query_builder.bind(limit).bind(query.offset);

    let schedules: Vec<CmsScheduleExtended> = query_builder
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get total count with parameterized query
    let mut count_sql = String::from("SELECT COUNT(*) FROM cms_schedules s WHERE 1=1");
    let mut count_param_idx = 1;
    let mut count_bind_values: Vec<String> = Vec::new();

    if let Some(ref status) = query.status {
        count_sql.push_str(&format!(" AND s.status = ${count_param_idx}"));
        count_bind_values.push(status.clone());
        count_param_idx += 1;
    }
    if let Some(content_id) = query.content_id {
        count_sql.push_str(&format!(" AND s.content_id = ${count_param_idx}"));
        count_bind_values.push(content_id.to_string());
    }

    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for val in &count_bind_values {
        count_query = count_query.bind(val);
    }

    let total: (i64,) = count_query.fetch_one(&state.db.pool).await.unwrap_or((0,));

    Ok(Json(ScheduleListResponse {
        schedules,
        total: total.0,
        limit,
        offset: query.offset,
        has_more: query.offset + limit < total.0,
    }))
}

/// Get a specific schedule
#[utoipa::path(
    get,
    path = "/api/cms/schedules/{id}",
    params(("id" = Uuid, Path, description = "Schedule ID")),
    responses(
        (status = 200, description = "Schedule details", body = CmsScheduleExtended),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Schedule not found")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn get_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsScheduleExtended> {
    require_cms_editor(&user)?;

    let schedule: CmsScheduleExtended = sqlx::query_as(
        r"
        SELECT
            s.id, s.content_id, s.action, s.scheduled_at, s.timezone,
            s.status, s.executed_at, s.error_message, s.notes,
            s.created_by, s.created_at,
            c.title as content_title, c.slug as content_slug,
            ct.name as content_type_name,
            u.display_name as created_by_name
        FROM cms_schedules s
        LEFT JOIN cms_content c ON s.content_id = c.id
        LEFT JOIN cms_content_types ct ON c.content_type_id = ct.id
        LEFT JOIN users u ON s.created_by = u.id
        WHERE s.id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Schedule not found"))?;

    Ok(Json(schedule))
}

/// Update a schedule (only pending schedules can be updated)
#[utoipa::path(
    put,
    path = "/api/cms/schedules/{id}",
    params(("id" = Uuid, Path, description = "Schedule ID")),
    request_body = UpdateScheduleRequest,
    responses(
        (status = 200, description = "Schedule updated", body = CmsSchedule),
        (status = 400, description = "Cannot update non-pending schedule"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Schedule not found")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn update_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateScheduleRequest>,
) -> ApiResult<CmsSchedule> {
    require_cms_editor(&user)?;

    // Verify schedule exists and is pending
    let existing: Option<CmsSchedule> = sqlx::query_as("SELECT * FROM cms_schedules WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let existing =
        existing.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Schedule not found"))?;

    if !matches!(existing.status, ScheduleStatus::Pending) {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Only pending schedules can be updated",
        ));
    }

    // Validate new scheduled time if provided
    if let Some(ref scheduled_at) = req.scheduled_at {
        if *scheduled_at <= Utc::now() {
            return Err(api_error(
                StatusCode::BAD_REQUEST,
                "Scheduled time must be in the future",
            ));
        }
    }

    let schedule: CmsSchedule = sqlx::query_as(
        r"
        UPDATE cms_schedules SET
            action = COALESCE($2, action),
            scheduled_at = COALESCE($3, scheduled_at),
            timezone = COALESCE($4, timezone),
            staged_data = COALESCE($5, staged_data),
            notes = COALESCE($6, notes)
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(id)
    .bind(&req.action)
    .bind(req.scheduled_at)
    .bind(&req.timezone)
    .bind(&req.staged_data)
    .bind(&req.notes)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Log to history
    let _ = sqlx::query(
        r"
        INSERT INTO cms_schedule_history (
            schedule_id, content_id, event_type, user_id, event_data
        ) VALUES ($1, $2, 'schedule_updated', $3, $4)
        ",
    )
    .bind(id)
    .bind(existing.content_id)
    .bind(user.id)
    .bind(json!({
        "changes": {
            "action": req.action,
            "scheduled_at": req.scheduled_at,
            "timezone": req.timezone
        }
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(schedule))
}

/// Cancel a schedule
#[utoipa::path(
    post,
    path = "/api/cms/schedules/{id}/cancel",
    params(("id" = Uuid, Path, description = "Schedule ID")),
    responses(
        (status = 200, description = "Schedule cancelled", body = CmsSchedule),
        (status = 400, description = "Cannot cancel non-pending schedule"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Schedule not found")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn cancel_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsSchedule> {
    require_cms_editor(&user)?;

    let schedule: CmsSchedule = sqlx::query_as(
        r"
        UPDATE cms_schedules SET
            status = 'cancelled',
            cancelled_by = $2,
            cancelled_at = NOW()
        WHERE id = $1 AND status = 'pending'
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
            "Schedule not found or cannot be cancelled",
        )
    })?;

    // Log to history
    let _ = sqlx::query(
        r"
        INSERT INTO cms_schedule_history (
            schedule_id, content_id, event_type, previous_status, new_status, user_id
        ) VALUES ($1, $2, 'schedule_cancelled', 'pending', 'cancelled', $3)
        ",
    )
    .bind(id)
    .bind(schedule.content_id)
    .bind(user.id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(schedule))
}

/// Delete a schedule (only cancelled/completed/failed)
#[utoipa::path(
    delete,
    path = "/api/cms/schedules/{id}",
    params(("id" = Uuid, Path, description = "Schedule ID")),
    responses(
        (status = 204, description = "Schedule deleted"),
        (status = 400, description = "Cannot delete pending schedule"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Schedule not found")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn delete_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<JsonValue>)> {
    require_cms_admin(&user)?;

    let result = sqlx::query(
        r"
        DELETE FROM cms_schedules
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
            "Schedule not found or cannot be deleted (must be cancelled, completed, or failed)",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
