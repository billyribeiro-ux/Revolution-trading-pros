//! Application configuration

use anyhow::{Context, Result};

/// Application configuration loaded from environment variables
#[derive(Clone, Debug)]
pub struct Config {
    // Server
    pub port: u16,
    pub environment: String,

    // Database (Neon PostgreSQL)
    pub database_url: String,

    // Redis (Upstash)
    pub redis_url: String,

    // Cloudflare R2
    pub r2_endpoint: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket: String,
    pub r2_public_url: String,

    // JWT Authentication
    pub jwt_secret: String,
    pub jwt_expires_in: i64, // hours

    // Stripe
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,

    // CORS
    pub cors_origins: Vec<String>,

    // Email (Postmark)
    pub postmark_token: Option<String>,
    pub from_email: String,

    // Meilisearch
    pub meilisearch_host: String,
    pub meilisearch_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Invalid PORT")?,
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),

            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL required")?,
            redis_url: std::env::var("REDIS_URL").context("REDIS_URL required")?,

            r2_endpoint: std::env::var("R2_ENDPOINT").unwrap_or_default(),
            r2_access_key_id: std::env::var("R2_ACCESS_KEY_ID").unwrap_or_default(),
            r2_secret_access_key: std::env::var("R2_SECRET_ACCESS_KEY").unwrap_or_default(),
            r2_bucket: std::env::var("R2_BUCKET").unwrap_or_else(|_| "revolution-trading-media".to_string()),
            r2_public_url: std::env::var("R2_PUBLIC_URL")
                .unwrap_or_else(|_| "https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev".to_string()),

            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET required")?,
            jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),

            stripe_secret_key: std::env::var("STRIPE_SECRET").unwrap_or_default(),
            stripe_webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),

            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:5173,https://revolution-trading-pros.pages.dev".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),

            postmark_token: std::env::var("POSTMARK_TOKEN").ok(),
            from_email: std::env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@revolutiontradingpros.com".to_string()),

            meilisearch_host: std::env::var("MEILISEARCH_HOST")
                .unwrap_or_else(|_| "http://localhost:7700".to_string()),
            meilisearch_api_key: std::env::var("MEILISEARCH_API_KEY").unwrap_or_default(),
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
