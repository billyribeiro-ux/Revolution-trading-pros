<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Http\Resources\SeoAnalysisResource;
use App\Jobs\AnalyzeSeoJob;
use App\Jobs\GenerateSitemapJob;
use App\Services\Seo\SeoAnalyzerService;
use App\Services\Seo\CompetitorAnalysisService;
use App\Services\Seo\SchemaGeneratorService;
use App\Services\Seo\PerformanceAnalyzerService;
use App\Services\Seo\ContentGapAnalyzerService;
use App\Services\Seo\KeywordResearchService;
use App\Services\Seo\TechnicalAuditService;
use App\Services\Seo\AutoFixService;
use App\Models\SeoAnalysis;
use App\Enums\SeoAnalysisStatus;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\RateLimiter;
use Illuminate\Validation\Rule;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Symfony\Component\HttpFoundation\Response;

class SeoController extends Controller
{
    private const CACHE_TTL = 3600; // 1 hour
    private const RATE_LIMIT_KEY = 'seo-analysis';
    
    public function __construct(
        private readonly SeoAnalyzerService $seoAnalyzer,
        private readonly CompetitorAnalysisService $competitorAnalyzer,
        private readonly SchemaGeneratorService $schemaGenerator,
        private readonly PerformanceAnalyzerService $performanceAnalyzer,
        private readonly ContentGapAnalyzerService $contentGapAnalyzer,
        private readonly KeywordResearchService $keywordResearch,
        private readonly TechnicalAuditService $technicalAudit,
        private readonly AutoFixService $autoFix
    ) {}

