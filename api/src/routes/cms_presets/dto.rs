//! Data transfer objects + database models for the CMS presets API.
//!
//! Split from the original `cms_presets.rs` (R25-B). Pure structural
//! move — every type retains its public path via `pub use dto::*;`
//! in the parent module so external callers (notably the integration
//! test `tests/cms_presets_test.rs`) keep compiling unchanged.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// ============================================================================
// DATABASE MODELS
// ============================================================================

/// Preset category enum - maps directly to PostgreSQL cms_preset_category
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cms_preset_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CmsPresetCategory {
    #[default]
    Default,
    Brand,
    Seasonal,
    Marketing,
    Trading,
    Custom,
}

/// Component preset stored in the preset library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsPreset {
    /// Unique identifier
    pub id: Uuid,
    /// Block type this preset applies to (e.g., button, heading)
    pub block_type: String,
    /// Display name of the preset
    pub name: String,
    /// URL-friendly unique identifier
    pub slug: String,
    /// Optional description
    pub description: Option<String>,
    /// Preset content and settings (JSON)
    pub preset_data: JsonValue,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Thumbnail blurhash for loading
    pub thumbnail_blurhash: Option<String>,
    /// Category for organization
    pub category: CmsPresetCategory,
    /// Tags for filtering and search
    pub tags: Option<Vec<String>>,
    /// Whether this is the default preset for the block type
    pub is_default: bool,
    /// Whether the preset is locked from editing
    pub is_locked: bool,
    /// Whether available globally to all users
    pub is_global: bool,
    /// Number of times this preset has been used
    pub usage_count: i32,
    /// Version number for tracking changes
    pub version: i32,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// UUID of the CMS user who created the preset
    pub created_by: Option<Uuid>,
    /// UUID of the CMS user who last updated the preset
    pub updated_by: Option<Uuid>,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Lightweight preset summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsPresetSummary {
    pub id: Uuid,
    pub block_type: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_blurhash: Option<String>,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub is_default: bool,
    pub is_locked: bool,
    pub is_global: bool,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Block type with preset count
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlockTypeWithPresets {
    pub block_type: String,
    pub preset_count: i64,
    pub has_default: bool,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new preset
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePresetRequest {
    /// Block type this preset applies to (required)
    pub block_type: String,
    /// Display name (required)
    pub name: String,
    /// Custom slug (optional, auto-generated if not provided)
    pub slug: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Preset content and settings
    pub preset_data: JsonValue,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Thumbnail blurhash
    pub thumbnail_blurhash: Option<String>,
    /// Category for organization
    pub category: Option<CmsPresetCategory>,
    /// Tags for filtering
    pub tags: Option<Vec<String>>,
    /// Mark as default for this block type
    #[serde(default)]
    pub is_default: bool,
    /// Make globally available
    #[serde(default = "default_true")]
    pub is_global: bool,
}

/// Request to update a preset
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePresetRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub preset_data: Option<JsonValue>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_blurhash: Option<String>,
    pub category: Option<CmsPresetCategory>,
    pub tags: Option<Vec<String>>,
    pub is_default: Option<bool>,
    pub is_global: Option<bool>,
    pub is_locked: Option<bool>,
}

pub(super) fn default_true() -> bool {
    true
}

/// Query parameters for listing presets
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListPresetsQuery {
    /// Filter by block type
    pub block_type: Option<String>,
    /// Filter by category
    pub category: Option<String>,
    /// Search in name, description, tags
    pub search: Option<String>,
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
    /// Include only global presets
    pub is_global: Option<bool>,
    /// Include only default presets
    pub is_default: Option<bool>,
    /// Sort field (name, created_at, updated_at, usage_count)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit (default 50, max 200)
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedPresetsResponse {
    pub data: Vec<CmsPresetSummary>,
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

/// Response for block types with presets
#[derive(Debug, Serialize, ToSchema)]
pub struct BlockTypesWithPresetsResponse {
    pub data: Vec<BlockTypeWithPresets>,
}

/// Response for presets grouped by category
#[derive(Debug, Serialize, ToSchema)]
pub struct PresetsByCategory {
    pub category: String,
    pub presets: Vec<CmsPresetSummary>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GroupedPresetsResponse {
    pub block_type: String,
    pub categories: Vec<PresetsByCategory>,
    pub total_count: i64,
}

/// Save current block as a new preset
#[derive(Debug, Deserialize, ToSchema)]
pub struct SaveBlockAsPresetRequest {
    /// Block type
    pub block_type: String,
    /// Preset name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Block content
    pub content: JsonValue,
    /// Block settings
    pub settings: JsonValue,
    /// Category
    pub category: Option<CmsPresetCategory>,
    /// Tags
    pub tags: Option<Vec<String>>,
    /// Thumbnail URL (optional)
    pub thumbnail_url: Option<String>,
}
