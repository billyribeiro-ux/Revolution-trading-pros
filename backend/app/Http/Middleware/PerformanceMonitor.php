<?php

declare(strict_types=1);

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Symfony\Component\HttpFoundation\Response;
use App\Services\Performance\DatabaseOptimizer;

/**
 * Performance Monitor Middleware (ICT9+ Enterprise Grade)
 *
 * Monitors and optimizes request performance:
 * - Request timing with Server-Timing headers
 * - Memory usage tracking
 * - Query count and time monitoring
 * - Slow request alerting
 * - Performance metrics collection
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class PerformanceMonitor
{
    /**
     * Slow request threshold in milliseconds
     */
    private const SLOW_REQUEST_THRESHOLD = 500;

    /**
     * High memory threshold in bytes (50MB)
     */
    private const HIGH_MEMORY_THRESHOLD = 52428800;

    /**
     * High query count threshold
     */
    private const HIGH_QUERY_COUNT = 50;

    /**
     * Handle an incoming request
     */
    public function handle(Request $request, Closure $next): Response
    {
        $startTime = microtime(true);
        $startMemory = memory_get_usage(true);

        // Enable query logging for metrics
        DB::enableQueryLog();

        /** @var Response $response */
        $response = $next($request);

        // Calculate metrics
        $duration = (microtime(true) - $startTime) * 1000;
        $memoryUsed = memory_get_usage(true) - $startMemory;
        $peakMemory = memory_get_peak_usage(true);
        $queryLog = DB::getQueryLog();
        $queryCount = count($queryLog);
        $queryTime = array_sum(array_column($queryLog, 'time'));

        // Add Server-Timing headers
        $this->addServerTimingHeaders($response, $duration, $queryTime, $queryCount);

        // Add performance headers
        $response->headers->set('X-Response-Time', round($duration, 2) . 'ms');
        $response->headers->set('X-Query-Count', (string) $queryCount);
        $response->headers->set('X-Memory-Peak', $this->formatBytes($peakMemory));

        // Check for performance issues
        $this->checkPerformanceThresholds(
            $request,
            $duration,
            $memoryUsed,
            $queryCount,
            $queryTime
        );

        // Record metrics for analytics
        $this->recordMetrics($request, $duration, $memoryUsed, $queryCount, $queryTime);

        // Disable query log to free memory
        DB::disableQueryLog();

        return $response;
    }

    /**
     * Add Server-Timing headers for browser DevTools
     */
    private function addServerTimingHeaders(
        Response $response,
        float $totalDuration,
        float $queryTime,
        int $queryCount
    ): void {
        $timings = [
            "total;dur=" . round($totalDuration, 2),
            "db;dur=" . round($queryTime, 2) . ";desc=\"{$queryCount} queries\"",
            "app;dur=" . round($totalDuration - $queryTime, 2),
        ];

        // Add cache timing if available
        $cacheHits = Cache::get('cache:mtc:metrics:l1_hit', 0) +
                     Cache::get('cache:mtc:metrics:l2_hit', 0);
        $cacheMisses = Cache::get('cache:mtc:metrics:miss', 0);

        if ($cacheHits + $cacheMisses > 0) {
            $hitRate = round(($cacheHits / ($cacheHits + $cacheMisses)) * 100);
            $timings[] = "cache;desc=\"{$hitRate}% hit rate\"";
        }

        $response->headers->set('Server-Timing', implode(', ', $timings));
    }

    /**
     * Check performance thresholds and alert if exceeded
     */
    private function checkPerformanceThresholds(
        Request $request,
        float $duration,
        int $memoryUsed,
        int $queryCount,
        float $queryTime
    ): void {
        $alerts = [];

        if ($duration > self::SLOW_REQUEST_THRESHOLD) {
            $alerts['slow_request'] = [
                'duration_ms' => round($duration, 2),
                'threshold_ms' => self::SLOW_REQUEST_THRESHOLD,
            ];
        }

        if ($memoryUsed > self::HIGH_MEMORY_THRESHOLD) {
            $alerts['high_memory'] = [
                'used_bytes' => $memoryUsed,
                'threshold_bytes' => self::HIGH_MEMORY_THRESHOLD,
            ];
        }

        if ($queryCount > self::HIGH_QUERY_COUNT) {
            $alerts['high_query_count'] = [
                'count' => $queryCount,
                'threshold' => self::HIGH_QUERY_COUNT,
                'total_time_ms' => round($queryTime, 2),
            ];
        }

        if (!empty($alerts)) {
            Log::warning('Performance alert', [
                'url' => $request->fullUrl(),
                'method' => $request->method(),
                'alerts' => $alerts,
                'user_id' => $request->user()?->id,
            ]);
        }
    }

    /**
     * Record metrics for analytics
     */
    private function recordMetrics(
        Request $request,
        float $duration,
        int $memoryUsed,
        int $queryCount,
        float $queryTime
    ): void {
        // Store in Redis for aggregation
        $minute = (int) (time() / 60) * 60;
        $route = $request->route()?->getName() ?? $request->path();

        try {
            // Increment request counter
            Cache::increment("perf:requests:{$minute}");

            // Track response times
            Cache::put(
                "perf:timing:{$minute}:" . md5($route),
                [
                    'route' => $route,
                    'duration' => $duration,
                    'queries' => $queryCount,
                    'query_time' => $queryTime,
                    'memory' => $memoryUsed,
                ],
                300 // Keep for 5 minutes
            );

            // Track slow routes
            if ($duration > 100) {
                $slowRoutes = Cache::get("perf:slow_routes:{$minute}", []);
                $slowRoutes[$route] = ($slowRoutes[$route] ?? 0) + 1;
                Cache::put("perf:slow_routes:{$minute}", $slowRoutes, 300);
            }
        } catch (\Throwable $e) {
            // Ignore metric errors
        }
    }

    /**
     * Format bytes to human readable
     */
    private function formatBytes(int $bytes): string
    {
        $units = ['B', 'KB', 'MB', 'GB'];
        $i = 0;
        while ($bytes >= 1024 && $i < count($units) - 1) {
            $bytes /= 1024;
            $i++;
        }
        return round($bytes, 2) . $units[$i];
    }
}
