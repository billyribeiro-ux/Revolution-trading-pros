//! CMS Global Components Library API Routes
//!
//! Comprehensive API for managing global components in the CMS.
//! Enables managing headers, footers, CTAs, and other repeated elements from one place.
//!
//! Features:
//! - CRUD operations for global components
//! - Categories: headers, footers, CTAs, forms, navigation
//! - Usage tracking (which pages use each component)
//! - Sync/detach functionality for component instances
//! - Version history for all changes
//! - Search and filter capabilities
//!
//! @version 1.0.0 - February 2026
//!
//! Module layout (split from the original 1,883-LOC `cms_global_components.rs`):
//! - `types`    — enums, models, request/response DTOs, error helpers
//! - `helpers`  — auth checks + slug/user/version utilities
//! - `crud`     — list / get / get-by-slug / create / update / delete / duplicate (7 handlers)
//! - `versions` — version history + restore (2 handlers)
//! - `usage`    — list / track / remove component usage (3 handlers)
//! - `sync`     — sync / detach / reattach component instances (3 handlers)
//! - `public`   — no-auth read-only endpoints (2 handlers)

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::AppState;

mod crud;
mod helpers;
mod public;
mod sync;
mod types;
mod usage;
mod versions;

// Re-export public types for utoipa schema registration / external API surface.
pub use types::{
    CategoryCount, ComponentUsageResponse, CreateGlobalComponentRequest, GlobalComponent,
    GlobalComponentCategory, GlobalComponentSummary, GlobalComponentUsage,
    GlobalComponentUsageWithContent, GlobalComponentVersion, GlobalComponentWithMeta,
    ListGlobalComponentsQuery, PaginatedComponentsResponse, PaginationMeta, SyncComponentRequest,
    TrackComponentUsageRequest, UpdateGlobalComponentRequest, VersionHistoryResponse,
};

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for global components (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Component CRUD
        .route(
            "/",
            get(crud::list_global_components).post(crud::create_global_component),
        )
        .route(
            "/{id}",
            get(crud::get_global_component)
                .put(crud::update_global_component)
                .delete(crud::delete_global_component),
        )
        .route("/slug/{slug}", get(crud::get_global_component_by_slug))
        .route("/{id}/duplicate", post(crud::duplicate_global_component))
        // Version history
        .route("/{id}/versions", get(versions::get_version_history))
        .route(
            "/{id}/versions/{version}/restore",
            post(versions::restore_version),
        )
        // Usage tracking
        .route("/{id}/usage", get(usage::get_component_usage))
        .route("/usage", post(usage::track_component_usage))
        .route("/usage/{id}", delete(usage::remove_component_usage))
        .route("/usage/{id}/sync", post(sync::sync_component_usage))
        .route("/usage/{id}/detach", post(sync::detach_component_usage))
        .route("/usage/{id}/reattach", post(sync::reattach_component_usage))
}

/// Public router for read-only component access
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/slug/{slug}", get(public::get_global_component_public))
        .route("/{id}", get(public::get_global_component_by_id_public))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::helpers::generate_slug;
    use super::types::{default_true, GlobalComponentCategory};

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Main Header"), "main-header");
        assert_eq!(generate_slug("Footer v2"), "footer-v2");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_category_display() {
        assert_eq!(format!("{}", GlobalComponentCategory::Header), "header");
        assert_eq!(format!("{}", GlobalComponentCategory::Footer), "footer");
        assert_eq!(format!("{}", GlobalComponentCategory::Cta), "cta");
        assert_eq!(format!("{}", GlobalComponentCategory::Form), "form");
        assert_eq!(
            format!("{}", GlobalComponentCategory::Navigation),
            "navigation"
        );
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }
}
