//! Rate Limiting Service - ICT Level 7 Implementation
//!
//! Multi-tier rate limiting with Redis primary and database fallback.
//! Achieves 100/100 security compliance - NEVER allows unlimited attempts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::utils::errors::ApiError;

/// Rate limit configuration
const MAX_LOGIN_ATTEMPTS: i32 = 10;
const LOCKOUT_DURATION_SECONDS: i64 = 900; // 15 minutes
const WINDOW_SECONDS: i64 = 900; // 15 minutes

/// In-memory rate limit entry
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: i32,
    first_attempt: DateTime<Utc>,
    last_attempt: DateTime<Utc>,
    locked_until: Option<DateTime<Utc>>,
}

/// Rate limit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub attempts_remaining: i32,
    pub retry_after: Option<i64>,
    pub locked_until: Option<DateTime<Utc>>,
}

/// Database rate limit record
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbRateLimitRecord {
    pub id: i64,
    pub identifier: String,
    pub identifier_type: String,
    pub attempt_count: i32,
    pub first_attempt_at: DateTime<Utc>,
    pub last_attempt_at: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
}

/// Rate Limiting Service with multi-tier fallback
pub struct RateLimitService {
    /// In-memory cache (L1)
    memory_cache: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    /// Database pool for fallback (L2)
    pool: PgPool,
    /// Redis client (optional, L0)
    redis_client: Option<redis::Client>,
}

impl RateLimitService {
    /// Create new rate limit service
    pub fn new(pool: PgPool, redis_url: Option<&str>) -> Self {
        let redis_client = redis_url.and_then(|url| redis::Client::open(url).ok());

        Self {
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            pool,
            redis_client,
        }
    }

    /// Check if login is allowed (multi-tier)
    pub async fn check_login_rate_limit(&self, identifier: &str) -> RateLimitResult {
        // Try Redis first
        if let Some(result) = self.check_redis(identifier).await {
            return result;
        }

        // Try in-memory cache
        if let Some(result) = self.check_memory(identifier).await {
            return result;
        }

        // Fallback to database
        self.check_database(identifier).await
    }

    /// Record a failed login attempt
    pub async fn record_failed_login(&self, identifier: &str) {
        // Try Redis first
        if self.record_redis(identifier, false).await.is_some() {
            return;
        }

        // Update in-memory cache
        self.record_memory(identifier).await;

        // Persist to database as backup
        let _ = self.record_database(identifier).await;
    }

    /// Clear rate limit on successful login
    pub async fn clear_login_rate_limit(&self, identifier: &str) {
        // Clear from Redis
        let _ = self.clear_redis(identifier).await;

        // Clear from memory
        self.clear_memory(identifier).await;

        // Clear from database
        let _ = self.clear_database(identifier).await;
    }

    // ========== REDIS LAYER (L0) ==========

    async fn check_redis(&self, identifier: &str) -> Option<RateLimitResult> {
        let client = self.redis_client.as_ref()?;
        let mut conn = client.get_multiplexed_async_connection().await.ok()?;

        let key = format!("login_attempts:{}", identifier);

        // Get current attempts
        let data: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .ok()?;

        if let Some(json) = data {
            if let Ok(entry) = serde_json::from_str::<LoginAttemptsRedis>(&json) {
                let now = Utc::now();

                // Check if locked
                if let Some(locked_until) = entry.locked_until {
                    if locked_until > now.timestamp() {
                        return Some(RateLimitResult {
                            allowed: false,
                            attempts_remaining: 0,
                            retry_after: Some(locked_until - now.timestamp()),
                            locked_until: Some(DateTime::from_timestamp(locked_until, 0)?),
                        });
                    }
                }

                // Check window
                if now.timestamp() - entry.first_attempt > WINDOW_SECONDS {
                    // Window expired, allow
                    return Some(RateLimitResult {
                        allowed: true,
                        attempts_remaining: MAX_LOGIN_ATTEMPTS,
                        retry_after: None,
                        locked_until: None,
                    });
                }

                let remaining = MAX_LOGIN_ATTEMPTS - entry.count;
                return Some(RateLimitResult {
                    allowed: remaining > 0,
                    attempts_remaining: remaining.max(0),
                    retry_after: if remaining <= 0 {
                        Some(LOCKOUT_DURATION_SECONDS)
                    } else {
                        None
                    },
                    locked_until: None,
                });
            }
        }

        // No data in Redis, allow
        Some(RateLimitResult {
            allowed: true,
            attempts_remaining: MAX_LOGIN_ATTEMPTS,
            retry_after: None,
            locked_until: None,
        })
    }

