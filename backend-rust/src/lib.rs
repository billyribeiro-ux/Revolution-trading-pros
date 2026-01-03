//! Revolution Trading Pros - Library Root
//!
//! Apple ICT 11+ Principal Engineer Grade
//! Enterprise-grade Rust/Axum API

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
pub use responses::ApiResponse;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL connection pool
    pub db: sqlx::PgPool,
    /// Redis client (optional)
    pub redis: Option<redis::Client>,
    /// Application configuration
    pub config: Arc<config::AppConfig>,
}
