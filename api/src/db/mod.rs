//! Database module - Fly.io PostgreSQL connection

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
}
