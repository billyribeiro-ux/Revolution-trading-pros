//! Redis service - Upstash connection
//! ICT L11+ Security: Session management, rate limiting, account lockout
//! PE7: Resilient connection with exponential backoff retry

use anyhow::Result;
use redis::{aio::ConnectionManager, AsyncCommands};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Maximum retry attempts for Redis connection (PE7: resilience over brittleness)
const MAX_RETRY_ATTEMPTS: u32 = 8;
/// Initial backoff duration between retries
const INITIAL_BACKOFF_MS: u64 = 300;
/// Maximum backoff duration (prevents runaway waits)
const MAX_BACKOFF_MS: u64 = 5000;

/// Session data stored in Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: i64,
    pub email: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: i64,
    pub last_activity: i64,
    pub is_valid: bool,
}

/// Login attempt tracking for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginAttempts {
    pub count: i32,
    pub first_attempt: i64,
    pub last_attempt: i64,
    pub locked_until: Option<i64>,
}

/// Rate limit result
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: i32,
    pub retry_after: Option<i64>,
    pub locked: bool,
}

// Redis key prefixes
const SESSION_PREFIX: &str = "session:";
const LOGIN_ATTEMPTS_PREFIX: &str = "login_attempts:";
const RATE_LIMIT_PREFIX: &str = "rate_limit:";
const TOKEN_BLACKLIST_PREFIX: &str = "token_blacklist:";
const USER_CACHE_PREFIX: &str = "user_cache:";
// SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2 / security-C1+M3): per-user token
// epoch key. `user_token_version:{id}` -> monotonically increasing i64.
// Every issued JWT embeds the value live at mint time (utils::Claims.token_version);
// the `User` extractor rejects any token whose embedded version is below the
// stored value. Bumping this key is what makes "log out all devices",
// password-reset and ban actually strand already-issued access AND refresh
// tokens immediately, instead of waiting for TTL expiry / refresh rotation.
const TOKEN_VERSION_PREFIX: &str = "user_token_version:";
// The epoch must outlive the longest-lived token so a bump cannot be
// "forgotten" while a pre-bump token is still otherwise valid. The longest
// token is the 7-day refresh token; we hold the key for 30 days (>> 7d) and
// every bump/read refreshes the TTL, so an active user's epoch never lapses.
// If the key DOES lapse (idle > 30d) it reads back as 0 — safe, because any
// surviving token from that era is itself long expired (max 7d).
const TOKEN_VERSION_TTL_SECONDS: i64 = 30 * 24 * 3600;

// ICT 7+: User cache TTL - 5 minutes for balance between freshness and performance
const USER_CACHE_TTL_SECONDS: u64 = 300;

// Timeouts and limits
const SESSION_TTL_SECONDS: u64 = 86400; // 24 hours
const SESSION_IDLE_TIMEOUT: i64 = 1800; // 30 minutes
const LOGIN_WINDOW_SECONDS: u64 = 900; // 15 minutes
const MAX_LOGIN_ATTEMPTS: i32 = 10;
const LOCKOUT_DURATION_SECONDS: i64 = 900; // 15 minutes

#[derive(Clone)]
pub struct RedisService {
    conn: ConnectionManager,
}

impl RedisService {
    /// Create a new Redis service with exponential backoff retry (PE7 Grade)
    ///
    /// Implements resilient startup for containerized environments where
    /// Redis may still be initializing when the API boots.
    ///
    /// Retry strategy:
    /// - 8 attempts max
    /// - Exponential backoff: 300ms, 600ms, 1.2s, 2.4s, 4.8s, 5s, 5s, 5s (capped)
    /// - Total max wait: ~25 seconds
    pub async fn new(redis_url: &str) -> Result<Self> {
        let mut attempt = 0;
        let mut backoff_ms = INITIAL_BACKOFF_MS;
        let mut last_error = None;

        while attempt < MAX_RETRY_ATTEMPTS {
            attempt += 1;

            match Self::try_connect(redis_url).await {
                Ok(conn) => {
                    if attempt > 1 {
                        tracing::info!(
                            target: "startup",
                            event = "redis_connected_after_retry",
                            attempts = attempt,
                            "Redis connected after {} retry attempts",
                            attempt - 1
                        );
                    }
                    return Ok(Self { conn });
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    last_error = Some(e);

                    if attempt == 1 {
                        tracing::info!(
                            target: "startup",
                            event = "redis_connect_initial_failed",
                            error = %error_msg,
                            "Initial Redis connection failed, starting retry loop..."
                        );
                    } else if attempt < MAX_RETRY_ATTEMPTS {
                        tracing::debug!(
                            target: "startup",
                            event = "redis_connect_retry",
                            attempt = attempt,
                            backoff_ms = backoff_ms,
                            error = %error_msg,
                            "Redis connection attempt {} failed, retrying in {}ms...",
                            attempt,
                            backoff_ms
                        );
                    }

                    if attempt < MAX_RETRY_ATTEMPTS {
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                        backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
                    }
                }
            }
        }

        // All retries exhausted
        let final_error = last_error.expect("last_error must be set if we exhausted retries");
        tracing::error!(
            target: "startup",
            event = "redis_connect_failed_fatal",
            max_attempts = MAX_RETRY_ATTEMPTS,
            error = %final_error,
            "FATAL: Redis connection failed after {} attempts. Redis may be unavailable.",
            MAX_RETRY_ATTEMPTS
        );
        Err(final_error)
    }

