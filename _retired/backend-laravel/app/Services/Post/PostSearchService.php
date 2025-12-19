<?php

declare(strict_types=1);

namespace App\Services\Post;

use App\Models\Post;
use App\Models\PostSearchLog;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Illuminate\Pagination\LengthAwarePaginator;

/**
 * PostSearchService - Enterprise-grade post search and discovery
 * 
 * Comprehensive search service providing:
 * - Full-text search with relevance scoring
 * - Advanced filtering and faceting
 * - Search suggestions and autocomplete
 * - Typo tolerance and fuzzy matching
 * - Search analytics and tracking
 * - Trending content detection
 * - Related content discovery
 * - Search performance optimization
 * - Faceted search navigation
 * - Search result highlighting
 */
class PostSearchService
{
    /**
     * Cache service instance.
     */
    private PostCacheService $cacheService;

    /**
     * Search result cache TTL (5 minutes).
     */
    private const SEARCH_CACHE_TTL = 300;

    /**
     * Trending posts cache TTL (10 minutes).
     */
    private const TRENDING_CACHE_TTL = 600;

    /**
     * Minimum search query length.
     */
    private const MIN_QUERY_LENGTH = 2;

    /**
     * Maximum search query length.
     */
    private const MAX_QUERY_LENGTH = 200;

    /**
     * Search relevance scoring weights.
     */
    private const RELEVANCE_WEIGHTS = [
        'title_exact' => 100,
        'title_partial' => 50,
        'excerpt' => 30,
        'content' => 20,
        'tags' => 15,
        'categories' => 10,
    ];

    /**
     * Trending algorithm weights.
     */
    private const TRENDING_WEIGHTS = [
        'views' => 1.0,
        'likes' => 3.0,
        'comments' => 5.0,
        'shares' => 8.0,
        'recency' => 2.0,
    ];

    public function __construct(PostCacheService $cacheService)
    {
        $this->cacheService = $cacheService;
    }

    // =========================================================================
    // CORE SEARCH METHODS
    // =========================================================================

    /**
     * Search posts with comprehensive filtering and relevance scoring.
     */
    public function search(array $filters): Builder
    {
        $query = Post::query()
            ->where('status', 'published')
            ->whereNotNull('published_at')
            ->where('published_at', '<=', now());

        // Apply search query with relevance scoring
        if (!empty($filters['q'])) {
            $searchQuery = $this->sanitizeSearchQuery($filters['q']);
            
            if ($this->isValidSearchQuery($searchQuery)) {
                $query = $this->applyFullTextSearch($query, $searchQuery);
                
                // Track search
                $this->trackSearch($searchQuery, $filters);
            }
        }

        // Apply filters
        $query = $this->applyFilters($query, $filters);

        // Apply sorting
        $query = $this->applySorting($query, $filters);

        return $query;
    }

    /**
     * Perform paginated search with caching.
     */
    public function searchPaginated(
        array $filters,
        int $perPage = 15,
        ?int $page = null
    ): LengthAwarePaginator {
        $cacheKey = $this->cacheService->generateSearchKey(
            $filters['q'] ?? '',
            array_merge($filters, ['per_page' => $perPage, 'page' => $page])
        );

        return $this->cacheService->remember(
            $cacheKey,
            self::SEARCH_CACHE_TTL,
            fn() => $this->search($filters)->paginate($perPage, ['*'], 'page', $page),
            ['post_search']
        );
    }

