//! Database module - Fly.io PostgreSQL connection
//! ICT 7 Principal Engineer Grade - Secure Bootstrap System with Resilient Connection Retry

use anyhow::{Context, Result};
use argon2::PasswordHash;
use sqlx::{Connection, PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use crate::config::Config;

/// Maximum retry attempts for database connection (PE7: resilience over brittleness)
const MAX_RETRY_ATTEMPTS: u32 = 10;
/// Initial backoff duration between retries
const INITIAL_BACKOFF_MS: u64 = 500;
/// Maximum backoff duration (prevents runaway waits)
const MAX_BACKOFF_MS: u64 = 8000;

/// Database wrapper for PostgreSQL connection pool
#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Create a new database connection with exponential backoff retry (PE7 Grade)
    ///
    /// This implements resilient startup for containerized environments where
    /// the database may still be initializing when the API boots (common with
    /// docker-compose or Kubernetes rolling updates).
    ///
    /// Retry strategy:
    /// - 10 attempts max
    /// - Exponential backoff: 500ms, 1s, 2s, 4s, 8s, 8s, ... (capped)
    /// - Total max wait: ~50 seconds (well within most orchestrator health check windows)
    pub async fn new(config: &Config) -> Result<Self> {
        let mut attempt = 0;
        let mut backoff_ms = INITIAL_BACKOFF_MS;
        let mut last_error = None;

        while attempt < MAX_RETRY_ATTEMPTS {
            attempt += 1;

            match Self::try_connect(config).await {
                Ok(pool) => {
                    if attempt > 1 {
                        tracing::info!(
                            target: "startup",
                            event = "db_connected_after_retry",
                            attempts = attempt,
                            "Database connected after {} retry attempts",
                            attempt - 1
                        );
                    }
                    return Ok(Self { pool });
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    last_error = Some(e);

                    // Log at appropriate level based on attempt
                    if attempt == 1 {
                        tracing::info!(
                            target: "startup",
                            event = "db_connect_initial_failed",
                            error = %error_msg,
                            "Initial database connection failed, starting retry loop..."
                        );
                    } else if attempt < MAX_RETRY_ATTEMPTS {
                        tracing::debug!(
                            target: "startup",
                            event = "db_connect_retry",
                            attempt = attempt,
                            backoff_ms = backoff_ms,
                            error = %error_msg,
                            "Database connection attempt {} failed, retrying in {}ms...",
                            attempt,
                            backoff_ms
                        );
                    }

                    if attempt < MAX_RETRY_ATTEMPTS {
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        // Exponential backoff with cap
                        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
                    }
                }
            }
        }

        // All retries exhausted
        let final_error = last_error.expect("last_error must be set if we exhausted retries");
        tracing::error!(
            target: "startup",
            event = "db_connect_failed_fatal",
            max_attempts = MAX_RETRY_ATTEMPTS,
            error = %final_error,
            "FATAL: Database connection failed after {} attempts. \
             The database may be unavailable or misconfigured.",
            MAX_RETRY_ATTEMPTS
        );
        Err(final_error)
    }

    /// Single connection attempt (extracted for retry logic clarity)
    async fn try_connect(config: &Config) -> Result<PgPool> {
        // ICT 11+ FIX: Increased pool size for production load
        // 10 was causing connection starvation at scale
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&config.database_url)
            .await?;

        // Verify connection with a simple query (catches auth/db-exists issues)
        sqlx::query("SELECT 1").fetch_one(&pool).await?;

        Ok(pool)
    }

    /// Initialize an empty database atomically, then apply forward migrations.
    /// Existing schemas are never baselined or marked applied automatically.
    pub async fn migrate(&self) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        // Serialize first boot across API replicas. Transaction-scoped locking
        // releases even if initialization fails or the process disconnects.
        sqlx::query("SELECT pg_advisory_xact_lock(728194630126)")
            .execute(&mut *tx)
            .await?;
        let has_schema: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables
             WHERE table_schema = 'public' AND table_name <> '_sqlx_migrations')",
        )
        .fetch_one(&mut *tx)
        .await?;
        if !has_schema {
            sqlx::raw_sql(include_str!("../../migrations/schema.sql"))
                .execute(&mut *tx)
                .await
                .context("Failed to initialize empty database schema")?;
            // The schema dump clears search_path. Restore it before seeding
            // the historical ledger and returning this connection to the pool.
            sqlx::query("SELECT pg_catalog.set_config('search_path', 'public', false)")
                .execute(&mut *tx)
                .await?;
            sqlx::raw_sql(include_str!("../../migrations/sqlx-seed.sql"))
                .execute(&mut *tx)
                .await
                .context("Failed to initialize baseline migration ledger")?;
        }
        tx.commit().await?;
        // SQLx can retain a session advisory lock when migration validation
        // returns early (for example a checksum mismatch). Never return that
        // connection to the pool: close the dedicated connection on both paths.
        let mut connection = self.pool.acquire().await?.detach();
        let migration_result = sqlx::migrate!("./migrations")
            .run_direct(None, &mut connection, false)
            .await;
        let close_result = connection.close().await;
        migration_result.context("Required database migrations failed; API startup aborted")?;
        close_result?;
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

        // ICT 7+: Config-time validation - catch malformed hashes at startup, not runtime
        if !password_hash.starts_with("$argon2id$") {
            tracing::error!(
                target: "security_audit",
                event = "developer_bootstrap_invalid_hash",
                "ICT 7: DEVELOPER_BOOTSTRAP_PASSWORD_HASH must be an Argon2id hash (starts with $argon2id$)"
            );
            return Err(anyhow::anyhow!(
                "Invalid DEVELOPER_BOOTSTRAP_PASSWORD_HASH format. Expected Argon2id hash. \
                 Use 'cargo run --bin bootstrap_dev' to generate a proper hash."
            ));
        }

        // Validate hash can be parsed
        if PasswordHash::new(&password_hash).is_err() {
            tracing::error!(
                target: "security_audit",
                event = "developer_bootstrap_malformed_hash",
                "ICT 7: DEVELOPER_BOOTSTRAP_PASSWORD_HASH is malformed and cannot be parsed"
            );
            return Err(anyhow::anyhow!(
                "Malformed DEVELOPER_BOOTSTRAP_PASSWORD_HASH. The hash cannot be parsed. \
                 Use 'cargo run --bin bootstrap_dev' to generate a valid hash."
            ));
        }

        let name = config
            .developer_bootstrap_name
            .clone()
            .unwrap_or_else(|| "Developer".to_string());

        // Upsert developer account
        let result = sqlx::query(r"
            INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
            VALUES ($1, $2, $3, 'developer', NOW() AT TIME ZONE 'UTC', NOW() AT TIME ZONE 'UTC', NOW() AT TIME ZONE 'UTC')
            ON CONFLICT (email) DO UPDATE SET
                password_hash = EXCLUDED.password_hash,
                role = 'developer',
                email_verified_at = COALESCE(users.email_verified_at, NOW() AT TIME ZONE 'UTC'),
                updated_at = NOW() AT TIME ZONE 'UTC'
            RETURNING id
        ")
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
