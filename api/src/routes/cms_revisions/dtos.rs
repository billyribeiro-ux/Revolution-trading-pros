//! Public DTOs + query/request/response shapes for the CMS revisions
//! sub-module.
//!
//! R27-B4 split (2026-05-20) — extracted verbatim from the original
//! `routes/cms_revisions.rs` (lines 71-323). No behavioural change.
//!
//! These are re-exported by `cms_revisions/mod.rs` so the existing
//! `tests/cms_revisions_test.rs` import paths
//! (`revolution_api::routes::cms_revisions::{...}`) keep working.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════════
// MODELS - Extended Revision with Full Metadata
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Extended CMS revision with additional metadata from the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsRevisionExtended {
    /// Unique revision identifier
    pub id: Uuid,
    /// Content ID this revision belongs to
    pub content_id: Uuid,
    /// Sequential revision number (1, 2, 3, ...)
    pub revision_number: i32,
    /// Whether this is the current active revision
    pub is_current: bool,
    /// Complete snapshot of content data at this revision
    pub data: JsonValue,
    /// Human-readable summary of changes
    pub change_summary: Option<String>,
    /// List of field names that were changed
    pub changed_fields: Option<Vec<String>>,
    /// When this revision was created
    pub created_at: DateTime<Utc>,
    /// CMS user ID who created this revision
    pub created_by: Option<Uuid>,
    /// Type of change: create, update, restore, publish, etc.
    #[sqlx(default)]
    pub change_type: Option<String>,
    /// Word count of content at this revision
    #[sqlx(default)]
    pub word_count: Option<i32>,
    /// JSON stats about the diff (blocks added/removed/modified)
    #[sqlx(default)]
    pub diff_stats: Option<JsonValue>,
}

/// Extended revision with creator name for diff queries
#[derive(Debug, Clone, FromRow)]
pub(super) struct CmsRevisionWithCreator {
    pub id: Uuid,
    pub content_id: Uuid,
    pub revision_number: i32,
    pub is_current: bool,
    pub data: JsonValue,
    pub change_summary: Option<String>,
    pub changed_fields: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    #[sqlx(default)]
    pub change_type: Option<String>,
    #[sqlx(default)]
    pub word_count: Option<i32>,
    #[sqlx(default)]
    pub diff_stats: Option<JsonValue>,
    pub created_by_name: Option<String>,
}

/// Summary view of a revision for listings
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RevisionSummary {
    /// Unique revision identifier
    pub id: Uuid,
    /// Sequential revision number
    pub revision_number: i32,
    /// Whether this is the current active revision
    pub is_current: bool,
    /// Human-readable summary of changes
    pub change_summary: Option<String>,
    /// List of fields that were changed
    pub changed_fields: Option<Vec<String>>,
    /// When this revision was created
    pub created_at: DateTime<Utc>,
    /// User who created this revision
    pub created_by: Option<Uuid>,
    /// Display name of the user who created this revision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_name: Option<String>,
    /// Type of change
    pub change_type: Option<String>,
    /// Word count at this revision
    pub word_count: Option<i32>,
}

/// Query parameters for listing revisions
#[derive(Debug, Deserialize, IntoParams)]
pub struct RevisionListQuery {
    /// Maximum number of revisions to return (default: 25, max: 100)
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Number of revisions to skip for pagination
    #[serde(default)]
    pub offset: i64,
    /// Filter by change type (create, update, restore, publish)
    pub change_type: Option<String>,
    /// Include full data in response (default: false for performance)
    #[serde(default)]
    pub include_data: bool,
}

fn default_limit() -> i64 {
    25
}

/// Response for listing revisions
#[derive(Debug, Serialize, ToSchema)]
pub struct RevisionListResponse {
    /// List of revisions
    pub revisions: Vec<RevisionSummary>,
    /// Total number of revisions for this content
    pub total: i64,
    /// Current offset
    pub offset: i64,
    /// Current limit
    pub limit: i64,
    /// Whether there are more revisions
    pub has_more: bool,
    /// Current (latest) revision number
    pub current_revision: i32,
}

