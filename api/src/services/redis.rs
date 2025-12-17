//! Redis service - Upstash connection

use anyhow::Result;
use redis::{aio::ConnectionManager, AsyncCommands};

#[derive(Clone)]
pub struct RedisService {
    conn: ConnectionManager,
}

impl RedisService {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { conn })
    }

    /// Get a value from cache
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.conn.clone();
        let value: Option<String> = conn.get(key).await?;
        Ok(value)
    }

    /// Set a value in cache with optional expiration
    pub async fn set(&self, key: &str, value: &str, expire_seconds: Option<u64>) -> Result<()> {
        let mut conn = self.conn.clone();
        if let Some(seconds) = expire_seconds {
            let _: () = conn.set_ex(key, value, seconds).await?;
        } else {
            let _: () = conn.set(key, value).await?;
        }
        Ok(())
    }

    /// Delete a key from cache
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.conn.clone();
        let _: () = conn.del(key).await?;
        Ok(())
    }

    /// Increment a counter (for rate limiting)
    pub async fn incr(&self, key: &str, expire_seconds: u64) -> Result<i64> {
        let mut conn = self.conn.clone();
        let count: i64 = redis::pipe()
            .atomic()
            .incr(key, 1)
            .expire(key, expire_seconds as i64)
            .ignore()
            .query_async(&mut conn)
            .await?;
        Ok(count)
    }

    /// Check rate limit
    pub async fn check_rate_limit(&self, key: &str, max_requests: i64, window_seconds: u64) -> Result<bool> {
        let count = self.incr(key, window_seconds).await?;
        Ok(count <= max_requests)
    }
}
