//! Cloudflare R2 storage service
//!
//! Uses Workers R2 binding for direct bucket access (no HTTP API needed)

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// R2 storage service for file uploads
/// Uses Cloudflare Workers R2 binding for direct access
#[derive(Clone)]
pub struct R2Service {
    bucket_name: String,
    public_url: String,
}

impl R2Service {
    pub fn new(
        bucket_name: &str,
        public_url: &str,
        _account_id: &str,
        _access_key_id: &str,
        _secret_access_key: &str,
    ) -> Self {
        Self {
            bucket_name: bucket_name.to_string(),
            public_url: public_url.to_string(),
        }
    }

    /// Upload a file to R2 using Workers binding
    /// Note: In actual usage, pass the R2 bucket binding from env
    pub async fn upload_with_bucket(
        &self,
        bucket: &worker::Bucket,
        key: &str,
        data: Vec<u8>,
        _content_type: &str,
    ) -> Result<UploadResult, ApiError> {
        let size = data.len() as u64;
        
        // worker-rs 0.7 R2 API - put returns a PutOptionsBuilder
        bucket
            .put(key, data)
            .execute()
            .await
            .map_err(|e| ApiError::ExternalService(format!("R2 upload failed: {:?}", e)))?;

        Ok(UploadResult {
            key: key.to_string(),
            url: format!("{}/{}", self.public_url, key),
            size,
        })
    }

    /// Upload a file (simplified - returns URL based on key)
    pub async fn upload(
        &self,
        key: &str,
        data: Vec<u8>,
        _content_type: &str,
    ) -> Result<UploadResult, ApiError> {
        // This is a placeholder - actual upload requires bucket binding
        // In routes, use upload_with_bucket with ctx.env.bucket("MEDIA")
        Ok(UploadResult {
            key: key.to_string(),
            url: format!("{}/{}", self.public_url, key),
            size: data.len() as u64,
        })
    }

    /// Delete a file from R2 using Workers binding
    pub async fn delete_with_bucket(
        &self,
        bucket: &worker::Bucket,
        key: &str,
    ) -> Result<(), ApiError> {
        bucket
            .delete(key)
            .await
            .map_err(|e| ApiError::ExternalService(format!("R2 delete failed: {:?}", e)))?;
        Ok(())
    }

    /// Delete a file (placeholder)
    pub async fn delete(&self, _key: &str) -> Result<(), ApiError> {
        // Actual delete requires bucket binding
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
