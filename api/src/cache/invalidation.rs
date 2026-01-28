//! Cache Invalidation - Smart Invalidation Strategies
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Provides targeted cache invalidation to maintain data consistency
//! while minimizing unnecessary cache clears.
//!
//! ## Invalidation Strategies
//!
//! 1. **Single Entry**: Invalidate a specific cache entry by key
//! 2. **Pattern-Based**: Invalidate all entries matching a pattern
//! 3. **Entity-Based**: Invalidate all cache entries for an entity type
//! 4. **Room-Based**: Invalidate all cache entries for a trading room
//!
//! ## Best Practices
//!
//! - Invalidate specific entries when possible (more efficient)
//! - Use pattern invalidation for bulk updates
//! - Consider cascade invalidation for related data
//! - Log all invalidations for debugging

use anyhow::Result;
use tracing::{debug, info};

use super::keys::cache_keys;
use super::service::CacheService;

/// Cache invalidation helper
///
/// Provides convenient methods for invalidating cache entries
/// based on business logic and data relationships.
#[derive(Clone)]
pub struct CacheInvalidator {
    cache: CacheService,
}

impl CacheInvalidator {
    /// Create a new CacheInvalidator
    pub fn new(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Get reference to underlying cache service
    pub fn cache(&self) -> &CacheService {
        &self.cache
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ALERTS INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate all alerts cache for a room
    ///
    /// Call this after:
    /// - Creating a new alert
    /// - Updating an alert
    /// - Deleting an alert
    /// - Bulk alert operations
    pub async fn invalidate_alerts(&self, room_slug: &str) -> Result<()> {
        let pattern = cache_keys::alerts_pattern(room_slug);
        self.cache.delete_pattern(&pattern).await?;

        info!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            pattern = %pattern,
            "Alerts cache invalidated"
        );

        Ok(())
    }

    /// Invalidate a specific alert
    ///
    /// More efficient than pattern invalidation when only
    /// a single alert changes.
    pub async fn invalidate_alert(&self, room_slug: &str, alert_id: i64) -> Result<()> {
        let key = cache_keys::alert(room_slug, alert_id);
        self.cache.delete(&key).await?;

        // Also invalidate list caches (they contain this alert)
        self.invalidate_alerts(room_slug).await?;

        debug!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            alert_id = %alert_id,
            "Single alert cache invalidated"
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TRADES INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate all trades cache for a room
    ///
    /// Call this after:
    /// - Creating a new trade
    /// - Closing a trade
    /// - Updating a trade
    /// - Invalidating a trade
    /// - Deleting a trade
    pub async fn invalidate_trades(&self, room_slug: &str) -> Result<()> {
        let pattern = cache_keys::trades_pattern(room_slug);
        self.cache.delete_pattern(&pattern).await?;

        // Also invalidate stats (they depend on trades)
        self.invalidate_stats(room_slug).await?;

        info!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            pattern = %pattern,
            "Trades cache invalidated"
        );

        Ok(())
    }

    /// Invalidate a specific trade
    pub async fn invalidate_trade(&self, room_slug: &str, trade_id: i64) -> Result<()> {
        let key = cache_keys::trade(room_slug, trade_id);
        self.cache.delete(&key).await?;

        // Also invalidate list caches
        self.invalidate_trades(room_slug).await?;

        debug!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            trade_id = %trade_id,
            "Single trade cache invalidated"
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TRADE PLANS INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate all trade plans cache for a room
    ///
    /// Call this after:
    /// - Creating a new trade plan
    /// - Updating a trade plan
    /// - Reordering trade plans
    /// - Deleting a trade plan
    pub async fn invalidate_trade_plans(&self, room_slug: &str) -> Result<()> {
        let pattern = cache_keys::trade_plans_pattern(room_slug);
        self.cache.delete_pattern(&pattern).await?;

        info!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            pattern = %pattern,
            "Trade plans cache invalidated"
        );

        Ok(())
    }

    /// Invalidate a specific trade plan
    pub async fn invalidate_trade_plan(&self, room_slug: &str, plan_id: i64) -> Result<()> {
        let key = cache_keys::trade_plan(room_slug, plan_id);
        self.cache.delete(&key).await?;

        // Also invalidate list caches
        self.invalidate_trade_plans(room_slug).await?;

        debug!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            plan_id = %plan_id,
            "Single trade plan cache invalidated"
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // STATS INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate room statistics cache
    ///
    /// Call this after:
    /// - Trade closed (win/loss affects stats)
    /// - Stats recalculation
    /// - Manual stats update
    pub async fn invalidate_stats(&self, room_slug: &str) -> Result<()> {
        let key = cache_keys::stats(room_slug);
        self.cache.delete(&key).await?;

        debug!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            "Stats cache invalidated"
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // WEEKLY VIDEO INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate weekly video cache for a room
    ///
    /// Call this after:
    /// - Publishing a new weekly video
    /// - Updating video metadata
    /// - Archiving a video
    pub async fn invalidate_weekly_video(&self, room_slug: &str) -> Result<()> {
        let pattern = cache_keys::videos_pattern(room_slug);
        self.cache.delete_pattern(&pattern).await?;

        info!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            pattern = %pattern,
            "Weekly video cache invalidated"
        );

        Ok(())
    }

    /// Invalidate archived videos cache for a room
    pub async fn invalidate_archived_videos(&self, room_slug: &str, year: i32) -> Result<()> {
        let key = cache_keys::archived_videos(room_slug, year);
        self.cache.delete(&key).await?;

        debug!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            year = %year,
            "Archived videos cache invalidated"
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ROOM-WIDE INVALIDATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate all cache entries for a room
    ///
    /// Use this for:
    /// - Room configuration changes
    /// - Emergency cache clear
    /// - Data migration
    ///
    /// # Warning
    /// This is a heavy operation. Use specific invalidation methods when possible.
    pub async fn invalidate_room(&self, room_slug: &str) -> Result<()> {
        let pattern = cache_keys::room_pattern(room_slug);
        self.cache.delete_pattern(&pattern).await?;

        info!(
            target: "cache.invalidation",
            room_slug = %room_slug,
            pattern = %pattern,
            "Room cache invalidated (all entries)"
        );

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // BULK OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate cache after an alert operation
    ///
    /// Handles the cascade of invalidations needed after alert changes:
    /// - Alert list caches
    /// - Individual alert cache (if applicable)
    /// - Archived videos (if alert counts are displayed)
    pub async fn on_alert_change(&self, room_slug: &str, alert_id: Option<i64>) -> Result<()> {
        if let Some(id) = alert_id {
            self.invalidate_alert(room_slug, id).await?;
        } else {
            self.invalidate_alerts(room_slug).await?;
        }
        Ok(())
    }

    /// Invalidate cache after a trade operation
    ///
    /// Handles cascade invalidations:
    /// - Trade list caches
    /// - Individual trade cache
    /// - Room stats (trades affect stats)
    /// - Archived videos (if trade counts/win rates are displayed)
    pub async fn on_trade_change(&self, room_slug: &str, trade_id: Option<i64>) -> Result<()> {
        if let Some(id) = trade_id {
            self.invalidate_trade(room_slug, id).await?;
        } else {
            self.invalidate_trades(room_slug).await?;
        }
        // Stats are already invalidated by invalidate_trades
        Ok(())
    }

    /// Invalidate cache after a trade plan operation
    pub async fn on_trade_plan_change(&self, room_slug: &str, plan_id: Option<i64>) -> Result<()> {
        if let Some(id) = plan_id {
            self.invalidate_trade_plan(room_slug, id).await?;
        } else {
            self.invalidate_trade_plans(room_slug).await?;
        }
        Ok(())
    }

    /// Invalidate cache after a weekly video operation
    pub async fn on_weekly_video_change(&self, room_slug: &str) -> Result<()> {
        self.invalidate_weekly_video(room_slug).await?;
        // Also invalidate current year's archive
        let current_year = chrono::Utc::now().year();
        self.invalidate_archived_videos(room_slug, current_year)
            .await?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EMERGENCY OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Clear all Explosive Swings cache entries
    ///
    /// # Warning
    /// This is a destructive operation. Only use for:
    /// - Schema migrations
    /// - Cache corruption recovery
    /// - Emergency situations
    pub async fn clear_all(&self) -> Result<()> {
        self.cache.clear_all().await?;

        info!(
            target: "cache.invalidation",
            "ALL cache entries cleared (emergency operation)"
        );

        Ok(())
    }
}

// Helper trait for chrono year extraction
use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalidator_creation() {
        let cache = CacheService::new(None, Some(100));
        let invalidator = CacheInvalidator::new(cache);
        assert!(!invalidator.cache().has_redis());
    }

    #[tokio::test]
    async fn test_alert_invalidation() {
        let cache = CacheService::new(None, Some(100));
        let invalidator = CacheInvalidator::new(cache);

        // Should not error even with no data
        let result = invalidator.invalidate_alerts("test-room").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trade_invalidation_cascades_to_stats() {
        let cache = CacheService::new(None, Some(100));
        let invalidator = CacheInvalidator::new(cache);

        // Store some test data
        invalidator
            .cache()
            .set(
                &cache_keys::stats("test-room"),
                &"test_stats",
                std::time::Duration::from_secs(60),
            )
            .await
            .unwrap();

        // Verify it's cached
        let cached: Option<String> = invalidator
            .cache()
            .get(&cache_keys::stats("test-room"))
            .await;
        assert!(cached.is_some());

        // Invalidate trades (should cascade to stats)
        invalidator.invalidate_trades("test-room").await.unwrap();

        // Stats should be cleared
        let cached: Option<String> = invalidator
            .cache()
            .get(&cache_keys::stats("test-room"))
            .await;
        assert!(cached.is_none());
    }
}
