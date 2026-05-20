//! CMS Reusable Blocks API Routes - Apple ICT 7+ Principal Engineer Grade
//!
//! Comprehensive API for managing reusable content blocks in the CMS.
//! Supports block libraries, usage tracking, versioning, and sync management.
//!
//! Features:
//! - CRUD operations for reusable blocks
//! - Block duplication with version control
//! - Usage tracking across content items
//! - Sync/detach functionality for block instances
//! - Category and tag-based organization
//! - Full-text search support
//!
//! @version 1.0.0 - January 2026
//!
//! Module layout (split from the original 1,236-LOC
//! `cms_reusable_blocks.rs`):
//! - `types`   — enums, models, request/response DTOs, type aliases
//! - `helpers` — auth checks + slug utilities
//! - `crud`    — list / get / get-by-slug / create / update / delete /
//!   duplicate (7 handlers)
//! - `usage`   — get usages / track / remove / detach (4 handlers)
//! - `public`  — no-auth read-only block-by-slug (1 handler)

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::AppState;

mod crud;
mod helpers;
mod public;
mod types;
mod usage;

// Re-export public types for utoipa schema registration / external API
// surface. The contract test in `tests/cms_reusable_blocks_test.rs`
// consumes these at `routes::cms_reusable_blocks::*`.
pub use types::{
    BlockUsageResponse, CmsReusableBlock, CmsReusableBlockCategory, CmsReusableBlockSummary,
    CmsReusableBlockUsage, CmsReusableBlockUsageWithContent, CreateReusableBlockRequest,
    ListReusableBlocksQuery, PaginatedBlocksResponse, PaginationMeta, TrackBlockUsageRequest,
    UpdateReusableBlockRequest,
};

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for reusable blocks (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Block CRUD
        .route(
            "/",
            get(crud::list_reusable_blocks).post(crud::create_reusable_block),
        )
        .route(
            "/{id}",
            get(crud::get_reusable_block)
                .put(crud::update_reusable_block)
                .delete(crud::delete_reusable_block),
        )
        .route("/slug/{slug}", get(crud::get_reusable_block_by_slug))
        .route("/{id}/duplicate", post(crud::duplicate_reusable_block))
        .route("/{id}/usage", get(usage::get_block_usage))
        // Usage tracking
        .route("/usage", post(usage::track_block_usage))
        .route("/usage/{id}", delete(usage::remove_block_usage))
        .route("/usage/{id}/detach", post(usage::detach_block_usage))
}

/// Public router for read-only block access (optional, for block previews)
pub fn public_router() -> Router<AppState> {
    Router::new().route(
        "/slug/{slug}",
        get(public::get_reusable_block_by_slug_public),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::helpers::generate_slug;
    use super::types::default_true;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hero Section"), "hero-section");
        assert_eq!(generate_slug("CTA Banner v2"), "cta-banner-v2");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }
}
