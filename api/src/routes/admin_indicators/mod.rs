//! Indicator Management Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full-service indicator management matching existing database schema.
//! Database uses BIGINT id, not UUID.
//!
//! ## Structure
//!
//! This module is a pure structural split of the legacy
//! `admin_indicators.rs` (1,104 LOC). Sub-modules:
//!
//! - [`types`]      — Public DTOs (re-exported from this module)
//! - [`helpers`]    — `slugify()` (internal)
//! - [`crud`]       — indicator CRUD: list / get / create / update / delete / toggle
//! - [`files`]      — multi-platform file management endpoints
//! - [`videos`]     — Bunny-backed video management endpoints
//! - [`analytics`]  — download-count rollup endpoint
//! - [`pricing`]    — Stripe-synchronised price change (BEGIN/COMMIT + audit row)
//!
//! ## Public API
//!
//! Re-exports kept stable for `tests/admin_indicators_test.rs` and any
//! external consumers via `pub use types::*`:
//!
//! - `CreateFileRequest`, `CreateIndicatorRequest`, `CreateVideoRequest`
//! - `IndicatorFileRow`, `IndicatorVideoRow`, `IndicatorRow`
//! - `IndicatorQueryParams`, `UpdateIndicatorRequest`, `UpdateFileRequest`
//!
//! Plus the [`router`] constructor with the unchanged route table.

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::AppState;

mod analytics;
mod crud;
mod files;
mod helpers;
mod pricing;
pub mod types;
mod videos;

pub use types::{
    CreateFileRequest, CreateIndicatorRequest, CreateVideoRequest, IndicatorFileRow,
    IndicatorQueryParams, IndicatorRow, IndicatorVideoRow, UpdateFileRequest,
    UpdateIndicatorRequest,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Indicator CRUD
        .route("/", get(crud::list_indicators).post(crud::create_indicator))
        .route(
            "/{id}",
            get(crud::get_indicator)
                .put(crud::update_indicator)
                .delete(crud::delete_indicator),
        )
        .route("/{id}/toggle", post(crud::toggle_indicator))
        .route("/{id}/change-price", post(pricing::change_indicator_price))
        // ICT 7: File management
        .route(
            "/{id}/files",
            get(files::list_indicator_files).post(files::create_indicator_file),
        )
        .route(
            "/{id}/files/{file_id}",
            put(files::update_indicator_file).delete(files::delete_indicator_file),
        )
        // ICT 7: Video management
        .route(
            "/{id}/videos",
            get(videos::list_indicator_videos).post(videos::create_indicator_video),
        )
        .route(
            "/{id}/videos/{video_id}",
            delete(videos::delete_indicator_video),
        )
        // ICT 7: Analytics
        .route("/{id}/analytics", get(analytics::get_download_analytics))
}
