//! Public (unauthenticated) read endpoint for the CMS presets API.
//!
//! Exposes presets-by-block-type, filtered to `is_global = true`
//! entries only, for use in the public site / signed-out editors.
//!
//! Split from the original `cms_presets.rs` (R25-B). Pure structural
//! move — handler body is unchanged.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{utils::errors::ApiError, AppState};

use super::dto::{CmsPresetSummary, GroupedPresetsResponse, PresetsByCategory};
use super::helpers::ApiResult;

/// Public endpoint to get presets by block type (global only, no auth)
pub(super) async fn get_presets_by_block_type_public(
    State(state): State<AppState>,
    Path(block_type): Path<String>,
) -> ApiResult<GroupedPresetsResponse> {
    // Fetch all global presets for this block type
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

    // Convert to response format
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
