//! Cache Module - Phase 2 Backend Redis Caching for Explosive Swings
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! A robust, production-ready caching layer built for the next 10 years.
//!
//! ## Architecture
//!
//! This module implements a multi-tier caching strategy:
//! 1. **L1 Cache (In-Memory)**: Sub-millisecond access for hot data
//! 2. **L2 Cache (Redis)**: Distributed cache for cross-instance consistency
//!
//! ## Key Features
//!
//! - **Graceful Degradation**: Continues operating if Redis is unavailable
//! - **Automatic Fallback**: Falls back to database when cache misses occur
//! - **Smart Invalidation**: Pattern-based cache invalidation for related data
//! - **Comprehensive Metrics**: Tracks hits, misses, errors for monitoring
//! - **Type-Safe Keys**: Compile-time key generation prevents typos
//!
//! ## Usage
//!
//! ```rust,ignore
//! use crate::cache::{CacheService, cache_keys, cache_ttl};
//!
//! // Get or fetch pattern - most common usage
//! let alerts = cache.get_or_fetch(
//!     &cache_keys::alerts("explosive-swings", 1, 20),
//!     cache_ttl::ALERTS,
//!     || async { fetch_alerts_from_db().await }
//! ).await?;
//!
//! // Manual cache operations
//! cache.set(&key, &value, ttl).await?;
//! let cached: Option<MyType> = cache.get(&key).await;
//! cache.delete(&key).await?;
//!
//! // Pattern-based invalidation
//! cache.delete_pattern("room:explosive-swings:*").await?;
//! ```

pub mod invalidation;
pub mod keys;
pub mod service;

// Re-export commonly used items
pub use invalidation::CacheInvalidator;
pub use keys::{cache_keys, cache_ttl};
pub use service::{CacheMetrics, CacheService, CachedValue};
