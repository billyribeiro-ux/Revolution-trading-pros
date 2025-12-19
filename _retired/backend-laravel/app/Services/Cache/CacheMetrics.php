<?php

declare(strict_types=1);

namespace App\Services\Cache;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Redis;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * Cache Metrics Service (ICT9+ Enterprise Grade)
 *
 * Comprehensive cache metrics and monitoring:
 * - Hit/miss rates per cache tier
 * - Memory usage tracking
 * - Response time monitoring
 * - Cache efficiency analysis
 * - Alerting thresholds
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class CacheMetrics
{
    /**
     * Metric keys
     */
    private const METRICS_PREFIX = 'cache:metrics:';

    /**
     * Alerting thresholds
     */
    private const THRESHOLDS = [
        'min_hit_rate' => 80,        // Alert if hit rate below 80%
        'max_memory_pct' => 90,      // Alert if memory usage above 90%
        'max_evictions_per_min' => 100, // Alert if too many evictions
        'max_avg_response_ms' => 50, // Alert if avg response > 50ms
    ];

    /**
     * Record a cache operation
     */
    public function record(string $tier, string $operation, float $durationMs = 0, int $size = 0): void
    {
        $timestamp = time();
        $minute = (int) ($timestamp / 60) * 60;

        try {
            // Increment operation counters
            Cache::increment(self::METRICS_PREFIX . "{$tier}:{$operation}");
            Cache::increment(self::METRICS_PREFIX . "{$tier}:{$operation}:{$minute}");

            // Track response times
            if ($durationMs > 0) {
                $this->recordResponseTime($tier, $durationMs);
            }

            // Track sizes
            if ($size > 0) {
                $this->recordSize($tier, $size);
            }
        } catch (\Throwable $e) {
            // Silently ignore metric errors
        }
    }

    /**
     * Record response time
     */
    private function recordResponseTime(string $tier, float $durationMs): void
    {
        $key = self::METRICS_PREFIX . "{$tier}:response_times";

        // Keep last 1000 response times
        Redis::lpush($key, $durationMs);
        Redis::ltrim($key, 0, 999);
    }

    /**
     * Record cache entry size
     */
    private function recordSize(string $tier, int $size): void
    {
        $key = self::METRICS_PREFIX . "{$tier}:total_size";
        Cache::increment($key, $size);
    }

    /**
     * Get comprehensive metrics
     */
    public function getMetrics(): array
    {
        return [
            'l1' => $this->getTierMetrics('l1'),
            'l2' => $this->getTierMetrics('l2'),
            'l3' => $this->getTierMetrics('l3'),
            'overall' => $this->getOverallMetrics(),
            'redis' => $this->getRedisMetrics(),
            'database' => $this->getDatabaseCacheMetrics(),
            'alerts' => $this->checkAlerts(),
            'timestamp' => now()->toIso8601String(),
        ];
    }

    /**
     * Get metrics for specific cache tier
     */
    public function getTierMetrics(string $tier): array
    {
        $hits = (int) Cache::get(self::METRICS_PREFIX . "{$tier}:hit", 0);
        $misses = (int) Cache::get(self::METRICS_PREFIX . "{$tier}:miss", 0);
        $total = $hits + $misses;

        return [
            'hits' => $hits,
            'misses' => $misses,
            'total' => $total,
            'hit_rate' => $total > 0 ? round(($hits / $total) * 100, 2) : 0,
            'avg_response_ms' => $this->getAverageResponseTime($tier),
            'p95_response_ms' => $this->getPercentileResponseTime($tier, 95),
            'p99_response_ms' => $this->getPercentileResponseTime($tier, 99),
            'total_size_bytes' => (int) Cache::get(self::METRICS_PREFIX . "{$tier}:total_size", 0),
        ];
    }

    /**
     * Get overall cache metrics
     */
    private function getOverallMetrics(): array
    {
        $l1 = $this->getTierMetrics('l1');
        $l2 = $this->getTierMetrics('l2');
        $l3 = $this->getTierMetrics('l3');

        $totalHits = $l1['hits'] + $l2['hits'] + $l3['hits'];
        $totalMisses = $l1['misses'] + $l2['misses'] + $l3['misses'];
        $total = $totalHits + $totalMisses;

        return [
            'total_hits' => $totalHits,
            'total_misses' => $totalMisses,
            'total_requests' => $total,
            'overall_hit_rate' => $total > 0 ? round(($totalHits / $total) * 100, 2) : 0,
            'l1_efficiency' => $total > 0 ? round(($l1['hits'] / $total) * 100, 2) : 0,
            'cache_savings_pct' => $this->calculateCacheSavings($l1, $l2, $l3),
        ];
    }

    /**
     * Get Redis-specific metrics
     */
    private function getRedisMetrics(): array
    {
        try {
            $info = Redis::info();

            return [
                'used_memory' => $info['used_memory_human'] ?? 'N/A',
                'used_memory_bytes' => $info['used_memory'] ?? 0,
                'max_memory' => $info['maxmemory_human'] ?? 'N/A',
                'memory_usage_pct' => $this->calculateMemoryPct($info),
                'connected_clients' => $info['connected_clients'] ?? 0,
                'total_connections' => $info['total_connections_received'] ?? 0,
                'keyspace_hits' => $info['keyspace_hits'] ?? 0,
                'keyspace_misses' => $info['keyspace_misses'] ?? 0,
                'evicted_keys' => $info['evicted_keys'] ?? 0,
                'expired_keys' => $info['expired_keys'] ?? 0,
                'uptime_seconds' => $info['uptime_in_seconds'] ?? 0,
                'db_size' => $this->getDbSize(),
            ];
        } catch (\Throwable $e) {
            return ['error' => $e->getMessage()];
        }
    }

    /**
     * Get database cache metrics (L3)
     */
    private function getDatabaseCacheMetrics(): array
    {
        try {
            return [
                'total_entries' => DB::table('cache_l3')->count(),
                'expired_entries' => DB::table('cache_l3')->where('expires_at', '<', now())->count(),
                'total_size_bytes' => DB::table('cache_l3')->sum(DB::raw('LENGTH(value)')),
                'oldest_entry' => DB::table('cache_l3')->min('created_at'),
                'newest_entry' => DB::table('cache_l3')->max('created_at'),
            ];
        } catch (\Throwable $e) {
            return ['error' => $e->getMessage()];
        }
    }

    /**
     * Get average response time for tier
     */
    private function getAverageResponseTime(string $tier): float
    {
        try {
            $key = self::METRICS_PREFIX . "{$tier}:response_times";
            $times = Redis::lrange($key, 0, -1);

            if (empty($times)) {
                return 0;
            }

            $times = array_map('floatval', $times);
            return round(array_sum($times) / count($times), 2);
        } catch (\Throwable) {
            return 0;
        }
    }

    /**
     * Get percentile response time
     */
    private function getPercentileResponseTime(string $tier, int $percentile): float
    {
        try {
            $key = self::METRICS_PREFIX . "{$tier}:response_times";
            $times = Redis::lrange($key, 0, -1);

            if (empty($times)) {
                return 0;
            }

            $times = array_map('floatval', $times);
            sort($times);

            $index = (int) ceil(($percentile / 100) * count($times)) - 1;
            $index = max(0, min($index, count($times) - 1));

            return round($times[$index], 2);
        } catch (\Throwable) {
            return 0;
        }
    }

    /**
     * Calculate memory usage percentage
     */
    private function calculateMemoryPct(array $info): float
    {
        $used = $info['used_memory'] ?? 0;
        $max = $info['maxmemory'] ?? 0;

        if ($max === 0) {
            return 0;
        }

        return round(($used / $max) * 100, 2);
    }

    /**
     * Get Redis database size
     */
    private function getDbSize(): int
    {
        try {
            return Redis::dbsize();
        } catch (\Throwable) {
            return 0;
        }
    }

    /**
     * Calculate cache savings percentage
     */
    private function calculateCacheSavings(array $l1, array $l2, array $l3): float
    {
        // Estimate that L1 saves 10ms, L2 saves 5ms, L3 saves 2ms per request
        $savedMs = ($l1['hits'] * 10) + ($l2['hits'] * 5) + ($l3['hits'] * 2);
        $totalRequests = $l1['total'] + $l2['total'] + $l3['total'];

        if ($totalRequests === 0) {
            return 0;
        }

        // Calculate as percentage of potential time saved vs actual
        $potentialMs = $totalRequests * 10; // All requests taking 10ms
        return round(($savedMs / $potentialMs) * 100, 2);
    }

    /**
     * Check alerting thresholds
     */
    public function checkAlerts(): array
    {
        $alerts = [];

        // Check overall hit rate
        $overall = $this->getOverallMetrics();
        if ($overall['overall_hit_rate'] < self::THRESHOLDS['min_hit_rate']) {
            $alerts[] = [
                'level' => 'warning',
                'message' => "Cache hit rate ({$overall['overall_hit_rate']}%) below threshold (" . self::THRESHOLDS['min_hit_rate'] . "%)",
                'metric' => 'hit_rate',
                'value' => $overall['overall_hit_rate'],
            ];
        }

        // Check Redis memory
        $redis = $this->getRedisMetrics();
        if (isset($redis['memory_usage_pct']) && $redis['memory_usage_pct'] > self::THRESHOLDS['max_memory_pct']) {
            $alerts[] = [
                'level' => 'critical',
                'message' => "Redis memory usage ({$redis['memory_usage_pct']}%) above threshold (" . self::THRESHOLDS['max_memory_pct'] . "%)",
                'metric' => 'memory_usage',
                'value' => $redis['memory_usage_pct'],
            ];
        }

        // Check L2 response time
        $l2 = $this->getTierMetrics('l2');
        if ($l2['avg_response_ms'] > self::THRESHOLDS['max_avg_response_ms']) {
            $alerts[] = [
                'level' => 'warning',
                'message' => "L2 cache response time ({$l2['avg_response_ms']}ms) above threshold (" . self::THRESHOLDS['max_avg_response_ms'] . "ms)",
                'metric' => 'response_time',
                'value' => $l2['avg_response_ms'],
            ];
        }

        return $alerts;
    }

    /**
     * Reset all metrics
     */
    public function reset(): void
    {
        try {
            $keys = Redis::keys(self::METRICS_PREFIX . '*');
            if (!empty($keys)) {
                Redis::del($keys);
            }
            Log::info('Cache metrics reset');
        } catch (\Throwable $e) {
            Log::error('Failed to reset cache metrics', ['error' => $e->getMessage()]);
        }
    }

    /**
     * Get historical metrics (last N minutes)
     */
    public function getHistorical(int $minutes = 60): array
    {
        $now = time();
        $data = [];

        for ($i = 0; $i < $minutes; $i++) {
            $minute = (int) (($now - ($i * 60)) / 60) * 60;

            $data[] = [
                'timestamp' => date('Y-m-d H:i:00', $minute),
                'l1_hits' => (int) Cache::get(self::METRICS_PREFIX . "l1:hit:{$minute}", 0),
                'l1_misses' => (int) Cache::get(self::METRICS_PREFIX . "l1:miss:{$minute}", 0),
                'l2_hits' => (int) Cache::get(self::METRICS_PREFIX . "l2:hit:{$minute}", 0),
                'l2_misses' => (int) Cache::get(self::METRICS_PREFIX . "l2:miss:{$minute}", 0),
            ];
        }

        return array_reverse($data);
    }
}
