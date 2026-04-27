//! Application configuration

use anyhow::{Context, Result};

/// Application configuration loaded from environment variables
#[derive(Clone)]
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

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            // Non-sensitive: server configuration
            .field("port", &self.port)
            .field("environment", &self.environment)
            // Sensitive: database credentials
            .field("database_url", &"[REDACTED]")
            .field("redis_url", &"[REDACTED]")
            // Sensitive: R2 credentials (endpoint/bucket/public_url are non-sensitive)
            .field("r2_endpoint", &self.r2_endpoint)
            .field("r2_access_key_id", &"[REDACTED]")
            .field("r2_secret_access_key", &"[REDACTED]")
            .field("r2_bucket", &self.r2_bucket)
            .field("r2_public_url", &self.r2_public_url)
            // Sensitive: JWT
            .field("jwt_secret", &"[REDACTED]")
            .field("jwt_expires_in", &self.jwt_expires_in)
            // Sensitive: Stripe
            .field("stripe_secret_key", &"[REDACTED]")
            .field("stripe_publishable_key", &"[REDACTED]")
            .field("stripe_webhook_secret", &"[REDACTED]")
            // Non-sensitive: CORS
            .field("cors_origins", &self.cors_origins)
            // Sensitive: Postmark token
            .field(
                "postmark_token",
                &self.postmark_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("from_email", &self.from_email)
            .field("app_url", &self.app_url)
            // Sensitive: Meilisearch
            .field("meilisearch_host", &self.meilisearch_host)
            .field("meilisearch_api_key", &"[REDACTED]")
            // Non-sensitive: admin/developer config
            .field("superadmin_emails", &self.superadmin_emails)
            .field("developer_emails", &self.developer_emails)
            .field("developer_mode", &self.developer_mode)
            // Sensitive: developer bootstrap
            .field("developer_bootstrap_email", &self.developer_bootstrap_email)
            .field(
                "developer_bootstrap_password_hash",
                &self
                    .developer_bootstrap_password_hash
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field("developer_bootstrap_name", &self.developer_bootstrap_name)
            // Sensitive: OAuth
            .field("google_client_id", &self.google_client_id)
            .field(
                "google_client_secret",
                &self.google_client_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("apple_client_id", &self.apple_client_id)
            .field("apple_team_id", &self.apple_team_id)
            .field("apple_key_id", &"[REDACTED]")
            .field(
                "apple_private_key",
                &self.apple_private_key.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let environment =
            std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let is_dev = environment == "development";

        // FIX-2026-04-27: defence-in-depth. The dev-only fallback below would
        // silently use placeholder R2/Stripe/Meili creds if ENVIRONMENT=development
        // ever landed in production secrets by accident. Refuse to boot when
        // ENVIRONMENT is "development" but APP_URL points at a known prod host.
        if is_dev {
            let app_url = std::env::var("APP_URL").unwrap_or_default();
            const PROD_INDICATORS: &[&str] = &[
                "revolution-trading-pros.pages.dev",
                "revolution-trading-pros-api.fly.dev",
                "revolutiontradingpros.com",
            ];
            if PROD_INDICATORS.iter().any(|d| app_url.contains(d)) {
                panic!(
                    "FATAL: ENVIRONMENT=development but APP_URL ({}) looks like production. \
                     Refusing to start with placeholder credentials in production. \
                     Set ENVIRONMENT=production or fix APP_URL.",
                    app_url
                );
            }
        }

        // FIX-2026-04-27: dev-only fallback. In production these still hard-fail
        // via `.context(...)?`; in development we accept missing values so the
        // local stack can boot without R2/Stripe/Meili creds. Real uploads /
        // payments / search will still 500 at the call site if used.
        fn required_or_dev(key: &str, dev: bool, fallback: &str) -> Result<String> {
            match std::env::var(key) {
                Ok(v) => Ok(v),
                Err(_) if dev => {
                    tracing::warn!(
                        "{key} not set - using development fallback. Feature using {key} will not work end-to-end."
                    );
                    Ok(fallback.to_string())
                }
                Err(e) => Err(anyhow::Error::new(e).context(format!("{key} is required"))),
            }
        }

        Ok(Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Invalid PORT")?,
            environment: environment.clone(),

            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL required")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),

            r2_endpoint: required_or_dev(
                "R2_ENDPOINT",
                is_dev,
                "https://example-account.r2.cloudflarestorage.com",
            )?,
            r2_access_key_id: required_or_dev("R2_ACCESS_KEY_ID", is_dev, "dev-placeholder")?,
            r2_secret_access_key: required_or_dev(
                "R2_SECRET_ACCESS_KEY",
                is_dev,
                "dev-placeholder",
            )?,
            r2_bucket: required_or_dev("R2_BUCKET", is_dev, "revolution-trading-media")?,
            r2_public_url: std::env::var("R2_PUBLIC_URL").unwrap_or_else(|_| {
                "https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev".to_string()
            }),

            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET required")?,
            jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),

            stripe_secret_key: required_or_dev("STRIPE_SECRET", is_dev, "sk_test_placeholder")?,
            stripe_publishable_key: std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_else(|_| {
                tracing::warn!("STRIPE_PUBLISHABLE_KEY not set - payment features will not work");
                String::new()
            }),
            stripe_webhook_secret: required_or_dev(
                "STRIPE_WEBHOOK_SECRET",
                is_dev,
                "whsec_placeholder",
            )?,

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
            meilisearch_api_key: required_or_dev("MEILISEARCH_API_KEY", is_dev, "dev-placeholder")?,

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
