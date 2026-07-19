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
//!
//! ---
//!
//! R27-B4 SPLIT (2026-05-20): Originally a 1,209 LOC monolith
//! `routes/cms_revisions.rs`. Split into 4 sub-domain files preserving
//! the exact public API (`cms_revisions::router()`, the re-exported
//! DTOs, and `CmsRevisionsApi`) byte-for-byte. No SQL, audit log, or
//! tracing field was changed.
//!
//! - `errors`   — `api_error`, `require_cms_editor`,
//!   `require_cms_admin`, `ApiResult` type aliases.
//! - `dtos`     — Public request / response DTOs
//!   (`CmsRevisionExtended`, `RevisionSummary`, `RevisionListQuery`,
//!   `RevisionListResponse`, `CompareQuery`, `FieldChange`,
//!   `BlockChange`, `DiffStats`, `RevisionMetadata`,
//!   `RevisionCompareResponse`, `RestoreResponse`) + internal
//!   `CmsRevisionWithCreator` row type.
//! - `diff`     — Pure-Rust `compute_diff` / `count_words` helpers
//!   plus the original `#[cfg(test)] mod tests` block (R15-D —
//!   preserved verbatim).
//! - `handlers` — Axum handlers (`list_revisions`, `get_revision`,
//!   `compare_revisions`, `restore_revision`).
//!
//! Public-API contract pinned by `tests/cms_revisions_test.rs`
//! (R15-D — 7 tests, all passing post-split). The test file imports
//! DTOs directly from `revolution_api::routes::cms_revisions::{...}`
//! and exercises `router()`; both are preserved via the `pub use`
//! re-exports below.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod diff;
mod dtos;
mod errors;
mod handlers;

// Re-export the public DTOs so external callers
// (`revolution_api::routes::cms_revisions::CmsRevisionExtended` etc.)
// keep working unchanged. These exact names are imported by
// `tests/cms_revisions_test.rs`.
pub use dtos::{
    BlockChange, CmsRevisionExtended, CompareQuery, DiffStats, FieldChange, RestoreResponse,
    RevisionCompareResponse, RevisionListQuery, RevisionListResponse, RevisionMetadata,
    RevisionSummary,
};

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
        .route("/content/{id}/revisions", get(handlers::list_revisions))
        // Compare revisions - must come before :version to avoid conflict
        .route(
            "/content/{id}/revisions/compare",
            get(handlers::compare_revisions),
        )
        // Get specific revision
        .route(
            "/content/{id}/revisions/{version}",
            get(handlers::get_revision),
        )
        // Restore revision
        .route(
            "/content/{id}/revisions/{version}/restore",
            post(handlers::restore_revision),
        )
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// OPENAPI SCHEMA
// ═══════════════════════════════════════════════════════════════════════════════════════

/// OpenAPI schema for CMS Revisions API
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        handlers::list_revisions,
        handlers::get_revision,
        handlers::compare_revisions,
        handlers::restore_revision
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
