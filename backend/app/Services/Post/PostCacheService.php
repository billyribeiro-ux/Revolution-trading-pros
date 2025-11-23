<?php

declare(strict_types=1);

namespace App\Services\Post;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Collection;
use App\Models\Post;

/**
 * PostCacheService - Enterprise-grade cache management for posts
 * 
 * Comprehensive caching service providing:
 * - Multi-tier cache strategy (memory, Redis, database)
 * - Tag-based cache invalidation
 * - Cache warming and preloading
 * - Cache hit/miss tracking
 * - Intelligent TTL management
 * - Atomic cache operations
 * - Cache versioning
 * - Bulk cache operations
 * - Cache analytics and monitoring
 */
class PostCacheService
{
    // =========================================================================
    // CONSTANTS - TTL CONFIGURATION
    // =========================================================================

    private const TTL_SHORT = 300;      // 5 minutes - frequently changing data
    private const TTL_MEDIUM = 1800;    // 30 minutes - moderately changing data
    private const TTL_LONG = 3600;      // 1 hour - relatively stable data
    private const TTL_EXTENDED = 7200;  // 2 hours - very stable data
    private const TTL_DAY = 86400;      // 24 hours - rarely changing data

    /**
     * Default cache TTL in seconds.
     */
    private const TTL = self::TTL_LONG;

    /**
     * Cache key prefix.
     */
    private const PREFIX = 'post:';

    /**
     * Cache version (increment to invalidate all caches).
     */
    private const VERSION = 'v2';

    // =========================================================================
    // CONSTANTS - CACHE TAGS
    // =========================================================================

    private const TAG_POSTS = 'posts';
    private const TAG_POST_LIST = 'post_list';
    private const TAG_POST_DETAIL = 'post_detail';
    private const TAG_POST_ANALYTICS = 'post_analytics';
    private const TAG_POST_CATEGORY = 'post_category';
    private const TAG_POST_USER = 'post_user';
    private const TAG_POST_SEARCH = 'post_search';

    /**
     * Cache statistics tracking.
     */
    private array $stats = [
        'hits' => 0,
        'misses' => 0,
        'sets' => 0,
        'deletes' => 0,
    ];

    // =========================================================================
    // CORE CACHE OPERATIONS
    // =========================================================================

