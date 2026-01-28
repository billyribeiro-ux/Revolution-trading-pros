//! Cache Key Management - Type-Safe Key Generation
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Centralized cache key definitions ensure consistency across the application.
//! All keys follow a hierarchical namespace pattern: `{prefix}:{entity}:{room}:{identifier}`
//!
//! ## Key Naming Convention
//!
//! - Keys are namespaced to prevent collisions
//! - Room-specific data includes room_slug for isolation
//! - Pagination parameters are encoded in keys for proper cache segmentation
//!
//! ## TTL Strategy
//!
//! TTLs are chosen based on data characteristics:
//! - **Frequently updated data** (alerts): 60 seconds
//! - **Moderately updated data** (trades, stats): 5 minutes
//! - **Rarely updated data** (trade plans): 10 minutes
//! - **Static data** (weekly videos): 1 hour

use std::time::Duration;

/// Cache key version - increment to invalidate all cache entries on schema changes
const KEY_VERSION: &str = "v1";

/// Root prefix for all Explosive Swings cache keys
const CACHE_PREFIX: &str = "es";

/// Cache key generation functions
///
/// All functions return deterministic, URL-safe cache keys.
/// Keys include version prefix for easy cache invalidation on schema changes.
pub mod cache_keys {
    use super::{CACHE_PREFIX, KEY_VERSION};

    /// Generate cache key for paginated alerts list
    ///
    /// # Arguments
    /// * `room_slug` - The room identifier (e.g., "explosive-swings")
    /// * `page` - Current page number
    /// * `limit` - Items per page
    ///
    /// # Example
    /// ```ignore
    /// let key = cache_keys::alerts("explosive-swings", 1, 20);
    /// // Returns: "es:v1:alerts:explosive-swings:p1:l20"
    /// ```
    #[inline]
    pub fn alerts(room_slug: &str, page: i64, limit: i64) -> String {
        format!(
            "{}:{}:alerts:{}:p{}:l{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, page, limit
        )
    }

    /// Generate cache key for a single alert
    ///
    /// # Arguments
    /// * `room_slug` - The room identifier
    /// * `alert_id` - The alert's database ID
    #[inline]
    pub fn alert(room_slug: &str, alert_id: i64) -> String {
        format!(
            "{}:{}:alert:{}:{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, alert_id
        )
    }

    /// Generate cache key for trades list
    ///
    /// # Arguments
    /// * `room_slug` - The room identifier
    /// * `status` - Optional status filter (e.g., "open", "closed")
    /// * `page` - Current page number
    /// * `limit` - Items per page
    #[inline]
    pub fn trades(room_slug: &str, status: Option<&str>, page: i64, limit: i64) -> String {
        let status_part = status.unwrap_or("all");
        format!(
            "{}:{}:trades:{}:s{}:p{}:l{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, status_part, page, limit
        )
    }

    /// Generate cache key for a single trade
    #[inline]
    pub fn trade(room_slug: &str, trade_id: i64) -> String {
        format!(
            "{}:{}:trade:{}:{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, trade_id
        )
    }

    /// Generate cache key for trade plans list
    ///
    /// # Arguments
    /// * `room_slug` - The room identifier
    /// * `week_of` - Optional week filter in YYYY-MM-DD format
    /// * `page` - Current page number
    /// * `limit` - Items per page
    #[inline]
    pub fn trade_plans(room_slug: &str, week_of: Option<&str>, page: i64, limit: i64) -> String {
        let week_part = week_of.unwrap_or("current");
        format!(
            "{}:{}:trade_plans:{}:w{}:p{}:l{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, week_part, page, limit
        )
    }

    /// Generate cache key for a single trade plan
    #[inline]
    pub fn trade_plan(room_slug: &str, plan_id: i64) -> String {
        format!(
            "{}:{}:trade_plan:{}:{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, plan_id
        )
    }

    /// Generate cache key for room statistics
    #[inline]
    pub fn stats(room_slug: &str) -> String {
        format!("{}:{}:stats:{}", CACHE_PREFIX, KEY_VERSION, room_slug)
    }

    /// Generate cache key for current weekly video
    #[inline]
    pub fn weekly_video(room_slug: &str) -> String {
        format!(
            "{}:{}:weekly_video:{}",
            CACHE_PREFIX, KEY_VERSION, room_slug
        )
    }

    /// Generate cache key for weekly videos list
    #[inline]
    pub fn weekly_videos(room_slug: &str, page: i64, limit: i64) -> String {
        format!(
            "{}:{}:weekly_videos:{}:p{}:l{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, page, limit
        )
    }

