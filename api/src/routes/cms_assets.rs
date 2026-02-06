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

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState, services::cms_content};

// ═══════════════════════════════════════════════════════════════════════════════════════
// DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Asset with full metadata for the DAM
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub file_extension: String,
    pub storage_provider: String,
    pub storage_key: String,
    pub cdn_url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub aspect_ratio: Option<Decimal>,
    pub blurhash: Option<String>,
    pub dominant_color: Option<String>,
    pub duration_seconds: Option<i32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bunny_video_id: Option<String>,
    pub bunny_library_id: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub variants: Option<JsonValue>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub credits: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub usage_count: i32,
    pub last_used_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Lightweight asset for grid listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AssetSummary {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,
    pub filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub cdn_url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub blurhash: Option<String>,
    pub thumbnail_url: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Asset folder for organization
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AssetFolder {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub depth: i32,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub asset_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Content usage reference
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AssetUsage {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub content_type: String,
    pub content_id: Uuid,
    pub content_title: Option<String>,
    pub content_slug: Option<String>,
    pub field_name: String,
    pub created_at: DateTime<Utc>,
}

/// Asset statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStats {
    pub total_assets: i64,
    pub total_size: i64,
    pub total_size_formatted: String,
    pub by_type: TypeStats,
    pub recent_uploads: i64,
    pub unused_assets: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeStats {
    pub images: i64,
    pub videos: i64,
    pub audio: i64,
    pub documents: i64,
    pub other: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REQUEST/RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct AssetListQuery {
    pub folder_id: Option<Uuid>,
    pub r#type: Option<String>,  // image, video, audio, document
    pub search: Option<String>,
    pub tags: Option<String>,    // comma-separated
    pub mime_type: Option<String>,
    pub min_width: Option<i32>,
    pub max_width: Option<i32>,
    pub sort_by: Option<String>, // created_at, filename, file_size, usage_count
    pub sort_order: Option<String>, // asc, desc
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub include_deleted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFolderRequest {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssetRequest {
    pub folder_id: Option<Uuid>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub credits: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct BulkMoveRequest {
    pub asset_ids: Vec<Uuid>,
    pub target_folder_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
    pub asset_ids: Vec<Uuid>,
    pub permanent: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BulkTagRequest {
    pub asset_ids: Vec<Uuid>,
    pub tags_to_add: Option<Vec<String>>,
    pub tags_to_remove: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UploadConfirmRequest {
    pub file_key: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub folder_id: Option<Uuid>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub blurhash: Option<String>,
    pub dominant_color: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceAssetRequest {
    pub file_key: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedAssets {
    pub data: Vec<AssetSummary>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
    pub has_more: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// FOLDER ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// GET /cms/assets/folders - List all folders with hierarchy
#[tracing::instrument(skip(state, _admin))]
pub async fn list_folders(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Vec<AssetFolder>>, ApiError> {
    let folders: Vec<AssetFolder> = sqlx::query_as(
        r#"
        WITH folder_counts AS (
            SELECT folder_id, COUNT(*) as cnt
            FROM cms_assets
            WHERE deleted_at IS NULL
            GROUP BY folder_id
        )
        SELECT
            f.id, f.name, f.slug, f.parent_id, f.path, f.depth,
            f.color, f.icon,
            COALESCE(fc.cnt, 0)::int as asset_count,
            f.created_at, f.updated_at, f.created_by
        FROM cms_asset_folders f
        LEFT JOIN folder_counts fc ON fc.folder_id = f.id
        ORDER BY f.path, f.name
        "#,
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(folders))
}

/// POST /cms/assets/folders - Create a new folder
#[tracing::instrument(skip(state, admin))]
pub async fn create_folder(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<AssetFolder>, ApiError> {
    // Generate slug from name
    let slug = slugify(&payload.name);

    // Calculate path and depth
    let (path, depth) = if let Some(parent_id) = payload.parent_id {
        let parent: Option<(String, i32)> = sqlx::query_as(
            "SELECT path, depth FROM cms_asset_folders WHERE id = $1"
        )
        .bind(parent_id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        match parent {
            Some((parent_path, parent_depth)) => {
                (format!("{}/{}", parent_path, slug), parent_depth + 1)
            }
            None => return Err(ApiError::not_found("Parent folder not found")),
        }
    } else {
        (format!("/{}", slug), 0)
    };

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;

    let folder: AssetFolder = sqlx::query_as(
        r#"
        INSERT INTO cms_asset_folders (
            id, name, slug, parent_id, path, depth, color, icon,
            created_at, updated_at, created_by
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8,
            NOW(), NOW(), $9
        )
        RETURNING id, name, slug, parent_id, path, depth, color, icon,
                  0 as asset_count, created_at, updated_at, created_by
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&payload.name)
    .bind(&slug)
    .bind(payload.parent_id)
    .bind(&path)
    .bind(depth)
    .bind(&payload.color)
    .bind(&payload.icon)
    .bind(cms_user_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(folder))
}

/// PUT /cms/assets/folders/:id - Update folder
#[tracing::instrument(skip(state, admin))]
pub async fn update_folder(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFolderRequest>,
) -> Result<Json<AssetFolder>, ApiError> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if payload.name.is_some() {
        updates.push(format!("name = ${}", param_idx));
        param_idx += 1;
        updates.push(format!("slug = ${}", param_idx));
        param_idx += 1;
    }
    if payload.color.is_some() {
        updates.push(format!("color = ${}", param_idx));
        param_idx += 1;
    }
    if payload.icon.is_some() {
        updates.push(format!("icon = ${}", param_idx));
        param_idx += 1;
    }
    updates.push(format!("updated_at = NOW()"));
    updates.push(format!("updated_by = ${}", param_idx));
    param_idx += 1;

    if updates.len() <= 2 {
        return Err(ApiError::validation_error("No fields to update"));
    }

    let query = format!(
        r#"
        UPDATE cms_asset_folders
        SET {}
        WHERE id = ${}
        RETURNING id, name, slug, parent_id, path, depth, color, icon,
                  0 as asset_count, created_at, updated_at, created_by
        "#,
        updates.join(", "),
        param_idx
    );

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;
    let mut q = sqlx::query_as::<_, AssetFolder>(&query);

    if let Some(ref name) = payload.name {
        q = q.bind(name);
        q = q.bind(slugify(name));
    }
    if let Some(ref color) = payload.color {
        q = q.bind(color);
    }
    if let Some(ref icon) = payload.icon {
        q = q.bind(icon);
    }
    q = q.bind(cms_user_id);
    q = q.bind(id);

    let folder = q
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(folder))
}

/// DELETE /cms/assets/folders/:id - Delete folder (moves assets to parent)
#[tracing::instrument(skip(state, _admin))]
pub async fn delete_folder(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<JsonValue>, ApiError> {
    let move_to: Option<Uuid> = params.get("move_to").and_then(|s| Uuid::parse_str(s).ok());

    // Move assets to target folder or root
    sqlx::query("UPDATE cms_assets SET folder_id = $1 WHERE folder_id = $2")
        .bind(move_to)
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Move child folders to target or root
    sqlx::query("UPDATE cms_asset_folders SET parent_id = $1 WHERE parent_id = $2")
        .bind(move_to)
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Delete the folder
    sqlx::query("DELETE FROM cms_asset_folders WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "Folder deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSET LISTING & SEARCH
// ═══════════════════════════════════════════════════════════════════════════════════════

/// GET /cms/assets - List assets with search, filter, pagination
#[tracing::instrument(skip(state, _admin))]
pub async fn list_assets(
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
        conditions.push(format!("folder_id = ${}", param_idx));
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
        conditions.push(format!("mime_type LIKE ${}", param_idx));
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
            let pattern = format!("%{}%", sanitized);
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
            conditions.push(format!("tags && ${}", param_idx));
            bind_values.push(format!("{{{}}}", tags.join(",")));
            param_idx += 1;
        }
    }

    // Dimension filters
    if let Some(min_w) = params.min_width {
        conditions.push(format!("width >= ${}", param_idx));
        bind_values.push(min_w.to_string());
        param_idx += 1;
    }
    if let Some(max_w) = params.max_width {
        conditions.push(format!("width <= ${}", param_idx));
        bind_values.push(max_w.to_string());
        #[allow(unused_assignments)]
        { param_idx += 1; }
    }

    // Sorting
    let allowed_columns = ["created_at", "filename", "file_size", "usage_count", "updated_at"];
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = params.sort_order.as_deref().unwrap_or("desc");

    let sort_column = if allowed_columns.contains(&sort_by) { sort_by } else { "created_at" };
    let sort_direction = if sort_order.eq_ignore_ascii_case("asc") { "ASC" } else { "DESC" };

    // Pagination
    let per_page = params.per_page.unwrap_or(48).min(100).max(1);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Build query
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // Get total count
    let count_query = format!("SELECT COUNT(*) FROM cms_assets{}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // Get assets
    let query = format!(
        r#"
        SELECT id, folder_id, filename, mime_type, file_size, cdn_url,
               width, height, blurhash, thumbnail_url, title, alt_text,
               tags, usage_count, created_at
        FROM cms_assets{}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, sort_column, sort_direction, per_page, offset
    );

    let mut main_q = sqlx::query_as::<_, AssetSummary>(&query);
    for val in &bind_values {
        main_q = main_q.bind(val);
    }

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
pub async fn recent_assets(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<AssetSummary>>, ApiError> {
    let limit: i64 = params.get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(12)
        .min(50);

    let assets: Vec<AssetSummary> = sqlx::query_as(
        r#"
        SELECT id, folder_id, filename, mime_type, file_size, cdn_url,
               width, height, blurhash, thumbnail_url, title, alt_text,
               tags, usage_count, created_at
        FROM cms_assets
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(assets))
}

/// GET /cms/assets/:id - Get single asset with full metadata
#[tracing::instrument(skip(state, _admin))]
pub async fn get_asset(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Asset>, ApiError> {
    let asset: Option<Asset> = sqlx::query_as(
        r#"
        SELECT id, folder_id, filename, original_filename, mime_type, file_size,
               file_extension, storage_provider, storage_key, cdn_url,
               width, height, aspect_ratio, blurhash, dominant_color,
               duration_seconds, video_codec, audio_codec, bunny_video_id, bunny_library_id,
               thumbnail_url, variants, title, alt_text, caption,
               description, credits, seo_title, seo_description, tags, usage_count, last_used_at,
               deleted_at, version, created_at, updated_at, created_by, updated_by
        FROM cms_assets
        WHERE id = $1
        "#,
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

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSET CRUD
// ═══════════════════════════════════════════════════════════════════════════════════════

/// POST /cms/assets/upload - Confirm upload and create asset record
#[tracing::instrument(skip(state, admin))]
pub async fn create_asset(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<UploadConfirmRequest>,
) -> Result<Json<Asset>, ApiError> {
    // Validate file key
    if payload.file_key.contains("..") || payload.file_key.contains("//") {
        return Err(ApiError::validation_error("Invalid file key"));
    }

    let extension = payload.original_filename
        .rsplit('.')
        .next()
        .unwrap_or("bin")
        .to_lowercase();

    let public_url = format!("{}/{}", state.config.r2_public_url, payload.file_key);

    // Calculate aspect ratio
    let aspect_ratio = match (payload.width, payload.height) {
        (Some(w), Some(h)) if h > 0 => Some(Decimal::from(w) / Decimal::from(h)),
        _ => None,
    };

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;

    let asset: Asset = sqlx::query_as(
        r#"
        INSERT INTO cms_assets (
            id, folder_id, filename, original_filename, mime_type, file_size,
            file_extension, storage_provider, storage_key, cdn_url,
            width, height, aspect_ratio, blurhash, dominant_color,
            title, alt_text, tags, version,
            created_at, updated_at, created_by
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, 'r2', $8, $9,
            $10, $11, $12, $13, $14,
            $15, $16, $17, 1,
            NOW(), NOW(), $18
        )
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(payload.folder_id)
    .bind(&payload.file_key.split('/').last().unwrap_or(&payload.file_key))
    .bind(&payload.original_filename)
    .bind(&payload.mime_type)
    .bind(payload.file_size)
    .bind(&extension)
    .bind(&payload.file_key)
    .bind(&public_url)
    .bind(payload.width)
    .bind(payload.height)
    .bind(aspect_ratio)
    .bind(&payload.blurhash)
    .bind(&payload.dominant_color)
    .bind(&payload.title)
    .bind(&payload.alt_text)
    .bind(&payload.tags)
    .bind(cms_user_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(asset))
}

/// PUT /cms/assets/:id - Update asset metadata
#[tracing::instrument(skip(state, admin))]
pub async fn update_asset(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, ApiError> {
    // Build dynamic update
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.folder_id.is_some() {
        updates.push(format!("folder_id = ${}", param_count));
        param_count += 1;
    }
    if payload.title.is_some() {
        updates.push(format!("title = ${}", param_count));
        param_count += 1;
    }
    if payload.alt_text.is_some() {
        updates.push(format!("alt_text = ${}", param_count));
        param_count += 1;
    }
    if payload.caption.is_some() {
        updates.push(format!("caption = ${}", param_count));
        param_count += 1;
    }
    if payload.description.is_some() {
        updates.push(format!("description = ${}", param_count));
        param_count += 1;
    }
    if payload.credits.is_some() {
        updates.push(format!("credits = ${}", param_count));
        param_count += 1;
    }
    if payload.seo_title.is_some() {
        updates.push(format!("seo_title = ${}", param_count));
        param_count += 1;
    }
    if payload.seo_description.is_some() {
        updates.push(format!("seo_description = ${}", param_count));
        param_count += 1;
    }
    if payload.tags.is_some() {
        updates.push(format!("tags = ${}", param_count));
        param_count += 1;
    }

    if updates.is_empty() {
        return Err(ApiError::validation_error("No fields to update"));
    }

    updates.push("updated_at = NOW()".to_string());
    updates.push(format!("updated_by = ${}", param_count));
    param_count += 1;
    updates.push("version = version + 1".to_string());

    let query = format!(
        "UPDATE cms_assets SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;
    let mut q = sqlx::query_as::<_, Asset>(&query);

    if let Some(folder_id) = payload.folder_id {
        q = q.bind(folder_id);
    }
    if let Some(ref title) = payload.title {
        q = q.bind(title);
    }
    if let Some(ref alt_text) = payload.alt_text {
        q = q.bind(alt_text);
    }
    if let Some(ref caption) = payload.caption {
        q = q.bind(caption);
    }
    if let Some(ref description) = payload.description {
        q = q.bind(description);
    }
    if let Some(ref credits) = payload.credits {
        q = q.bind(credits);
    }
    if let Some(ref seo_title) = payload.seo_title {
        q = q.bind(seo_title);
    }
    if let Some(ref seo_description) = payload.seo_description {
        q = q.bind(seo_description);
    }
    if let Some(ref tags) = payload.tags {
        q = q.bind(tags);
    }
    q = q.bind(cms_user_id);
    q = q.bind(id);

    let asset = q
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(asset))
}

/// DELETE /cms/assets/:id - Soft delete asset
#[tracing::instrument(skip(state, _admin))]
pub async fn delete_asset(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<JsonValue>, ApiError> {
    let permanent = params.get("permanent").map(|v| v == "true").unwrap_or(false);

    if permanent {
        // Get storage key first
        let asset: Option<(String,)> = sqlx::query_as(
            "SELECT storage_key FROM cms_assets WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if let Some((storage_key,)) = asset {
            // Delete from storage
            let storage = &state.services.storage;
            if let Err(e) = storage.delete(&storage_key).await {
                tracing::warn!("Failed to delete from storage: {}", e);
            }
        }

        // Hard delete
        sqlx::query("DELETE FROM cms_assets WHERE id = $1")
            .bind(id)
            .execute(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    } else {
        // Soft delete
        sqlx::query("UPDATE cms_assets SET deleted_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    }

    Ok(Json(json!({
        "success": true,
        "message": "Asset deleted successfully"
    })))
}

/// POST /cms/assets/:id/restore - Restore soft-deleted asset
#[tracing::instrument(skip(state, _admin))]
pub async fn restore_asset(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JsonValue>, ApiError> {
    sqlx::query("UPDATE cms_assets SET deleted_at = NULL WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "Asset restored successfully"
    })))
}

/// POST /cms/assets/:id/replace - Replace asset file (keeps metadata)
#[tracing::instrument(skip(state, admin))]
pub async fn replace_asset(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<ReplaceAssetRequest>,
) -> Result<Json<Asset>, ApiError> {
    // Get old storage key
    let old_key: Option<(String,)> = sqlx::query_as(
        "SELECT storage_key FROM cms_assets WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let public_url = format!("{}/{}", state.config.r2_public_url, payload.file_key);
    let extension = payload.original_filename
        .rsplit('.')
        .next()
        .unwrap_or("bin")
        .to_lowercase();

    let aspect_ratio = match (payload.width, payload.height) {
        (Some(w), Some(h)) if h > 0 => Some(Decimal::from(w) / Decimal::from(h)),
        _ => None,
    };

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;

    // Update asset
    let asset: Asset = sqlx::query_as(
        r#"
        UPDATE cms_assets SET
            original_filename = $1, mime_type = $2, file_size = $3,
            file_extension = $4, storage_key = $5, cdn_url = $6,
            width = $7, height = $8, aspect_ratio = $9,
            version = version + 1, updated_at = NOW(), updated_by = $10
        WHERE id = $11
        RETURNING *
        "#,
    )
    .bind(&payload.original_filename)
    .bind(&payload.mime_type)
    .bind(payload.file_size)
    .bind(&extension)
    .bind(&payload.file_key)
    .bind(&public_url)
    .bind(payload.width)
    .bind(payload.height)
    .bind(aspect_ratio)
    .bind(cms_user_id)
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Delete old file from storage
    if let Some((old_storage_key,)) = old_key {
        let storage = &state.services.storage;
        if let Err(e) = storage.delete(&old_storage_key).await {
            tracing::warn!("Failed to delete old file: {}", e);
        }
    }

    Ok(Json(asset))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// BULK OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// POST /cms/assets/bulk/move - Move multiple assets to folder
#[tracing::instrument(skip(state, _admin))]
pub async fn bulk_move(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkMoveRequest>,
) -> Result<Json<JsonValue>, ApiError> {
    if payload.asset_ids.is_empty() {
        return Err(ApiError::validation_error("No assets specified"));
    }
    if payload.asset_ids.len() > 100 {
        return Err(ApiError::validation_error("Maximum 100 assets per operation"));
    }

    let mut moved = 0;
    for asset_id in &payload.asset_ids {
        let result = sqlx::query("UPDATE cms_assets SET folder_id = $1, updated_at = NOW() WHERE id = $2")
            .bind(payload.target_folder_id)
            .bind(asset_id)
            .execute(state.db.pool())
            .await;
        if result.is_ok() {
            moved += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "moved": moved,
        "message": format!("{} asset(s) moved successfully", moved)
    })))
}

/// POST /cms/assets/bulk/delete - Delete multiple assets
#[tracing::instrument(skip(state, _admin))]
pub async fn bulk_delete(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkDeleteRequest>,
) -> Result<Json<JsonValue>, ApiError> {
    if payload.asset_ids.is_empty() {
        return Err(ApiError::validation_error("No assets specified"));
    }
    if payload.asset_ids.len() > 100 {
        return Err(ApiError::validation_error("Maximum 100 assets per operation"));
    }

    let permanent = payload.permanent.unwrap_or(false);
    let mut deleted = 0;

    for asset_id in &payload.asset_ids {
        if permanent {
            // Get storage key
            let key: Option<(String,)> = sqlx::query_as(
                "SELECT storage_key FROM cms_assets WHERE id = $1"
            )
            .bind(asset_id)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

            if let Some((storage_key,)) = key {
                let storage = &state.services.storage;
                let _ = storage.delete(&storage_key).await;
            }

            if sqlx::query("DELETE FROM cms_assets WHERE id = $1")
                .bind(asset_id)
                .execute(state.db.pool())
                .await
                .is_ok()
            {
                deleted += 1;
            }
        } else {
            if sqlx::query("UPDATE cms_assets SET deleted_at = NOW() WHERE id = $1")
                .bind(asset_id)
                .execute(state.db.pool())
                .await
                .is_ok()
            {
                deleted += 1;
            }
        }
    }

    Ok(Json(json!({
        "success": true,
        "deleted": deleted,
        "message": format!("{} asset(s) deleted successfully", deleted)
    })))
}

/// POST /cms/assets/bulk/tag - Add/remove tags from multiple assets
#[tracing::instrument(skip(state, _admin))]
pub async fn bulk_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkTagRequest>,
) -> Result<Json<JsonValue>, ApiError> {
    if payload.asset_ids.is_empty() {
        return Err(ApiError::validation_error("No assets specified"));
    }
    if payload.asset_ids.len() > 100 {
        return Err(ApiError::validation_error("Maximum 100 assets per operation"));
    }

    let mut updated = 0;

    for asset_id in &payload.asset_ids {
        // Get current tags
        let current: Option<(Option<Vec<String>>,)> = sqlx::query_as(
            "SELECT tags FROM cms_assets WHERE id = $1"
        )
        .bind(asset_id)
        .fetch_optional(state.db.pool())
        .await
        .ok()
        .flatten();

        let mut tags: Vec<String> = current
            .and_then(|(t,)| t)
            .unwrap_or_default();

        // Add new tags
        if let Some(ref to_add) = payload.tags_to_add {
            for tag in to_add {
                if !tags.contains(tag) {
                    tags.push(tag.clone());
                }
            }
        }

        // Remove tags
        if let Some(ref to_remove) = payload.tags_to_remove {
            tags.retain(|t| !to_remove.contains(t));
        }

        // Update
        if sqlx::query("UPDATE cms_assets SET tags = $1, updated_at = NOW() WHERE id = $2")
            .bind(&tags)
            .bind(asset_id)
            .execute(state.db.pool())
            .await
            .is_ok()
        {
            updated += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "updated": updated,
        "message": format!("{} asset(s) updated successfully", updated)
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// USAGE TRACKING
// ═══════════════════════════════════════════════════════════════════════════════════════

/// GET /cms/assets/:id/usage - Get content using this asset
#[tracing::instrument(skip(state, _admin))]
pub async fn get_asset_usage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<AssetUsage>>, ApiError> {
    let usage: Vec<AssetUsage> = sqlx::query_as(
        r#"
        SELECT u.id, u.asset_id, u.content_type, u.content_id,
               c.title as content_title, c.slug as content_slug,
               u.field_name, u.created_at
        FROM cms_asset_usage u
        LEFT JOIN cms_content c ON c.id = u.content_id
        WHERE u.asset_id = $1
        ORDER BY u.created_at DESC
        "#,
    )
    .bind(id)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(usage))
}

/// POST /cms/assets/:id/track-usage - Track asset usage in content
#[tracing::instrument(skip(state, _admin))]
pub async fn track_usage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<JsonValue>, ApiError> {
    let content_type = payload.get("content_type").and_then(|v| v.as_str()).unwrap_or("");
    let content_id = payload.get("content_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok());
    let field_name = payload.get("field_name").and_then(|v| v.as_str()).unwrap_or("unknown");

    if let Some(content_id) = content_id {
        // Upsert usage record
        sqlx::query(
            r#"
            INSERT INTO cms_asset_usage (id, asset_id, content_type, content_id, field_name, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            ON CONFLICT (asset_id, content_id, field_name) DO UPDATE SET created_at = NOW()
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(id)
        .bind(content_type)
        .bind(content_id)
        .bind(field_name)
        .execute(state.db.pool())
        .await
        .ok();

        // Update usage count
        sqlx::query(
            "UPDATE cms_assets SET usage_count = usage_count + 1, last_used_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(state.db.pool())
        .await
        .ok();
    }

    Ok(Json(json!({ "success": true })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// GET /cms/assets/stats - Get asset library statistics
#[tracing::instrument(skip(state, _admin))]
pub async fn get_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<AssetStats>, ApiError> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cms_assets WHERE deleted_at IS NULL")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    let total_size: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(file_size), 0) FROM cms_assets WHERE deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let images: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'image/%' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let videos: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'video/%' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let audio: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'audio/%' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let documents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'application/%' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let recent: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE created_at > NOW() - INTERVAL '7 days' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let unused: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE usage_count = 0 AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    Ok(Json(AssetStats {
        total_assets: total,
        total_size,
        total_size_formatted: format_bytes(total_size),
        by_type: TypeStats {
            images,
            videos,
            audio,
            documents,
            other: total - images - videos - audio - documents,
        },
        recent_uploads: recent,
        unused_assets: unused,
    }))
}

/// GET /cms/assets/tags - Get all unique tags
#[tracing::instrument(skip(state, _admin))]
pub async fn get_all_tags(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Vec<String>>, ApiError> {
    let tags: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT DISTINCT unnest(tags) as tag
        FROM cms_assets
        WHERE deleted_at IS NULL AND tags IS NOT NULL
        ORDER BY tag
        "#,
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(tags.into_iter().map(|(t,)| t).collect()))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Get CMS user UUID from admin user
async fn get_cms_user_id(
    pool: &sqlx::PgPool,
    admin: &AdminUser,
) -> Option<Uuid> {
    let user_id = admin.0.id;
    cms_content::get_cms_user_by_user_id(pool, user_id)
        .await
        .ok()
        .flatten()
        .map(|u| u.id)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Build the CMS Assets router - DAM API
pub fn router() -> Router<AppState> {
    Router::new()
        // Asset listing & search
        .route("/", get(list_assets))
        .route("/recent", get(recent_assets))
        .route("/stats", get(get_stats))
        .route("/tags", get(get_all_tags))
        // Folders
        .route("/folders", get(list_folders).post(create_folder))
        .route("/folders/:id", put(update_folder).delete(delete_folder))
        // Single asset operations
        .route("/upload", post(create_asset))
        .route("/:id", get(get_asset).put(update_asset).delete(delete_asset))
        .route("/:id/restore", post(restore_asset))
        .route("/:id/replace", post(replace_asset))
        .route("/:id/usage", get(get_asset_usage))
        .route("/:id/track-usage", post(track_usage))
        // Bulk operations
        .route("/bulk/move", post(bulk_move))
        .route("/bulk/delete", post(bulk_delete))
        .route("/bulk/tag", post(bulk_tag))
}