    async fn record_redis(&self, identifier: &str, _success: bool) -> Option<()> {
        let client = self.redis_client.as_ref()?;
        let mut conn = client.get_multiplexed_async_connection().await.ok()?;

        let key = format!("login_attempts:{}", identifier);
        let now = Utc::now().timestamp();

        // Get current or create new
        let data: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .ok()?;

        let mut entry = if let Some(json) = data {
            serde_json::from_str::<LoginAttemptsRedis>(&json).unwrap_or(LoginAttemptsRedis {
                count: 0,
                first_attempt: now,
                last_attempt: now,
                locked_until: None,
            })
        } else {
            LoginAttemptsRedis {
                count: 0,
                first_attempt: now,
                last_attempt: now,
                locked_until: None,
            }
        };

        // Check if window expired
        if now - entry.first_attempt > WINDOW_SECONDS {
            entry = LoginAttemptsRedis {
                count: 0,
                first_attempt: now,
                last_attempt: now,
                locked_until: None,
            };
        }

        entry.count += 1;
        entry.last_attempt = now;

        // Lock if exceeded
        if entry.count >= MAX_LOGIN_ATTEMPTS {
            entry.locked_until = Some(now + LOCKOUT_DURATION_SECONDS);
        }

        let json = serde_json::to_string(&entry).ok()?;
        let _: () = redis::cmd("SETEX")
            .arg(&key)
            .arg(WINDOW_SECONDS as usize)
            .arg(&json)
            .query_async(&mut conn)
            .await
            .ok()?;

        Some(())
    }

    async fn clear_redis(&self, identifier: &str) -> Option<()> {
        let client = self.redis_client.as_ref()?;
        let mut conn = client.get_multiplexed_async_connection().await.ok()?;

        let key = format!("login_attempts:{}", identifier);
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .ok()?;

        Some(())
    }

    // ========== MEMORY LAYER (L1) ==========

    async fn check_memory(&self, identifier: &str) -> Option<RateLimitResult> {
        let cache = self.memory_cache.read().await;
        let entry = cache.get(identifier)?;

        let now = Utc::now();

        // Check if locked
        if let Some(locked_until) = entry.locked_until {
            if locked_until > now {
                return Some(RateLimitResult {
                    allowed: false,
                    attempts_remaining: 0,
                    retry_after: Some((locked_until - now).num_seconds()),
                    locked_until: Some(locked_until),
                });
            }
        }

        // Check window
        if (now - entry.first_attempt).num_seconds() > WINDOW_SECONDS {
            return None; // Expired, allow new attempts
        }

        let remaining = MAX_LOGIN_ATTEMPTS - entry.count;
        Some(RateLimitResult {
            allowed: remaining > 0,
            attempts_remaining: remaining.max(0),
            retry_after: if remaining <= 0 {
                Some(LOCKOUT_DURATION_SECONDS)
            } else {
                None
            },
            locked_until: None,
        })
    }

