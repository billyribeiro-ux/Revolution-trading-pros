//! Configuration management for the API

use worker::Env;

/// Application configuration loaded from environment
#[derive(Clone)]
pub struct Config {
    pub environment: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub postmark_api_key: String,
    pub meilisearch_url: String,
    pub meilisearch_api_key: String,
    pub cors_origins: Vec<String>,
    pub site_url: String,
    pub from_email: String,
    pub from_name: String,
}

impl Config {
    /// Load configuration from Cloudflare Worker environment
    pub fn from_env(env: &Env) -> Result<Self, worker::Error> {
        Ok(Self {
            environment: env.var("ENVIRONMENT")?.to_string(),
            database_url: env.secret("DATABASE_URL")?.to_string(),
            redis_url: env.secret("REDIS_URL")?.to_string(),
            jwt_secret: env.secret("JWT_SECRET")?.to_string(),
            jwt_issuer: env.var("JWT_ISSUER").map(|v| v.to_string()).unwrap_or_else(|_| "revolution-trading-pros".to_string()),
            jwt_audience: env.var("JWT_AUDIENCE").map(|v| v.to_string()).unwrap_or_else(|_| "revolution-trading-pros-api".to_string()),
            stripe_secret_key: env.secret("STRIPE_SECRET_KEY")?.to_string(),
            stripe_webhook_secret: env.secret("STRIPE_WEBHOOK_SECRET")?.to_string(),
            postmark_api_key: env.secret("POSTMARK_API_KEY")?.to_string(),
            meilisearch_url: env.secret("MEILISEARCH_URL")?.to_string(),
            meilisearch_api_key: env.secret("MEILISEARCH_API_KEY")?.to_string(),
            cors_origins: vec![
                "https://revolutiontradingpros.com".to_string(),
                "https://www.revolutiontradingpros.com".to_string(),
                "https://revolution-trading-pros.pages.dev".to_string(),
                "http://localhost:5173".to_string(),
                "http://localhost:5174".to_string(),
                "http://127.0.0.1:5173".to_string(),
                "http://127.0.0.1:5174".to_string(),
            ],
            site_url: env.var("SITE_URL").map(|v| v.to_string()).unwrap_or_else(|_| "https://revolution-trading-pros.pages.dev".to_string()),
            from_email: env.var("FROM_EMAIL").map(|v| v.to_string()).unwrap_or_else(|_| "noreply@revolutiontradingpros.com".to_string()),
            from_name: env.var("FROM_NAME").map(|v| v.to_string()).unwrap_or_else(|_| "Revolution Trading Pros".to_string()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
