//! Application configuration

use anyhow::{Context, Result};

/// Application configuration loaded from environment variables
#[derive(Clone, Debug)]
pub struct Config {
    // Server
    pub port: u16,
    pub environment: String,

    // Database (Fly.io PostgreSQL)
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
    pub stripe_publishable_key: String,
    pub stripe_webhook_secret: String,

    // CORS
    pub cors_origins: Vec<String>,

    // Email (Postmark)
    pub postmark_token: Option<String>,
    pub from_email: String,
    pub app_url: String, // Frontend URL for email links

    // Meilisearch
    pub meilisearch_host: String,
    pub meilisearch_api_key: String,

    // Superadmin Configuration
    pub superadmin_emails: Vec<String>,

    // Developer Configuration (Enterprise Pattern)
    pub developer_emails: Vec<String>,
    pub developer_mode: bool,

    // Developer Bootstrap (ICT 7 - No Hardcoded Credentials)
    // These are read from environment variables and used to bootstrap developer account on startup
    pub developer_bootstrap_email: Option<String>,
    pub developer_bootstrap_password_hash: Option<String>,
    pub developer_bootstrap_name: Option<String>,

    // OAuth Configuration (ICT Level 7 - Google & Apple Sign-In)
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub apple_client_id: Option<String>,
    pub apple_team_id: Option<String>,
    pub apple_key_id: Option<String>,
    pub apple_private_key: Option<String>,
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
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),

            r2_endpoint: std::env::var("R2_ENDPOINT").unwrap_or_default(),
            r2_access_key_id: std::env::var("R2_ACCESS_KEY_ID").unwrap_or_default(),
            r2_secret_access_key: std::env::var("R2_SECRET_ACCESS_KEY").unwrap_or_default(),
            r2_bucket: std::env::var("R2_BUCKET")
                .unwrap_or_else(|_| "revolution-trading-media".to_string()),
            r2_public_url: std::env::var("R2_PUBLIC_URL").unwrap_or_else(|_| {
                "https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev".to_string()
            }),

            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET required")?,
            jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),

            stripe_secret_key: std::env::var("STRIPE_SECRET").unwrap_or_default(),
            stripe_publishable_key: std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_default(),
            stripe_webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),

            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| {
                    // ICT 7: Include all localhost origins for development and E2E testing
                    let origins = [
                        "https://revolution-trading-pros.pages.dev",
                        "https://www.revolution-trading-pros.pages.dev",
                        "http://localhost:5173",
                        "http://localhost:5174", // Playwright E2E tests
                        "http://localhost:3000",
                        "http://127.0.0.1:5173",
                        "http://127.0.0.1:5174", // Playwright E2E tests
                        "http://127.0.0.1:3000",
                    ];
                    origins.join(",")
                })
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),

            postmark_token: std::env::var("POSTMARK_TOKEN").ok(),
            from_email: std::env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@example.com".to_string()),
            app_url: std::env::var("APP_URL")
                .unwrap_or_else(|_| "https://revolution-trading-pros.pages.dev".to_string()),

            meilisearch_host: std::env::var("MEILISEARCH_HOST")
                .unwrap_or_else(|_| "http://localhost:7700".to_string()),
            meilisearch_api_key: std::env::var("MEILISEARCH_API_KEY").unwrap_or_default(),

            // ICT 11+: NO HARDCODED EMAILS - must be set via environment variables
            superadmin_emails: std::env::var("SUPERADMIN_EMAILS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect(),

            // ICT 11+: NO HARDCODED EMAILS - must be set via environment variables
            developer_emails: std::env::var("DEVELOPER_EMAILS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect(),

            developer_mode: std::env::var("DEVELOPER_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            // ICT 7 Developer Bootstrap - Read from secure environment variables
            developer_bootstrap_email: std::env::var("DEVELOPER_BOOTSTRAP_EMAIL").ok(),
            developer_bootstrap_password_hash: std::env::var("DEVELOPER_BOOTSTRAP_PASSWORD_HASH")
                .ok(),
            developer_bootstrap_name: std::env::var("DEVELOPER_BOOTSTRAP_NAME").ok(),

            // ICT Level 7: OAuth Configuration for Google & Apple Sign-In
            google_client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            apple_client_id: std::env::var("APPLE_CLIENT_ID").ok(),
            apple_team_id: std::env::var("APPLE_TEAM_ID").ok(),
            apple_key_id: std::env::var("APPLE_KEY_ID").ok(),
            apple_private_key: std::env::var("APPLE_PRIVATE_KEY").ok(),
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Check if an email is a superadmin
    pub fn is_superadmin_email(&self, email: &str) -> bool {
        self.superadmin_emails.contains(&email.to_lowercase())
    }

    /// Check if an email is a developer (Enterprise Pattern)
    /// Developers have complete access to all features, memberships, and development tools
    pub fn is_developer_email(&self, email: &str) -> bool {
        self.developer_emails.contains(&email.to_lowercase())
    }

    /// Check if developer mode is enabled
    pub fn is_developer_mode(&self) -> bool {
        self.developer_mode || !self.is_production()
    }
}
