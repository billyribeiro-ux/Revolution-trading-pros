//! CMS Scheduling Routes - Apple ICT 7+ Principal Engineer Grade
//!
//! Storyblok-style content scheduling and releases system:
//! - Schedule single content publish/unpublish operations
//! - Bundle multiple content changes into releases
//! - Timezone-aware scheduling with cron job support
//! - Complete audit trail for all scheduling operations
//!
//! Features:
//! - Individual content scheduling (publish, unpublish, archive, update)
//! - Release bundles for coordinated multi-content updates
//! - Calendar view of scheduled content
//! - Schedule history and audit log
//! - Cron job integration endpoints
//!
//! @version 1.0.0 - February 2026

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{models::User, AppState};

// =====================================================================================
// TYPE ALIASES AND ERROR HANDLING
// =====================================================================================

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;

fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (
        status,
        Json(json!({
            "error": message,
            "status": status.as_u16(),
            "timestamp": Utc::now().to_rfc3339()
        })),
    )
}

fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing"
    ) {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Editor access required"))
    }
}

fn require_cms_admin(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin") {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Admin access required"))
    }
}

// =====================================================================================
// DATABASE MODELS
// =====================================================================================

/// Schedule status enum matching database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "cms_schedule_status", rename_all = "lowercase")]
pub enum ScheduleStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Schedule action enum matching database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "cms_schedule_action", rename_all = "lowercase")]
pub enum ScheduleAction {
    Publish,
    Unpublish,
    Archive,
    Update,
}

/// Release status enum matching database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "cms_release_status", rename_all = "lowercase")]
pub enum ReleaseStatus {
    Draft,
    Scheduled,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// CMS Schedule record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsSchedule {
    pub id: Uuid,
    pub content_id: Uuid,
    pub action: ScheduleAction,
    pub scheduled_at: DateTime<Utc>,
    pub timezone: String,
    pub status: ScheduleStatus,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
    pub staged_data: Option<JsonValue>,
    pub notes: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cancelled_by: Option<i64>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

/// Extended schedule with content info
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsScheduleExtended {
    pub id: Uuid,
    pub content_id: Uuid,
    pub action: ScheduleAction,
    pub scheduled_at: DateTime<Utc>,
    pub timezone: String,
    pub status: ScheduleStatus,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    // Content info
    pub content_title: Option<String>,
    pub content_slug: Option<String>,
    pub content_type_name: Option<String>,
    pub created_by_name: Option<String>,
}

/// CMS Release record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsRelease {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub timezone: String,
    pub status: ReleaseStatus,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub total_items: i32,
    pub completed_items: i32,
    pub failed_items: i32,
    pub stop_on_error: bool,
    pub notify_on_complete: bool,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<DateTime<Utc>>,
    pub cancelled_by: Option<i64>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

/// Release item record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReleaseItem {
    pub id: Uuid,
    pub release_id: Uuid,
    pub content_id: Uuid,
    pub action: ScheduleAction,
    pub order_index: i32,
    pub staged_data: Option<JsonValue>,
    pub status: ScheduleStatus,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Extended release item with content info
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReleaseItemExtended {
    pub id: Uuid,
    pub release_id: Uuid,
    pub content_id: Uuid,
    pub action: ScheduleAction,
    pub order_index: i32,
    pub status: ScheduleStatus,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    // Content info
    pub content_title: Option<String>,
    pub content_slug: Option<String>,
    pub content_type_name: Option<String>,
}

/// Schedule history record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsScheduleHistory {
    pub id: Uuid,
    pub schedule_id: Option<Uuid>,
    pub release_id: Option<Uuid>,
    pub content_id: Option<Uuid>,
    pub event_type: String,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub event_data: Option<JsonValue>,
    pub error_details: Option<String>,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

// =====================================================================================
// REQUEST/RESPONSE MODELS
// =====================================================================================

/// Create schedule request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateScheduleRequest {
    /// Content ID to schedule
    pub content_id: Uuid,
    /// Action to perform
    pub action: ScheduleAction,
    /// When to execute (ISO 8601)
    pub scheduled_at: DateTime<Utc>,
    /// Timezone for display (e.g., "America/New_York")
    #[serde(default = "default_timezone")]
    pub timezone: String,
    /// Optional staged data for 'update' action
    pub staged_data: Option<JsonValue>,
    /// Optional notes
    pub notes: Option<String>,
}

