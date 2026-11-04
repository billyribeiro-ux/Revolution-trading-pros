//! Authenticated client for the optional, real Sharp processing service.
use anyhow::{Context, Result, bail};
use bytes::Bytes;
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const MAX_BYTES: usize = 50 * 1024 * 1024;
const MAX_PIXELS: u64 = 16_000_000;
#[derive(Clone)]
pub struct MediaProcessorService {
    client: Client,
    base: Url,
    token: String,
}
#[derive(Debug, Deserialize)]
pub struct ProcessorHealth {
    pub available: bool,
    pub engine: String,
    pub version: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageInfo {
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub pages: u32,
    pub animated: bool,
    pub size: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOptions {
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    pub quality: u8,
    pub lossless: bool,
}
impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            format: "webp".into(),
            width: None,
            height: None,
            quality: 80,
            lossless: false,
        }
    }
}
pub struct ProcessedImage {
    pub bytes: Bytes,
    pub info: ImageInfo,
    pub content_type: String,
}

impl MediaProcessorService {
    /// Missing URL disables this integration. Enabled configuration is strict;
    /// provider availability is checked on use so unrelated modules can start.
    pub fn from_env() -> Result<Option<Self>> {
        let Some(url) = std::env::var("MEDIA_PROCESSOR_URL")
            .ok()
            .filter(|s| !s.trim().is_empty())
        else {
            return Ok(None);
        };
        let token = std::env::var("MEDIA_PROCESSOR_TOKEN")
            .context("MEDIA_PROCESSOR_TOKEN is required when MEDIA_PROCESSOR_URL is enabled")?;
        Self::new(&url, token).map(Some)
    }
    pub fn new(url: &str, token: String) -> Result<Self> {
        if !(32..=256).contains(&token.len()) || !token.bytes().all(|b| (0x21..=0x7e).contains(&b))
        {
            bail!("MEDIA_PROCESSOR_TOKEN must contain 32–256 visible ASCII characters");
        }
        let mut base = Url::parse(url).context("Invalid MEDIA_PROCESSOR_URL")?;
        if !matches!(base.scheme(), "http" | "https")
            || base.host_str().is_none()
            || !base.username().is_empty()
            || base.password().is_some()
            || base.query().is_some()
            || base.fragment().is_some()
        {
            bail!(
                "MEDIA_PROCESSOR_URL must be an HTTP(S) base URL without credentials, query or fragment"
            );
        }
        if !base.path().ends_with('/') {
            base.set_path(&format!("{}/", base.path()));
        }
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(40))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            client,
            base,
            token,
        })
    }
    pub fn url(&self) -> &str {
        self.base.as_str()
    }
    pub async fn health(&self) -> Result<ProcessorHealth> {
        let response = self
            .client
            .get(self.base.join("health")?)
            .bearer_auth(&self.token)
            .send()
            .await?;
        let bytes = read_bounded(response, 16384).await?;
        let health: ProcessorHealth =
            serde_json::from_slice(&bytes).context("Invalid processor health response")?;
        if !health.available || health.engine != "sharp" || health.version.is_empty() {
            bail!("Configured processor did not acknowledge Sharp availability");
        }
        Ok(health)
    }
    pub async fn inspect(&self, data: Bytes) -> Result<ImageInfo> {
        validate_input(&data)?;
        let response = self
            .client
            .post(self.base.join("inspect")?)
            .bearer_auth(&self.token)
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await?;
        let bytes = read_bounded(response, 16384).await?;
        let info: ImageInfo =
            serde_json::from_slice(&bytes).context("Invalid image inspection response")?;
        validate_info(&info)?;
        Ok(info)
    }
    pub async fn process(&self, data: Bytes, options: &ProcessOptions) -> Result<ProcessedImage> {
        validate_input(&data)?;
        if !matches!(
            options.format.as_str(),
            "jpeg" | "png" | "webp" | "avif" | "gif"
        ) || !(1..=100).contains(&options.quality)
            || options.width.is_some_and(|v| !(1..=8192).contains(&v))
            || options.height.is_some_and(|v| !(1..=8192).contains(&v))
        {
            bail!("Invalid image processing options");
        }
        let response = self
            .client
            .post(self.base.join("process")?)
            .query(options)
            .bearer_auth(&self.token)
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(response_error(response).await);
        }
        let number = |name: &str| -> Result<u32> {
            response
                .headers()
                .get(name)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .context(format!("Invalid processor header {name}"))
        };
        let mut info = ImageInfo {
            width: number("x-image-width")?,
            height: number("x-image-height")?,
            pages: number("x-image-pages")?,
            format: response
                .headers()
                .get("x-image-format")
                .and_then(|v| v.to_str().ok())
                .context("Missing processor format")?
                .to_owned(),
            animated: false,
            size: 0,
        };
        info.animated = info.pages > 1;
        let content_type = format!("image/{}", options.format);
        if response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            != Some(content_type.as_str())
        {
            bail!("Unexpected processor response content type");
        }
        let bytes = read_bounded(response, MAX_BYTES).await?;
        info.size = bytes.len() as u64;
        validate_info(&info)?;
        Ok(ProcessedImage {
            bytes,
            info,
            content_type,
        })
    }
}
fn validate_input(data: &Bytes) -> Result<()> {
    if data.is_empty() || data.len() > MAX_BYTES {
        bail!("Image input must contain 1–50 MiB");
    }
    Ok(())
}
fn validate_info(info: &ImageInfo) -> Result<()> {
    if info.width == 0
        || info.height == 0
        || info.pages == 0
        || info.pages > 256
        || u64::from(info.width) * u64::from(info.height) * u64::from(info.pages) > MAX_PIXELS
        || info.size > MAX_BYTES as u64
    {
        bail!("Processor returned image dimensions or size outside configured limits");
    }
    if !matches!(
        info.format.as_str(),
        "jpeg" | "png" | "webp" | "avif" | "heif" | "gif"
    ) {
        bail!("Unsupported processor image format");
    }
    Ok(())
}
async fn response_error(mut response: Response) -> anyhow::Error {
    let status = response.status();
    let mut bytes = Vec::new();
    while bytes.len() < 16384 {
        match response.chunk().await {
            Ok(Some(chunk)) => {
                let len = chunk.len().min(16384 - bytes.len());
                bytes.extend_from_slice(&chunk[..len]);
            }
            _ => break,
        }
    }
    let message = serde_json::from_slice::<serde_json::Value>(&bytes)
        .ok()
        .and_then(|v| v["error"].as_str().map(str::to_owned))
        .unwrap_or_else(|| "Image processor request failed".into());
    anyhow::anyhow!("Image processor returned {status}: {message}")
}
async fn read_bounded(mut response: Response, limit: usize) -> Result<Bytes> {
    if !response.status().is_success() {
        return Err(response_error(response).await);
    }
    if response.content_length().is_some_and(|s| s > limit as u64) {
        bail!("Processor response exceeds the size limit");
    }
    let mut data = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if data.len() + chunk.len() > limit {
            bail!("Processor response exceeds the size limit");
        }
        data.extend_from_slice(&chunk);
    }
    Ok(Bytes::from(data))
}
