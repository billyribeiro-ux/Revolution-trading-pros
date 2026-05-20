//! Shared types for the CMS Global Components Library API.
//!
//! Enums, database models, and request/response DTOs used across the
//! `cms_global_components` sub-modules.

use axum::{http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::utils::errors::ApiError;

// ============================================================================
// TYPE ALIASES
// ============================================================================

pub(super) type ApiResult<T> = Result<Json<T>, ApiError>;
pub(super) type ApiResultEmpty = Result<Json<JsonValue>, ApiError>;

// ============================================================================
// DATABASE MODELS
// ============================================================================

/// Global component category enum
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cms_global_component_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GlobalComponentCategory {
    #[default]
    Header,
    Footer,
    Cta,
    Form,
    Navigation,
    Sidebar,
    Banner,
    Modal,
    Card,
    Section,
}

impl std::fmt::Display for GlobalComponentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalComponentCategory::Header => write!(f, "header"),
            GlobalComponentCategory::Footer => write!(f, "footer"),
            GlobalComponentCategory::Cta => write!(f, "cta"),
            GlobalComponentCategory::Form => write!(f, "form"),
            GlobalComponentCategory::Navigation => write!(f, "navigation"),
            GlobalComponentCategory::Sidebar => write!(f, "sidebar"),
            GlobalComponentCategory::Banner => write!(f, "banner"),
            GlobalComponentCategory::Modal => write!(f, "modal"),
            GlobalComponentCategory::Card => write!(f, "card"),
            GlobalComponentCategory::Section => write!(f, "section"),
        }
    }
}

/// Global component stored in the component library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponent {
    /// Unique identifier
    pub id: Uuid,
    /// Display name of the component
    pub name: String,
    /// URL-friendly unique identifier
    pub slug: String,
    /// Optional description for the component library
    pub description: Option<String>,
    /// Component content and configuration (JSON array of blocks)
    pub component_data: JsonValue,
    /// Category for organization
    pub category: GlobalComponentCategory,
    /// Tags for filtering and search
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Number of times this component is used across content
    pub usage_count: i32,
    /// Whether the component is available globally to all users
    pub is_global: bool,
    /// Whether the component is locked from editing
    pub is_locked: bool,
    /// Current version number
    pub version: i32,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// UUID of the CMS user who created the component
    pub created_by: Option<Uuid>,
    /// UUID of the CMS user who last updated the component
    pub updated_by: Option<Uuid>,
}

/// Lightweight component summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponentSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub usage_count: i32,
    pub is_global: bool,
    pub is_locked: bool,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Version history record for a global component
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponentVersion {
    /// Unique identifier for this version
    pub id: Uuid,
    /// Reference to the global component
    pub component_id: Uuid,
    /// Version number
    pub version: i32,
    /// Component data at this version
    pub component_data: JsonValue,
    /// Change description/commit message
    pub change_message: Option<String>,
    /// Who made this change
    pub created_by: Option<Uuid>,
    /// Creator's display name
    pub created_by_name: Option<String>,
    /// When this version was created
    pub created_at: DateTime<Utc>,
}

/// Tracks where a global component is used
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponentUsage {
    /// Unique identifier for this usage record
    pub id: Uuid,
    /// Reference to the global component
    pub component_id: Uuid,
    /// Content item where the component is used
    pub content_id: Uuid,
    /// Instance ID of the component within the content
    pub instance_id: String,
    /// Whether this instance syncs with the source component
    pub is_synced: bool,
    /// Version at which this instance was last synced
    pub synced_version: i32,
    /// When the component was detached from sync
    pub detached_at: Option<DateTime<Utc>>,
    /// Who detached the component
    pub detached_by: Option<Uuid>,
    /// When this usage was created
    pub created_at: DateTime<Utc>,
}

/// Usage record with content metadata for display
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GlobalComponentUsageWithContent {
    pub id: Uuid,
    pub component_id: Uuid,
    pub content_id: Uuid,
    pub content_title: String,
    pub content_type: String,
    pub content_slug: String,
    pub instance_id: String,
    pub is_synced: bool,
    pub synced_version: i32,
    pub needs_update: bool,
    pub detached_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new global component
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateGlobalComponentRequest {
    /// Display name (required)
    pub name: String,
    /// Custom slug (optional, auto-generated if not provided)
    pub slug: Option<String>,
    /// Component description
    pub description: Option<String>,
    /// Component content and configuration (array of blocks)
    pub component_data: JsonValue,
    /// Category for organization
    pub category: GlobalComponentCategory,
    /// Tags for filtering
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Make globally available
    #[serde(default)]
    pub is_global: bool,
}

/// Request to update a global component
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGlobalComponentRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub component_data: Option<JsonValue>,
    pub category: Option<GlobalComponentCategory>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub is_global: Option<bool>,
    pub is_locked: Option<bool>,
    /// Optional change message for version history
    pub change_message: Option<String>,
}

/// Request to track component usage
#[derive(Debug, Deserialize, ToSchema)]
pub struct TrackComponentUsageRequest {
    /// The global component being used
    pub component_id: Uuid,
    /// Content item where the component is placed
    pub content_id: Uuid,
    /// Unique instance ID within the content
    pub instance_id: String,
    /// Whether to sync updates from source
    #[serde(default = "default_true")]
    pub is_synced: bool,
}

pub(super) fn default_true() -> bool {
    true
}

/// Request to sync a component instance to latest version
#[derive(Debug, Deserialize, ToSchema)]
pub struct SyncComponentRequest {
    /// Whether to force sync even if there are local changes
    #[serde(default)]
    pub force: bool,
}

/// Query parameters for listing components
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListGlobalComponentsQuery {
    /// Filter by category
    pub category: Option<String>,
    /// Search in name, description, tags
    pub search: Option<String>,
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
    /// Include only global components
    pub is_global: Option<bool>,
    /// Sort field (name, created_at, updated_at, usage_count)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit (default 20, max 100)
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
    /// Include deleted components
    pub include_deleted: Option<bool>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedComponentsResponse {
    pub data: Vec<GlobalComponentSummary>,
    pub meta: PaginationMeta,
    pub categories: Vec<CategoryCount>,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// Category count for sidebar filters
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

/// Response for component usage listing
#[derive(Debug, Serialize, ToSchema)]
pub struct ComponentUsageResponse {
    pub usages: Vec<GlobalComponentUsageWithContent>,
    pub total_count: i64,
    pub synced_count: i64,
    pub outdated_count: i64,
}

/// Response for version history
#[derive(Debug, Serialize, ToSchema)]
pub struct VersionHistoryResponse {
    pub versions: Vec<GlobalComponentVersion>,
    pub total_count: i64,
    pub current_version: i32,
}

/// Response for component with full details
#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalComponentWithMeta {
    #[serde(flatten)]
    pub component: GlobalComponent,
    pub created_by_name: Option<String>,
    pub updated_by_name: Option<String>,
    pub recent_versions: Vec<GlobalComponentVersion>,
}

// ============================================================================
// COMMON ERROR HELPERS
// ============================================================================

/// Build a 500 INTERNAL_SERVER_ERROR from a sqlx error.
#[inline]
pub(super) fn sqlx_err(e: sqlx::Error) -> ApiError {
    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

/// Build a NOT_FOUND ApiError for the component-not-found case.
#[inline]
pub(super) fn not_found_component() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
}