fn default_timezone() -> String {
    "UTC".to_string()
}

/// Update schedule request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateScheduleRequest {
    /// New action (optional)
    pub action: Option<ScheduleAction>,
    /// New scheduled time (optional)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// New timezone (optional)
    pub timezone: Option<String>,
    /// Updated staged data (optional)
    pub staged_data: Option<JsonValue>,
    /// Updated notes (optional)
    pub notes: Option<String>,
}

/// Create release request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReleaseRequest {
    /// Release name
    pub name: String,
    /// Release description
    pub description: Option<String>,
    /// When to execute (optional - null for draft)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// Timezone for display
    #[serde(default = "default_timezone")]
    pub timezone: String,
    /// Stop processing on first error
    #[serde(default)]
    pub stop_on_error: bool,
    /// Send notification when complete
    #[serde(default = "default_true")]
    pub notify_on_complete: bool,
}

fn default_true() -> bool {
    true
}

/// Update release request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReleaseRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub stop_on_error: Option<bool>,
    pub notify_on_complete: Option<bool>,
}

/// Add item to release request
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddReleaseItemRequest {
    /// Content ID to add
    pub content_id: Uuid,
    /// Action to perform
    pub action: ScheduleAction,
    /// Optional staged data
    pub staged_data: Option<JsonValue>,
    /// Optional order index
    pub order_index: Option<i32>,
}

/// Calendar query parameters
#[derive(Debug, Deserialize, IntoParams)]
pub struct CalendarQuery {
    /// Start date (ISO 8601)
    pub start_date: DateTime<Utc>,
    /// End date (ISO 8601)
    pub end_date: DateTime<Utc>,
    /// Optional content type filter
    pub content_type_id: Option<Uuid>,
}

/// List query parameters
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    /// Status filter
    pub status: Option<String>,
    /// Content ID filter
    pub content_id: Option<Uuid>,
    /// Limit (default 50)
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset for pagination
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

