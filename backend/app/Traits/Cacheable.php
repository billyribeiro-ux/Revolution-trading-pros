<?php

declare(strict_types=1);

namespace App\Traits;

use Illuminate\Support\Facades\Cache;
use Illuminate\Database\Eloquent\Collection;

/**
 * Cacheable Trait (ICT9+ Enterprise Grade)
 *
 * Provides automatic caching capabilities to Eloquent models:
 * - Find by ID with caching
 * - Find by field with caching
 * - Collection caching
 * - Automatic cache invalidation on model events
 * - Tag-based cache management (when supported)
 *
 * Usage:
 * ```php
 * class User extends Model
 * {
 *     use Cacheable;
 *
 *     protected static string $cachePrefix = 'user';
 *     protected static string $cacheTag = 'users';
 *     protected static int $cacheTtl = 3600;
 * }
 * ```
 *
 * @version 1.1.0
 * @level ICT9+ Principal Engineer Grade
 */
trait Cacheable
{
    /**
     * Boot the cacheable trait
     */
    public static function bootCacheable(): void
    {
        // Clear cache on model events
        static::saved(fn ($model) => $model->clearModelCache());
        static::deleted(fn ($model) => $model->clearModelCache());
    }

    /**
     * Get cache prefix for this model
     */
    protected static function getCachePrefix(): string
    {
        return static::$cachePrefix ?? strtolower(class_basename(static::class));
    }

    /**
     * Get cache tag for this model
     */
    protected static function getCacheTag(): string
    {
        return static::$cacheTag ?? strtolower(class_basename(static::class)) . 's';
    }

    /**
     * Get cache TTL for this model
     */
    protected static function getCacheTtl(): int
    {
        return static::$cacheTtl ?? 3600;
    }

    /**
     * Get cache TTL for statistics (shorter)
     */
    protected static function getStatsCacheTtl(): int
    {
        return static::$statsCacheTtl ?? 300;
    }

    /**
     * Check if cache driver supports tagging
     */
    protected static function cacheSupportsTagging(): bool
    {
        try {
            $store = Cache::getStore();
            return method_exists($store, 'tags');
        } catch (\Throwable) {
            return false;
        }
    }

    /**
     * Get cache instance (with or without tags based on driver support)
     */
    protected static function getCacheInstance(): mixed
    {
        if (static::cacheSupportsTagging()) {
            return Cache::tags([static::getCacheTag()]);
        }
        return Cache::store();
    }

    /**
     * Get cache key with tag prefix for non-tagging drivers
     */
    protected static function getCacheKey(string $key): string
    {
        if (static::cacheSupportsTagging()) {
            return $key;
        }
        // For non-tagging drivers, include tag in key for pseudo-namespacing
        return static::getCacheTag() . ':' . $key;
    }

    /**
     * Find by ID with caching
     *
     * @param string|int $id
     * @param array $relations Relations to eager load
     * @return static|null
     */
    public static function findCached(string|int $id, array $relations = []): ?static
    {
        $key = static::getCacheKey(static::getCachePrefix() . ':' . $id);

        return static::getCacheInstance()->remember(
            $key,
            static::getCacheTtl(),
            function () use ($id, $relations) {
                $query = static::query();
                if (!empty($relations)) {
                    $query->with($relations);
                }
                return $query->find($id);
            }
        );
    }

    /**
     * Find by field with caching
     *
     * @param string $field
     * @param mixed $value
     * @param array $relations Relations to eager load
     * @return static|null
     */
    public static function findByCached(string $field, mixed $value, array $relations = []): ?static
    {
        $key = static::getCacheKey(static::getCachePrefix() . ":{$field}:" . md5((string) $value));

        return static::getCacheInstance()->remember(
            $key,
            static::getCacheTtl(),
            function () use ($field, $value, $relations) {
                $query = static::query()->where($field, $value);
                if (!empty($relations)) {
                    $query->with($relations);
                }
                return $query->first();
            }
        );
    }

