//! Upstash Redis cache via HTTP
//! 
//! Edge-compatible Redis client using Upstash's REST API
//! WASM-compatible using worker::Fetch

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// Redis cache client for Upstash
#[derive(Clone)]
pub struct Cache {
    url: String,
    token: String,
}

#[derive(Deserialize)]
struct UpstashResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

impl Cache {
    /// Create a new cache connection from Upstash URL
    /// URL format: https://xxx.upstash.io with token
    pub fn new(redis_url: &str) -> Result<Self, ApiError> {
        // Parse UPSTASH_REDIS_REST_URL and UPSTASH_REDIS_REST_TOKEN from URL
        // Format: redis://default:TOKEN@HOST:PORT or https://HOST with separate token
        let (url, token) = Self::parse_url(redis_url)?;
        
        Ok(Self { url, token })
    }

    fn parse_url(redis_url: &str) -> Result<(String, String), ApiError> {
        // Handle Upstash REST URL format
        if redis_url.starts_with("https://") {
            // Assume token is part of the URL or will be set separately
            return Err(ApiError::Database("Redis URL must include token".to_string()));
        }
        
        // Handle redis:// URL format: redis://default:TOKEN@host:port
        if redis_url.starts_with("redis://") || redis_url.starts_with("rediss://") {
            let without_scheme = redis_url
                .trim_start_matches("redis://")
                .trim_start_matches("rediss://");
            
            let parts: Vec<&str> = without_scheme.split('@').collect();
            if parts.len() != 2 {
                return Err(ApiError::Database("Invalid Redis URL format".to_string()));
            }
            
            let auth_parts: Vec<&str> = parts[0].split(':').collect();
            if auth_parts.len() != 2 {
                return Err(ApiError::Database("Invalid Redis auth format".to_string()));
            }
            
            let token = auth_parts[1].to_string();
            let host = parts[1].split(':').next().unwrap_or(parts[1]);
            let url = format!("https://{}", host);
            
            return Ok((url, token));
        }
        
        Err(ApiError::Database("Unsupported Redis URL format".to_string()))
    }

    /// Get a value from cache
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, ApiError> {
        let response: UpstashResponse<String> = self.command(&["GET", key]).await?;
        
        match response.result {
            Some(value) => {
                let parsed: T = serde_json::from_str(&value)
                    .map_err(|e| ApiError::Database(format!("Cache deserialize error: {}", e)))?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    /// Set a value in cache with optional TTL
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> Result<(), ApiError> {
        let json = serde_json::to_string(value)
            .map_err(|e| ApiError::Database(format!("Cache serialize error: {}", e)))?;
        
        let ttl_str = ttl_seconds.map(|t| t.to_string());
        let cmd: Vec<&str> = match &ttl_str {
            Some(ttl) => vec!["SET", key, &json, "EX", ttl],
            None => vec!["SET", key, &json],
        };
        
        let _: UpstashResponse<String> = self.command(&cmd).await?;
        Ok(())
    }

    /// Delete a key from cache
    pub async fn delete(&self, key: &str) -> Result<(), ApiError> {
        let _: UpstashResponse<i64> = self.command(&["DEL", key]).await?;
        Ok(())
    }

    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> Result<bool, ApiError> {
        let response: UpstashResponse<i64> = self.command(&["EXISTS", key]).await?;
        Ok(response.result.unwrap_or(0) > 0)
    }

    /// Increment a counter
    pub async fn incr(&self, key: &str) -> Result<i64, ApiError> {
        let response: UpstashResponse<i64> = self.command(&["INCR", key]).await?;
        Ok(response.result.unwrap_or(0))
    }

    /// Set expiration on a key
    pub async fn expire(&self, key: &str, seconds: u64) -> Result<(), ApiError> {
        let secs = seconds.to_string();
        let _: UpstashResponse<i64> = self.command(&["EXPIRE", key, &secs]).await?;
        Ok(())
    }

    /// Execute a Redis command using worker::Fetch
    async fn command<T: for<'de> Deserialize<'de>>(&self, args: &[&str]) -> Result<UpstashResponse<T>, ApiError> {
        let body = serde_json::to_string(&args)
            .map_err(|e| ApiError::Database(format!("Failed to serialize command: {}", e)))?;

        let headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.token)).ok();
        headers.set("Content-Type", "application/json").ok();

        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Post);
        init.with_headers(headers);
        init.with_body(Some(wasm_bindgen::JsValue::from_str(&body)));

        let request = worker::Request::new_with_init(&self.url, &init)
            .map_err(|e| ApiError::Database(format!("Failed to create request: {:?}", e)))?;

        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::Database(format!("Redis HTTP error: {:?}", e)))?;

        let status = response.status_code();
        let text = response.text().await
            .map_err(|e| ApiError::Database(format!("Failed to read response: {:?}", e)))?;

        if status >= 400 {
            return Err(ApiError::Database(format!("Redis error: {}", text)));
        }

        serde_json::from_str(&text)
            .map_err(|e| ApiError::Database(format!("Redis parse error: {}", e)))
    }
}

/// Rate limiter using Redis
pub struct RateLimiter {
    cache: Cache,
}

impl RateLimiter {
    pub fn new(cache: Cache) -> Self {
        Self { cache }
    }

    /// Check if request is rate limited
    /// Returns (allowed, remaining, reset_at)
    pub async fn check(&self, key: &str, limit: u64, window_seconds: u64) -> Result<(bool, u64, u64), ApiError> {
        let count = self.cache.incr(key).await?;
        
        if count == 1 {
            // First request in window, set expiration
            self.cache.expire(key, window_seconds).await?;
        }
        
        let allowed = (count as u64) <= limit;
        let remaining = if allowed { limit - count as u64 } else { 0 };
        // Use js_sys for time in WASM
        let now_ms = js_sys::Date::now() as u64;
        let reset_at = (now_ms / 1000) + window_seconds;
        
        Ok((allowed, remaining, reset_at))
    }
}