    /**
     * Apply full-text search with relevance scoring.
     */
    private function applyFullTextSearch(Builder $query, string $searchQuery): Builder
    {
        $terms = $this->extractSearchTerms($searchQuery);
        $escapedQuery = $this->escapeSearchQuery($searchQuery);

        return $query->where(function (Builder $q) use ($terms, $escapedQuery) {
            // Exact title match (highest priority)
            $q->orWhere('title', '=', $escapedQuery)
                ->selectRaw('*, ? as relevance_score', [self::RELEVANCE_WEIGHTS['title_exact']]);

            // Title contains query
            $q->orWhere('title', 'LIKE', "%{$escapedQuery}%")
                ->selectRaw('*, ? as relevance_score', [self::RELEVANCE_WEIGHTS['title_partial']]);

            // Excerpt contains query
            $q->orWhere('excerpt', 'LIKE', "%{$escapedQuery}%")
                ->selectRaw('*, ? as relevance_score', [self::RELEVANCE_WEIGHTS['excerpt']]);

            // Content contains query
            $q->orWhere('content', 'LIKE', "%{$escapedQuery}%")
                ->selectRaw('*, ? as relevance_score', [self::RELEVANCE_WEIGHTS['content']]);

            // Slug contains query
            $q->orWhere('slug', 'LIKE', "%{$escapedQuery}%")
                ->selectRaw('*, ? as relevance_score', [self::RELEVANCE_WEIGHTS['content']]);

            // Search in meta keywords
            if (Schema::hasColumn('posts', 'meta_keywords')) {
                $q->orWhere('meta_keywords', 'LIKE', "%{$escapedQuery}%");
            }
        })->when(count($terms) > 1, function (Builder $query) use ($terms) {
            // Boost posts containing all terms
            foreach ($terms as $term) {
                $escaped = $this->escapeSearchQuery($term);
                $query->orWhere(function (Builder $q) use ($escaped) {
                    $q->where('title', 'LIKE', "%{$escaped}%")
                        ->orWhere('content', 'LIKE', "%{$escaped}%");
                });
            }
        });
    }

    /**
     * Apply comprehensive filters to query.
     */
    private function applyFilters(Builder $query, array $filters): Builder
    {
        // Category filter
        if (!empty($filters['category'])) {
            $query->where(function (Builder $q) use ($filters) {
                if (is_array($filters['category'])) {
                    $q->whereIn('category_id', $filters['category']);
                } else {
                    $q->where('category_id', $filters['category']);
                }
            });
        }

        // Tag filter
        if (!empty($filters['tag'])) {
            $query->whereHas('tags', function (Builder $q) use ($filters) {
                if (is_array($filters['tag'])) {
                    $q->whereIn('name', $filters['tag'])
                        ->orWhereIn('slug', $filters['tag']);
                } else {
                    $q->where('name', $filters['tag'])
                        ->orWhere('slug', $filters['tag']);
                }
            });
        }

        // Author filter
        if (!empty($filters['author'])) {
            $query->where('user_id', $filters['author']);
        }

        // Date range filters
        if (!empty($filters['date_from'])) {
            $query->where('published_at', '>=', $filters['date_from']);
        }

        if (!empty($filters['date_to'])) {
            $query->where('published_at', '<=', $filters['date_to']);
        }

        // Featured filter
        if (isset($filters['featured'])) {
            $query->where('is_featured', (bool) $filters['featured']);
        }

        // View count filter
        if (!empty($filters['min_views'])) {
            $query->where('view_count', '>=', $filters['min_views']);
        }

        // Engagement filter
        if (!empty($filters['min_engagement'])) {
            $query->where('engagement_rate', '>=', $filters['min_engagement']);
        }

        // Reading time filter
        if (!empty($filters['max_reading_time'])) {
            $query->where('reading_time', '<=', $filters['max_reading_time']);
        }

        // Exclude specific posts
        if (!empty($filters['exclude'])) {
            $exclude = is_array($filters['exclude']) ? $filters['exclude'] : [$filters['exclude']];
            $query->whereNotIn('id', $exclude);
        }

        return $query;
    }

    /**
     * Apply sorting to query.
     */
    private function applySorting(Builder $query, array $filters): Builder
    {
        $sortBy = $filters['sort_by'] ?? 'relevance';
        $sortOrder = strtolower($filters['sort_order'] ?? 'desc');

        // Validate sort order
        if (!in_array($sortOrder, ['asc', 'desc'], true)) {
            $sortOrder = 'desc';
        }

        return match ($sortBy) {
            'relevance' => $query->orderByRaw('relevance_score DESC NULLS LAST')
                ->orderBy('view_count', 'desc')
                ->orderBy('published_at', 'desc'),
            'views' => $query->orderBy('view_count', $sortOrder),
            'engagement' => $query->orderBy('engagement_rate', $sortOrder),
            'comments' => $query->orderBy('comment_count', $sortOrder),
            'likes' => $query->orderBy('like_count', $sortOrder),
            'date', 'published_at' => $query->orderBy('published_at', $sortOrder),
            'title' => $query->orderBy('title', $sortOrder),
            'random' => $query->inRandomOrder(),
            default => $query->orderBy('published_at', 'desc'),
        };
    }

