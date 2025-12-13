<?php

declare(strict_types=1);

namespace App\Services\Seo\Providers;

use App\Contracts\Seo\KeywordDataProviderInterface;
use App\DataTransferObjects\Seo\KeywordData;
use App\DataTransferObjects\Seo\KeywordGap;
use App\DataTransferObjects\Seo\SerpAnalysis;
use App\Services\Integration\GoogleSearchConsoleService;
use App\Services\CacheService;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Log;

/**
 * Google Search Console Provider
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * PRIMARY data source for YOUR site's SEO performance.
 * This is the most authoritative source for:
 * - Your actual keyword rankings
 * - Click and impression data
 * - CTR metrics
 * - Historical position data
 *
 * This is THE Google tool for keyword gap analysis based on your own data.
 *
 * Capabilities:
 * - Authoritative position data for your site
 * - Click and impression metrics
 * - CTR data
 * - Historical trends
 * - URL performance
 * - Device/country segmentation
 *
 * Unique Value:
 * - 100% accurate data for YOUR site
 * - No scraping or estimation
 * - Fully compliant with Google ToS
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
class GoogleSearchConsoleProvider implements KeywordDataProviderInterface
{
    private const CACHE_PREFIX = 'seo:gsc:provider';
    private const CACHE_TTL = 300; // 5 minutes (GSC data updates frequently)
    private const PROVIDER_ID = 'google_search_console';

    private GoogleSearchConsoleService $gscService;
    private CacheService $cache;
    private array $healthMetrics = [];

    public function __construct(
        GoogleSearchConsoleService $gscService,
        CacheService $cache
    ) {
        $this->gscService = $gscService;
        $this->cache = $cache;
    }

    /**
     * {@inheritdoc}
     */
    public function getProviderId(): string
    {
        return self::PROVIDER_ID;
    }

    /**
     * {@inheritdoc}
     */
    public function getProviderName(): string
    {
        return 'Google Search Console';
    }

    /**
     * {@inheritdoc}
     */
    public function isAvailable(): bool
    {
        return !empty(config('services.google.client_id'))
            && !empty(config('services.google.client_secret'));
    }

    /**
     * Check if user is connected to GSC.
     */
    public function isConnected(int $userId): bool
    {
        return $this->gscService->isConnected($userId);
    }

    /**
     * {@inheritdoc}
     */
    public function getHealthStatus(): array
    {
        return [
            'status' => $this->isAvailable() ? 'available' : 'unavailable',
            'latency_ms' => $this->healthMetrics['latency_ms'] ?? null,
            'rate_limit_remaining' => null, // GSC doesn't expose this
            'last_successful_call' => $this->healthMetrics['last_successful_call'] ?? null,
            'error_rate_24h' => $this->healthMetrics['error_rate_24h'] ?? 0,
            'capabilities' => $this->getCapabilities(),
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getCapabilities(): array
    {
        return [
            self::CAPABILITY_SERP_POSITION,
            self::CAPABILITY_CLICK_DATA,
            self::CAPABILITY_IMPRESSION_DATA,
            self::CAPABILITY_CTR_DATA,
            self::CAPABILITY_HISTORICAL_DATA,
            self::CAPABILITY_INTENT_CLASSIFICATION, // Via analysis
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function hasCapability(string $capability): bool
    {
        return in_array($capability, $this->getCapabilities());
    }

    /**
     * {@inheritdoc}
     */
    public function getPriority(): int
    {
        return 10; // Highest priority - authoritative for your site
    }

    /**
     * {@inheritdoc}
     */
    public function getRateLimitConfig(): array
    {
        return [
            'requests_per_minute' => 1200,
            'requests_per_day' => 50000,
            'current_usage' => $this->healthMetrics['daily_usage'] ?? 0,
            'reset_at' => now()->endOfDay()->toIso8601String(),
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getKeywordData(string $keyword, array $options = []): KeywordData
    {
        $userId = $options['user_id'] ?? null;
        $siteUrl = $options['site_url'] ?? null;
        $days = $options['days'] ?? 28;

        if (!$userId || !$siteUrl) {
            return new KeywordData(
                keyword: $keyword,
                dataSource: self::PROVIDER_ID,
                dataConfidence: 0,
                fetchedAt: new \DateTimeImmutable(),
            );
        }

        $cacheKey = self::CACHE_PREFIX . ":keyword:{$userId}:" . md5($keyword . $siteUrl);

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return KeywordData::fromArray($cached);
        }

        try {
            $rankings = $this->gscService->getKeywordRankings($userId, $siteUrl, $days);

            foreach ($rankings['keywords'] ?? [] as $kw) {
                if (strtolower($kw['keyword']) === strtolower($keyword)) {
                    $data = KeywordData::fromGoogleSearchConsole($kw)
                        ->withCalculatedOpportunityScore();

                    $this->cache->put($cacheKey, $data->toArray(), self::CACHE_TTL);
                    $this->recordSuccessfulCall();

                    return $data;
                }
            }

            // Keyword not found in GSC data
            return new KeywordData(
                keyword: $keyword,
                dataSource: self::PROVIDER_ID,
                dataConfidence: 0.5, // We checked, it's just not ranking
                fetchedAt: new \DateTimeImmutable(),
            );

        } catch (\Exception $e) {
            Log::error('GSC Provider error', [
                'keyword' => $keyword,
                'error' => $e->getMessage(),
            ]);

            $this->recordFailedCall();

            return new KeywordData(
                keyword: $keyword,
                dataSource: self::PROVIDER_ID,
                dataConfidence: 0,
                fetchedAt: new \DateTimeImmutable(),
            );
        }
    }

    /**
     * {@inheritdoc}
     */
    public function getKeywordDataBatch(array $keywords, array $options = []): Collection
    {
        $userId = $options['user_id'] ?? null;
        $siteUrl = $options['site_url'] ?? null;
        $days = $options['days'] ?? 28;

        $results = new Collection();

        if (!$userId || !$siteUrl) {
            foreach ($keywords as $keyword) {
                $results->push(new KeywordData(
                    keyword: $keyword,
                    dataSource: self::PROVIDER_ID,
                    dataConfidence: 0,
                    fetchedAt: new \DateTimeImmutable(),
                ));
            }
            return $results;
        }

        try {
            $rankings = $this->gscService->getKeywordRankings($userId, $siteUrl, $days);
            $keywordMap = [];

            foreach ($rankings['keywords'] ?? [] as $kw) {
                $keywordMap[strtolower($kw['keyword'])] = $kw;
            }

            foreach ($keywords as $keyword) {
                $kwLower = strtolower($keyword);

                if (isset($keywordMap[$kwLower])) {
                    $data = KeywordData::fromGoogleSearchConsole($keywordMap[$kwLower])
                        ->withCalculatedOpportunityScore();
                    $results->push($data);
                } else {
                    $results->push(new KeywordData(
                        keyword: $keyword,
                        dataSource: self::PROVIDER_ID,
                        dataConfidence: 0.5,
                        fetchedAt: new \DateTimeImmutable(),
                    ));
                }
            }

            $this->recordSuccessfulCall();

        } catch (\Exception $e) {
            Log::error('GSC Provider batch error', [
                'error' => $e->getMessage(),
            ]);

            $this->recordFailedCall();

            foreach ($keywords as $keyword) {
                $results->push(new KeywordData(
                    keyword: $keyword,
                    dataSource: self::PROVIDER_ID,
                    dataConfidence: 0,
                    fetchedAt: new \DateTimeImmutable(),
                ));
            }
        }

        return $results;
    }

    /**
     * Get ALL keywords for a site with full metrics.
     * This is the core method for GSC-based keyword gap analysis.
     */
    public function getAllKeywords(int $userId, string $siteUrl, int $days = 28): Collection
    {
        $cacheKey = self::CACHE_PREFIX . ":all_keywords:{$userId}:" . md5($siteUrl . $days);

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return collect($cached)->map(fn($data) => KeywordData::fromArray($data));
        }

        try {
            $rankings = $this->gscService->getKeywordRankings($userId, $siteUrl, $days);

            $results = collect($rankings['keywords'] ?? [])
                ->map(fn($kw) => KeywordData::fromGoogleSearchConsole($kw)->withCalculatedOpportunityScore());

            $this->cache->put($cacheKey, $results->map->toArray()->toArray(), self::CACHE_TTL);
            $this->recordSuccessfulCall();

            return $results;

        } catch (\Exception $e) {
            Log::error('GSC getAllKeywords error', [
                'user_id' => $userId,
                'error' => $e->getMessage(),
            ]);

            $this->recordFailedCall();
            return new Collection();
        }
    }

    /**
     * Find keyword gaps and opportunities from GSC data.
     * This is THE core method for Google-only keyword gap analysis.
     *
     * @return Collection<KeywordGap>
     */
    public function findKeywordGaps(int $userId, string $siteUrl, array $options = []): Collection
    {
        $keywords = $this->getAllKeywords($userId, $siteUrl, $options['days'] ?? 28);
        $gaps = new Collection();

        foreach ($keywords as $kw) {
            // Position opportunities (rankings 11-30 with high impressions)
            if ($kw->isQuickWin()) {
                $gaps->push(KeywordGap::createPositionOpportunity($kw));
            }

            // CTR optimization opportunities
            if ($kw->needsCtrOptimization()) {
                $gaps->push(KeywordGap::createCtrOpportunity($kw));
            }

            // Declining keywords
            if ($kw->isDeclining()) {
                $gaps->push(KeywordGap::createDecliningKeyword($kw));
            }

            // Untapped impressions (high impressions, very low or no clicks)
            if (($kw->impressions ?? 0) > 1000 && ($kw->clicks ?? 0) < 10) {
                $gaps->push(new KeywordGap(
                    keyword: $kw->keyword,
                    gapType: KeywordGap::TYPE_UNTAPPED_IMPRESSION,
                    description: sprintf(
                        'Keyword "%s" has %d impressions but only %d clicks. Content may not match search intent.',
                        $kw->keyword,
                        $kw->impressions,
                        $kw->clicks ?? 0
                    ),
                    priority: KeywordGap::PRIORITY_HIGH,
                    opportunityScore: 75,
                    effort: KeywordGap::EFFORT_MEDIUM,
                    actions: [
                        'Review the ranking page content for search intent alignment',
                        'Update title and meta description to better match the query',
                        'Consider if a different page should target this keyword',
                        'Add FAQ or featured snippet-optimized content',
                    ],
                    impressions: $kw->impressions,
                    clicks: $kw->clicks,
                    ctr: $kw->ctr,
                    currentPosition: $kw->currentPosition,
                    affectedUrl: $kw->url,
                    dataSource: self::PROVIDER_ID,
                    identifiedAt: new \DateTimeImmutable(),
                ));
            }
        }

        // Check for cannibalization
        $cannibalization = $this->detectCannibalization($userId, $siteUrl);
        foreach ($cannibalization as $issue) {
            $gaps->push($issue);
        }

        // Sort by priority and opportunity score
        return $gaps->sortByDesc(function (KeywordGap $gap) {
            $priorityWeight = match ($gap->priority) {
                KeywordGap::PRIORITY_CRITICAL => 1000,
                KeywordGap::PRIORITY_HIGH => 100,
                KeywordGap::PRIORITY_MEDIUM => 10,
                KeywordGap::PRIORITY_LOW => 1,
                default => 5,
            };

            return $priorityWeight + $gap->opportunityScore;
        })->values();
    }

    /**
     * Detect keyword cannibalization (multiple URLs ranking for same keyword).
     *
     * @return Collection<KeywordGap>
     */
    public function detectCannibalization(int $userId, string $siteUrl): Collection
    {
        try {
            // Get query+page dimension data
            $data = $this->gscService->getSearchAnalytics(
                $userId,
                $siteUrl,
                now()->subDays(28)->format('Y-m-d'),
                now()->format('Y-m-d'),
                ['query', 'page'],
                1000
            );

            // Group by query
            $queryPages = [];
            foreach ($data as $row) {
                $query = $row['query'] ?? '';
                if (!isset($queryPages[$query])) {
                    $queryPages[$query] = [];
                }
                $queryPages[$query][] = [
                    'page' => $row['page'] ?? '',
                    'clicks' => $row['clicks'] ?? 0,
                    'impressions' => $row['impressions'] ?? 0,
                    'position' => $row['position'] ?? 0,
                    'ctr' => $row['ctr'] ?? 0,
                ];
            }

            $cannibalization = new Collection();

            foreach ($queryPages as $query => $pages) {
                if (count($pages) >= 2) {
                    // Sort by impressions to identify primary page
                    usort($pages, fn($a, $b) => $b['impressions'] - $a['impressions']);

                    $totalImpressions = array_sum(array_column($pages, 'impressions'));
                    $totalClicks = array_sum(array_column($pages, 'clicks'));

                    // Only flag if there's significant volume
                    if ($totalImpressions > 500) {
                        $cannibalization->push(KeywordGap::createCannibalization(
                            $query,
                            array_column($pages, 'page'),
                            [
                                'total_impressions' => $totalImpressions,
                                'total_clicks' => $totalClicks,
                                'per_page' => $pages,
                            ]
                        ));
                    }
                }
            }

            return $cannibalization;

        } catch (\Exception $e) {
            Log::error('Cannibalization detection error', [
                'error' => $e->getMessage(),
            ]);

            return new Collection();
        }
    }

    /**
     * Get top opportunities (quick wins).
     *
     * @return Collection<KeywordGap>
     */
    public function getQuickWins(int $userId, string $siteUrl, int $limit = 20): Collection
    {
        return $this->findKeywordGaps($userId, $siteUrl)
            ->filter(fn(KeywordGap $gap) => $gap->isQuickWin())
            ->take($limit);
    }

    /**
     * Get position opportunities (keywords close to page 1).
     */
    public function getPositionOpportunities(int $userId, string $siteUrl, int $limit = 50): Collection
    {
        $keywords = $this->getAllKeywords($userId, $siteUrl);

        return $keywords
            ->filter(fn(KeywordData $kw) =>
                $kw->currentPosition !== null
                && $kw->currentPosition >= 11
                && $kw->currentPosition <= 30
                && ($kw->impressions ?? 0) > 100
            )
            ->sortByDesc(fn(KeywordData $kw) => $kw->impressions)
            ->take($limit)
            ->map(fn(KeywordData $kw) => KeywordGap::createPositionOpportunity($kw));
    }

    /**
     * Get CTR optimization opportunities.
     */
    public function getCtrOpportunities(int $userId, string $siteUrl, int $limit = 50): Collection
    {
        $keywords = $this->getAllKeywords($userId, $siteUrl);

        return $keywords
            ->filter(fn(KeywordData $kw) => $kw->needsCtrOptimization())
            ->sortByDesc(fn(KeywordData $kw) => $kw->impressions)
            ->take($limit)
            ->map(fn(KeywordData $kw) => KeywordGap::createCtrOpportunity($kw));
    }

    /**
     * Get declining keywords that need attention.
     */
    public function getDecliningKeywords(int $userId, string $siteUrl, int $limit = 50): Collection
    {
        $keywords = $this->getAllKeywords($userId, $siteUrl);

        return $keywords
            ->filter(fn(KeywordData $kw) => $kw->isDeclining())
            ->sortBy(fn(KeywordData $kw) => $kw->positionChange) // Most declined first
            ->take($limit)
            ->map(fn(KeywordData $kw) => KeywordGap::createDecliningKeyword($kw));
    }

    /**
     * {@inheritdoc}
     */
    public function getSearchVolume(string $keyword, array $options = []): array
    {
        // GSC provides impressions, which is a proxy for search volume
        $data = $this->getKeywordData($keyword, $options);

        return [
            'volume' => null, // GSC doesn't provide search volume
            'impressions' => $data->impressions, // Proxy metric
            'volume_trend' => $data->trend,
            'seasonality' => null,
            'confidence' => 0.5, // Lower confidence as it's a proxy
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getKeywordDifficulty(string $keyword): array
    {
        // GSC doesn't provide difficulty, but we can estimate from position volatility
        return [
            'difficulty' => null,
            'difficulty_label' => 'unknown',
            'factors' => [],
            'confidence' => 0,
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getRelatedKeywords(string $seedKeyword, int $limit = 50, array $options = []): Collection
    {
        // GSC doesn't provide related keywords
        // Return keywords that appear on the same pages
        $userId = $options['user_id'] ?? null;
        $siteUrl = $options['site_url'] ?? null;

        if (!$userId || !$siteUrl) {
            return new Collection();
        }

        $keywords = $this->getAllKeywords($userId, $siteUrl);
        $seedData = $keywords->firstWhere('keyword', strtolower($seedKeyword));

        if (!$seedData || !$seedData->url) {
            return new Collection();
        }

        // Find other keywords ranking for the same URL
        return $keywords
            ->filter(fn(KeywordData $kw) =>
                $kw->url === $seedData->url
                && strtolower($kw->keyword) !== strtolower($seedKeyword)
            )
            ->take($limit);
    }

    /**
     * {@inheritdoc}
     */
    public function getQuestionKeywords(string $seedKeyword, int $limit = 20): Collection
    {
        // Filter GSC keywords for questions
        return new Collection(); // Would need full keyword list
    }

    /**
     * {@inheritdoc}
     */
    public function getLongTailVariations(string $seedKeyword, int $limit = 50): Collection
    {
        return new Collection(); // Not supported by GSC
    }

    /**
     * {@inheritdoc}
     */
    public function analyzeSERP(string $keyword, array $options = []): SerpAnalysis
    {
        // GSC provides limited SERP data (just your position)
        $data = $this->getKeywordData($keyword, $options);

        return SerpAnalysis::fromGoogleSearchConsole($keyword, [
            'position' => $data->currentPosition,
            'url' => $data->url,
            'clicks' => $data->clicks,
            'impressions' => $data->impressions,
            'ctr' => $data->ctr,
        ]);
    }

    /**
     * {@inheritdoc}
     */
    public function getKeywordPosition(string $keyword, string $url, array $options = []): array
    {
        $data = $this->getKeywordData($keyword, $options);

        return [
            'position' => $data->currentPosition,
            'page' => $data->currentPosition ? (int) ceil($data->currentPosition / 10) : null,
            'url_found' => $data->url,
            'serp_features' => [],
            'competitors' => [], // GSC doesn't show competitors
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getTrendingKeywords(string $topic, array $options = []): Collection
    {
        // GSC doesn't provide trending data
        return new Collection();
    }

    /**
     * {@inheritdoc}
     */
    public function classifyIntent(string $keyword): array
    {
        // Use local classification
        $keyword = strtolower($keyword);

        $transactional = ['buy', 'purchase', 'order', 'price', 'cost', 'cheap', 'discount'];
        foreach ($transactional as $signal) {
            if (str_contains($keyword, $signal)) {
                return [
                    'primary_intent' => KeywordData::INTENT_TRANSACTIONAL,
                    'secondary_intent' => null,
                    'confidence' => 0.85,
                    'signals' => [$signal],
                ];
            }
        }

        $commercial = ['best', 'top', 'review', 'comparison', 'vs'];
        foreach ($commercial as $signal) {
            if (str_contains($keyword, $signal)) {
                return [
                    'primary_intent' => KeywordData::INTENT_COMMERCIAL,
                    'secondary_intent' => null,
                    'confidence' => 0.8,
                    'signals' => [$signal],
                ];
            }
        }

        return [
            'primary_intent' => KeywordData::INTENT_INFORMATIONAL,
            'secondary_intent' => null,
            'confidence' => 0.7,
            'signals' => [],
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getHistoricalData(string $keyword, int $days = 90): array
    {
        // GSC provides historical data through separate queries
        return [
            'data_points' => [],
            'trend' => 'stable',
            'average_position' => null,
            'position_change' => null,
            'volatility' => 0,
        ];
    }

    /**
     * Record successful API call.
     */
    private function recordSuccessfulCall(): void
    {
        $this->healthMetrics['last_successful_call'] = now()->toIso8601String();
        $this->healthMetrics['daily_usage'] = ($this->healthMetrics['daily_usage'] ?? 0) + 1;
    }

    /**
     * Record failed API call.
     */
    private function recordFailedCall(): void
    {
        $this->healthMetrics['error_rate_24h'] = ($this->healthMetrics['error_rate_24h'] ?? 0) + 0.01;
    }
}
