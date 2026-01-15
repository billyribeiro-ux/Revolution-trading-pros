//! Revolution Trading Pros - Library Root
//!
//! ICT 7 Principal Engineer Grade
//! Enterprise-grade Rust/Axum API with security hardening

use std::sync::Arc;

pub mod config;
pub mod database;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod responses;
pub mod routes;
pub mod services;
pub mod utils;
pub mod validation;

pub use errors::AppError;
pub use middleware::rate_limit::RateLimiter;
pub use responses::ApiResponse;
pub use services::TokenBlacklist;

/// Application state shared across all handlers
/// ICT 7 SECURITY: Includes rate limiter and token blacklist
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL connection pool
    pub db: sqlx::PgPool,
    /// Redis client (optional)
    pub redis: Option<redis::Client>,
    /// Application configuration
    pub config: Arc<config::AppConfig>,
    /// ICT 7 SECURITY: In-memory rate limiter for auth endpoints
    pub rate_limiter: RateLimiter,
    /// ICT 7 SECURITY: Token blacklist for logout/revocation
    pub token_blacklist: TokenBlacklist,
}
