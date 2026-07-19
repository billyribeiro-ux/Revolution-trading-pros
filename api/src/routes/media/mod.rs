//! Media Controller - ICT 11+ Principal Engineer
//!
//! Manages media library with CRUD operations, filtering, pagination,
//! and full file upload support with Cloudflare R2 integration.
//!
//! Features:
//! - File upload with automatic optimization
//! - Presigned URL generation for direct uploads
//! - Image dimension extraction
//! - MIME type validation
//!
//! R27-B split: this module was a single 1269-line file. It has been
//! broken up by sub-domain (dto / helpers / crud / uploads / bulk /
//! scanning) for navigability. Public API is unchanged — `admin_router()`
//! is the only externally-mounted entry point, and every public DTO that
//! was reachable as `routes::media::Foo` (consumed by `tests/media_test.rs`)
//! is re-exported below.
//!
//! @version 2.0.0 - January 2026

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod bulk;
mod crud;
mod dto;
mod helpers;
mod scanning;
mod uploads;

// Re-export every DTO that `tests/media_test.rs` reaches into via
// `revolution_api::routes::media::{...}`. Keeping the same flat
// surface preserves the pre-R27-B import paths.
pub use dto::{
    BulkDeleteRequest, BulkUpdateRequest, CleanupResult, ConfirmUploadRequest, MalwareScanResult,
    Media, MediaQuery, PresignedUploadRequest, PresignedUploadResponse, UpdateMedia,
};

/// Build the media admin router
/// ICT 7+ Principal Engineer - Complete Media Management API
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(crud::index))
        .route("/files", get(crud::index)) // Frontend uses /admin/media/files
        .route("/upload", post(uploads::direct_upload))
        .route("/presigned-upload", post(uploads::presigned_upload))
        .route("/confirm-upload", post(uploads::confirm_upload))
        .route("/statistics", get(crud::statistics))
        .route("/bulk-delete", post(bulk::bulk_delete))
        .route("/bulk-update", post(bulk::bulk_update))
        .route("/cleanup-orphans", post(bulk::cleanup_orphaned_files))
        .route("/scan/{id}", post(scanning::scan_media_file))
        .route(
            "/{id}",
            get(crud::show).put(crud::update).delete(crud::destroy),
        )
}
