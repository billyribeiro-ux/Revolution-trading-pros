<?php

declare(strict_types=1);

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Facades\Cache;
use App\Services\Cache\MultiTierCacheManager;
use App\Services\Cache\CacheWarmer;
use App\Services\Cache\CacheMetrics;

/**
 * Cache Service Provider (ICT9+ Enterprise Grade)
 *
 * Registers cache services and adds query builder macros
 *
 * @version 1.0.0
 */
class CacheServiceProvider extends ServiceProvider
{
    /**
     * Register services
     */
    public function register(): void
    {
        // Register MultiTierCacheManager as singleton
        $this->app->singleton(MultiTierCacheManager::class, function ($app) {
            return new MultiTierCacheManager();
        });

        // Register CacheWarmer
        $this->app->singleton(CacheWarmer::class, function ($app) {
            return new CacheWarmer($app->make(MultiTierCacheManager::class));
        });

        // Register CacheMetrics
        $this->app->singleton(CacheMetrics::class, function ($app) {
            return new CacheMetrics();
        });

        // Alias for convenience
        $this->app->alias(MultiTierCacheManager::class, 'cache.multitier');
    }

    /**
     * Bootstrap services
     */
    public function boot(): void
    {
        // Add cached() macro to query builder
        $this->registerQueryBuilderMacros();

        // Clear L1 cache after request (for long-running processes)
        $this->app->terminating(function () {
            MultiTierCacheManager::clearL1();
        });
    }

    /**
     * Register query builder macros for cached queries
     */
    private function registerQueryBuilderMacros(): void
    {
        /**
         * Cache the query results
         *
         * @param int $ttl TTL in seconds
         * @param array $tags Cache tags
         * @param string|null $key Custom cache key
         */
        Builder::macro('cached', function (int $ttl = 3600, array $tags = [], ?string $key = null) {
            /** @var Builder $this */
            $cacheKey = $key ?? $this->generateCacheKey();

            if (!empty($tags)) {
                return Cache::tags($tags)->remember($cacheKey, $ttl, fn () => $this->get());
            }

            return Cache::remember($cacheKey, $ttl, fn () => $this->get());
        });

        /**
         * Cache the first result
         */
        Builder::macro('firstCached', function (int $ttl = 3600, array $tags = [], ?string $key = null) {
            /** @var Builder $this */
            $cacheKey = $key ?? $this->generateCacheKey() . ':first';

            if (!empty($tags)) {
                return Cache::tags($tags)->remember($cacheKey, $ttl, fn () => $this->first());
            }

            return Cache::remember($cacheKey, $ttl, fn () => $this->first());
        });

        /**
         * Cache the count
         */
        Builder::macro('countCached', function (int $ttl = 300, array $tags = [], ?string $key = null) {
            /** @var Builder $this */
            $cacheKey = $key ?? $this->generateCacheKey() . ':count';

            if (!empty($tags)) {
                return Cache::tags($tags)->remember($cacheKey, $ttl, fn () => $this->count());
            }

            return Cache::remember($cacheKey, $ttl, fn () => $this->count());
        });

        /**
         * Cache paginated results
         */
        Builder::macro('paginateCached', function (int $perPage = 15, int $ttl = 300, array $tags = []) {
            /** @var Builder $this */
            $page = request()->get('page', 1);
            $cacheKey = $this->generateCacheKey() . ":page:{$page}:per:{$perPage}";

            if (!empty($tags)) {
                return Cache::tags($tags)->remember($cacheKey, $ttl, fn () => $this->paginate($perPage));
            }

            return Cache::remember($cacheKey, $ttl, fn () => $this->paginate($perPage));
        });

        /**
         * Generate cache key from query
         */
        Builder::macro('generateCacheKey', function () {
            /** @var Builder $this */
            $sql = $this->toSql();
            $bindings = $this->getBindings();
            $table = $this->getModel()->getTable();

            return "query:{$table}:" . md5($sql . serialize($bindings));
        });

        /**
         * Flush cache for this model's table
         */
        Builder::macro('flushCache', function (array $tags = []) {
            /** @var Builder $this */
            $table = $this->getModel()->getTable();

            if (!empty($tags)) {
                Cache::tags($tags)->flush();
            } else {
                // Flush by prefix pattern (requires Redis)
                try {
                    $prefix = config('cache.prefix', 'laravel') . ':query:' . $table . ':';
                    $keys = Cache::getRedis()->keys($prefix . '*');
                    if (!empty($keys)) {
                        Cache::getRedis()->del($keys);
                    }
                } catch (\Throwable $e) {
                    // Fallback: just log warning
                    \Log::warning('Could not flush query cache', ['table' => $table, 'error' => $e->getMessage()]);
                }
            }

            return $this;
        });
    }
}
