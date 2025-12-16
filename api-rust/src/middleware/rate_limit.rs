//! Rate limiting middleware

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;

/// Rate limit configuration
pub struct RateLimitConfig {
    pub requests: u64,
    pub window_seconds: u64,
}

impl RateLimitConfig {
    pub fn new(requests: u64, window_seconds: u64) -> Self {
        Self { requests, window_seconds }
    }

    /// Default rate limit: 100 requests per minute
    pub fn default() -> Self {
        Self::new(100, 60)
    }

    /// Strict rate limit: 5 requests per minute (for auth endpoints)
    pub fn strict() -> Self {
        Self::new(5, 60)
    }

    /// Relaxed rate limit: 1000 requests per minute (for public read endpoints)
    pub fn relaxed() -> Self {
        Self::new(1000, 60)
    }
}

/// Check rate limit for a request
pub async fn check_rate_limit(
    req: &Request,
    ctx: &RouteContext<AppState>,
    config: &RateLimitConfig,
    key_prefix: &str,
) -> Result<RateLimitResult, ApiError> {
    // Get client identifier (IP address or user ID)
    let client_id = get_client_identifier(req);
    let key = format!("ratelimit:{}:{}", key_prefix, client_id);

    // Check rate limit using Redis
    let rate_limiter = crate::db::redis::RateLimiter::new(ctx.data.cache.clone());
    let (allowed, remaining, reset_at) = rate_limiter.check(&key, config.requests, config.window_seconds).await?;

    Ok(RateLimitResult {
        allowed,
        remaining,
        reset_at,
        limit: config.requests,
    })
}

/// Rate limit result
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u64,
    pub reset_at: u64,
    pub limit: u64,
}

impl RateLimitResult {
    /// Add rate limit headers to response
    pub fn add_headers(&self, response: Response) -> Response {
        let mut headers = response.headers().clone();
        headers.set("X-RateLimit-Limit", &self.limit.to_string()).ok();
        headers.set("X-RateLimit-Remaining", &self.remaining.to_string()).ok();
        headers.set("X-RateLimit-Reset", &self.reset_at.to_string()).ok();
        
        response.with_headers(headers)
    }

    /// Create a rate limited error response
    pub fn error_response(&self) -> Response {
        let body = serde_json::json!({
            "error": "rate_limited",
            "message": "Too many requests. Please try again later.",
            "retry_after": self.reset_at - crate::utils::now().timestamp() as u64
        });

        Response::from_json(&body)
            .unwrap_or_else(|_| Response::error("Rate limited", 429).unwrap())
            .with_status(429)
            .with_headers({
                let mut headers = worker::Headers::new();
                headers.set("X-RateLimit-Limit", &self.limit.to_string()).ok();
                headers.set("X-RateLimit-Remaining", "0").ok();
                headers.set("X-RateLimit-Reset", &self.reset_at.to_string()).ok();
                headers.set("Retry-After", &(self.reset_at - crate::utils::now().timestamp() as u64).to_string()).ok();
                headers
            })
    }
}

/// Get client identifier from request (IP address)
fn get_client_identifier(req: &Request) -> String {
    // Try Cloudflare headers first
    if let Ok(Some(ip)) = req.headers().get("CF-Connecting-IP") {
        return ip;
    }
    
    // Try X-Forwarded-For
    if let Ok(Some(forwarded)) = req.headers().get("X-Forwarded-For") {
        if let Some(ip) = forwarded.split(',').next() {
            return ip.trim().to_string();
        }
    }
    
    // Try X-Real-IP
    if let Ok(Some(ip)) = req.headers().get("X-Real-IP") {
        return ip;
    }
    
    // Fallback to a default
    "unknown".to_string()
}

/// Rate limit decorator for route handlers
#[macro_export]
macro_rules! rate_limited {
    ($handler:expr, $config:expr) => {
        |req: Request, ctx: RouteContext<AppState>| async move {
            let result = check_rate_limit(&req, &ctx, &$config, stringify!($handler)).await?;
            
            if !result.allowed {
                return Ok(result.error_response());
            }
            
            let response = $handler(req, ctx).await?;
            Ok(result.add_headers(response))
        }
    };
}
