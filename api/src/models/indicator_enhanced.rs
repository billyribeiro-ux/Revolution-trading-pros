//! Enhanced Indicator Models - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Complete indicator management: platforms, files, videos, docs, TradingView access

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ═══════════════════════════════════════════════════════════════════════════════════
// PLATFORM MODELS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorPlatform {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub display_name: String,
    pub icon_url: Option<String>,
    pub file_extension: Option<String>,
    pub installation_instructions: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INDICATOR MODELS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorEnhanced {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub short_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub preview_video_url: Option<String>,
    pub preview_bunny_guid: Option<String>,
    pub category: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub version: String,
    pub version_notes: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_free: bool,
    pub required_plan_id: Option<i64>,
    pub price_cents: Option<i32>,
    pub total_downloads: i32,
    pub supported_platforms: Option<serde_json::Value>,
    pub has_tradingview_access: bool,
    pub tradingview_invite_only: bool,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorVideo {
    pub id: i64,
    pub indicator_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub video_platform: String,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration_seconds: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub video_type: String,
    pub sort_order: i32,
    pub is_preview: bool,
    pub is_published: bool,
    pub view_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorPlatformFile {
    pub id: i64,
    pub indicator_id: i64,
    pub platform_id: i64,
    pub file_url: String,
    pub file_name: String,
    pub file_size_bytes: Option<i64>,
    pub version: String,
    pub version_notes: Option<String>,
    pub installation_notes: Option<String>,
    pub is_latest: bool,
    pub download_count: i32,
    pub checksum_sha256: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorDocumentation {
    pub id: i64,
    pub indicator_id: i64,
    pub title: String,
    pub doc_type: String,
    pub content_html: Option<String>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub sort_order: i32,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorTradingViewAccess {
    pub id: i64,
    pub indicator_id: i64,
    pub user_id: i64,
    pub tradingview_username: String,
    pub access_type: String,
    pub granted_at: DateTime<Utc>,
    pub granted_by: Option<i64>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub notes: Option<String>,
    pub synced_to_tradingview: bool,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserIndicatorAccess {
    pub id: i64,
    pub user_id: i64,
    pub indicator_id: i64,
    pub granted_at: DateTime<Utc>,
    pub access_source: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub download_count: i32,
    pub last_download_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorDownloadLog {
    pub id: i64,
    pub user_id: i64,
    pub indicator_id: i64,
    pub platform_file_id: i64,
    pub platform_id: i64,
    pub downloaded_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndicatorRequest {
    pub name: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub short_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub preview_video_url: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub version: Option<String>,
    pub version_notes: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_free: Option<bool>,
    pub required_plan_id: Option<i64>,
    pub price_cents: Option<i32>,
    pub has_tradingview_access: Option<bool>,
    pub tradingview_invite_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIndicatorRequest {
    pub name: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub short_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub preview_video_url: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub version: Option<String>,
    pub version_notes: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_free: Option<bool>,
    pub required_plan_id: Option<i64>,
    pub price_cents: Option<i32>,
    pub has_tradingview_access: Option<bool>,
    pub tradingview_invite_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndicatorVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub video_type: Option<String>,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIndicatorVideoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub video_type: Option<String>,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlatformFileRequest {
    pub platform_id: i64,
    pub file_url: String,
    pub file_name: String,
    pub file_size_bytes: Option<i64>,
    pub version: Option<String>,
    pub version_notes: Option<String>,
    pub installation_notes: Option<String>,
    pub is_latest: Option<bool>,
    pub checksum_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlatformFileRequest {
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub version: Option<String>,
    pub version_notes: Option<String>,
    pub installation_notes: Option<String>,
    pub is_latest: Option<bool>,
    pub checksum_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentationRequest {
    pub title: String,
    pub doc_type: Option<String>,
    pub content_html: Option<String>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentationRequest {
    pub title: Option<String>,
    pub doc_type: Option<String>,
    pub content_html: Option<String>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantTradingViewAccessRequest {
    pub user_id: i64,
    pub tradingview_username: String,
    pub access_type: Option<String>,
    pub expires_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkGrantTradingViewAccessRequest {
    pub accesses: Vec<TradingViewAccessInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingViewAccessInput {
    pub user_id: i64,
    pub tradingview_username: String,
    pub access_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeTradingViewAccessRequest {
    pub user_id: Option<i64>,
    pub tradingview_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlatformRequest {
    pub name: String,
    pub display_name: String,
    pub icon_url: Option<String>,
    pub file_extension: Option<String>,
    pub installation_instructions: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlatformRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub icon_url: Option<String>,
    pub file_extension: Option<String>,
    pub installation_instructions: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUploadFilesRequest {
    pub files: Vec<BulkFileInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkFileInput {
    pub platform_id: i64,
    pub file_url: String,
    pub file_name: String,
    pub file_size_bytes: Option<i64>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderItemsRequest {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderItem {
    pub id: i64,
    pub sort_order: i32,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RESPONSE DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub short_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub preview_video_url: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub version: String,
    pub version_notes: Option<String>,
    pub release_date: Option<String>,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_free: bool,
    pub price_cents: Option<i32>,
    pub total_downloads: i32,
    pub supported_platforms: Vec<PlatformSummary>,
    pub has_tradingview_access: bool,
    pub tradingview_invite_only: bool,
    pub videos: Option<Vec<IndicatorVideoResponse>>,
    pub platform_files: Option<Vec<PlatformFileResponse>>,
    pub documentation: Option<Vec<DocumentationResponse>>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSummary {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub display_name: String,
    pub icon_url: Option<String>,
    pub file_extension: Option<String>,
    pub installation_instructions: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorVideoResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub embed_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub formatted_duration: Option<String>,
    pub video_type: String,
    pub sort_order: i32,
    pub is_preview: bool,
    pub is_published: bool,
    pub view_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFileResponse {
    pub id: i64,
    pub platform: PlatformSummary,
    pub file_url: String,
    pub file_name: String,
    pub file_size_bytes: Option<i64>,
    pub formatted_size: String,
    pub version: String,
    pub version_notes: Option<String>,
    pub installation_notes: Option<String>,
    pub is_latest: bool,
    pub download_count: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationResponse {
    pub id: i64,
    pub title: String,
    pub doc_type: String,
    pub content_html: Option<String>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub sort_order: i32,
    pub is_published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingViewAccessResponse {
    pub id: i64,
    pub user_id: i64,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
    pub tradingview_username: String,
    pub access_type: String,
    pub granted_at: String,
    pub expires_at: Option<String>,
    pub is_active: bool,
    pub is_expired: bool,
    pub synced_to_tradingview: bool,
    pub last_sync_at: Option<String>,
    pub sync_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIndicatorAccessResponse {
    pub indicator_id: i64,
    pub indicator_name: String,
    pub indicator_slug: String,
    pub thumbnail_url: Option<String>,
    pub granted_at: String,
    pub expires_at: Option<String>,
    pub is_active: bool,
    pub download_count: i32,
    pub available_platforms: Vec<PlatformSummary>,
    pub has_tradingview_access: bool,
    pub tradingview_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLogResponse {
    pub id: i64,
    pub user_id: i64,
    pub user_email: Option<String>,
    pub indicator_name: String,
    pub platform_name: String,
    pub file_name: String,
    pub downloaded_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorStatsResponse {
    pub total_indicators: i64,
    pub published_indicators: i64,
    pub total_downloads: i64,
    pub total_tradingview_users: i64,
    pub downloads_by_platform: Vec<PlatformDownloadStats>,
    pub top_indicators: Vec<IndicatorDownloadStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDownloadStats {
    pub platform_name: String,
    pub download_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorDownloadStats {
    pub indicator_id: i64,
    pub indicator_name: String,
    pub download_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct IndicatorListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub category: Option<String>,
    pub platform_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub has_tradingview: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TradingViewAccessQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub is_active: Option<bool>,
    pub is_synced: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadLogQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub user_id: Option<i64>,
    pub platform_id: Option<i64>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn format_duration_seconds(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

pub fn format_file_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub fn get_indicator_embed_url(
    video_url: &Option<String>,
    bunny_guid: &Option<String>,
    bunny_library: Option<i64>,
) -> Option<String> {
    if let (Some(guid), Some(lib_id)) = (bunny_guid, bunny_library) {
        return Some(format!(
            "https://iframe.mediadelivery.net/embed/{}/{}",
            lib_id, guid
        ));
    }
    video_url.clone()
}

/// Generate presigned download URL for platform files
pub fn generate_download_url(file_url: &str, file_name: &str) -> String {
    // In production, this would generate a presigned URL from R2/S3
    // For now, return the direct URL with download disposition hint
    if file_url.contains('?') {
        format!(
            "{}&response-content-disposition=attachment%3B%20filename%3D{}",
            file_url, file_name
        )
    } else {
        format!(
            "{}?response-content-disposition=attachment%3B%20filename%3D{}",
            file_url, file_name
        )
    }
}

/// Validate TradingView username format
pub fn validate_tradingview_username(username: &str) -> bool {
    // TradingView usernames: 3-20 chars, alphanumeric with underscores, starts with letter
    let len = username.len();
    if !(3..=20).contains(&len) {
        return false;
    }

    let chars: Vec<char> = username.chars().collect();
    if !chars[0].is_ascii_alphabetic() {
        return false;
    }

    chars.iter().all(|c| c.is_ascii_alphanumeric() || *c == '_')
}

/// Parse supported platforms from JSON array
pub fn parse_supported_platforms(json: &Option<serde_json::Value>) -> Vec<i64> {
    match json {
        Some(serde_json::Value::Array(arr)) => arr.iter().filter_map(|v| v.as_i64()).collect(),
        _ => vec![],
    }
}

/// Parse tags from JSON array
pub fn parse_tags(json: &Option<serde_json::Value>) -> Vec<String> {
    match json {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => vec![],
    }
}
