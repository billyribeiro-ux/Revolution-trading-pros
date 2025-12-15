<?php

declare(strict_types=1);

namespace App\Http\Controllers;

use App\Support\CircuitBreaker\CircuitBreakerFactory;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Queue;
use Illuminate\Support\Facades\Redis;
use Throwable;

/**
 * Health Check Controller
 *
 * Provides liveness and readiness probes for Kubernetes/orchestration.
 * Also exposes detailed health status for monitoring.
 *
 * @version 1.0.0
 */
class HealthController extends Controller
{
    public function __construct(
        private readonly CircuitBreakerFactory $circuitBreaker,
    ) {}

    /**
     * Liveness probe - is the application running?
     *
     * Returns 200 if the application is alive.
     * Used by Kubernetes to determine if container should be restarted.
     *
     * @route GET /health/live
     */
    public function liveness(): JsonResponse
    {
        return response()->json([
            'status' => 'alive',
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Readiness probe - is the application ready to receive traffic?
     *
     * Checks critical dependencies (database, cache).
     * Used by Kubernetes to determine if traffic should be routed.
     *
     * @route GET /health/ready
     */
    public function readiness(): JsonResponse
    {
        $checks = [
            'database' => $this->checkDatabase(),
            'cache' => $this->checkCache(),
        ];

        $isReady = !in_array(false, array_column($checks, 'healthy'), true);

        return response()->json([
            'status' => $isReady ? 'ready' : 'not_ready',
            'checks' => $checks,
            'timestamp' => now()->toIso8601String(),
        ], $isReady ? 200 : 503);
    }

    /**
     * Detailed health check - full system status.
     *
     * Provides comprehensive health information for monitoring dashboards.
     *
     * @route GET /health
     */
    public function detailed(): JsonResponse
    {
        $checks = [
            'database' => $this->checkDatabase(),
            'cache' => $this->checkCache(),
            'redis' => $this->checkRedis(),
            'queue' => $this->checkQueue(),
            'storage' => $this->checkStorage(),
            'circuits' => $this->checkCircuitBreakers(),
        ];

        $healthyCount = count(array_filter($checks, fn ($c) => $c['healthy'] ?? false));
        $totalCount = count($checks);
        $allHealthy = $healthyCount === $totalCount;

        return response()->json([
            'status' => $allHealthy ? 'healthy' : 'degraded',
            'version' => config('app.version', '1.0.0'),
            'environment' => config('app.env'),
            'checks' => $checks,
            'summary' => [
                'healthy' => $healthyCount,
                'total' => $totalCount,
            ],
            'timestamp' => now()->toIso8601String(),
        ], $allHealthy ? 200 : 503);
    }

    /**
     * Check database connectivity.
     *
     * @return array{healthy: bool, latency_ms: float|null, error: string|null}
     */
    private function checkDatabase(): array
    {
        try {
            $start = microtime(true);
            DB::select('SELECT 1');
            $latency = (microtime(true) - $start) * 1000;

            return [
                'healthy' => true,
                'latency_ms' => round($latency, 2),
                'error' => null,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'latency_ms' => null,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check cache connectivity.
     *
     * @return array{healthy: bool, latency_ms: float|null, error: string|null}
     */
    private function checkCache(): array
    {
        try {
            $start = microtime(true);
            $key = 'health_check_' . uniqid();
            Cache::put($key, 'test', 10);
            $value = Cache::get($key);
            Cache::forget($key);
            $latency = (microtime(true) - $start) * 1000;

            return [
                'healthy' => $value === 'test',
                'latency_ms' => round($latency, 2),
                'error' => null,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'latency_ms' => null,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check Redis connectivity.
     *
     * @return array{healthy: bool, latency_ms: float|null, error: string|null}
     */
    private function checkRedis(): array
    {
        try {
            $start = microtime(true);
            Redis::ping();
            $latency = (microtime(true) - $start) * 1000;

            return [
                'healthy' => true,
                'latency_ms' => round($latency, 2),
                'error' => null,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'latency_ms' => null,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check queue connectivity.
     *
     * @return array{healthy: bool, queue_size: int|null, error: string|null}
     */
    private function checkQueue(): array
    {
        try {
            $size = Queue::size('emails');

            return [
                'healthy' => true,
                'queue_size' => $size,
                'error' => null,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'queue_size' => null,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check storage connectivity.
     *
     * @return array{healthy: bool, error: string|null}
     */
    private function checkStorage(): array
    {
        try {
            $disk = config('inbound-email.attachments.disk', 'local');
            $exists = \Storage::disk($disk)->exists('.health_check_marker');

            // Try to write if marker doesn't exist
            if (!$exists) {
                \Storage::disk($disk)->put('.health_check_marker', now()->toIso8601String());
            }

            return [
                'healthy' => true,
                'disk' => $disk,
                'error' => null,
            ];
        } catch (Throwable $e) {
            return [
                'healthy' => false,
                'disk' => null,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Check circuit breaker states.
     *
     * @return array{healthy: bool, circuits: array<string, array<string, mixed>>}
     */
    private function checkCircuitBreakers(): array
    {
        $circuits = [
            'postmark' => $this->circuitBreaker->postmark()->getStats(),
            'ses' => $this->circuitBreaker->ses()->getStats(),
            'storage' => $this->circuitBreaker->storage()->getStats(),
        ];

        $allClosed = true;
        foreach ($circuits as $circuit) {
            if ($circuit['state'] === 'open') {
                $allClosed = false;
                break;
            }
        }

        return [
            'healthy' => $allClosed,
            'circuits' => $circuits,
        ];
    }
}