    /**
     * Remember a value in cache with automatic tag management.
     */
    public function remember(
        string $key,
        ?int $ttl = null,
        callable $callback,
        ?array $tags = null
    ): mixed {
        $fullKey = $this->buildKey($key);
        $ttl = $ttl ?? self::TTL;

        try {
            // Use tagged cache if tags provided
            if ($tags) {
                $result = Cache::tags($tags)->remember($fullKey, $ttl, function () use ($callback, $fullKey) {
                    $this->stats['misses']++;
                    $this->logCacheMiss($fullKey);
                    return $callback();
                });
            } else {
                $result = Cache::remember($fullKey, $ttl, function () use ($callback, $fullKey) {
                    $this->stats['misses']++;
                    $this->logCacheMiss($fullKey);
                    return $callback();
                });
            }

            if (Cache::has($fullKey)) {
                $this->stats['hits']++;
            }

            return $result;
        } catch (\Exception $e) {
            Log::error('Cache remember failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            // Fallback to callback on cache failure
            return $callback();
        }
    }

    /**
     * Remember forever (until explicitly cleared).
     */
    public function rememberForever(string $key, callable $callback, ?array $tags = null): mixed
    {
        $fullKey = $this->buildKey($key);

        try {
            if ($tags) {
                return Cache::tags($tags)->rememberForever($fullKey, function () use ($callback, $fullKey) {
                    $this->stats['misses']++;
                    $this->logCacheMiss($fullKey);
                    return $callback();
                });
            }

            return Cache::rememberForever($fullKey, function () use ($callback, $fullKey) {
                $this->stats['misses']++;
                $this->logCacheMiss($fullKey);
                return $callback();
            });
        } catch (\Exception $e) {
            Log::error('Cache rememberForever failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return $callback();
        }
    }

    /**
     * Put a value in cache.
     */
    public function put(string $key, mixed $value, ?int $ttl = null, ?array $tags = null): bool
    {
        $fullKey = $this->buildKey($key);
        $ttl = $ttl ?? self::TTL;

        try {
            $this->stats['sets']++;

            if ($tags) {
                return Cache::tags($tags)->put($fullKey, $value, $ttl);
            }

            return Cache::put($fullKey, $value, $ttl);
        } catch (\Exception $e) {
            Log::error('Cache put failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Get a value from cache.
     */
    public function get(string $key, mixed $default = null): mixed
    {
        $fullKey = $this->buildKey($key);

        try {
            $value = Cache::get($fullKey, $default);

            if ($value !== $default) {
                $this->stats['hits']++;
            } else {
                $this->stats['misses']++;
            }

            return $value;
        } catch (\Exception $e) {
            Log::error('Cache get failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return $default;
        }
    }

    /**
     * Check if key exists in cache.
     */
    public function has(string $key): bool
    {
        $fullKey = $this->buildKey($key);

        try {
            return Cache::has($fullKey);
        } catch (\Exception $e) {
            Log::error('Cache has check failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Forget a cache key.
     */
    public function forget(string $key): bool
    {
        $fullKey = $this->buildKey($key);

        try {
            $this->stats['deletes']++;
            return Cache::forget($fullKey);
        } catch (\Exception $e) {
            Log::error('Cache forget failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Forget multiple cache keys.
     */
    public function forgetMany(array $keys): int
    {
        $deleted = 0;

        foreach ($keys as $key) {
            if ($this->forget($key)) {
                $deleted++;
            }
        }

        return $deleted;
    }

    // =========================================================================
    // TAG-BASED CACHE OPERATIONS
    // =========================================================================

    /**
     * Flush caches by tag.
     */
    public function flushTag(string $tag): bool
    {
        try {
            Cache::tags([$tag])->flush();
            
            Log::info('Cache tag flushed', ['tag' => $tag]);
            return true;
        } catch (\Exception $e) {
            Log::error('Cache tag flush failed', [
                'tag' => $tag,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Flush multiple cache tags.
     */
    public function flushTags(array $tags): bool
    {
        try {
            Cache::tags($tags)->flush();
            
            Log::info('Cache tags flushed', ['tags' => $tags]);
            return true;
        } catch (\Exception $e) {
            Log::error('Cache tags flush failed', [
                'tags' => $tags,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Flush all post-related caches.
     */
    public function flushAllPostCaches(): bool
    {
        return $this->flushTags([
            self::TAG_POSTS,
            self::TAG_POST_LIST,
            self::TAG_POST_DETAIL,
            self::TAG_POST_ANALYTICS,
            self::TAG_POST_CATEGORY,
            self::TAG_POST_USER,
            self::TAG_POST_SEARCH,
        ]);
    }

    /**
     * Flush all caches (use with extreme caution).
     */
    public function flush(): bool
    {
        try {
            Cache::flush();
            
            Log::warning('All caches flushed');
            return true;
        } catch (\Exception $e) {
            Log::error('Cache flush failed', [
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    // =========================================================================
    // POST-SPECIFIC CACHE OPERATIONS
    // =========================================================================

    /**
     * Cache a single post.
     */
    public function cachePost(Post $post, ?int $ttl = null): bool
    {
        $key = "detail:{$post->id}";
        return $this->put($key, $post, $ttl ?? self::TTL_LONG, [self::TAG_POST_DETAIL, self::TAG_POSTS]);
    }

    /**
     * Get cached post.
     */
    public function getCachedPost(int $postId): ?Post
    {
        $key = "detail:{$postId}";
        return $this->get($key);
    }

    /**
     * Invalidate post cache.
     */
    public function invalidatePost(int $postId): bool
    {
        $keys = [
            "detail:{$postId}",
            "analytics:{$postId}",
            "related:{$postId}",
            "comments:{$postId}",
        ];

        return $this->forgetMany($keys) > 0;
    }

    /**
     * Cache post list.
     */
    public function cachePostList(string $identifier, Collection $posts, ?int $ttl = null): bool
    {
        $key = "list:{$identifier}";
        return $this->put($key, $posts, $ttl ?? self::TTL_MEDIUM, [self::TAG_POST_LIST, self::TAG_POSTS]);
    }

    /**
     * Cache post analytics.
     */
    public function cacheAnalytics(int $postId, array $analytics, ?int $ttl = null): bool
    {
        $key = "analytics:{$postId}";
        return $this->put($key, $analytics, $ttl ?? self::TTL_SHORT, [self::TAG_POST_ANALYTICS]);
    }

    /**
     * Invalidate category cache.
     */
    public function invalidateCategory(int $categoryId): bool
    {
        return $this->flushTag(self::TAG_POST_CATEGORY . ":{$categoryId}");
    }

    /**
     * Invalidate user posts cache.
     */
    public function invalidateUserPosts(int $userId): bool
    {
        return $this->flushTag(self::TAG_POST_USER . ":{$userId}");
    }

    // =========================================================================
    // CACHE KEY GENERATION
    // =========================================================================

    /**
     * Generate a cache key from a prefix and parameters.
     */
    public function generateKey(string $prefix, array $params = []): string
    {
        // Sort params for consistent keys
        ksort($params);
        
        // Filter out null/empty values
        $params = array_filter($params, fn($value) => $value !== null && $value !== '');
        
        // Create hash of params
        $hash = md5(json_encode($params));
        
        return $prefix . ':' . $hash;
    }

    /**
     * Build full cache key with prefix and version.
     */
    private function buildKey(string $key): string
    {
        return self::PREFIX . self::VERSION . ':' . $key;
    }

    /**
     * Generate key for post list with filters.
     */
    public function generateListKey(array $filters = []): string
    {
        return $this->generateKey('list', [
            'status' => $filters['status'] ?? null,
            'category' => $filters['category'] ?? null,
            'tag' => $filters['tag'] ?? null,
            'author' => $filters['author'] ?? null,
            'search' => $filters['search'] ?? null,
            'sort' => $filters['sort'] ?? null,
            'page' => $filters['page'] ?? 1,
            'per_page' => $filters['per_page'] ?? 15,
        ]);
    }

    /**
     * Generate key for search results.
     */
    public function generateSearchKey(string $query, array $filters = []): string
    {
        return $this->generateKey('search', array_merge(['query' => $query], $filters));
    }

    // =========================================================================
    // CACHE WARMING & PRELOADING
    // =========================================================================

    /**
     * Warm cache for popular posts.
     */
    public function warmPopularPosts(int $limit = 10): int
    {
        $warmed = 0;

        try {
            $posts = Post::where('status', 'published')
                ->orderBy('view_count', 'desc')
                ->limit($limit)
                ->get();

            foreach ($posts as $post) {
                if ($this->cachePost($post)) {
                    $warmed++;
                }
            }

            Log::info('Popular posts cache warmed', ['count' => $warmed]);
        } catch (\Exception $e) {
            Log::error('Cache warming failed', [
                'error' => $e->getMessage(),
            ]);
        }

        return $warmed;
    }

    /**
     * Preload related data for a post.
     */
    public function preloadPostData(Post $post): bool
    {
        try {
            // Cache post
            $this->cachePost($post);

            // Cache related posts
            $related = $post->getRelated(5);
            $this->put("related:{$post->id}", $related, self::TTL_EXTENDED, [self::TAG_POSTS]);

            // Cache comments count
            $this->put("comments_count:{$post->id}", $post->comments()->count(), self::TTL_SHORT);

            return true;
        } catch (\Exception $e) {
            Log::error('Post data preload failed', [
                'post_id' => $post->id,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Warm cache for entire category.
     */
    public function warmCategory(int $categoryId, int $limit = 20): int
    {
        $warmed = 0;

        try {
            $posts = Post::where('status', 'published')
                ->where('category_id', $categoryId)
                ->orderBy('published_at', 'desc')
                ->limit($limit)
                ->get();

            foreach ($posts as $post) {
                if ($this->cachePost($post)) {
                    $warmed++;
                }
            }

            // Cache the list
            $this->cachePostList("category:{$categoryId}", $posts, self::TTL_MEDIUM);

            Log::info('Category cache warmed', [
                'category_id' => $categoryId,
                'count' => $warmed,
            ]);
        } catch (\Exception $e) {
            Log::error('Category cache warming failed', [
                'category_id' => $categoryId,
                'error' => $e->getMessage(),
            ]);
        }

        return $warmed;
    }

    // =========================================================================
    // ATOMIC CACHE OPERATIONS
    // =========================================================================

    /**
     * Increment a cached counter atomically.
     */
    public function increment(string $key, int $value = 1): int|bool
    {
        $fullKey = $this->buildKey($key);

        try {
            return Cache::increment($fullKey, $value);
        } catch (\Exception $e) {
            Log::error('Cache increment failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Decrement a cached counter atomically.
     */
    public function decrement(string $key, int $value = 1): int|bool
    {
        $fullKey = $this->buildKey($key);

        try {
            return Cache::decrement($fullKey, $value);
        } catch (\Exception $e) {
            Log::error('Cache decrement failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Add item to cache only if it doesn't exist.
     */
    public function add(string $key, mixed $value, ?int $ttl = null): bool
    {
        $fullKey = $this->buildKey($key);
        $ttl = $ttl ?? self::TTL;

        try {
            return Cache::add($fullKey, $value, $ttl);
        } catch (\Exception $e) {
            Log::error('Cache add failed', [
                'key' => $fullKey,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    // =========================================================================
    // CACHE ANALYTICS & MONITORING
    // =========================================================================

    /**
     * Get cache statistics.
     */
    public function getStats(): array
    {
        $hitRate = $this->stats['hits'] + $this->stats['misses'] > 0
            ? round(($this->stats['hits'] / ($this->stats['hits'] + $this->stats['misses'])) * 100, 2)
            : 0;

        return [
            'hits' => $this->stats['hits'],
            'misses' => $this->stats['misses'],
            'sets' => $this->stats['sets'],
            'deletes' => $this->stats['deletes'],
            'hit_rate' => $hitRate,
            'total_operations' => array_sum($this->stats),
        ];
    }

    /**
     * Reset cache statistics.
     */
    public function resetStats(): void
    {
        $this->stats = [
            'hits' => 0,
            'misses' => 0,
            'sets' => 0,
            'deletes' => 0,
        ];
    }

    /**
     * Get cache size estimate (if supported by driver).
     */
    public function getCacheSize(): ?int
    {
        try {
            // This depends on the cache driver
            // Redis example:
            if (Cache::getStore() instanceof \Illuminate\Cache\RedisStore) {
                $redis = Cache::getStore()->connection();
                return $redis->dbsize();
            }

            return null;
        } catch (\Exception $e) {
            Log::error('Cache size check failed', [
                'error' => $e->getMessage(),
            ]);

            return null;
        }
    }

    // =========================================================================
    // TTL MANAGEMENT
    // =========================================================================

    /**
     * Get default TTL.
     */
    public function getDefaultTtl(): int
    {
        return self::TTL;
    }

    /**
     * Get TTL by data type.
     */
    public function getTtlByType(string $type): int
    {
        return match ($type) {
            'analytics', 'counters', 'trending' => self::TTL_SHORT,
            'list', 'search', 'category' => self::TTL_MEDIUM,
            'detail', 'related', 'metadata' => self::TTL_LONG,
            'static', 'config', 'settings' => self::TTL_EXTENDED,
            'permanent' => self::TTL_DAY,
            default => self::TTL,
        };
    }

    /**
     * Get recommended TTL based on access frequency.
     */
    public function getSmartTtl(string $key): int
    {
        $accessKey = "access_count:{$key}";
        $accessCount = $this->get($accessKey, 0);

        // More frequently accessed = longer TTL
        return match (true) {
            $accessCount > 1000 => self::TTL_EXTENDED,
            $accessCount > 100 => self::TTL_LONG,
            $accessCount > 10 => self::TTL_MEDIUM,
            default => self::TTL_SHORT,
        };
    }

    // =========================================================================
    // LOGGING & DEBUGGING
    // =========================================================================

    /**
     * Log cache miss for monitoring.
     */
    private function logCacheMiss(string $key): void
    {
        if (config('cache.log_misses', false)) {
            Log::debug('Cache miss', ['key' => $key]);
        }
    }

    /**
     * Get cache keys matching pattern (debug only).
     */
    public function getKeysMatching(string $pattern): array
    {
        if (!app()->environment('local', 'development')) {
            return [];
        }

        try {
            if (Cache::getStore() instanceof \Illuminate\Cache\RedisStore) {
                $redis = Cache::getStore()->connection();
                return $redis->keys(self::PREFIX . self::VERSION . ':' . $pattern);
            }

            return [];
        } catch (\Exception $e) {
            Log::error('Cache keys search failed', [
                'pattern' => $pattern,
                'error' => $e->getMessage(),
            ]);

            return [];
        }
    }

    /**
     * Clear cache by pattern (debug only).
     */
    public function clearPattern(string $pattern): int
    {
        if (!app()->environment('local', 'development')) {
            return 0;
        }

        $keys = $this->getKeysMatching($pattern);
        $cleared = 0;

        foreach ($keys as $key) {
            if (Cache::forget($key)) {
                $cleared++;
            }
        }

        return $cleared;
    }

    // =========================================================================
    // CACHE HEALTH CHECKS
    // =========================================================================

    /**
     * Check if cache is working properly.
     */
    public function healthCheck(): array
    {
        $testKey = 'health_check_' . time();
        $testValue = 'ok';

        try {
            // Test write
            $writeSuccess = $this->put($testKey, $testValue, 60);

            // Test read
            $readValue = $this->get($testKey);
            $readSuccess = $readValue === $testValue;

            // Test delete
            $deleteSuccess = $this->forget($testKey);

            $healthy = $writeSuccess && $readSuccess && $deleteSuccess;

            return [
                'healthy' => $healthy,
                'write' => $writeSuccess,
                'read' => $readSuccess,
                'delete' => $deleteSuccess,
                'driver' => config('cache.default'),
                'timestamp' => now()->toIso8601String(),
            ];
        } catch (\Exception $e) {
            return [
                'healthy' => false,
                'error' => $e->getMessage(),
                'driver' => config('cache.default'),
                'timestamp' => now()->toIso8601String(),
            ];
        }
    }
}