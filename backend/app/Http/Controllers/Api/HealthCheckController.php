<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\CircuitBreaker;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Queue;
use Illuminate\Support\Facades\Storage;
use Throwable;

/**
 * Health Check Controller
 *
 * Google Enterprise Grade health monitoring with:
 * - Kubernetes readiness/liveness probes
 * - Dependency health verification
 * - Circuit breaker status reporting
 * - Performance metrics
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class HealthCheckController extends Controller
{
    /**
     * Basic health check (liveness probe)
     * Returns 200 if application is running
     */
    public function liveness(): JsonResponse
    {
        return response()->json([
            'status' => 'healthy',
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Readiness probe
     * Checks all critical dependencies
     */
    public function readiness(): JsonResponse
    {
        $checks = [
            'database' => $this->checkDatabase(),
            'cache' => $this->checkCache(),
            'storage' => $this->checkStorage(),
            'queue' => $this->checkQueue(),
        ];

        $allHealthy = collect($checks)->every(fn ($check) => $check['healthy']);
        $status = $allHealthy ? 'ready' : 'not_ready';
        $httpCode = $allHealthy ? 200 : 503;

        return response()->json([
            'status' => $status,
            'timestamp' => now()->toIso8601String(),
            'checks' => $checks,
        ], $httpCode);
    }

    /**
     * Detailed health check for image optimization engine
     */
    public function optimization(): JsonResponse
    {
        $checks = [
            'database' => $this->checkDatabase(),
            'cache' => $this->checkCache(),
            'storage' => $this->checkStorage(),
            'queue' => $this->checkQueue(),
            'image_library' => $this->checkImageLibrary(),
            'circuit_breakers' => $this->checkCircuitBreakers(),
            'queue_metrics' => $this->getQueueMetrics(),
            'storage_metrics' => $this->getStorageMetrics(),
        ];

        $criticalHealthy = collect(['database', 'cache', 'storage'])
            ->every(fn ($key) => $checks[$key]['healthy'] ?? false);

        $status = $criticalHealthy ? 'healthy' : 'degraded';
        $httpCode = $criticalHealthy ? 200 : 503;

        return response()->json([
            'status' => $status,
            'service' => 'image-optimization-engine',
            'version' => config('app.version', '1.0.0'),
            'timestamp' => now()->toIso8601String(),
            'uptime_seconds' => $this->getUptime(),
            'checks' => $checks,
        ], $httpCode);
    }

    /**
     * Check database connectivity
     */
    protected function checkDatabase(): array
    {
        $start = microtime(true);
        try {
            DB::select('SELECT 1');
            $latency = round((microtime(true) - $start) * 1000, 2);

            return [
                'healthy' => true,
                'latency_ms' => $latency,
                'connection' => config('database.default'),
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check cache connectivity
     */
    protected function checkCache(): array
    {
        $start = microtime(true);
        try {
            $testKey = 'health_check_' . uniqid();
            Cache::put($testKey, true, 10);
            $result = Cache::get($testKey);
            Cache::forget($testKey);

            $latency = round((microtime(true) - $start) * 1000, 2);

            return [
                'healthy' => $result === true,
                'latency_ms' => $latency,
                'driver' => config('cache.default'),
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check storage accessibility
     */
    protected function checkStorage(): array
    {
        try {
            $disk = config('image-optimization.storage.disk', 'public');
            $testFile = '.health_check_' . uniqid();

            Storage::disk($disk)->put($testFile, 'test');
            $exists = Storage::disk($disk)->exists($testFile);
            Storage::disk($disk)->delete($testFile);

            return [
                'healthy' => $exists,
                'disk' => $disk,
                'writable' => $exists,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check queue connectivity
     */
    protected function checkQueue(): array
    {
        try {
            $connection = config('image-optimization.queue.connection', 'database');
            $queueName = config('image-optimization.queue.queue_name', 'image-optimization');

            // Check if we can access the queue
            $size = Queue::connection($connection)->size($queueName);

            return [
                'healthy' => true,
                'connection' => $connection,
                'queue' => $queueName,
                'pending_jobs' => $size,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check image processing library
     */
    protected function checkImageLibrary(): array
    {
        try {
            $driver = config('image-optimization.processing.driver', 'gd');

            if ($driver === 'gd') {
                $healthy = extension_loaded('gd') && function_exists('imagecreate');
                $info = $healthy ? gd_info() : null;

                return [
                    'healthy' => $healthy,
                    'driver' => 'gd',
                    'webp_support' => $info['WebP Support'] ?? false,
                    'avif_support' => $info['AVIF Support'] ?? false,
                ];
            }

            if ($driver === 'imagick') {
                $healthy = extension_loaded('imagick');

                return [
                    'healthy' => $healthy,
                    'driver' => 'imagick',
                    'webp_support' => $healthy && in_array('WEBP', \Imagick::queryFormats()),
                    'avif_support' => $healthy && in_array('AVIF', \Imagick::queryFormats()),
                ];
            }

            return [
                'healthy' => false,
                'error' => "Unknown driver: {$driver}",
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check circuit breaker states
     */
    protected function checkCircuitBreakers(): array
    {
        $breakers = [
            'cloudflare' => new CircuitBreaker('cloudflare_cdn'),
            'bunny' => new CircuitBreaker('bunny_cdn'),
        ];

        $statuses = [];
        foreach ($breakers as $name => $breaker) {
            $statuses[$name] = $breaker->getMetrics();
        }

        return $statuses;
    }

    /**
     * Get queue metrics
     */
    protected function getQueueMetrics(): array
    {
        try {
            $pending = DB::table('image_optimization_jobs')
                ->where('status', 'pending')
                ->count();

            $processing = DB::table('image_optimization_jobs')
                ->where('status', 'processing')
                ->count();

            $completed24h = DB::table('image_optimization_jobs')
                ->where('status', 'completed')
                ->where('updated_at', '>=', now()->subDay())
                ->count();

            $failed24h = DB::table('image_optimization_jobs')
                ->where('status', 'failed')
                ->where('updated_at', '>=', now()->subDay())
                ->count();

            $avgProcessingTime = DB::table('image_optimization_jobs')
                ->where('status', 'completed')
                ->where('updated_at', '>=', now()->subHour())
                ->avg(DB::raw('TIMESTAMPDIFF(SECOND, started_at, completed_at)'));

            return [
                'pending' => $pending,
                'processing' => $processing,
                'completed_24h' => $completed24h,
                'failed_24h' => $failed24h,
                'avg_processing_time_seconds' => round($avgProcessingTime ?? 0, 2),
                'failure_rate_24h' => $completed24h > 0
                    ? round(($failed24h / ($completed24h + $failed24h)) * 100, 2)
                    : 0,
            ];
        } catch (Throwable $e) {
            return ['error' => $e->getMessage()];
        }
    }

    /**
     * Get storage metrics
     */
    protected function getStorageMetrics(): array
    {
        try {
            $totalSize = DB::table('media')->sum('size');
            $optimizedSize = DB::table('media')
                ->where('is_optimized', true)
                ->sum('size');
            $variantSize = DB::table('media_variants')->sum('file_size');

            return [
                'total_original_bytes' => (int) $totalSize,
                'total_optimized_bytes' => (int) $optimizedSize,
                'total_variant_bytes' => (int) $variantSize,
                'total_files' => DB::table('media')->count(),
                'optimized_files' => DB::table('media')->where('is_optimized', true)->count(),
                'total_variants' => DB::table('media_variants')->count(),
            ];
        } catch (Throwable $e) {
            return ['error' => $e->getMessage()];
        }
    }

    /**
     * Get application uptime
     */
    protected function getUptime(): int
    {
        $startTime = Cache::get('app_start_time');
        if (!$startTime) {
            Cache::forever('app_start_time', now()->timestamp);
            return 0;
        }

        return now()->timestamp - $startTime;
    }
}