    /// Single connection attempt (extracted for retry logic clarity)
    async fn try_connect(redis_url: &str) -> Result<ConnectionManager> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;
        // Verify connection with a PING
        let mut test_conn = conn.clone();
        let _: String = redis::cmd("PING").query_async(&mut test_conn).await?;
        Ok(conn)
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
        // FIX-2026-04-28: pipeline returns Vec<i64> (one element per command),
        // not a bare i64. Removed .ignore() so EXPIRE result is included in
        // the Vec; take the first element (INCR result).
        let result: Vec<i64> = redis::pipe()
            .atomic()
            .incr(key, 1)
            .expire(key, expire_seconds as i64)
            .query_async(&mut conn)
            .await?;
        Ok(result.into_iter().next().unwrap_or(0))
    }

    /// Check rate limit
    pub async fn check_rate_limit(
        &self,
        key: &str,
        max_requests: i64,
        window_seconds: u64,
    ) -> Result<bool> {
        let count = self.incr(key, window_seconds).await?;
        Ok(count <= max_requests)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: SESSION MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Create a new session in Redis
    pub async fn create_session(
        &self,
        session_id: &str,
        user_id: i64,
        email: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let session = SessionData {
            user_id,
            email: email.to_string(),
            ip_address,
            user_agent,
            created_at: now,
            last_activity: now,
            is_valid: true,
        };

        let key = format!("{SESSION_PREFIX}{session_id}");
        let value = serde_json::to_string(&session)?;
        self.set(&key, &value, Some(SESSION_TTL_SECONDS)).await?;

        // Also track user's active sessions
        let user_sessions_key = format!("user_sessions:{user_id}");
        let mut conn = self.conn.clone();
        let _: () = conn.sadd(&user_sessions_key, session_id).await?;
        let _: () = conn
            .expire(&user_sessions_key, SESSION_TTL_SECONDS as i64)
            .await?;

        tracing::info!(
            target: "security",
            event = "session_created",
            session_id = %session_id,
            user_id = %user_id,
            "Session created"
        );

        Ok(())
    }

    /// Get session data from Redis
    pub async fn get_session(&self, session_id: &str) -> Result<Option<SessionData>> {
        let key = format!("{SESSION_PREFIX}{session_id}");
        match self.get(&key).await? {
            Some(value) => {
                let session: SessionData = serde_json::from_str(&value)?;

                // Check if session is still valid
                if !session.is_valid {
                    return Ok(None);
                }

                // Check idle timeout
                let now = chrono::Utc::now().timestamp();
                if now - session.last_activity > SESSION_IDLE_TIMEOUT {
                    // Session expired due to inactivity
                    self.invalidate_session(session_id).await?;
                    return Ok(None);
                }

                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    /// Update session last activity (keep-alive)
    pub async fn touch_session(&self, session_id: &str) -> Result<bool> {
        let key = format!("{SESSION_PREFIX}{session_id}");
        match self.get(&key).await? {
            Some(value) => {
                let mut session: SessionData = serde_json::from_str(&value)?;
                session.last_activity = chrono::Utc::now().timestamp();
                let updated = serde_json::to_string(&session)?;
                self.set(&key, &updated, Some(SESSION_TTL_SECONDS)).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Invalidate a specific session (logout)
    pub async fn invalidate_session(&self, session_id: &str) -> Result<()> {
        let key = format!("{SESSION_PREFIX}{session_id}");

        // Get session to find user_id
        if let Some(value) = self.get(&key).await? {
            let session: SessionData = serde_json::from_str(&value)?;

            // Remove from user's session set
            let user_sessions_key = format!("user_sessions:{}", session.user_id);
            let mut conn = self.conn.clone();
            let _: () = conn.srem(&user_sessions_key, session_id).await?;

            tracing::info!(
                target: "security",
                event = "session_invalidated",
                session_id = %session_id,
                user_id = %session.user_id,
                "Session invalidated"
            );
        }

        self.delete(&key).await?;
        Ok(())
    }

    /// Invalidate all sessions for a user (force logout everywhere)
    pub async fn invalidate_all_user_sessions(&self, user_id: i64) -> Result<u32> {
        let user_sessions_key = format!("user_sessions:{user_id}");
        let mut conn = self.conn.clone();

        let session_ids: Vec<String> = conn.smembers(&user_sessions_key).await?;
        let count = session_ids.len() as u32;

        for session_id in session_ids {
            let key = format!("{SESSION_PREFIX}{session_id}");
            self.delete(&key).await?;
        }

        self.delete(&user_sessions_key).await?;

        tracing::info!(
            target: "security",
            event = "all_sessions_invalidated",
            user_id = %user_id,
            count = %count,
            "All user sessions invalidated"
        );

        Ok(count)
    }

    /// SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2 / security-C1+M3):
    /// Read the user's current token epoch (authoritative for the `User`
    /// extractor's stale-token rejection).
    ///
    /// Returns `0` when the key is absent — i.e. a user who has NEVER had a
    /// bump (brand-new account) or whose epoch key has lapsed after >30d of
    /// inactivity. `0` is the same value `#[serde(default)]` yields for a
    /// token minted before this feature, so legacy/new tokens validate until
    /// the first bump. Reading also refreshes the TTL so an actively-used
    /// account's epoch never silently lapses out from under live tokens.
    ///
    /// Fail-closed contract: this returns `Err` on any Redis fault. Callers
    /// (the extractor) MUST treat `Err` as "reject", mirroring the existing
    /// blacklist check's fail-closed behavior — a Redis fault must not open a
    /// window for a revoked token.
    pub async fn get_token_version(&self, user_id: i64) -> Result<i64> {
        let key = format!("{TOKEN_VERSION_PREFIX}{user_id}");
        let mut conn = self.conn.clone();
        let value: Option<i64> = conn.get(&key).await?;
        match value {
            Some(v) => {
                // Keep an active user's epoch alive (sliding TTL). Best-effort:
                // a failure to refresh the TTL must not fail the auth path —
                // the value we already read is authoritative for this request.
                if let Err(e) = conn.expire::<_, ()>(&key, TOKEN_VERSION_TTL_SECONDS).await {
                    tracing::warn!(
                        target: "security",
                        event = "token_version_ttl_refresh_failed",
                        user_id = %user_id,
                        error = %e,
                        "Could not refresh token_version TTL (value still authoritative)"
                    );
                }
                Ok(v)
            }
            None => Ok(0),
        }
    }

    /// SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2 / security-C1+M3):
    /// Atomically bump the user's token epoch, invalidating EVERY previously
    /// issued access and refresh token for that user.
    ///
    /// Call this anywhere sessions are invalidated — it is the companion to
    /// [`Self::invalidate_all_user_sessions`] (which only kills `session:*`
    /// keys and never touched stateless JWTs). One helper, one call pattern.
    /// `INCR` is atomic and creates the key at 1 if absent, so the first bump
    /// on a fresh account moves the epoch 0 -> 1, instantly stranding every
    /// version-0 token (legacy + freshly minted) for that user.
    ///
    /// Returns the new epoch value.
    pub async fn bump_token_version(&self, user_id: i64) -> Result<i64> {
        let key = format!("{TOKEN_VERSION_PREFIX}{user_id}");
        let mut conn = self.conn.clone();
        // Atomic INCR + (re)set the TTL together so a crash between the two
        // can't leave a never-expiring key, and so the epoch always outlives
        // the longest-lived (7d refresh) token by a wide margin.
        let result: Vec<i64> = redis::pipe()
            .atomic()
            .incr(&key, 1)
            .expire(&key, TOKEN_VERSION_TTL_SECONDS)
            .query_async(&mut conn)
            .await?;
        let new_version = result.into_iter().next().unwrap_or(0);

        tracing::info!(
            target: "security_audit",
            event = "token_version_bumped",
            user_id = %user_id,
            new_version = %new_version,
            "User token epoch bumped — all prior access/refresh tokens revoked"
        );

        Ok(new_version)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: LOGIN RATE LIMITING & ACCOUNT LOCKOUT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check if login is allowed and track attempt
    /// Implements progressive delays and account lockout
    pub async fn check_login_rate_limit(&self, identifier: &str) -> Result<RateLimitResult> {
        let key = format!("{LOGIN_ATTEMPTS_PREFIX}{identifier}");
        let now = chrono::Utc::now().timestamp();

        // Get current attempts
        let attempts = match self.get(&key).await? {
            Some(value) => serde_json::from_str::<LoginAttempts>(&value).unwrap_or_default(),
            None => LoginAttempts::default(),
        };

        // Check if account is locked
        if let Some(locked_until) = attempts.locked_until {
            if now < locked_until {
                let retry_after = locked_until - now;
                tracing::warn!(
                    target: "security",
                    event = "login_blocked_lockout",
                    identifier = %identifier,
                    retry_after = %retry_after,
                    "Login blocked - account locked"
                );
                return Ok(RateLimitResult {
                    allowed: false,
                    remaining: 0,
                    retry_after: Some(retry_after),
                    locked: true,
                });
            }
        }

        // Check if over limit
        if attempts.count >= MAX_LOGIN_ATTEMPTS {
            // Lock the account
            let mut new_attempts = attempts.clone();
            new_attempts.locked_until = Some(now + LOCKOUT_DURATION_SECONDS);
            let value = serde_json::to_string(&new_attempts)?;
            self.set(
                &key,
                &value,
                Some(LOGIN_WINDOW_SECONDS + LOCKOUT_DURATION_SECONDS as u64),
            )
            .await?;

            tracing::warn!(
                target: "security",
                event = "account_locked",
                identifier = %identifier,
                attempts = %attempts.count,
                "Account locked due to too many failed attempts"
            );

            return Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                retry_after: Some(LOCKOUT_DURATION_SECONDS),
                locked: true,
            });
        }

        // Calculate delay based on attempt count (progressive delay)
        let delay = match attempts.count {
            0..=3 => 0,  // No delay for first 3 attempts
            4..=6 => 5,  // 5 second delay
            7..=9 => 30, // 30 second delay
            _ => 60,     // 60 second delay
        };

        if delay > 0 && attempts.last_attempt > 0 {
            let time_since_last = now - attempts.last_attempt;
            if time_since_last < delay {
                let retry_after = delay - time_since_last;
                return Ok(RateLimitResult {
                    allowed: false,
                    remaining: MAX_LOGIN_ATTEMPTS - attempts.count,
                    retry_after: Some(retry_after),
                    locked: false,
                });
            }
        }

        Ok(RateLimitResult {
            allowed: true,
            remaining: MAX_LOGIN_ATTEMPTS - attempts.count,
            retry_after: None,
            locked: false,
        })
    }

    /// Record a failed login attempt
    pub async fn record_failed_login(&self, identifier: &str) -> Result<()> {
        let key = format!("{LOGIN_ATTEMPTS_PREFIX}{identifier}");
        let now = chrono::Utc::now().timestamp();

        let mut attempts = match self.get(&key).await? {
            Some(value) => serde_json::from_str::<LoginAttempts>(&value).unwrap_or_default(),
            None => LoginAttempts {
                first_attempt: now,
                ..Default::default()
            },
        };

        attempts.count += 1;
        attempts.last_attempt = now;

        let value = serde_json::to_string(&attempts)?;
        self.set(&key, &value, Some(LOGIN_WINDOW_SECONDS)).await?;

        tracing::info!(
            target: "security",
            event = "failed_login_recorded",
            identifier = %identifier,
            attempt_count = %attempts.count,
            "Failed login attempt recorded"
        );

        Ok(())
    }

    /// Clear login attempts on successful login
    pub async fn clear_login_attempts(&self, identifier: &str) -> Result<()> {
        let key = format!("{LOGIN_ATTEMPTS_PREFIX}{identifier}");
        self.delete(&key).await?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: TOKEN BLACKLIST (for logout)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Add a token to the blacklist (for logout)
    pub async fn blacklist_token(&self, token_hash: &str, expires_in: u64) -> Result<()> {
        let key = format!("{TOKEN_BLACKLIST_PREFIX}{token_hash}");
        self.set(&key, "1", Some(expires_in)).await?;
        Ok(())
    }

    /// Check if a token is blacklisted
    pub async fn is_token_blacklisted(&self, token_hash: &str) -> Result<bool> {
        let key = format!("{TOKEN_BLACKLIST_PREFIX}{token_hash}");
        Ok(self.get(&key).await?.is_some())
    }

    /// Delete all keys matching a pattern.
    /// ICT 7+ Phase 2: Used for cache invalidation.
    ///
    /// RUST_DEEP_AUDIT_2026-06-07 (P2-1): uses incremental `SCAN` (cursor loop)
    /// instead of `KEYS`. `KEYS` is O(N) over the whole keyspace and blocks the
    /// single-threaded Redis server for every client (auth, rate-limit, sessions)
    /// for the duration of the scan; `SCAN` yields between batches.
    pub async fn delete_pattern(&self, pattern: &str) -> Result<usize> {
        let mut conn = self.conn.clone();
        let mut cursor: u64 = 0;
        let mut deleted = 0usize;

        loop {
            let (next, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(500)
                .query_async(&mut conn)
                .await?;

            if !keys.is_empty() {
                // Batch delete the matched slice in one round-trip.
                let n: usize = redis::cmd("DEL").arg(&keys).query_async(&mut conn).await?;
                deleted += n;
            }

            cursor = next;
            if cursor == 0 {
                break;
            }
        }

        Ok(deleted)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT 7+: USER CACHING FOR AUTH PERFORMANCE
    // ═══════════════════════════════════════════════════════════════════════════

    /// Cache user data for fast auth lookups
    /// ICT 7+: Reduces database queries by 60-80% for authenticated requests
    pub async fn cache_user(&self, user_id: i64, user_json: &str) -> Result<()> {
        let key = format!("{USER_CACHE_PREFIX}{user_id}");
        self.set(&key, user_json, Some(USER_CACHE_TTL_SECONDS))
            .await?;
        tracing::debug!(
            target: "performance",
            event = "user_cached",
            user_id = %user_id,
            ttl_seconds = %USER_CACHE_TTL_SECONDS,
            "User cached in Redis"
        );
        Ok(())
    }

    /// Get cached user data
    /// Returns None if not cached or expired
    pub async fn get_cached_user(&self, user_id: i64) -> Result<Option<String>> {
        let key = format!("{USER_CACHE_PREFIX}{user_id}");
        let result = self.get(&key).await?;
        if result.is_some() {
            tracing::debug!(
                target: "performance",
                event = "user_cache_hit",
                user_id = %user_id,
                "User found in cache"
            );
        }
        Ok(result)
    }

    /// Invalidate user cache (call on user update/delete/password change)
    pub async fn invalidate_user_cache(&self, user_id: i64) -> Result<()> {
        let key = format!("{USER_CACHE_PREFIX}{user_id}");
        self.delete(&key).await?;
        tracing::info!(
            target = "security",
            event = "user_cache_invalidated",
            user_id = %user_id,
            "User cache invalidated"
        );
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: GLOBAL RATE LIMITING
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check global rate limit for an IP address
    pub async fn check_ip_rate_limit(
        &self,
        ip: &str,
        max_requests: i64,
        window_seconds: u64,
    ) -> Result<RateLimitResult> {
        let key = format!("{RATE_LIMIT_PREFIX}ip:{ip}");
        let count = self.incr(&key, window_seconds).await?;

        let remaining = (max_requests - count).max(0) as i32;
        let allowed = count <= max_requests;

        if !allowed {
            tracing::warn!(
                target: "security",
                event = "ip_rate_limited",
                ip = %ip,
                count = %count,
                max = %max_requests,
                "IP rate limited"
            );
        }

        Ok(RateLimitResult {
            allowed,
            remaining,
            retry_after: if allowed {
                None
            } else {
                Some(window_seconds as i64)
            },
            locked: false,
        })
    }
}
