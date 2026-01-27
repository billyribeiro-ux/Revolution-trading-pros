//! CMS Revisions Routes - Apple ICT 7+ Principal Engineer Grade
//!
//! Comprehensive revision management and comparison API:
//! - List all revisions for content
//! - Get specific revision by version
//! - Compare two revisions with detailed diff
//! - Restore content from revision
//!
//! Features:
//! - Field-by-field comparison with change detection
//! - Block-level diff analysis (added, removed, modified)
//! - Complete audit trail with metadata
//! - Atomic restore operations with version increment
//!
//! @version 2.0.0 - January 2026

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{models::User, services::cms_content, AppState};

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPE ALIASES AND ERROR HANDLING
// ═══════════════════════════════════════════════════════════════════════════════════════

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;
type ApiResultEmpty = Result<Json<JsonValue>, (StatusCode, Json<JsonValue>)>;

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
struct CmsRevisionWithCreator {
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

// ═══════════════════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List all revisions for content
///
/// Returns a paginated list of revisions for the specified content item,
/// ordered by revision number (newest first). Includes metadata about each
/// revision such as who made the change and what fields were modified.
#[utoipa::path(
    get,
    path = "/api/cms/content/{id}/revisions",
    params(
        ("id" = Uuid, Path, description = "Content ID"),
        RevisionListQuery
    ),
    responses(
        (status = 200, description = "List of revisions", body = RevisionListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Editor access required"),
        (status = 404, description = "Content not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "CMS Revisions",
    security(("bearer_auth" = []))
)]
async fn list_revisions(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Query(query): Query<RevisionListQuery>,
) -> ApiResult<RevisionListResponse> {
    require_cms_editor(&user)?;

    // Validate content exists
    let content = cms_content::get_content(&state.db.pool, content_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Content not found"))?;

    let limit = query.limit.min(100);
    let offset = query.offset;

    // Get revisions with creator names
    let revisions: Vec<(CmsRevisionExtended, Option<String>)> = sqlx::query_as(
        r#"
        SELECT
            r.id, r.content_id, r.revision_number, r.is_current, r.data,
            r.change_summary, r.changed_fields, r.created_at, r.created_by,
            r.change_type, r.word_count, r.diff_stats,
            u.display_name as created_by_name
        FROM cms_revisions r
        LEFT JOIN cms_users u ON r.created_by = u.id
        WHERE r.content_id = $1
        ORDER BY r.revision_number DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(content_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get total count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cms_revisions WHERE content_id = $1")
        .bind(content_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get current revision number
    let current_revision: (i32,) = sqlx::query_as(
        "SELECT COALESCE(MAX(revision_number), 0) FROM cms_revisions WHERE content_id = $1 AND is_current = true",
    )
    .bind(content_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((content.version,));

    let summaries: Vec<RevisionSummary> = revisions
        .into_iter()
        .map(|(r, name)| RevisionSummary {
            id: r.id,
            revision_number: r.revision_number,
            is_current: r.is_current,
            change_summary: r.change_summary,
            changed_fields: r.changed_fields,
            created_at: r.created_at,
            created_by: r.created_by,
            created_by_name: name,
            change_type: r.change_type,
            word_count: r.word_count,
        })
        .collect();

    Ok(Json(RevisionListResponse {
        revisions: summaries,
        total: total.0,
        offset,
        limit,
        has_more: offset + limit < total.0,
        current_revision: current_revision.0,
    }))
}

/// Get a specific revision by version number
///
/// Returns the complete revision data for a specific version number.
/// Includes full content snapshot and metadata.
#[utoipa::path(
    get,
    path = "/api/cms/content/{id}/revisions/{version}",
    params(
        ("id" = Uuid, Path, description = "Content ID"),
        ("version" = i32, Path, description = "Revision number")
    ),
    responses(
        (status = 200, description = "Revision details", body = CmsRevisionExtended),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Editor access required"),
        (status = 404, description = "Revision not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "CMS Revisions",
    security(("bearer_auth" = []))
)]
async fn get_revision(
    State(state): State<AppState>,
    user: User,
    Path((content_id, version)): Path<(Uuid, i32)>,
) -> ApiResult<CmsRevisionExtended> {
    require_cms_editor(&user)?;

    let revision: CmsRevisionExtended = sqlx::query_as(
        r#"
        SELECT
            id, content_id, revision_number, is_current, data,
            change_summary, changed_fields, created_at, created_by,
            change_type, word_count, diff_stats
        FROM cms_revisions
        WHERE content_id = $1 AND revision_number = $2
        "#,
    )
    .bind(content_id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::NOT_FOUND,
            &format!("Revision {} not found for content", version),
        )
    })?;

    Ok(Json(revision))
}

/// Compare two revisions
///
/// Performs a detailed comparison between two revisions, returning:
/// - Both revision's complete data
/// - Field-by-field changes
/// - Block-level changes (added, removed, modified, reordered)
/// - Word count differences
/// - Comprehensive diff statistics
#[utoipa::path(
    get,
    path = "/api/cms/content/{id}/revisions/compare",
    params(
        ("id" = Uuid, Path, description = "Content ID"),
        CompareQuery
    ),
    responses(
        (status = 200, description = "Revision comparison", body = RevisionCompareResponse),
        (status = 400, description = "Invalid version numbers"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Editor access required"),
        (status = 404, description = "One or both revisions not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "CMS Revisions",
    security(("bearer_auth" = []))
)]
async fn compare_revisions(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Query(query): Query<CompareQuery>,
) -> ApiResult<RevisionCompareResponse> {
    require_cms_editor(&user)?;

    // Validate version numbers
    if query.v1 <= 0 || query.v2 <= 0 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Version numbers must be positive",
        ));
    }

    if query.v1 == query.v2 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Cannot compare a revision with itself",
        ));
    }

