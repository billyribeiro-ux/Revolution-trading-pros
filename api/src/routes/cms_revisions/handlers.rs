//! Axum handlers for the CMS revisions sub-module.
//!
//! R27-B4 split (2026-05-20) — extracted verbatim from the original
//! `routes/cms_revisions.rs` (lines 325-702). Every SQL statement,
//! audit log, tracing field, error mapping, and `require_cms_*` gate
//! call is preserved byte-for-byte.
//!
//! Handlers:
//!   - `list_revisions`     — GET  /content/:id/revisions
//!   - `get_revision`       — GET  /content/:id/revisions/:version
//!   - `compare_revisions`  — GET  /content/:id/revisions/compare
//!   - `restore_revision`   — POST /content/:id/revisions/:version/restore

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::{models::User, services::cms_content, AppState};

use super::diff::compute_diff;
use super::dtos::{
    CmsRevisionExtended, CmsRevisionWithCreator, CompareQuery, RestoreResponse,
    RevisionCompareResponse, RevisionListQuery, RevisionListResponse, RevisionMetadata,
    RevisionSummary,
};
use super::errors::{api_error, require_cms_admin, require_cms_editor, ApiResult};

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
pub(super) async fn list_revisions(
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
    let revisions: Vec<CmsRevisionWithCreator> = sqlx::query_as(
        r"
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
        ",
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
        .map(|r| RevisionSummary {
            id: r.id,
            revision_number: r.revision_number,
            is_current: r.is_current,
            change_summary: r.change_summary,
            changed_fields: r.changed_fields,
            created_at: r.created_at,
            created_by: r.created_by,
            created_by_name: r.created_by_name,
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
pub(super) async fn get_revision(
    State(state): State<AppState>,
    user: User,
    Path((content_id, version)): Path<(Uuid, i32)>,
) -> ApiResult<CmsRevisionExtended> {
    require_cms_editor(&user)?;

    let revision: CmsRevisionExtended = sqlx::query_as(
        r"
        SELECT
            id, content_id, revision_number, is_current, data,
            change_summary, changed_fields, created_at, created_by,
            change_type, word_count, diff_stats
        FROM cms_revisions
        WHERE content_id = $1 AND revision_number = $2
        ",
    )
    .bind(content_id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::NOT_FOUND,
            &format!("Revision {version} not found for content"),
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
pub(super) async fn compare_revisions(
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
        r"
        SELECT
            r.id, r.content_id, r.revision_number, r.is_current, r.data,
            r.change_summary, r.changed_fields, r.created_at, r.created_by,
            r.change_type, r.word_count, r.diff_stats,
            u.display_name as created_by_name
        FROM cms_revisions r
        LEFT JOIN cms_users u ON r.created_by = u.id
        WHERE r.content_id = $1 AND r.revision_number = $2
        ",
    )
    .bind(content_id)
    .bind(v1)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let to_revision: Option<CmsRevisionWithCreator> = sqlx::query_as(
        r"
        SELECT
            r.id, r.content_id, r.revision_number, r.is_current, r.data,
            r.change_summary, r.changed_fields, r.created_at, r.created_by,
            r.change_type, r.word_count, r.diff_stats,
            u.display_name as created_by_name
        FROM cms_revisions r
        LEFT JOIN cms_users u ON r.created_by = u.id
        WHERE r.content_id = $1 AND r.revision_number = $2
        ",
    )
    .bind(content_id)
    .bind(v2)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let from_rev = from_revision
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, &format!("Revision {v1} not found")))?;

    let to_rev = to_revision
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, &format!("Revision {v2} not found")))?;

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
pub(super) async fn restore_revision(
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
        r"
        SELECT
            id, content_id, revision_number, is_current, data,
            change_summary, changed_fields, created_at, created_by,
            change_type, word_count, diff_stats
        FROM cms_revisions
        WHERE content_id = $1 AND revision_number = $2
        ",
    )
    .bind(content_id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| {
        api_error(
            StatusCode::NOT_FOUND,
            &format!("Revision {version} not found"),
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
        message: format!("Successfully restored content to revision {version}"),
        restored_from_revision: version,
        new_revision_number: new_revision_number.0,
        new_content_version: content.version,
        content: serde_json::to_value(&content).unwrap_or_default(),
        restored_at: Utc::now(),
    }))
}
