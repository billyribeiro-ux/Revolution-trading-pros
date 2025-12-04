<?php

declare(strict_types=1);

namespace App\Services;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Redis;
use Illuminate\Support\Facades\Log;

/**
 * PerformanceOptimizationService - Enterprise Website Speed Optimization
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive performance optimization for maximum website speed.
 *
 * Features:
 * - Query optimization and caching
 * - Response compression configuration
 * - Asset optimization recommendations
 * - Database query analysis
 * - Cache warming strategies
 * - CDN integration helpers
 * - Core Web Vitals optimization
 * - Memory usage optimization
 *
 * @version 1.0.0
 */
class PerformanceOptimizationService
{
    private array $queryLog = [];
    private float $startTime;

    public function __construct()
    {
        $this->startTime = microtime(true);
    }

    /**
     * Get comprehensive performance metrics
     */
    public function getPerformanceMetrics(): array
    {
        return [
            'server' => $this->getServerMetrics(),
            'database' => $this->getDatabaseMetrics(),
            'cache' => $this->getCacheMetrics(),
            'memory' => $this->getMemoryMetrics(),
            'response_time' => $this->getResponseTimeMetrics(),
            'recommendations' => $this->getOptimizationRecommendations(),
        ];
    }

    /**
     * Get server performance metrics
     */
    public function getServerMetrics(): array
    {
        return [
            'php_version' => PHP_VERSION,
            'opcache_enabled' => function_exists('opcache_get_status') && opcache_get_status() !== false,
            'opcache_stats' => function_exists('opcache_get_status') ? $this->getOpcacheStats() : null,
            'max_execution_time' => ini_get('max_execution_time'),
            'memory_limit' => ini_get('memory_limit'),
            'upload_max_filesize' => ini_get('upload_max_filesize'),
            'post_max_size' => ini_get('post_max_size'),
            'loaded_extensions' => $this->getPerformanceExtensions(),
        ];
    }

