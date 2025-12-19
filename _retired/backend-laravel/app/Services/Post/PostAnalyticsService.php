<?php

declare(strict_types=1);

namespace App\Services\Post;

use App\Models\Post;
use App\Models\PostAnalytic;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Carbon;

/**
 * PostAnalyticsService - Enterprise-grade post analytics and tracking
 * 
 * Comprehensive analytics service providing:
 * - Real-time view tracking with deduplication
 * - Engagement metrics calculation
 * - Performance analytics and insights
 * - Trending content detection
 * - SEO performance tracking
 * - Audience behavior analysis
 * - Revenue attribution
 * - Comparative analytics
 */
class PostAnalyticsService
{
    /**
     * Cache TTL in seconds.
     */
    private const CACHE_TTL = 300; // 5 minutes

    /**
     * View tracking session duration (seconds).
     */
    private const VIEW_SESSION_DURATION = 1800; // 30 minutes

    /**
     * Engagement rate calculation weights.
     */
    private const ENGAGEMENT_WEIGHTS = [
        'view' => 1,
        'like' => 5,
        'comment' => 10,
        'share' => 15,
    ];

    /**
     * Popular post thresholds.
     */
    private const POPULAR_MIN_VIEWS = 1000;
    private const TRENDING_MIN_VIEWS = 500;
    private const VIRAL_MIN_VIEWS = 10000;

    // =========================================================================
    // VIEW TRACKING METHODS
    // =========================================================================

