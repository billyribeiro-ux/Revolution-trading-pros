//! Application Configuration
//!
//! ICT 11+ Principal Engineer Grade
//! Environment-based configuration with validation

use std::env;

use serde::Deserialize;

/// Main application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub jwt: JwtSettings,
    pub stripe: StripeSettings,
    pub cors: CorsSettings,
    pub email: EmailSettings,
    pub developer: DeveloperSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub env: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettings {
    pub url: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    pub access_token_expires_in: String,
    pub refresh_token_expires_in: String,
    pub issuer: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeSettings {
    pub secret_key: String,
    pub publishable_key: String,
    pub webhook_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
    pub max_age: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

/// ICT 11+: Developer mode settings for full access bypass
#[derive(Debug, Clone, Deserialize)]
pub struct DeveloperSettings {
    pub enabled: bool,
    pub emails: Vec<String>,
    pub password: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            app: AppSettings {
                name: env::var("APP_NAME")
                    .unwrap_or_else(|_| "Revolution Trading Pros API".to_string()),
                env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
                debug: env::var("APP_DEBUG").map(|v| v == "true").unwrap_or(false),
            },
            server: ServerSettings {
                host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
            },
            database: DatabaseSettings {
                url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(100),
                min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
            },
            redis: RedisSettings {
                url: env::var("REDIS_URL").ok(),
                password: env::var("REDIS_PASSWORD").ok(),
            },
            jwt: JwtSettings {
                secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                access_token_expires_in: env::var("JWT_ACCESS_TOKEN_EXPIRES_IN")
                    .unwrap_or_else(|_| "15m".to_string()),
                refresh_token_expires_in: env::var("JWT_REFRESH_TOKEN_EXPIRES_IN")
                    .unwrap_or_else(|_| "7d".to_string()),
                issuer: env::var("JWT_ISSUER")
                    .unwrap_or_else(|_| "revolution-trading-pros".to_string()),
            },
            stripe: StripeSettings {
                secret_key: env::var("STRIPE_SECRET_KEY").unwrap_or_default(),
                publishable_key: env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_default(),
                webhook_secret: env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),
            },
            cors: CorsSettings {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:5173".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                max_age: env::var("CORS_MAX_AGE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600),
            },
            email: EmailSettings {
                smtp_host: env::var("SMTP_HOST")
                    .unwrap_or_else(|_| "smtp.postmarkapp.com".to_string()),
                smtp_port: env::var("SMTP_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(587),
                smtp_username: env::var("SMTP_USERNAME").unwrap_or_default(),
                smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
                from_email: env::var("SMTP_FROM_EMAIL")
                    .unwrap_or_else(|_| "noreply@example.com".to_string()),
                from_name: env::var("SMTP_FROM_NAME")
                    .unwrap_or_else(|_| "Revolution Trading Pros".to_string()),
            },
            developer: DeveloperSettings {
                enabled: env::var("DEVELOPER_MODE")
                    .map(|v| v == "true")
                    .unwrap_or(false),
                emails: env::var("DEVELOPER_EMAILS")
                    .unwrap_or_default()
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect(),
                password: env::var("DEVELOPER_PASSWORD").unwrap_or_else(|_| "dev123!".to_string()),
            },
        })
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.app.env == "production"
    }

    /// Check if running in development
    pub fn is_development(&self) -> bool {
        self.app.env == "development"
    }
}
