//! Rate Limiting Middleware
//!
//! ICT 7 SECURITY: Rate limiting for authentication endpoints
//! Apple Principal Engineer Grade: Defense against brute force attacks
//!
//! SECURITY FEATURES:
//! - Per-IP rate limiting for login attempts
//! - Configurable limits and windows
//! - Thread-safe in-memory storage
//! - Graceful degradation (allows through if storage fails)

use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use crate::AppState;

/// Rate limit configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Maximum requests allowed in the window
    pub max_requests: u32,
    /// Time window in seconds
    pub window_seconds: u64,
    /// Lockout duration after exceeding limit
    pub lockout_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 5,      // 5 attempts
            window_seconds: 300,  // 5 minute window
            lockout_seconds: 900, // 15 minute lockout
        }
    }
}

/// Rate limit entry for a single IP
#[derive(Clone)]
pub struct RateLimitEntry {
    count: u32,
    window_start: Instant,
    locked_until: Option<Instant>,
}

impl Default for RateLimitEntry {
    fn default() -> Self {
        Self {
            count: 0,
            window_start: Instant::now(),
            locked_until: None,
        }
    }
}

/// In-memory rate limiter storage
/// ICT 7: Use Arc<RwLock> for thread-safe concurrent access
pub type RateLimiter = Arc<RwLock<HashMap<String, RateLimitEntry>>>;

/// Create a new rate limiter instance
pub fn new_rate_limiter() -> RateLimiter {
    Arc::new(RwLock::new(HashMap::new()))
}

/// ICT 7 SECURITY: Rate limiting middleware for login endpoints
/// Prevents brute force attacks by limiting requests per IP
pub async fn rate_limit_login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let config = RateLimitConfig::default();
    let ip = addr.ip().to_string();

    // Check rate limit
    let result = check_rate_limit(&state.rate_limiter, &ip, &config).await;

    match result {
        RateLimitResult::Allowed { remaining } => {
            // Record this request
            record_request(&state.rate_limiter, &ip).await;

            // Add rate limit headers to response
            let mut response = next.run(request).await;
            let headers = response.headers_mut();
            headers.insert(
                "X-RateLimit-Limit",
                config.max_requests.to_string().parse().unwrap(),
            );
            headers.insert(
                "X-RateLimit-Remaining",
                remaining.to_string().parse().unwrap(),
            );
            response
        }
        RateLimitResult::Limited { retry_after } => {
            tracing::warn!(
                target: "security",
                ip = %ip,
                retry_after = %retry_after,
                "Rate limit exceeded for login attempt"
            );

            let body = json!({
                "error": "Too many login attempts",
                "message": "Please wait before trying again",
                "retry_after": retry_after,
                "code": "RATE_LIMITED"
            });

            (
                StatusCode::TOO_MANY_REQUESTS,
                [
                    ("Retry-After", retry_after.to_string()),
                    ("X-RateLimit-Limit", config.max_requests.to_string()),
                    ("X-RateLimit-Remaining", "0".to_string()),
                ],
                Json(body),
            )
                .into_response()
        }
        RateLimitResult::Locked { until } => {
            tracing::warn!(
                target: "security",
                ip = %ip,
                locked_until = %until,
                "Account locked due to too many failed attempts"
            );

            let body = json!({
                "error": "Account temporarily locked",
                "message": "Too many failed login attempts. Please try again later.",
                "retry_after": until,
                "locked": true,
                "code": "ACCOUNT_LOCKED"
            });

            (
                StatusCode::TOO_MANY_REQUESTS,
                [
                    ("Retry-After", until.to_string()),
                    ("X-RateLimit-Limit", config.max_requests.to_string()),
                    ("X-RateLimit-Remaining", "0".to_string()),
                ],
                Json(body),
            )
                .into_response()
        }
    }
}

/// Result of rate limit check
enum RateLimitResult {
    Allowed { remaining: u32 },
    Limited { retry_after: u64 },
    Locked { until: u64 },
}

/// Check if request is within rate limit
async fn check_rate_limit(
    limiter: &RateLimiter,
    ip: &str,
    config: &RateLimitConfig,
) -> RateLimitResult {
    let limiter = limiter.read().await;

    if let Some(entry) = limiter.get(ip) {
        // Check if locked
        if let Some(locked_until) = entry.locked_until {
            if Instant::now() < locked_until {
                let until = locked_until.duration_since(Instant::now()).as_secs();
                return RateLimitResult::Locked { until };
            }
        }

        // Check if window expired
        let window_duration = Duration::from_secs(config.window_seconds);
        if entry.window_start.elapsed() > window_duration {
            // Window expired, reset
            return RateLimitResult::Allowed {
                remaining: config.max_requests - 1,
            };
        }

        // Check count
        if entry.count >= config.max_requests {
            let retry_after = window_duration
                .saturating_sub(entry.window_start.elapsed())
                .as_secs();
            return RateLimitResult::Limited { retry_after };
        }

        RateLimitResult::Allowed {
            remaining: config.max_requests - entry.count - 1,
        }
    } else {
        RateLimitResult::Allowed {
            remaining: config.max_requests - 1,
        }
    }
}

/// Record a request for rate limiting
async fn record_request(limiter: &RateLimiter, ip: &str) {
    let mut limiter = limiter.write().await;
    let config = RateLimitConfig::default();

    let entry = limiter.entry(ip.to_string()).or_default();
    let window_duration = Duration::from_secs(config.window_seconds);

    // Reset if window expired
    if entry.window_start.elapsed() > window_duration {
        entry.count = 0;
        entry.window_start = Instant::now();
        entry.locked_until = None;
    }

    entry.count += 1;

    // Lock if exceeded
    if entry.count >= config.max_requests {
        entry.locked_until = Some(Instant::now() + Duration::from_secs(config.lockout_seconds));
    }
}

/// ICT 7: Clear rate limit for an IP (called on successful login)
pub async fn clear_rate_limit(limiter: &RateLimiter, ip: &str) {
    let mut limiter = limiter.write().await;
    limiter.remove(ip);
}

/// ICT 7: Record failed login attempt (more aggressive rate limiting)
pub async fn record_failed_login(limiter: &RateLimiter, ip: &str) {
    let mut limiter = limiter.write().await;
    let config = RateLimitConfig::default();

    let entry = limiter.entry(ip.to_string()).or_default();

    // Failed logins count double
    entry.count += 2;

    // Lock immediately if too many failures
    if entry.count >= config.max_requests {
        entry.locked_until = Some(Instant::now() + Duration::from_secs(config.lockout_seconds));
        tracing::warn!(
            target: "security",
            ip = %ip,
            "IP locked due to failed login attempts"
        );
    }
}
