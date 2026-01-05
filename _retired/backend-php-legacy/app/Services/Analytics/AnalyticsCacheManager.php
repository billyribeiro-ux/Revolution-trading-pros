<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Redis;

/**
 * AnalyticsCacheManager - Enterprise Redis Cache Management
 *
 * Manages caching strategy for analytics data with TTL optimization,
 * cache warming, and invalidation patterns.
 *
 * @package App\Services\Analytics
 */
class AnalyticsCacheManager
{
    /**
     * Cache prefix for analytics data
     */
    private const CACHE_PREFIX = 'analytics:';

    /**
     * Real-time metrics cache prefix
     */
    private const REALTIME_PREFIX = 'analytics:realtime:';

    /**
     * Default TTL in seconds
     */
    private const DEFAULT_TTL = 3600;

    /**
     * Short TTL for frequently changing data
     */
    private const SHORT_TTL = 300;

    /**
     * Long TTL for stable data
     */
    private const LONG_TTL = 86400;

    /**
     * Remember a value in cache
     */
    public function remember(string $key, callable $callback, int $ttl = self::DEFAULT_TTL): mixed
    {
        $cacheKey = self::CACHE_PREFIX . $key;

        return Cache::remember($cacheKey, $ttl, $callback);
    }

    /**
     * Get value from cache
     */
    public function get(string $key): mixed
    {
        return Cache::get(self::CACHE_PREFIX . $key);
    }

    /**
     * Put value in cache
     */
    public function put(string $key, mixed $value, int $ttl = self::DEFAULT_TTL): void
    {
        Cache::put(self::CACHE_PREFIX . $key, $value, $ttl);
    }

    /**
     * Forget a cached value
     */
    public function forget(string $key): void
    {
        Cache::forget(self::CACHE_PREFIX . $key);
    }

    /**
     * Invalidate KPI cache
     */
    public function invalidateKpi(string $kpiKey): void
    {
        $patterns = [
            "kpi:{$kpiKey}:*",
            "dashboard:kpis:*",
        ];

        $this->invalidatePatterns($patterns);
    }

    /**
     * Invalidate cohort cache
     */
    public function invalidateCohort(string $cohortKey): void
    {
        $patterns = [
            "cohort:{$cohortKey}:*",
            "cohort:matrix:{$cohortKey}:*",
        ];

        $this->invalidatePatterns($patterns);
    }

    /**
     * Invalidate funnel cache
     */
    public function invalidateFunnel(string $funnelKey): void
    {
        $patterns = [
            "funnel:{$funnelKey}:*",
            "funnel:analysis:{$funnelKey}:*",
        ];

        $this->invalidatePatterns($patterns);
    }

    /**
     * Invalidate segment cache
     */
    public function invalidateSegment(string $segmentKey): void
    {
        $patterns = [
            "segment:{$segmentKey}:*",
        ];

        $this->invalidatePatterns($patterns);
    }

    /**
     * Invalidate all analytics cache
     */
    public function invalidateAll(): void
    {
        $this->invalidatePatterns([self::CACHE_PREFIX . '*']);
    }

    /**
     * Invalidate cache patterns
     */
    protected function invalidatePatterns(array $patterns): void
    {
        foreach ($patterns as $pattern) {
            $fullPattern = self::CACHE_PREFIX . $pattern;

            // Use Redis SCAN for pattern matching if available
            if ($this->isRedisAvailable()) {
                $this->invalidateRedisPattern($fullPattern);
            } else {
                // Fall back to forgetting specific keys
                Cache::forget($fullPattern);
            }
        }
    }

    /**
     * Check if Redis is available
     */
    protected function isRedisAvailable(): bool
    {
        try {
            return Cache::getStore() instanceof \Illuminate\Cache\RedisStore;
        } catch (\Exception) {
            return false;
        }
    }

    /**
     * Invalidate Redis keys by pattern
     */
    protected function invalidateRedisPattern(string $pattern): void
    {
        try {
            $redis = Redis::connection();
            $cursor = 0;

            do {
                [$cursor, $keys] = $redis->scan($cursor, 'MATCH', $pattern, 'COUNT', 100);

                if (!empty($keys)) {
                    $redis->del(...$keys);
                }
            } while ($cursor != 0);
        } catch (\Exception) {
            // Silently fail if Redis is not available
        }
    }

