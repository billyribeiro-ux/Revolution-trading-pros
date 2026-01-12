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

/// ICT 7 SECURITY: Separate secrets for access and refresh tokens
/// Apple Principal Engineer Grade: Defense in depth - token type confusion prevention
#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    pub refresh_secret: String, // ICT 7: Separate secret for refresh tokens
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

/// ICT 7 SECURITY: Developer mode settings with production safeguards
/// Apple Principal Engineer Grade: Defense in depth - never enable in production
#[derive(Debug, Clone, Deserialize)]
pub struct DeveloperSettings {
    pub enabled: bool,
    pub emails: Vec<String>,
    pub password: String,
}

impl DeveloperSettings {
    /// ICT 7: Validate developer settings - CRITICAL SECURITY CHECK
    /// Returns error if developer mode is misconfigured
    pub fn validate(&self, is_production: bool) -> Result<(), String> {
        // CRITICAL: Developer mode MUST be disabled in production
        if is_production && self.enabled {
            return Err(
                "SECURITY VIOLATION: Developer mode cannot be enabled in production. \
                Set DEVELOPER_MODE=false or APP_ENV to non-production value.".to_string()
            );
        }

        // Warn if enabled with weak password
        if self.enabled && self.password.len() < 16 {
            tracing::warn!(
                target: "security",
                "Developer mode password is weak. Use at least 16 characters."
            );
        }

        // Warn if enabled with no emails
        if self.enabled && self.emails.is_empty() {
            tracing::warn!(
                target: "security",
                "Developer mode enabled but no emails configured. This is a security risk."
            );
        }

        Ok(())
    }
}

impl AppConfig {
    /// Load configuration from environment variables
    /// ICT 7 SECURITY: Validates configuration before returning
    pub fn from_env() -> anyhow::Result<Self> {
        // ICT 7: Check if developer mode requires password upfront
        let dev_mode_enabled = env::var("DEVELOPER_MODE")
            .map(|v| v == "true")
            .unwrap_or(false);

        // Only require DEVELOPER_PASSWORD if developer mode is actually enabled
        let dev_password = if dev_mode_enabled {
            env::var("DEVELOPER_PASSWORD")
                .expect("DEVELOPER_PASSWORD must be set when DEVELOPER_MODE=true")
        } else {
            // Dummy value when not in dev mode - never used
            String::new()
        };

        let config = Self {
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
                // ICT 7 SECURITY: Separate refresh token secret - falls back to main secret if not set
                // Apple Principal Engineer Grade: Defense in depth - recommend setting separate secret
                refresh_secret: env::var("JWT_REFRESH_SECRET")
                    .unwrap_or_else(|_| {
                        let main_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                        // Derive a different secret by appending suffix (still recommend separate env var)
                        format!("{}_refresh_v1", main_secret)
                    }),
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
                enabled: dev_mode_enabled,
                emails: env::var("DEVELOPER_EMAILS")
                    .unwrap_or_default()
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect(),
                // ICT 7 SECURITY: Password required only when dev mode enabled
                password: dev_password,
            },
        };

        // ICT 7 SECURITY: Validate developer settings BEFORE returning config
        let is_production = config.is_production();
        if let Err(e) = config.developer.validate(is_production) {
            // In production with developer mode, this is a fatal security violation
            if is_production {
                panic!("SECURITY: {}", e);
            } else {
                tracing::error!(target: "security", "{}", e);
            }
        }

        // Log security-relevant configuration on startup
        if dev_mode_enabled {
            tracing::warn!(
                target: "security",
                developer_emails = ?config.developer.emails,
                "⚠️ DEVELOPER MODE ENABLED - Authentication bypass active for configured emails"
            );
        }

        Ok(config)
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
