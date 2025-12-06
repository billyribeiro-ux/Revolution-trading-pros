<?php

declare(strict_types=1);

namespace App\Services\Cache;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;
use Illuminate\Contracts\Cache\Repository;
use Closure;

/**
 * Multi-Tier Cache Manager (ICT9+ Enterprise Grade)
 *
 * Three-tier caching architecture:
 * - L1: Request-level in-memory cache (fastest, per-request)
 * - L2: Redis/Memcached distributed cache (fast, shared)
 * - L3: Database-backed cache (persistent, fallback)
 *
 * Features:
 * - Automatic tier fallback and promotion
 * - Write-through and write-behind strategies
 * - Cache stampede protection with probabilistic early expiration
 * - Adaptive TTL based on access patterns
 * - Compression for large values
 * - Circuit breaker for failing cache stores
 *
 * @version 2.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class MultiTierCacheManager
{
    /**
     * L1 in-memory cache (request-scoped)
     */
    private static array $l1Cache = [];

    /**
     * L1 cache hit counts for adaptive TTL
     */
    private static array $l1HitCounts = [];

    /**
     * L2 cache store (Redis/Memcached)
     */
    private Repository $l2Cache;

    /**
     * Circuit breaker state for L2
     */
    private static bool $l2CircuitOpen = false;
    private static int $l2FailureCount = 0;
    private static ?int $l2CircuitOpenedAt = null;

    /**
     * TTL configurations (seconds)
     */
    private const TTL_L1 = 60;           // 1 minute request cache
    private const TTL_L2_SHORT = 300;    // 5 minutes
    private const TTL_L2_MEDIUM = 3600;  // 1 hour
    private const TTL_L2_LONG = 86400;   // 24 hours
    private const TTL_L3 = 604800;       // 7 days (database)

    /**
     * Thresholds
     */
    private const L1_MAX_SIZE = 1000;           // Max L1 entries
    private const L1_MAX_VALUE_SIZE = 65536;    // 64KB max L1 value
    private const L2_COMPRESSION_THRESHOLD = 1024; // 1KB
    private const CIRCUIT_FAILURE_THRESHOLD = 5;
    private const CIRCUIT_RESET_TIMEOUT = 60;   // seconds
    private const EARLY_EXPIRATION_BETA = 1.0;  // XFetch beta parameter

    /**
     * Cache prefixes
     */
    private const PREFIX = 'mtc:'; // Multi-tier cache

    /**
     * Cache tags for different domains
     */
    public const TAGS = [
        'users' => 'mtc:users',
        'contacts' => 'mtc:contacts',
        'crm' => 'mtc:crm',
        'posts' => 'mtc:posts',
        'products' => 'mtc:products',
        'forms' => 'mtc:forms',
        'emails' => 'mtc:emails',
        'seo' => 'mtc:seo',
        'analytics' => 'mtc:analytics',
        'config' => 'mtc:config',
        'sessions' => 'mtc:sessions',
        'workflows' => 'mtc:workflows',
    ];

    public function __construct()
    {
        $this->l2Cache = Cache::store(config('cache.default', 'redis'));
    }

    /**
     * Get value from multi-tier cache with automatic promotion
     *
     * @template T
     * @param string $key
     * @param Closure(): T $callback
     * @param int $ttl
     * @param array $tags
     * @return T
     */
    public function get(string $key, Closure $callback, int $ttl = self::TTL_L2_MEDIUM, array $tags = []): mixed
    {
        $fullKey = self::PREFIX . $key;

        // L1: Check in-memory cache first (fastest)
        if ($value = $this->getFromL1($fullKey)) {
            $this->recordMetric('l1_hit');
            return $value;
        }

        // L2: Check distributed cache
        if (!self::$l2CircuitOpen) {
            try {
                $cached = $this->getFromL2($fullKey, $tags);
                if ($cached !== null) {
                    // Check for probabilistic early expiration (XFetch algorithm)
                    if ($this->shouldRecompute($cached)) {
                        // Recompute in background, return stale value
                        $this->asyncRecompute($key, $callback, $ttl, $tags);
                        return $cached['value'];
                    }

                    // Promote to L1
                    $this->setL1($fullKey, $cached['value']);
                    $this->recordMetric('l2_hit');
                    return $cached['value'];
                }
            } catch (\Throwable $e) {
                $this->handleL2Failure($e);
            }
        }

        // L3: Check database cache (fallback)
        $dbCached = $this->getFromL3($key);
        if ($dbCached !== null) {
            // Promote to L2 and L1
            $this->setL2($fullKey, $dbCached, $ttl, $tags);
            $this->setL1($fullKey, $dbCached);
            $this->recordMetric('l3_hit');
            return $dbCached;
        }

        // Cache miss - compute value with stampede protection
        $this->recordMetric('miss');
        return $this->computeWithLock($key, $callback, $ttl, $tags);
    }

    /**
     * Put value directly into all cache tiers
     */
    public function put(string $key, mixed $value, int $ttl = self::TTL_L2_MEDIUM, array $tags = []): void
    {
        $fullKey = self::PREFIX . $key;

        // Write to all tiers (write-through)
        $this->setL1($fullKey, $value);

        if (!self::$l2CircuitOpen) {
            try {
                $this->setL2($fullKey, $value, $ttl, $tags);
            } catch (\Throwable $e) {
                $this->handleL2Failure($e);
            }
        }

        // Async write to L3 for persistence
        $this->setL3Async($key, $value, $ttl);
    }

    /**
     * Forget value from all cache tiers
     */
    public function forget(string $key): void
    {
        $fullKey = self::PREFIX . $key;

        // Remove from all tiers
        unset(self::$l1Cache[$fullKey]);

        if (!self::$l2CircuitOpen) {
            try {
                $this->l2Cache->forget($fullKey);
            } catch (\Throwable $e) {
                Log::warning('L2 cache forget failed', ['key' => $key, 'error' => $e->getMessage()]);
            }
        }

        $this->forgetL3($key);
    }

    /**
     * Flush cache by tags
     */
    public function flushTags(array $tags): void
    {
        // Flush L1 by prefix matching
        $tagPrefixes = array_map(fn($t) => self::TAGS[$t] ?? $t, $tags);
        foreach (self::$l1Cache as $key => $value) {
            foreach ($tagPrefixes as $prefix) {
                if (str_contains($key, $prefix)) {
                    unset(self::$l1Cache[$key]);
                    break;
                }
            }
        }

        // Flush L2 using Redis tags
        if (!self::$l2CircuitOpen) {
            try {
                Cache::tags($tags)->flush();
            } catch (\Throwable $e) {
                Log::warning('L2 tag flush failed', ['tags' => $tags, 'error' => $e->getMessage()]);
            }
        }

        // Flush L3 by tag
        $this->flushL3Tags($tags);

        Log::info('Multi-tier cache flushed by tags', ['tags' => $tags]);
    }

    /**
     * Flush entire cache
     */
    public function flush(): void
    {
        self::$l1Cache = [];
        self::$l1HitCounts = [];

        if (!self::$l2CircuitOpen) {
            try {
                $this->l2Cache->flush();
            } catch (\Throwable $e) {
                Log::warning('L2 flush failed', ['error' => $e->getMessage()]);
            }
        }

        $this->flushL3();

        Log::info('Multi-tier cache flushed completely');
    }

    // =========================================================================
    // L1 CACHE METHODS (In-Memory)
    // =========================================================================

    private function getFromL1(string $key): mixed
    {
        if (!isset(self::$l1Cache[$key])) {
            return null;
        }

        $entry = self::$l1Cache[$key];

        // Check TTL
        if ($entry['expires_at'] < time()) {
            unset(self::$l1Cache[$key]);
            return null;
        }

        // Track hit count for adaptive TTL
        self::$l1HitCounts[$key] = (self::$l1HitCounts[$key] ?? 0) + 1;

        return $entry['value'];
    }

    private function setL1(string $key, mixed $value): void
    {
        // Check size limits
        $serialized = serialize($value);
        if (strlen($serialized) > self::L1_MAX_VALUE_SIZE) {
            return; // Too large for L1
        }

        // Evict if at capacity (LRU approximation)
        if (count(self::$l1Cache) >= self::L1_MAX_SIZE) {
            $this->evictL1();
        }

        // Adaptive TTL based on hit count
        $hitCount = self::$l1HitCounts[$key] ?? 0;
        $ttl = min(self::TTL_L1 * (1 + $hitCount / 10), 300); // Max 5 minutes

        self::$l1Cache[$key] = [
            'value' => $value,
            'expires_at' => time() + (int) $ttl,
            'size' => strlen($serialized),
        ];
    }

    private function evictL1(): void
    {
        // Remove 10% of least accessed entries
        $toRemove = (int) (count(self::$l1Cache) * 0.1);

        // Sort by hit count (ascending)
        $sorted = self::$l1HitCounts;
        asort($sorted);

        $removed = 0;
        foreach (array_keys($sorted) as $key) {
            if ($removed >= $toRemove) {
                break;
            }
            unset(self::$l1Cache[$key], self::$l1HitCounts[$key]);
            $removed++;
        }
    }

    // =========================================================================
    // L2 CACHE METHODS (Redis/Memcached)
    // =========================================================================

    private function getFromL2(string $key, array $tags = []): ?array
    {
        $cached = !empty($tags)
            ? Cache::tags($tags)->get($key)
            : $this->l2Cache->get($key);

        if ($cached === null) {
            return null;
        }

        // Decompress if needed
        if (is_array($cached) && isset($cached['compressed'])) {
            $cached['value'] = $this->decompress($cached['value']);
        }

        return $cached;
    }

    private function setL2(string $key, mixed $value, int $ttl, array $tags = []): void
    {
        $serialized = serialize($value);
        $shouldCompress = strlen($serialized) > self::L2_COMPRESSION_THRESHOLD;

        $entry = [
            'value' => $shouldCompress ? $this->compress($serialized) : $value,
            'compressed' => $shouldCompress,
            'created_at' => time(),
            'ttl' => $ttl,
            'delta' => $this->computeDelta($ttl), // For XFetch
        ];

        // Add jitter to prevent cache stampede
        $ttlWithJitter = $this->addJitter($ttl);

        if (!empty($tags)) {
            Cache::tags($tags)->put($key, $entry, $ttlWithJitter);
        } else {
            $this->l2Cache->put($key, $entry, $ttlWithJitter);
        }
    }

    private function handleL2Failure(\Throwable $e): void
    {
        self::$l2FailureCount++;
        Log::warning('L2 cache failure', [
            'error' => $e->getMessage(),
            'failure_count' => self::$l2FailureCount,
        ]);

        if (self::$l2FailureCount >= self::CIRCUIT_FAILURE_THRESHOLD) {
            self::$l2CircuitOpen = true;
            self::$l2CircuitOpenedAt = time();
            Log::error('L2 cache circuit breaker opened');
        }
    }

    private function checkCircuitBreaker(): void
    {
        if (self::$l2CircuitOpen && self::$l2CircuitOpenedAt !== null) {
            if (time() - self::$l2CircuitOpenedAt > self::CIRCUIT_RESET_TIMEOUT) {
                // Try to close circuit
                self::$l2CircuitOpen = false;
                self::$l2FailureCount = 0;
                self::$l2CircuitOpenedAt = null;
                Log::info('L2 cache circuit breaker closed');
            }
        }
    }

    // =========================================================================
    // L3 CACHE METHODS (Database)
    // =========================================================================

    private function getFromL3(string $key): mixed
    {
        try {
            $row = DB::table('cache_l3')
                ->where('key', $key)
                ->where('expires_at', '>', now())
                ->first();

            if (!$row) {
                return null;
            }

            $value = unserialize($row->value);

            // Update access time for LRU
            DB::table('cache_l3')
                ->where('key', $key)
                ->update(['last_accessed_at' => now()]);

            return $value;
        } catch (\Throwable $e) {
            Log::warning('L3 cache get failed', ['key' => $key, 'error' => $e->getMessage()]);
            return null;
        }
    }

    private function setL3Async(string $key, mixed $value, int $ttl): void
    {
        // Use queue for async write, fallback to sync
        try {
            dispatch(function () use ($key, $value, $ttl) {
                $this->setL3($key, $value, $ttl);
            })->afterResponse();
        } catch (\Throwable $e) {
            // Fallback to sync write
            $this->setL3($key, $value, $ttl);
        }
    }

    private function setL3(string $key, mixed $value, int $ttl): void
    {
        try {
            DB::table('cache_l3')->updateOrInsert(
                ['key' => $key],
                [
                    'value' => serialize($value),
                    'tags' => json_encode([]),
                    'expires_at' => now()->addSeconds($ttl),
                    'last_accessed_at' => now(),
                    'created_at' => now(),
                    'updated_at' => now(),
                ]
            );
        } catch (\Throwable $e) {
            Log::warning('L3 cache set failed', ['key' => $key, 'error' => $e->getMessage()]);
        }
    }

    private function forgetL3(string $key): void
    {
        try {
            DB::table('cache_l3')->where('key', $key)->delete();
        } catch (\Throwable $e) {
            Log::warning('L3 cache forget failed', ['key' => $key, 'error' => $e->getMessage()]);
        }
    }

    private function flushL3Tags(array $tags): void
    {
        try {
            foreach ($tags as $tag) {
                DB::table('cache_l3')
                    ->whereJsonContains('tags', $tag)
                    ->delete();
            }
        } catch (\Throwable $e) {
            Log::warning('L3 tag flush failed', ['tags' => $tags, 'error' => $e->getMessage()]);
        }
    }

    private function flushL3(): void
    {
        try {
            DB::table('cache_l3')->truncate();
        } catch (\Throwable $e) {
            Log::warning('L3 flush failed', ['error' => $e->getMessage()]);
        }
    }

    // =========================================================================
    // STAMPEDE PROTECTION & XFETCH
    // =========================================================================

    /**
     * Compute value with lock (stampede protection)
     */
    private function computeWithLock(string $key, Closure $callback, int $ttl, array $tags): mixed
    {
        $lockKey = self::PREFIX . $key . ':lock';
        $lock = Cache::lock($lockKey, 30);

        try {
            if ($lock->get()) {
                // Recheck caches after acquiring lock
                $fullKey = self::PREFIX . $key;

                if ($value = $this->getFromL1($fullKey)) {
                    return $value;
                }

                if (!self::$l2CircuitOpen) {
                    $cached = $this->getFromL2($fullKey, $tags);
                    if ($cached !== null) {
                        $this->setL1($fullKey, $cached['value']);
                        return $cached['value'];
                    }
                }

                // Compute value
                $startTime = microtime(true);
                $value = $callback();
                $computeTime = microtime(true) - $startTime;

                // Store in all tiers with computed delta
                $this->setL1($fullKey, $value);
                if (!self::$l2CircuitOpen) {
                    try {
                        $entry = [
                            'value' => $value,
                            'compressed' => false,
                            'created_at' => time(),
                            'ttl' => $ttl,
                            'delta' => $computeTime,
                        ];
                        $ttlWithJitter = $this->addJitter($ttl);

                        if (!empty($tags)) {
                            Cache::tags($tags)->put($fullKey, $entry, $ttlWithJitter);
                        } else {
                            $this->l2Cache->put($fullKey, $entry, $ttlWithJitter);
                        }
                    } catch (\Throwable $e) {
                        $this->handleL2Failure($e);
                    }
                }
                $this->setL3Async($key, $value, $ttl);

                return $value;
            }

            // Could not acquire lock, wait and retry
            usleep(50000); // 50ms
            return $this->get($key, $callback, $ttl, $tags);
        } finally {
            optional($lock)->release();
        }
    }

    /**
     * XFetch: Should we recompute before expiration?
     * Probabilistic early expiration to prevent stampede
     */
    private function shouldRecompute(array $cached): bool
    {
        $now = time();
        $createdAt = $cached['created_at'] ?? $now;
        $ttl = $cached['ttl'] ?? 3600;
        $delta = $cached['delta'] ?? 1;

        $expiresAt = $createdAt + $ttl;
        $remainingTtl = $expiresAt - $now;

        // XFetch formula: random() < delta * beta * log(random())
        // If remaining TTL is less than delta * beta * log(1/random), recompute early
        $threshold = $delta * self::EARLY_EXPIRATION_BETA * log(1 / (mt_rand() / mt_getrandmax() + 0.001));

        return $remainingTtl < $threshold;
    }

    /**
     * Async recompute (fire and forget)
     */
    private function asyncRecompute(string $key, Closure $callback, int $ttl, array $tags): void
    {
        try {
            dispatch(function () use ($key, $callback, $ttl, $tags) {
                $value = $callback();
                $this->put($key, $value, $ttl, $tags);
            })->afterResponse();
        } catch (\Throwable $e) {
            // Ignore - we'll return stale value
        }
    }

    private function computeDelta(int $ttl): float
    {
        // Estimate compute time as fraction of TTL
        return min($ttl * 0.01, 5.0); // Max 5 seconds
    }

    // =========================================================================
    // UTILITIES
    // =========================================================================

    private function compress(string $data): string
    {
        return base64_encode(gzcompress($data, 6));
    }

    private function decompress(string $data): mixed
    {
        $decompressed = gzuncompress(base64_decode($data));
        return $decompressed !== false ? unserialize($decompressed) : null;
    }

    private function addJitter(int $ttl): int
    {
        // Add 0-10% jitter to prevent thundering herd
        return $ttl + (int) ($ttl * (mt_rand(0, 100) / 1000));
    }

    private function recordMetric(string $type): void
    {
        try {
            Cache::increment("cache:mtc:metrics:{$type}");
        } catch (\Throwable) {
            // Ignore metric errors
        }
    }

    /**
     * Get cache statistics
     */
    public function getMetrics(): array
    {
        $l1Hits = Cache::get('cache:mtc:metrics:l1_hit', 0);
        $l2Hits = Cache::get('cache:mtc:metrics:l2_hit', 0);
        $l3Hits = Cache::get('cache:mtc:metrics:l3_hit', 0);
        $misses = Cache::get('cache:mtc:metrics:miss', 0);
        $total = $l1Hits + $l2Hits + $l3Hits + $misses;

        return [
            'l1' => [
                'hits' => $l1Hits,
                'size' => count(self::$l1Cache),
                'memory_bytes' => array_sum(array_column(self::$l1Cache, 'size')),
            ],
            'l2' => [
                'hits' => $l2Hits,
                'circuit_open' => self::$l2CircuitOpen,
                'failure_count' => self::$l2FailureCount,
            ],
            'l3' => [
                'hits' => $l3Hits,
            ],
            'misses' => $misses,
            'total_requests' => $total,
            'hit_rate' => $total > 0 ? round((($l1Hits + $l2Hits + $l3Hits) / $total) * 100, 2) : 0,
            'l1_hit_rate' => $total > 0 ? round(($l1Hits / $total) * 100, 2) : 0,
        ];
    }

    /**
     * Reset metrics
     */
    public function resetMetrics(): void
    {
        Cache::forget('cache:mtc:metrics:l1_hit');
        Cache::forget('cache:mtc:metrics:l2_hit');
        Cache::forget('cache:mtc:metrics:l3_hit');
        Cache::forget('cache:mtc:metrics:miss');
    }

    /**
     * Get L1 cache info (for debugging)
     */
    public function getL1Info(): array
    {
        return [
            'count' => count(self::$l1Cache),
            'max_size' => self::L1_MAX_SIZE,
            'keys' => array_keys(self::$l1Cache),
        ];
    }

    /**
     * Clear L1 cache only (useful between requests in testing)
     */
    public static function clearL1(): void
    {
        self::$l1Cache = [];
        self::$l1HitCounts = [];
    }
}
