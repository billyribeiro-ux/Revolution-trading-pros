<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Http\Resources\PostResource;
use App\Http\Resources\PostDetailResource;
use App\Services\Post\PostService;
use App\Services\Post\PostCacheService;
use App\Services\Post\PostSearchService;
use App\Services\Post\PostAnalyticsService;
use App\Models\Post;
use App\Models\Category;
use App\Models\Tag;
use App\Enums\PostStatus;
use App\Events\PostViewed;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Validation\Rule;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

class PostController extends Controller
{
    private const CACHE_TTL = 3600; // 1 hour
    private const POPULAR_POSTS_CACHE = 7200; // 2 hours
    private const MAX_PER_PAGE = 100;
    private const DEFAULT_PER_PAGE = 12;
    
    public function __construct(
        private readonly PostService $postService,
        private readonly PostCacheService $cacheService,
        private readonly PostSearchService $searchService,
        private readonly PostAnalyticsService $analyticsService
    ) {}

    /**
     * List posts with advanced filtering, search, and sorting.
     */
    public function index(Request $request): JsonResponse
    {
        $validated = $request->validate([
            // Pagination
            'page' => 'nullable|integer|min:1',
            'per_page' => 'nullable|integer|min:1|max:' . self::MAX_PER_PAGE,
            
            // Filtering
            'status' => ['nullable', Rule::in(['published', 'draft', 'scheduled', 'archived'])],
            'author_id' => 'nullable|integer|exists:users,id',
            'author_ids' => 'nullable|array',
            'author_ids.*' => 'integer|exists:users,id',
            'category_id' => 'nullable|integer|exists:categories,id',
            'category_ids' => 'nullable|array',
            'category_ids.*' => 'integer|exists:categories,id',
            'category_slug' => 'nullable|string|exists:categories,slug',
            'tag_ids' => 'nullable|array',
            'tag_ids.*' => 'integer|exists:tags,id',
            'tag_slugs' => 'nullable|array',
            'tag_slugs.*' => 'string|exists:tags,slug',
            
            // Date filters
            'date_from' => 'nullable|date',
            'date_to' => 'nullable|date|after_or_equal:date_from',
            'year' => 'nullable|integer|min:2000|max:' . date('Y'),
            'month' => 'nullable|integer|min:1|max:12',
            
            // Search
            'search' => 'nullable|string|max:255',
            'search_fields' => 'nullable|array',
            'search_fields.*' => Rule::in(['title', 'content', 'excerpt', 'meta_description']),
            
            // Sorting
            'sort_by' => ['nullable', Rule::in([
                'published_at', 'created_at', 'updated_at', 
                'title', 'views_count', 'comments_count', 
                'likes_count', 'reading_time', 'relevance'
            ])],
            'sort_order' => ['nullable', Rule::in(['asc', 'desc'])],
            
            // Special filters
            'featured' => 'nullable|boolean',
            'has_video' => 'nullable|boolean',
            'has_gallery' => 'nullable|boolean',
            'min_reading_time' => 'nullable|integer|min:1',
            'max_reading_time' => 'nullable|integer|min:1',
            'language' => 'nullable|string|size:2',
            
            // Include relationships
            'include' => 'nullable|array',
            'include.*' => Rule::in([
                'author', 'categories', 'tags', 'comments_count', 
                'related_posts', 'seo_data', 'media', 'translations'
            ]),
            
            // Response format
            'fields' => 'nullable|array',
            'fields.*' => Rule::in([
                'id', 'title', 'slug', 'excerpt', 'content', 
                'featured_image', 'published_at', 'author_id',
                'views_count', 'reading_time', 'meta_title', 
                'meta_description'
            ]),
        ]);

        // Generate cache key
        $cacheKey = $this->cacheService->generateKey('posts:list', $validated);
        
        // Try cache first
        if (!$request->boolean('no_cache')) {
            $cached = Cache::get($cacheKey);
            if ($cached) {
                return response()->json($cached);
            }
        }

        // Build query
        $query = Post::query();
        
        // Apply status filter (default to published for public API)
        $status = $validated['status'] ?? 'published';
        if ($status === 'published') {
            $query->published();
        } else {
            $query->where('status', $status);
        }
        
        // Apply filters
        $this->applyFilters($query, $validated);
        
        // Apply search if provided
        if (!empty($validated['search'])) {
            $searchFields = $validated['search_fields'] ?? ['title', 'content', 'excerpt'];
            $query = $this->searchService->search($query, $validated['search'], $searchFields);
        }
        
        // Apply sorting
        $sortBy = $validated['sort_by'] ?? 'published_at';
        $sortOrder = $validated['sort_order'] ?? 'desc';
        
        if ($sortBy === 'relevance' && !empty($validated['search'])) {
            $query = $this->searchService->orderByRelevance($query, $validated['search']);
        } else {
            $query->orderBy($sortBy, $sortOrder);
        }
        
        // Select specific fields if requested
        if (!empty($validated['fields'])) {
            $fields = array_merge(['id'], $validated['fields']); // Always include ID
            $query->select($fields);
        }
        
        // Eager load relationships
        $this->loadRelationships($query, $validated['include'] ?? []);
        
        // Paginate results
        $perPage = $validated['per_page'] ?? self::DEFAULT_PER_PAGE;
        $posts = $query->paginate($perPage);
        
        // Transform with resource
        $resource = PostResource::collection($posts);
        
        // Add metadata
        $response = [
            'data' => $resource,
            'meta' => [
                'total' => $posts->total(),
                'per_page' => $posts->perPage(),
                'current_page' => $posts->currentPage(),
                'last_page' => $posts->lastPage(),
                'from' => $posts->firstItem(),
                'to' => $posts->lastItem(),
            ],
            'links' => [
                'first' => $posts->url(1),
                'last' => $posts->url($posts->lastPage()),
                'prev' => $posts->previousPageUrl(),
                'next' => $posts->nextPageUrl(),
            ],
        ];
        
        // Add aggregations if requested
        if ($request->boolean('include_aggregations')) {
            $response['aggregations'] = $this->getAggregations($validated);
        }
        
        // Cache the response
        Cache::put($cacheKey, $response, self::CACHE_TTL);
        
        return response()->json($response);
    }

