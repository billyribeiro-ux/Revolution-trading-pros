//! CMS Assets — listing, search, single-asset reads
//!
//! Endpoints (mounted under `/cms/assets` in `mod.rs::router()`):
//!   GET /            — paginated search with filters
//!   GET /recent      — most-recent N uploads (quick access)
//!   GET /:id         — single asset with full metadata

use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

use super::dto::{Asset, AssetListQuery, AssetSummary, PaginatedAssets, PaginationMeta};

/// GET /cms/assets - List assets with search, filter, pagination
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn list_assets(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<AssetListQuery>,
) -> Result<Json<PaginatedAssets>, ApiError> {
    let mut conditions = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut param_idx = 1;

    // Exclude deleted unless requested
    if !params.include_deleted.unwrap_or(false) {
        conditions.push("deleted_at IS NULL".to_string());
    }

    // Folder filter
    if let Some(folder_id) = params.folder_id {
        conditions.push(format!("folder_id = ${param_idx}"));
        bind_values.push(folder_id.to_string());
        param_idx += 1;
    }

    // Type filter (image, video, audio, document)
    if let Some(ref asset_type) = params.r#type {
        let mime_prefix = match asset_type.as_str() {
            "image" => "image/%",
            "video" => "video/%",
            "audio" => "audio/%",
            "document" => "application/%",
            _ => "%",
        };
        conditions.push(format!("mime_type LIKE ${param_idx}"));
        bind_values.push(mime_prefix.to_string());
        param_idx += 1;
    }

    // Search filter
    if let Some(ref search) = params.search {
        let sanitized: String = search
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.')
            .take(100)
            .collect();
        if !sanitized.is_empty() {
            conditions.push(format!(
                "(filename ILIKE ${} OR title ILIKE ${} OR alt_text ILIKE ${} OR description ILIKE ${})",
                param_idx, param_idx + 1, param_idx + 2, param_idx + 3
            ));
            let pattern = format!("%{sanitized}%");
            bind_values.push(pattern.clone());
            bind_values.push(pattern.clone());
            bind_values.push(pattern.clone());
            bind_values.push(pattern);
            param_idx += 4;
        }
    }

    // Tags filter
    if let Some(ref tags_str) = params.tags {
        let tags: Vec<&str> = tags_str.split(',').map(|s| s.trim()).collect();
        if !tags.is_empty() {
            conditions.push(format!("tags && ${param_idx}"));
            bind_values.push(format!("{{{}}}", tags.join(",")));
            param_idx += 1;
        }
    }

    // Dimension filters
    if let Some(min_w) = params.min_width {
        conditions.push(format!("width >= ${param_idx}"));
        bind_values.push(min_w.to_string());
        param_idx += 1;
    }
    if let Some(max_w) = params.max_width {
        conditions.push(format!("width <= ${param_idx}"));
        bind_values.push(max_w.to_string());
        #[allow(unused_assignments)]
        {
            param_idx += 1;
        }
    }

    // Sorting
    let allowed_columns = [
        "created_at",
        "filename",
        "file_size",
        "usage_count",
        "updated_at",
    ];
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "created_at"
    };
    let sort_direction = if sort_order.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    // Pagination
    let per_page = params.per_page.unwrap_or(48).clamp(1, 100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Build query
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // Get total count
    let count_query = format!("SELECT COUNT(*) FROM cms_assets{where_clause}");
    let mut count_q = sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(count_query.as_str()));
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(state.db.pool()).await.unwrap_or(0);

    // Get assets.
    //
    // SAFETY: `sort_column` and `sort_direction` are checked against the
    // `allowed_columns` allow-list and a binary ASC/DESC choice above.
    // Format-string injection through them is therefore impossible. LIMIT
    // and OFFSET are bound as parameters below — never formatted in.
    let query = format!(
        r"
        SELECT id, folder_id, filename, mime_type, file_size, cdn_url,
               width, height, blurhash, thumbnail_url, title, alt_text,
               tags, usage_count, created_at
        FROM cms_assets{}
        ORDER BY {} {}
        LIMIT ${} OFFSET ${}
        ",
        where_clause,
        sort_column,
        sort_direction,
        bind_values.len() + 1,
        bind_values.len() + 2,
    );

    let mut main_q = sqlx::query_as::<_, AssetSummary>(sqlx::AssertSqlSafe(query.as_str()));
    for val in &bind_values {
        main_q = main_q.bind(val);
    }
    main_q = main_q.bind(per_page).bind(offset);

    let assets: Vec<AssetSummary> = main_q
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;

    Ok(Json(PaginatedAssets {
        data: assets,
        meta: PaginationMeta {
            total,
            page,
            per_page,
            total_pages,
            has_more: page < total_pages,
        },
    }))
}

/// GET /cms/assets/recent - Get recently uploaded assets (quick access)
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn recent_assets(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<AssetSummary>>, ApiError> {
    let limit: i64 = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(12)
        .min(50);

    let assets: Vec<AssetSummary> = sqlx::query_as(
        r"
        SELECT id, folder_id, filename, mime_type, file_size, cdn_url,
               width, height, blurhash, thumbnail_url, title, alt_text,
               tags, usage_count, created_at
        FROM cms_assets
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1
        ",
    )
    .bind(limit)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(assets))
}

/// GET /cms/assets/:id - Get single asset with full metadata
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn get_asset(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Asset>, ApiError> {
    let asset: Option<Asset> = sqlx::query_as(
        r"
        SELECT id, folder_id, filename, original_filename, mime_type, file_size,
               file_extension, storage_provider, storage_key, cdn_url,
               width, height, aspect_ratio, blurhash, dominant_color,
               duration_seconds, video_codec, audio_codec, bunny_video_id, bunny_library_id,
               thumbnail_url, variants, title, alt_text, caption,
               description, credits, seo_title, seo_description, tags, usage_count, last_used_at,
               deleted_at, version, created_at, updated_at, created_by, updated_by
        FROM cms_assets
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    match asset {
        Some(a) => Ok(Json(a)),
        None => Err(ApiError::not_found("Asset not found")),
    }
}
