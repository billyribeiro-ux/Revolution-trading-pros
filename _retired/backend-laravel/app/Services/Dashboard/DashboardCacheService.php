<?php

namespace App\Services\Dashboard;

use Illuminate\Support\Facades\Cache;

class DashboardCacheService
{
    private const CACHE_PREFIX = 'dashboard:';
    private const DEFAULT_TTL = 300;

    public function remember(string $key, int $ttl, callable $callback): mixed
    {
        return Cache::remember(self::CACHE_PREFIX . $key, $ttl, $callback);
    }

    public function get(string $key, mixed $default = null): mixed
    {
        return Cache::get(self::CACHE_PREFIX . $key, $default);
    }

    public function set(string $key, mixed $value, ?int $ttl = null): bool
    {
        return Cache::put(self::CACHE_PREFIX . $key, $value, $ttl ?? self::DEFAULT_TTL);
    }

    public function invalidateWidgetCache(string $widgetId): bool
    {
        return Cache::forget(self::CACHE_PREFIX . "widget_data_{$widgetId}");
    }

    public function invalidateDashboardCache(string $dashboardId): bool
    {
        $keys = Cache::get(self::CACHE_PREFIX . "dashboard_{$dashboardId}_keys", []);
        foreach ($keys as $key) {
            Cache::forget($key);
        }
        return true;
    }
}
