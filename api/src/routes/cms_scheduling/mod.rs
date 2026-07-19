//! CMS Scheduling Routes - Apple ICT 7+ Principal Engineer Grade
//!
//! Content scheduling and releases system:
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
//!
//! Module layout (split from the original 2,003-LOC `cms_scheduling.rs`):
//! - `schedules`        — single-content schedule CRUD (6 handlers)
//! - `releases`         — release bundle CRUD + item ops (9 handlers)
//! - `calendar_history` — calendar view + audit log (2 handlers)
//! - `cron`             — internal cron endpoints + execution helper (2 handlers)

use axum::{
    extract::Json,
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{models::User, AppState};

mod calendar_history;
mod cron;
mod releases;
mod schedules;

// =====================================================================================
// TYPE ALIASES AND ERROR HANDLING (shared across sub-modules)
// =====================================================================================

pub(super) type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;

pub(super) fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (
        status,
        Json(json!({
            "error": message,
            "status": status.as_u16(),
            "timestamp": Utc::now().to_rfc3339()
        })),
    )
}

pub(super) fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
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

pub(super) fn require_cms_admin(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
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

pub(super) fn default_timezone() -> String {
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

pub(super) fn default_true() -> bool {
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

pub(super) fn default_limit() -> i64 {
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
// ROUTER
// =====================================================================================

/// Build the CMS scheduling router
pub fn router() -> Router<AppState> {
    Router::new()
        // Schedules
        .route("/schedules", post(schedules::create_schedule))
        .route("/schedules", get(schedules::list_schedules))
        .route("/schedules/calendar", get(calendar_history::get_calendar))
        .route("/schedules/history", get(calendar_history::get_history))
        .route("/schedules/process", post(cron::process_pending_schedules))
        .route("/schedules/{id}", get(schedules::get_schedule))
        .route("/schedules/{id}", put(schedules::update_schedule))
        .route("/schedules/{id}", delete(schedules::delete_schedule))
        .route("/schedules/{id}/cancel", post(schedules::cancel_schedule))
        // Releases
        .route("/releases", post(releases::create_release))
        .route("/releases", get(releases::list_releases))
        .route("/releases/process", post(cron::process_pending_releases))
        .route("/releases/{id}", get(releases::get_release))
        .route("/releases/{id}", put(releases::update_release))
        .route("/releases/{id}", delete(releases::delete_release))
        .route("/releases/{id}/items", post(releases::add_release_item))
        .route(
            "/releases/{release_id}/items/{item_id}",
            delete(releases::remove_release_item),
        )
        .route("/releases/{id}/schedule", post(releases::schedule_release))
        .route("/releases/{id}/cancel", post(releases::cancel_release))
}

// =====================================================================================
// OPENAPI SCHEMA
// =====================================================================================

/// OpenAPI schema for CMS Scheduling API
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        schedules::create_schedule,
        schedules::list_schedules,
        schedules::get_schedule,
        schedules::update_schedule,
        schedules::cancel_schedule,
        schedules::delete_schedule,
        calendar_history::get_calendar,
        calendar_history::get_history,
        cron::process_pending_schedules,
        releases::create_release,
        releases::list_releases,
        releases::get_release,
        releases::update_release,
        releases::add_release_item,
        releases::remove_release_item,
        releases::schedule_release,
        releases::cancel_release,
        releases::delete_release,
        cron::process_pending_releases
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
