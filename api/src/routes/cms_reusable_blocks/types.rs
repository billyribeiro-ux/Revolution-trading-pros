//! Shared DTOs, enums, and request/response models for the
//! `cms_reusable_blocks` routes.
//!
//! All types here are `pub` (not `pub(super)`) because they're re-exported
//! by `mod.rs` so external consumers (the typed-client crate, utoipa schema
//! registration, and the public contract test in
//! `tests/cms_reusable_blocks_test.rs`) can reach them at
//! `revolution_api::routes::cms_reusable_blocks::*`.

use axum::Json;
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

/// Block category enum - maps directly to PostgreSQL cms_reusable_block_category
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cms_reusable_block_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CmsReusableBlockCategory {
    #[default]
    General,
    Trading,
    Layout,
    Callout,
    Marketing,
    Navigation,
    Media,
    Form,
}

/// Reusable content block stored in the block library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReusableBlock {
    /// Unique identifier
    pub id: Uuid,
    /// Display name of the block
    pub name: String,
    /// URL-friendly unique identifier
    pub slug: String,
    /// Optional description for the block library
    pub description: Option<String>,
    /// Block content and configuration (JSON)
    pub block_data: JsonValue,
    /// Category for organization
    pub category: Option<CmsReusableBlockCategory>,
    /// Tags for filtering and search
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Number of times this block is used across content
    pub usage_count: i32,
    /// Whether the block is available globally to all users
    pub is_global: bool,
    /// Whether the block is locked from editing
    pub is_locked: bool,
    /// Version number for tracking changes
    pub version: i32,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// UUID of the CMS user who created the block
    pub created_by: Option<Uuid>,
}

/// Lightweight block summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReusableBlockSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub usage_count: i32,
    pub is_global: bool,
    pub is_locked: bool,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tracks where a reusable block is used
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReusableBlockUsage {
    /// Unique identifier for this usage record
    pub id: Uuid,
    /// Reference to the reusable block
    pub reusable_block_id: Uuid,
    /// Content item where the block is used
    pub content_id: Uuid,
    /// Instance ID of the block within the content
    pub block_instance_id: String,
    /// Whether this instance syncs with the source block
    pub is_synced: bool,
    /// When the block was detached from sync
    pub detached_at: Option<DateTime<Utc>>,
    /// Who detached the block
    pub detached_by: Option<Uuid>,
    /// When this usage was created
    pub created_at: DateTime<Utc>,
}

/// Usage record with content metadata for display
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CmsReusableBlockUsageWithContent {
    pub id: Uuid,
    pub reusable_block_id: Uuid,
    pub content_id: Uuid,
    pub content_title: String,
    pub content_type: String,
    pub content_slug: String,
    pub block_instance_id: String,
    pub is_synced: bool,
    pub detached_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new reusable block
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReusableBlockRequest {
    /// Display name (required)
    pub name: String,
    /// Custom slug (optional, auto-generated if not provided)
    pub slug: Option<String>,
    /// Block description
    pub description: Option<String>,
    /// Block content and configuration
    pub block_data: JsonValue,
    /// Category for organization
    pub category: Option<CmsReusableBlockCategory>,
    /// Tags for filtering
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Make globally available
    #[serde(default)]
    pub is_global: bool,
}

/// Request to update a reusable block
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReusableBlockRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub block_data: Option<JsonValue>,
    pub category: Option<CmsReusableBlockCategory>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub is_global: Option<bool>,
    pub is_locked: Option<bool>,
}

/// Request to track block usage
#[derive(Debug, Deserialize, ToSchema)]
pub struct TrackBlockUsageRequest {
    /// The reusable block being used
    pub reusable_block_id: Uuid,
    /// Content item where the block is placed
    pub content_id: Uuid,
    /// Unique instance ID within the content
    pub block_instance_id: String,
    /// Whether to sync updates from source
    #[serde(default = "default_true")]
    pub is_synced: bool,
}

pub(super) fn default_true() -> bool {
    true
}

/// Query parameters for listing blocks
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListReusableBlocksQuery {
    /// Filter by category
    pub category: Option<String>,
    /// Search in name, description, tags
    pub search: Option<String>,
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
    /// Include only global blocks
    pub is_global: Option<bool>,
    /// Sort field (name, created_at, updated_at, usage_count)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit (default 20, max 100)
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
    /// Include deleted blocks
    pub include_deleted: Option<bool>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedBlocksResponse {
    pub data: Vec<CmsReusableBlockSummary>,
    pub meta: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// Response for block usage listing
#[derive(Debug, Serialize, ToSchema)]
pub struct BlockUsageResponse {
    pub usages: Vec<CmsReusableBlockUsageWithContent>,
    pub total_count: i64,
}