    /**
     * Analyze content for SEO with async processing support.
     */
    public function analyze(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => ['required', 'string', Rule::in(['post', 'page', 'product'])],
            'content_id' => 'required|integer|min:1',
            'focus_keyword' => 'nullable|string|max:255',
            'secondary_keywords' => 'nullable|array|max:10',
            'secondary_keywords.*' => 'string|max:255',
            'async' => 'boolean',
            'force_refresh' => 'boolean',
            'depth' => ['nullable', 'string', Rule::in(['quick', 'standard', 'comprehensive'])],
        ]);

        // Rate limiting for analysis requests
        $rateLimitKey = self::RATE_LIMIT_KEY . ':' . $request->user()->id;
        if (!RateLimiter::attempt($rateLimitKey, 30, fn() => null, 60)) {
            return response()->json([
                'error' => 'Too many analysis requests. Please try again later.',
                'retry_after' => RateLimiter::availableIn($rateLimitKey)
            ], Response::HTTP_TOO_MANY_REQUESTS);
        }

        try {
            // Check for existing recent analysis if not forcing refresh
            if (!($validated['force_refresh'] ?? false)) {
                $existingAnalysis = $this->getCachedAnalysis(
                    $validated['content_type'],
                    $validated['content_id']
                );
                
                if ($existingAnalysis && $existingAnalysis->isRecent()) {
                    return response()->json([
                        'analysis' => new SeoAnalysisResource($existingAnalysis),
                        'cached' => true
                    ]);
                }
            }

            // Async processing for comprehensive analysis
            if ($validated['async'] ?? false) {
                $analysis = SeoAnalysis::create([
                    'analyzable_type' => $validated['content_type'],
                    'analyzable_id' => $validated['content_id'],
                    'focus_keyword' => $validated['focus_keyword'] ?? null,
                    'secondary_keywords' => $validated['secondary_keywords'] ?? [],
                    'status' => SeoAnalysisStatus::PENDING,
                    'depth' => $validated['depth'] ?? 'standard',
                ]);

                AnalyzeSeoJob::dispatch($analysis)
                    ->onQueue('seo-analysis')
                    ->delay(now()->addSeconds(2));

                return response()->json([
                    'analysis_id' => $analysis->id,
                    'status' => 'pending',
                    'webhook_url' => route('api.seo.analysis-status', $analysis->id),
                    'estimated_completion' => now()->addMinutes(2),
                ], Response::HTTP_ACCEPTED);
            }

            // Synchronous analysis for quick results
            $analysis = $this->seoAnalyzer->analyze(
                $validated['content_type'],
                $validated['content_id'],
                $validated['focus_keyword'] ?? null,
                $validated['secondary_keywords'] ?? [],
                $validated['depth'] ?? 'standard'
            );

            // Cache the analysis
            $this->cacheAnalysis($analysis);

            return response()->json([
                'analysis' => new SeoAnalysisResource($analysis),
                'cached' => false
            ]);

        } catch (ModelNotFoundException $e) {
            return response()->json([
                'error' => 'Content not found',
                'message' => 'The specified content could not be found.'
            ], Response::HTTP_NOT_FOUND);
        } catch (\Exception $e) {
            report($e);
            return response()->json([
                'error' => 'Analysis failed',
                'message' => 'An error occurred during SEO analysis. Please try again.'
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Get analysis status for async requests.
     */
    public function getAnalysisStatus(int $analysisId): JsonResponse
    {
        $analysis = SeoAnalysis::findOrFail($analysisId);
        
        // Verify ownership
        $this->authorize('view', $analysis);

        return response()->json([
            'id' => $analysis->id,
            'status' => $analysis->status->value,
            'progress' => $analysis->progress ?? 0,
            'completed_at' => $analysis->completed_at,
            'analysis' => $analysis->status === SeoAnalysisStatus::COMPLETED 
                ? new SeoAnalysisResource($analysis) 
                : null,
        ]);
    }

    /**
     * Get cached analysis with proper error handling.
     */
    public function getAnalysis(string $contentType, int $contentId): JsonResponse
    {
        try {
            $analysis = $this->getCachedAnalysis($contentType, $contentId);

            if (!$analysis) {
                return response()->json([
                    'message' => 'No analysis found. Run an analysis first.',
                    'analysis' => $this->getEmptyAnalysisStructure(),
                ], Response::HTTP_NOT_FOUND);
            }

            return response()->json([
                'analysis' => new SeoAnalysisResource($analysis),
                'age_minutes' => $analysis->created_at->diffInMinutes(now()),
                'is_stale' => !$analysis->isRecent(),
            ]);

        } catch (\Exception $e) {
            report($e);
            return response()->json([
                'error' => 'Failed to retrieve analysis',
                'analysis' => $this->getEmptyAnalysisStructure(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Get actionable SEO recommendations with priority scoring.
     */
    public function getRecommendations(string $contentType, int $contentId): JsonResponse
    {
        $cacheKey = "seo:recommendations:{$contentType}:{$contentId}";
        
        $recommendations = Cache::remember($cacheKey, self::CACHE_TTL, function () use ($contentType, $contentId) {
            $analysis = SeoAnalysis::where('analyzable_type', $contentType)
                ->where('analyzable_id', $contentId)
                ->latest()
                ->first();

            if (!$analysis) {
                return [];
            }

            return $this->seoAnalyzer->generatePrioritizedRecommendations($analysis);
        });

        return response()->json([
            'recommendations' => $recommendations,
            'total_estimated_impact' => collect($recommendations)->sum('estimated_impact'),
            'implementation_time_hours' => collect($recommendations)->sum('effort_hours'),
        ]);
    }

    /**
     * Auto-fix SEO issues with rollback support.
     */
    public function autoFix(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => ['required', 'string', Rule::in(['post', 'page', 'product'])],
            'content_id' => 'required|integer|min:1',
            'issues' => 'required|array|min:1|max:20',
            'issues.*' => ['string', Rule::in($this->autoFix->getFixableIssues())],
            'create_backup' => 'boolean',
            'dry_run' => 'boolean',
        ]);

        try {
            $result = $this->autoFix->fix(
                $validated['content_type'],
                $validated['content_id'],
                $validated['issues'],
                $validated['create_backup'] ?? true,
                $validated['dry_run'] ?? false
            );

            // Invalidate caches after fixes
            if (!($validated['dry_run'] ?? false) && count($result['fixed']) > 0) {
                $this->invalidateAnalysisCache($validated['content_type'], $validated['content_id']);
            }

            return response()->json($result);

        } catch (\Exception $e) {
            report($e);
            return response()->json([
                'error' => 'Auto-fix failed',
                'message' => $e->getMessage(),
                'fixed' => [],
                'failed' => $validated['issues'],
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Analyze competitors with SERP data.
     */
    public function analyzeCompetitors(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keyword' => 'required|string|max:255',
            'url' => 'nullable|url|max:500',
            'competitors' => 'nullable|array|max:10',
            'competitors.*' => 'url',
            'location' => 'nullable|string|max:100',
            'language' => 'nullable|string|size:2',
            'include_serp' => 'boolean',
        ]);

        $cacheKey = 'competitor:' . md5(json_encode($validated));
        
        $analysis = Cache::remember($cacheKey, self::CACHE_TTL * 2, function () use ($validated) {
            return $this->competitorAnalyzer->analyze(
                $validated['keyword'],
                $validated['url'] ?? null,
                $validated['competitors'] ?? [],
                [
                    'location' => $validated['location'] ?? 'US',
                    'language' => $validated['language'] ?? 'en',
                    'include_serp' => $validated['include_serp'] ?? true,
                ]
            );
        });

        return response()->json(['analysis' => $analysis]);
    }

    /**
     * Find keyword opportunities using real data sources.
     */
    public function keywordOpportunities(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'seed_keywords' => 'required|array|min:1|max:20',
            'seed_keywords.*' => 'string|max:255',
            'domain' => 'nullable|string|max:255',
            'min_volume' => 'nullable|integer|min:0',
            'max_difficulty' => 'nullable|integer|min:0|max:100',
            'intent' => 'nullable|array',
            'intent.*' => Rule::in(['informational', 'navigational', 'commercial', 'transactional']),
            'include_questions' => 'boolean',
            'include_long_tail' => 'boolean',
        ]);

        try {
            $opportunities = $this->keywordResearch->findOpportunities($validated);

            return response()->json([
                'opportunities' => $opportunities,
                'total_found' => count($opportunities),
                'filters_applied' => array_keys(array_filter($validated, fn($v) => !is_null($v))),
            ]);

        } catch (\Exception $e) {
            report($e);
            return response()->json([
                'error' => 'Keyword research failed',
                'opportunities' => [],
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Analyze content gaps with actionable insights.
     */
    public function contentGaps(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keyword' => 'required|string|max:255',
            'url' => 'required|url|max:500',
            'competitors' => 'nullable|array|max:10',
            'competitors.*' => 'url',
            'content_types' => 'nullable|array',
            'content_types.*' => Rule::in(['blog', 'video', 'infographic', 'tool', 'calculator']),
        ]);

        $gaps = $this->contentGapAnalyzer->analyze(
            $validated['keyword'],
            $validated['url'],
            $validated['competitors'] ?? [],
            $validated['content_types'] ?? []
        );

        return response()->json([
            'gaps' => $gaps,
            'priority_topics' => collect($gaps)->sortByDesc('opportunity_score')->take(10)->values(),
        ]);
    }

    /**
     * Perform comprehensive technical SEO audit.
     */
    public function technicalAudit(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => ['required', 'string', Rule::in(['post', 'page', 'product'])],
            'content_id' => 'required|integer|min:1',
            'checks' => 'nullable|array',
            'checks.*' => Rule::in($this->technicalAudit->getAvailableChecks()),
        ]);

        $audit = $this->technicalAudit->audit(
            $validated['content_type'],
            $validated['content_id'],
            $validated['checks'] ?? null
        );

        return response()->json([
            'audit' => $audit,
            'critical_issues' => collect($audit['issues'])->where('severity', 'critical')->values(),
            'total_issues' => count($audit['issues']),
        ]);
    }

    /**
     * Analyze page performance with Core Web Vitals.
     */
    public function performanceAnalyze(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => ['required', 'string', Rule::in(['post', 'page', 'product'])],
            'content_id' => 'required|integer|min:1',
            'device' => ['nullable', 'string', Rule::in(['mobile', 'desktop', 'both'])],
            'throttling' => ['nullable', 'string', Rule::in(['3g', '4g', 'wifi'])],
        ]);

        $device = $validated['device'] ?? 'both';
        $cacheKey = "performance:{$validated['content_type']}:{$validated['content_id']}:{$device}";

        
        $performance = Cache::remember($cacheKey, 600, function () use ($validated) {
            return $this->performanceAnalyzer->analyze(
                $validated['content_type'],
                $validated['content_id'],
                [
                    'device' => $validated['device'] ?? 'both',
                    'throttling' => $validated['throttling'] ?? '4g',
                ]
            );
        });

        return response()->json($performance);
    }

    /**
     * Get all technical issues across the site.
     */
    public function technicalIssues(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'severity' => ['nullable', 'string', Rule::in(['critical', 'high', 'medium', 'low'])],
            'category' => ['nullable', 'string'],
            'limit' => 'nullable|integer|min:1|max:100',
            'offset' => 'nullable|integer|min:0',
        ]);

        $issues = $this->technicalAudit->getSiteWideIssues(
            $validated['severity'] ?? null,
            $validated['category'] ?? null,
            $validated['limit'] ?? 50,
            $validated['offset'] ?? 0
        );

        return response()->json($issues);
    }

    /**
     * Generate XML sitemap with proper formatting.
     */
    public function generateSitemap(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'types' => 'nullable|array',
            'types.*' => Rule::in(['posts', 'pages', 'products', 'categories', 'tags']),
            'include_images' => 'boolean',
            'include_videos' => 'boolean',
            'format' => ['nullable', 'string', Rule::in(['xml', 'txt', 'html'])],
            'async' => 'boolean',
        ]);

        if ($validated['async'] ?? false) {
            $jobId = GenerateSitemapJob::dispatch($validated)
                ->onQueue('sitemaps')
                ->delay(now()->addSeconds(5));

            return response()->json([
                'status' => 'generating',
                'job_id' => $jobId,
                'check_url' => route('api.seo.sitemap-status', ['jobId' => $jobId]),
            ], Response::HTTP_ACCEPTED);
        }

        $sitemap = $this->schemaGenerator->generateSitemap($validated);

        return response()->json([
            'url' => $sitemap['url'],
            'entries' => $sitemap['entries'],
            'size_bytes' => $sitemap['size'],
            'last_generated' => now()->toIso8601String(),
        ]);
    }

    /**
     * Generate structured data with validation.
     */
    public function generateSchema(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'type' => ['required', 'string', Rule::in($this->schemaGenerator->getSupportedTypes())],
            'data' => 'required|array',
            'validate' => 'boolean',
            'format' => ['nullable', 'string', Rule::in(['json-ld', 'microdata', 'rdfa'])],
        ]);

        try {
            $schema = $this->schemaGenerator->generate(
                $validated['type'],
                $validated['data'],
                $validated['format'] ?? 'json-ld'
            );

            if ($validated['validate'] ?? true) {
                $validation = $this->schemaGenerator->validate($schema);
                
                if (!$validation['valid']) {
                    return response()->json([
                        'error' => 'Schema validation failed',
                        'errors' => $validation['errors'],
                        'schema' => $schema,
                    ], Response::HTTP_UNPROCESSABLE_ENTITY);
                }
            }

            return response()->json([
                'schema' => $schema,
                'type' => $validated['type'],
                'format' => $validated['format'] ?? 'json-ld',
                'valid' => true,
            ]);

        } catch (\Exception $e) {
            report($e);
            return response()->json([
                'error' => 'Schema generation failed',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Batch analyze multiple pieces of content.
     */
    public function batchAnalyze(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'items' => 'required|array|min:1|max:50',
            'items.*.content_type' => ['required', 'string', Rule::in(['post', 'page', 'product'])],
            'items.*.content_id' => 'required|integer|min:1',
            'items.*.focus_keyword' => 'nullable|string|max:255',
            'async' => 'boolean',
        ]);

        $batchId = uniqid('batch_', true);
        
        foreach ($validated['items'] as $item) {
            AnalyzeSeoJob::dispatch(
                $item['content_type'],
                $item['content_id'],
                $item['focus_keyword'] ?? null,
                $batchId
            )->onQueue('seo-batch-analysis');
        }

        return response()->json([
            'batch_id' => $batchId,
            'total_items' => count($validated['items']),
            'status_url' => route('api.seo.batch-status', ['batchId' => $batchId]),
        ], Response::HTTP_ACCEPTED);
    }

    /**
     * Helper method to get cached analysis.
     */
    private function getCachedAnalysis(string $contentType, int $contentId): ?SeoAnalysis
    {
        $cacheKey = "seo:analysis:{$contentType}:{$contentId}";
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($contentType, $contentId) {
            return SeoAnalysis::where('analyzable_type', $contentType)
                ->where('analyzable_id', $contentId)
                ->where('status', SeoAnalysisStatus::COMPLETED)
                ->latest()
                ->first();
        });
    }

    /**
     * Cache analysis results.
     */
    private function cacheAnalysis(SeoAnalysis $analysis): void
    {
        $cacheKey = "seo:analysis:{$analysis->analyzable_type}:{$analysis->analyzable_id}";
        Cache::put($cacheKey, $analysis, self::CACHE_TTL);
    }

    /**
     * Invalidate analysis cache.
     */
    private function invalidateAnalysisCache(string $contentType, int $contentId): void
    {
        $patterns = [
            "seo:analysis:{$contentType}:{$contentId}",
            "seo:recommendations:{$contentType}:{$contentId}",
            "performance:{$contentType}:{$contentId}:*",
        ];

        foreach ($patterns as $pattern) {
            Cache::forget($pattern);
        }
    }

    /**
     * Get empty analysis structure for consistency.
     */
    private function getEmptyAnalysisStructure(): array
    {
        return [
            'overall_score' => 0,
            'technical_score' => 0,
            'content_score' => 0,
            'user_experience_score' => 0,
            'suggestions' => [],
            'issues' => [],
            'opportunities' => [],
        ];
    }
}