    // Ensure v1 < v2 (older to newer)
    let (v1, v2) = if query.v1 < query.v2 {
        (query.v1, query.v2)
    } else {
        (query.v2, query.v1)
    };

    // Fetch both revisions with creator info
    let from_revision: Option<CmsRevisionWithCreator> = sqlx::query_as(
        r#"
        SELECT
            r.id, r.content_id, r.revision_number, r.is_current, r.data,
            r.change_summary, r.changed_fields, r.created_at, r.created_by,
            r.change_type, r.word_count, r.diff_stats,
            u.display_name as created_by_name
        FROM cms_revisions r
        LEFT JOIN cms_users u ON r.created_by = u.id
        WHERE r.content_id = $1 AND r.revision_number = $2
        "#,
    )
    .bind(content_id)
    .bind(v1)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let to_revision: Option<CmsRevisionWithCreator> = sqlx::query_as(
        r#"
        SELECT
            r.id, r.content_id, r.revision_number, r.is_current, r.data,
            r.change_summary, r.changed_fields, r.created_at, r.created_by,
            r.change_type, r.word_count, r.diff_stats,
            u.display_name as created_by_name
        FROM cms_revisions r
        LEFT JOIN cms_users u ON r.created_by = u.id
        WHERE r.content_id = $1 AND r.revision_number = $2
        "#,
    )
    .bind(content_id)
    .bind(v2)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let from_rev = from_revision
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, &format!("Revision {} not found", v1)))?;

    let to_rev = to_revision
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, &format!("Revision {} not found", v2)))?;

    let from_name = from_rev.created_by_name.clone();
    let to_name = to_rev.created_by_name.clone();

    // Compute the diff
    let diff = compute_diff(&from_rev.data, &to_rev.data, query.include_blocks);

    Ok(Json(RevisionCompareResponse {
        content_id,
        version_from: v1,
        version_to: v2,
        from_revision: from_rev.data.clone(),
        to_revision: to_rev.data.clone(),
        from_metadata: RevisionMetadata {
            revision_number: from_rev.revision_number,
            created_at: from_rev.created_at,
            created_by: from_rev.created_by,
            created_by_name: from_name,
            change_summary: from_rev.change_summary,
            change_type: from_rev.change_type,
            word_count: from_rev.word_count,
        },
        to_metadata: RevisionMetadata {
            revision_number: to_rev.revision_number,
            created_at: to_rev.created_at,
            created_by: to_rev.created_by,
            created_by_name: to_name,
            change_summary: to_rev.change_summary,
            change_type: to_rev.change_type,
            word_count: to_rev.word_count,
        },
        diff,
    }))
}