    /**
     * Track post view with session deduplication.
     */
    public function trackView(Post $post, ?int $userId = null, ?string $sessionId = null): bool
    {
        try {
            $sessionId = $sessionId ?? session()->getId();
            $userId = $userId ?? auth()->id();
            $cacheKey = "post_view_{$post->id}_{$sessionId}";

            // Check if this session already viewed this post recently
            if (Cache::has($cacheKey)) {
                return false;
            }

            DB::transaction(function () use ($post, $userId, $sessionId) {
                // Increment post view count
                $post->increment('view_count');

                // Record detailed analytics
                PostAnalytic::create([
                    'post_id' => $post->id,
                    'user_id' => $userId,
                    'session_id' => $sessionId,
                    'event_type' => 'view',
                    'ip_address' => request()->ip(),
                    'user_agent' => request()->userAgent(),
                    'referrer' => request()->header('referer'),
                    'timestamp' => now(),
                ]);
            });

            // Cache the view for session duration
            Cache::put($cacheKey, true, self::VIEW_SESSION_DURATION);

            // Clear analytics cache
            $this->clearAnalyticsCache($post->id);

            Log::info('Post view tracked', [
                'post_id' => $post->id,
                'user_id' => $userId,
                'session_id' => $sessionId,
            ]);

            return true;
        } catch (\Exception $e) {
            Log::error('Failed to track post view', [
                'post_id' => $post->id,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Track post engagement (like, comment, share).
     */
    public function trackEngagement(
        Post $post,
        string $eventType,
        ?int $userId = null,
        array $metadata = []
    ): bool {
        try {
            $userId = $userId ?? auth()->id();

            DB::transaction(function () use ($post, $eventType, $userId, $metadata) {
                // Update post counters
                $this->updateEngagementCounters($post, $eventType);

                // Record detailed analytics
                PostAnalytic::create([
                    'post_id' => $post->id,
                    'user_id' => $userId,
                    'session_id' => session()->getId(),
                    'event_type' => $eventType,
                    'event_data' => $metadata,
                    'ip_address' => request()->ip(),
                    'user_agent' => request()->userAgent(),
                    'timestamp' => now(),
                ]);

                // Recalculate engagement rate
                $this->updateEngagementRate($post);
            });

            $this->clearAnalyticsCache($post->id);

            return true;
        } catch (\Exception $e) {
            Log::error('Failed to track post engagement', [
                'post_id' => $post->id,
                'event_type' => $eventType,
                'error' => $e->getMessage(),
            ]);

            return false;
        }
    }

    /**
     * Track time spent on post.
     */
    public function trackTimeSpent(Post $post, int $seconds, ?string $sessionId = null): void
    {
        try {
            $sessionId = $sessionId ?? session()->getId();

            PostAnalytic::where('post_id', $post->id)
                ->where('session_id', $sessionId)
                ->where('event_type', 'view')
                ->latest()
                ->first()
                ?->update(['time_spent' => $seconds]);

            // Update average time on page
            $avgTime = PostAnalytic::where('post_id', $post->id)
                ->where('event_type', 'view')
                ->whereNotNull('time_spent')
                ->avg('time_spent');

            $post->update(['avg_time_on_page' => round($avgTime ?? 0)]);

            $this->clearAnalyticsCache($post->id);
        } catch (\Exception $e) {
            Log::error('Failed to track time spent', [
                'post_id' => $post->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    // =========================================================================
    // ANALYTICS RETRIEVAL METHODS
    // =========================================================================

    /**
     * Get comprehensive post analytics.
     */
    public function getAnalytics(Post $post, ?int $days = null): array
    {
        $cacheKey = "post_analytics_{$post->id}" . ($days ? "_{$days}d" : '');

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($post, $days) {
            $query = PostAnalytic::where('post_id', $post->id);

            if ($days) {
                $query->where('timestamp', '>=', now()->subDays($days));
            }

            $analytics = $query->get();
            $uniqueViews = $analytics->where('event_type', 'view')->unique('session_id')->count();
            $totalViews = $analytics->where('event_type', 'view')->count();

            return [
                // Basic metrics
                'views' => $post->view_count ?? 0,
                'unique_views' => $uniqueViews,
                'total_views' => $totalViews,
                'comments' => $post->comment_count ?? 0,
                'shares' => $post->share_count ?? 0,
                'likes' => $post->like_count ?? 0,

                // Engagement metrics
                'engagement_rate' => $this->calculateEngagementRate($post),
                'engagement_score' => $this->calculateEngagementScore($post),
                'interactions' => $analytics->whereIn('event_type', ['like', 'comment', 'share'])->count(),

                // Performance metrics
                'avg_time_on_page' => $post->avg_time_on_page ?? 0,
                'bounce_rate' => $this->calculateBounceRate($post),
                'completion_rate' => $this->calculateCompletionRate($post),

                // SEO metrics
                'seo_score' => $post->seo_score ?? 0,
                'organic_views' => $this->getOrganicViews($post, $days),
                'search_impressions' => $post->search_impressions ?? 0,

                // Traffic sources
                'traffic_sources' => $this->getTrafficSources($post, $days),
                'top_referrers' => $this->getTopReferrers($post, $days),

                // Audience insights
                'unique_visitors' => $uniqueViews,
                'returning_visitors' => $this->getReturningVisitors($post, $days),
                'device_breakdown' => $this->getDeviceBreakdown($post, $days),

                // Temporal metrics
                'views_trend' => $this->getViewsTrend($post, $days ?? 30),
                'peak_hours' => $this->getPeakHours($post, $days ?? 30),
            ];
        });
    }

    /**
     * Get post performance summary.
     */
    public function getPerformanceSummary(Post $post): array
    {
        return Cache::remember("post_performance_{$post->id}", self::CACHE_TTL, function () use ($post) {
            $analytics = $this->getAnalytics($post, 30);

            return [
                'status' => $this->getPerformanceStatus($post),
                'score' => $this->calculatePerformanceScore($post),
                'rank' => $this->getPerformanceRank($post),
                'is_trending' => $this->isTrending($post),
                'is_viral' => $this->isViral($post),
                'is_popular' => $this->isPopular($post),
                'recommendations' => $this->generateRecommendations($post, $analytics),
            ];
        });
    }

    // =========================================================================
    // POPULAR & TRENDING METHODS
    // =========================================================================

    /**
     * Get popular posts with advanced filtering.
     */
    public function getPopular(
        int $limit = 10,
        int $days = 30,
        ?string $category = null,
        ?array $tags = null
    ): Collection {
        $cacheKey = "popular_posts_{$limit}_{$days}" . 
                    ($category ? "_{$category}" : '') . 
                    ($tags ? '_' . implode('_', $tags) : '');

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($limit, $days, $category, $tags) {
            $query = Post::where('status', 'published')
                ->where('published_at', '>=', now()->subDays($days))
                ->where('view_count', '>=', self::POPULAR_MIN_VIEWS);

            if ($category) {
                $query->where('category_id', $category);
            }

            if ($tags) {
                $query->whereHas('tags', fn($q) => $q->whereIn('name', $tags));
            }

            return $query->orderBy('view_count', 'desc')
                ->limit($limit)
                ->get()
                ->map(fn($post) => [
                    'post' => $post,
                    'analytics' => $this->getAnalytics($post, $days),
                    'performance' => $this->getPerformanceSummary($post),
                ]);
        });
    }

    /**
     * Get trending posts (high recent engagement).
     */
    public function getTrending(int $limit = 10, int $hours = 24): Collection
    {
        $cacheKey = "trending_posts_{$limit}_{$hours}h";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($limit, $hours) {
            $since = now()->subHours($hours);

            return Post::where('status', 'published')
                ->where('published_at', '>=', $since)
                ->where(function ($query) {
                    $query->where('view_count', '>=', self::TRENDING_MIN_VIEWS)
                        ->orWhere('like_count', '>=', 50)
                        ->orWhere('comment_count', '>=', 20);
                })
                ->orderByRaw('(view_count * 1 + like_count * 5 + comment_count * 10 + share_count * 15) DESC')
                ->limit($limit)
                ->get();
        });
    }

    /**
     * Get viral posts.
     */
    public function getViral(int $limit = 10, int $days = 7): Collection
    {
        return Post::where('status', 'published')
            ->where('published_at', '>=', now()->subDays($days))
            ->where('view_count', '>=', self::VIRAL_MIN_VIEWS)
            ->orderBy('view_count', 'desc')
            ->limit($limit)
            ->get();
    }

    // =========================================================================
    // ENGAGEMENT METRICS METHODS
    // =========================================================================

    /**
     * Get comprehensive engagement statistics.
     */
    public function getEngagementStats(?int $days = null, ?int $categoryId = null): array
    {
        $cacheKey = "engagement_stats" . ($days ? "_{$days}d" : '') . ($categoryId ? "_cat{$categoryId}" : '');

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($days, $categoryId) {
            $query = Post::where('status', 'published');

            if ($days) {
                $query->where('published_at', '>=', now()->subDays($days));
            }

            if ($categoryId) {
                $query->where('category_id', $categoryId);
            }

            $posts = $query->get();

            return [
                'total_posts' => $posts->count(),
                'total_views' => $posts->sum('view_count'),
                'total_comments' => $posts->sum('comment_count'),
                'total_shares' => $posts->sum('share_count'),
                'total_likes' => $posts->sum('like_count'),
                'avg_views_per_post' => round($posts->avg('view_count'), 2),
                'avg_engagement_rate' => round($posts->avg('engagement_rate'), 2),
                'avg_time_on_page' => round($posts->avg('avg_time_on_page')),
                'avg_seo_score' => round($posts->avg('seo_score'), 2),
                'top_performing' => $posts->sortByDesc('view_count')->take(5)->values(),
                'least_performing' => $posts->sortBy('view_count')->take(5)->values(),
            ];
        });
    }

    /**
     * Calculate engagement rate for a post.
     */
    public function calculateEngagementRate(Post $post): float
    {
        $views = $post->view_count ?? 0;

        if ($views === 0) {
            return 0.0;
        }

        $engagements = ($post->like_count ?? 0) + 
                      ($post->comment_count ?? 0) + 
                      ($post->share_count ?? 0);

        return round(($engagements / $views) * 100, 2);
    }

    /**
     * Calculate weighted engagement score.
     */
    public function calculateEngagementScore(Post $post): int
    {
        return (int) (
            ($post->view_count ?? 0) * self::ENGAGEMENT_WEIGHTS['view'] +
            ($post->like_count ?? 0) * self::ENGAGEMENT_WEIGHTS['like'] +
            ($post->comment_count ?? 0) * self::ENGAGEMENT_WEIGHTS['comment'] +
            ($post->share_count ?? 0) * self::ENGAGEMENT_WEIGHTS['share']
        );
    }

    /**
     * Update engagement rate for a post.
     */
    private function updateEngagementRate(Post $post): void
    {
        $rate = $this->calculateEngagementRate($post);
        $post->update(['engagement_rate' => $rate]);
    }

    /**
     * Update engagement counters based on event type.
     */
    private function updateEngagementCounters(Post $post, string $eventType): void
    {
        match ($eventType) {
            'like' => $post->increment('like_count'),
            'comment' => $post->increment('comment_count'),
            'share' => $post->increment('share_count'),
            default => null,
        };
    }

    // =========================================================================
    // PERFORMANCE ANALYSIS METHODS
    // =========================================================================

    /**
     * Calculate bounce rate for a post.
     */
    public function calculateBounceRate(Post $post): float
    {
        $analytics = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->get();

        if ($analytics->isEmpty()) {
            return 0.0;
        }

        $quickExits = $analytics->where('time_spent', '<', 10)->count();
        return round(($quickExits / $analytics->count()) * 100, 2);
    }

    /**
     * Calculate completion rate (read to end).
     */
    public function calculateCompletionRate(Post $post): float
    {
        $analytics = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->whereNotNull('scroll_depth')
            ->get();

        if ($analytics->isEmpty()) {
            return 0.0;
        }

        $completed = $analytics->where('scroll_depth', '>=', 90)->count();
        return round(($completed / $analytics->count()) * 100, 2);
    }

    /**
     * Calculate overall performance score (0-100).
     */
    public function calculatePerformanceScore(Post $post): int
    {
        $weights = [
            'views' => 0.30,
            'engagement' => 0.40,
            'seo' => 0.20,
            'time_on_page' => 0.10,
        ];

        // Normalize metrics to 0-100 scale
        $viewsScore = min(100, ($post->view_count / 100) * 10);
        $engagementScore = min(100, ($post->engagement_rate ?? 0) * 10);
        $seoScore = $post->seo_score ?? 0;
        $timeScore = min(100, ($post->avg_time_on_page / 180) * 100);

        $totalScore = (
            $viewsScore * $weights['views'] +
            $engagementScore * $weights['engagement'] +
            $seoScore * $weights['seo'] +
            $timeScore * $weights['time_on_page']
        );

        return (int) round($totalScore);
    }

    /**
     * Get performance status label.
     */
    public function getPerformanceStatus(Post $post): string
    {
        $score = $this->calculatePerformanceScore($post);

        return match (true) {
            $score >= 90 => 'excellent',
            $score >= 70 => 'good',
            $score >= 50 => 'average',
            $score >= 30 => 'poor',
            default => 'very_poor',
        };
    }

    /**
     * Get post performance rank among all posts.
     */
    public function getPerformanceRank(Post $post): int
    {
        return Post::where('status', 'published')
            ->where('view_count', '>', $post->view_count)
            ->count() + 1;
    }

    // =========================================================================
    // TRAFFIC & AUDIENCE ANALYSIS
    // =========================================================================

    /**
     * Get traffic sources breakdown.
     */
    public function getTrafficSources(Post $post, ?int $days = null): array
    {
        $query = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view');

        if ($days) {
            $query->where('timestamp', '>=', now()->subDays($days));
        }

        $analytics = $query->get();

        return [
            'organic' => $analytics->whereNull('referrer')->count(),
            'social' => $analytics->where('referrer', 'like', '%facebook%')
                ->merge($analytics->where('referrer', 'like', '%twitter%'))
                ->merge($analytics->where('referrer', 'like', '%linkedin%'))
                ->count(),
            'direct' => $analytics->where('referrer', '')->count(),
            'referral' => $analytics->whereNotNull('referrer')
                ->where('referrer', 'not like', '%facebook%')
                ->where('referrer', 'not like', '%twitter%')
                ->where('referrer', 'not like', '%linkedin%')
                ->count(),
        ];
    }

    /**
     * Get top referring domains.
     */
    public function getTopReferrers(Post $post, ?int $days = null, int $limit = 10): array
    {
        $query = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->whereNotNull('referrer');

        if ($days) {
            $query->where('timestamp', '>=', now()->subDays($days));
        }

        return $query->select('referrer', DB::raw('COUNT(*) as count'))
            ->groupBy('referrer')
            ->orderBy('count', 'desc')
            ->limit($limit)
            ->get()
            ->pluck('count', 'referrer')
            ->toArray();
    }

    /**
     * Get organic search views.
     */
    public function getOrganicViews(Post $post, ?int $days = null): int
    {
        $query = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->where(function ($q) {
                $q->where('referrer', 'like', '%google%')
                    ->orWhere('referrer', 'like', '%bing%')
                    ->orWhere('referrer', 'like', '%yahoo%')
                    ->orWhere('referrer', 'like', '%duckduckgo%');
            });

        if ($days) {
            $query->where('timestamp', '>=', now()->subDays($days));
        }

        return $query->count();
    }

    /**
     * Get returning visitors count.
     */
    public function getReturningVisitors(Post $post, ?int $days = null): int
    {
        $query = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view');

        if ($days) {
            $query->where('timestamp', '>=', now()->subDays($days));
        }

        return $query->select('session_id')
            ->groupBy('session_id')
            ->havingRaw('COUNT(*) > 1')
            ->count();
    }

    /**
     * Get device breakdown (desktop, mobile, tablet).
     */
    public function getDeviceBreakdown(Post $post, ?int $days = null): array
    {
        $query = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view');

        if ($days) {
            $query->where('timestamp', '>=', now()->subDays($days));
        }

        $analytics = $query->get();

        return [
            'desktop' => $analytics->where('device_type', 'desktop')->count(),
            'mobile' => $analytics->where('device_type', 'mobile')->count(),
            'tablet' => $analytics->where('device_type', 'tablet')->count(),
        ];
    }

    // =========================================================================
    // TREND ANALYSIS METHODS
    // =========================================================================

    /**
     * Get views trend over time.
     */
    public function getViewsTrend(Post $post, int $days = 30): array
    {
        return PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->where('timestamp', '>=', now()->subDays($days))
            ->select(DB::raw('DATE(timestamp) as date'), DB::raw('COUNT(*) as views'))
            ->groupBy('date')
            ->orderBy('date')
            ->get()
            ->pluck('views', 'date')
            ->toArray();
    }

    /**
     * Get peak traffic hours.
     */
    public function getPeakHours(Post $post, int $days = 30): array
    {
        return PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->where('timestamp', '>=', now()->subDays($days))
            ->select(DB::raw('HOUR(timestamp) as hour'), DB::raw('COUNT(*) as views'))
            ->groupBy('hour')
            ->orderBy('views', 'desc')
            ->limit(5)
            ->get()
            ->pluck('views', 'hour')
            ->toArray();
    }

    /**
     * Check if post is trending.
     */
    public function isTrending(Post $post, int $hours = 24): bool
    {
        $recentViews = PostAnalytic::where('post_id', $post->id)
            ->where('event_type', 'view')
            ->where('timestamp', '>=', now()->subHours($hours))
            ->count();

        return $recentViews >= self::TRENDING_MIN_VIEWS;
    }

    /**
     * Check if post is viral.
     */
    public function isViral(Post $post, int $days = 7): bool
    {
        return $post->view_count >= self::VIRAL_MIN_VIEWS &&
               $post->published_at >= now()->subDays($days);
    }

    /**
     * Check if post is popular.
     */
    public function isPopular(Post $post): bool
    {
        return $post->view_count >= self::POPULAR_MIN_VIEWS;
    }

    // =========================================================================
    // RECOMMENDATIONS & INSIGHTS
    // =========================================================================

    /**
     * Generate performance recommendations.
     */
    public function generateRecommendations(Post $post, array $analytics): array
    {
        $recommendations = [];

        // Low engagement rate
        if ($analytics['engagement_rate'] < 2) {
            $recommendations[] = [
                'type' => 'engagement',
                'priority' => 'high',
                'message' => 'Engagement rate is low. Consider adding more CTAs or interactive elements.',
            ];
        }

        // High bounce rate
        if ($analytics['bounce_rate'] > 70) {
            $recommendations[] = [
                'type' => 'content',
                'priority' => 'high',
                'message' => 'High bounce rate detected. Review content quality and page load speed.',
            ];
        }

        // Low time on page
        if ($analytics['avg_time_on_page'] < 60) {
            $recommendations[] = [
                'type' => 'content',
                'priority' => 'medium',
                'message' => 'Readers are leaving quickly. Consider making content more engaging or scannable.',
            ];
        }

        // SEO opportunity
        if ($analytics['seo_score'] < 70) {
            $recommendations[] = [
                'type' => 'seo',
                'priority' => 'medium',
                'message' => 'SEO score can be improved. Optimize meta tags, headings, and keywords.',
            ];
        }

        // Low organic traffic
        if ($analytics['organic_views'] < $analytics['views'] * 0.2) {
            $recommendations[] = [
                'type' => 'seo',
                'priority' => 'low',
                'message' => 'Low organic traffic. Focus on SEO optimization and keyword targeting.',
            ];
        }

        return $recommendations;
    }

    // =========================================================================
    // COMPARISON & BENCHMARKING
    // =========================================================================

    /**
     * Compare post performance against category average.
     */
    public function compareToCategory(Post $post): array
    {
        $categoryPosts = Post::where('category_id', $post->category_id)
            ->where('status', 'published')
            ->where('id', '!=', $post->id)
            ->get();

        return [
            'views_vs_avg' => $this->calculatePercentageDiff(
                $post->view_count,
                $categoryPosts->avg('view_count')
            ),
            'engagement_vs_avg' => $this->calculatePercentageDiff(
                $post->engagement_rate,
                $categoryPosts->avg('engagement_rate')
            ),
            'rank_in_category' => $categoryPosts->where('view_count', '>', $post->view_count)->count() + 1,
            'total_in_category' => $categoryPosts->count() + 1,
        ];
    }

    /**
     * Calculate percentage difference.
     */
    private function calculatePercentageDiff(float $value, float $average): float
    {
        if ($average == 0) {
            return 0.0;
        }

        return round((($value - $average) / $average) * 100, 2);
    }

    // =========================================================================
    // UTILITY METHODS
    // =========================================================================

    /**
     * Clear analytics cache for a post.
     */
    private function clearAnalyticsCache(int $postId): void
    {
        Cache::forget("post_analytics_{$postId}");
        Cache::forget("post_performance_{$postId}");
        Cache::tags(['post_analytics'])->flush();
    }

    /**
     * Export analytics data to array.
     */
    public function exportAnalytics(Post $post, ?Carbon $startDate = null, ?Carbon $endDate = null): array
    {
        $query = PostAnalytic::where('post_id', $post->id);

        if ($startDate) {
            $query->where('timestamp', '>=', $startDate);
        }

        if ($endDate) {
            $query->where('timestamp', '<=', $endDate);
        }

        return $query->get()->map(function ($analytic) {
            return [
                'event_type' => $analytic->event_type,
                'timestamp' => $analytic->timestamp->toIso8601String(),
                'session_id' => $analytic->session_id,
                'user_id' => $analytic->user_id,
                'time_spent' => $analytic->time_spent,
                'device_type' => $analytic->device_type,
                'referrer' => $analytic->referrer,
            ];
        })->toArray();
    }
}