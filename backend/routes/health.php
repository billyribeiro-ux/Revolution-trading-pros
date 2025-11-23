<?php

use Illuminate\Support\Facades\Route;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Redis;
use Illuminate\Http\Request;

/**
 * Health Check Endpoints - Enterprise Grade
 * Google L7+ Principal Engineer Level
 * 
 * Endpoints:
 * - /health/live - Liveness probe (is app running?)
 * - /health/ready - Readiness probe (can app serve traffic?)
 * - /health/startup - Startup probe (has app finished starting?)
 * - /health - Detailed health check
 */

// ═══════════════════════════════════════════════════════════════════════════
// Liveness Probe - Kubernetes liveness check
// ═══════════════════════════════════════════════════════════════════════════

Route::get('/health/live', function () {
    return response()->json([
        'status' => 'ok',
        'timestamp' => now()->toIso8601String()
    ]);
});

// ═══════════════════════════════════════════════════════════════════════════
// Readiness Probe - Can serve traffic?
// ═══════════════════════════════════════════════════════════════════════════

Route::get('/health/ready', function () {
    $checks = [];
    $healthy = true;

    // Check database connection
    try {
        DB::connection()->getPdo();
        $checks['database'] = [
            'status' => 'healthy',
            'message' => 'Database connection successful'
        ];
    } catch (\Exception $e) {
        $healthy = false;
        $checks['database'] = [
            'status' => 'unhealthy',
            'message' => 'Database connection failed: ' . $e->getMessage()
        ];
    }

    // Check cache
    try {
        Cache::put('health_check', true, 10);
        $value = Cache::get('health_check');
        
        $checks['cache'] = [
            'status' => $value ? 'healthy' : 'unhealthy',
            'message' => $value ? 'Cache working' : 'Cache not working'
        ];
        
        if (!$value) $healthy = false;
    } catch (\Exception $e) {
        $healthy = false;
        $checks['cache'] = [
            'status' => 'unhealthy',
            'message' => 'Cache failed: ' . $e->getMessage()
        ];
    }

    $status = $healthy ? 200 : 503;

    return response()->json([
        'status' => $healthy ? 'ready' : 'not_ready',
        'checks' => $checks,
        'timestamp' => now()->toIso8601String()
    ], $status);
});

// ═══════════════════════════════════════════════════════════════════════════
// Startup Probe - Has app finished starting?
// ═══════════════════════════════════════════════════════════════════════════

Route::get('/health/startup', function () {
    $checks = [];
    $ready = true;

    // Check if migrations are up to date
    try {
        // This is a simple check - in production you might want more sophisticated checks
        DB::table('migrations')->count();
        $checks['migrations'] = [
            'status' => 'ok',
            'message' => 'Migrations table accessible'
        ];
    } catch (\Exception $e) {
        $ready = false;
        $checks['migrations'] = [
            'status' => 'failed',
            'message' => 'Migrations check failed: ' . $e->getMessage()
        ];
    }

    // Check if config is loaded
    $checks['config'] = [
        'status' => config('app.name') ? 'ok' : 'failed',
        'message' => config('app.name') ? 'Config loaded' : 'Config not loaded'
    ];

    if (!config('app.name')) $ready = false;

    $status = $ready ? 200 : 503;

    return response()->json([
        'status' => $ready ? 'started' : 'starting',
        'checks' => $checks,
        'timestamp' => now()->toIso8601String()
    ], $status);
});

// ═══════════════════════════════════════════════════════════════════════════
// Detailed Health Check - Full system status
// ═══════════════════════════════════════════════════════════════════════════