    /// Generate cache key for archived videos
    #[inline]
    pub fn archived_videos(room_slug: &str, year: i32) -> String {
        format!(
            "{}:{}:archived_videos:{}:y{}",
            CACHE_PREFIX, KEY_VERSION, room_slug, year
        )
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // PATTERN KEYS FOR INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Pattern to match all alerts for a room (for cache invalidation)
    #[inline]
    pub fn alerts_pattern(room_slug: &str) -> String {
        format!("{}:{}:alert*:{}:*", CACHE_PREFIX, KEY_VERSION, room_slug)
    }

    /// Pattern to match all trades for a room
    #[inline]
    pub fn trades_pattern(room_slug: &str) -> String {
        format!("{}:{}:trade*:{}:*", CACHE_PREFIX, KEY_VERSION, room_slug)
    }

    /// Pattern to match all trade plans for a room
    #[inline]
    pub fn trade_plans_pattern(room_slug: &str) -> String {
        format!(
            "{}:{}:trade_plan*:{}:*",
            CACHE_PREFIX, KEY_VERSION, room_slug
        )
    }

    /// Pattern to match all videos for a room
    #[inline]
    pub fn videos_pattern(room_slug: &str) -> String {
        format!("{}:{}:*video*:{}*", CACHE_PREFIX, KEY_VERSION, room_slug)
    }

    /// Pattern to match all cache entries for a room
    #[inline]
    pub fn room_pattern(room_slug: &str) -> String {
        format!("{}:{}:*:{}*", CACHE_PREFIX, KEY_VERSION, room_slug)
    }

    /// Pattern to match all cache entries (for emergency flush)
    #[inline]
    pub fn all_pattern() -> String {
        format!("{}:{}:*", CACHE_PREFIX, KEY_VERSION)
    }
}

/// Cache TTL (Time-To-Live) constants
///
/// TTLs are carefully chosen based on:
/// 1. How frequently the data changes
/// 2. How critical freshness is for user experience
/// 3. Database load considerations
///
/// ## Strategy
///
/// - Alerts: Short TTL (60s) as they're time-sensitive
/// - Trades: Medium TTL (5min) as positions update periodically
/// - Trade Plans: Longer TTL (10min) as they're set weekly
/// - Stats: Medium TTL (5min) for reasonable freshness
/// - Videos: Long TTL (1hr) as they rarely change mid-week
pub mod cache_ttl {
    use std::time::Duration;

    /// TTL for alerts - 60 seconds
    /// Short TTL because alerts are time-sensitive and frequently updated
    pub const ALERTS: Duration = Duration::from_secs(60);

    /// TTL for individual alert - 60 seconds
    pub const ALERT: Duration = Duration::from_secs(60);

    /// TTL for trades list - 5 minutes
    /// Medium TTL as trade status updates are less frequent
    pub const TRADES: Duration = Duration::from_secs(300);

    /// TTL for individual trade - 5 minutes
    pub const TRADE: Duration = Duration::from_secs(300);

    /// TTL for trade plans - 10 minutes
    /// Longer TTL as trade plans are typically set weekly
    pub const TRADE_PLANS: Duration = Duration::from_secs(600);

    /// TTL for individual trade plan - 10 minutes
    pub const TRADE_PLAN: Duration = Duration::from_secs(600);

    /// TTL for room statistics - 5 minutes
    /// Medium TTL for reasonable balance between freshness and load
    pub const STATS: Duration = Duration::from_secs(300);

    /// TTL for weekly video - 1 hour
    /// Long TTL as video content rarely changes
    pub const WEEKLY_VIDEO: Duration = Duration::from_secs(3600);

    /// TTL for weekly videos list - 1 hour
    pub const WEEKLY_VIDEOS: Duration = Duration::from_secs(3600);

    /// TTL for archived videos - 1 hour
    /// Long TTL as archived content is static
    pub const ARCHIVED_VIDEOS: Duration = Duration::from_secs(3600);

    /// Default TTL for unspecified cache entries - 5 minutes
    pub const DEFAULT: Duration = Duration::from_secs(300);

    /// Short TTL for rapidly changing data - 30 seconds
    pub const SHORT: Duration = Duration::from_secs(30);

    /// Long TTL for mostly static data - 24 hours
    pub const LONG: Duration = Duration::from_secs(86400);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alerts_key_generation() {
        let key = cache_keys::alerts("explosive-swings", 1, 20);
        assert!(key.starts_with("es:v1:alerts:"));
        assert!(key.contains("explosive-swings"));
        assert!(key.contains("p1"));
        assert!(key.contains("l20"));
    }

    #[test]
    fn test_trades_key_with_status() {
        let key = cache_keys::trades("explosive-swings", Some("open"), 1, 10);
        assert!(key.contains("sopen"));

        let key_all = cache_keys::trades("explosive-swings", None, 1, 10);
        assert!(key_all.contains("sall"));
    }

    #[test]
    fn test_trade_plans_key_with_week() {
        let key = cache_keys::trade_plans("explosive-swings", Some("2026-01-20"), 1, 50);
        assert!(key.contains("w2026-01-20"));

        let key_current = cache_keys::trade_plans("explosive-swings", None, 1, 50);
        assert!(key_current.contains("wcurrent"));
    }

    #[test]
    fn test_pattern_keys() {
        let pattern = cache_keys::alerts_pattern("explosive-swings");
        assert!(pattern.contains("*"));
        assert!(pattern.contains("explosive-swings"));
    }

    #[test]
    fn test_ttl_values() {
        assert_eq!(cache_ttl::ALERTS.as_secs(), 60);
        assert_eq!(cache_ttl::TRADES.as_secs(), 300);
        assert_eq!(cache_ttl::TRADE_PLANS.as_secs(), 600);
        assert_eq!(cache_ttl::WEEKLY_VIDEO.as_secs(), 3600);
    }
}
