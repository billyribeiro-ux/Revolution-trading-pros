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
use App\Services\CacheService;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Log;

/**
 * SEO Data Source Manager
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Intelligent orchestration layer for all SEO data providers.
 * Implements a Google-first strategy with optional third-party fallbacks.
 *
 * Architecture Principles:
 * - Provider Abstraction (Strategy Pattern)
 * - Intelligent Data Aggregation
 * - Graceful Degradation
 * - Configurable Priority
 * - Comprehensive Caching
 * - Health Monitoring
 *
 * Data Source Priority (Google-First):
 * 1. Google Search Console (YOUR site's authoritative data)
 * 2. Google Keyword Planner (Official search volume)
 * 3. Google Trends (Trending & related keywords)
 * 4. SerpAPI (Optional third-party fallback - disabled by default)
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
class SeoDataSourceManager
{
    private const CACHE_PREFIX = 'seo:manager';
    private const CACHE_TTL = 1800; // 30 minutes

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
        CacheService $cache
    ) {
        $this->cache = $cache;
        $this->loadConfig();

        // Register Google providers (always enabled)
        $this->registerProvider($gscProvider);
        $this->registerProvider($gkpProvider);
        $this->registerProvider($trendsProvider);

        // Register third-party providers if enabled
        if ($this->config['enable_third_party'] ?? false) {
            $this->registerThirdPartyProviders();
        }
    }

    /**
     * Load configuration from config file.
     */
    private function loadConfig(): void
    {
        $this->config = config('seo.data_sources', [
            'enable_third_party' => false,
            'google_only' => true,
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
     * Register third-party providers (SerpAPI, etc.)
     * Only called if explicitly enabled in config.
     */
    private function registerThirdPartyProviders(): void
    {
        // This would be implemented to load SerpAPI provider
        // Keeping it separate for Google-only compliance
        Log::info('Third-party SEO providers enabled');
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