    /**
     * Get database performance metrics
     */
    public function getDatabaseMetrics(): array
    {
        $slowQueries = [];

        try {
            // Get slow query log if available
            $slowQueryLog = DB::select("SHOW VARIABLES LIKE 'slow_query_log'");
            $longQueryTime = DB::select("SHOW VARIABLES LIKE 'long_query_time'");

            // Get table sizes
            $tableSizes = DB::select("
                SELECT
                    table_name,
                    ROUND(((data_length + index_length) / 1024 / 1024), 2) AS size_mb,
                    table_rows
                FROM information_schema.tables
                WHERE table_schema = ?
                ORDER BY (data_length + index_length) DESC
                LIMIT 20
            ", [config('database.connections.mysql.database')]);

            // Get index usage
            $indexStats = DB::select("
                SELECT
                    table_name,
                    index_name,
                    seq_in_index,
                    column_name
                FROM information_schema.statistics
                WHERE table_schema = ?
                LIMIT 50
            ", [config('database.connections.mysql.database')]);

        } catch (\Exception $e) {
            $tableSizes = [];
            $indexStats = [];
        }

        return [
            'connection_pool' => $this->getConnectionPoolStatus(),
            'slow_query_log_enabled' => ($slowQueryLog[0]->Value ?? 'OFF') === 'ON',
            'long_query_time' => $longQueryTime[0]->Value ?? '10',
            'table_sizes' => $tableSizes,
            'index_count' => count($indexStats),
            'query_cache' => $this->getQueryCacheStatus(),
        ];
    }

    /**
     * Get cache performance metrics
     */
    public function getCacheMetrics(): array
    {
        $driver = config('cache.default');
        $metrics = [
            'driver' => $driver,
            'prefix' => config('cache.prefix'),
        ];

        if ($driver === 'redis') {
            try {
                $info = Redis::info();
                $metrics['redis'] = [
                    'version' => $info['redis_version'] ?? 'unknown',
                    'used_memory' => $info['used_memory_human'] ?? 'unknown',
                    'connected_clients' => $info['connected_clients'] ?? 0,
                    'total_connections' => $info['total_connections_received'] ?? 0,
                    'keyspace_hits' => $info['keyspace_hits'] ?? 0,
                    'keyspace_misses' => $info['keyspace_misses'] ?? 0,
                    'hit_rate' => $this->calculateHitRate(
                        $info['keyspace_hits'] ?? 0,
                        $info['keyspace_misses'] ?? 0
                    ),
                    'evicted_keys' => $info['evicted_keys'] ?? 0,
                ];
            } catch (\Exception $e) {
                $metrics['redis'] = ['error' => 'Could not connect to Redis'];
            }
        }

        return $metrics;
    }

    /**
     * Get memory usage metrics
     */
    public function getMemoryMetrics(): array
    {
        return [
            'current_usage' => $this->formatBytes(memory_get_usage(true)),
            'peak_usage' => $this->formatBytes(memory_get_peak_usage(true)),
            'limit' => ini_get('memory_limit'),
            'usage_percentage' => $this->calculateMemoryUsagePercentage(),
        ];
    }

    /**
     * Get response time metrics
     */
    public function getResponseTimeMetrics(): array
    {
        $elapsed = (microtime(true) - $this->startTime) * 1000;

        return [
            'current_request_ms' => round($elapsed, 2),
            'ttfb_target_ms' => 200, // Time to First Byte target
            'status' => $elapsed < 200 ? 'excellent' : ($elapsed < 500 ? 'good' : 'needs_improvement'),
        ];
    }

    /**
     * Get optimization recommendations
     */
    public function getOptimizationRecommendations(): array
    {
        $recommendations = [];

        // OPcache check
        if (!function_exists('opcache_get_status') || !opcache_get_status()) {
            $recommendations[] = [
                'type' => 'critical',
                'category' => 'php',
                'title' => 'Enable OPcache',
                'description' => 'OPcache significantly improves PHP performance by caching compiled bytecode.',
                'impact' => 'high',
            ];
        }

        // Memory check
        $memoryLimit = $this->parseMemoryLimit(ini_get('memory_limit'));
        if ($memoryLimit < 256 * 1024 * 1024) {
            $recommendations[] = [
                'type' => 'warning',
                'category' => 'php',
                'title' => 'Increase memory_limit',
                'description' => 'Consider increasing memory_limit to at least 256M for better performance.',
                'impact' => 'medium',
            ];
        }

        // Redis check
        if (config('cache.default') !== 'redis') {
            $recommendations[] = [
                'type' => 'warning',
                'category' => 'cache',
                'title' => 'Use Redis for caching',
                'description' => 'Redis provides faster caching than file-based caching.',
                'impact' => 'high',
            ];
        }

        // Queue driver check
        if (config('queue.default') === 'sync') {
            $recommendations[] = [
                'type' => 'critical',
                'category' => 'queue',
                'title' => 'Use async queue driver',
                'description' => 'Sync queue blocks requests. Use Redis or database queue for async processing.',
                'impact' => 'high',
            ];
        }

        // Session driver check
        if (config('session.driver') === 'file') {
            $recommendations[] = [
                'type' => 'warning',
                'category' => 'session',
                'title' => 'Use Redis for sessions',
                'description' => 'Redis sessions are faster and support horizontal scaling.',
                'impact' => 'medium',
            ];
        }

        return $recommendations;
    }

    /**
     * Warm critical caches
     */
    public function warmCaches(): array
    {
        $warmed = [];

        // Warm route cache
        try {
            \Artisan::call('route:cache');
            $warmed[] = 'routes';
        } catch (\Exception $e) {
            Log::warning('Failed to cache routes: ' . $e->getMessage());
        }

        // Warm config cache
        try {
            \Artisan::call('config:cache');
            $warmed[] = 'config';
        } catch (\Exception $e) {
            Log::warning('Failed to cache config: ' . $e->getMessage());
        }

        // Warm view cache
        try {
            \Artisan::call('view:cache');
            $warmed[] = 'views';
        } catch (\Exception $e) {
            Log::warning('Failed to cache views: ' . $e->getMessage());
        }

        return [
            'warmed' => $warmed,
            'timestamp' => now()->toIso8601String(),
        ];
    }

    /**
     * Get CDN configuration recommendations
     */
    public function getCdnRecommendations(): array
    {
        return [
            'recommended_providers' => [
                [
                    'name' => 'Cloudflare',
                    'features' => ['Free tier', 'DDoS protection', 'Edge caching', 'HTTP/3'],
                    'best_for' => 'Most websites',
                ],
                [
                    'name' => 'AWS CloudFront',
                    'features' => ['Global edge network', 'Lambda@Edge', 'S3 integration'],
                    'best_for' => 'AWS-hosted applications',
                ],
                [
                    'name' => 'Fastly',
                    'features' => ['Real-time purging', 'Edge computing', 'Instant config'],
                    'best_for' => 'High-traffic sites needing instant purge',
                ],
            ],
            'cache_headers' => [
                'static_assets' => 'public, max-age=31536000, immutable',
                'html_pages' => 'public, max-age=3600, stale-while-revalidate=86400',
                'api_responses' => 'private, max-age=0, must-revalidate',
                'images' => 'public, max-age=31536000, immutable',
            ],
            'compression' => [
                'gzip' => true,
                'brotli' => true,
                'min_size' => 1024, // bytes
            ],
        ];
    }

    /**
     * Get Core Web Vitals optimization tips
     */
    public function getCoreWebVitalsOptimization(): array
    {
        return [
            'LCP' => [
                'target' => '< 2.5s',
                'optimizations' => [
                    'Preload critical resources (fonts, hero images)',
                    'Use responsive images with srcset',
                    'Implement lazy loading for below-fold images',
                    'Optimize server response time (TTFB < 200ms)',
                    'Use CDN for static assets',
                ],
            ],
            'FID' => [
                'target' => '< 100ms',
                'optimizations' => [
                    'Break up long JavaScript tasks',
                    'Use web workers for heavy processing',
                    'Defer non-critical JavaScript',
                    'Minimize main thread work',
                    'Reduce JavaScript bundle size',
                ],
            ],
            'CLS' => [
                'target' => '< 0.1',
                'optimizations' => [
                    'Set explicit dimensions on images and videos',
                    'Reserve space for dynamic content',
                    'Avoid inserting content above existing content',
                    'Use transform animations instead of layout-triggering properties',
                    'Preload fonts to prevent FOUT/FOIT',
                ],
            ],
            'INP' => [
                'target' => '< 200ms',
                'optimizations' => [
                    'Optimize event handlers',
                    'Use requestIdleCallback for non-urgent work',
                    'Implement virtualization for long lists',
                    'Debounce/throttle frequent events',
                ],
            ],
        ];
    }

    /**
     * Analyze and optimize database queries
     */
    public function analyzeQueries(): array
    {
        DB::enableQueryLog();

        return [
            'query_log_enabled' => true,
            'message' => 'Query logging enabled. Make requests and call getQueryAnalysis() for results.',
        ];
    }

    /**
     * Get query analysis results
     */
    public function getQueryAnalysis(): array
    {
        $queries = DB::getQueryLog();
        $analysis = [];

        foreach ($queries as $query) {
            $time = $query['time'];
            $sql = $query['query'];

            $issues = [];

            // Check for SELECT *
            if (preg_match('/SELECT \*/i', $sql)) {
                $issues[] = 'Avoid SELECT * - specify columns explicitly';
            }

            // Check for missing LIMIT
            if (preg_match('/SELECT/i', $sql) && !preg_match('/LIMIT/i', $sql) && !preg_match('/COUNT|SUM|AVG|MAX|MIN/i', $sql)) {
                $issues[] = 'Consider adding LIMIT to prevent large result sets';
            }

            // Check for slow query
            if ($time > 100) {
                $issues[] = 'Slow query (>100ms) - consider adding indexes';
            }

            $analysis[] = [
                'sql' => $sql,
                'time_ms' => $time,
                'bindings' => $query['bindings'],
                'issues' => $issues,
                'status' => empty($issues) ? 'ok' : 'needs_review',
            ];
        }

        return [
            'total_queries' => count($queries),
            'total_time_ms' => array_sum(array_column($queries, 'time')),
            'queries' => $analysis,
            'slow_queries' => array_filter($analysis, fn($q) => $q['time_ms'] > 100),
        ];
    }

    /**
     * Get OPcache statistics
     */
    private function getOpcacheStats(): array
    {
        $status = opcache_get_status(false);

        if (!$status) {
            return ['enabled' => false];
        }

        return [
            'enabled' => true,
            'memory_usage' => [
                'used' => $this->formatBytes($status['memory_usage']['used_memory']),
                'free' => $this->formatBytes($status['memory_usage']['free_memory']),
                'wasted' => $this->formatBytes($status['memory_usage']['wasted_memory']),
            ],
            'statistics' => [
                'cached_scripts' => $status['opcache_statistics']['num_cached_scripts'],
                'hits' => $status['opcache_statistics']['hits'],
                'misses' => $status['opcache_statistics']['misses'],
                'hit_rate' => round($status['opcache_statistics']['opcache_hit_rate'], 2) . '%',
            ],
        ];
    }

    /**
     * Get performance-related PHP extensions
     */
    private function getPerformanceExtensions(): array
    {
        $extensions = [
            'opcache' => extension_loaded('Zend OPcache'),
            'redis' => extension_loaded('redis'),
            'apcu' => extension_loaded('apcu'),
            'memcached' => extension_loaded('memcached'),
            'curl' => extension_loaded('curl'),
            'gd' => extension_loaded('gd'),
            'imagick' => extension_loaded('imagick'),
            'zip' => extension_loaded('zip'),
        ];

        return $extensions;
    }

    /**
     * Get connection pool status
     */
    private function getConnectionPoolStatus(): array
    {
        try {
            $connections = DB::select("SHOW STATUS LIKE 'Threads_connected'");
            $maxConnections = DB::select("SHOW VARIABLES LIKE 'max_connections'");

            return [
                'current' => (int) ($connections[0]->Value ?? 0),
                'max' => (int) ($maxConnections[0]->Value ?? 151),
            ];
        } catch (\Exception $e) {
            return ['error' => 'Could not fetch connection pool status'];
        }
    }

    /**
     * Get query cache status
     */
    private function getQueryCacheStatus(): array
    {
        try {
            $status = DB::select("SHOW VARIABLES LIKE 'query_cache%'");
            $result = [];
            foreach ($status as $row) {
                $result[$row->Variable_name] = $row->Value;
            }
            return $result;
        } catch (\Exception $e) {
            return ['note' => 'Query cache deprecated in MySQL 8.0+'];
        }
    }

    /**
     * Calculate cache hit rate
     */
    private function calculateHitRate(int $hits, int $misses): string
    {
        $total = $hits + $misses;
        if ($total === 0) {
            return '0%';
        }
        return round(($hits / $total) * 100, 2) . '%';
    }

    /**
     * Calculate memory usage percentage
     */
    private function calculateMemoryUsagePercentage(): string
    {
        $limit = $this->parseMemoryLimit(ini_get('memory_limit'));
        if ($limit <= 0) {
            return 'unlimited';
        }
        $usage = memory_get_usage(true);
        return round(($usage / $limit) * 100, 2) . '%';
    }

    /**
     * Parse memory limit string to bytes
     */
    private function parseMemoryLimit(string $limit): int
    {
        if ($limit === '-1') {
            return -1;
        }

        $unit = strtolower(substr($limit, -1));
        $value = (int) $limit;

        return match($unit) {
            'g' => $value * 1024 * 1024 * 1024,
            'm' => $value * 1024 * 1024,
            'k' => $value * 1024,
            default => $value,
        };
    }

    /**
     * Format bytes to human readable
     */
    private function formatBytes(int $bytes): string
    {
        $units = ['B', 'KB', 'MB', 'GB', 'TB'];
        $i = 0;
        while ($bytes >= 1024 && $i < count($units) - 1) {
            $bytes /= 1024;
            $i++;
        }
        return round($bytes, 2) . ' ' . $units[$i];
    }
}
