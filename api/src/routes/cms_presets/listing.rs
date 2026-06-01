//! List / index endpoints for the CMS presets API:
//!
//! - `list_presets` — paginated, filterable listing
//! - `get_presets_by_block_type` — presets grouped by category for a
//!   given block type (admin / authenticated view)
//! - `get_block_types_with_presets` — block-type roll-up with counts
//!
//! Split from the original `cms_presets.rs` (R25-B). Pure structural
//! move — handler bodies are unchanged.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{models::User, utils::errors::ApiError, AppState};

use super::dto::{
    BlockTypeWithPresets, BlockTypesWithPresetsResponse, CmsPresetSummary, GroupedPresetsResponse,
    ListPresetsQuery, PaginatedPresetsResponse, PaginationMeta, PresetsByCategory,
};
use super::helpers::{require_cms_editor, ApiResult};

/// List presets with pagination and filtering
///
/// Supports filtering by block type, category, tags, and search term.
/// Results are paginated with customizable sort order.
#[utoipa::path(
    get,
    path = "/api/cms/presets",
    tag = "CMS Presets",
    params(ListPresetsQuery),
    responses(
        (status = 200, description = "List of presets", body = PaginatedPresetsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - editor access required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn list_presets(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListPresetsQuery>,
) -> ApiResult<PaginatedPresetsResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(50).min(200);
    let offset = query.offset.unwrap_or(0);

    // Sort configuration - validated against whitelist to prevent SQL injection
    let sort_column = match query.sort_by.as_deref() {
        Some("name") => "name",
        Some("usage_count") => "usage_count",
        Some("updated_at") => "updated_at",
        Some("block_type") => "block_type",
        _ => "created_at",
    };
    let sort_order = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref().map(|s| format!("%{s}%"));

    // Count total using parameterized query
    let count_result: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)
        FROM cms_presets
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR block_type = $1)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
          AND ($5::boolean IS NULL OR is_default = $5)
        ",
    )
    .bind(&query.block_type)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .bind(query.is_default)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch presets using parameterized query with safe sort columns
    let presets: Vec<CmsPresetSummary> = sqlx::query_as(&format!(
        r"
        SELECT id, block_type, name, slug, description, thumbnail_url, thumbnail_blurhash,
               category::text as category, tags, is_default, is_locked, is_global,
               usage_count, created_at, updated_at
        FROM cms_presets
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR block_type = $1)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
          AND ($5::boolean IS NULL OR is_default = $5)
        ORDER BY is_default DESC, {sort_column} {sort_order}
        LIMIT $6 OFFSET $7
        "
    ))
    .bind(&query.block_type)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .bind(query.is_default)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PaginatedPresetsResponse {
        data: presets,
        meta: PaginationMeta {
            total,
            limit,
            offset,
            has_more: offset + limit < total,
        },
    }))
}

/// Get presets for a specific block type, grouped by category
///
/// Returns presets organized by category for the block inserter UI.
#[utoipa::path(
    get,
    path = "/api/cms/presets/block/{block_type}",
    tag = "CMS Presets",
    params(
        ("block_type" = String, Path, description = "Block type (e.g., button, heading)")
    ),
    responses(
        (status = 200, description = "Presets grouped by category", body = GroupedPresetsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_presets_by_block_type(
    State(state): State<AppState>,
    user: User,
    Path(block_type): Path<String>,
) -> ApiResult<GroupedPresetsResponse> {
    require_cms_editor(&user)?;

    // Fetch all presets for this block type
    let presets: Vec<CmsPresetSummary> = sqlx::query_as(
        r"
        SELECT id, block_type, name, slug, description, thumbnail_url, thumbnail_blurhash,
               category::text as category, tags, is_default, is_locked, is_global,
               usage_count, created_at, updated_at
        FROM cms_presets
        WHERE block_type = $1 AND deleted_at IS NULL AND is_global = true
        ORDER BY category, is_default DESC, usage_count DESC, name
        ",
    )
    .bind(&block_type)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_count = presets.len() as i64;

    // Group by category
    let mut category_map: std::collections::HashMap<String, Vec<CmsPresetSummary>> =
        std::collections::HashMap::new();

    for preset in presets {
        category_map
            .entry(preset.category.clone())
            .or_default()
            .push(preset);
    }

    // Convert to response format with specific ordering
    let category_order = [
        "default",
        "brand",
        "trading",
        "marketing",
        "seasonal",
        "custom",
    ];
    let mut categories: Vec<PresetsByCategory> = Vec::new();

    for cat in category_order {
        if let Some(preset_list) = category_map.remove(cat) {
            categories.push(PresetsByCategory {
                category: cat.to_string(),
                presets: preset_list,
            });
        }
    }

    // Add any remaining categories
    for (cat, preset_list) in category_map {
        categories.push(PresetsByCategory {
            category: cat,
            presets: preset_list,
        });
    }

    Ok(Json(GroupedPresetsResponse {
        block_type,
        categories,
        total_count,
    }))
}

/// Get all block types that have presets
#[utoipa::path(
    get,
    path = "/api/cms/presets/block-types",
    tag = "CMS Presets",
    responses(
        (status = 200, description = "Block types with preset counts", body = BlockTypesWithPresetsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_block_types_with_presets(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<BlockTypesWithPresetsResponse> {
    require_cms_editor(&user)?;

    let block_types: Vec<BlockTypeWithPresets> = sqlx::query_as(
        r"
        SELECT
            block_type,
            COUNT(*)::BIGINT AS preset_count,
            BOOL_OR(is_default) AS has_default
        FROM cms_presets
        WHERE deleted_at IS NULL AND is_global = true
        GROUP BY block_type
        ORDER BY block_type
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(BlockTypesWithPresetsResponse { data: block_types }))
}
