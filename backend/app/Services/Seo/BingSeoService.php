<?php

declare(strict_types=1);

namespace App\Services\Seo;

use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * BingSeoService - Enterprise Bing Search Engine Optimization
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Complete Bing SEO integration that most websites lack - MASSIVE competitive advantage.
 *
 * Features:
 * - IndexNow instant URL submission (URLs indexed in minutes, not days)
 * - Bing Webmaster Tools API integration
 * - Bing-specific meta tag generation
 * - URL submission tracking and analytics
 * - Batch URL submission for efficiency
 * - Automatic submission on content publish
 * - Bing sitemap submission
 * - Search performance analytics
 *
 * @version 1.0.0
 */
class BingSeoService
{
    private const INDEXNOW_ENDPOINT = 'https://api.indexnow.org/indexnow';
    private const BING_WEBMASTER_API = 'https://ssl.bing.com/webmaster/api.svc/json';
    private const BING_URL_SUBMISSION_API = 'https://ssl.bing.com/webmaster/api.svc/json/SubmitUrlBatch';

    private string $indexNowKey;
    private ?string $bingApiKey;
    private string $siteUrl;

    public function __construct()
    {
        $this->indexNowKey = config('seo.bing.indexnow_key', $this->generateIndexNowKey());
        $this->bingApiKey = config('seo.bing.api_key');
        $this->siteUrl = config('app.url');
    }