    /**
     * Find by slug with caching (convenience method)
     */
    public static function findBySlugCached(string $slug, array $relations = []): ?static
    {
        return static::findByCached('slug', $slug, $relations);
    }

    /**
     * Get all with caching
     */
    public static function allCached(array $relations = []): Collection
    {
        $key = static::getCacheKey(static::getCachePrefix() . ':all');

        return static::getCacheInstance()->remember(
            $key,
            static::getCacheTtl(),
            function () use ($relations) {
                $query = static::query();
                if (!empty($relations)) {
                    $query->with($relations);
                }
                return $query->get();
            }
        );
    }

    /**
     * Get with scope and caching
     *
     * @param string $scope Scope name
     * @param array $scopeArgs Scope arguments
     * @param array $relations Relations to eager load
     */
    public static function scopeCached(string $scope, array $scopeArgs = [], array $relations = []): Collection
    {
        $key = static::getCacheKey(static::getCachePrefix() . ':scope:' . $scope . ':' . md5(serialize($scopeArgs)));

        return static::getCacheInstance()->remember(
            $key,
            static::getCacheTtl(),
            function () use ($scope, $scopeArgs, $relations) {
                $query = static::query()->{$scope}(...$scopeArgs);
                if (!empty($relations)) {
                    $query->with($relations);
                }
                return $query->get();
            }
        );
    }

    /**
     * Get count with caching
     */
    public static function countCached(?string $scope = null, array $scopeArgs = []): int
    {
        $key = static::getCacheKey(static::getCachePrefix() . ':count' . ($scope ? ":{$scope}" : ''));

        return static::getCacheInstance()->remember(
            $key,
            static::getStatsCacheTtl(),
            function () use ($scope, $scopeArgs) {
                $query = static::query();
                if ($scope) {
                    $query->{$scope}(...$scopeArgs);
                }
                return $query->count();
            }
        );
    }

    /**
     * Get aggregated stats with caching
     */
    public static function getAggregatedStatsCached(string $column, string $function = 'sum'): mixed
    {
        $key = static::getCacheKey(static::getCachePrefix() . ":stats:{$function}:{$column}");

        return static::getCacheInstance()->remember(
            $key,
            static::getStatsCacheTtl(),
            fn () => static::query()->{$function}($column)
        );
    }

    /**
     * Clear cache for this model instance
     */
    public function clearModelCache(): void
    {
        $prefix = static::getCachePrefix();

        if (static::cacheSupportsTagging()) {
            $tag = static::getCacheTag();
            // Clear by ID
            Cache::tags([$tag])->forget($prefix . ':' . $this->getKey());

            // Clear by slug if exists
            if (isset($this->slug)) {
                Cache::tags([$tag])->forget($prefix . ':slug:' . md5($this->slug));
            }

            // Clear all cached
            Cache::tags([$tag])->forget($prefix . ':all');

            // Clear counts
            Cache::tags([$tag])->forget($prefix . ':count');
        } else {
            // For non-tagging drivers, forget individual keys
            $tag = static::getCacheTag();
            Cache::forget($tag . ':' . $prefix . ':' . $this->getKey());

            if (isset($this->slug)) {
                Cache::forget($tag . ':' . $prefix . ':slug:' . md5($this->slug));
            }

            Cache::forget($tag . ':' . $prefix . ':all');
            Cache::forget($tag . ':' . $prefix . ':count');
        }
    }

    /**
     * Clear all cache for this model type
     */
    public static function clearAllModelCache(): void
    {
        if (static::cacheSupportsTagging()) {
            Cache::tags([static::getCacheTag()])->flush();
        } else {
            // For non-tagging drivers, we can't easily flush all keys
            // This is a limitation of file/database cache drivers
            // Best effort: clear common keys
            $prefix = static::getCachePrefix();
            $tag = static::getCacheTag();
            Cache::forget($tag . ':' . $prefix . ':all');
            Cache::forget($tag . ':' . $prefix . ':count');
        }
    }

    /**
     * Refresh cache for this model instance
     */
    public function refreshCache(): static
    {
        $this->clearModelCache();
        return static::findCached($this->getKey());
    }
}
