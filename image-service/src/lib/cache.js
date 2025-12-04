/**
 * Redis Cache Manager
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Provides caching layer for image metadata, BlurHash, and variant URLs.
 * Falls back to in-memory cache if Redis is not available.
 */

import { createClient } from 'redis';

class CacheManager {
  constructor() {
    this.redis = null;
    this.memoryCache = new Map();
    this.connected = false;
    this.defaultTTL = 3600; // 1 hour
    this.maxMemoryCacheSize = 1000;
  }

  /**
   * Initialize Redis connection
   */
  async initialize() {
    const redisUrl = process.env.REDIS_URL;

    if (!redisUrl) {
      console.log('Redis URL not configured, using in-memory cache');
      return;
    }

    try {
      this.redis = createClient({ url: redisUrl });

      this.redis.on('error', (err) => {
        console.error('Redis error:', err);
        this.connected = false;
      });

      this.redis.on('connect', () => {
        console.log('Redis connected');
        this.connected = true;
      });

      await this.redis.connect();
    } catch (error) {
      console.error('Failed to connect to Redis:', error.message);
      console.log('Falling back to in-memory cache');
    }
  }

  /**
   * Get value from cache
   */
  async get(key) {
    try {
      if (this.connected && this.redis) {
        const value = await this.redis.get(key);
        return value ? JSON.parse(value) : null;
      }
      return this.memoryCache.get(key) || null;
    } catch (error) {
      console.error('Cache get error:', error);
      return this.memoryCache.get(key) || null;
    }
  }

  /**
   * Set value in cache
   */
  async set(key, value, ttl = this.defaultTTL) {
    const serialized = typeof value === 'string' ? value : JSON.stringify(value);

    try {
      if (this.connected && this.redis) {
        await this.redis.setEx(key, ttl, serialized);
      }

      // Always update memory cache as backup
      this.setMemoryCache(key, value, ttl);
    } catch (error) {
      console.error('Cache set error:', error);
      this.setMemoryCache(key, value, ttl);
    }
  }

  /**
   * Set in-memory cache with TTL
   */
  setMemoryCache(key, value, ttl) {
    // Enforce max size
    if (this.memoryCache.size >= this.maxMemoryCacheSize) {
      const firstKey = this.memoryCache.keys().next().value;
      this.memoryCache.delete(firstKey);
    }

    this.memoryCache.set(key, value);

    // Set TTL for memory cache
    setTimeout(() => {
      this.memoryCache.delete(key);
    }, ttl * 1000);
  }

  /**
   * Delete from cache
   */
  async delete(key) {
    try {
      if (this.connected && this.redis) {
        await this.redis.del(key);
      }
      this.memoryCache.delete(key);
    } catch (error) {
      console.error('Cache delete error:', error);
    }
  }

  /**
   * Delete all keys matching pattern
   */
  async deletePattern(pattern) {
    try {
      if (this.connected && this.redis) {
        const keys = await this.redis.keys(pattern);
        if (keys.length > 0) {
          await this.redis.del(keys);
        }
      }

      // Clear matching keys from memory cache
      for (const key of this.memoryCache.keys()) {
        if (this.matchPattern(key, pattern)) {
          this.memoryCache.delete(key);
        }
      }
    } catch (error) {
      console.error('Cache deletePattern error:', error);
    }
  }

  /**
   * Simple pattern matching for memory cache
   */
  matchPattern(key, pattern) {
    const regex = new RegExp('^' + pattern.replace(/\*/g, '.*') + '$');
    return regex.test(key);
  }

  /**
   * Cache media metadata
   */
  async cacheMediaMeta(mediaId, metadata) {
    const key = `media:${mediaId}:meta`;
    await this.set(key, metadata, 86400); // 24 hours
  }

  /**
   * Get cached media metadata
   */
  async getMediaMeta(mediaId) {
    const key = `media:${mediaId}:meta`;
    return this.get(key);
  }

  /**
   * Cache BlurHash
   */
  async cacheBlurhash(mediaId, blurhash) {
    const key = `media:${mediaId}:blurhash`;
    await this.set(key, blurhash, 604800); // 7 days
  }

  /**
   * Get cached BlurHash
   */
  async getBlurhash(mediaId) {
    const key = `media:${mediaId}:blurhash`;
    return this.get(key);
  }

  /**
   * Cache variant URLs
   */
  async cacheVariants(mediaId, variants) {
    const key = `media:${mediaId}:variants`;
    await this.set(key, variants, 86400); // 24 hours
  }

  /**
   * Get cached variants
   */
  async getVariants(mediaId) {
    const key = `media:${mediaId}:variants`;
    return this.get(key);
  }

  /**
   * Cache on-demand variant
   */
  async cacheOnDemandVariant(mediaId, variantKey, data) {
    const key = `media:${mediaId}:variant:${variantKey}`;
    await this.set(key, data, 86400); // 24 hours
  }

  /**
   * Get cached on-demand variant
   */
  async getOnDemandVariant(mediaId, variantKey) {
    const key = `media:${mediaId}:variant:${variantKey}`;
    return this.get(key);
  }

  /**
   * Invalidate all cache for a media item
   */
  async invalidateMedia(mediaId) {
    await this.deletePattern(`media:${mediaId}:*`);
  }

  /**
   * Get cache statistics
   */
  getStats() {
    return {
      redisConnected: this.connected,
      memoryCacheSize: this.memoryCache.size,
      maxMemoryCacheSize: this.maxMemoryCacheSize,
    };
  }

  /**
   * Shutdown cache
   */
  async shutdown() {
    if (this.redis) {
      await this.redis.quit();
    }
    this.memoryCache.clear();
    this.connected = false;
  }
}

// Export singleton instance
export const cache = new CacheManager();
export default CacheManager;
