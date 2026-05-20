//! CMS Presets API Routes - Component Presets/Templates
//!
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Comprehensive API for managing block presets/templates in the CMS.
//! Provides pre-designed block configurations for quick content creation.
//!
//! Features:
//! - CRUD operations for presets
//! - Block type filtering
//! - Category organization (default, brand, seasonal, etc.)
//! - Thumbnail preview support
//! - Usage tracking
//! - Search and filtering
//!
//! @version 1.0.0 - February 2026
//!
//! ## Module layout (R25-B structural split)
//!
//! This module was split from a single 1,364-line `cms_presets.rs`
//! into the following submodules. The split is purely structural:
//! every public symbol (`admin_router`, `public_router`, DTOs) keeps
//! its old path via `pub use dto::*;` so `tests/cms_presets_test.rs`
//! and the `routes/mod.rs` nest calls (`/cms/presets`,
//! `/cms/presets/public`) keep compiling unchanged.
//!
//! - [`dto`]       — request/response/data structs (re-exported publicly)
//! - [`helpers`]   — RBAC checks, slug generation, slug uniqueness,
//!   shared `ApiResult` type aliases
//! - [`listing`]   — list + group-by-block-type + block-types index
//! - [`retrieval`] — single-preset reads (by id, by slug)
//! - [`crud`]      — create / update / soft-delete
//! - [`actions`]   — duplicate, apply, save-from-block
//! - [`public`]    — unauthenticated read-only endpoint

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod actions;
mod crud;
pub mod dto;
mod helpers;
mod listing;
mod public;
mod retrieval;

// Re-export DTOs so the public path
// `routes::cms_presets::{CmsPreset, CmsPresetCategory, ...}`
// remains identical to the pre-split layout. The integration test
// `tests/cms_presets_test.rs` imports symbols through this path.
pub use dto::*;

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for presets (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Listing and retrieval
        .route("/", get(listing::list_presets).post(crud::create_preset))
        .route("/block-types", get(listing::get_block_types_with_presets))
        .route(
            "/block/{block_type}",
            get(listing::get_presets_by_block_type),
        )
        .route(
            "/block/{block_type}/slug/{slug}",
            get(retrieval::get_preset_by_slug),
        )
        // CRUD by ID
        .route(
            "/{id}",
            get(retrieval::get_preset)
                .put(crud::update_preset)
                .delete(crud::delete_preset),
        )
        // Actions
        .route("/{id}/duplicate", post(actions::duplicate_preset))
        .route("/{id}/apply", post(actions::apply_preset))
        // Save from block
        .route("/save-from-block", post(actions::save_block_as_preset))
}

/// Public router for read-only preset access
pub fn public_router() -> Router<AppState> {
    Router::new().route(
        "/block/{block_type}",
        get(public::get_presets_by_block_type_public),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::dto::{default_true, CmsPresetCategory};
    use super::helpers::generate_slug;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Primary CTA"), "primary-cta");
        assert_eq!(generate_slug("Hero Title v2"), "hero-title-v2");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }

    #[test]
    fn test_preset_category_serialization() {
        let category = CmsPresetCategory::Default;
        let json = serde_json::to_string(&category).unwrap();
        assert_eq!(json, "\"default\"");

        let category = CmsPresetCategory::Trading;
        let json = serde_json::to_string(&category).unwrap();
        assert_eq!(json, "\"trading\"");
    }
}