/// Schedule list response
#[derive(Debug, Serialize, ToSchema)]
pub struct ScheduleListResponse {
    pub schedules: Vec<CmsScheduleExtended>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// Release list response
#[derive(Debug, Serialize, ToSchema)]
pub struct ReleaseListResponse {
    pub releases: Vec<CmsRelease>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// Release detail response
#[derive(Debug, Serialize, ToSchema)]
pub struct ReleaseDetailResponse {
    #[serde(flatten)]
    pub release: CmsRelease,
    pub items: Vec<CmsReleaseItemExtended>,
    pub created_by_name: Option<String>,
}

/// Calendar entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CalendarEntry {
    pub id: Uuid,
    pub content_id: Uuid,
    pub content_title: Option<String>,
    pub content_type: Option<String>,
    pub action: ScheduleAction,
    pub scheduled_at: DateTime<Utc>,
    pub timezone: String,
    pub status: ScheduleStatus,
    pub is_release: bool,
    pub release_id: Option<Uuid>,
    pub release_name: Option<String>,
}

/// History query parameters
#[derive(Debug, Deserialize, IntoParams)]
pub struct HistoryQuery {
    /// Schedule ID filter
    pub schedule_id: Option<Uuid>,
    /// Release ID filter
    pub release_id: Option<Uuid>,
    /// Content ID filter
    pub content_id: Option<Uuid>,
    /// Event type filter
    pub event_type: Option<String>,
    /// Limit
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Offset
    #[serde(default)]
    pub offset: i64,
}

/// History list response
#[derive(Debug, Serialize, ToSchema)]
pub struct HistoryListResponse {
    pub history: Vec<CmsScheduleHistory>,
    pub total: i64,
    pub has_more: bool,
}

/// Cron execution response
#[derive(Debug, Serialize, ToSchema)]
pub struct CronExecutionResponse {
    pub processed: i32,
    pub successful: i32,
    pub failed: i32,
    pub errors: Vec<String>,
}

// =====================================================================================
// SCHEDULE HANDLERS
// =====================================================================================

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
async fn create_schedule(
    State(state): State<AppState>,
    user: User,
    Json(req): Json<CreateScheduleRequest>,
) -> ApiResult<CmsSchedule> {
    require_cms_editor(&user)?;

    // Verify content exists
    let content_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1)",
    )
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
        r#"
        INSERT INTO cms_schedules (
            content_id, action, scheduled_at, timezone, staged_data, notes, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
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
        r#"
        INSERT INTO cms_schedule_history (
            schedule_id, content_id, event_type, new_status, user_id, event_data
        ) VALUES ($1, $2, 'schedule_created', 'pending', $3, $4)
        "#,
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
async fn list_schedules(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListQuery>,
) -> ApiResult<ScheduleListResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.min(100);

    // Build dynamic query
    let mut sql = String::from(
        r#"
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
        "#,
    );

    // Build parameterized query to prevent SQL injection
    let mut param_idx = 1;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref status) = query.status {
        sql.push_str(&format!(" AND s.status = ${}", param_idx));
        bind_values.push(status.clone());
        param_idx += 1;
    }
    if let Some(content_id) = query.content_id {
        sql.push_str(&format!(" AND s.content_id = ${}", param_idx));
        bind_values.push(content_id.to_string());
        param_idx += 1;
    }

    sql.push_str(&format!(" ORDER BY s.scheduled_at DESC LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

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
        count_sql.push_str(&format!(" AND s.status = ${}", count_param_idx));
        count_bind_values.push(status.clone());
        count_param_idx += 1;
    }
    if let Some(content_id) = query.content_id {
        count_sql.push_str(&format!(" AND s.content_id = ${}", count_param_idx));
        count_bind_values.push(content_id.to_string());
    }

    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for val in &count_bind_values {
        count_query = count_query.bind(val);
    }

    let total: (i64,) = count_query
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

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
async fn get_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsScheduleExtended> {
    require_cms_editor(&user)?;

    let schedule: CmsScheduleExtended = sqlx::query_as(
        r#"
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
        "#,
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
async fn update_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateScheduleRequest>,
) -> ApiResult<CmsSchedule> {
    require_cms_editor(&user)?;

    // Verify schedule exists and is pending
    let existing: Option<CmsSchedule> = sqlx::query_as(
        "SELECT * FROM cms_schedules WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let existing = existing.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Schedule not found"))?;

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
        r#"
        UPDATE cms_schedules SET
            action = COALESCE($2, action),
            scheduled_at = COALESCE($3, scheduled_at),
            timezone = COALESCE($4, timezone),
            staged_data = COALESCE($5, staged_data),
            notes = COALESCE($6, notes)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&req.action)
    .bind(&req.scheduled_at)
    .bind(&req.timezone)
    .bind(&req.staged_data)
    .bind(&req.notes)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Log to history
    let _ = sqlx::query(
        r#"
        INSERT INTO cms_schedule_history (
            schedule_id, content_id, event_type, user_id, event_data
        ) VALUES ($1, $2, 'schedule_updated', $3, $4)
        "#,
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
async fn cancel_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsSchedule> {
    require_cms_editor(&user)?;

    let schedule: CmsSchedule = sqlx::query_as(
        r#"
        UPDATE cms_schedules SET
            status = 'cancelled',
            cancelled_by = $2,
            cancelled_at = NOW()
        WHERE id = $1 AND status = 'pending'
        RETURNING *
        "#,
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
        r#"
        INSERT INTO cms_schedule_history (
            schedule_id, content_id, event_type, previous_status, new_status, user_id
        ) VALUES ($1, $2, 'schedule_cancelled', 'pending', 'cancelled', $3)
        "#,
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
async fn delete_schedule(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<JsonValue>)> {
    require_cms_admin(&user)?;

    let result = sqlx::query(
        r#"
        DELETE FROM cms_schedules
        WHERE id = $1 AND status IN ('cancelled', 'completed', 'failed')
        "#,
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

// =====================================================================================
// RELEASE HANDLERS
// =====================================================================================

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
async fn create_release(
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
        r#"
        INSERT INTO cms_releases (
            name, description, scheduled_at, timezone, status,
            stop_on_error, notify_on_complete, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.scheduled_at)
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
        r#"
        INSERT INTO cms_schedule_history (
            release_id, event_type, new_status, user_id, event_data
        ) VALUES ($1, 'release_created', $2, $3, $4)
        "#,
    )
    .bind(release.id)
    .bind(format!("{:?}", status).to_lowercase())
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
async fn list_releases(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListQuery>,
) -> ApiResult<ReleaseListResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.min(100);

    let mut sql = String::from(
        "SELECT * FROM cms_releases WHERE 1=1",
    );

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
        sqlx::query_as(&sql)
            .bind(status)
            .bind(limit)
            .bind(query.offset)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    } else {
        sqlx::query_as(&sql)
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
async fn get_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<ReleaseDetailResponse> {
    require_cms_editor(&user)?;

    let release: CmsRelease = sqlx::query_as(
        "SELECT * FROM cms_releases WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Release not found"))?;

    let items: Vec<CmsReleaseItemExtended> = sqlx::query_as(
        r#"
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
        "#,
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
async fn update_release(
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
        r#"
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
        "#,
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.scheduled_at)
    .bind(&req.timezone)
    .bind(&req.stop_on_error)
    .bind(&req.notify_on_complete)
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
async fn add_release_item(
    State(state): State<AppState>,
    user: User,
    Path(release_id): Path<Uuid>,
    Json(req): Json<AddReleaseItemRequest>,
) -> ApiResult<CmsReleaseItem> {
    require_cms_editor(&user)?;

    // Verify release exists and is draft/scheduled
    let release: Option<CmsRelease> = sqlx::query_as(
        "SELECT * FROM cms_releases WHERE id = $1",
    )
    .bind(release_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let release = release.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Release not found"))?;

    if !matches!(release.status, ReleaseStatus::Draft | ReleaseStatus::Scheduled) {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Can only add items to draft or scheduled releases",
        ));
    }

    // Verify content exists
    let content_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1)",
    )
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
        let max_order: (Option<i32>,) = sqlx::query_as(
            "SELECT MAX(order_index) FROM cms_release_items WHERE release_id = $1",
        )
        .bind(release_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((None,));
        max_order.0.unwrap_or(-1) + 1
    };

    let item: CmsReleaseItem = sqlx::query_as(
        r#"
        INSERT INTO cms_release_items (
            release_id, content_id, action, order_index, staged_data
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
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
async fn remove_release_item(
    State(state): State<AppState>,
    user: User,
    Path((release_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, Json<JsonValue>)> {
    require_cms_editor(&user)?;

    let result = sqlx::query(
        r#"
        DELETE FROM cms_release_items ri
        USING cms_releases r
        WHERE ri.id = $1
          AND ri.release_id = $2
          AND r.id = ri.release_id
          AND r.status IN ('draft', 'scheduled')
        "#,
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
async fn schedule_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(req): Json<ScheduleReleaseRequest>,
) -> ApiResult<CmsRelease> {
    require_cms_editor(&user)?;

    // Verify release has items
    let item_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_release_items WHERE release_id = $1",
    )
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
        r#"
        UPDATE cms_releases SET
            scheduled_at = $2,
            timezone = $3,
            status = 'scheduled'
        WHERE id = $1 AND status = 'draft'
        RETURNING *
        "#,
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
        r#"
        INSERT INTO cms_schedule_history (
            release_id, event_type, previous_status, new_status, user_id, event_data
        ) VALUES ($1, 'release_scheduled', 'draft', 'scheduled', $2, $3)
        "#,
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
struct ScheduleReleaseRequest {
    scheduled_at: DateTime<Utc>,
    #[serde(default = "default_timezone")]
    timezone: String,
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
async fn cancel_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsRelease> {
    require_cms_editor(&user)?;

    let release: CmsRelease = sqlx::query_as(
        r#"
        UPDATE cms_releases SET
            status = 'cancelled',
            cancelled_by = $2,
            cancelled_at = NOW()
        WHERE id = $1 AND status IN ('draft', 'scheduled')
        RETURNING *
        "#,
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
async fn delete_release(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<JsonValue>)> {
    require_cms_admin(&user)?;

    let result = sqlx::query(
        r#"
        DELETE FROM cms_releases
        WHERE id = $1 AND status IN ('cancelled', 'completed', 'failed')
        "#,
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

// =====================================================================================
// CALENDAR AND HISTORY HANDLERS
// =====================================================================================

/// Get schedule calendar view
#[utoipa::path(
    get,
    path = "/api/cms/schedules/calendar",
    params(CalendarQuery),
    responses(
        (status = 200, description = "Calendar entries", body = Vec<CalendarEntry>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
async fn get_calendar(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<CalendarQuery>,
) -> ApiResult<Vec<CalendarEntry>> {
    require_cms_editor(&user)?;

    let entries: Vec<CalendarEntry> = sqlx::query_as(
        "SELECT * FROM cms_get_schedule_calendar($1, $2, $3)",
    )
    .bind(query.start_date)
    .bind(query.end_date)
    .bind(query.content_type_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(entries))
}

/// Get schedule history/audit log
#[utoipa::path(
    get,
    path = "/api/cms/schedules/history",
    params(HistoryQuery),
    responses(
        (status = 200, description = "History entries", body = HistoryListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
async fn get_history(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<HistoryListResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.min(100);

    let mut sql = String::from(
        "SELECT * FROM cms_schedule_history WHERE 1=1",
    );

    // Build parameterized query to prevent SQL injection
    let mut param_idx = 1;
    let mut conditions: Vec<(&str, String)> = Vec::new();

    if let Some(schedule_id) = query.schedule_id {
        sql.push_str(&format!(" AND schedule_id = ${}", param_idx));
        conditions.push(("uuid", schedule_id.to_string()));
        param_idx += 1;
    }
    if let Some(release_id) = query.release_id {
        sql.push_str(&format!(" AND release_id = ${}", param_idx));
        conditions.push(("uuid", release_id.to_string()));
        param_idx += 1;
    }
    if let Some(content_id) = query.content_id {
        sql.push_str(&format!(" AND content_id = ${}", param_idx));
        conditions.push(("uuid", content_id.to_string()));
        param_idx += 1;
    }
    if let Some(ref event_type) = query.event_type {
        sql.push_str(&format!(" AND event_type = ${}", param_idx));
        conditions.push(("string", event_type.clone()));
        param_idx += 1;
    }

    sql.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

    // Build query with all bindings
    let mut query_builder = sqlx::query_as::<_, CmsScheduleHistory>(&sql);
    for (_, val) in &conditions {
        query_builder = query_builder.bind(val);
    }
    query_builder = query_builder.bind(limit).bind(query.offset);

    let history: Vec<CmsScheduleHistory> = query_builder
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get total for pagination with parameterized query
    let mut count_sql = String::from("SELECT COUNT(*) FROM cms_schedule_history WHERE 1=1");
    let mut count_param_idx = 1;

    if query.schedule_id.is_some() {
        count_sql.push_str(&format!(" AND schedule_id = ${}", count_param_idx));
        count_param_idx += 1;
    }
    if query.release_id.is_some() {
        count_sql.push_str(&format!(" AND release_id = ${}", count_param_idx));
    }

    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(schedule_id) = query.schedule_id {
        count_query = count_query.bind(schedule_id.to_string());
    }
    if let Some(release_id) = query.release_id {
        count_query = count_query.bind(release_id.to_string());
    }

    let total: (i64,) = count_query
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(HistoryListResponse {
        history,
        total: total.0,
        has_more: query.offset + limit < total.0,
    }))
}

// =====================================================================================
// CRON JOB HANDLERS (Internal use)
// =====================================================================================

/// Process pending schedules (cron job endpoint)
#[utoipa::path(
    post,
    path = "/api/cms/schedules/process",
    responses(
        (status = 200, description = "Processing complete", body = CronExecutionResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required")
    ),
    tag = "CMS Scheduling (Internal)",
    security(("bearer_auth" = []))
)]
async fn process_pending_schedules(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<CronExecutionResponse> {
    require_cms_admin(&user)?;

    let mut processed = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    // Get pending schedules that are due
    let pending: Vec<(Uuid, Uuid, ScheduleAction, Option<JsonValue>)> = sqlx::query_as(
        r#"
        SELECT id, content_id, action, staged_data
        FROM cms_schedules
        WHERE status = 'pending' AND scheduled_at <= NOW()
        ORDER BY scheduled_at ASC
        LIMIT 100
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    for (schedule_id, content_id, action, staged_data) in pending {
        processed += 1;

        // Mark as processing
        let _ = sqlx::query("UPDATE cms_schedules SET status = 'processing' WHERE id = $1")
            .bind(schedule_id)
            .execute(&state.db.pool)
            .await;

        // Execute the action
        let result = execute_schedule_action(&state, content_id, &action, staged_data.as_ref()).await;

        match result {
            Ok(_) => {
                successful += 1;
                let _ = sqlx::query(
                    "UPDATE cms_schedules SET status = 'completed', executed_at = NOW() WHERE id = $1",
                )
                .bind(schedule_id)
                .execute(&state.db.pool)
                .await;

                // Log success
                let _ = sqlx::query(
                    r#"
                    INSERT INTO cms_schedule_history (
                        schedule_id, content_id, event_type, previous_status, new_status
                    ) VALUES ($1, $2, 'schedule_executed', 'processing', 'completed')
                    "#,
                )
                .bind(schedule_id)
                .bind(content_id)
                .execute(&state.db.pool)
                .await;
            }
            Err(e) => {
                failed += 1;
                let error_msg = e.to_string();
                errors.push(format!("Schedule {}: {}", schedule_id, error_msg));

                // Check retry count
                let schedule: Option<CmsSchedule> = sqlx::query_as(
                    "SELECT * FROM cms_schedules WHERE id = $1",
                )
                .bind(schedule_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

                if let Some(s) = schedule {
                    if s.retry_count < s.max_retries {
                        let _ = sqlx::query(
                            r#"
                            UPDATE cms_schedules SET
                                status = 'pending',
                                retry_count = retry_count + 1,
                                error_message = $2
                            WHERE id = $1
                            "#,
                        )
                        .bind(schedule_id)
                        .bind(&error_msg)
                        .execute(&state.db.pool)
                        .await;
                    } else {
                        let _ = sqlx::query(
                            r#"
                            UPDATE cms_schedules SET
                                status = 'failed',
                                executed_at = NOW(),
                                error_message = $2
                            WHERE id = $1
                            "#,
                        )
                        .bind(schedule_id)
                        .bind(&error_msg)
                        .execute(&state.db.pool)
                        .await;
                    }
                }
            }
        }
    }

    Ok(Json(CronExecutionResponse {
        processed,
        successful,
        failed,
        errors,
    }))
}

/// Execute a schedule action on content
async fn execute_schedule_action(
    state: &AppState,
    content_id: Uuid,
    action: &ScheduleAction,
    staged_data: Option<&JsonValue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match action {
        ScheduleAction::Publish => {
            sqlx::query(
                "UPDATE cms_content SET status = 'published', published_at = NOW(), updated_at = NOW() WHERE id = $1",
            )
            .bind(content_id)
            .execute(&state.db.pool)
            .await?;
        }
        ScheduleAction::Unpublish => {
            sqlx::query(
                "UPDATE cms_content SET status = 'draft', updated_at = NOW() WHERE id = $1",
            )
            .bind(content_id)
            .execute(&state.db.pool)
            .await?;
        }
        ScheduleAction::Archive => {
            sqlx::query(
                "UPDATE cms_content SET status = 'archived', updated_at = NOW() WHERE id = $1",
            )
            .bind(content_id)
            .execute(&state.db.pool)
            .await?;
        }
        ScheduleAction::Update => {
            if let Some(data) = staged_data {
                // Apply staged changes
                sqlx::query(
                    r#"
                    UPDATE cms_content SET
                        title = COALESCE($2->>'title', title),
                        content = COALESCE($2->>'content', content),
                        data = COALESCE($2->'data', data),
                        custom_fields = COALESCE($2->'custom_fields', custom_fields),
                        updated_at = NOW(),
                        version = version + 1
                    WHERE id = $1
                    "#,
                )
                .bind(content_id)
                .bind(data)
                .execute(&state.db.pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Process pending releases (cron job endpoint)
#[utoipa::path(
    post,
    path = "/api/cms/releases/process",
    responses(
        (status = 200, description = "Processing complete", body = CronExecutionResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required")
    ),
    tag = "CMS Releases (Internal)",
    security(("bearer_auth" = []))
)]
async fn process_pending_releases(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<CronExecutionResponse> {
    require_cms_admin(&user)?;

    let mut processed = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    // Get pending releases
    let pending: Vec<(Uuid, bool)> = sqlx::query_as(
        r#"
        SELECT id, stop_on_error
        FROM cms_releases
        WHERE status = 'scheduled' AND scheduled_at <= NOW()
        ORDER BY scheduled_at ASC
        LIMIT 10
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    for (release_id, stop_on_error) in pending {
        processed += 1;

        // Mark as processing
        let _ = sqlx::query("UPDATE cms_releases SET status = 'processing' WHERE id = $1")
            .bind(release_id)
            .execute(&state.db.pool)
            .await;

        // Get release items
        let items: Vec<(Uuid, Uuid, ScheduleAction, Option<JsonValue>)> = sqlx::query_as(
            r#"
            SELECT id, content_id, action, staged_data
            FROM cms_release_items
            WHERE release_id = $1 AND status = 'pending'
            ORDER BY order_index ASC
            "#,
        )
        .bind(release_id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        let mut release_failed = false;

        for (item_id, content_id, action, staged_data) in items {
            let result = execute_schedule_action(&state, content_id, &action, staged_data.as_ref()).await;

            match result {
                Ok(_) => {
                    let _ = sqlx::query(
                        "UPDATE cms_release_items SET status = 'completed', executed_at = NOW() WHERE id = $1",
                    )
                    .bind(item_id)
                    .execute(&state.db.pool)
                    .await;
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = sqlx::query(
                        "UPDATE cms_release_items SET status = 'failed', executed_at = NOW(), error_message = $2 WHERE id = $1",
                    )
                    .bind(item_id)
                    .bind(&error_msg)
                    .execute(&state.db.pool)
                    .await;

                    release_failed = true;
                    errors.push(format!("Release {} item {}: {}", release_id, item_id, error_msg));

                    if stop_on_error {
                        break;
                    }
                }
            }
        }

        if release_failed {
            failed += 1;
            let _ = sqlx::query(
                "UPDATE cms_releases SET status = 'failed', executed_at = NOW() WHERE id = $1",
            )
            .bind(release_id)
            .execute(&state.db.pool)
            .await;
        } else {
            successful += 1;
            let _ = sqlx::query(
                "UPDATE cms_releases SET status = 'completed', executed_at = NOW() WHERE id = $1",
            )
            .bind(release_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(CronExecutionResponse {
        processed,
        successful,
        failed,
        errors,
    }))
}

// =====================================================================================
// ROUTER
// =====================================================================================

/// Build the CMS scheduling router
pub fn router() -> Router<AppState> {
    Router::new()
        // Schedules
        .route("/schedules", post(create_schedule))
        .route("/schedules", get(list_schedules))
        .route("/schedules/calendar", get(get_calendar))
        .route("/schedules/history", get(get_history))
        .route("/schedules/process", post(process_pending_schedules))
        .route("/schedules/:id", get(get_schedule))
        .route("/schedules/:id", put(update_schedule))
        .route("/schedules/:id", delete(delete_schedule))
        .route("/schedules/:id/cancel", post(cancel_schedule))
        // Releases
        .route("/releases", post(create_release))
        .route("/releases", get(list_releases))
        .route("/releases/process", post(process_pending_releases))
        .route("/releases/:id", get(get_release))
        .route("/releases/:id", put(update_release))
        .route("/releases/:id", delete(delete_release))
        .route("/releases/:id/items", post(add_release_item))
        .route("/releases/:release_id/items/:item_id", delete(remove_release_item))
        .route("/releases/:id/schedule", post(schedule_release))
        .route("/releases/:id/cancel", post(cancel_release))
}

// =====================================================================================
// OPENAPI SCHEMA
// =====================================================================================

/// OpenAPI schema for CMS Scheduling API
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        create_schedule,
        list_schedules,
        get_schedule,
        update_schedule,
        cancel_schedule,
        delete_schedule,
        get_calendar,
        get_history,
        process_pending_schedules,
        create_release,
        list_releases,
        get_release,
        update_release,
        add_release_item,
        remove_release_item,
        schedule_release,
        cancel_release,
        delete_release,
        process_pending_releases
    ),
    components(
        schemas(
            CmsSchedule,
            CmsScheduleExtended,
            CmsRelease,
            CmsReleaseItem,
            CmsReleaseItemExtended,
            CmsScheduleHistory,
            ScheduleStatus,
            ScheduleAction,
            ReleaseStatus,
            CreateScheduleRequest,
            UpdateScheduleRequest,
            CreateReleaseRequest,
            UpdateReleaseRequest,
            AddReleaseItemRequest,
            ScheduleListResponse,
            ReleaseListResponse,
            ReleaseDetailResponse,
            CalendarEntry,
            HistoryListResponse,
            CronExecutionResponse
        )
    ),
    tags(
        (name = "CMS Scheduling", description = "Content scheduling API"),
        (name = "CMS Releases", description = "Release bundle management API")
    )
)]
pub struct CmsSchedulingApi;
