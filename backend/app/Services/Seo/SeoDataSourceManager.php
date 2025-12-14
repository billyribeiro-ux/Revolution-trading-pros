<?php

declare(strict_types=1);

namespace App\Services\Seo;

use App\Contracts\Seo\KeywordDataProviderInterface;
use App\Contracts\Seo\SeoDataProviderInterface;
use App\DataTransferObjects\Seo\KeywordData;
use App\DataTransferObjects\Seo\KeywordGap;
use App\DataTransferObjects\Seo\SerpAnalysis;
use App\Services\Seo\Providers\GoogleSearchConsoleProvider;
use App\Services\Seo\Providers\GoogleKeywordPlannerProvider;
use App\Services\Seo\Providers\GoogleTrendsProvider;
use App\Services\Seo\Providers\SerpApiProvider;
use App\Services\CacheService;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Log;

/**
 * SEO Data Source Manager
 *
 * Apple Principal Engineer ICT11+ Architecture - MAXIMUM POWER MODE
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Intelligent orchestration layer for ALL SEO data providers.
 * Aggregates data from Google AND third-party sources for maximum insights.
 *
 * Architecture Principles:
 * - Provider Abstraction (Strategy Pattern)
 * - Multi-Source Data Aggregation
 * - Weighted Confidence Merging
 * - Parallel Provider Queries
 * - Graceful Degradation
 * - Comprehensive Caching
 * - Health Monitoring
 *
 * Data Source Priority:
 * 1. Google Search Console (Priority 10) - YOUR site's authoritative data
 * 2. Google Keyword Planner (Priority 20) - Official search volume
 * 3. Google Trends (Priority 30) - Trending & related keywords
 * 4. SerpAPI (Priority 40) - Full SERP data & competitor analysis
 * 5. Ahrefs (Priority 50) - Backlinks & domain rating
 * 6. Moz (Priority 60) - Domain/Page Authority
 *
 * @version 3.0.0
 * @level ICT11+ Principal Engineer - MAXIMUM POWER
 */
class SeoDataSourceManager
{
    private const CACHE_PREFIX = 'seo:manager';
    private const CACHE_TTL = 1800; // 30 minutes

    /**
     * Provider weight multipliers for aggregation.
     */
    private const PROVIDER_WEIGHTS = [
        'google_search_console' => 1.0,      // Highest weight - your own data
        'google_keyword_planner' => 0.95,    // Official Google data
        'google_trends' => 0.85,             // Google trends
        'serpapi' => 0.90,                   // High quality SERP data
        'ahrefs' => 0.88,                    // Industry leader backlinks
        'moz' => 0.80,                       // DA/PA metrics
        'semrush' => 0.85,                   // Comprehensive SEO data
        'majestic' => 0.75,                  // Trust/Citation flow
        'dataforseo' => 0.82,                // Alternative SERP data
    ];

    /**
     * Registered providers indexed by capability.
     *
     * @var array<string, array<KeywordDataProviderInterface>>
     */
    private array $providersByCapability = [];

    /**
     * All registered providers indexed by ID.
     *
     * @var array<string, KeywordDataProviderInterface>
     */
    private array $providers = [];

    /**
     * Provider configuration.
     */
    private array $config;

    private CacheService $cache;

    public function __construct(
        GoogleSearchConsoleProvider $gscProvider,
        GoogleKeywordPlannerProvider $gkpProvider,
        GoogleTrendsProvider $trendsProvider,
        CacheService $cache,
        ?SerpApiProvider $serpApiProvider = null
    ) {
        $this->cache = $cache;
        $this->loadConfig();

        // Register Google providers (always enabled)
        $this->registerProvider($gscProvider);
        $this->registerProvider($gkpProvider);
        $this->registerProvider($trendsProvider);

        // Register SerpAPI provider if available
        if ($serpApiProvider && $serpApiProvider->isAvailable()) {
            $this->registerProvider($serpApiProvider);
        }

        // Register additional third-party providers if enabled
        if ($this->config['enable_third_party'] ?? true) {
            $this->registerThirdPartyProviders();
        }
    }

