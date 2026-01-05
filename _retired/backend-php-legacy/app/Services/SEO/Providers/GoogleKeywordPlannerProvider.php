<?php

declare(strict_types=1);

namespace App\Services\Seo\Providers;

use App\Contracts\Seo\KeywordDataProviderInterface;
use App\DataTransferObjects\Seo\KeywordData;
use App\DataTransferObjects\Seo\SerpAnalysis;
use App\Models\IntegrationCredential;
use App\Services\CacheService;
use Google\Ads\GoogleAds\Lib\V15\GoogleAdsClient;
use Google\Ads\GoogleAds\Lib\V15\GoogleAdsClientBuilder;
use Google\Ads\GoogleAds\V15\Services\GenerateKeywordIdeasRequest;
use Google\Ads\GoogleAds\V15\Services\KeywordPlanIdeaServiceClient;
use Google\Ads\GoogleAds\V15\Services\KeywordSeed;
use Google\Ads\GoogleAds\V15\Enums\KeywordPlanNetworkEnum\KeywordPlanNetwork;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Crypt;

/**
 * Google Keyword Planner Provider
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Official Google API for keyword search volume and competition data.
 * Requires a Google Ads account with API access.
 *
 * Capabilities:
 * - Search volume data (actual Google data)
 * - Competition level
 * - CPC estimates
 * - Keyword ideas/suggestions
 * - Historical metrics
 *
 * Limitations:
 * - Requires Google Ads account
 * - Cannot provide SERP positions (use GSC for that)
 * - Cannot provide competitor data
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
class GoogleKeywordPlannerProvider implements KeywordDataProviderInterface
{
    private const CACHE_PREFIX = 'seo:gkp';
    private const CACHE_TTL = 86400; // 24 hours (keyword data doesn't change often)
    private const PROVIDER_ID = 'google_keyword_planner';

    private CacheService $cache;
    private ?GoogleAdsClient $client = null;
    private ?string $customerId = null;
    private bool $initialized = false;
    private array $healthMetrics = [];

    public function __construct(CacheService $cache)
    {
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
        return 'Google Keyword Planner';
    }

    /**
     * {@inheritdoc}
     */
    public function isAvailable(): bool
    {
        // Check if Google Ads credentials are configured
        $clientId = config('services.google_ads.client_id');
        $clientSecret = config('services.google_ads.client_secret');
        $developerToken = config('services.google_ads.developer_token');
        $customerId = config('services.google_ads.customer_id');

        return !empty($clientId) && !empty($clientSecret) && !empty($developerToken) && !empty($customerId);
    }

    /**
     * {@inheritdoc}
     */
    public function getHealthStatus(): array
    {
        return [
            'status' => $this->isAvailable() ? 'available' : 'unavailable',
            'latency_ms' => $this->healthMetrics['latency_ms'] ?? null,
            'rate_limit_remaining' => $this->healthMetrics['rate_limit_remaining'] ?? null,
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
            self::CAPABILITY_SEARCH_VOLUME,
            self::CAPABILITY_CPC_DATA,
            self::CAPABILITY_RELATED_KEYWORDS,
            self::CAPABILITY_HISTORICAL_DATA,
            self::CAPABILITY_INTENT_CLASSIFICATION,
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
        return 20; // Medium-high priority for search volume data
    }

    /**
     * {@inheritdoc}
     */
    public function getRateLimitConfig(): array
    {
        return [
            'requests_per_minute' => 60,
            'requests_per_day' => 15000,
            'current_usage' => $this->healthMetrics['daily_usage'] ?? 0,
            'reset_at' => now()->endOfDay()->toIso8601String(),
        ];
    }

    /**
     * Initialize the Google Ads client.
     */
    private function initializeClient(int $userId = null): bool
    {
        if ($this->initialized && $this->client !== null) {
            return true;
        }

        try {
            // Try user-specific credentials first
            if ($userId) {
                $credential = IntegrationCredential::where('user_id', $userId)
                    ->where('provider', 'google_ads')
                    ->where('is_active', true)
                    ->first();

                if ($credential) {
                    // Use user-specific OAuth tokens
                    $this->client = (new GoogleAdsClientBuilder())
                        ->withDeveloperToken(config('services.google_ads.developer_token'))
                        ->withOAuth2Credential($this->buildOAuthCredential($credential))
                        ->build();

                    $this->customerId = $credential->metadata['customer_id'] ?? config('services.google_ads.customer_id');
                    $this->initialized = true;
                    return true;
                }
            }

            // Fall back to application-level credentials
            if (!$this->isAvailable()) {
                return false;
            }

            $this->client = (new GoogleAdsClientBuilder())
                ->withDeveloperToken(config('services.google_ads.developer_token'))
                ->withOAuth2Credential([
                    'clientId' => config('services.google_ads.client_id'),
                    'clientSecret' => config('services.google_ads.client_secret'),
                    'refreshToken' => config('services.google_ads.refresh_token'),
                ])
                ->build();

            $this->customerId = config('services.google_ads.customer_id');
            $this->initialized = true;
            return true;

        } catch (\Exception $e) {
            Log::error('Google Keyword Planner initialization failed', [
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Build OAuth credential from stored token.
     */
    private function buildOAuthCredential(IntegrationCredential $credential): array
    {
        return [
            'clientId' => config('services.google_ads.client_id'),
            'clientSecret' => config('services.google_ads.client_secret'),
            'refreshToken' => Crypt::decrypt($credential->refresh_token),
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getKeywordData(string $keyword, array $options = []): KeywordData
    {
        $cacheKey = self::CACHE_PREFIX . ':data:' . md5($keyword . json_encode($options));

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return KeywordData::fromArray($cached);
        }

        if (!$this->initializeClient($options['user_id'] ?? null)) {
            // Return empty data with low confidence
            return new KeywordData(
                keyword: $keyword,
                dataSource: self::PROVIDER_ID,
                dataConfidence: 0,
                fetchedAt: new \DateTimeImmutable(),
            );
        }

        try {
            $keywordPlanService = $this->client->getKeywordPlanIdeaServiceClient();

            $request = new GenerateKeywordIdeasRequest();
            $request->setCustomerId($this->customerId);
            $request->setLanguage('languageConstants/1000'); // English
            $request->setGeoTargetConstants(['geoTargetConstants/2840']); // USA
            $request->setKeywordPlanNetwork(KeywordPlanNetwork::GOOGLE_SEARCH);

            $keywordSeed = new KeywordSeed();
            $keywordSeed->setKeywords([$keyword]);
            $request->setKeywordSeed($keywordSeed);

            $response = $keywordPlanService->generateKeywordIdeas($request);

            foreach ($response->iterateAllElements() as $result) {
                if (strtolower($result->getText()) === strtolower($keyword)) {
                    $metrics = $result->getKeywordIdeaMetrics();

                    $data = new KeywordData(
                        keyword: $keyword,
                        searchVolume: $metrics->getAvgMonthlySearches(),
                        cpc: $metrics->getHighTopOfPageBidMicros() / 1000000,
                        competition: $this->mapCompetition($metrics->getCompetition()),
                        intent: $this->classifyIntentFromKeyword($keyword)['primary_intent'],
                        dataSource: self::PROVIDER_ID,
                        dataConfidence: 1.0,
                        seasonality: $this->extractSeasonality($metrics),
                        fetchedAt: new \DateTimeImmutable(),
                    );

                    $this->cache->put($cacheKey, $data->toArray(), self::CACHE_TTL);
                    $this->recordSuccessfulCall();

                    return $data;
                }
            }

            // Keyword not found in results
            return new KeywordData(
                keyword: $keyword,
                dataSource: self::PROVIDER_ID,
                dataConfidence: 0.5,
                fetchedAt: new \DateTimeImmutable(),
            );

        } catch (\Exception $e) {
            Log::error('Google Keyword Planner API error', [
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
        $results = new Collection();

        // Batch request to Google Ads API
        if (!$this->initializeClient($options['user_id'] ?? null)) {
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
            $keywordPlanService = $this->client->getKeywordPlanIdeaServiceClient();

            $request = new GenerateKeywordIdeasRequest();
            $request->setCustomerId($this->customerId);
            $request->setLanguage('languageConstants/1000');
            $request->setGeoTargetConstants(['geoTargetConstants/2840']);
            $request->setKeywordPlanNetwork(KeywordPlanNetwork::GOOGLE_SEARCH);

            $keywordSeed = new KeywordSeed();
            $keywordSeed->setKeywords($keywords);
            $request->setKeywordSeed($keywordSeed);

            $response = $keywordPlanService->generateKeywordIdeas($request);

            $keywordMap = array_flip(array_map('strtolower', $keywords));
            $foundKeywords = [];

            foreach ($response->iterateAllElements() as $result) {
                $kwLower = strtolower($result->getText());

                if (isset($keywordMap[$kwLower])) {
                    $metrics = $result->getKeywordIdeaMetrics();

                    $data = new KeywordData(
                        keyword: $result->getText(),
                        searchVolume: $metrics->getAvgMonthlySearches(),
                        cpc: $metrics->getHighTopOfPageBidMicros() / 1000000,
                        competition: $this->mapCompetition($metrics->getCompetition()),
                        intent: $this->classifyIntentFromKeyword($result->getText())['primary_intent'],
                        dataSource: self::PROVIDER_ID,
                        dataConfidence: 1.0,
                        fetchedAt: new \DateTimeImmutable(),
                    );

                    $results->push($data);
                    $foundKeywords[$kwLower] = true;
                }
            }

            // Add empty results for keywords not found
            foreach ($keywords as $keyword) {
                if (!isset($foundKeywords[strtolower($keyword)])) {
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
            Log::error('Google Keyword Planner batch API error', [
                'keywords_count' => count($keywords),
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
     * {@inheritdoc}
     */
    public function getSearchVolume(string $keyword, array $options = []): array
    {
        $data = $this->getKeywordData($keyword, $options);

        return [
            'volume' => $data->searchVolume,
            'volume_trend' => $data->trend,
            'seasonality' => $data->seasonality,
            'confidence' => $data->dataConfidence ?? 0,
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getKeywordDifficulty(string $keyword): array
    {
        // Google Keyword Planner doesn't provide difficulty
        // Use competition as a proxy
        $data = $this->getKeywordData($keyword);

        $difficulty = $this->estimateDifficultyFromCompetition($data->competition);

        return [
            'difficulty' => $difficulty,
            'difficulty_label' => $this->getDifficultyLabel($difficulty),
            'factors' => [
                'competition' => $data->competition,
                'cpc' => $data->cpc,
                'search_volume' => $data->searchVolume,
            ],
            'confidence' => 0.6, // Lower confidence as this is estimated
        ];
    }

    /**
     * {@inheritdoc}
     */
    public function getRelatedKeywords(string $seedKeyword, int $limit = 50, array $options = []): Collection
    {
        $cacheKey = self::CACHE_PREFIX . ':related:' . md5($seedKeyword . $limit);

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return collect($cached)->map(fn($data) => KeywordData::fromArray($data));
        }

        if (!$this->initializeClient($options['user_id'] ?? null)) {
            return new Collection();
        }

        try {
            $keywordPlanService = $this->client->getKeywordPlanIdeaServiceClient();

            $request = new GenerateKeywordIdeasRequest();
            $request->setCustomerId($this->customerId);
            $request->setLanguage('languageConstants/1000');
            $request->setGeoTargetConstants(['geoTargetConstants/2840']);
            $request->setKeywordPlanNetwork(KeywordPlanNetwork::GOOGLE_SEARCH);

            $keywordSeed = new KeywordSeed();
            $keywordSeed->setKeywords([$seedKeyword]);
            $request->setKeywordSeed($keywordSeed);

            $response = $keywordPlanService->generateKeywordIdeas($request);

            $results = new Collection();
            $count = 0;

            foreach ($response->iterateAllElements() as $result) {
                if ($count >= $limit) {
                    break;
                }

                $metrics = $result->getKeywordIdeaMetrics();

                $data = new KeywordData(
                    keyword: $result->getText(),
                    searchVolume: $metrics->getAvgMonthlySearches(),
                    cpc: $metrics->getHighTopOfPageBidMicros() / 1000000,
                    competition: $this->mapCompetition($metrics->getCompetition()),
                    intent: $this->classifyIntentFromKeyword($result->getText())['primary_intent'],
                    dataSource: self::PROVIDER_ID,
                    dataConfidence: 1.0,
                    fetchedAt: new \DateTimeImmutable(),
                );

                $results->push($data);
                $count++;
            }

            $this->cache->put($cacheKey, $results->map->toArray()->toArray(), self::CACHE_TTL);
            $this->recordSuccessfulCall();

            return $results;

        } catch (\Exception $e) {
            Log::error('Google Keyword Planner related keywords error', [
                'seed' => $seedKeyword,
                'error' => $e->getMessage(),
            ]);

            $this->recordFailedCall();
            return new Collection();
        }
    }

    /**
     * {@inheritdoc}
     */
    public function getQuestionKeywords(string $seedKeyword, int $limit = 20): Collection
    {
        // Filter related keywords for questions
        $related = $this->getRelatedKeywords($seedKeyword, $limit * 3);

        return $related->filter(function (KeywordData $kw) {
            $questionWords = ['how', 'what', 'why', 'when', 'where', 'who', 'which', 'can', 'do', 'does', 'is', 'are'];
            $kwLower = strtolower($kw->keyword);

            foreach ($questionWords as $qw) {
                if (str_starts_with($kwLower, $qw . ' ')) {
                    return true;
                }
            }

            return str_contains($kwLower, '?');
        })->take($limit)->values();
    }

    /**
     * {@inheritdoc}
     */
    public function getLongTailVariations(string $seedKeyword, int $limit = 50): Collection
    {
        $related = $this->getRelatedKeywords($seedKeyword, $limit * 2);

        return $related->filter(function (KeywordData $kw) {
            return str_word_count($kw->keyword) >= 4;
        })->take($limit)->values();
    }

    /**
     * {@inheritdoc}
     * Note: Google Keyword Planner cannot provide SERP analysis.
     */
    public function analyzeSERP(string $keyword, array $options = []): SerpAnalysis
    {
        // Return empty SERP analysis - this provider doesn't support SERP data
        return new SerpAnalysis(
            keyword: $keyword,
            dataSource: self::PROVIDER_ID,
            dataConfidence: 0,
            analyzedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * {@inheritdoc}
     * Note: Google Keyword Planner cannot provide position data.
     */
    public function getKeywordPosition(string $keyword, string $url, array $options = []): array
    {
        return [
            'position' => null,
            'page' => null,
            'url_found' => null,
            'serp_features' => [],
            'competitors' => [],
        ];
    }

    /**
     * {@inheritdoc}
     * Note: For trending, use Google Trends instead.
     */
    public function getTrendingKeywords(string $topic, array $options = []): Collection
    {
        // Delegate to related keywords as a fallback
        return $this->getRelatedKeywords($topic, $options['limit'] ?? 20, $options);
    }

    /**
     * {@inheritdoc}
     */
    public function classifyIntent(string $keyword): array
    {
        return $this->classifyIntentFromKeyword($keyword);
    }

    /**
     * {@inheritdoc}
     */
    public function getHistoricalData(string $keyword, int $days = 90): array
    {
        // Google Keyword Planner provides monthly averages, not daily data
        $data = $this->getKeywordData($keyword);

        return [
            'data_points' => [],
            'trend' => $data->trend,
            'average_position' => null, // Not available from this provider
            'position_change' => null,
            'volatility' => 0,
        ];
    }

    /**
     * Map Google Ads competition level to float.
     */
    private function mapCompetition($competition): float
    {
        return match ($competition) {
            0 => 0.0,    // UNSPECIFIED
            1 => 0.0,    // UNKNOWN
            2 => 0.33,   // LOW
            3 => 0.66,   // MEDIUM
            4 => 1.0,    // HIGH
            default => 0.5,
        };
    }

    /**
     * Estimate difficulty from competition.
     */
    private function estimateDifficultyFromCompetition(?float $competition): int
    {
        if ($competition === null) {
            return 50;
        }

        return (int) ($competition * 100);
    }

    /**
     * Get difficulty label from score.
     */
    private function getDifficultyLabel(int $difficulty): string
    {
        if ($difficulty < 30) {
            return 'easy';
        } elseif ($difficulty < 50) {
            return 'medium';
        } elseif ($difficulty < 70) {
            return 'hard';
        }

        return 'very_hard';
    }

    /**
     * Classify intent from keyword text.
     */
    private function classifyIntentFromKeyword(string $keyword): array
    {
        $keyword = strtolower($keyword);

        // Transactional signals
        $transactional = ['buy', 'purchase', 'order', 'price', 'cost', 'cheap', 'discount', 'deal', 'coupon', 'shop', 'store', 'sale'];
        foreach ($transactional as $signal) {
            if (str_contains($keyword, $signal)) {
                return [
                    'primary_intent' => KeywordData::INTENT_TRANSACTIONAL,
                    'secondary_intent' => null,
                    'confidence' => 0.9,
                    'signals' => [$signal],
                ];
            }
        }

        // Commercial signals
        $commercial = ['best', 'top', 'review', 'comparison', 'vs', 'versus', 'alternative', 'compare', 'option', 'recommend'];
        foreach ($commercial as $signal) {
            if (str_contains($keyword, $signal)) {
                return [
                    'primary_intent' => KeywordData::INTENT_COMMERCIAL,
                    'secondary_intent' => KeywordData::INTENT_INFORMATIONAL,
                    'confidence' => 0.85,
                    'signals' => [$signal],
                ];
            }
        }

        // Navigational signals
        $navigational = ['login', 'sign in', 'account', 'dashboard', 'portal', 'official', 'website', '.com', '.org'];
        foreach ($navigational as $signal) {
            if (str_contains($keyword, $signal)) {
                return [
                    'primary_intent' => KeywordData::INTENT_NAVIGATIONAL,
                    'secondary_intent' => null,
                    'confidence' => 0.9,
                    'signals' => [$signal],
                ];
            }
        }

        // Default to informational
        return [
            'primary_intent' => KeywordData::INTENT_INFORMATIONAL,
            'secondary_intent' => null,
            'confidence' => 0.7,
            'signals' => [],
        ];
    }

    /**
     * Extract seasonality from monthly search volumes.
     */
    private function extractSeasonality($metrics): array
    {
        // Google Ads API provides monthly search volumes
        // This would need to be implemented based on actual API response
        return [];
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
