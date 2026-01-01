<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\Seo\BingSeoService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Validator;

/**
 * BingSeoController - Bing Search Engine Optimization API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Complete Bing SEO management including:
 * - IndexNow instant URL submission
 * - Bing Webmaster Tools integration
 * - Meta tag generation
 * - Performance analytics
 * - Sitemap submission
 *
 * @version 1.0.0
 */
class BingSeoController extends Controller
{
    public function __construct(
        private readonly BingSeoService $bingSeoService
    ) {}

    /**
     * Submit single URL to IndexNow for instant Bing indexing
     */
    public function submitUrl(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'url' => 'required|url|max:2048',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $result = $this->bingSeoService->submitToIndexNow($request->get('url'));

        return response()->json([
            'success' => $result['success'],
            'data' => $result,
        ], $result['success'] ? 200 : 400);
    }

    /**
     * Batch submit multiple URLs to IndexNow
     */
    public function batchSubmitUrls(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'urls' => 'required|array|min:1|max:10000',
            'urls.*' => 'required|url|max:2048',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $result = $this->bingSeoService->batchSubmitToIndexNow($request->get('urls'));

        return response()->json([
            'success' => $result['success'],
            'data' => $result,
        ], $result['success'] ? 200 : 400);
    }

    /**
     * Submit URL via Bing Webmaster API
     */
    public function submitToWebmaster(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'url' => 'required|url|max:2048',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $result = $this->bingSeoService->submitToBingWebmaster($request->get('url'));

        return response()->json([
            'success' => $result['success'],
            'data' => $result,
        ], $result['success'] ? 200 : 400);
    }

    /**
     * Submit sitemap to Bing
     */
    public function submitSitemap(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'sitemap_url' => 'required|url|max:2048',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $result = $this->bingSeoService->submitSitemap($request->get('sitemap_url'));

        return response()->json([
            'success' => $result['success'],
            'data' => $result,
        ]);
    }

    /**
     * Get Bing meta tags for a page
     */
    public function getMetaTags(Request $request): JsonResponse
    {
        $options = $request->only([
            'no_archive',
            'no_snippet',
            'max_snippet',
            'max_image_preview',
            'max_video_preview',
            'msnbot',
            'locale',
            'geo',
        ]);

        $tags = $this->bingSeoService->generateBingMetaTags($options);

        return response()->json([
            'success' => true,
            'data' => [
                'tags' => $tags,
                'html' => $this->tagsToHtml($tags),
            ],
        ]);
    }

    /**
     * Get URL submission statistics
     */
    public function getSubmissionStats(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $stats = $this->bingSeoService->getSubmissionStats($period);

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get Bing search performance data
     */
    public function getSearchPerformance(Request $request): JsonResponse
    {
        $period = $request->get('period', '30d');
        $result = $this->bingSeoService->getSearchPerformance($period);

        return response()->json([
            'success' => $result['success'],
            'data' => $result['data'] ?? null,
            'message' => $result['message'] ?? null,
        ], $result['success'] ? 200 : 400);
    }

    /**
     * Get Bing crawl statistics
     */
    public function getCrawlStats(): JsonResponse
    {
        $result = $this->bingSeoService->getCrawlStats();

        return response()->json([
            'success' => $result['success'],
            'data' => $result['data'] ?? null,
            'message' => $result['message'] ?? null,
        ], $result['success'] ? 200 : 400);
    }

    /**
     * Generate Bing-optimized structured data
     */
    public function generateSchema(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'type' => 'required|string|in:Article,NewsArticle,BlogPosting,Product,LocalBusiness,Organization,WebPage',
            'data' => 'required|array',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $schema = $this->bingSeoService->generateBingSchema(
            $request->get('type'),
            $request->get('data')
        );

        return response()->json([
            'success' => true,
            'data' => [
                'schema' => $schema,
                'json_ld' => '<script type="application/ld+json">' . json_encode($schema, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . '</script>',
            ],
        ]);
    }

    /**
     * Serve IndexNow key file
     * This endpoint should be accessible at /{key}.txt
     */
    public function indexNowKeyFile(): Response
    {
        $key = $this->bingSeoService->getIndexNowKeyFile();

        return response($key, 200)
            ->header('Content-Type', 'text/plain');
    }

    /**
     * Get Bing SEO configuration status
     */
    public function getConfiguration(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => [
                'indexnow' => [
                    'enabled' => true,
                    'key_configured' => !empty($this->bingSeoService->getIndexNowKeyFile()),
                    'endpoint' => config('app.url') . '/' . $this->bingSeoService->getIndexNowKeyFile() . '.txt',
                ],
                'webmaster_api' => [
                    'configured' => !empty(config('seo.bing.api_key')),
                ],
                'verification' => [
                    'configured' => !empty(config('seo.bing.verification_code')),
                    'meta_tag' => config('seo.bing.verification_code')
                        ? '<meta name="msvalidate.01" content="' . config('seo.bing.verification_code') . '" />'
                        : null,
                ],
            ],
        ]);
    }

    /**
     * Get complete Bing SEO checklist
     */
    public function getChecklist(): JsonResponse
    {
        $checks = [
            [
                'id' => 'indexnow_key',
                'name' => 'IndexNow Key File',
                'description' => 'Key file accessible for IndexNow verification',
                'status' => 'pass',
                'priority' => 'high',
            ],
            [
                'id' => 'verification',
                'name' => 'Bing Verification',
                'description' => 'Site verified in Bing Webmaster Tools',
                'status' => !empty(config('seo.bing.verification_code')) ? 'pass' : 'fail',
                'priority' => 'high',
                'fix' => 'Add BING_VERIFICATION_CODE to .env',
            ],
            [
                'id' => 'api_key',
                'name' => 'Webmaster API Key',
                'description' => 'Bing Webmaster API key for advanced features',
                'status' => !empty(config('seo.bing.api_key')) ? 'pass' : 'warning',
                'priority' => 'medium',
                'fix' => 'Add BING_WEBMASTER_API_KEY to .env',
            ],
            [
                'id' => 'sitemap',
                'name' => 'Sitemap Submitted',
                'description' => 'XML sitemap submitted to Bing',
                'status' => 'check_required',
                'priority' => 'high',
            ],
            [
                'id' => 'meta_tags',
                'name' => 'Bing Meta Tags',
                'description' => 'Bing-specific meta tags on pages',
                'status' => 'check_required',
                'priority' => 'medium',
            ],
            [
                'id' => 'schema',
                'name' => 'Structured Data',
                'description' => 'Schema.org markup for rich results',
                'status' => 'check_required',
                'priority' => 'high',
            ],
        ];

        $passCount = count(array_filter($checks, fn($c) => $c['status'] === 'pass'));
        $score = round(($passCount / count($checks)) * 100);

        return response()->json([
            'success' => true,
            'data' => [
                'score' => $score,
                'checks' => $checks,
                'passed' => $passCount,
                'total' => count($checks),
            ],
        ]);
    }

    /**
     * Convert meta tag array to HTML string
     */
    private function tagsToHtml(array $tags): string
    {
        $html = [];
        foreach ($tags as $tag) {
            $attrs = [];
            foreach ($tag as $key => $value) {
                $attrs[] = $key . '="' . htmlspecialchars($value, ENT_QUOTES) . '"';
            }
            $html[] = '<meta ' . implode(' ', $attrs) . ' />';
        }
        return implode("\n", $html);
    }
}
