<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\Seo\NlpIntelligenceService;
use App\Services\Seo\AiContentOptimizerService;
use App\Services\Seo\KeywordIntelligenceService;
use App\Services\Seo\InternalLinkIntelligenceService;
use App\Services\Seo\SchemaIntelligenceService;
use App\Services\Seo\SeoCacheService;
use App\Models\SeoEntity;
use App\Models\SeoKeyword;
use App\Models\SeoAiSuggestion;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;

/**
 * SEO Intelligence API Controller
 * 
 * Google L8 Enterprise-Grade SEO Intelligence API
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class SeoIntelligenceController extends Controller
{
    public function __construct(
        private NlpIntelligenceService $nlp,
        private AiContentOptimizerService $aiOptimizer,
        private KeywordIntelligenceService $keywords,
        private InternalLinkIntelligenceService $links,
        private SchemaIntelligenceService $schema,
        private SeoCacheService $cache
    ) {}

    /**
     * Analyze content with NLP.
     */
    public function analyzeContent(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => 'required|string',
            'content_id' => 'required|integer',
            'text' => 'required|string',
            'html' => 'nullable|string',
        ]);

        try {
            $analysis = $this->nlp->analyze(
                $validated['content_type'],
                $validated['content_id'],
                $validated['text'],
                $validated['html'] ?? null
            );

            return response()->json([
                'success' => true,
                'data' => $analysis,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get entity coverage for content.
     */
    public function getEntityCoverage(string $contentType, int $contentId): JsonResponse
    {
        try {
            $coverage = \DB::table('seo_entity_coverage')
                ->where('content_type', $contentType)
                ->where('content_id', $contentId)
                ->first();

            if (!$coverage) {
                return response()->json([
                    'success' => false,
                    'error' => 'No entity coverage analysis found',
                ], 404);
            }

            return response()->json([
                'success' => true,
                'data' => [
                    'coverage_score' => $coverage->coverage_score,
                    'expected_entities' => json_decode($coverage->expected_entities, true),
                    'found_entities' => json_decode($coverage->found_entities, true),
                    'missing_entities' => json_decode($coverage->missing_entities, true),
                    'analyzed_at' => $coverage->analyzed_at,
                ],
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Generate AI title suggestions.
     */
    public function generateTitles(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => 'required|string',
            'content_id' => 'required|integer',
            'current_title' => 'required|string',
            'focus_keyword' => 'required|string',
            'content_summary' => 'required|string',
            'count' => 'nullable|integer|min:1|max:20',
        ]);

        try {
            $suggestions = $this->aiOptimizer->generateTitleSuggestions(
                $validated['content_type'],
                $validated['content_id'],
                $validated['current_title'],
                $validated['focus_keyword'],
                $validated['content_summary'],
                $validated['count'] ?? 10
            );

            return response()->json([
                'success' => true,
                'data' => $suggestions,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Generate AI meta description.
     */
    public function generateMeta(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => 'required|string',
            'content_id' => 'required|integer',
            'current_meta' => 'nullable|string',
            'focus_keyword' => 'required|string',
            'content_summary' => 'required|string',
            'count' => 'nullable|integer|min:1|max:10',
        ]);

        try {
            $suggestions = $this->aiOptimizer->generateMetaDescription(
                $validated['content_type'],
                $validated['content_id'],
                $validated['current_meta'] ?? '',
                $validated['focus_keyword'],
                $validated['content_summary'],
                $validated['count'] ?? 5
            );

            return response()->json([
                'success' => true,
                'data' => $suggestions,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Generate content outline.
     */
    public function generateOutline(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'topic' => 'required|string',
            'focus_keyword' => 'required|string',
            'secondary_keywords' => 'nullable|array',
            'target_audience' => 'nullable|string',
            'target_word_count' => 'nullable|integer',
        ]);

        try {
            $outline = $this->aiOptimizer->generateOutline(
                $validated['topic'],
                $validated['focus_keyword'],
                $validated['secondary_keywords'] ?? [],
                $validated['target_audience'] ?? 'general',
                $validated['target_word_count'] ?? 1500
            );

            return response()->json([
                'success' => true,
                'data' => $outline,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get AI suggestions for content.
     */
    public function getSuggestions(string $contentType, int $contentId): JsonResponse
    {
        try {
            $suggestions = SeoAiSuggestion::forContent($contentType, $contentId)
                ->pending()
                ->orderByDesc('impact_score')
                ->get();

            return response()->json([
                'success' => true,
                'data' => $suggestions,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Accept AI suggestion.
     */
    public function acceptSuggestion(int $suggestionId): JsonResponse
    {
        try {
            $suggestion = SeoAiSuggestion::findOrFail($suggestionId);
            $suggestion->accept();

            return response()->json([
                'success' => true,
                'message' => 'Suggestion accepted',
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Research keywords.
     */
    public function researchKeywords(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'seed_keyword' => 'required|string',
            'limit' => 'nullable|integer|min:1|max:500',
        ]);

        try {
            $keywords = $this->keywords->researchKeywords(
                $validated['seed_keyword'],
                $validated['limit'] ?? 100
            );

            return response()->json([
                'success' => true,
                'data' => $keywords,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Analyze SERP for keyword.
     */
    public function analyzeSERP(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keyword' => 'required|string',
            'force_refresh' => 'nullable|boolean',
        ]);

        try {
            $results = $this->keywords->analyzeSERP(
                $validated['keyword'],
                $validated['force_refresh'] ?? false
            );

            return response()->json([
                'success' => true,
                'data' => $results,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get keyword clusters.
     */
    public function getKeywordClusters(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'keywords' => 'required|array',
        ]);

        try {
            $clusters = $this->keywords->createClusters($validated['keywords']);

            return response()->json([
                'success' => true,
                'data' => $clusters,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get internal link suggestions.
     */
    public function getLinkSuggestions(string $contentType, int $contentId): JsonResponse
    {
        try {
            // Get content
            $content = \DB::table('posts')->where('id', $contentId)->first();
            
            if (!$content) {
                return response()->json([
                    'success' => false,
                    'error' => 'Content not found',
                ], 404);
            }

            $suggestions = $this->links->generateSuggestions(
                $contentType,
                $contentId,
                $content->content ?? '',
                10
            );

            return response()->json([
                'success' => true,
                'data' => $suggestions,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get link graph.
     */
    public function getLinkGraph(): JsonResponse
    {
        try {
            $graph = $this->links->buildLinkGraph();

            return response()->json([
                'success' => true,
                'data' => $graph,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get orphan pages.
     */
    public function getOrphans(): JsonResponse
    {
        try {
            $orphans = $this->links->detectOrphans();

            return response()->json([
                'success' => true,
                'data' => $orphans,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Generate schema for content.
     */
    public function generateSchema(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content_type' => 'required|string',
            'content_id' => 'required|integer',
            'content' => 'required|array',
            'entities' => 'nullable|array',
        ]);

        try {
            $schemas = $this->schema->generateSchema(
                $validated['content_type'],
                $validated['content_id'],
                $validated['content'],
                $validated['entities'] ?? []
            );

            return response()->json([
                'success' => true,
                'data' => $schemas,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get cache statistics.
     */
    public function getCacheStats(): JsonResponse
    {
        try {
            $stats = $this->cache->getStats();

            return response()->json([
                'success' => true,
                'data' => $stats,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Warm cache.
     */
    public function warmCache(): JsonResponse
    {
        try {
            $result = $this->cache->warmCache();

            return response()->json([
                'success' => true,
                'data' => $result,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Monitor cache health.
     */
    public function monitorCache(): JsonResponse
    {
        try {
            $health = $this->cache->monitorHealth();

            return response()->json([
                'success' => true,
                'data' => $health,
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }
}
