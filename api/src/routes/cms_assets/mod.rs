//! CMS Asset Manager API - Digital Asset Manager
//! ============================================
//! Apple ICT 11+ Principal Engineer Grade
//!
//! Enterprise-grade Digital Asset Management with:
//! - Folder organization with hierarchy
//! - Rich metadata (alt, title, caption, copyright)
//! - Usage tracking across content
//! - Bulk operations (delete, move, tag)
//! - Image optimization on upload
//! - Search by filename, tags, metadata
//! - Filter by type, folder, date
//!
//! @version 1.0.0 - February 2026
//!
//! ## Module layout (R22-B structural split)
//!
//! This module was split from a single 1,444-line `cms_assets.rs`
//! into the following submodules. The split is purely structural:
//! every public symbol (`router`, DTOs) preserves its old path via
//! `pub use dto::*;` so `tests/cms_assets_test.rs` and the
//! `routes/mod.rs` nest call (`/cms/assets`) keep compiling unchanged.
//!
//! - [`dto`]     — request/response/data structs (re-exported publicly)
//! - [`helpers`] — `slugify`, `format_bytes`, `get_cms_user_id`
//! - [`folders`] — folder hierarchy CRUD
//! - [`listing`] — list / recent / get-by-id
//! - [`crud`]    — single-asset CRUD + restore + replace
//! - [`bulk`]    — bulk move / delete / tag
//! - [`usage`]   — usage tracking
//! - [`stats`]   — library statistics + tag aggregation

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::AppState;

mod bulk;
mod crud;
pub mod dto;
mod folders;
mod helpers;
mod listing;
mod stats;
mod usage;

// Re-export DTOs so the public path `routes::cms_assets::{Asset, …}`
// remains identical to the pre-split layout. The integration test
// `tests/cms_assets_test.rs` imports symbols through this path.
pub use dto::*;

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Build the CMS Assets router - DAM API
pub fn router() -> Router<AppState> {
    Router::new()
        // Asset listing & search
        .route("/", get(listing::list_assets))
        .route("/recent", get(listing::recent_assets))
        .route("/stats", get(stats::get_stats))
        .route("/tags", get(stats::get_all_tags))
        // Folders
        .route(
            "/folders",
            get(folders::list_folders).post(folders::create_folder),
        )
        .route(
            "/folders/{id}",
            put(folders::update_folder).delete(folders::delete_folder),
        )
        // Single asset operations
        .route("/upload", post(crud::create_asset))
        .route(
            "/{id}",
            get(listing::get_asset)
                .put(crud::update_asset)
                .delete(crud::delete_asset),
        )
        .route("/{id}/restore", post(crud::restore_asset))
        .route("/{id}/replace", post(crud::replace_asset))
        .route("/{id}/usage", get(usage::get_asset_usage))
        .route("/{id}/track-usage", post(usage::track_usage))
        // Bulk operations
        .route("/bulk/move", post(bulk::bulk_move))
        .route("/bulk/delete", post(bulk::bulk_delete))
        .route("/bulk/tag", post(bulk::bulk_tag))
}
