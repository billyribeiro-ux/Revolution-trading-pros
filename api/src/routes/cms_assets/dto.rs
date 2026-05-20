//! CMS Assets — DTO definitions
//!
//! Shared data structures for the Digital Asset Manager API. All
//! types here are `pub` (not `pub(super)`) because the integration
//! tests in `tests/cms_assets_test.rs` import them directly via
//! `revolution_api::routes::cms_assets::{...}`. The module-level
//! `pub use dto::*;` in `mod.rs` keeps the public path stable
//! across the R22-B structural split.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

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
    pub r#type: Option<String>, // image, video, audio, document
    pub search: Option<String>,
    pub tags: Option<String>, // comma-separated
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