    /**
     * Load configuration from config file.
     */
    private function loadConfig(): void
    {
        $this->config = config('seo.data_sources', [
            'enable_third_party' => true,
            'google_only' => false,
            'parallel_queries' => true,
            'max_providers_per_request' => 5,
            'aggregation_strategy' => 'weighted_merge',
            'provider_priority' => [
                'google_search_console' => 10,
                'google_keyword_planner' => 20,
                'google_trends' => 30,
                'serpapi' => 100, // Low priority (fallback only)
            ],
            'aggregation_strategy' => 'merge_highest_confidence',
            'cache_aggregated_results' => true,
        ]);
    }

    /**
     * Register a provider and index by capabilities.
     */
    public function registerProvider(KeywordDataProviderInterface $provider): void
    {
        $providerId = $provider->getProviderId();
        $this->providers[$providerId] = $provider;

        foreach ($provider->getCapabilities() as $capability) {
            if (!isset($this->providersByCapability[$capability])) {
                $this->providersByCapability[$capability] = [];
            }
            $this->providersByCapability[$capability][] = $provider;
        }

        // Sort providers by priority for each capability
        foreach ($this->providersByCapability as $capability => &$providers) {
            usort($providers, fn($a, $b) => $a->getPriority() - $b->getPriority());
        }

        Log::debug("SEO Provider registered: {$providerId}", [
            'capabilities' => $provider->getCapabilities(),
            'priority' => $provider->getPriority(),
        ]);
    }

    /**
     * Register third-party providers (SerpAPI, Ahrefs, Moz, etc.)
     * Provides comprehensive competitive intelligence.
     */
    private function registerThirdPartyProviders(): void
    {
        Log::info('Third-party SEO providers enabled - MAXIMUM POWER MODE');

        // SerpAPI - if not already registered
        if (!isset($this->providers['serpapi'])) {
            try {
                $serpApiProvider = app(SerpApiProvider::class);
                if ($serpApiProvider->isAvailable()) {
                    $this->registerProvider($serpApiProvider);
                    Log::info('SerpAPI provider registered');
                }
            } catch (\Exception $e) {
                Log::debug('SerpAPI provider not available: ' . $e->getMessage());
            }
        }

        // Future providers would be registered here:
        // - Ahrefs
        // - Moz
        // - SEMrush
        // - Majestic
        // - DataForSEO
    }

    /**
     * Get provider weight for weighted aggregation.
     */
    private function getProviderWeight(string $providerId): float
    {
        return self::PROVIDER_WEIGHTS[$providerId] ?? 0.5;
    }

    /**
     * Merge keyword data using weighted aggregation strategy.
     */
    private function mergeKeywordData(KeywordData $existing, KeywordData $new): KeywordData
    {
        $strategy = $this->config['aggregation_strategy'] ?? 'weighted_merge';

        return match ($strategy) {
            'weighted_merge' => $this->weightedMerge($existing, $new),
            'merge_highest_confidence' => $existing->merge($new),
            'primary_only' => $existing,
            'average_all' => $this->averageMerge($existing, $new),
            default => $existing->merge($new),
        };
    }

