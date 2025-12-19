//! Redis service - Upstash connection
//! ICT L11+ Security: Session management, rate limiting, account lockout

use anyhow::Result;
use redis::{aio::ConnectionManager, AsyncCommands};
use serde::{Deserialize, Serialize};

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

        let key = format!("{}{}", SESSION_PREFIX, session_id);
        let value = serde_json::to_string(&session)?;
        self.set(&key, &value, Some(SESSION_TTL_SECONDS)).await?;

        // Also track user's active sessions
        let user_sessions_key = format!("user_sessions:{}", user_id);
        let mut conn = self.conn.clone();
        let _: () = conn.sadd(&user_sessions_key, session_id).await?;
        let _: () = conn.expire(&user_sessions_key, SESSION_TTL_SECONDS as i64).await?;

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
        let key = format!("{}{}", SESSION_PREFIX, session_id);
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
        let key = format!("{}{}", SESSION_PREFIX, session_id);
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
        let key = format!("{}{}", SESSION_PREFIX, session_id);
        
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
        let user_sessions_key = format!("user_sessions:{}", user_id);
        let mut conn = self.conn.clone();
        
        let session_ids: Vec<String> = conn.smembers(&user_sessions_key).await?;
        let count = session_ids.len() as u32;

        for session_id in session_ids {
            let key = format!("{}{}", SESSION_PREFIX, session_id);
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

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: LOGIN RATE LIMITING & ACCOUNT LOCKOUT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check if login is allowed and track attempt
    /// Implements progressive delays and account lockout
    pub async fn check_login_rate_limit(&self, identifier: &str) -> Result<RateLimitResult> {
        let key = format!("{}{}", LOGIN_ATTEMPTS_PREFIX, identifier);
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
            self.set(&key, &value, Some(LOGIN_WINDOW_SECONDS + LOCKOUT_DURATION_SECONDS as u64)).await?;

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
            0..=3 => 0,      // No delay for first 3 attempts
            4..=6 => 5,      // 5 second delay
            7..=9 => 30,     // 30 second delay
            _ => 60,         // 60 second delay
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
        let key = format!("{}{}", LOGIN_ATTEMPTS_PREFIX, identifier);
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
        let key = format!("{}{}", LOGIN_ATTEMPTS_PREFIX, identifier);
        self.delete(&key).await?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: TOKEN BLACKLIST (for logout)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Add a token to the blacklist (for logout)
    pub async fn blacklist_token(&self, token_hash: &str, expires_in: u64) -> Result<()> {
        let key = format!("{}{}", TOKEN_BLACKLIST_PREFIX, token_hash);
        self.set(&key, "1", Some(expires_in)).await?;
        Ok(())
    }

    /// Check if a token is blacklisted
    pub async fn is_token_blacklisted(&self, token_hash: &str) -> Result<bool> {
        let key = format!("{}{}", TOKEN_BLACKLIST_PREFIX, token_hash);
        Ok(self.get(&key).await?.is_some())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ICT L11+ SECURITY: GLOBAL RATE LIMITING
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check global rate limit for an IP address
    pub async fn check_ip_rate_limit(&self, ip: &str, max_requests: i64, window_seconds: u64) -> Result<RateLimitResult> {
        let key = format!("{}ip:{}", RATE_LIMIT_PREFIX, ip);
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
            retry_after: if allowed { None } else { Some(window_seconds as i64) },
            locked: false,
        })
    }
}
