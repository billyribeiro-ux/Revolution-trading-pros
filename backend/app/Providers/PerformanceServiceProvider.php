<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use Illuminate\Support\Facades\DB;
use Illuminate\Database\Connection;
use App\Services\Performance\DatabaseOptimizer;
use App\Services\Performance\LazyLoadingPrevention;
use App\Services\Performance\AsyncProcessor;
use App\Services\Performance\ConnectionPool;
use App\Services\Performance\ResponseOptimizer;
use App\Services\Performance\ReadReplicaManager;

/**
 * Performance Service Provider (ICT9+ Enterprise Grade)
 *
 * Registers and configures all performance optimization services
 *
 * @version 1.0.0
 */
class PerformanceServiceProvider extends ServiceProvider
{
    /**
     * Register services
     */
    public function register(): void
    {
        // Register as singletons for efficiency
        $this->app->singleton(DatabaseOptimizer::class);
        $this->app->singleton(AsyncProcessor::class);
        $this->app->singleton(ConnectionPool::class);
        $this->app->singleton(ResponseOptimizer::class);

        // Aliases
        $this->app->alias(DatabaseOptimizer::class, 'db.optimizer');
        $this->app->alias(AsyncProcessor::class, 'async');
        $this->app->alias(ConnectionPool::class, 'http.pool');
        $this->app->alias(ResponseOptimizer::class, 'response.optimizer');
    }

    /**
     * Bootstrap services
     */
    public function boot(): void
    {
        // Enable query monitoring in non-production
        if (!$this->app->isProduction()) {
            $this->enableQueryMonitoring();
        }

        // Enable lazy loading prevention
        $this->configureLazyLoading();

        // Optimize database connections
        $this->optimizeDatabaseConnections();

        // Configure model events for performance
        $this->configureModelPerformance();

        // Reset performance state after each request
        $this->app->terminating(function () {
            DatabaseOptimizer::reset();
            ReadReplicaManager::reset();
        });
    }

    /**
     * Enable query monitoring
     */
    private function enableQueryMonitoring(): void
    {
        $optimizer = $this->app->make(DatabaseOptimizer::class);
        $optimizer->enableMonitoring();
    }

    /**
     * Configure lazy loading prevention
     */
    private function configureLazyLoading(): void
    {
        if ($this->app->isProduction()) {
            // Log violations in production
            LazyLoadingPrevention::enableLogging();
        } else {
            // Throw exceptions in development
            LazyLoadingPrevention::enableStrict();
        }
    }

    /**
     * Optimize database connections
     */
    private function optimizeDatabaseConnections(): void
    {
        // Configure connection with optimal settings
        DB::connection()->getSchemaBuilder()->defaultStringLength(191);

        // Enable query log only in debug mode
        if (config('app.debug')) {
            DB::enableQueryLog();
        }
    }

    /**
     * Configure model performance optimizations
     */
    private function configureModelPerformance(): void
    {
        // Disable timestamp updates when not needed
        // Models can override this individually
    }

    /**
     * Get performance diagnostics
     */
    public function getDiagnostics(): array
    {
        $optimizer = $this->app->make(DatabaseOptimizer::class);

        return [
            'query_stats' => $optimizer->getQueryStats(),
            'index_suggestions' => $optimizer->getIndexSuggestions(),
            'connection_pool' => $this->app->make(ConnectionPool::class)->getStats(),
            'queue_health' => $this->app->make(AsyncProcessor::class)->getQueueHealth(),
            'replica_health' => ReadReplicaManager::checkReplicaHealth(),
            'opcache' => $this->getOpcacheStats(),
            'memory' => [
                'current' => memory_get_usage(true),
                'peak' => memory_get_peak_usage(true),
                'limit' => ini_get('memory_limit'),
            ],
        ];
    }

    /**
     * Get OPcache statistics
     */
    private function getOpcacheStats(): array
    {
        if (!function_exists('opcache_get_status')) {
            return ['enabled' => false];
        }

        $status = opcache_get_status(false);

        if (!$status) {
            return ['enabled' => false];
        }

        return [
            'enabled' => $status['opcache_enabled'] ?? false,
            'memory_usage' => [
                'used' => $status['memory_usage']['used_memory'] ?? 0,
                'free' => $status['memory_usage']['free_memory'] ?? 0,
                'wasted' => $status['memory_usage']['wasted_memory'] ?? 0,
                'usage_pct' => round(
                    (($status['memory_usage']['used_memory'] ?? 0) /
                    (($status['memory_usage']['used_memory'] ?? 1) + ($status['memory_usage']['free_memory'] ?? 1))) * 100,
                    2
                ),
            ],
            'statistics' => [
                'hits' => $status['opcache_statistics']['hits'] ?? 0,
                'misses' => $status['opcache_statistics']['misses'] ?? 0,
                'hit_rate' => round($status['opcache_statistics']['opcache_hit_rate'] ?? 0, 2),
                'cached_scripts' => $status['opcache_statistics']['num_cached_scripts'] ?? 0,
            ],
            'jit' => $status['jit'] ?? ['enabled' => false],
        ];
    }
}