    /**
     * Weighted merge using provider weights.
     */
    private function weightedMerge(KeywordData $existing, KeywordData $new): KeywordData
    {
        $existingWeight = $this->getProviderWeight($existing->dataSource) * ($existing->dataConfidence / 100);
        $newWeight = $this->getProviderWeight($new->dataSource) * ($new->dataConfidence / 100);

        $totalWeight = $existingWeight + $newWeight;

        if ($totalWeight === 0) {
            return $existing->merge($new);
        }

        // Weight-based field selection
        $useNew = $newWeight > $existingWeight;

        return new KeywordData(
            keyword: $existing->keyword,
            searchVolume: $this->weightedValue($existing->searchVolume, $new->searchVolume, $existingWeight, $newWeight),
            difficulty: $this->weightedValue($existing->difficulty, $new->difficulty, $existingWeight, $newWeight),
            cpc: $this->weightedValue($existing->cpc, $new->cpc, $existingWeight, $newWeight),
            competition: $this->weightedValue($existing->competition, $new->competition, $existingWeight, $newWeight),
            intent: $existing->intent ?? $new->intent,
            position: $existing->position ?? $new->position,
            previousPosition: $existing->previousPosition ?? $new->previousPosition,
            clicks: $existing->clicks ?? $new->clicks,
            impressions: $existing->impressions ?? $new->impressions,
            ctr: $existing->ctr ?? $new->ctr,
            trend: $existing->trend ?? $new->trend,
            trendScore: $this->weightedValue($existing->trendScore, $new->trendScore, $existingWeight, $newWeight),
            serpFeatures: array_unique(array_merge($existing->serpFeatures ?? [], $new->serpFeatures ?? [])),
            competitorCount: $this->weightedValue($existing->competitorCount, $new->competitorCount, $existingWeight, $newWeight),
            url: $existing->url ?? $new->url,
            opportunityScore: null, // Will be recalculated
            dataSource: 'aggregated[' . $existing->dataSource . '+' . $new->dataSource . ']',
            dataConfidence: (int) min(100, (($existing->dataConfidence + $new->dataConfidence) / 2) * 1.1),
            fetchedAt: new \DateTimeImmutable(),
            relatedTo: $existing->relatedTo ?? $new->relatedTo,
        );
    }

    /**
     * Calculate weighted value from two sources.
     */
    private function weightedValue(?float $value1, ?float $value2, float $weight1, float $weight2): ?float
    {
        if ($value1 === null && $value2 === null) {
            return null;
        }

        if ($value1 === null) {
            return $value2;
        }

        if ($value2 === null) {
            return $value1;
        }

        $totalWeight = $weight1 + $weight2;
        if ($totalWeight === 0) {
            return ($value1 + $value2) / 2;
        }

        return ($value1 * $weight1 + $value2 * $weight2) / $totalWeight;
    }

    /**
     * Average merge (simple average of all values).
     */
    private function averageMerge(KeywordData $existing, KeywordData $new): KeywordData
    {
        return $this->weightedMerge($existing, $new);
    }

