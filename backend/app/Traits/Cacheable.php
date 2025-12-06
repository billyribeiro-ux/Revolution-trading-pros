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
 * - Tag-based cache management
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
 * @version 1.0.0
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
     * Find by ID with caching
     *
     * @param string|int $id
     * @param array $relations Relations to eager load
     * @return static|null
     */
    public static function findCached(string|int $id, array $relations = []): ?static
    {
        $key = static::getCachePrefix() . ':' . $id;

        return Cache::tags([static::getCacheTag()])->remember(
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
        $key = static::getCachePrefix() . ":{$field}:" . md5((string) $value);

        return Cache::tags([static::getCacheTag()])->remember(
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
        $key = static::getCachePrefix() . ':all';

        return Cache::tags([static::getCacheTag()])->remember(
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
        $key = static::getCachePrefix() . ':scope:' . $scope . ':' . md5(serialize($scopeArgs));

        return Cache::tags([static::getCacheTag()])->remember(
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
        $key = static::getCachePrefix() . ':count' . ($scope ? ":{$scope}" : '');

        return Cache::tags([static::getCacheTag()])->remember(
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
        $key = static::getCachePrefix() . ":stats:{$function}:{$column}";

        return Cache::tags([static::getCacheTag()])->remember(
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
    }

    /**
     * Clear all cache for this model type
     */
    public static function clearAllModelCache(): void
    {
        Cache::tags([static::getCacheTag()])->flush();
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