    /**
     * Get a single post by slug with comprehensive data.
     */
    public function show(string $slug, Request $request): JsonResponse
    {
        $validated = $request->validate([
            'include' => 'nullable|array',
            'include.*' => Rule::in([
                'author', 'categories', 'tags', 'comments', 
                'related_posts', 'seo_data', 'media', 'translations',
                'previous_post', 'next_post', 'breadcrumbs'
            ]),
            'preview' => 'nullable|boolean',
            'version' => 'nullable|integer',
        ]);

        // Cache key
        $cacheKey = $this->cacheService->generateKey("posts:show:{$slug}", $validated);
        
        // Try cache first
        if (!$request->boolean('no_cache') && !$validated['preview']) {
            $cached = Cache::tags(['posts', "post:{$slug}"])->get($cacheKey);
            if ($cached) {
                // Still fire view event for analytics
                event(new PostViewed($cached['data']['id'], $request));
                return response()->json($cached);
            }
        }

        // Build query
        $query = Post::where('slug', $slug);
        
        // Handle preview mode
        if ($validated['preview'] ?? false) {
            // Allow draft/scheduled posts in preview mode
            $this->authorize('preview', Post::class);
        } else {
            $query->published();
        }
        
        // Handle versioning
        if (!empty($validated['version'])) {
            $query->where('version', $validated['version']);
        }
        
        // Eager load relationships
        $this->loadRelationships($query, $validated['include'] ?? []);
        
        $post = $query->firstOrFail();
        
        // Increment view count (using Redis for performance)
        if (!$validated['preview']) {
            $this->analyticsService->incrementViews($post);
            event(new PostViewed($post->id, $request));
        }
        
        // Build response
        $response = [
            'data' => new PostDetailResource($post),
        ];
        
        // Add navigation if requested
        if (in_array('previous_post', $validated['include'] ?? [])) {
            $response['navigation']['previous'] = $this->getPreviousPost($post);
        }
        
        if (in_array('next_post', $validated['include'] ?? [])) {
            $response['navigation']['next'] = $this->getNextPost($post);
        }
        
        // Add breadcrumbs if requested
        if (in_array('breadcrumbs', $validated['include'] ?? [])) {
            $response['breadcrumbs'] = $this->generateBreadcrumbs($post);
        }
        
        // Add SEO data
        $response['seo'] = $this->generateSeoData($post);
        
        // Add reading progress data
        $response['reading'] = [
            'time_minutes' => $post->reading_time,
            'word_count' => $post->word_count,
            'difficulty_level' => $post->difficulty_level ?? 'medium',
        ];
        
        // Cache the response (shorter TTL for single posts)
        if (!$validated['preview']) {
            Cache::tags(['posts', "post:{$slug}"])->put($cacheKey, $response, 1800);
        }
        
        return response()->json($response);
    }

