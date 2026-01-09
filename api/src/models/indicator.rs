//! Trading Indicator Management System - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full-service indicator management with:
//! - Multi-platform file downloads (ThinkorSwim, TradingView, etc.)
//! - Instant video playback via Bunny Stream
//! - Secure hash-based download URLs (WordPress-compatible)
//! - User ownership verification
//! - Download analytics

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ═══════════════════════════════════════════════════════════════════════════════════
// INDICATOR (Core Entity)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Indicator {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub price_cents: i32,
    pub is_free: Option<bool>,
    pub sale_price_cents: Option<i32>,
    pub sale_ends_at: Option<DateTime<Utc>>,
    pub logo_url: Option<String>,
    pub card_image_url: Option<String>,
    pub banner_image_url: Option<String>,
    pub featured_video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub compatibility: Option<serde_json::Value>,
    pub supported_platforms: Option<serde_json::Value>,
    pub version: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub last_update: Option<NaiveDate>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_image_url: Option<String>,
    pub status: Option<String>,
    pub is_published: Option<bool>,
    pub published_at: Option<DateTime<Utc>>,
    pub view_count: Option<i32>,
    pub download_count: Option<i32>,
    pub purchase_count: Option<i32>,
    pub creator_id: Option<i32>,
    pub product_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorListItem {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub tagline: Option<String>,
    pub price_cents: i32,
    pub is_free: Option<bool>,
    pub sale_price_cents: Option<i32>,
    pub logo_url: Option<String>,
    pub card_image_url: Option<String>,
    pub status: Option<String>,
    pub is_published: Option<bool>,
    pub view_count: Option<i32>,
    pub download_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INDICATOR FILES (Multi-platform downloads)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorFile {
    pub id: i32,
    pub indicator_id: Uuid,
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
    pub download_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub version: Option<String>,
    pub is_current_version: Option<bool>,
    pub changelog: Option<String>,
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
    pub icon_url: Option<String>,
    pub is_active: Option<bool>,
    pub download_count: Option<i32>,
    pub uploaded_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INDICATOR VIDEOS (Bunny Stream)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorVideo {
    pub id: i32,
    pub indicator_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub bunny_video_guid: String,
    pub bunny_library_id: Option<String>,
    pub embed_url: Option<String>,
    pub play_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_preview: Option<bool>,
    pub encoding_status: Option<String>,
    pub is_published: Option<bool>,
    pub view_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// USER OWNERSHIP
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserIndicatorOwnership {
    pub id: i32,
    pub user_id: i32,
    pub indicator_id: Uuid,
    pub order_id: Option<i32>,
    pub order_item_id: Option<i32>,
    pub price_paid_cents: Option<i32>,
    pub currency: Option<String>,
    pub access_granted_at: Option<DateTime<Utc>>,
    pub access_expires_at: Option<DateTime<Utc>>,
    pub is_lifetime_access: Option<bool>,
    pub source: Option<String>,
    pub granted_by: Option<i32>,
    pub notes: Option<String>,
    pub is_active: Option<bool>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DOWNLOADS (Secure token-based)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorDownload {
    pub id: i32,
    pub user_id: Option<i32>,
    pub indicator_id: Uuid,
    pub file_id: i32,
    pub ownership_id: Option<i32>,
    pub download_token: String,
    pub token_hash: Option<String>,
    pub token_expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub status: Option<String>,
    pub downloaded_at: Option<DateTime<Utc>>,
    pub download_count: Option<i32>,
    pub max_downloads: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADING PLATFORMS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingPlatform {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub display_name: String,
    pub icon_url: Option<String>,
    pub file_extension: Option<String>,
    pub install_instructions: Option<String>,
    pub documentation_url: Option<String>,
    pub is_active: Option<bool>,
    pub display_order: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST/RESPONSE DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize, Validate)]
pub struct CreateIndicatorRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub slug: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: Option<i32>,
    pub is_free: Option<bool>,
    pub logo_url: Option<String>,
    pub card_image_url: Option<String>,
    pub banner_image_url: Option<String>,
    pub features: Option<Vec<String>>,
    pub requirements: Option<Vec<String>>,
    pub supported_platforms: Option<Vec<String>>,
    pub version: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIndicatorRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: Option<i32>,
    pub is_free: Option<bool>,
    pub sale_price_cents: Option<i32>,
    pub logo_url: Option<String>,
    pub card_image_url: Option<String>,
    pub banner_image_url: Option<String>,
    pub featured_video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub compatibility: Option<serde_json::Value>,
    pub supported_platforms: Option<serde_json::Value>,
    pub version: Option<String>,
    pub is_published: Option<bool>,
    pub status: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIndicatorFileRequest {
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
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIndicatorFileRequest {
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
    pub platform_version: Option<String>,
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub is_current_version: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIndicatorVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub bunny_video_guid: String,
    pub bunny_library_id: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_preview: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GrantOwnershipRequest {
    pub user_id: i32,
    pub source: Option<String>,
    pub is_lifetime_access: Option<bool>,
    pub access_expires_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IndicatorQueryParams {
    pub status: Option<String>,
    pub platform: Option<String>,
    pub is_free: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedIndicators {
    pub indicators: Vec<IndicatorListItem>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Serialize)]
pub struct IndicatorWithDetails {
    pub indicator: Indicator,
    pub files: Vec<IndicatorFile>,
    pub videos: Vec<IndicatorVideo>,
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SecureDownloadUrl {
    pub download_url: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub file_name: String,
    pub file_size_bytes: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DownloadAnalytics {
    pub indicator_id: Uuid,
    pub total_downloads: i64,
    pub unique_users: i64,
    pub by_platform: serde_json::Value,
    pub by_date: Vec<DailyDownloads>,
}

#[derive(Debug, Serialize)]
pub struct DailyDownloads {
    pub date: NaiveDate,
    pub count: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LEGACY COMPATIBILITY (Keep old struct for existing code)
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create indicator request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateIndicator {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub is_active: Option<bool>,
    #[validate(length(min = 1, max = 50))]
    pub platform: String,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

/// Update indicator request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateIndicator {
    pub name: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
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
}

/// User indicator ownership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserIndicator {
    pub id: i64,
    pub user_id: i64,
    pub indicator_id: i64,
    pub purchased_at: NaiveDateTime,
    pub license_key: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Indicator with user ownership status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorWithOwnership {
    #[serde(flatten)]
    pub indicator: Indicator,
    pub owned: bool,
    pub purchased_at: Option<NaiveDateTime>,
    pub license_key: Option<String>,
}
