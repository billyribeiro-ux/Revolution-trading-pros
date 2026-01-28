//! Cache Service - Multi-Tier Caching with Graceful Degradation
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! A production-ready caching service built for reliability and performance.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        CacheService                              │
//! │  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐   │
//! │  │  L1: Memory   │ -> │   L2: Redis   │ -> │   Database    │   │
//! │  │  (HashMap)    │    │   (Upstash)   │    │   (fallback)  │   │
//! │  └───────────────┘    └───────────────┘    └───────────────┘   │
//! │         │                     │                     │           │
//! │         └─────────────────────┴─────────────────────┘           │
//! │                          Metrics                                 │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Features
//!
//! - **L1 In-Memory Cache**: Sub-millisecond lookups for frequently accessed data
//! - **L2 Redis Cache**: Distributed caching for cross-instance consistency
//! - **Graceful Degradation**: Continues operating if Redis is unavailable
//! - **Automatic TTL Management**: Entries expire based on configured TTL
//! - **Pattern-Based Invalidation**: Delete multiple related cache entries
//! - **Comprehensive Metrics**: Track cache hits, misses, and errors

use std::{
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;

use crate::services::redis::RedisService;

/// Cached value with metadata for TTL enforcement
#[derive(Clone, Debug)]
pub struct CachedValue {
    /// Serialized JSON data
    pub data: String,
    /// When this entry was created
    pub created_at: Instant,
    /// Time-to-live for this entry
    pub ttl: Duration,
}

impl CachedValue {
    /// Create a new cached value with TTL
    pub fn new(data: String, ttl: Duration) -> Self {
        Self {
            data,
            created_at: Instant::now(),
            ttl,
        }
    }

    /// Check if this cached value has expired
    #[inline]
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.ttl
    }

    /// Get remaining TTL in seconds (for Redis expiration)
    #[inline]
    pub fn remaining_ttl_secs(&self) -> u64 {
        let elapsed = self.created_at.elapsed();
        if elapsed >= self.ttl {
            0
        } else {
            (self.ttl - elapsed).as_secs()
        }
    }
}

/// Cache metrics for monitoring and alerting
///
/// All counters are atomic for thread-safe concurrent updates.
/// Metrics are exposed via the monitoring endpoint.
#[derive(Debug, Default)]
pub struct CacheMetrics {
    /// Total cache hits (L1 + L2)
    pub hits: AtomicU64,
    /// L1 (in-memory) cache hits
    pub l1_hits: AtomicU64,
    /// L2 (Redis) cache hits
    pub l2_hits: AtomicU64,
    /// Total cache misses
    pub misses: AtomicU64,
    /// Total cache errors
    pub errors: AtomicU64,
    /// Total cache writes
    pub writes: AtomicU64,
    /// Total cache deletions
    pub deletions: AtomicU64,
    /// Total fetch operations (database calls on cache miss)
    pub fetches: AtomicU64,
    /// Redis connection failures
    pub redis_failures: AtomicU64,
}

