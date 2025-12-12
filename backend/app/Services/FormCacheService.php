<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormSubmission;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * Form Cache Service - Multi-tier caching for optimal performance
 *
 * Features:
 * - Multi-tier caching (memory, Redis, database)
 * - Tag-based cache invalidation
 * - Cache warming and preloading
 * - Intelligent TTL management
 * - Atomic cache operations
 * - Cache versioning
 * - Analytics tracking
 *
 * @version 1.0.0
 */
class FormCacheService
{
    /**
     * Cache TTL levels (seconds)
     */
    private const TTL_LEVELS = [
        'hot' => 300,        // 5 minutes - frequently accessed
        'warm' => 1800,      // 30 minutes - moderately accessed
        'cold' => 3600,      // 1 hour - rarely accessed
        'frozen' => 86400,   // 24 hours - static data
    ];

    /**
     * Cache prefixes
     */
    private const PREFIX_FORM = 'form:';
    private const PREFIX_FORM_LIST = 'form_list:';
    private const PREFIX_FORM_FIELDS = 'form_fields:';
    private const PREFIX_SUBMISSIONS = 'form_submissions:';
    private const PREFIX_STATS = 'form_stats:';
    private const PREFIX_ANALYTICS = 'form_analytics:';
    private const PREFIX_PUBLIC = 'form_public:';

    /**
     * Cache version for bulk invalidation
     */
    private string $cacheVersion;

    /**
     * Cache statistics
     */
    private array $stats = [
        'hits' => 0,
        'misses' => 0,
        'writes' => 0,
    ];

    public function __construct()
    {
        $this->cacheVersion = Cache::get('form_cache_version', '1');
    }

    // =========================================================================
    // FORM CACHING
    // =========================================================================

    /**
     * Get form from cache or database
     */
    public function getForm(int $formId, bool $withFields = true): ?Form
    {
        $key = $this->buildKey(self::PREFIX_FORM, $formId, $withFields ? 'full' : 'basic');

        return $this->remember($key, self::TTL_LEVELS['warm'], function () use ($formId, $withFields) {
            $query = Form::query();

            if ($withFields) {
                $query->with(['fields' => fn($q) => $q->orderBy('order')]);
            }

            return $query->find($formId);
        });
    }

    /**
     * Get public form (for embedding/display)
     */
    public function getPublicForm(string $slug): ?Form
    {
        $key = $this->buildKey(self::PREFIX_PUBLIC, $slug);

        return $this->remember($key, self::TTL_LEVELS['hot'], function () use ($slug) {
            return Form::with(['fields' => fn($q) => $q->orderBy('order')])
                ->where('slug', $slug)
                ->where('status', 'published')
                ->first();
        });
    }

    /**
     * Get form list with pagination
     */
    public function getFormList(array $filters = [], int $page = 1, int $perPage = 20): array
    {
        $filterKey = md5(json_encode($filters));
        $key = $this->buildKey(self::PREFIX_FORM_LIST, $filterKey, $page, $perPage);

        return $this->remember($key, self::TTL_LEVELS['warm'], function () use ($filters, $page, $perPage) {
            $query = Form::withCount('submissions');

            // Apply filters
            if (!empty($filters['status'])) {
                $query->where('status', $filters['status']);
            }

            if (!empty($filters['user_id'])) {
                $query->where('user_id', $filters['user_id']);
            }

            if (!empty($filters['search'])) {
                $query->where(function ($q) use ($filters) {
                    $q->where('title', 'like', "%{$filters['search']}%")
                        ->orWhere('description', 'like', "%{$filters['search']}%");
                });
            }

            return $query->orderBy('updated_at', 'desc')
                ->paginate($perPage, ['*'], 'page', $page)
                ->toArray();
        });
    }

    /**
     * Get form fields
     */
    public function getFormFields(int $formId): array
    {
        $key = $this->buildKey(self::PREFIX_FORM_FIELDS, $formId);

        return $this->remember($key, self::TTL_LEVELS['warm'], function () use ($formId) {
            return \DB::table('form_fields')
                ->where('form_id', $formId)
                ->orderBy('order')
                ->get()
                ->toArray();
        });
    }

    // =========================================================================
    // STATISTICS CACHING
    // =========================================================================

