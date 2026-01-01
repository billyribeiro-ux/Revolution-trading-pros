<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Http\Resources\RankTrackingResource;
use App\Http\Resources\RankHistoryResource;
use App\Http\Resources\CompetitorRankingResource;
use App\Services\RankTracking\SerpApiService;
use App\Services\RankTracking\RankAnalysisService;
use App\Services\RankTracking\CompetitorTrackingService;
use App\Services\RankTracking\RankAlertService;
use App\Models\RankTracking;
use App\Models\RankHistory;
use App\Models\CompetitorDomain;
use App\Models\RankAlert;
use App\Models\SearchEngine;
use App\Jobs\CheckRankingsJob;
use App\Jobs\BulkRankCheckJob;
use App\Events\RankingChanged;
use App\Events\RankingDropped;
use App\Enums\RankChangeType;
use App\Enums\SearchEngineType;
use App\Enums\DeviceType;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\RateLimiter;
use Illuminate\Validation\Rule;
use Illuminate\Validation\ValidationException;
use Symfony\Component\HttpFoundation\Response;
use Carbon\Carbon;

class RankingController extends Controller
{
    private const CACHE_TTL = 1800; // 30 minutes
    private const RATE_LIMIT_KEY = 'rank-check';
    private const MAX_KEYWORDS_PER_REQUEST = 100;
    private const MAX_BULK_KEYWORDS = 1000;
    
    /**
     * Search engine configurations with API endpoints
     */
    private const SEARCH_ENGINES = [
        'google' => [
            'name' => 'Google',
            'api_endpoint' => 'https://api.serpapi.com/search',
            'supports_local' => true,
            'max_results' => 100,
        ],
        'bing' => [
            'name' => 'Bing',
            'api_endpoint' => 'https://api.bing.com/search',
            'supports_local' => true,
            'max_results' => 50,
        ],
        'yahoo' => [
            'name' => 'Yahoo',
            'api_endpoint' => 'https://api.yahoo.com/search',
            'supports_local' => false,
            'max_results' => 50,
        ],
        'duckduckgo' => [
            'name' => 'DuckDuckGo',
            'api_endpoint' => 'https://api.duckduckgo.com/search',
            'supports_local' => false,
            'max_results' => 30,
        ],
    ];
    
    public function __construct(
        private readonly SerpApiService $serpApi,
        private readonly RankAnalysisService $analysisService,
        private readonly CompetitorTrackingService $competitorService,
        private readonly RankAlertService $alertService
    ) {}

