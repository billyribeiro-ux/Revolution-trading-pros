//! Bunny.net Video & Storage Service - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! Lightning-fast video delivery via Bunny.net Stream + CDN
//! 114 global edge locations, <50ms latency

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error, info};

// ═══════════════════════════════════════════════════════════════════════════
// BUNNY.NET CLIENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone)]
pub struct BunnyClient {
    http: Client,
    stream_api_key: String,
    stream_library_id: String,
    storage_api_key: String,
    storage_zone: String,
    storage_hostname: String,
    cdn_url: String,
}

impl BunnyClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
            stream_api_key: env::var("BUNNY_STREAM_API_KEY").unwrap_or_default(),
            stream_library_id: env::var("BUNNY_STREAM_LIBRARY_ID").unwrap_or_default(),
            storage_api_key: env::var("BUNNY_STORAGE_API_KEY").unwrap_or_default(),
            storage_zone: env::var("BUNNY_STORAGE_ZONE").unwrap_or_else(|_| "revolution-trading-thumbnails".to_string()),
            storage_hostname: env::var("BUNNY_STORAGE_HOSTNAME").unwrap_or_else(|_| "ny.storage.bunnycdn.com".to_string()),
            cdn_url: env::var("BUNNY_CDN_URL").unwrap_or_else(|_| "https://revolution-thumbnails-cdn.b-cdn.net".to_string()),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.stream_api_key.is_empty() && !self.stream_library_id.is_empty()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // STREAM API (Video Hosting)
    // ═══════════════════════════════════════════════════════════════════════

    /// Create a new video in Bunny Stream and get upload URL
    pub async fn create_video(&self, title: &str) -> Result<BunnyVideoCreate, BunnyError> {
        let url = format!(
            "https://video.bunnycdn.com/library/{}/videos",
            self.stream_library_id
        );

        let response = self.http
            .post(&url)
            .header("AccessKey", &self.stream_api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "title": title }))
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            error!("Bunny create video failed: {} - {}", status, text);
            return Err(BunnyError::Api(format!("Status {}: {}", status, text)));
        }

        response.json().await.map_err(|e| BunnyError::Parse(e.to_string()))
    }

    /// Get direct upload URL for a video (TUS resumable upload)
    pub async fn get_upload_url(&self, video_guid: &str) -> Result<String, BunnyError> {
        // Bunny uses TUS protocol for uploads
        Ok(format!(
            "https://video.bunnycdn.com/tusupload?libraryId={}&videoId={}&expirationTime={}",
            self.stream_library_id,
            video_guid,
            chrono::Utc::now().timestamp() + 3600 // 1 hour expiry
        ))
    }

    /// Get video details
    pub async fn get_video(&self, video_guid: &str) -> Result<BunnyVideo, BunnyError> {
        let url = format!(
            "https://video.bunnycdn.com/library/{}/videos/{}",
            self.stream_library_id, video_guid
        );

        let response = self.http
            .get(&url)
            .header("AccessKey", &self.stream_api_key)
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            return Err(BunnyError::NotFound);
        }

        response.json().await.map_err(|e| BunnyError::Parse(e.to_string()))
    }

    /// List all videos in library
    pub async fn list_videos(&self, page: i32, per_page: i32) -> Result<BunnyVideoList, BunnyError> {
        let url = format!(
            "https://video.bunnycdn.com/library/{}/videos?page={}&itemsPerPage={}",
            self.stream_library_id, page, per_page
        );

        let response = self.http
            .get(&url)
            .header("AccessKey", &self.stream_api_key)
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(BunnyError::Api(format!("Status {}: {}", status, text)));
        }

        response.json().await.map_err(|e| BunnyError::Parse(e.to_string()))
    }

    /// Delete a video
    pub async fn delete_video(&self, video_guid: &str) -> Result<(), BunnyError> {
        let url = format!(
            "https://video.bunnycdn.com/library/{}/videos/{}",
            self.stream_library_id, video_guid
        );

        let response = self.http
            .delete(&url)
            .header("AccessKey", &self.stream_api_key)
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            return Err(BunnyError::Api("Delete failed".to_string()));
        }

        info!("Deleted Bunny video: {}", video_guid);
        Ok(())
    }

    /// Get embed URL for a video
    pub fn get_embed_url(&self, video_guid: &str) -> String {
        format!(
            "https://iframe.mediadelivery.net/embed/{}/{}",
            self.stream_library_id, video_guid
        )
    }

    /// Get direct play URL
    pub fn get_play_url(&self, video_guid: &str) -> String {
        format!(
            "https://iframe.mediadelivery.net/play/{}/{}",
            self.stream_library_id, video_guid
        )
    }

    /// Get thumbnail URL from Bunny
    pub fn get_thumbnail_url(&self, video_guid: &str) -> String {
        format!(
            "https://vz-{}.b-cdn.net/{}/thumbnail.jpg",
            self.stream_library_id, video_guid
        )
    }

    // ═══════════════════════════════════════════════════════════════════════
    // STORAGE API (Thumbnails)
    // ═══════════════════════════════════════════════════════════════════════

    /// Upload thumbnail to Bunny Storage
    pub async fn upload_thumbnail(&self, path: &str, data: Vec<u8>) -> Result<String, BunnyError> {
        let url = format!(
            "https://{}/{}/{}",
            self.storage_hostname, self.storage_zone, path
        );

        let response = self.http
            .put(&url)
            .header("AccessKey", &self.storage_api_key)
            .header("Content-Type", "image/jpeg")
            .body(data)
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(BunnyError::Api(format!("Upload failed: {} - {}", status, text)));
        }

        // Return CDN URL
        let cdn_url = format!("{}/{}", self.cdn_url, path);
        info!("Uploaded thumbnail: {}", cdn_url);
        Ok(cdn_url)
    }

    /// Delete thumbnail from Bunny Storage
    pub async fn delete_thumbnail(&self, path: &str) -> Result<(), BunnyError> {
        let url = format!(
            "https://{}/{}/{}",
            self.storage_hostname, self.storage_zone, path
        );

        let response = self.http
            .delete(&url)
            .header("AccessKey", &self.storage_api_key)
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            return Err(BunnyError::Api("Delete failed".to_string()));
        }

        Ok(())
    }

    /// Download image from URL and upload to Bunny Storage
    pub async fn upload_thumbnail_from_url(&self, source_url: &str, path: &str) -> Result<String, BunnyError> {
        // Download image
        let response = self.http
            .get(source_url)
            .send()
            .await
            .map_err(|e| BunnyError::Request(e.to_string()))?;

        if !response.status().is_success() {
            return Err(BunnyError::Api("Failed to download source image".to_string()));
        }

        let data = response.bytes().await
            .map_err(|e| BunnyError::Request(e.to_string()))?
            .to_vec();

        self.upload_thumbnail(path, data).await
    }
}