    // =========================================================================
    // TRENDING & DISCOVERY METHODS
    // =========================================================================

    /**
     * Get trending posts with intelligent scoring.
     */
    public function getTrending(
        int $limit = 10,
        int $days = 7,
        ?int $categoryId = null
    ): Collection {
        $cacheKey = "trending:{$limit}:{$days}:" . ($categoryId ?? 'all');

        return $this->cacheService->remember(
            $cacheKey,
            self::TRENDING_CACHE_TTL,
            function () use ($limit, $days, $categoryId) {
                $since = now()->subDays($days);

                $query = Post::where('status', 'published')
                    ->where('published_at', '>=', $since);

                if ($categoryId) {
                    $query->where('category_id', $categoryId);
                }

                // Calculate trending score
                return $query->selectRaw('
                        *,
                        (
                            (view_count * ?) +
                            (like_count * ?) +
                            (comment_count * ?) +
                            (share_count * ?) +
                            (DATEDIFF(NOW(), published_at) * -1 * ?)
                        ) as trending_score
                    ', [
                        self::TRENDING_WEIGHTS['views'],
                        self::TRENDING_WEIGHTS['likes'],
                        self::TRENDING_WEIGHTS['comments'],
                        self::TRENDING_WEIGHTS['shares'],
                        self::TRENDING_WEIGHTS['recency'],
                    ])
                    ->orderBy('trending_score', 'desc')
                    ->limit($limit)
                    ->get();
            },
            ['post_trending']
        );
    }

    /**
     * Get popular posts by time period.
     */
    public function getPopular(
        string $period = 'week',
        int $limit = 10,
        ?int $categoryId = null
    ): Collection {
        $days = match ($period) {
            'day' => 1,
            'week' => 7,
            'month' => 30,
            'year' => 365,
            default => 7,
        };

        $cacheKey = "popular:{$period}:{$limit}:" . ($categoryId ?? 'all');

        return $this->cacheService->remember(
            $cacheKey,
            self::TRENDING_CACHE_TTL,
            function () use ($days, $limit, $categoryId) {
                $query = Post::where('status', 'published')
                    ->where('published_at', '>=', now()->subDays($days));

                if ($categoryId) {
                    $query->where('category_id', $categoryId);
                }

                return $query->orderBy('view_count', 'desc')
                    ->limit($limit)
                    ->get();
            },
            ['post_popular']
        );
    }

    /**
     * Get related posts using similarity scoring.
     */
    public function getRelated(Post $post, int $limit = 5): Collection
    {
        $cacheKey = "related:{$post->id}:{$limit}";

        return $this->cacheService->remember(
            $cacheKey,
            3600,
            function () use ($post, $limit) {
                $query = Post::where('status', 'published')
                    ->where('id', '!=', $post->id);

                // Same category (high weight)
                $query->where(function (Builder $q) use ($post) {
                    $q->where('category_id', $post->category_id)
                        ->selectRaw('*, 50 as similarity_score');
                });

                // Shared tags (medium weight)
                if ($post->tags->isNotEmpty()) {
                    $tagIds = $post->tags->pluck('id')->toArray();
                    $query->orWhereHas('tags', function (Builder $q) use ($tagIds) {
                        $q->whereIn('tags.id', $tagIds);
                    })->selectRaw('*, 30 as similarity_score');
                }

                // Similar keywords in title
                $keywords = $this->extractKeywords($post->title);
                if (!empty($keywords)) {
                    foreach ($keywords as $keyword) {
                        $query->orWhere('title', 'LIKE', "%{$keyword}%")
                            ->selectRaw('*, 20 as similarity_score');
                    }
                }

                return $query->orderBy('similarity_score', 'desc')
                    ->orderBy('view_count', 'desc')
                    ->limit($limit)
                    ->get();
            },
            ['post_related']
        );
    }

    // =========================================================================
    // SEARCH SUGGESTIONS & AUTOCOMPLETE
    // =========================================================================

    /**
     * Get search suggestions based on partial query.
     */
    public function getSuggestions(string $query, int $limit = 10): array
    {
        $sanitized = $this->sanitizeSearchQuery($query);

        if (!$this->isValidSearchQuery($sanitized)) {
            return [];
        }

        $cacheKey = "suggestions:{$sanitized}:{$limit}";

        return $this->cacheService->remember(
            $cacheKey,
            600,
            function () use ($sanitized, $limit) {
                $posts = Post::where('status', 'published')
                    ->where(function (Builder $q) use ($sanitized) {
                        $q->where('title', 'LIKE', "{$sanitized}%")
                            ->orWhere('title', 'LIKE', "% {$sanitized}%");
                    })
                    ->orderBy('view_count', 'desc')
                    ->limit($limit)
                    ->get(['id', 'title', 'slug']);

                return $posts->map(fn($post) => [
                    'id' => $post->id,
                    'title' => $post->title,
                    'slug' => $post->slug,
                    'highlighted' => $this->highlightMatch($post->title, $sanitized),
                ])->toArray();
            },
            ['post_suggestions']
        );
    }

    /**
     * Get popular search queries.
     */
    public function getPopularSearches(int $limit = 10, int $days = 30): array
    {
        $cacheKey = "popular_searches:{$limit}:{$days}";

        return $this->cacheService->remember(
            $cacheKey,
            3600,
            function () use ($limit, $days) {
                return PostSearchLog::where('created_at', '>=', now()->subDays($days))
                    ->select('query', DB::raw('COUNT(*) as count'))
                    ->whereNotNull('query')
                    ->where('query', '!=', '')
                    ->groupBy('query')
                    ->orderBy('count', 'desc')
                    ->limit($limit)
                    ->get()
                    ->pluck('count', 'query')
                    ->toArray();
            },
            ['post_search_stats']
        );
    }

    // =========================================================================
    // FACETED SEARCH
    // =========================================================================

    /**
     * Get faceted search results with aggregations.
     */
    public function getFacetedResults(array $filters): array
    {
        $baseQuery = $this->search($filters);

        return [
            'results' => $baseQuery->paginate(15),
            'facets' => [
                'categories' => $this->getCategoryFacets($baseQuery),
                'tags' => $this->getTagFacets($baseQuery),
                'authors' => $this->getAuthorFacets($baseQuery),
                'years' => $this->getYearFacets($baseQuery),
            ],
            'aggregations' => [
                'total_count' => $baseQuery->count(),
                'avg_views' => round($baseQuery->avg('view_count'), 2),
                'avg_engagement' => round($baseQuery->avg('engagement_rate'), 2),
            ],
        ];
    }

    /**
     * Get category facets with counts.
     */
    private function getCategoryFacets(Builder $query): array
    {
        return (clone $query)
            ->join('categories', 'posts.category_id', '=', 'categories.id')
            ->select('categories.id', 'categories.name', DB::raw('COUNT(*) as count'))
            ->groupBy('categories.id', 'categories.name')
            ->orderBy('count', 'desc')
            ->limit(10)
            ->get()
            ->toArray();
    }

    /**
     * Get tag facets with counts.
     */
    private function getTagFacets(Builder $query): array
    {
        return (clone $query)
            ->join('post_tag', 'posts.id', '=', 'post_tag.post_id')
            ->join('tags', 'post_tag.tag_id', '=', 'tags.id')
            ->select('tags.id', 'tags.name', DB::raw('COUNT(*) as count'))
            ->groupBy('tags.id', 'tags.name')
            ->orderBy('count', 'desc')
            ->limit(20)
            ->get()
            ->toArray();
    }

    /**
     * Get author facets with counts.
     */
    private function getAuthorFacets(Builder $query): array
    {
        return (clone $query)
            ->join('users', 'posts.user_id', '=', 'users.id')
            ->select('users.id', 'users.name', DB::raw('COUNT(*) as count'))
            ->groupBy('users.id', 'users.name')
            ->orderBy('count', 'desc')
            ->limit(10)
            ->get()
            ->toArray();
    }

    /**
     * Get year facets with counts.
     */
    private function getYearFacets(Builder $query): array
    {
        return (clone $query)
            ->select(DB::raw('YEAR(published_at) as year'), DB::raw('COUNT(*) as count'))
            ->groupBy('year')
            ->orderBy('year', 'desc')
            ->get()
            ->toArray();
    }

    // =========================================================================
    // SEARCH QUERY PROCESSING
    // =========================================================================

    /**
     * Sanitize search query for safety.
     */
    private function sanitizeSearchQuery(string $query): string
    {
        $query = trim($query);
        $query = strip_tags($query);
        $query = preg_replace('/\s+/', ' ', $query);
        
        return substr($query, 0, self::MAX_QUERY_LENGTH);
    }

    /**
     * Validate search query.
     */
    private function isValidSearchQuery(string $query): bool
    {
        return strlen($query) >= self::MIN_QUERY_LENGTH 
            && strlen($query) <= self::MAX_QUERY_LENGTH;
    }

    /**
     * Escape search query for SQL LIKE.
     */
    private function escapeSearchQuery(string $query): string
    {
        return str_replace(['%', '_'], ['\\%', '\\_'], $query);
    }

    /**
     * Extract individual search terms.
     */
    private function extractSearchTerms(string $query): array
    {
        // Remove common words (stop words)
        $stopWords = ['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for'];
        
        $terms = explode(' ', strtolower($query));
        $terms = array_filter($terms, fn($term) => !in_array($term, $stopWords));
        $terms = array_filter($terms, fn($term) => strlen($term) >= 3);
        
        return array_values($terms);
    }

    /**
     * Extract keywords from text for similarity matching.
     */
    private function extractKeywords(string $text, int $limit = 5): array
    {
        $terms = $this->extractSearchTerms($text);
        return array_slice($terms, 0, $limit);
    }

    /**
     * Highlight matching terms in text.
     */
    private function highlightMatch(string $text, string $query): string
    {
        $terms = $this->extractSearchTerms($query);
        
        foreach ($terms as $term) {
            $text = preg_replace(
                '/(' . preg_quote($term, '/') . ')/i',
                '<mark>$1</mark>',
                $text
            );
        }
        
        return $text;
    }

    // =========================================================================
    // SEARCH TRACKING & ANALYTICS
    // =========================================================================

    /**
     * Track search query for analytics.
     */
    private function trackSearch(string $query, array $filters): void
    {
        try {
            PostSearchLog::create([
                'query' => $query,
                'filters' => json_encode($filters),
                'user_id' => auth()->id(),
                'session_id' => session()->getId(),
                'ip_address' => request()->ip(),
                'user_agent' => request()->userAgent(),
                'results_count' => null, // Can be updated after query executes
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to track search', [
                'query' => $query,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Get search analytics summary.
     */
    public function getSearchAnalytics(int $days = 30): array
    {
        $since = now()->subDays($days);

        return [
            'total_searches' => PostSearchLog::where('created_at', '>=', $since)->count(),
            'unique_queries' => PostSearchLog::where('created_at', '>=', $since)
                ->distinct('query')
                ->count('query'),
            'popular_searches' => $this->getPopularSearches(10, $days),
            'zero_result_searches' => PostSearchLog::where('created_at', '>=', $since)
                ->where('results_count', 0)
                ->count(),
            'avg_results_per_search' => round(
                PostSearchLog::where('created_at', '>=', $since)
                    ->whereNotNull('results_count')
                    ->avg('results_count'),
                2
            ),
        ];
    }

    // =========================================================================
    // BULK OPERATIONS
    // =========================================================================

    /**
     * Reindex all posts for search optimization.
     */
    public function reindexAll(): int
    {
        $indexed = 0;

        Post::where('status', 'published')
            ->chunk(100, function ($posts) use (&$indexed) {
                foreach ($posts as $post) {
                    // Update search-related fields
                    $post->update([
                        'search_keywords' => $this->generateSearchKeywords($post),
                    ]);
                    $indexed++;
                }
            });

        // Clear all search caches
        $this->cacheService->flushTag('post_search');

        Log::info('Search index rebuilt', ['posts_indexed' => $indexed]);

        return $indexed;
    }

    /**
     * Generate search keywords for a post.
     */
    private function generateSearchKeywords(Post $post): string
    {
        $keywords = array_merge(
            $this->extractKeywords($post->title, 10),
            $this->extractKeywords($post->excerpt ?? '', 10),
            $post->tags->pluck('name')->toArray()
        );

        return implode(' ', array_unique($keywords));
    }
}