    /**
     * List all rank trackings with advanced filtering.
     */
    public function index(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keywords' => 'nullable|array|max:100',
            'keywords.*' => 'string|max:255',
            'urls' => 'nullable|array|max:50',
            'urls.*' => 'url',
            'search_engines' => 'nullable|array',
            'search_engines.*' => Rule::in(array_keys(self::SEARCH_ENGINES)),
            'locations' => 'nullable|array',
            'locations.*' => 'string|max:10',
            'devices' => 'nullable|array',
            'devices.*' => Rule::in(['desktop', 'mobile', 'tablet']),
            'position_range' => 'nullable|array|size:2',
            'position_range.0' => 'integer|min:1',
            'position_range.1' => 'integer|min:1',
            'tags' => 'nullable|array',
            'tags.*' => 'string|max:50',
            'group_by' => 'nullable|string|in:keyword,url,search_engine,location',
            'sort_by' => 'nullable|string|in:keyword,position,change,last_checked,created_at',
            'sort_order' => 'nullable|string|in:asc,desc',
            'include' => 'nullable|array',
            'include.*' => Rule::in(['history', 'competitors', 'serp_features', 'alerts']),
            'page' => 'integer|min:1',
            'per_page' => 'integer|min:1|max:100',
        ]);

        $cacheKey = 'rankings:' . md5(json_encode($validated));
        
        $result = Cache::remember($cacheKey, 300, function () use ($validated) {
            $query = RankTracking::query()
                ->with(['searchEngine', 'device', 'location']);

            // Apply filters
            if (!empty($validated['keywords'])) {
                $query->whereIn('keyword', $validated['keywords']);
            }

            if (!empty($validated['urls'])) {
                $query->whereIn('url', $validated['urls']);
            }

            if (!empty($validated['search_engines'])) {
                $query->whereIn('search_engine', $validated['search_engines']);
            }

            if (!empty($validated['locations'])) {
                $query->whereIn('location', $validated['locations']);
            }

            if (!empty($validated['devices'])) {
                $query->whereIn('device', $validated['devices']);
            }

            if (!empty($validated['position_range'])) {
                $query->whereBetween('current_position', $validated['position_range']);
            }

            if (!empty($validated['tags'])) {
                $query->whereJsonContains('tags', $validated['tags']);
            }

            // Apply sorting
            $sortBy = $validated['sort_by'] ?? 'last_checked_at';
            $sortOrder = $validated['sort_order'] ?? 'desc';
            
            if ($sortBy === 'change') {
                $query->orderByRaw('(current_position - previous_position) ' . $sortOrder);
            } else {
                $query->orderBy($sortBy, $sortOrder);
            }

            // Eager load relationships if requested
            if (!empty($validated['include'])) {
                if (in_array('history', $validated['include'])) {
                    $query->with(['histories' => function ($q) {
                        $q->orderBy('created_at', 'desc')->limit(30);
                    }]);
                }
                
                if (in_array('competitors', $validated['include'])) {
                    $query->with('competitors');
                }
                
                if (in_array('serp_features', $validated['include'])) {
                    $query->with('serpFeatures');
                }
                
                if (in_array('alerts', $validated['include'])) {
                    $query->with(['alerts' => function ($q) {
                        $q->where('is_active', true);
                    }]);
                }
            }

            return $query->paginate($validated['per_page'] ?? 50);
        });

        // Add analytics data
        $analytics = $this->getAnalytics($result->items());

        // Group results if requested
        if (!empty($validated['group_by'])) {
            $grouped = $this->groupResults($result->items(), $validated['group_by']);
            
            return response()->json([
                'grouped_rankings' => $grouped,
                'analytics' => $analytics,
                'pagination' => [
                    'total' => $result->total(),
                    'per_page' => $result->perPage(),
                    'current_page' => $result->currentPage(),
                    'last_page' => $result->lastPage(),
                ],
            ]);
        }

        return response()->json([
            'rankings' => RankTrackingResource::collection($result),
            'analytics' => $analytics,
            'pagination' => [
                'total' => $result->total(),
                'per_page' => $result->perPage(),
                'current_page' => $result->currentPage(),
                'last_page' => $result->lastPage(),
            ],
        ]);
    }

    /**
     * Check rankings with real SERP API.
     */
    public function check(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'tracking_ids' => 'nullable|array|max:' . self::MAX_KEYWORDS_PER_REQUEST,
            'tracking_ids.*' => 'integer|exists:rank_trackings,id',
            'force' => 'boolean',
            'async' => 'boolean',
        ]);

        // Rate limiting
        $rateLimitKey = self::RATE_LIMIT_KEY . ':' . $request->user()->id;
        if (!RateLimiter::attempt($rateLimitKey, 60, fn() => null, 3600)) {
            return response()->json([
                'error' => 'Too many rank check requests',
                'retry_after' => RateLimiter::availableIn($rateLimitKey),
            ], Response::HTTP_TOO_MANY_REQUESTS);
        }

        // Get trackings to check
        $query = RankTracking::query()->where('is_active', true);
        
        if (!empty($validated['tracking_ids'])) {
            $query->whereIn('id', $validated['tracking_ids']);
        } else {
            // Check trackings that need updating
            $query->where(function ($q) {
                $q->whereNull('last_checked_at')
                  ->orWhere('last_checked_at', '<=', now()->subHours(24));
            });
        }

        $trackings = $query->limit(self::MAX_KEYWORDS_PER_REQUEST)->get();

        if ($trackings->isEmpty()) {
            return response()->json([
                'message' => 'No rankings need checking',
                'next_check' => $this->getNextScheduledCheck(),
            ]);
        }

        // Check if async processing is requested
        if ($validated['async'] ?? false) {
            $jobId = uniqid('rank_check_', true);
            
            CheckRankingsJob::dispatch($trackings->pluck('id')->toArray(), $jobId)
                ->onQueue('rank-checking')
                ->delay(now()->addSeconds(2));

            return response()->json([
                'status' => 'queued',
                'job_id' => $jobId,
                'trackings_count' => $trackings->count(),
                'status_url' => route('api.rankings.check-status', ['jobId' => $jobId]),
                'estimated_completion' => now()->addSeconds($trackings->count() * 2),
            ], Response::HTTP_ACCEPTED);
        }

        // Synchronous checking
        $results = [
            'success' => [],
            'failed' => [],
            'unchanged' => [],
        ];

        DB::beginTransaction();

        try {
            foreach ($trackings as $tracking) {
                try {
                    $previousPosition = $tracking->current_position;
                    
                    // Get real SERP data
                    $serpData = $this->serpApi->checkRanking(
                        $tracking->keyword,
                        $tracking->url,
                        [
                            'search_engine' => $tracking->search_engine,
                            'location' => $tracking->location,
                            'device' => $tracking->device,
                            'language' => $tracking->language,
                            'num_results' => 100,
                        ]
                    );

                    // Update tracking with real data
                    $tracking->update([
                        'current_position' => $serpData['position'],
                        'previous_position' => $previousPosition,
                        'search_volume' => $serpData['search_volume'] ?? null,
                        'cpc' => $serpData['cpc'] ?? null,
                        'competition' => $serpData['competition'] ?? null,
                        'serp_features' => $serpData['serp_features'] ?? [],
                        'top_competitors' => array_slice($serpData['top_results'] ?? [], 0, 10),
                        'last_checked_at' => now(),
                        'last_serp_data' => $serpData,
                    ]);

                    // Record history
                    RankHistory::create([
                        'rank_tracking_id' => $tracking->id,
                        'position' => $serpData['position'],
                        'search_volume' => $serpData['search_volume'] ?? null,
                        'serp_features' => $serpData['serp_features'] ?? [],
                        'competitors' => $serpData['top_results'] ?? [],
                        'date' => now()->toDateString(),
                        'checked_at' => now(),
                    ]);

                    // Check for significant changes
                    if ($previousPosition && abs($serpData['position'] - $previousPosition) >= 3) {
                        $changeType = $serpData['position'] < $previousPosition 
                            ? RankChangeType::IMPROVED 
                            : RankChangeType::DROPPED;
                        
                        event(new RankingChanged($tracking, $previousPosition, $serpData['position'], $changeType));
                        
                        // Create alert for significant drops
                        if ($changeType === RankChangeType::DROPPED && abs($serpData['position'] - $previousPosition) >= 10) {
                            $this->alertService->createDropAlert($tracking, $previousPosition, $serpData['position']);
                        }
                    }

                    if ($serpData['position'] === $previousPosition) {
                        $results['unchanged'][] = $tracking->id;
                    } else {
                        $results['success'][] = [
                            'id' => $tracking->id,
                            'keyword' => $tracking->keyword,
                            'old_position' => $previousPosition,
                            'new_position' => $serpData['position'],
                            'change' => $serpData['position'] - ($previousPosition ?? 0),
                        ];
                    }

                } catch (\Exception $e) {
                    Log::error('Failed to check ranking', [
                        'tracking_id' => $tracking->id,
                        'error' => $e->getMessage(),
                    ]);
                    
                    $results['failed'][] = [
                        'id' => $tracking->id,
                        'keyword' => $tracking->keyword,
                        'error' => $e->getMessage(),
                    ];
                }
            }

            DB::commit();

            // Clear caches
            Cache::tags(['rankings'])->flush();

            return response()->json([
                'message' => 'Rankings checked successfully',
                'results' => $results,
                'summary' => [
                    'total_checked' => $trackings->count(),
                    'successful' => count($results['success']),
                    'failed' => count($results['failed']),
                    'unchanged' => count($results['unchanged']),
                ],
            ]);

        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Rank checking failed',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Track a new keyword with validation.
     */
    public function track(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keyword' => 'required|string|max:255',
            'url' => 'required|url|max:500',
            'search_engine' => ['nullable', 'string', Rule::in(array_keys(self::SEARCH_ENGINES))],
            'location' => 'nullable|string|max:10',
            'language' => 'nullable|string|size:2',
            'device' => ['nullable', Rule::in(['desktop', 'mobile', 'tablet'])],
            'tags' => 'nullable|array|max:10',
            'tags.*' => 'string|max:50',
            'competitors' => 'nullable|array|max:10',
            'competitors.*' => 'url',
            'alert_rules' => 'nullable|array',
            'alert_rules.*.type' => ['required', Rule::in(['drop', 'rise', 'out_of_top', 'entered_top'])],
            'alert_rules.*.threshold' => 'required|integer|min:1',
            'alert_rules.*.notify_email' => 'boolean',
            'alert_rules.*.notify_slack' => 'boolean',
            'check_immediately' => 'boolean',
            'track_competitors' => 'boolean',
            'track_serp_features' => 'boolean',
        ]);

        // Check for duplicate tracking
        $exists = RankTracking::where('keyword', $validated['keyword'])
            ->where('url', $validated['url'])
            ->where('search_engine', $validated['search_engine'] ?? 'google')
            ->where('location', $validated['location'] ?? 'us')
            ->where('device', $validated['device'] ?? 'desktop')
            ->exists();

        if ($exists) {
            return response()->json([
                'error' => 'This keyword is already being tracked for the specified parameters',
            ], Response::HTTP_CONFLICT);
        }

        DB::beginTransaction();

        try {
            // Create tracking
            $tracking = RankTracking::create([
                'keyword' => $validated['keyword'],
                'url' => $validated['url'],
                'search_engine' => $validated['search_engine'] ?? 'google',
                'location' => $validated['location'] ?? 'us',
                'language' => $validated['language'] ?? 'en',
                'device' => $validated['device'] ?? 'desktop',
                'tags' => $validated['tags'] ?? [],
                'is_active' => true,
                'track_competitors' => $validated['track_competitors'] ?? false,
                'track_serp_features' => $validated['track_serp_features'] ?? true,
                'current_position' => null,
                'previous_position' => null,
                'best_position' => null,
                'worst_position' => null,
                'average_position' => null,
                'last_checked_at' => null,
            ]);

            // Add competitors if specified
            if (!empty($validated['competitors'])) {
                foreach ($validated['competitors'] as $competitorUrl) {
                    CompetitorDomain::firstOrCreate([
                        'domain' => parse_url($competitorUrl, PHP_URL_HOST),
                        'url' => $competitorUrl,
                    ]);
                    
                    $tracking->competitors()->attach(
                        CompetitorDomain::where('url', $competitorUrl)->first()->id
                    );
                }
            }

            // Set up alert rules
            if (!empty($validated['alert_rules'])) {
                foreach ($validated['alert_rules'] as $rule) {
                    RankAlert::create([
                        'rank_tracking_id' => $tracking->id,
                        'type' => $rule['type'],
                        'threshold' => $rule['threshold'],
                        'notify_email' => $rule['notify_email'] ?? true,
                        'notify_slack' => $rule['notify_slack'] ?? false,
                        'is_active' => true,
                    ]);
                }
            }

            // Check immediately if requested
            if ($validated['check_immediately'] ?? false) {
                CheckRankingsJob::dispatch([$tracking->id])
                    ->onQueue('rank-checking-priority');
            }

            DB::commit();

            return response()->json([
                'tracking' => new RankTrackingResource($tracking),
                'message' => 'Keyword tracking created successfully',
                'next_check' => $validated['check_immediately'] ?? false 
                    ? 'Within 2 minutes' 
                    : 'Within 24 hours',
            ], Response::HTTP_CREATED);

        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Failed to create tracking',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Bulk track multiple keywords.
     */
    public function bulkTrack(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keywords' => 'required|array|min:1|max:' . self::MAX_BULK_KEYWORDS,
            'keywords.*.keyword' => 'required|string|max:255',
            'keywords.*.url' => 'required|url|max:500',
            'default_settings' => 'nullable|array',
            'default_settings.search_engine' => ['nullable', Rule::in(array_keys(self::SEARCH_ENGINES))],
            'default_settings.location' => 'nullable|string|max:10',
            'default_settings.device' => ['nullable', Rule::in(['desktop', 'mobile', 'tablet'])],
            'default_settings.tags' => 'nullable|array|max:10',
            'skip_duplicates' => 'boolean',
            'async' => 'boolean',
        ]);

        if ($validated['async'] ?? false) {
            $jobId = uniqid('bulk_track_', true);
            
            BulkRankCheckJob::dispatch($validated, $jobId)
                ->onQueue('bulk-operations')
                ->delay(now()->addSeconds(5));

            return response()->json([
                'status' => 'queued',
                'job_id' => $jobId,
                'keywords_count' => count($validated['keywords']),
                'status_url' => route('api.rankings.bulk-status', ['jobId' => $jobId]),
            ], Response::HTTP_ACCEPTED);
        }

        $results = [
            'created' => [],
            'skipped' => [],
            'failed' => [],
        ];

        DB::beginTransaction();

        try {
            foreach ($validated['keywords'] as $item) {
                try {
                    $settings = array_merge(
                        $validated['default_settings'] ?? [],
                        $item
                    );

                    // Check for duplicates
                    $exists = RankTracking::where('keyword', $item['keyword'])
                        ->where('url', $item['url'])
                        ->where('search_engine', $settings['search_engine'] ?? 'google')
                        ->exists();

                    if ($exists && ($validated['skip_duplicates'] ?? true)) {
                        $results['skipped'][] = $item['keyword'];
                        continue;
                    }

                    $tracking = RankTracking::create([
                        'keyword' => $item['keyword'],
                        'url' => $item['url'],
                        'search_engine' => $settings['search_engine'] ?? 'google',
                        'location' => $settings['location'] ?? 'us',
                        'device' => $settings['device'] ?? 'desktop',
                        'tags' => $settings['tags'] ?? [],
                        'is_active' => true,
                    ]);

                    $results['created'][] = $tracking->id;

                } catch (\Exception $e) {
                    $results['failed'][] = [
                        'keyword' => $item['keyword'],
                        'error' => $e->getMessage(),
                    ];
                }
            }

            if (empty($results['failed'])) {
                DB::commit();
                
                // Schedule initial check for created trackings
                if (!empty($results['created'])) {
                    CheckRankingsJob::dispatch($results['created'])
                        ->onQueue('rank-checking')
                        ->delay(now()->addMinutes(5));
                }

                return response()->json([
                    'message' => 'Bulk tracking created successfully',
                    'results' => $results,
                ], Response::HTTP_CREATED);
            } else {
                DB::rollback();
                
                return response()->json([
                    'error' => 'Some trackings failed to create',
                    'results' => $results,
                ], Response::HTTP_PARTIAL_CONTENT);
            }

        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Bulk tracking failed',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Update ranking settings.
     */
    public function update(int $id, Request $request): JsonResponse
    {
        $tracking = RankTracking::findOrFail($id);
        
        $validated = $request->validate([
            'is_active' => 'boolean',
            'tags' => 'nullable|array|max:10',
            'tags.*' => 'string|max:50',
            'track_competitors' => 'boolean',
            'track_serp_features' => 'boolean',
            'alert_rules' => 'nullable|array',
            'alert_rules.*.type' => ['required', Rule::in(['drop', 'rise', 'out_of_top', 'entered_top'])],
            'alert_rules.*.threshold' => 'required|integer|min:1',
        ]);

        $tracking->update($validated);

        // Update alert rules if provided
        if (isset($validated['alert_rules'])) {
            $tracking->alerts()->delete();
            
            foreach ($validated['alert_rules'] as $rule) {
                RankAlert::create([
                    'rank_tracking_id' => $tracking->id,
                    'type' => $rule['type'],
                    'threshold' => $rule['threshold'],
                    'is_active' => true,
                ]);
            }
        }

        Cache::tags(['rankings'])->flush();

        return response()->json([
            'tracking' => new RankTrackingResource($tracking),
            'message' => 'Tracking updated successfully',
        ]);
    }

    /**
     * Get ranking history with advanced analytics.
     */
    public function history(int $trackingId, Request $request): JsonResponse
    {
        $tracking = RankTracking::findOrFail($trackingId);
        
        $validated = $request->validate([
            'days' => 'nullable|integer|min:1|max:365',
            'date_from' => 'nullable|date',
            'date_to' => 'nullable|date|after_or_equal:date_from',
            'granularity' => ['nullable', Rule::in(['daily', 'weekly', 'monthly'])],
            'include_competitors' => 'boolean',
            'include_serp_features' => 'boolean',
            'include_analytics' => 'boolean',
        ]);

        $query = $tracking->histories();

        // Apply date filters
        if (!empty($validated['date_from']) && !empty($validated['date_to'])) {
            $query->whereBetween('date', [$validated['date_from'], $validated['date_to']]);
        } elseif (!empty($validated['days'])) {
            $query->where('date', '>=', now()->subDays($validated['days']));
        } else {
            $query->where('date', '>=', now()->subDays(30));
        }

        $query->orderBy('date', 'asc');
        
        $history = $query->get();

        // Apply granularity grouping
        if (!empty($validated['granularity']) && $validated['granularity'] !== 'daily') {
            $history = $this->groupHistoryByGranularity($history, $validated['granularity']);
        }

        // Include competitor data if requested
        $competitorHistory = [];
        if ($validated['include_competitors'] ?? false) {
            $competitorHistory = $this->getCompetitorHistory(
                $tracking,
                $validated['date_from'] ?? now()->subDays($validated['days'] ?? 30),
                $validated['date_to'] ?? now()
            );
        }

        // Calculate analytics if requested
        $analytics = [];
        if ($validated['include_analytics'] ?? false) {
            $analytics = $this->analysisService->analyzeHistory($history);
        }

        return response()->json([
            'tracking' => [
                'id' => $tracking->id,
                'keyword' => $tracking->keyword,
                'url' => $tracking->url,
                'current_position' => $tracking->current_position,
            ],
            'history' => RankHistoryResource::collection($history),
            'competitor_history' => $competitorHistory,
            'analytics' => $analytics,
            'summary' => [
                'best_position' => $history->min('position'),
                'worst_position' => $history->max('position'),
                'average_position' => round($history->avg('position'), 1),
                'volatility' => $this->calculateVolatility($history),
                'trend' => $this->calculateTrend($history),
            ],
        ]);
    }

    /**
     * Compare rankings across competitors.
     */
    public function compareCompetitors(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keyword' => 'required|string|max:255',
            'competitors' => 'required|array|min:2|max:10',
            'competitors.*' => 'url',
            'search_engine' => ['nullable', Rule::in(array_keys(self::SEARCH_ENGINES))],
            'location' => 'nullable|string|max:10',
            'device' => ['nullable', Rule::in(['desktop', 'mobile', 'tablet'])],
            'days' => 'nullable|integer|min:1|max:365',
        ]);

        $competitorData = $this->competitorService->compareRankings(
            $validated['keyword'],
            $validated['competitors'],
            [
                'search_engine' => $validated['search_engine'] ?? 'google',
                'location' => $validated['location'] ?? 'us',
                'device' => $validated['device'] ?? 'desktop',
                'days' => $validated['days'] ?? 30,
            ]
        );

        return response()->json([
            'keyword' => $validated['keyword'],
            'comparison' => CompetitorRankingResource::collection($competitorData),
            'winner' => $this->determineWinner($competitorData),
            'insights' => $this->generateCompetitorInsights($competitorData),
        ]);
    }

    /**
     * Export rankings data.
     */
    public function export(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'format' => ['required', Rule::in(['csv', 'json', 'xlsx'])],
            'include_history' => 'boolean',
            'days' => 'nullable|integer|min:1|max:365',
            'tracking_ids' => 'nullable|array',
            'tracking_ids.*' => 'integer|exists:rank_trackings,id',
        ]);

        $query = RankTracking::query();
        
        if (!empty($validated['tracking_ids'])) {
            $query->whereIn('id', $validated['tracking_ids']);
        }

        $trackings = $query->get();

        if ($validated['include_history'] ?? false) {
            $trackings->load(['histories' => function ($q) use ($validated) {
                $q->where('date', '>=', now()->subDays($validated['days'] ?? 30))
                  ->orderBy('date', 'desc');
            }]);
        }

        $exportData = $this->formatExportData($trackings, $validated['format']);

        return response()->json($exportData)
            ->header('Content-Disposition', 'attachment; filename=rankings-export.' . $validated['format']);
    }

    /**
     * Delete a rank tracking.
     */
    public function destroy(int $id, Request $request): JsonResponse
    {
        $tracking = RankTracking::findOrFail($id);
        
        $validated = $request->validate([
            'delete_history' => 'boolean',
        ]);

        DB::beginTransaction();

        try {
            if ($validated['delete_history'] ?? false) {
                $tracking->histories()->delete();
                $tracking->alerts()->delete();
            }

            $tracking->delete();
            
            DB::commit();
            
            Cache::tags(['rankings'])->flush();

            return response()->json([
                'message' => 'Rank tracking deleted successfully',
            ]);

        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Failed to delete tracking',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Get ranking insights and recommendations.
     */
    public function insights(int $trackingId): JsonResponse
    {
        $tracking = RankTracking::with(['histories' => function ($q) {
            $q->where('date', '>=', now()->subDays(90))
              ->orderBy('date', 'desc');
        }])->findOrFail($trackingId);

        $insights = $this->analysisService->generateInsights($tracking);

        return response()->json([
            'tracking' => new RankTrackingResource($tracking),
            'insights' => $insights,
            'recommendations' => $this->generateRecommendations($tracking, $insights),
        ]);
    }

    /**
     * Get SERP features analysis.
     */
    public function serpFeatures(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keyword' => 'required|string|max:255',
            'search_engine' => ['nullable', Rule::in(array_keys(self::SEARCH_ENGINES))],
            'location' => 'nullable|string|max:10',
        ]);

        $features = $this->serpApi->analyzeSerpFeatures(
            $validated['keyword'],
            [
                'search_engine' => $validated['search_engine'] ?? 'google',
                'location' => $validated['location'] ?? 'us',
            ]
        );

        return response()->json([
            'keyword' => $validated['keyword'],
            'features' => $features,
            'opportunities' => $this->identifyFeatureOpportunities($features),
        ]);
    }

    // Helper methods

    private function getAnalytics($rankings): array
    {
        $collection = collect($rankings);
        
        return [
            'total' => $collection->count(),
            'average_position' => round($collection->avg('current_position'), 1),
            'top_10' => $collection->where('current_position', '<=', 10)->count(),
            'top_30' => $collection->where('current_position', '<=', 30)->count(),
            'improved' => $collection->filter(fn($r) => $r->current_position < $r->previous_position)->count(),
            'dropped' => $collection->filter(fn($r) => $r->current_position > $r->previous_position)->count(),
            'unchanged' => $collection->filter(fn($r) => $r->current_position === $r->previous_position)->count(),
            'not_found' => $collection->whereNull('current_position')->count(),
        ];
    }

    private function groupResults($rankings, string $groupBy): array
    {
        return collect($rankings)
            ->groupBy($groupBy)
            ->map(function ($group) {
                return [
                    'count' => $group->count(),
                    'average_position' => round($group->avg('current_position'), 1),
                    'best' => $group->min('current_position'),
                    'worst' => $group->max('current_position'),
                    'items' => RankTrackingResource::collection($group),
                ];
            })
            ->toArray();
    }

    private function groupHistoryByGranularity($history, string $granularity): Collection
    {
        return $history->groupBy(function ($item) use ($granularity) {
            $date = Carbon::parse($item->date);
            
            return match($granularity) {
                'weekly' => $date->startOfWeek()->toDateString(),
                'monthly' => $date->startOfMonth()->toDateString(),
                default => $item->date,
            };
        })->map(function ($group) {
            return [
                'date' => $group->first()->date,
                'position' => round($group->avg('position')),
                'min_position' => $group->min('position'),
                'max_position' => $group->max('position'),
                'search_volume' => $group->avg('search_volume'),
            ];
        });
    }

    private function getCompetitorHistory($tracking, $dateFrom, $dateTo): array
    {
        $competitors = $tracking->competitors;
        $history = [];

        foreach ($competitors as $competitor) {
            $competitorTracking = RankTracking::where('keyword', $tracking->keyword)
                ->where('url', 'like', '%' . $competitor->domain . '%')
                ->first();

            if ($competitorTracking) {
                $history[$competitor->domain] = $competitorTracking->histories()
                    ->whereBetween('date', [$dateFrom, $dateTo])
                    ->orderBy('date', 'asc')
                    ->get();
            }
        }

        return $history;
    }

    private function calculateVolatility($history): float
    {
        if ($history->count() < 2) {
            return 0;
        }

        $positions = $history->pluck('position')->toArray();
        $changes = [];
        
        for ($i = 1; $i < count($positions); $i++) {
            $changes[] = abs($positions[$i] - $positions[$i - 1]);
        }

        return round(array_sum($changes) / count($changes), 2);
    }

    private function calculateTrend($history): string
    {
        if ($history->count() < 2) {
            return 'stable';
        }

        $recentPositions = $history->sortByDesc('date')->take(7)->pluck('position');
        $olderPositions = $history->sortByDesc('date')->skip(7)->take(7)->pluck('position');

        $recentAvg = $recentPositions->avg();
        $olderAvg = $olderPositions->avg();

        if ($recentAvg < $olderAvg - 2) {
            return 'improving';
        } elseif ($recentAvg > $olderAvg + 2) {
            return 'declining';
        }

        return 'stable';
    }

    private function determineWinner($competitorData): ?array
    {
        $best = collect($competitorData)->sortBy('current_position')->first();
        
        return $best ? [
            'domain' => $best['domain'],
            'position' => $best['current_position'],
        ] : null;
    }

    private function generateCompetitorInsights($competitorData): array
    {
        $insights = [];
        
        // Analyze position gaps
        $positions = collect($competitorData)->pluck('current_position')->sort();
        if ($positions->count() > 1) {
            $gap = $positions->last() - $positions->first();
            $insights[] = "Position gap between competitors: {$gap} positions";
        }

        // Identify trends
        foreach ($competitorData as $competitor) {
            if (isset($competitor['trend'])) {
                $insights[] = "{$competitor['domain']} is {$competitor['trend']}";
            }
        }

        return $insights;
    }

    private function generateRecommendations($tracking, $insights): array
    {
        $recommendations = [];

        if ($tracking->current_position > 10) {
            $recommendations[] = [
                'type' => 'optimization',
                'priority' => 'high',
                'action' => 'Improve on-page SEO for target keyword',
                'estimated_impact' => 'Move to first page',
            ];
        }

        if ($insights['volatility'] > 5) {
            $recommendations[] = [
                'type' => 'stability',
                'priority' => 'medium',
                'action' => 'Investigate ranking volatility causes',
                'estimated_impact' => 'Stabilize rankings',
            ];
        }

        if ($insights['competitor_gap'] > 5) {
            $recommendations[] = [
                'type' => 'competitive',
                'priority' => 'high',
                'action' => 'Analyze competitor content and backlinks',
                'estimated_impact' => 'Close competitive gap',
            ];
        }

        return $recommendations;
    }

    private function identifyFeatureOpportunities($features): array
    {
        $opportunities = [];

        if (isset($features['featured_snippet']) && !$features['featured_snippet']['owned']) {
            $opportunities[] = [
                'feature' => 'Featured Snippet',
                'action' => 'Optimize content for snippet capture',
                'difficulty' => 'medium',
            ];
        }

        if (isset($features['people_also_ask'])) {
            $opportunities[] = [
                'feature' => 'People Also Ask',
                'action' => 'Create FAQ content addressing these questions',
                'difficulty' => 'low',
            ];
        }

        return $opportunities;
    }

    private function formatExportData($trackings, string $format): array
    {
        $data = [];
        
        foreach ($trackings as $tracking) {
            $row = [
                'keyword' => $tracking->keyword,
                'url' => $tracking->url,
                'current_position' => $tracking->current_position,
                'previous_position' => $tracking->previous_position,
                'change' => $tracking->current_position - $tracking->previous_position,
                'search_engine' => $tracking->search_engine,
                'location' => $tracking->location,
                'device' => $tracking->device,
                'last_checked' => $tracking->last_checked_at,
            ];

            if ($tracking->relationLoaded('histories')) {
                $row['history'] = $tracking->histories->map(function ($h) {
                    return [
                        'date' => $h->date,
                        'position' => $h->position,
                    ];
                })->toArray();
            }

            $data[] = $row;
        }

        return $format === 'csv' ? $this->convertToCsv($data) : $data;
    }

    private function convertToCsv($data): string
    {
        $csv = '';
        
        if (!empty($data)) {
            // Headers
            $headers = array_keys($data[0]);
            $csv .= implode(',', $headers) . "\n";
            
            // Data rows
            foreach ($data as $row) {
                $values = array_map(function ($value) {
                    return is_array($value) ? json_encode($value) : $value;
                }, array_values($row));
                $csv .= implode(',', $values) . "\n";
            }
        }

        return $csv;
    }

    private function getNextScheduledCheck(): ?string
    {
        $nextJob = DB::table('jobs')
            ->where('queue', 'rank-checking')
            ->orderBy('available_at')
            ->first();

        return $nextJob 
            ? Carbon::createFromTimestamp($nextJob->available_at)->toDateTimeString()
            : 'No checks scheduled';
    }
}