impl Default for BunnyClient {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BUNNY API TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize)]
pub struct BunnyVideoCreate {
    pub guid: String,
    #[serde(rename = "videoLibraryId")]
    pub video_library_id: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BunnyVideo {
    pub guid: String,
    #[serde(rename = "videoLibraryId")]
    pub video_library_id: i64,
    pub title: String,
    #[serde(rename = "dateUploaded")]
    pub date_uploaded: Option<String>,
    pub views: i64,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub length: i32, // duration in seconds
    pub status: i32, // 0=created, 1=uploaded, 2=processing, 3=transcoding, 4=finished, 5=error
    #[serde(rename = "encodeProgress")]
    pub encode_progress: Option<i32>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BunnyVideoList {
    pub items: Vec<BunnyVideo>,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,
}

// ═══════════════════════════════════════════════════════════════════════════
// ERROR HANDLING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum BunnyError {
    Request(String),
    Api(String),
    Parse(String),
    NotFound,
    NotConfigured,
}

impl std::fmt::Display for BunnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BunnyError::Request(e) => write!(f, "Request error: {}", e),
            BunnyError::Api(e) => write!(f, "API error: {}", e),
            BunnyError::Parse(e) => write!(f, "Parse error: {}", e),
            BunnyError::NotFound => write!(f, "Not found"),
            BunnyError::NotConfigured => write!(f, "Bunny.net not configured"),
        }
    }
}

impl std::error::Error for BunnyError {}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Get encoding status as human-readable string
pub fn get_encoding_status(status: i32) -> &'static str {
    match status {
        0 => "created",
        1 => "uploaded",
        2 => "processing",
        3 => "transcoding",
        4 => "finished",
        5 => "error",
        _ => "unknown",
    }
}

/// Generate unique thumbnail path
pub fn generate_thumbnail_path(video_id: i64) -> String {
    let date = chrono::Utc::now().format("%Y/%m/%d");
    format!("thumbnails/{}/{}.jpg", date, video_id)
}
