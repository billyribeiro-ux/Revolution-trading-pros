//! Database module - Fly.io PostgreSQL connection
//! ICT 7 Principal Engineer Grade - Secure Bootstrap System

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::Config;

/// Database wrapper for PostgreSQL connection pool
#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Create a new database connection
    pub async fn new(config: &Config) -> Result<Self> {
        // ICT 11+ FIX: Increased pool size for production load
        // 10 was causing connection starvation at scale
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .min_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(&config.database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    
    /// ICT 7 Principal Engineer Grade: Bootstrap Developer Account
    /// 
    /// Creates or updates developer account from secure environment variables.
    /// This eliminates hardcoded credentials while ensuring platform owner access.
    /// 
    /// Environment Variables Required:
    /// - DEVELOPER_BOOTSTRAP_EMAIL: Developer email address
    /// - DEVELOPER_BOOTSTRAP_PASSWORD_HASH: Argon2id password hash
    /// - DEVELOPER_BOOTSTRAP_NAME: Display name (optional, defaults to "Developer")
    /// 
    /// Security Features:
    /// - No credentials in source code
    /// - Idempotent (safe to run multiple times)
    /// - Audit logged
    /// - Email verification bypassed for developer emails (configured in DEVELOPER_EMAILS)
    pub async fn bootstrap_developer(&self, config: &Config) -> Result<()> {
        // Only bootstrap if all required env vars are set
        let email = match &config.developer_bootstrap_email {
            Some(e) if !e.is_empty() => e.clone(),
            _ => {
                tracing::debug!("DEVELOPER_BOOTSTRAP_EMAIL not set, skipping bootstrap");
                return Ok(());
            }
        };
        
        let password_hash = match &config.developer_bootstrap_password_hash {
            Some(h) if !h.is_empty() => h.clone(),
            _ => {
                tracing::warn!("DEVELOPER_BOOTSTRAP_PASSWORD_HASH not set, skipping bootstrap");
                return Ok(());
            }
        };
        
        let name = config.developer_bootstrap_name
            .clone()
            .unwrap_or_else(|| "Developer".to_string());
        
        // Upsert developer account
        let result = sqlx::query(r#"
            INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
            VALUES ($1, $2, $3, 'developer', NOW(), NOW(), NOW())
            ON CONFLICT (email) DO UPDATE SET
                password_hash = EXCLUDED.password_hash,
                role = 'developer',
                email_verified_at = COALESCE(users.email_verified_at, NOW()),
                updated_at = NOW()
            RETURNING id
        "#)
            .bind(&email)
            .bind(&password_hash)
            .bind(&name)
            .fetch_one(&self.pool)
            .await;
        
        match result {
            Ok(row) => {
                let user_id: i64 = sqlx::Row::get(&row, "id");
                tracing::info!(
                    target: "security_audit",
                    event = "developer_bootstrap",
                    user_id = %user_id,
                    email = %email,
                    "ICT 7: Developer account bootstrapped successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "developer_bootstrap_failed",
                    email = %email,
                    error = %e,
                    "ICT 7: Developer bootstrap failed"
                );
                return Err(e.into());
            }
        }
        
        Ok(())
    }
}