impl CacheMetrics {
    /// Create new metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a cache hit
    #[inline]
    pub fn record_hit(&self, is_l1: bool) {
        self.hits.fetch_add(1, Ordering::Relaxed);
        if is_l1 {
            self.l1_hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.l2_hits.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Record a cache miss
    #[inline]
    pub fn record_miss(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache error
    #[inline]
    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache write
    #[inline]
    pub fn record_write(&self) {
        self.writes.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache deletion
    #[inline]
    pub fn record_deletion(&self) {
        self.deletions.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a fetch operation (database call)
    #[inline]
    pub fn record_fetch(&self) {
        self.fetches.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a Redis connection failure
    #[inline]
    pub fn record_redis_failure(&self) {
        self.redis_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Get hit rate as a percentage
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed) as f64;
        let total = hits + self.misses.load(Ordering::Relaxed) as f64;
        if total == 0.0 {
            0.0
        } else {
            (hits / total) * 100.0
        }
    }

    /// Get metrics as a JSON-serializable snapshot
    pub fn snapshot(&self) -> CacheMetricsSnapshot {
        CacheMetricsSnapshot {
            hits: self.hits.load(Ordering::Relaxed),
            l1_hits: self.l1_hits.load(Ordering::Relaxed),
            l2_hits: self.l2_hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
            writes: self.writes.load(Ordering::Relaxed),
            deletions: self.deletions.load(Ordering::Relaxed),
            fetches: self.fetches.load(Ordering::Relaxed),
            redis_failures: self.redis_failures.load(Ordering::Relaxed),
            hit_rate: self.hit_rate(),
        }
    }
}

/// Serializable snapshot of cache metrics
#[derive(Debug, Clone, Serialize)]
pub struct CacheMetricsSnapshot {
    pub hits: u64,
    pub l1_hits: u64,
    pub l2_hits: u64,
    pub misses: u64,
    pub errors: u64,
    pub writes: u64,
    pub deletions: u64,
    pub fetches: u64,
    pub redis_failures: u64,
    pub hit_rate: f64,
}

/// Multi-tier caching service with graceful degradation
///
/// Provides a unified interface for caching with automatic fallback:
/// 1. Check L1 (in-memory) cache first
/// 2. Check L2 (Redis) cache on L1 miss
/// 3. Fetch from database on total cache miss
///
/// ## Graceful Degradation
///
/// If Redis is unavailable, the service:
/// - Continues operating with L1 cache only
/// - Logs warnings for monitoring
/// - Does not throw errors that would break the application
#[derive(Clone)]
pub struct CacheService {
    /// Redis service (optional for graceful degradation)
    redis: Option<Arc<RedisService>>,
    /// L1 in-memory cache
    local_cache: Arc<RwLock<HashMap<String, CachedValue>>>,
    /// Cache metrics
    metrics: Arc<CacheMetrics>,
    /// Maximum L1 cache entries (prevents unbounded memory growth)
    max_local_entries: usize,
}

impl CacheService {
    /// Create a new CacheService
    ///
    /// # Arguments
    /// * `redis` - Optional Redis service (None for graceful degradation)
    /// * `max_local_entries` - Maximum entries in L1 cache (default: 10,000)
    pub fn new(redis: Option<RedisService>, max_local_entries: Option<usize>) -> Self {
        let max_entries = max_local_entries.unwrap_or(10_000);

        if redis.is_some() {
            tracing::info!(
                target: "cache",
                max_local_entries = %max_entries,
                "CacheService initialized with Redis backend"
            );
        } else {
            tracing::warn!(
                target: "cache",
                max_local_entries = %max_entries,
                "CacheService initialized without Redis - L1 cache only"
            );
        }

        Self {
            redis: redis.map(Arc::new),
            local_cache: Arc::new(RwLock::new(HashMap::with_capacity(1000))),
            metrics: Arc::new(CacheMetrics::new()),
            max_local_entries: max_entries,
        }
    }

    /// Get cache metrics
    pub fn metrics(&self) -> &CacheMetrics {
        &self.metrics
    }

    /// Get metrics snapshot for monitoring
    pub fn metrics_snapshot(&self) -> CacheMetricsSnapshot {
        self.metrics.snapshot()
    }

    /// Check if Redis is available
    pub fn has_redis(&self) -> bool {
        self.redis.is_some()
    }

    /// Get a value from cache
    ///
    /// Checks L1 cache first, then L2 (Redis) on miss.
    /// Returns None if key not found or expired.
    ///
    /// # Type Parameters
    /// * `T` - The type to deserialize into (must implement DeserializeOwned)
    ///
    /// # Example
    /// ```ignore
    /// let alerts: Option<AlertsResponse> = cache.get(&key).await;
    /// ```
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        // L1: Check in-memory cache first
        {
            let cache = self.local_cache.read().await;
            if let Some(cached) = cache.get(key) {
                if !cached.is_expired() {
                    self.metrics.record_hit(true);
                    match serde_json::from_str::<T>(&cached.data) {
                        Ok(value) => {
                            tracing::trace!(
                                target: "cache",
                                key = %key,
                                "L1 cache hit"
                            );
                            return Some(value);
                        }
                        Err(e) => {
                            tracing::warn!(
                                target: "cache",
                                key = %key,
                                error = %e,
                                "L1 cache deserialization error"
                            );
                            self.metrics.record_error();
                        }
                    }
                }
            }
        }

        // L2: Check Redis on L1 miss
        if let Some(ref redis) = self.redis {
            match redis.get(key).await {
                Ok(Some(data)) => {
                    self.metrics.record_hit(false);
                    match serde_json::from_str::<T>(&data) {
                        Ok(value) => {
                            tracing::trace!(
                                target: "cache",
                                key = %key,
                                "L2 cache hit"
                            );
                            // Promote to L1 cache
                            let cached = CachedValue::new(data, Duration::from_secs(60));
                            self.set_local(key.to_string(), cached).await;
                            return Some(value);
                        }
                        Err(e) => {
                            tracing::warn!(
                                target: "cache",
                                key = %key,
                                error = %e,
                                "L2 cache deserialization error"
                            );
                            self.metrics.record_error();
                        }
                    }
                }
                Ok(None) => {
                    tracing::trace!(
                        target: "cache",
                        key = %key,
                        "Cache miss"
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        target: "cache",
                        key = %key,
                        error = %e,
                        "Redis get error - continuing without cache"
                    );
                    self.metrics.record_redis_failure();
                }
            }
        }

        self.metrics.record_miss();
        None
    }

    /// Set a value in cache with TTL
    ///
    /// Writes to both L1 and L2 caches for consistency.
    ///
    /// # Arguments
    /// * `key` - Cache key
    /// * `value` - Value to cache (must implement Serialize)
    /// * `ttl` - Time-to-live for this entry
    ///
    /// # Returns
    /// Ok(()) on success, Err on serialization failure
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()> {
        let data = serde_json::to_string(value).context("Failed to serialize cache value")?;

        // Write to L1 cache
        let cached = CachedValue::new(data.clone(), ttl);
        self.set_local(key.to_string(), cached).await;

        // Write to L2 (Redis) cache
        if let Some(ref redis) = self.redis {
            if let Err(e) = redis.set(key, &data, Some(ttl.as_secs())).await {
                tracing::warn!(
                    target: "cache",
                    key = %key,
                    error = %e,
                    "Redis set error - L1 cache still valid"
                );
                self.metrics.record_redis_failure();
            }
        }

        self.metrics.record_write();
        tracing::trace!(
            target: "cache",
            key = %key,
            ttl_secs = %ttl.as_secs(),
            "Cache write"
        );

        Ok(())
    }

    /// Delete a key from cache
    ///
    /// Removes from both L1 and L2 caches.
    pub async fn delete(&self, key: &str) -> Result<()> {
        // Remove from L1
        {
            let mut cache = self.local_cache.write().await;
            cache.remove(key);
        }

        // Remove from L2 (Redis)
        if let Some(ref redis) = self.redis {
            if let Err(e) = redis.delete(key).await {
                tracing::warn!(
                    target: "cache",
                    key = %key,
                    error = %e,
                    "Redis delete error"
                );
                self.metrics.record_redis_failure();
            }
        }

        self.metrics.record_deletion();
        tracing::trace!(
            target: "cache",
            key = %key,
            "Cache delete"
        );

        Ok(())
    }

    /// Delete all keys matching a pattern
    ///
    /// Uses Redis SCAN + DELETE for safe pattern matching.
    /// Also clears matching entries from L1 cache.
    ///
    /// # Arguments
    /// * `pattern` - Glob pattern (e.g., "es:v1:alerts:explosive-swings:*")
    ///
    /// # Note
    /// This is a potentially expensive operation. Use sparingly.
    pub async fn delete_pattern(&self, pattern: &str) -> Result<()> {
        // Clear matching entries from L1 cache
        {
            let mut cache = self.local_cache.write().await;
            let pattern_matcher = PatternMatcher::new(pattern);
            cache.retain(|key, _| !pattern_matcher.matches(key));
        }

        // Clear from Redis using SCAN (safe for production)
        if let Some(ref redis) = self.redis {
            // Note: We use the pattern directly with Redis KEYS command
            // In production, consider using SCAN for very large datasets
            if let Err(e) = self.delete_redis_pattern(redis, pattern).await {
                tracing::warn!(
                    target: "cache",
                    pattern = %pattern,
                    error = %e,
                    "Redis pattern delete error"
                );
                self.metrics.record_redis_failure();
            }
        }

        tracing::debug!(
            target: "cache",
            pattern = %pattern,
            "Cache pattern delete"
        );

        Ok(())
    }

    /// Get a value from cache, or fetch and cache it on miss
    ///
    /// This is the preferred method for most cache operations.
    /// Implements the cache-aside pattern with automatic caching.
    ///
    /// # Arguments
    /// * `key` - Cache key
    /// * `ttl` - TTL for cached value
    /// * `fetcher` - Async function to fetch data on cache miss
    ///
    /// # Example
    /// ```ignore
    /// let alerts = cache.get_or_fetch(
    ///     &cache_keys::alerts("explosive-swings", 1, 20),
    ///     cache_ttl::ALERTS,
    ///     || async {
    ///         // Fetch from database
    ///         sqlx::query_as("SELECT * FROM room_alerts...")
    ///             .fetch_all(&pool)
    ///             .await
    ///     }
    /// ).await?;
    /// ```
    pub async fn get_or_fetch<T, F, Fut>(&self, key: &str, ttl: Duration, fetcher: F) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Clone,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        // Try to get from cache first
        if let Some(cached) = self.get::<T>(key).await {
            return Ok(cached);
        }

        // Cache miss - fetch from source
        self.metrics.record_fetch();
        let value = fetcher().await.context("Failed to fetch value")?;

        // Cache the fetched value
        if let Err(e) = self.set(key, &value, ttl).await {
            tracing::warn!(
                target: "cache",
                key = %key,
                error = %e,
                "Failed to cache fetched value - returning uncached"
            );
        }

        Ok(value)
    }

    /// Set a value in L1 cache with eviction policy
    async fn set_local(&self, key: String, value: CachedValue) {
        let mut cache = self.local_cache.write().await;

        // Evict expired entries and enforce size limit
        if cache.len() >= self.max_local_entries {
            // Remove expired entries first
            cache.retain(|_, v| !v.is_expired());

            // If still over limit, remove oldest entries
            if cache.len() >= self.max_local_entries {
                let to_remove = cache.len() - self.max_local_entries + 100;
                let mut entries: Vec<_> = cache
                    .iter()
                    .map(|(k, v)| (k.clone(), v.created_at))
                    .collect();
                entries.sort_by(|a, b| a.1.cmp(&b.1));
                for (key, _) in entries.into_iter().take(to_remove) {
                    cache.remove(&key);
                }
                tracing::debug!(
                    target: "cache",
                    evicted = %to_remove,
                    "L1 cache eviction"
                );
            }
        }

        cache.insert(key, value);
    }

    /// Delete keys matching pattern from Redis
    async fn delete_redis_pattern(&self, redis: &RedisService, pattern: &str) -> Result<()> {
        let count = redis.delete_pattern(pattern).await?;

        if count > 0 {
            tracing::debug!(
                target: "cache",
                pattern = %pattern,
                deleted = %count,
                "Redis pattern delete completed"
            );
        }

        Ok(())
    }

    /// Clear all cache entries (L1 and L2)
    ///
    /// # Warning
    /// This is a destructive operation. Use with caution.
    pub async fn clear_all(&self) -> Result<()> {
        // Clear L1
        {
            let mut cache = self.local_cache.write().await;
            cache.clear();
        }

        // Clear L2 (matching our prefix only)
        if let Some(ref redis) = self.redis {
            if let Err(e) = self.delete_redis_pattern(redis, "es:v1:*").await {
                tracing::error!(
                    target: "cache",
                    error = %e,
                    "Failed to clear Redis cache"
                );
                return Err(e);
            }
        }

        tracing::warn!(
            target: "cache",
            "All cache entries cleared"
        );

        Ok(())
    }

    /// Get L1 cache size
    pub async fn local_cache_size(&self) -> usize {
        let cache = self.local_cache.read().await;
        cache.len()
    }
}

/// Simple glob pattern matcher for L1 cache invalidation
struct PatternMatcher {
    prefix: String,
    suffix: String,
    has_wildcard: bool,
}

impl PatternMatcher {
    fn new(pattern: &str) -> Self {
        if let Some(pos) = pattern.find('*') {
            Self {
                prefix: pattern[..pos].to_string(),
                suffix: pattern[pos + 1..].to_string(),
                has_wildcard: true,
            }
        } else {
            Self {
                prefix: pattern.to_string(),
                suffix: String::new(),
                has_wildcard: false,
            }
        }
    }

    fn matches(&self, key: &str) -> bool {
        if self.has_wildcard {
            key.starts_with(&self.prefix) && (self.suffix.is_empty() || key.ends_with(&self.suffix))
        } else {
            key == self.prefix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cached_value_expiration() {
        let value = CachedValue::new("test".to_string(), Duration::from_millis(10));
        assert!(!value.is_expired());

        std::thread::sleep(Duration::from_millis(15));
        assert!(value.is_expired());
    }

    #[test]
    fn test_pattern_matcher() {
        let matcher = PatternMatcher::new("es:v1:alerts:*");
        assert!(matcher.matches("es:v1:alerts:test:p1"));
        assert!(matcher.matches("es:v1:alerts:another"));
        assert!(!matcher.matches("es:v1:trades:test"));
    }

    #[test]
    fn test_metrics_hit_rate() {
        let metrics = CacheMetrics::new();

        // No data
        assert_eq!(metrics.hit_rate(), 0.0);

        // 50% hit rate
        metrics.hits.fetch_add(5, Ordering::Relaxed);
        metrics.misses.fetch_add(5, Ordering::Relaxed);
        assert_eq!(metrics.hit_rate(), 50.0);
    }

    #[tokio::test]
    async fn test_cache_service_local_only() {
        let cache = CacheService::new(None, Some(100));

        // Set a value
        cache
            .set(
                "test_key",
                &"test_value".to_string(),
                Duration::from_secs(60),
            )
            .await
            .unwrap();

        // Get the value
        let result: Option<String> = cache.get("test_key").await;
        assert_eq!(result, Some("test_value".to_string()));

        // Delete the value
        cache.delete("test_key").await.unwrap();

        // Verify deletion
        let result: Option<String> = cache.get("test_key").await;
        assert!(result.is_none());
    }
}