Route::get('/health', function (Request $request) {
    $startTime = microtime(true);
    $checks = [];
    $healthy = true;

    // Database check with query time
    $dbStart = microtime(true);
    try {
        DB::connection()->getPdo();
        $dbTime = (microtime(true) - $dbStart) * 1000;
        
        $checks['database'] = [
            'status' => 'healthy',
            'response_time_ms' => round($dbTime, 2),
            'connection' => DB::connection()->getDatabaseName()
        ];
    } catch (\Exception $e) {
        $healthy = false;
        $checks['database'] = [
            'status' => 'unhealthy',
            'error' => $e->getMessage()
        ];
    }

    // Cache check
    $cacheStart = microtime(true);
    try {
        Cache::put('health_check_detailed', microtime(true), 10);
        $value = Cache::get('health_check_detailed');
        $cacheTime = (microtime(true) - $cacheStart) * 1000;
        
        $checks['cache'] = [
            'status' => 'healthy',
            'response_time_ms' => round($cacheTime, 2),
            'driver' => config('cache.default')
        ];
    } catch (\Exception $e) {
        $healthy = false;
        $checks['cache'] = [
            'status' => 'unhealthy',
            'error' => $e->getMessage()
        ];
    }

    // Redis check (if configured)
    if (config('database.redis.client')) {
        try {
            Redis::ping();
            $checks['redis'] = [
                'status' => 'healthy',
                'message' => 'Redis connection successful'
            ];
        } catch (\Exception $e) {
            // Redis is optional, so don't mark as unhealthy
            $checks['redis'] = [
                'status' => 'unavailable',
                'message' => 'Redis not configured or unavailable'
            ];
        }
    }

    // Queue check
    try {
        $queueConnection = config('queue.default');
        $checks['queue'] = [
            'status' => 'configured',
            'driver' => $queueConnection
        ];
    } catch (\Exception $e) {
        $checks['queue'] = [
            'status' => 'error',
            'error' => $e->getMessage()
        ];
    }

    // Disk space check
    $diskFree = disk_free_space('/');
    $diskTotal = disk_total_space('/');
    $diskUsedPercent = (($diskTotal - $diskFree) / $diskTotal) * 100;
    
    $checks['disk'] = [
        'status' => $diskUsedPercent < 90 ? 'healthy' : 'warning',
        'used_percent' => round($diskUsedPercent, 2),
        'free_gb' => round($diskFree / 1024 / 1024 / 1024, 2),
        'total_gb' => round($diskTotal / 1024 / 1024 / 1024, 2)
    ];

    if ($diskUsedPercent >= 95) {
        $healthy = false;
    }

    // Memory check
    $memoryUsage = memory_get_usage(true);
    $memoryLimit = ini_get('memory_limit');
    $memoryLimitBytes = $memoryLimit === '-1' ? PHP_INT_MAX : $this->convertToBytes($memoryLimit);
    $memoryPercent = ($memoryUsage / $memoryLimitBytes) * 100;

    $checks['memory'] = [
        'status' => $memoryPercent < 90 ? 'healthy' : 'warning',
        'used_mb' => round($memoryUsage / 1024 / 1024, 2),
        'limit' => $memoryLimit,
        'used_percent' => round($memoryPercent, 2)
    ];

    // System info
    $checks['system'] = [
        'php_version' => PHP_VERSION,
        'laravel_version' => app()->version(),
        'environment' => app()->environment(),
        'debug_mode' => config('app.debug'),
        'timezone' => config('app.timezone'),
        'uptime_seconds' => round(microtime(true) - LARAVEL_START, 2)
    ];

    $totalTime = (microtime(true) - $startTime) * 1000;

    $response = [
        'status' => $healthy ? 'healthy' : 'unhealthy',
        'timestamp' => now()->toIso8601String(),
        'response_time_ms' => round($totalTime, 2),
        'checks' => $checks
    ];

    // Add detailed info if requested
    if ($request->query('detailed') === 'true') {
        $response['server'] = [
            'hostname' => gethostname(),
            'server_software' => $_SERVER['SERVER_SOFTWARE'] ?? 'unknown',
            'server_protocol' => $_SERVER['SERVER_PROTOCOL'] ?? 'unknown'
        ];
    }

    $status = $healthy ? 200 : 503;

    return response()->json($response, $status);
});

// ═══════════════════════════════════════════════════════════════════════════
// Metrics Endpoint - Prometheus format
// ═══════════════════════════════════════════════════════════════════════════

Route::get('/metrics', function () {
    $metrics = [];

    // App info
    $metrics[] = '# HELP app_info Application information';
    $metrics[] = '# TYPE app_info gauge';
    $metrics[] = sprintf(
        'app_info{version="%s",environment="%s",php_version="%s"} 1',
        app()->version(),
        app()->environment(),
        PHP_VERSION
    );

    // Memory usage
    $memoryUsage = memory_get_usage(true);
    $metrics[] = '# HELP php_memory_usage_bytes PHP memory usage in bytes';
    $metrics[] = '# TYPE php_memory_usage_bytes gauge';
    $metrics[] = sprintf('php_memory_usage_bytes %d', $memoryUsage);

    // Database connections
    try {
        $dbConnected = DB::connection()->getPdo() ? 1 : 0;
        $metrics[] = '# HELP database_connected Database connection status';
        $metrics[] = '# TYPE database_connected gauge';
        $metrics[] = sprintf('database_connected %d', $dbConnected);
    } catch (\Exception $e) {
        $metrics[] = 'database_connected 0';
    }

    // Cache status
    try {
        Cache::put('metrics_check', true, 10);
        $cacheWorking = Cache::get('metrics_check') ? 1 : 0;
        $metrics[] = '# HELP cache_working Cache system status';
        $metrics[] = '# TYPE cache_working gauge';
        $metrics[] = sprintf('cache_working %d', $cacheWorking);
    } catch (\Exception $e) {
        $metrics[] = 'cache_working 0';
    }

    // Disk usage
    $diskFree = disk_free_space('/');
    $diskTotal = disk_total_space('/');
    $metrics[] = '# HELP disk_free_bytes Free disk space in bytes';
    $metrics[] = '# TYPE disk_free_bytes gauge';
    $metrics[] = sprintf('disk_free_bytes %d', $diskFree);
    $metrics[] = sprintf('disk_total_bytes %d', $diskTotal);

    return response(implode("\n", $metrics) . "\n")
        ->header('Content-Type', 'text/plain; version=0.0.4');
});

// Helper function
if (!function_exists('convertToBytes')) {
    function convertToBytes($from) {
        $number = substr($from, 0, -1);
        switch (strtoupper(substr($from, -1))) {
            case 'K':
                return $number * 1024;
            case 'M':
                return $number * pow(1024, 2);
            case 'G':
                return $number * pow(1024, 3);
            case 'T':
                return $number * pow(1024, 4);
            case 'P':
                return $number * pow(1024, 5);
            default:
                return $from;
        }
    }
}
