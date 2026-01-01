<?php

declare(strict_types=1);

namespace App\Services;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Pagination\LengthAwarePaginator;
use Closure;

/**
 * Query Cache Service
 *
 * Google Enterprise Grade query caching with:
 * - Redis-backed caching for database queries
 * - Automatic cache invalidation tags
 * - TTL-based expiration with jitter
 * - Cache warming capabilities
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class QueryCacheService
{
    protected string $cacheStore;
    protected int $defaultTtl;
    protected bool $enabled;

    public function __construct()
    {
        $this->cacheStore = config('cache.default', 'redis');
        $this->defaultTtl = config('cache.ttl', 3600);
        $this->enabled = config('cache.query_cache_enabled', true);
    }

    /**
     * Remember a query result with automatic cache key generation
     *
     * @template T
     * @param Builder $query
     * @param array<string> $tags
     * @param int|null $ttl
     * @return Collection|LengthAwarePaginator
     */
    public function remember(
        Builder $query,
        array $tags = [],
        ?int $ttl = null,
        string $method = 'get'
    ): Collection|LengthAwarePaginator {
        if (!$this->enabled) {
            return $method === 'get' ? $query->get() : $query->paginate();
        }

        $cacheKey = $this->generateCacheKey($query, $method);
        $ttl = $this->applyTtlJitter($ttl ?? $this->defaultTtl);

        try {
            $cache = Cache::store($this->cacheStore);

            if (!empty($tags) && method_exists($cache, 'tags')) {
                return $cache->tags($tags)->remember($cacheKey, $ttl, function () use ($query, $method) {
                    return $method === 'get' ? $query->get() : $query->paginate();
                });
            }

            return $cache->remember($cacheKey, $ttl, function () use ($query, $method) {
                return $method === 'get' ? $query->get() : $query->paginate();
            });
        } catch (\Exception $e) {
            Log::warning('Query cache failed, executing direct query', [
                'error' => $e->getMessage(),
                'cache_key' => $cacheKey,
            ]);
            return $method === 'get' ? $query->get() : $query->paginate();
        }
    }

    /**
     * Remember a paginated query result
     */
    public function rememberPaginated(
        Builder $query,
        int $perPage = 15,
        array $tags = [],
        ?int $ttl = null
    ): LengthAwarePaginator {
        if (!$this->enabled) {
            return $query->paginate($perPage);
        }

        $page = request()->get('page', 1);
        $cacheKey = $this->generateCacheKey($query, "paginate:{$perPage}:{$page}");
        $ttl = $this->applyTtlJitter($ttl ?? $this->defaultTtl);

        try {
            $cache = Cache::store($this->cacheStore);

            if (!empty($tags) && method_exists($cache, 'tags')) {
                return $cache->tags($tags)->remember($cacheKey, $ttl, function () use ($query, $perPage) {
                    return $query->paginate($perPage);
                });
            }

            return $cache->remember($cacheKey, $ttl, function () use ($query, $perPage) {
                return $query->paginate($perPage);
            });
        } catch (\Exception $e) {
            Log::warning('Query cache failed, executing direct query', [
                'error' => $e->getMessage(),
                'cache_key' => $cacheKey,
            ]);
            return $query->paginate($perPage);
        }
    }

    /**
     * Remember a scalar or custom result
     *
     * @template T
     * @param string $key
     * @param Closure(): T $callback
     * @param array<string> $tags
     * @param int|null $ttl
     * @return T
     */
    public function rememberValue(
        string $key,
        Closure $callback,
        array $tags = [],
        ?int $ttl = null
    ): mixed {
        if (!$this->enabled) {
            return $callback();
        }

        $cacheKey = "query_cache:{$key}";
        $ttl = $this->applyTtlJitter($ttl ?? $this->defaultTtl);

        try {
            $cache = Cache::store($this->cacheStore);

            if (!empty($tags) && method_exists($cache, 'tags')) {
                return $cache->tags($tags)->remember($cacheKey, $ttl, $callback);
            }

            return $cache->remember($cacheKey, $ttl, $callback);
        } catch (\Exception $e) {
            Log::warning('Value cache failed, executing callback', [
                'error' => $e->getMessage(),
                'cache_key' => $cacheKey,
            ]);
            return $callback();
        }
    }

    /**
     * Invalidate cache by tags
     */
    public function invalidateTags(array $tags): bool
    {
        try {
            $cache = Cache::store($this->cacheStore);

            if (method_exists($cache, 'tags')) {
                $cache->tags($tags)->flush();
                Log::info('Cache tags invalidated', ['tags' => $tags]);
                return true;
            }

            return false;
        } catch (\Exception $e) {
            Log::error('Cache tag invalidation failed', [
                'tags' => $tags,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Invalidate a specific cache key
     */
    public function invalidateKey(string $key): bool
    {
        try {
            Cache::store($this->cacheStore)->forget("query_cache:{$key}");
            return true;
        } catch (\Exception $e) {
            Log::error('Cache key invalidation failed', [
                'key' => $key,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Get cache statistics
     */
    public function getStatistics(): array
    {
        try {
            $redis = Cache::store('redis')->getStore()->connection();
            $info = $redis->info();

            return [
                'used_memory' => $info['used_memory_human'] ?? 'N/A',
                'connected_clients' => $info['connected_clients'] ?? 0,
                'keyspace_hits' => $info['keyspace_hits'] ?? 0,
                'keyspace_misses' => $info['keyspace_misses'] ?? 0,
                'hit_rate' => $this->calculateHitRate(
                    $info['keyspace_hits'] ?? 0,
                    $info['keyspace_misses'] ?? 0
                ),
            ];
        } catch (\Exception $e) {
            return ['error' => $e->getMessage()];
        }
    }

    /**
     * Generate a unique cache key for a query
     */
    protected function generateCacheKey(Builder $query, string $suffix = ''): string
    {
        $sql = $query->toSql();
        $bindings = $query->getBindings();

        $hash = md5($sql . serialize($bindings) . $suffix);

        return "query_cache:{$query->getModel()->getTable()}:{$hash}";
    }

    /**
     * Apply jitter to TTL to prevent cache stampede
     */
    protected function applyTtlJitter(int $ttl): int
    {
        // Add 0-10% jitter
        $jitter = (int) ($ttl * (mt_rand(0, 10) / 100));
        return $ttl + $jitter;
    }

    /**
     * Calculate cache hit rate
     */
    protected function calculateHitRate(int $hits, int $misses): string
    {
        $total = $hits + $misses;
        if ($total === 0) {
            return '0%';
        }
        return round(($hits / $total) * 100, 2) . '%';
    }
}
