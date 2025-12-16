//! Cloudflare R2 storage service

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// R2 storage service for file uploads
pub struct R2Service {
    bucket_name: String,
    public_url: String,
    account_id: String,
    access_key_id: String,
    secret_access_key: String,
    http_client: reqwest::Client,
}

impl R2Service {
    pub fn new(
        bucket_name: &str,
        public_url: &str,
        account_id: &str,
        access_key_id: &str,
        secret_access_key: &str,
    ) -> Self {
        Self {
            bucket_name: bucket_name.to_string(),
            public_url: public_url.to_string(),
            account_id: account_id.to_string(),
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Upload a file to R2
    pub async fn upload(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<UploadResult, ApiError> {
        let endpoint = format!(
            "https://{}.r2.cloudflarestorage.com/{}/{}",
            self.account_id, self.bucket_name, key
        );

        // Generate AWS Signature V4 (simplified - in production use aws-sigv4 crate)
        let date = chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
        
        let response = self.http_client
            .put(&endpoint)
            .header("Content-Type", content_type)
            .header("x-amz-date", &date)
            .header("x-amz-content-sha256", "UNSIGNED-PAYLOAD")
            .basic_auth(&self.access_key_id, Some(&self.secret_access_key))
            .body(data)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("R2 upload failed: {}", e)))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(ApiError::ExternalService(format!("R2 upload error: {}", text)));
        }

        Ok(UploadResult {
            key: key.to_string(),
            url: format!("{}/{}", self.public_url, key),
            size: 0, // Would need to track this
        })
    }

    /// Delete a file from R2
    pub async fn delete(&self, key: &str) -> Result<(), ApiError> {
        let endpoint = format!(
            "https://{}.r2.cloudflarestorage.com/{}/{}",
            self.account_id, self.bucket_name, key
        );

        let date = chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string();

        let response = self.http_client
            .delete(&endpoint)
            .header("x-amz-date", &date)
            .header("x-amz-content-sha256", "UNSIGNED-PAYLOAD")
            .basic_auth(&self.access_key_id, Some(&self.secret_access_key))
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("R2 delete failed: {}", e)))?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let text = response.text().await.unwrap_or_default();
            return Err(ApiError::ExternalService(format!("R2 delete error: {}", text)));
        }

        Ok(())
    }

    /// Generate a presigned URL for direct upload
    pub fn generate_presigned_url(&self, key: &str, expires_in_seconds: u64) -> String {
        // In production, use proper AWS Signature V4 presigning
        format!(
            "{}/{}?X-Amz-Expires={}",
            self.public_url, key, expires_in_seconds
        )
    }

    /// Get the public URL for a file
    pub fn get_public_url(&self, key: &str) -> String {
        format!("{}/{}", self.public_url, key)
    }

    /// Generate a unique key for a file
    pub fn generate_key(folder: &str, filename: &str) -> String {
        let uuid = uuid::Uuid::new_v4();
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        if extension.is_empty() {
            format!("{}/{}", folder, uuid)
        } else {
            format!("{}/{}.{}", folder, uuid, extension)
        }
    }
}

/// Result of a file upload
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResult {
    pub key: String,
    pub url: String,
    pub size: u64,
}

/// File metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub key: String,
    pub size: u64,
    pub content_type: String,
    pub last_modified: String,
    pub etag: Option<String>,
}

/// Supported image types for processing
pub fn is_image(content_type: &str) -> bool {
    matches!(
        content_type,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" | "image/svg+xml"
    )
}

/// Get content type from filename
pub fn get_content_type(filename: &str) -> &'static str {
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "json" => "application/json",
        "txt" => "text/plain",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "application/octet-stream",
    }
}