/// Query parameters for comparing revisions
#[derive(Debug, Deserialize, IntoParams)]
pub struct CompareQuery {
    /// First version to compare (older)
    pub v1: i32,
    /// Second version to compare (newer)
    pub v2: i32,
    /// Include full block-level diff (default: true)
    #[serde(default = "default_true")]
    pub include_blocks: bool,
}

fn default_true() -> bool {
    true
}

/// Individual field change in the diff
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FieldChange {
    /// Name of the field that changed
    pub field: String,
    /// Previous value (null if field was added)
    pub from: Option<JsonValue>,
    /// New value (null if field was removed)
    pub to: Option<JsonValue>,
    /// Type of change: added, removed, modified
    pub change_type: String,
}

/// Block-level change for content blocks
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockChange {
    /// Block ID
    pub block_id: String,
    /// Block type (e.g., "rich-text", "image", "hero-slider")
    pub block_type: String,
    /// Type of change: added, removed, modified, reordered
    pub change_type: String,
    /// Position in old revision (null if added)
    pub old_position: Option<i32>,
    /// Position in new revision (null if removed)
    pub new_position: Option<i32>,
    /// Specific changes within the block (for modified blocks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<FieldChange>>,
}

/// Comprehensive diff statistics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DiffStats {
    /// Whether the title changed
    pub title_changed: bool,
    /// Whether the main content changed
    pub content_changed: bool,
    /// Whether any SEO fields changed
    pub seo_changed: bool,
    /// Whether custom fields changed
    pub custom_fields_changed: bool,
    /// Number of content blocks added
    pub blocks_added: i32,
    /// Number of content blocks removed
    pub blocks_removed: i32,
    /// Number of content blocks modified
    pub blocks_modified: i32,
    /// Number of content blocks reordered
    pub blocks_reordered: i32,
    /// Total number of fields changed
    pub total_fields_changed: i32,
    /// Word count difference (positive = added, negative = removed)
    pub word_count_diff: i32,
    /// List of all field-level changes
    pub field_changes: Vec<FieldChange>,
    /// List of all block-level changes (if include_blocks is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_changes: Option<Vec<BlockChange>>,
}

/// Revision metadata for comparison
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RevisionMetadata {
    /// Revision number
    pub revision_number: i32,
    /// When the revision was created
    pub created_at: DateTime<Utc>,
    /// CMS user ID who created it
    pub created_by: Option<Uuid>,
    /// Display name of the creator
    pub created_by_name: Option<String>,
    /// Change summary
    pub change_summary: Option<String>,
    /// Change type
    pub change_type: Option<String>,
    /// Word count at this revision
    pub word_count: Option<i32>,
}

/// Complete revision comparison response
#[derive(Debug, Serialize, ToSchema)]
pub struct RevisionCompareResponse {
    /// Content ID being compared
    pub content_id: Uuid,
    /// Version number of the "from" revision
    pub version_from: i32,
    /// Version number of the "to" revision
    pub version_to: i32,
    /// Complete data from the older revision
    pub from_revision: JsonValue,
    /// Complete data from the newer revision
    pub to_revision: JsonValue,
    /// Metadata about the "from" revision
    pub from_metadata: RevisionMetadata,
    /// Metadata about the "to" revision
    pub to_metadata: RevisionMetadata,
    /// Comprehensive diff analysis
    pub diff: DiffStats,
}

/// Response after restoring a revision
#[derive(Debug, Serialize, ToSchema)]
pub struct RestoreResponse {
    /// Success message
    pub message: String,
    /// The restored revision number that was used as source
    pub restored_from_revision: i32,
    /// The new revision number created for this restore
    pub new_revision_number: i32,
    /// New version number of the content
    pub new_content_version: i32,
    /// The updated content
    pub content: JsonValue,
    /// Timestamp of the restore operation
    pub restored_at: DateTime<Utc>,
}