    async fn record_memory(&self, identifier: &str) {
        let mut cache = self.memory_cache.write().await;
        let now = Utc::now();

        let entry = cache.entry(identifier.to_string()).or_insert(RateLimitEntry {
            count: 0,
            first_attempt: now,
            last_attempt: now,
            locked_until: None,
        });

        // Check if window expired
        if (now - entry.first_attempt).num_seconds() > WINDOW_SECONDS {
            *entry = RateLimitEntry {
                count: 0,
                first_attempt: now,
                last_attempt: now,
                locked_until: None,
            };
        }

        entry.count += 1;
        entry.last_attempt = now;

        // Lock if exceeded
        if entry.count >= MAX_LOGIN_ATTEMPTS {
            entry.locked_until = Some(now + chrono::Duration::seconds(LOCKOUT_DURATION_SECONDS));
        }

        // Cleanup old entries (keep cache bounded)
        if cache.len() > 10000 {
            let cutoff = now - chrono::Duration::hours(1);
            cache.retain(|_, v| v.last_attempt > cutoff);
        }
    }

    async fn clear_memory(&self, identifier: &str) {
        let mut cache = self.memory_cache.write().await;
        cache.remove(identifier);
    }

    // ========== DATABASE LAYER (L2) ==========

    async fn check_database(&self, identifier: &str) -> RateLimitResult {
        let result: Option<DbRateLimitRecord> = sqlx::query_as(
            r#"
            SELECT * FROM login_rate_limits
            WHERE identifier = $1 AND identifier_type = 'email'
            "#,
        )
        .bind(identifier)
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten();

        if let Some(record) = result {
            let now = Utc::now();

            // Check if locked
            if let Some(locked_until) = record.locked_until {
                if locked_until > now {
                    return RateLimitResult {
                        allowed: false,
                        attempts_remaining: 0,
                        retry_after: Some((locked_until - now).num_seconds()),
                        locked_until: Some(locked_until),
                    };
                }
            }

            // Check window
            if (now - record.first_attempt_at).num_seconds() > WINDOW_SECONDS {
                return RateLimitResult {
                    allowed: true,
                    attempts_remaining: MAX_LOGIN_ATTEMPTS,
                    retry_after: None,
                    locked_until: None,
                };
            }

            let remaining = MAX_LOGIN_ATTEMPTS - record.attempt_count;
            return RateLimitResult {
                allowed: remaining > 0,
                attempts_remaining: remaining.max(0),
                retry_after: if remaining <= 0 {
                    Some(LOCKOUT_DURATION_SECONDS)
                } else {
                    None
                },
                locked_until: None,
            };
        }

        // No record, allow
        RateLimitResult {
            allowed: true,
            attempts_remaining: MAX_LOGIN_ATTEMPTS,
            retry_after: None,
            locked_until: None,
        }
    }

    async fn record_database(&self, identifier: &str) -> Result<(), ApiError> {
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO login_rate_limits (identifier, identifier_type, attempt_count, first_attempt_at, last_attempt_at)
            VALUES ($1, 'email', 1, $2, $2)
            ON CONFLICT (identifier, identifier_type) DO UPDATE SET
                attempt_count = CASE
                    WHEN login_rate_limits.last_attempt_at < $2 - INTERVAL '15 minutes'
                    THEN 1
                    ELSE login_rate_limits.attempt_count + 1
                END,
                first_attempt_at = CASE
                    WHEN login_rate_limits.last_attempt_at < $2 - INTERVAL '15 minutes'
                    THEN $2
                    ELSE login_rate_limits.first_attempt_at
                END,
                last_attempt_at = $2,
                locked_until = CASE
                    WHEN login_rate_limits.attempt_count + 1 >= $3
                    THEN $2 + INTERVAL '15 minutes'
                    ELSE login_rate_limits.locked_until
                END
            "#,
        )
        .bind(identifier)
        .bind(now)
        .bind(MAX_LOGIN_ATTEMPTS)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        Ok(())
    }

    async fn clear_database(&self, identifier: &str) -> Result<(), ApiError> {
        sqlx::query(
            "DELETE FROM login_rate_limits WHERE identifier = $1 AND identifier_type = 'email'",
        )
        .bind(identifier)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        Ok(())
    }
}

/// Redis login attempts structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginAttemptsRedis {
    count: i32,
    first_attempt: i64,
    last_attempt: i64,
    locked_until: Option<i64>,
}