    /**
     * Submit URL to IndexNow for instant Bing indexing
     * URLs are typically indexed within minutes instead of days/weeks
     */
    public function submitToIndexNow(string $url): array
    {
        $cacheKey = 'indexnow:submitted:' . md5($url);

        // Prevent duplicate submissions within 24 hours
        if (Cache::has($cacheKey)) {
            return [
                'success' => true,
                'message' => 'URL already submitted recently',
                'cached' => true,
            ];
        }

        try {
            $response = Http::timeout(10)->get(self::INDEXNOW_ENDPOINT, [
                'url' => $url,
                'key' => $this->indexNowKey,
            ]);

            $success = in_array($response->status(), [200, 202]);

            if ($success) {
                Cache::put($cacheKey, true, now()->addHours(24));
                $this->logSubmission($url, 'indexnow', $success);
            }

            return [
                'success' => $success,
                'status_code' => $response->status(),
                'message' => $this->getIndexNowStatusMessage($response->status()),
            ];
        } catch (\Exception $e) {
            Log::error('[BingSeo] IndexNow submission failed', [
                'url' => $url,
                'error' => $e->getMessage(),
            ]);

            return [
                'success' => false,
                'message' => 'Submission failed: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Batch submit multiple URLs to IndexNow
     * More efficient for bulk content updates
     */
    public function batchSubmitToIndexNow(array $urls): array
    {
        $urls = array_slice($urls, 0, 10000); // IndexNow limit

        try {
            $host = parse_url($this->siteUrl, PHP_URL_HOST);

            $response = Http::timeout(30)
                ->withHeaders(['Content-Type' => 'application/json'])
                ->post(self::INDEXNOW_ENDPOINT, [
                    'host' => $host,
                    'key' => $this->indexNowKey,
                    'keyLocation' => "{$this->siteUrl}/{$this->indexNowKey}.txt",
                    'urlList' => $urls,
                ]);

            $success = in_array($response->status(), [200, 202]);

            if ($success) {
                foreach ($urls as $url) {
                    $this->logSubmission($url, 'indexnow_batch', true);
                }
            }

            return [
                'success' => $success,
                'submitted_count' => count($urls),
                'status_code' => $response->status(),
                'message' => $this->getIndexNowStatusMessage($response->status()),
            ];
        } catch (\Exception $e) {
            Log::error('[BingSeo] Batch IndexNow submission failed', [
                'url_count' => count($urls),
                'error' => $e->getMessage(),
            ]);

            return [
                'success' => false,
                'message' => 'Batch submission failed: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Submit URL via Bing Webmaster Tools API
     */
    public function submitToBingWebmaster(string $url): array
    {
        if (!$this->bingApiKey) {
            return [
                'success' => false,
                'message' => 'Bing Webmaster API key not configured',
            ];
        }

        try {
            $response = Http::timeout(10)
                ->withHeaders([
                    'Content-Type' => 'application/json',
                ])
                ->post(self::BING_URL_SUBMISSION_API . "?apikey={$this->bingApiKey}", [
                    'siteUrl' => $this->siteUrl,
                    'urlList' => [$url],
                ]);

            $success = $response->successful();
            $this->logSubmission($url, 'bing_webmaster', $success);

            return [
                'success' => $success,
                'status_code' => $response->status(),
                'response' => $response->json(),
            ];
        } catch (\Exception $e) {
            Log::error('[BingSeo] Bing Webmaster submission failed', [
                'url' => $url,
                'error' => $e->getMessage(),
            ]);

            return [
                'success' => false,
                'message' => 'Submission failed: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Get Bing-specific meta tags for a page
     */
    public function generateBingMetaTags(array $options = []): array
    {
        $tags = [];

        // Bing site verification
        if ($verificationCode = config('seo.bing.verification_code')) {
            $tags[] = [
                'name' => 'msvalidate.01',
                'content' => $verificationCode,
            ];
        }

        // Bing bot directives
        $bingbotDirectives = [];

        if ($options['no_archive'] ?? false) {
            $bingbotDirectives[] = 'noarchive';
        }
        if ($options['no_snippet'] ?? false) {
            $bingbotDirectives[] = 'nosnippet';
        }
        if ($options['max_snippet'] ?? null) {
            $bingbotDirectives[] = "max-snippet:{$options['max_snippet']}";
        }
        if ($options['max_image_preview'] ?? null) {
            $bingbotDirectives[] = "max-image-preview:{$options['max_image_preview']}";
        }
        if ($options['max_video_preview'] ?? null) {
            $bingbotDirectives[] = "max-video-preview:{$options['max_video_preview']}";
        }

        if (!empty($bingbotDirectives)) {
            $tags[] = [
                'name' => 'bingbot',
                'content' => implode(', ', $bingbotDirectives),
            ];
        }

        // MSN bot (legacy but still used)
        $tags[] = [
            'name' => 'msnbot',
            'content' => $options['msnbot'] ?? 'index, follow',
        ];

        // Bing-specific language/region
        if ($locale = $options['locale'] ?? null) {
            $tags[] = [
                'name' => 'language',
                'content' => $locale,
            ];
        }

        // Bing geo tags for local SEO
        if ($geo = $options['geo'] ?? null) {
            if (isset($geo['region'])) {
                $tags[] = ['name' => 'geo.region', 'content' => $geo['region']];
            }
            if (isset($geo['placename'])) {
                $tags[] = ['name' => 'geo.placename', 'content' => $geo['placename']];
            }
            if (isset($geo['position'])) {
                $tags[] = ['name' => 'geo.position', 'content' => $geo['position']];
                $tags[] = ['name' => 'ICBM', 'content' => str_replace(';', ', ', $geo['position'])];
            }
        }

        return $tags;
    }

    /**
     * Generate IndexNow key file content
     */
    public function getIndexNowKeyFile(): string
    {
        return $this->indexNowKey;
    }

    /**
     * Submit sitemap to Bing
     */
    public function submitSitemap(string $sitemapUrl): array
    {
        if (!$this->bingApiKey) {
            // Use ping method (no API key required)
            return $this->pingSitemap($sitemapUrl);
        }

        try {
            $response = Http::timeout(10)->get(
                self::BING_WEBMASTER_API . '/SubmitSitemap',
                [
                    'apikey' => $this->bingApiKey,
                    'siteUrl' => $this->siteUrl,
                    'feedUrl' => $sitemapUrl,
                ]
            );

            return [
                'success' => $response->successful(),
                'status_code' => $response->status(),
                'response' => $response->json(),
            ];
        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Sitemap submission failed: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Ping sitemap to Bing (no API key required)
     */
    public function pingSitemap(string $sitemapUrl): array
    {
        try {
            $response = Http::timeout(10)->get(
                'https://www.bing.com/ping',
                ['sitemap' => $sitemapUrl]
            );

            return [
                'success' => $response->successful(),
                'method' => 'ping',
                'status_code' => $response->status(),
            ];
        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Sitemap ping failed: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Get URL submission statistics
     */
    public function getSubmissionStats(string $period = '30d'): array
    {
        $days = match($period) {
            '7d' => 7,
            '30d' => 30,
            '90d' => 90,
            default => 30,
        };

        $stats = DB::table('bing_url_submissions')
            ->where('created_at', '>=', now()->subDays($days))
            ->selectRaw('
                COUNT(*) as total_submissions,
                SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) as successful,
                SUM(CASE WHEN success = 0 THEN 1 ELSE 0 END) as failed,
                COUNT(DISTINCT url) as unique_urls,
                submission_type,
                DATE(created_at) as date
            ')
            ->groupBy('submission_type', 'date')
            ->get();

        $dailyStats = DB::table('bing_url_submissions')
            ->where('created_at', '>=', now()->subDays($days))
            ->selectRaw('DATE(created_at) as date, COUNT(*) as count')
            ->groupBy('date')
            ->orderBy('date')
            ->pluck('count', 'date');

        return [
            'period' => $period,
            'summary' => [
                'total' => $stats->sum('total_submissions'),
                'successful' => $stats->sum('successful'),
                'failed' => $stats->sum('failed'),
                'unique_urls' => $stats->sum('unique_urls'),
                'success_rate' => $stats->sum('total_submissions') > 0
                    ? round(($stats->sum('successful') / $stats->sum('total_submissions')) * 100, 2)
                    : 100,
            ],
            'by_type' => $stats->groupBy('submission_type'),
            'daily' => $dailyStats,
        ];
    }

    /**
     * Get Bing search performance data (requires API key)
     */
    public function getSearchPerformance(string $period = '30d'): array
    {
        if (!$this->bingApiKey) {
            return [
                'success' => false,
                'message' => 'Bing Webmaster API key required',
            ];
        }

        try {
            $response = Http::timeout(30)->get(
                self::BING_WEBMASTER_API . '/GetQueryStats',
                [
                    'apikey' => $this->bingApiKey,
                    'siteUrl' => $this->siteUrl,
                ]
            );

            return [
                'success' => $response->successful(),
                'data' => $response->json(),
            ];
        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Failed to fetch performance data: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Get crawl statistics from Bing
     */
    public function getCrawlStats(): array
    {
        if (!$this->bingApiKey) {
            return [
                'success' => false,
                'message' => 'Bing Webmaster API key required',
            ];
        }

        try {
            $response = Http::timeout(30)->get(
                self::BING_WEBMASTER_API . '/GetCrawlStats',
                [
                    'apikey' => $this->bingApiKey,
                    'siteUrl' => $this->siteUrl,
                ]
            );

            return [
                'success' => $response->successful(),
                'data' => $response->json(),
            ];
        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Failed to fetch crawl stats: ' . $e->getMessage(),
            ];
        }
    }

    /**
     * Auto-submit URL when content is published
     * Call this from model observers or events
     */
    public function onContentPublished(string $url, string $contentType = 'page'): void
    {
        // Submit to IndexNow (fastest)
        $this->submitToIndexNow($url);

        // Also submit to Bing Webmaster if configured
        if ($this->bingApiKey) {
            $this->submitToBingWebmaster($url);
        }

        Log::info('[BingSeo] Content auto-submitted to Bing', [
            'url' => $url,
            'content_type' => $contentType,
        ]);
    }

    /**
     * Generate Bing-optimized structured data
     */
    public function generateBingSchema(string $type, array $data): array
    {
        $schema = [
            '@context' => 'https://schema.org',
            '@type' => $type,
        ];

        // Bing prefers certain schema properties
        $bingOptimized = match($type) {
            'Article', 'NewsArticle', 'BlogPosting' => [
                'headline' => $data['title'] ?? '',
                'description' => $data['description'] ?? '',
                'image' => $data['image'] ?? '',
                'datePublished' => $data['published_at'] ?? now()->toIso8601String(),
                'dateModified' => $data['updated_at'] ?? now()->toIso8601String(),
                'author' => [
                    '@type' => 'Person',
                    'name' => $data['author'] ?? '',
                ],
                'publisher' => [
                    '@type' => 'Organization',
                    'name' => config('app.name'),
                    'logo' => [
                        '@type' => 'ImageObject',
                        'url' => config('app.url') . '/logo.png',
                    ],
                ],
                'mainEntityOfPage' => [
                    '@type' => 'WebPage',
                    '@id' => $data['url'] ?? '',
                ],
            ],
            'Product' => [
                'name' => $data['name'] ?? '',
                'description' => $data['description'] ?? '',
                'image' => $data['image'] ?? '',
                'sku' => $data['sku'] ?? '',
                'brand' => [
                    '@type' => 'Brand',
                    'name' => $data['brand'] ?? config('app.name'),
                ],
                'offers' => [
                    '@type' => 'Offer',
                    'price' => $data['price'] ?? '',
                    'priceCurrency' => $data['currency'] ?? 'USD',
                    'availability' => 'https://schema.org/InStock',
                    'url' => $data['url'] ?? '',
                ],
            ],
            'LocalBusiness' => [
                'name' => $data['name'] ?? config('app.name'),
                'image' => $data['image'] ?? '',
                'address' => [
                    '@type' => 'PostalAddress',
                    'streetAddress' => $data['street'] ?? '',
                    'addressLocality' => $data['city'] ?? '',
                    'addressRegion' => $data['state'] ?? '',
                    'postalCode' => $data['zip'] ?? '',
                    'addressCountry' => $data['country'] ?? 'US',
                ],
                'telephone' => $data['phone'] ?? '',
                'url' => $data['url'] ?? $this->siteUrl,
            ],
            default => $data,
        };

        return array_merge($schema, $bingOptimized);
    }

    /**
     * Log URL submission for analytics
     */
    private function logSubmission(string $url, string $type, bool $success): void
    {
        try {
            DB::table('bing_url_submissions')->insert([
                'url' => $url,
                'submission_type' => $type,
                'success' => $success,
                'created_at' => now(),
            ]);
        } catch (\Exception $e) {
            // Table might not exist yet, log to file instead
            Log::info('[BingSeo] URL submitted', [
                'url' => $url,
                'type' => $type,
                'success' => $success,
            ]);
        }
    }

    /**
     * Generate a new IndexNow key
     */
    private function generateIndexNowKey(): string
    {
        $key = Cache::get('indexnow_key');

        if (!$key) {
            $key = Str::uuid()->toString();
            Cache::forever('indexnow_key', $key);
        }

        return $key;
    }

    /**
     * Get human-readable IndexNow status message
     */
    private function getIndexNowStatusMessage(int $statusCode): string
    {
        return match($statusCode) {
            200 => 'URL submitted and will be indexed',
            202 => 'URL accepted for processing',
            400 => 'Invalid request format',
            403 => 'Key not valid or not matching',
            422 => 'URL not valid or not matching host',
            429 => 'Too many requests, rate limited',
            default => 'Unknown response status',
        };
    }
}