/// Restore a revision
///
/// Restores content to a previous revision state. This operation:
/// 1. Creates a new revision from the restored version
/// 2. Updates the content with the restored data
/// 3. Increments the content version
/// 4. Logs the restore operation in the audit trail
///
/// Requires admin access.
#[utoipa::path(
    post,
    path = "/api/cms/content/{id}/revisions/{version}/restore",
    params(
        ("id" = Uuid, Path, description = "Content ID"),
        ("version" = i32, Path, description = "Revision number to restore")
    ),
    responses(
        (status = 200, description = "Revision restored successfully", body = RestoreResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required"),
        (status = 404, description = "Revision not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "CMS Revisions",
    security(("bearer_auth" = []))
)]
async fn restore_revision(
    State(state): State<AppState>,
    user: User,
    Path((content_id, version)): Path<(Uuid, i32)>,
) -> ApiResult<RestoreResponse> {
    require_cms_admin(&user)?;

    // Get CMS user
    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Verify the revision exists
    let revision: CmsRevisionExtended = sqlx::query_as(
        r#"
        SELECT
            id, content_id, revision_number, is_current, data,
            change_summary, changed_fields, created_at, created_by,
            change_type, word_count, diff_stats
        FROM cms_revisions
        WHERE content_id = $1 AND revision_number = $2
        "#,
    )
    .bind(content_id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::NOT_FOUND,
            &format!("Revision {} not found", version),
        )
    })?;

    // Use the existing restore_revision function from cms_content service
    let content =
        cms_content::restore_revision(&state.db.pool, content_id, version, cms_user.map(|u| u.id))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get the new revision number (should be content.version)
    let new_revision_number: (i32,) = sqlx::query_as(
        "SELECT COALESCE(MAX(revision_number), 0) FROM cms_revisions WHERE content_id = $1",
    )
    .bind(content_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((content.version,));

    Ok(Json(RestoreResponse {
        message: format!("Successfully restored content to revision {}", version),
        restored_from_revision: version,
        new_revision_number: new_revision_number.0,
        new_content_version: content.version,
        content: serde_json::to_value(&content).unwrap_or_default(),
        restored_at: Utc::now(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// DIFF COMPUTATION
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Compute comprehensive diff between two revision data snapshots
fn compute_diff(from: &JsonValue, to: &JsonValue, include_blocks: bool) -> DiffStats {
    let mut field_changes = Vec::new();
    let mut block_changes = None;

    // Fields to compare for changes
    let simple_fields = [
        "title",
        "subtitle",
        "excerpt",
        "content",
        "slug",
        "meta_title",
        "meta_description",
        "canonical_url",
        "robots_directives",
        "template",
    ];

    let seo_fields = [
        "meta_title",
        "meta_description",
        "meta_keywords",
        "canonical_url",
        "robots_directives",
        "structured_data",
    ];

    let mut title_changed = false;
    let mut content_changed = false;
    let mut seo_changed = false;
    let mut custom_fields_changed = false;

    // Compare simple fields
    for field in &simple_fields {
        let from_val = from.get(field);
        let to_val = to.get(field);

        if from_val != to_val {
            let change_type = match (from_val, to_val) {
                (None, Some(_)) => "added",
                (Some(_), None) => "removed",
                _ => "modified",
            };

            field_changes.push(FieldChange {
                field: field.to_string(),
                from: from_val.cloned(),
                to: to_val.cloned(),
                change_type: change_type.to_string(),
            });

            if *field == "title" {
                title_changed = true;
            }
            if *field == "content" {
                content_changed = true;
            }
            if seo_fields.contains(field) {
                seo_changed = true;
            }
        }
    }

    // Check SEO fields separately
    for field in &seo_fields {
        if !simple_fields.contains(field) {
            let from_val = from.get(field);
            let to_val = to.get(field);
            if from_val != to_val {
                seo_changed = true;
                let change_type = match (from_val, to_val) {
                    (None, Some(_)) => "added",
                    (Some(_), None) => "removed",
                    _ => "modified",
                };
                field_changes.push(FieldChange {
                    field: field.to_string(),
                    from: from_val.cloned(),
                    to: to_val.cloned(),
                    change_type: change_type.to_string(),
                });
            }
        }
    }

    // Check custom_fields
    let from_custom = from.get("custom_fields");
    let to_custom = to.get("custom_fields");
    if from_custom != to_custom {
        custom_fields_changed = true;
        field_changes.push(FieldChange {
            field: "custom_fields".to_string(),
            from: from_custom.cloned(),
            to: to_custom.cloned(),
            change_type: "modified".to_string(),
        });
    }

    // Compute block-level changes
    let (blocks_added, blocks_removed, blocks_modified, blocks_reordered) = if include_blocks {
        let (added, removed, modified, reordered, changes) = compute_block_diff(from, to);
        block_changes = Some(changes);
        (added, removed, modified, reordered)
    } else {
        // Quick check without detailed diff
        let from_blocks = from.get("content_blocks").and_then(|v| v.as_array());
        let to_blocks = to.get("content_blocks").and_then(|v| v.as_array());

        match (from_blocks, to_blocks) {
            (Some(f), Some(t)) if f != t => {
                content_changed = true;
                // Rough estimate
                let diff = (t.len() as i32 - f.len() as i32).abs();
                if t.len() > f.len() {
                    (diff, 0, 0, 0)
                } else if t.len() < f.len() {
                    (0, diff, 0, 0)
                } else {
                    (0, 0, diff, 0)
                }
            }
            (None, Some(t)) => (t.len() as i32, 0, 0, 0),
            (Some(f), None) => (0, f.len() as i32, 0, 0),
            _ => (0, 0, 0, 0),
        }
    };

    if blocks_added > 0 || blocks_removed > 0 || blocks_modified > 0 {
        content_changed = true;
    }

    // Calculate word count difference
    let from_word_count = count_words(from);
    let to_word_count = count_words(to);
    let word_count_diff = to_word_count - from_word_count;

    DiffStats {
        title_changed,
        content_changed,
        seo_changed,
        custom_fields_changed,
        blocks_added,
        blocks_removed,
        blocks_modified,
        blocks_reordered,
        total_fields_changed: field_changes.len() as i32,
        word_count_diff,
        field_changes,
        block_changes,
    }
}

/// Compute detailed block-level diff
fn compute_block_diff(from: &JsonValue, to: &JsonValue) -> (i32, i32, i32, i32, Vec<BlockChange>) {
    let mut changes = Vec::new();
    let mut added = 0;
    let mut removed = 0;
    let mut modified = 0;
    let mut reordered = 0;

    let from_blocks = from
        .get("content_blocks")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let to_blocks = to
        .get("content_blocks")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // Create maps by block ID for efficient lookup
    let from_map: std::collections::HashMap<String, (usize, &JsonValue)> = from_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            b.get("id")
                .and_then(|id| id.as_str())
                .map(|id| (id.to_string(), (i, b)))
        })
        .collect();

    let to_map: std::collections::HashMap<String, (usize, &JsonValue)> = to_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            b.get("id")
                .and_then(|id| id.as_str())
                .map(|id| (id.to_string(), (i, b)))
        })
        .collect();

    // Find removed and modified blocks
    for (id, (old_pos, old_block)) in &from_map {
        let block_type = old_block
            .get("block_type")
            .and_then(|t| t.as_str())
            .unwrap_or("unknown")
            .to_string();

        if let Some((new_pos, new_block)) = to_map.get(id) {
            // Block exists in both - check for modifications or reordering
            if old_block != *new_block {
                modified += 1;
                let field_changes = compute_block_field_changes(old_block, new_block);
                changes.push(BlockChange {
                    block_id: id.clone(),
                    block_type: block_type.clone(),
                    change_type: "modified".to_string(),
                    old_position: Some(*old_pos as i32),
                    new_position: Some(*new_pos as i32),
                    changes: if field_changes.is_empty() {
                        None
                    } else {
                        Some(field_changes)
                    },
                });
            } else if old_pos != new_pos {
                reordered += 1;
                changes.push(BlockChange {
                    block_id: id.clone(),
                    block_type,
                    change_type: "reordered".to_string(),
                    old_position: Some(*old_pos as i32),
                    new_position: Some(*new_pos as i32),
                    changes: None,
                });
            }
        } else {
            // Block was removed
            removed += 1;
            changes.push(BlockChange {
                block_id: id.clone(),
                block_type,
                change_type: "removed".to_string(),
                old_position: Some(*old_pos as i32),
                new_position: None,
                changes: None,
            });
        }
    }

    // Find added blocks
    for (id, (new_pos, new_block)) in &to_map {
        if !from_map.contains_key(id) {
            added += 1;
            let block_type = new_block
                .get("block_type")
                .and_then(|t| t.as_str())
                .unwrap_or("unknown")
                .to_string();

            changes.push(BlockChange {
                block_id: id.clone(),
                block_type,
                change_type: "added".to_string(),
                old_position: None,
                new_position: Some(*new_pos as i32),
                changes: None,
            });
        }
    }

    // Sort changes by position for consistent output
    changes.sort_by(|a, b| {
        let a_pos = a.new_position.or(a.old_position).unwrap_or(999);
        let b_pos = b.new_position.or(b.old_position).unwrap_or(999);
        a_pos.cmp(&b_pos)
    });

    (added, removed, modified, reordered, changes)
}

/// Compute field-level changes within a block
fn compute_block_field_changes(old: &JsonValue, new: &JsonValue) -> Vec<FieldChange> {
    let mut changes = Vec::new();

    // Get the data objects from blocks
    let old_data = old.get("data");
    let new_data = new.get("data");

    if old_data == new_data {
        return changes;
    }

    // If both are objects, compare fields
    if let (Some(old_obj), Some(new_obj)) = (
        old_data.and_then(|d| d.as_object()),
        new_data.and_then(|d| d.as_object()),
    ) {
        // Check all fields in old
        for (key, old_val) in old_obj {
            let new_val = new_obj.get(key);
            if new_val != Some(old_val) {
                changes.push(FieldChange {
                    field: key.clone(),
                    from: Some(old_val.clone()),
                    to: new_val.cloned(),
                    change_type: if new_val.is_none() {
                        "removed"
                    } else {
                        "modified"
                    }
                    .to_string(),
                });
            }
        }

        // Check for new fields
        for (key, new_val) in new_obj {
            if !old_obj.contains_key(key) {
                changes.push(FieldChange {
                    field: key.clone(),
                    from: None,
                    to: Some(new_val.clone()),
                    change_type: "added".to_string(),
                });
            }
        }
    } else {
        // Data changed type or one is null
        changes.push(FieldChange {
            field: "data".to_string(),
            from: old_data.cloned(),
            to: new_data.cloned(),
            change_type: "modified".to_string(),
        });
    }

    // Check settings
    let old_settings = old.get("settings");
    let new_settings = new.get("settings");
    if old_settings != new_settings {
        changes.push(FieldChange {
            field: "settings".to_string(),
            from: old_settings.cloned(),
            to: new_settings.cloned(),
            change_type: "modified".to_string(),
        });
    }

    changes
}

/// Count approximate words in content
fn count_words(data: &JsonValue) -> i32 {
    let mut count = 0;

    // Count words in title
    if let Some(title) = data.get("title").and_then(|t| t.as_str()) {
        count += title.split_whitespace().count() as i32;
    }

    // Count words in content
    if let Some(content) = data.get("content").and_then(|c| c.as_str()) {
        count += content.split_whitespace().count() as i32;
    }

    // Count words in excerpt
    if let Some(excerpt) = data.get("excerpt").and_then(|e| e.as_str()) {
        count += excerpt.split_whitespace().count() as i32;
    }

    // Count words in content blocks
    if let Some(blocks) = data.get("content_blocks").and_then(|b| b.as_array()) {
        for block in blocks {
            if let Some(block_data) = block.get("data") {
                count += count_words_in_json(block_data);
            }
        }
    }

    count
}

/// Recursively count words in JSON values
fn count_words_in_json(value: &JsonValue) -> i32 {
    match value {
        JsonValue::String(s) => s.split_whitespace().count() as i32,
        JsonValue::Array(arr) => arr.iter().map(count_words_in_json).sum(),
        JsonValue::Object(obj) => obj.values().map(count_words_in_json).sum(),
        _ => 0,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Build the CMS revisions router
///
/// Provides the following endpoints:
/// - GET  /api/cms/content/:id/revisions              - List all revisions
/// - GET  /api/cms/content/:id/revisions/:version     - Get specific revision
/// - GET  /api/cms/content/:id/revisions/compare      - Compare two revisions
/// - POST /api/cms/content/:id/revisions/:version/restore - Restore a revision
pub fn router() -> Router<AppState> {
    Router::new()
        // List revisions for content
        .route("/content/:id/revisions", get(list_revisions))
        // Compare revisions - must come before :version to avoid conflict
        .route("/content/:id/revisions/compare", get(compare_revisions))
        // Get specific revision
        .route("/content/:id/revisions/:version", get(get_revision))
        // Restore revision
        .route(
            "/content/:id/revisions/:version/restore",
            post(restore_revision),
        )
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// OPENAPI SCHEMA
// ═══════════════════════════════════════════════════════════════════════════════════════

/// OpenAPI schema for CMS Revisions API
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        list_revisions,
        get_revision,
        compare_revisions,
        restore_revision
    ),
    components(
        schemas(
            CmsRevisionExtended,
            RevisionSummary,
            RevisionListResponse,
            RevisionCompareResponse,
            RevisionMetadata,
            DiffStats,
            FieldChange,
            BlockChange,
            RestoreResponse
        )
    ),
    tags(
        (name = "CMS Revisions", description = "Content revision management and comparison API")
    )
)]
pub struct CmsRevisionsApi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff_title_change() {
        let from = json!({
            "title": "Old Title",
            "content": "Same content"
        });
        let to = json!({
            "title": "New Title",
            "content": "Same content"
        });

        let diff = compute_diff(&from, &to, false);
        assert!(diff.title_changed);
        assert!(!diff.content_changed);
        assert_eq!(diff.total_fields_changed, 1);
    }

    #[test]
    fn test_compute_diff_block_added() {
        let from = json!({
            "title": "Test",
            "content_blocks": [
                {"id": "block1", "block_type": "rich-text", "data": {"text": "Hello"}}
            ]
        });
        let to = json!({
            "title": "Test",
            "content_blocks": [
                {"id": "block1", "block_type": "rich-text", "data": {"text": "Hello"}},
                {"id": "block2", "block_type": "image", "data": {"src": "test.jpg"}}
            ]
        });

        let diff = compute_diff(&from, &to, true);
        assert!(diff.content_changed);
        assert_eq!(diff.blocks_added, 1);
        assert_eq!(diff.blocks_removed, 0);
    }

    #[test]
    fn test_count_words() {
        let data = json!({
            "title": "Hello World",
            "content": "This is a test content with some words",
            "content_blocks": [
                {"data": {"text": "Block text here"}}
            ]
        });

        let count = count_words(&data);
        assert!(count > 0);
    }
}