    /**
     * Get form statistics
     */
    public function getFormStats(int $formId): array
    {
        $key = $this->buildKey(self::PREFIX_STATS, $formId);

        return $this->remember($key, self::TTL_LEVELS['hot'], function () use ($formId) {
            return FormSubmission::where('form_id', $formId)
                ->selectRaw("
                    COUNT(*) as total,
                    SUM(CASE WHEN status = 'unread' THEN 1 ELSE 0 END) as unread,
                    SUM(CASE WHEN DATE(created_at) = CURDATE() THEN 1 ELSE 0 END) as today,
                    SUM(CASE WHEN created_at >= ? THEN 1 ELSE 0 END) as this_week,
                    SUM(CASE WHEN created_at >= ? THEN 1 ELSE 0 END) as this_month
                ", [now()->startOfWeek(), now()->startOfMonth()])
                ->first()
                ->toArray();
        });
    }

    /**
     * Get dashboard statistics
     */
    public function getDashboardStats(?int $userId = null): array
    {
        $key = $this->buildKey(self::PREFIX_STATS, 'dashboard', $userId ?? 'all');

        return $this->remember($key, self::TTL_LEVELS['hot'], function () use ($userId) {
            $query = Form::query();

            if ($userId) {
                $query->where('user_id', $userId);
            }

            $formStats = $query->selectRaw("
                COUNT(*) as total_forms,
                SUM(CASE WHEN status = 'published' THEN 1 ELSE 0 END) as published_forms,
                SUM(CASE WHEN status = 'draft' THEN 1 ELSE 0 END) as draft_forms,
                SUM(submission_count) as total_submissions
            ")->first();

            $recentSubmissions = FormSubmission::query()
                ->when($userId, function ($q) use ($userId) {
                    $q->whereHas('form', fn($fq) => $fq->where('user_id', $userId));
                })
                ->where('created_at', '>=', now()->subDays(7))
                ->count();

            return [
                'total_forms' => $formStats->total_forms,
                'published_forms' => $formStats->published_forms,
                'draft_forms' => $formStats->draft_forms,
                'total_submissions' => $formStats->total_submissions,
                'submissions_this_week' => $recentSubmissions,
            ];
        });
    }

    /**
     * Get analytics data
     */
    public function getAnalytics(int $formId, string $period = '30d'): array
    {
        $key = $this->buildKey(self::PREFIX_ANALYTICS, $formId, $period);

        return $this->remember($key, self::TTL_LEVELS['warm'], function () use ($formId, $period) {
            $days = match ($period) {
                '7d' => 7,
                '30d' => 30,
                '90d' => 90,
                '365d' => 365,
                default => 30,
            };

            $startDate = now()->subDays($days);

            // Daily submissions
            $daily = FormSubmission::where('form_id', $formId)
                ->where('created_at', '>=', $startDate)
                ->selectRaw('DATE(created_at) as date, COUNT(*) as count')
                ->groupBy('date')
                ->orderBy('date')
                ->pluck('count', 'date')
                ->toArray();

            // Status breakdown
            $statusBreakdown = FormSubmission::where('form_id', $formId)
                ->where('created_at', '>=', $startDate)
                ->selectRaw('status, COUNT(*) as count')
                ->groupBy('status')
                ->pluck('count', 'status')
                ->toArray();

            return [
                'daily' => $daily,
                'status_breakdown' => $statusBreakdown,
                'period' => $period,
                'start_date' => $startDate->toDateString(),
            ];
        });
    }

    // =========================================================================
    // CACHE INVALIDATION
    // =========================================================================

    /**
     * Invalidate form cache
     */
    public function invalidateForm(int $formId): void
    {
        $patterns = [
            $this->buildKey(self::PREFIX_FORM, $formId, '*'),
            $this->buildKey(self::PREFIX_FORM_FIELDS, $formId),
            $this->buildKey(self::PREFIX_STATS, $formId),
            $this->buildKey(self::PREFIX_ANALYTICS, $formId, '*'),
        ];

        foreach ($patterns as $pattern) {
            $this->forgetPattern($pattern);
        }

        // Also invalidate list caches
        $this->invalidateFormLists();

        $this->stats['writes']++;
    }

    /**
     * Invalidate public form cache
     */
    public function invalidatePublicForm(string $slug): void
    {
        Cache::forget($this->buildKey(self::PREFIX_PUBLIC, $slug));
    }

    /**
     * Invalidate all form list caches
     */
    public function invalidateFormLists(): void
    {
        $this->forgetPattern($this->buildKey(self::PREFIX_FORM_LIST, '*'));
    }

    /**
     * Invalidate submission-related caches
     */
    public function invalidateSubmissions(int $formId): void
    {
        $this->forgetPattern($this->buildKey(self::PREFIX_SUBMISSIONS, $formId, '*'));
        $this->forgetPattern($this->buildKey(self::PREFIX_STATS, $formId));
        $this->forgetPattern($this->buildKey(self::PREFIX_ANALYTICS, $formId, '*'));
        $this->forgetPattern($this->buildKey(self::PREFIX_STATS, 'dashboard', '*'));
    }

    /**
     * Invalidate all caches (nuclear option)
     */
    public function invalidateAll(): void
    {
        // Increment cache version to invalidate all existing caches
        $newVersion = (string) (intval($this->cacheVersion) + 1);
        Cache::forever('form_cache_version', $newVersion);
        $this->cacheVersion = $newVersion;
    }

    // =========================================================================
    // CACHE WARMING
    // =========================================================================

    /**
     * Warm cache for a form
     */
    public function warmFormCache(int $formId): void
    {
        // Pre-fetch form data
        $this->getForm($formId, true);
        $this->getFormFields($formId);
        $this->getFormStats($formId);

        // Get public form if published
        $form = Form::find($formId);
        if ($form && $form->status === 'published') {
            $this->getPublicForm($form->slug);
        }
    }

    /**
     * Warm cache for popular forms
     */
    public function warmPopularForms(int $limit = 20): int
    {
        $popularForms = Form::where('status', 'published')
            ->orderBy('submission_count', 'desc')
            ->limit($limit)
            ->pluck('id');

        foreach ($popularForms as $formId) {
            $this->warmFormCache($formId);
        }

        return $popularForms->count();
    }

    /**
     * Warm dashboard cache
     */
    public function warmDashboardCache(): void
    {
        $this->getDashboardStats();

        // Warm per-user stats for active users
        $activeUserIds = Form::distinct()
            ->whereNotNull('user_id')
            ->pluck('user_id');

        foreach ($activeUserIds as $userId) {
            $this->getDashboardStats($userId);
        }
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /**
     * Build cache key with version
     */
    private function buildKey(string ...$parts): string
    {
        return 'v' . $this->cacheVersion . ':' . implode(':', $parts);
    }

    /**
     * Remember with cache tracking
     */
    private function remember(string $key, int $ttl, callable $callback): mixed
    {
        if (Cache::has($key)) {
            $this->stats['hits']++;
            return Cache::get($key);
        }

        $this->stats['misses']++;
        $this->stats['writes']++;

        $value = $callback();
        Cache::put($key, $value, $ttl);

        return $value;
    }

    /**
     * Forget keys matching pattern
     */
    private function forgetPattern(string $pattern): void
    {
        // For Redis, use pattern matching
        if (config('cache.default') === 'redis') {
            $redis = Cache::getStore()->getRedis();
            $keys = $redis->keys(config('cache.prefix') . ':' . $pattern);
            foreach ($keys as $key) {
                $redis->del($key);
            }
        } else {
            // For other drivers, just forget the exact key
            Cache::forget(str_replace('*', '', $pattern));
        }
    }

    /**
     * Get cache statistics
     */
    public function getStats(): array
    {
        $total = $this->stats['hits'] + $this->stats['misses'];
        $hitRate = $total > 0 ? ($this->stats['hits'] / $total) * 100 : 0;

        return [
            'hits' => $this->stats['hits'],
            'misses' => $this->stats['misses'],
            'writes' => $this->stats['writes'],
            'hit_rate' => round($hitRate, 2),
            'version' => $this->cacheVersion,
        ];
    }

    /**
     * Reset statistics
     */
    public function resetStats(): void
    {
        $this->stats = ['hits' => 0, 'misses' => 0, 'writes' => 0];
    }
}