    /**
     * Increment real-time counter
     */
    public function incrementRealTimeCounter(string $eventName): void
    {
        $key = self::REALTIME_PREFIX . 'events:' . date('YmdH') . ':' . $eventName;

        if ($this->isRedisAvailable()) {
            try {
                Redis::incr($key);
                Redis::expire($key, 7200); // 2 hours
            } catch (\Exception) {
                // Silently fail
            }
        } else {
            $current = Cache::get($key, 0);
            Cache::put($key, $current + 1, 7200);
        }
    }

    /**
     * Get real-time counter
     */
    public function getRealTimeCounter(string $eventName, ?string $hour = null): int
    {
        $hour = $hour ?? date('YmdH');
        $key = self::REALTIME_PREFIX . 'events:' . $hour . ':' . $eventName;

        if ($this->isRedisAvailable()) {
            try {
                return (int) Redis::get($key);
            } catch (\Exception) {
                return 0;
            }
        }

        return (int) Cache::get($key, 0);
    }

    /**
     * Get real-time metrics summary
     */
    public function getRealTimeMetrics(): array
    {
        $currentHour = date('YmdH');
        $previousHour = date('YmdH', strtotime('-1 hour'));

        return [
            'current_hour' => $this->getHourlyMetrics($currentHour),
            'previous_hour' => $this->getHourlyMetrics($previousHour),
        ];
    }

    /**
     * Get hourly metrics
     */
    protected function getHourlyMetrics(string $hour): array
    {
        $metrics = [];
        $pattern = self::REALTIME_PREFIX . 'events:' . $hour . ':*';

        if ($this->isRedisAvailable()) {
            try {
                $redis = Redis::connection();
                $cursor = 0;

                do {
                    [$cursor, $keys] = $redis->scan($cursor, 'MATCH', $pattern, 'COUNT', 100);

                    foreach ($keys as $key) {
                        $eventName = str_replace(self::REALTIME_PREFIX . 'events:' . $hour . ':', '', $key);
                        $metrics[$eventName] = (int) $redis->get($key);
                    }
                } while ($cursor != 0);
            } catch (\Exception) {
                // Return empty metrics
            }
        }

        return $metrics;
    }

    /**
     * Cache dashboard data with optimal TTL
     */
    public function cacheDashboard(string $dashboardKey, array $data): void
    {
        $this->put("dashboard:{$dashboardKey}", $data, self::SHORT_TTL);
    }

    /**
     * Get cached dashboard data
     */
    public function getDashboard(string $dashboardKey): ?array
    {
        return $this->get("dashboard:{$dashboardKey}");
    }

    /**
     * Cache historical report with long TTL
     */
    public function cacheHistoricalReport(string $reportKey, array $data): void
    {
        $this->put("report:{$reportKey}", $data, self::LONG_TTL);
    }

    /**
     * Warm cache for common queries
     */
    public function warmCache(array $kpiKeys, array $periods): void
    {
        foreach ($kpiKeys as $kpiKey) {
            foreach ($periods as $period) {
                $cacheKey = "kpi:{$kpiKey}:{$period}";

                // Only warm if not already cached
                if (!$this->get($cacheKey)) {
                    // The actual computation would be done by KPIComputationEngine
                    // This just marks the cache as needing warming
                    $this->put("warmup:{$cacheKey}", true, 60);
                }
            }
        }
    }

    /**
     * Get cache statistics
     */
    public function getStats(): array
    {
        if (!$this->isRedisAvailable()) {
            return ['driver' => 'non-redis', 'stats' => 'unavailable'];
        }

        try {
            $redis = Redis::connection();
            $info = $redis->info();

            return [
                'driver' => 'redis',
                'used_memory' => $info['used_memory_human'] ?? 'unknown',
                'connected_clients' => $info['connected_clients'] ?? 0,
                'total_keys' => $redis->dbsize(),
                'analytics_keys' => $this->countAnalyticsKeys(),
            ];
        } catch (\Exception $e) {
            return ['driver' => 'redis', 'error' => $e->getMessage()];
        }
    }

    /**
     * Count analytics keys in cache
     */
    protected function countAnalyticsKeys(): int
    {
        if (!$this->isRedisAvailable()) {
            return 0;
        }

        try {
            $redis = Redis::connection();
            $count = 0;
            $cursor = 0;

            do {
                [$cursor, $keys] = $redis->scan($cursor, 'MATCH', self::CACHE_PREFIX . '*', 'COUNT', 100);
                $count += count($keys);
            } while ($cursor != 0);

            return $count;
        } catch (\Exception) {
            return 0;
        }
    }
}
