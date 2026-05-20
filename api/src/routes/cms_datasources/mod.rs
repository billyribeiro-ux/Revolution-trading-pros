//! CMS Datasources API Routes - Reusable Option Lists
//!
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Comprehensive API for managing datasources (reusable option lists) in the CMS.
//! Provides key-value pairs for dropdowns, selects, and other form elements.
//!
//! Features:
//! - CRUD operations for datasources
//! - Entry management with drag-to-reorder
//! - CSV import/export
//! - Dimension support for translations
//! - Pagination for large datasources
//! - Search and filtering
//!
//! @version 1.0.0 - February 2026
//!
//! Module layout (split from the original 1,636-LOC `cms_datasources.rs`):
//! - `types`          — DB models, request/response DTOs, query structs
//! - `helpers`        — auth checks + slug utilities (private to this module)
//! - `datasources`    — datasource CRUD: list / get / create / update / delete / duplicate (7 handlers)
//! - `entries`        — entry CRUD: list / create / bulk / update / delete / reorder (6 handlers)
//! - `import_export`  — CSV export + import (2 handlers)
//! - `public`         — no-auth read-only endpoint (1 handler)

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::AppState;

mod datasources;
mod entries;
mod helpers;
mod import_export;
mod public;
mod types;

// Re-export public types for utoipa schema registration / external API surface.
pub use types::{
    BulkCreateEntriesRequest, CmsDatasource, CmsDatasourceEntry, CmsDatasourceEntrySummary,
    CmsDatasourceSummary, CreateDatasourceRequest, CreateEntryRequest, CsvEntryRow,
    ListDatasourcesQuery, ListEntriesQuery, PaginatedDatasourcesResponse, PaginatedEntriesResponse,
    PaginationMeta, PublicDatasourceResponse, PublicEntryResponse, ReorderEntriesRequest,
    UpdateDatasourceRequest, UpdateEntryRequest,
};

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for datasources (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Datasource CRUD
        .route(
            "/",
            get(datasources::list_datasources).post(datasources::create_datasource),
        )
        .route("/slug/{slug}", get(datasources::get_datasource_by_slug))
        .route(
            "/{id}",
            get(datasources::get_datasource)
                .put(datasources::update_datasource)
                .delete(datasources::delete_datasource),
        )
        .route("/{id}/duplicate", post(datasources::duplicate_datasource))
        // Entry management
        .route(
            "/{datasource_id}/entries",
            get(entries::list_entries).post(entries::create_entry),
        )
        .route(
            "/{datasource_id}/entries/bulk",
            post(entries::bulk_create_entries),
        )
        .route(
            "/{datasource_id}/entries/reorder",
            put(entries::reorder_entries),
        )
        .route(
            "/{datasource_id}/entries/{entry_id}",
            put(entries::update_entry).delete(entries::delete_entry),
        )
        // Import/Export
        .route(
            "/{datasource_id}/export",
            get(import_export::export_entries_csv),
        )
        .route(
            "/{datasource_id}/import",
            post(import_export::import_entries_csv),
        )
}

/// Public router for read-only datasource access (no auth required)
pub fn public_router() -> Router<AppState> {
    Router::new().route("/{slug}", get(public::public_get_entries))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::helpers::generate_slug;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("US States"), "us-states");
        assert_eq!(generate_slug("Color Palettes"), "color-palettes");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }
}
