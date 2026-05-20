//! Media DTOs — shared types for the media routes.
//!
//! R27-B split: this file holds every public struct that lived at the
//! top of the old monolithic `media.rs`. Integration tests reach into
//! `routes::media::{Media, MediaQuery, ...}` directly, so the parent
//! `mod.rs` re-exports each of these.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Media {
    pub id: i64,
    pub filename: String,
    pub original_filename: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub path: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
    pub is_optimized: Option<bool>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct MediaQuery {
    pub search: Option<String>,
    pub r#type: Option<String>,
    pub collection: Option<String>,
    pub images_only: Option<bool>,
    pub is_optimized: Option<bool>,
    #[serde(alias = "sort")]
    pub sort_by: Option<String>,
    #[serde(alias = "order")]
    pub sort_dir: Option<String>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMedia {
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
}

/// Request for presigned upload URL
#[derive(Debug, Deserialize)]
pub struct PresignedUploadRequest {
    pub filename: String,
    pub content_type: String,
    pub size: Option<i64>,
    pub collection: Option<String>,
}

/// Response for presigned upload
#[derive(Debug, Serialize)]
pub struct PresignedUploadResponse {
    pub upload_url: String,
    pub file_key: String,
    pub public_url: String,
    pub expires_in: u64,
}

/// Request to confirm upload after direct upload to R2
#[derive(Debug, Deserialize)]
pub struct ConfirmUploadRequest {
    pub file_key: String,
    pub original_filename: String,
    pub content_type: String,
    pub size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateRequest {
    pub ids: Vec<i64>,
    pub collection: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
}

/// Response for cleanup operation
#[derive(Debug, Serialize)]
pub struct CleanupResult {
    pub orphaned_files_found: i64,
    pub orphaned_files_deleted: i64,
    pub orphaned_db_records: i64,
    pub db_records_cleaned: i64,
    pub errors: Vec<String>,
}

/// Malware scan result
#[derive(Debug, Serialize)]
pub struct MalwareScanResult {
    pub is_clean: bool,
    pub threat_name: Option<String>,
    pub scan_provider: String,
    pub scanned_at: String,
}