    /**
     * Get popular posts.
     */
    public function popular(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'period' => ['nullable', Rule::in(['day', 'week', 'month', 'year', 'all'])],
            'limit' => 'nullable|integer|min:1|max:50',
            'category_id' => 'nullable|integer|exists:categories,id',
        ]);

        $period = $validated['period'] ?? 'week';
        $limit = $validated['limit'] ?? 10;
        
        $cacheKey = "posts:popular:{$period}:{$limit}";
        if (!empty($validated['category_id'])) {
            $cacheKey .= ":{$validated['category_id']}";
        }

        return Cache::tags(['posts', 'posts:popular'])->remember(
            $cacheKey,
            self::POPULAR_POSTS_CACHE,
            function () use ($period, $limit, $validated) {
                $query = Post::published()
                    ->select([
                        'id', 'title', 'slug', 'excerpt', 
                        'featured_image', 'published_at', 
                        'views_count', 'reading_time'
                    ])
                    ->with('author:id,name');

                // Filter by period
                if ($period !== 'all') {
                    $startDate = match($period) {
                        'day' => now()->subDay(),
                        'week' => now()->subWeek(),
                        'month' => now()->subMonth(),
                        'year' => now()->subYear(),
                    };
                    $query->where('published_at', '>=', $startDate);
                }

                // Filter by category if provided
                if (!empty($validated['category_id'])) {
                    $query->whereHas('categories', function ($q) use ($validated) {
                        $q->where('categories.id', $validated['category_id']);
                    });
                }

                $posts = $query->orderBy('views_count', 'desc')
                    ->limit($limit)
                    ->get();

                return [
                    'data' => PostResource::collection($posts),
                    'period' => $period,
                    'generated_at' => now()->toIso8601String(),
                ];
            }
        );
    }

    /**
     * Get trending posts based on velocity.
     */
    public function trending(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'hours' => 'nullable|integer|min:1|max:168', // Max 1 week
            'limit' => 'nullable|integer|min:1|max:50',
        ]);

        $hours = $validated['hours'] ?? 24;
        $limit = $validated['limit'] ?? 10;

        $trending = $this->analyticsService->getTrendingPosts($hours, $limit);

        return response()->json([
            'data' => PostResource::collection($trending),
            'period_hours' => $hours,
            'algorithm' => 'velocity_based',
        ]);
    }

    /**
     * Get related posts.
     */
    public function related(string $slug, Request $request): JsonResponse
    {
        $validated = $request->validate([
            'limit' => 'nullable|integer|min:1|max:20',
            'algorithm' => ['nullable', Rule::in(['tags', 'categories', 'content', 'hybrid'])],
        ]);

        $post = Post::where('slug', $slug)->published()->firstOrFail();
        $limit = $validated['limit'] ?? 5;
        $algorithm = $validated['algorithm'] ?? 'hybrid';

        $cacheKey = "posts:related:{$post->id}:{$algorithm}:{$limit}";

        $related = Cache::tags(['posts', 'posts:related'])->remember(
            $cacheKey,
            self::CACHE_TTL,
            function () use ($post, $limit, $algorithm) {
                return $this->postService->getRelatedPosts($post, $limit, $algorithm);
            }
        );

        return response()->json([
            'data' => PostResource::collection($related),
            'algorithm' => $algorithm,
        ]);
    }

    /**
     * Search posts with advanced options.
     */
    public function search(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'q' => 'required|string|min:2|max:255',
            'fields' => 'nullable|array',
            'fields.*' => Rule::in(['title', 'content', 'excerpt', 'tags']),
            'fuzzy' => 'nullable|boolean',
            'boost_recent' => 'nullable|boolean',
            'category_ids' => 'nullable|array',
            'category_ids.*' => 'integer|exists:categories,id',
            'author_ids' => 'nullable|array',
            'author_ids.*' => 'integer|exists:users,id',
            'date_from' => 'nullable|date',
            'date_to' => 'nullable|date',
            'min_score' => 'nullable|numeric|min:0|max:1',
            'limit' => 'nullable|integer|min:1|max:100',
            'offset' => 'nullable|integer|min:0',
            'highlight' => 'nullable|boolean',
        ]);

        $results = $this->searchService->advancedSearch($validated);

        return response()->json([
            'data' => PostResource::collection($results['hits']),
            'meta' => [
                'total' => $results['total'],
                'max_score' => $results['max_score'],
                'query' => $validated['q'],
                'took_ms' => $results['took_ms'],
            ],
            'aggregations' => $results['aggregations'] ?? [],
            'suggestions' => $results['suggestions'] ?? [],
        ]);
    }

    /**
     * Get posts by category.
     */
    public function byCategory(string $categorySlug, Request $request): JsonResponse
    {
        $category = Category::where('slug', $categorySlug)->firstOrFail();
        
        $validated = $request->validate([
            'include_subcategories' => 'nullable|boolean',
            'per_page' => 'nullable|integer|min:1|max:' . self::MAX_PER_PAGE,
            'sort_by' => ['nullable', Rule::in(['published_at', 'popularity', 'title'])],
        ]);

        $query = Post::published()
            ->whereHas('categories', function ($q) use ($category, $validated) {
                if ($validated['include_subcategories'] ?? false) {
                    $categoryIds = $category->descendants()->pluck('id')->push($category->id);
                    $q->whereIn('categories.id', $categoryIds);
                } else {
                    $q->where('categories.id', $category->id);
                }
            });

        // Apply sorting
        $sortBy = $validated['sort_by'] ?? 'published_at';
        if ($sortBy === 'popularity') {
            $query->orderBy('views_count', 'desc');
        } else {
            $query->orderBy($sortBy, $sortBy === 'title' ? 'asc' : 'desc');
        }

        $posts = $query->paginate($validated['per_page'] ?? self::DEFAULT_PER_PAGE);

        return response()->json([
            'category' => [
                'id' => $category->id,
                'name' => $category->name,
                'slug' => $category->slug,
                'description' => $category->description,
                'post_count' => $category->posts()->published()->count(),
            ],
            'posts' => PostResource::collection($posts),
            'meta' => [
                'total' => $posts->total(),
                'per_page' => $posts->perPage(),
                'current_page' => $posts->currentPage(),
                'last_page' => $posts->lastPage(),
            ],
        ]);
    }

    /**
     * Get posts by tag.
     */
    public function byTag(string $tagSlug, Request $request): JsonResponse
    {
        $tag = Tag::where('slug', $tagSlug)->firstOrFail();
        
        $validated = $request->validate([
            'per_page' => 'nullable|integer|min:1|max:' . self::MAX_PER_PAGE,
        ]);

        $posts = Post::published()
            ->whereHas('tags', function ($q) use ($tag) {
                $q->where('tags.id', $tag->id);
            })
            ->orderBy('published_at', 'desc')
            ->paginate($validated['per_page'] ?? self::DEFAULT_PER_PAGE);

        return response()->json([
            'tag' => [
                'id' => $tag->id,
                'name' => $tag->name,
                'slug' => $tag->slug,
                'post_count' => $tag->posts()->published()->count(),
            ],
            'posts' => PostResource::collection($posts),
            'meta' => [
                'total' => $posts->total(),
                'per_page' => $posts->perPage(),
                'current_page' => $posts->currentPage(),
                'last_page' => $posts->lastPage(),
            ],
        ]);
    }

    /**
     * Get posts by author.
     */
    public function byAuthor(int $authorId, Request $request): JsonResponse
    {
        $validated = $request->validate([
            'per_page' => 'nullable|integer|min:1|max:' . self::MAX_PER_PAGE,
            'include_bio' => 'nullable|boolean',
        ]);

        $posts = Post::published()
            ->where('author_id', $authorId)
            ->with('categories:id,name,slug')
            ->orderBy('published_at', 'desc')
            ->paginate($validated['per_page'] ?? self::DEFAULT_PER_PAGE);

        $response = [
            'posts' => PostResource::collection($posts),
            'meta' => [
                'total' => $posts->total(),
                'per_page' => $posts->perPage(),
                'current_page' => $posts->currentPage(),
                'last_page' => $posts->lastPage(),
            ],
        ];

        if ($validated['include_bio'] ?? false) {
            $author = \App\Models\User::find($authorId);
            if ($author) {
                $response['author'] = [
                    'id' => $author->id,
                    'name' => $author->name,
                    'bio' => $author->bio,
                    'avatar' => $author->avatar_url,
                    'social_links' => $author->social_links,
                    'post_count' => $author->posts()->published()->count(),
                ];
            }
        }

        return response()->json($response);
    }

    /**
     * Get post archives.
     */
    public function archives(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'group_by' => ['nullable', Rule::in(['year', 'month', 'year_month'])],
        ]);

        $groupBy = $validated['group_by'] ?? 'year_month';
        
        $cacheKey = "posts:archives:{$groupBy}";
        
        $archives = Cache::tags(['posts', 'posts:archives'])->remember(
            $cacheKey,
            self::CACHE_TTL,
            function () use ($groupBy) {
                $query = Post::published()
                    ->select(
                        DB::raw('YEAR(published_at) as year'),
                        DB::raw('MONTH(published_at) as month'),
                        DB::raw('COUNT(*) as post_count')
                    )
                    ->groupBy('year');

                if ($groupBy === 'year_month' || $groupBy === 'month') {
                    $query->groupBy('month');
                }

                return $query->orderBy('year', 'desc')
                    ->orderBy('month', 'desc')
                    ->get()
                    ->map(function ($item) {
                        return [
                            'year' => $item->year,
                            'month' => $item->month,
                            'month_name' => Carbon::create()->month($item->month)->format('F'),
                            'post_count' => $item->post_count,
                            'url' => route('posts.archive', [
                                'year' => $item->year,
                                'month' => $item->month
                            ]),
                        ];
                    });
            }
        );

        return response()->json([
            'data' => $archives,
            'group_by' => $groupBy,
        ]);
    }

    /**
     * Get RSS feed.
     */
    public function feed(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'format' => ['nullable', Rule::in(['rss', 'atom', 'json'])],
            'limit' => 'nullable|integer|min:1|max:100',
            'category_slug' => 'nullable|string|exists:categories,slug',
        ]);

        $format = $validated['format'] ?? 'rss';
        $limit = $validated['limit'] ?? 20;
        
        $query = Post::published()
            ->orderBy('published_at', 'desc')
            ->limit($limit);

        if (!empty($validated['category_slug'])) {
            $query->whereHas('categories', function ($q) use ($validated) {
                $q->where('slug', $validated['category_slug']);
            });
        }

        $posts = $query->get();

        $feed = $this->postService->generateFeed($posts, $format);

        return response()->json($feed)
            ->header('Content-Type', match($format) {
                'atom' => 'application/atom+xml',
                'json' => 'application/json',
                default => 'application/rss+xml',
            });
    }

    // Helper methods

    /**
     * Apply filters to query.
     */
    private function applyFilters(Builder $query, array $filters): void
    {
        // Author filters
        if (!empty($filters['author_id'])) {
            $query->where('author_id', $filters['author_id']);
        } elseif (!empty($filters['author_ids'])) {
            $query->whereIn('author_id', $filters['author_ids']);
        }

        // Category filters
        if (!empty($filters['category_id'])) {
            $query->whereHas('categories', fn($q) => $q->where('categories.id', $filters['category_id']));
        } elseif (!empty($filters['category_ids'])) {
            $query->whereHas('categories', fn($q) => $q->whereIn('categories.id', $filters['category_ids']));
        } elseif (!empty($filters['category_slug'])) {
            $query->whereHas('categories', fn($q) => $q->where('categories.slug', $filters['category_slug']));
        }

        // Tag filters
        if (!empty($filters['tag_ids'])) {
            $query->whereHas('tags', fn($q) => $q->whereIn('tags.id', $filters['tag_ids']));
        } elseif (!empty($filters['tag_slugs'])) {
            $query->whereHas('tags', fn($q) => $q->whereIn('tags.slug', $filters['tag_slugs']));
        }

        // Date filters
        if (!empty($filters['date_from'])) {
            $query->where('published_at', '>=', $filters['date_from']);
        }
        if (!empty($filters['date_to'])) {
            $query->where('published_at', '<=', $filters['date_to']);
        }
        if (!empty($filters['year'])) {
            $query->whereYear('published_at', $filters['year']);
        }
        if (!empty($filters['month'])) {
            $query->whereMonth('published_at', $filters['month']);
        }

        // Special filters
        if (isset($filters['featured'])) {
            $query->where('is_featured', $filters['featured']);
        }
        if (isset($filters['has_video'])) {
            $query->where('has_video', $filters['has_video']);
        }
        if (isset($filters['has_gallery'])) {
            $query->where('has_gallery', $filters['has_gallery']);
        }
        if (!empty($filters['min_reading_time'])) {
            $query->where('reading_time', '>=', $filters['min_reading_time']);
        }
        if (!empty($filters['max_reading_time'])) {
            $query->where('reading_time', '<=', $filters['max_reading_time']);
        }
        if (!empty($filters['language'])) {
            $query->where('language', $filters['language']);
        }
    }

    /**
     * Load relationships.
     */
    private function loadRelationships(Builder $query, array $include): void
    {
        $relationships = [];

        if (in_array('author', $include)) {
            $relationships[] = 'author:id,name,slug,avatar';
        }
        if (in_array('categories', $include)) {
            $relationships[] = 'categories:id,name,slug';
        }
        if (in_array('tags', $include)) {
            $relationships[] = 'tags:id,name,slug';
        }
        if (in_array('comments_count', $include)) {
            $query->withCount('comments');
        }
        if (in_array('media', $include)) {
            $relationships[] = 'media';
        }
        if (in_array('translations', $include)) {
            $relationships[] = 'translations';
        }
        if (in_array('related_posts', $include)) {
            $relationships[] = 'relatedPosts:id,title,slug,featured_image';
        }
        if (in_array('seo_data', $include)) {
            $relationships[] = 'seoData';
        }

        if (!empty($relationships)) {
            $query->with($relationships);
        }
    }

    /**
     * Get aggregations for the current filter set.
     */
    private function getAggregations(array $filters): array
    {
        return [
            'total_posts' => Post::published()->count(),
            'by_category' => $this->getCategoryAggregations($filters),
            'by_tag' => $this->getTagAggregations($filters),
            'by_author' => $this->getAuthorAggregations($filters),
            'by_month' => $this->getMonthlyAggregations($filters),
        ];
    }

    /**
     * Get category aggregations.
     */
    private function getCategoryAggregations(array $filters): array
    {
        return Cache::tags(['posts:aggregations'])->remember(
            'posts:agg:categories',
            300,
            function () {
                return DB::table('categories')
                    ->join('category_post', 'categories.id', '=', 'category_post.category_id')
                    ->join('posts', 'category_post.post_id', '=', 'posts.id')
                    ->where('posts.status', 'published')
                    ->where('posts.published_at', '<=', now())
                    ->select('categories.id', 'categories.name', 'categories.slug')
                    ->selectRaw('COUNT(DISTINCT posts.id) as post_count')
                    ->groupBy('categories.id', 'categories.name', 'categories.slug')
                    ->orderBy('post_count', 'desc')
                    ->limit(10)
                    ->get();
            }
        );
    }

    /**
     * Get tag aggregations.
     */
    private function getTagAggregations(array $filters): array
    {
        return Cache::tags(['posts:aggregations'])->remember(
            'posts:agg:tags',
            300,
            function () {
                return DB::table('tags')
                    ->join('post_tag', 'tags.id', '=', 'post_tag.tag_id')
                    ->join('posts', 'post_tag.post_id', '=', 'posts.id')
                    ->where('posts.status', 'published')
                    ->where('posts.published_at', '<=', now())
                    ->select('tags.id', 'tags.name', 'tags.slug')
                    ->selectRaw('COUNT(DISTINCT posts.id) as post_count')
                    ->groupBy('tags.id', 'tags.name', 'tags.slug')
                    ->orderBy('post_count', 'desc')
                    ->limit(20)
                    ->get();
            }
        );
    }

    /**
     * Get author aggregations.
     */
    private function getAuthorAggregations(array $filters): array
    {
        return Cache::tags(['posts:aggregations'])->remember(
            'posts:agg:authors',
            300,
            function () {
                return DB::table('users')
                    ->join('posts', 'users.id', '=', 'posts.author_id')
                    ->where('posts.status', 'published')
                    ->where('posts.published_at', '<=', now())
                    ->select('users.id', 'users.name')
                    ->selectRaw('COUNT(posts.id) as post_count')
                    ->groupBy('users.id', 'users.name')
                    ->orderBy('post_count', 'desc')
                    ->limit(10)
                    ->get();
            }
        );
    }

    /**
     * Get monthly aggregations.
     */
    private function getMonthlyAggregations(array $filters): array
    {
        return Cache::tags(['posts:aggregations'])->remember(
            'posts:agg:monthly',
            3600,
            function () {
                return Post::published()
                    ->select(
                        DB::raw('YEAR(published_at) as year'),
                        DB::raw('MONTH(published_at) as month'),
                        DB::raw('COUNT(*) as post_count')
                    )
                    ->where('published_at', '>=', now()->subYear())
                    ->groupBy('year', 'month')
                    ->orderBy('year', 'desc')
                    ->orderBy('month', 'desc')
                    ->get();
            }
        );
    }

    /**
     * Get previous post.
     */
    private function getPreviousPost(Post $post): ?array
    {
        $previous = Post::published()
            ->where('published_at', '<', $post->published_at)
            ->orderBy('published_at', 'desc')
            ->select('id', 'title', 'slug')
            ->first();

        return $previous ? [
            'id' => $previous->id,
            'title' => $previous->title,
            'slug' => $previous->slug,
            'url' => route('posts.show', $previous->slug),
        ] : null;
    }

    /**
     * Get next post.
     */
    private function getNextPost(Post $post): ?array
    {
        $next = Post::published()
            ->where('published_at', '>', $post->published_at)
            ->orderBy('published_at', 'asc')
            ->select('id', 'title', 'slug')
            ->first();

        return $next ? [
            'id' => $next->id,
            'title' => $next->title,
            'slug' => $next->slug,
            'url' => route('posts.show', $next->slug),
        ] : null;
    }

    /**
     * Generate breadcrumbs.
     */
    private function generateBreadcrumbs(Post $post): array
    {
        $breadcrumbs = [
            ['title' => 'Home', 'url' => route('home')],
            ['title' => 'Blog', 'url' => route('posts.index')],
        ];

        if ($post->categories->isNotEmpty()) {
            $category = $post->categories->first();
            $breadcrumbs[] = [
                'title' => $category->name,
                'url' => route('posts.category', $category->slug),
            ];
        }

        $breadcrumbs[] = [
            'title' => $post->title,
            'url' => route('posts.show', $post->slug),
            'current' => true,
        ];

        return $breadcrumbs;
    }

    /**
     * Generate SEO data.
     */
    private function generateSeoData(Post $post): array
    {
        return [
            'title' => $post->meta_title ?: $post->title . ' | ' . config('app.name'),
            'description' => $post->meta_description ?: Str::limit(strip_tags($post->content), 160),
            'keywords' => $post->meta_keywords ?: $post->tags->pluck('name')->implode(', '),
            'canonical_url' => route('posts.show', $post->slug),
            'og' => [
                'title' => $post->og_title ?: $post->title,
                'description' => $post->og_description ?: $post->excerpt,
                'image' => $post->og_image ?: $post->featured_image,
                'type' => 'article',
                'url' => route('posts.show', $post->slug),
                'article:published_time' => $post->published_at->toIso8601String(),
                'article:modified_time' => $post->updated_at->toIso8601String(),
                'article:author' => $post->author->name,
                'article:section' => $post->categories->first()?->name,
                'article:tag' => $post->tags->pluck('name')->toArray(),
            ],
            'twitter' => [
                'card' => 'summary_large_image',
                'title' => $post->twitter_title ?: $post->title,
                'description' => $post->twitter_description ?: $post->excerpt,
                'image' => $post->twitter_image ?: $post->featured_image,
                'creator' => $post->author->twitter_handle,
            ],
            'schema' => [
                '@context' => 'https://schema.org',
                '@type' => 'BlogPosting',
                'headline' => $post->title,
                'description' => $post->excerpt,
                'image' => $post->featured_image,
                'datePublished' => $post->published_at->toIso8601String(),
                'dateModified' => $post->updated_at->toIso8601String(),
                'author' => [
                    '@type' => 'Person',
                    'name' => $post->author->name,
                    'url' => route('posts.author', $post->author->id),
                ],
                'publisher' => [
                    '@type' => 'Organization',
                    'name' => config('app.name'),
                    'logo' => [
                        '@type' => 'ImageObject',
                        'url' => asset('logo.png'),
                    ],
                ],
                'mainEntityOfPage' => [
                    '@type' => 'WebPage',
                    '@id' => route('posts.show', $post->slug),
                ],
                'wordCount' => $post->word_count,
                'commentCount' => $post->comments_count ?? 0,
                'articleSection' => $post->categories->pluck('name')->toArray(),
                'keywords' => $post->tags->pluck('name')->implode(', '),
            ],
        ];
    }
}