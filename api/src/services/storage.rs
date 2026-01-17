//! Storage service - Cloudflare R2

use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    config::{Credentials, Region},
    primitives::ByteStream,
    Client,
};
use uuid::Uuid;

use crate::config::Config;

#[derive(Clone)]
pub struct StorageService {
    client: Client,
    bucket: String,
    public_url: String,
}

impl StorageService {
    pub async fn new(config: &Config) -> Result<Self> {
        let credentials = Credentials::new(
            &config.r2_access_key_id,
            &config.r2_secret_access_key,
            None,
            None,
            "r2",
        );

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .region(Region::new("auto"))
            .endpoint_url(&config.r2_endpoint)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: config.r2_bucket.clone(),
            public_url: config.r2_public_url.clone(),
        })
    }

    /// Upload a file to R2
    pub async fn upload(&self, data: Vec<u8>, content_type: &str, folder: &str) -> Result<String> {
        let extension = match content_type {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/webp" => "webp",
            "image/gif" => "gif",
            "video/mp4" => "mp4",
            "application/pdf" => "pdf",
            _ => "bin",
        };

        let key = format!("{}/{}.{}", folder, Uuid::new_v4(), extension);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await?;

        Ok(format!("{}/{}", self.public_url, key))
    }

    /// Delete a file from R2
    pub async fn delete(&self, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    /// Generate a presigned URL for direct upload
    /// ICT 11+ FIX: Proper presigned URL generation for R2/S3
    pub async fn presigned_upload_url(
        &self,
        key: &str,
        content_type: &str,
        expires_in_seconds: u64,
    ) -> Result<String> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use std::time::Duration;

        let presigning_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_in_seconds))
            .build()?;

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// Generate a presigned URL for direct download
    /// ICT 11+ ENHANCEMENT: Allow temporary access to private files
    pub async fn presigned_download_url(
        &self,
        key: &str,
        expires_in_seconds: u64,
    ) -> Result<String> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use std::time::Duration;

        let presigning_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_in_seconds))
            .build()?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// List files in a folder
    /// ICT 11+ ENHANCEMENT: Browse uploaded files
    pub async fn list_files(&self, prefix: &str) -> Result<Vec<String>> {
        let response = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(prefix)
            .send()
            .await?;

        let keys = response
            .contents()
            .iter()
            .filter_map(|obj| obj.key().map(|k| k.to_string()))
            .collect();

        Ok(keys)
    }
}
