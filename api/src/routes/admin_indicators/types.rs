//! Public DTOs for the admin-indicators routes.
//!
//! These types are re-exported by `routes::admin_indicators::*` and are
//! relied on by `tests/admin_indicators_test.rs` (which pins the wire
//! shapes, including `price_cents: i64`). Do not change field types or
//! visibility without updating that contract test.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct IndicatorQueryParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
}

/// Indicator row returned to admin UI. Money is integer cents.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: i64,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIndicatorRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: Option<i64>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIndicatorRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: Option<i64>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorFileRow {
    pub id: i32,
    pub indicator_id: i64,
    pub file_name: String,
    pub original_name: Option<String>,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    pub checksum_sha256: Option<String>,
    pub platform: String,
    pub platform_version: Option<String>,
    pub storage_provider: Option<String>,
    pub storage_bucket: Option<String>,
    pub storage_key: Option<String>,
    pub cdn_url: Option<String>,
    pub version: Option<String>,
    pub is_current_version: Option<bool>,
    pub changelog: Option<String>,
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
    pub download_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFileRequest {
    pub file_name: String,
    pub original_name: Option<String>,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    pub platform: String,
    pub platform_version: Option<String>,
    pub storage_key: Option<String>,
    pub cdn_url: Option<String>,
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFileRequest {
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
    pub platform_version: Option<String>,
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub is_current_version: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorVideoRow {
    pub id: i32,
    pub indicator_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<String>,
    pub embed_url: Option<String>,
    pub play_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_preview: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<String>,
    pub embed_url: Option<String>,
    pub play_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_preview: Option<bool>,
}