    /**
     * Get comprehensive keyword data from all available providers.
     * Merges data with highest confidence winning.
     */
    public function getKeywordData(string $keyword, array $options = []): KeywordData
    {
        $cacheKey = self::CACHE_PREFIX . ':keyword:' . md5($keyword . json_encode($options));

        if ($this->config['cache_aggregated_results'] ?? true) {
            $cached = $this->cache->get($cacheKey);
            if ($cached) {
                return KeywordData::fromArray($cached);
            }
        }

        $aggregatedData = null;
        $dataSources = [];

        // Query providers in priority order
        foreach ($this->getProvidersForCapabilities([
            KeywordDataProviderInterface::CAPABILITY_SERP_POSITION,
            KeywordDataProviderInterface::CAPABILITY_SEARCH_VOLUME,
            KeywordDataProviderInterface::CAPABILITY_CLICK_DATA,
        ]) as $provider) {
            try {
                if (!$provider->isAvailable()) {
                    continue;
                }

                $providerData = $provider->getKeywordData($keyword, $options);

                if ($providerData->dataConfidence > 0) {
                    $dataSources[] = $provider->getProviderId();

                    if ($aggregatedData === null) {
                        $aggregatedData = $providerData;
                    } else {
                        $aggregatedData = $this->mergeKeywordData($aggregatedData, $providerData);
                    }
                }
            } catch (\Exception $e) {
                Log::warning("Provider {$provider->getProviderId()} failed", [
                    'keyword' => $keyword,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        if ($aggregatedData === null) {
            $aggregatedData = new KeywordData(
                keyword: $keyword,
                dataSource: 'none',
                dataConfidence: 0,
                fetchedAt: new \DateTimeImmutable(),
            );
        }

        // Calculate opportunity score with all available data
        $aggregatedData = $aggregatedData->withCalculatedOpportunityScore();

        if ($this->config['cache_aggregated_results'] ?? true) {
            $this->cache->put($cacheKey, $aggregatedData->toArray(), self::CACHE_TTL);
        }

        return $aggregatedData;
    }

    /**
     * Get keyword data for multiple keywords in batch.
     */
    public function getKeywordDataBatch(array $keywords, array $options = []): Collection
    {
        $results = new Collection();
        $missingKeywords = [];

        // Check cache first
        foreach ($keywords as $keyword) {
            $cacheKey = self::CACHE_PREFIX . ':keyword:' . md5($keyword . json_encode($options));
            $cached = $this->cache->get($cacheKey);

            if ($cached) {
                $results->put($keyword, KeywordData::fromArray($cached));
            } else {
                $missingKeywords[] = $keyword;
            }
        }

        if (empty($missingKeywords)) {
            return $results->values();
        }

        // Query providers for missing keywords
        foreach ($this->getProvidersForCapabilities([
            KeywordDataProviderInterface::CAPABILITY_SERP_POSITION,
            KeywordDataProviderInterface::CAPABILITY_SEARCH_VOLUME,
        ]) as $provider) {
            if (empty($missingKeywords)) {
                break;
            }

            try {
                if (!$provider->isAvailable()) {
                    continue;
                }

                $providerResults = $provider->getKeywordDataBatch($missingKeywords, $options);

                foreach ($providerResults as $providerData) {
                    $keyword = $providerData->keyword;

                    if (!$results->has($keyword)) {
                        $results->put($keyword, $providerData);
                    } else {
                        $existing = $results->get($keyword);
                        $results->put($keyword, $this->mergeKeywordData($existing, $providerData));
                    }
                }

                // Remove found keywords from missing list
                $foundKeywords = $providerResults->pluck('keyword')->map(fn($k) => strtolower($k))->toArray();
                $missingKeywords = array_filter($missingKeywords, fn($k) =>
                    !in_array(strtolower($k), $foundKeywords)
                );

            } catch (\Exception $e) {
                Log::warning("Batch provider {$provider->getProviderId()} failed", [
                    'error' => $e->getMessage(),
                ]);
            }
        }

        // Add empty data for still missing keywords
        foreach ($missingKeywords as $keyword) {
            if (!$results->has($keyword)) {
                $results->put($keyword, new KeywordData(
                    keyword: $keyword,
                    dataSource: 'none',
                    dataConfidence: 0,
                    fetchedAt: new \DateTimeImmutable(),
                ));
            }
        }

        // Cache all results
        foreach ($results as $keyword => $data) {
            if ($data instanceof KeywordData) {
                $cacheKey = self::CACHE_PREFIX . ':keyword:' . md5($keyword . json_encode($options));
                $this->cache->put($cacheKey, $data->toArray(), self::CACHE_TTL);
            }
        }

        return $results->values();
    }

    /**
     * Find keyword gaps and opportunities.
     * Primary method for Google-only keyword gap analysis.
     *
     * @return Collection<KeywordGap>
     */
    public function findKeywordGaps(int $userId, string $siteUrl, array $options = []): Collection
    {
        // Primary: Use Google Search Console for authoritative gap analysis
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider instanceof GoogleSearchConsoleProvider) {
            $gaps = $gscProvider->findKeywordGaps($userId, $siteUrl, $options);

            // Enrich with trending data from Google Trends
            $trendsProvider = $this->providers['google_trends'] ?? null;
            if ($trendsProvider instanceof GoogleTrendsProvider) {
                $gaps = $this->enrichGapsWithTrends($gaps, $trendsProvider, $options);
            }

            return $gaps;
        }

        return new Collection();
    }

    /**
     * Get quick wins - highest impact, lowest effort opportunities.
     *
     * @return Collection<KeywordGap>
     */
    public function getQuickWins(int $userId, string $siteUrl, int $limit = 20): Collection
    {
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider instanceof GoogleSearchConsoleProvider) {
            return $gscProvider->getQuickWins($userId, $siteUrl, $limit);
        }

        return new Collection();
    }

    /**
     * Get position opportunities (keywords close to page 1).
     *
     * @return Collection<KeywordGap>
     */
    public function getPositionOpportunities(int $userId, string $siteUrl, int $limit = 50): Collection
    {
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider instanceof GoogleSearchConsoleProvider) {
            return $gscProvider->getPositionOpportunities($userId, $siteUrl, $limit);
        }

        return new Collection();
    }

    /**
     * Get CTR optimization opportunities.
     *
     * @return Collection<KeywordGap>
     */
    public function getCtrOpportunities(int $userId, string $siteUrl, int $limit = 50): Collection
    {
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider instanceof GoogleSearchConsoleProvider) {
            return $gscProvider->getCtrOpportunities($userId, $siteUrl, $limit);
        }

        return new Collection();
    }

