//! Token Blacklist Service
//!
//! ICT 7 SECURITY: Token revocation for logout
//! Apple Principal Engineer Grade: Defense in depth
//!
//! SECURITY FEATURES:
//! - Revoke tokens on logout (prevents session hijacking)
//! - In-memory storage with TTL cleanup
//! - Thread-safe concurrent access
//! - Graceful degradation (fails closed)

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

/// Token blacklist entry
#[derive(Clone)]
pub struct BlacklistEntry {
    /// When the token expires (no need to keep after expiry)
    expires_at: Instant,
}

/// In-memory token blacklist
/// ICT 7: Tokens are stored until their natural expiry
pub type TokenBlacklist = Arc<RwLock<HashMap<String, BlacklistEntry>>>;

/// Create a new token blacklist
pub fn new_token_blacklist() -> TokenBlacklist {
    Arc::new(RwLock::new(HashMap::new()))
}

/// ICT 7 SECURITY: Revoke a token by adding it to the blacklist
///
/// # Arguments
/// * `blacklist` - The token blacklist
/// * `token` - The JWT token to revoke
/// * `expires_in_secs` - How long until the token naturally expires
pub async fn revoke_token(blacklist: &TokenBlacklist, token: &str, expires_in_secs: u64) {
    let mut blacklist = blacklist.write().await;

    // Add token to blacklist with expiry time
    blacklist.insert(
        token.to_string(),
        BlacklistEntry {
            expires_at: Instant::now() + Duration::from_secs(expires_in_secs),
        },
    );

    tracing::debug!(
        target: "security",
        "Token revoked and added to blacklist"
    );
}

/// ICT 7 SECURITY: Check if a token is revoked
pub async fn is_token_revoked(blacklist: &TokenBlacklist, token: &str) -> bool {
    let blacklist = blacklist.read().await;

    if let Some(entry) = blacklist.get(token) {
        // Check if entry has expired
        if Instant::now() > entry.expires_at {
            // Entry expired, token is not actively revoked (but may have naturally expired)
            return false;
        }
        // Token is actively revoked
        true
    } else {
        false
    }
}

/// ICT 7: Cleanup expired tokens from blacklist
/// Should be called periodically to prevent memory growth
pub async fn cleanup_expired_tokens(blacklist: &TokenBlacklist) {
    let mut blacklist = blacklist.write().await;
    let now = Instant::now();

    let before_count = blacklist.len();
    blacklist.retain(|_, entry| entry.expires_at > now);
    let after_count = blacklist.len();

    if before_count != after_count {
        tracing::debug!(
            target: "security",
            removed = before_count - after_count,
            remaining = after_count,
            "Cleaned up expired tokens from blacklist"
        );
    }
}

/// ICT 7: Start background cleanup task
pub fn start_cleanup_task(blacklist: TokenBlacklist) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes

        loop {
            interval.tick().await;
            cleanup_expired_tokens(&blacklist).await;
        }
    });
}