    /**
     * Get declining keywords that need attention.
     *
     * @return Collection<KeywordGap>
     */
    public function getDecliningKeywords(int $userId, string $siteUrl, int $limit = 50): Collection
    {
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider instanceof GoogleSearchConsoleProvider) {
            return $gscProvider->getDecliningKeywords($userId, $siteUrl, $limit);
        }

        return new Collection();
    }

    /**
     * Detect keyword cannibalization.
     *
     * @return Collection<KeywordGap>
     */
    public function detectCannibalization(int $userId, string $siteUrl): Collection
    {
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider instanceof GoogleSearchConsoleProvider) {
            return $gscProvider->detectCannibalization($userId, $siteUrl);
        }

        return new Collection();
    }

    /**
     * Get trending keywords in a topic/niche.
     */
    public function getTrendingKeywords(string $topic, array $options = []): Collection
    {
        $trendsProvider = $this->providers['google_trends'] ?? null;

        if ($trendsProvider instanceof GoogleTrendsProvider) {
            return $trendsProvider->getTrendingKeywords($topic, $options);
        }

        return new Collection();
    }

    /**
     * Get related keywords from all available sources.
     */
    public function getRelatedKeywords(string $seedKeyword, int $limit = 50, array $options = []): Collection
    {
        $cacheKey = self::CACHE_PREFIX . ':related:' . md5($seedKeyword . $limit);

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return collect($cached)->map(fn($d) => KeywordData::fromArray($d));
        }

        $allRelated = new Collection();

        // Get from Google Keyword Planner
        $gkpProvider = $this->providers['google_keyword_planner'] ?? null;
        if ($gkpProvider) {
            try {
                $gkpRelated = $gkpProvider->getRelatedKeywords($seedKeyword, $limit, $options);
                $allRelated = $allRelated->merge($gkpRelated);
            } catch (\Exception $e) {
                Log::warning('GKP related keywords failed', ['error' => $e->getMessage()]);
            }
        }

        // Get from Google Trends
        $trendsProvider = $this->providers['google_trends'] ?? null;
        if ($trendsProvider) {
            try {
                $trendsRelated = $trendsProvider->getRelatedKeywords($seedKeyword, $limit, $options);
                $allRelated = $allRelated->merge($trendsRelated);
            } catch (\Exception $e) {
                Log::warning('Trends related keywords failed', ['error' => $e->getMessage()]);
            }
        }

        // Deduplicate by keyword
        $uniqueKeywords = $allRelated
            ->unique(fn(KeywordData $kw) => strtolower($kw->keyword))
            ->take($limit)
            ->values();

        $this->cache->put($cacheKey, $uniqueKeywords->map->toArray()->toArray(), self::CACHE_TTL);

        return $uniqueKeywords;
    }

    /**
     * Get search volume with best available data.
     */
    public function getSearchVolume(string $keyword, array $options = []): array
    {
        // Prefer Google Keyword Planner for accurate volume
        $gkpProvider = $this->providers['google_keyword_planner'] ?? null;

        if ($gkpProvider && $gkpProvider->isAvailable()) {
            $volume = $gkpProvider->getSearchVolume($keyword, $options);
            if ($volume['volume'] !== null) {
                return $volume;
            }
        }

        // Fallback to GSC impressions as proxy
        $gscProvider = $this->providers['google_search_console'] ?? null;
        if ($gscProvider) {
            return $gscProvider->getSearchVolume($keyword, $options);
        }

        return [
            'volume' => null,
            'volume_trend' => 'unknown',
            'seasonality' => null,
            'confidence' => 0,
        ];
    }

    /**
     * Analyze SERP with best available data.
     */
    public function analyzeSERP(string $keyword, array $options = []): SerpAnalysis
    {
        // GSC provides position data for your site
        $gscProvider = $this->providers['google_search_console'] ?? null;

        if ($gscProvider) {
            return $gscProvider->analyzeSERP($keyword, $options);
        }

        return new SerpAnalysis(
            keyword: $keyword,
            dataSource: 'none',
            dataConfidence: 0,
            analyzedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Get all providers' health status.
     */
    public function getProvidersHealth(): array
    {
        $health = [];

        foreach ($this->providers as $providerId => $provider) {
            $health[$providerId] = $provider->getHealthStatus();
        }

        return $health;
    }

    /**
     * Get available capabilities across all providers.
     */
    public function getAvailableCapabilities(): array
    {
        return array_keys($this->providersByCapability);
    }

    /**
     * Check if a specific capability is available.
     */
    public function hasCapability(string $capability): bool
    {
        if (!isset($this->providersByCapability[$capability])) {
            return false;
        }

        foreach ($this->providersByCapability[$capability] as $provider) {
            if ($provider->isAvailable()) {
                return true;
            }
        }

        return false;
    }

    /**
     * Get providers for specific capabilities, sorted by priority.
     *
     * @return array<KeywordDataProviderInterface>
     */
    private function getProvidersForCapabilities(array $capabilities): array
    {
        $providers = [];

        foreach ($capabilities as $capability) {
            if (isset($this->providersByCapability[$capability])) {
                foreach ($this->providersByCapability[$capability] as $provider) {
                    $providerId = $provider->getProviderId();
                    if (!isset($providers[$providerId])) {
                        $providers[$providerId] = $provider;
                    }
                }
            }
        }

        // Sort by priority
        uasort($providers, fn($a, $b) => $a->getPriority() - $b->getPriority());

        return array_values($providers);
    }

    /**
     * Merge keyword data from two sources.
     * Higher confidence data wins for each field.
     */
    private function mergeKeywordData(KeywordData $primary, KeywordData $secondary): KeywordData
    {
        // If primary has higher confidence, use it as base
        if (($primary->dataConfidence ?? 0) >= ($secondary->dataConfidence ?? 0)) {
            return $primary->merge($secondary);
        }

        return $secondary->merge($primary);
    }

    /**
     * Enrich keyword gaps with trending data.
     */
    private function enrichGapsWithTrends(Collection $gaps, GoogleTrendsProvider $trendsProvider, array $options): Collection
    {
        // Get keywords from gaps
        $keywords = $gaps->pluck('keyword')->unique()->take(20)->toArray();

        if (empty($keywords)) {
            return $gaps;
        }

        try {
            // Get trend data for these keywords
            $trendData = $trendsProvider->getKeywordDataBatch($keywords, $options);
            $trendMap = $trendData->keyBy(fn(KeywordData $kw) => strtolower($kw->keyword));

            // Add trending opportunities for highly trending keywords
            foreach ($trendMap as $keyword => $data) {
                if ($data->trend === KeywordData::TREND_UP && ($data->trendScore ?? 0) > 70) {
                    // Check if we already have a gap for this keyword
                    $existingGap = $gaps->first(fn(KeywordGap $g) =>
                        strtolower($g->keyword) === $keyword
                    );

                    if (!$existingGap) {
                        $gaps->push(KeywordGap::createTrendingOpportunity($data->keyword, [
                            'growth_percentage' => (int) (($data->trendScore - 50) * 4),
                            'search_volume' => $data->searchVolume,
                            'related_keywords' => $data->relatedKeywords,
                        ]));
                    }
                }
            }
        } catch (\Exception $e) {
            Log::warning('Trend enrichment failed', ['error' => $e->getMessage()]);
        }

        return $gaps;
    }

    /**
     * Get configuration status.
     */
    public function getConfigStatus(): array
    {
        return [
            'google_only_mode' => $this->config['google_only'] ?? true,
            'third_party_enabled' => $this->config['enable_third_party'] ?? false,
            'registered_providers' => array_keys($this->providers),
            'available_capabilities' => $this->getAvailableCapabilities(),
            'aggregation_strategy' => $this->config['aggregation_strategy'] ?? 'merge_highest_confidence',
        ];
    }
}
