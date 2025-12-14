<?php

declare(strict_types=1);

namespace App\Services\Seo\Providers;

use App\Contracts\Seo\KeywordDataProviderInterface;
use App\DataTransferObjects\Seo\KeywordData;
use App\DataTransferObjects\Seo\SerpAnalysis;
use App\Services\CacheService;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

/**
 * SerpAPI Provider
 *
 * Apple Principal Engineer ICT11+ Implementation
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive SERP data provider using SerpAPI for:
 * - Full SERP analysis with all features
 * - Competitor position tracking
 * - People Also Ask extraction
 * - Featured snippet detection
 * - Knowledge Graph data
 * - Local pack results
 * - Shopping, Images, Videos, News results
 * - Related searches
 *
 * @version 1.0.0
 * @level ICT11+ Principal Engineer
 */
class SerpApiProvider implements KeywordDataProviderInterface
{
    private const PROVIDER_ID = 'serpapi';
    private const PRIORITY = 40;
    private const CACHE_PREFIX = 'seo:serpapi';
    private const CACHE_TTL = 21600; // 6 hours

    private const API_BASE_URL = 'https://serpapi.com/search';

    private CacheService $cache;
    private array $config;

    public function __construct(CacheService $cache)
    {
        $this->cache = $cache;
        $this->config = config('seo.third_party.serpapi', []);
    }

    public function getProviderId(): string
    {
        return self::PROVIDER_ID;
    }

    public function isAvailable(): bool
    {
        return !empty($this->config['api_key']) && ($this->config['enabled'] ?? false);
    }

    public function getHealthStatus(): array
    {
        if (!$this->isAvailable()) {
            return [
                'status' => 'unavailable',
                'reason' => empty($this->config['api_key']) ? 'API key not configured' : 'Provider disabled',
            ];
        }

        // Check API connectivity
        try {
            $response = Http::timeout(5)->get(self::API_BASE_URL, [
                'api_key' => $this->config['api_key'],
                'q' => 'test',
                'engine' => 'google',
                'num' => 1,
            ]);

            return [
                'status' => $response->successful() ? 'healthy' : 'degraded',
                'response_time_ms' => $response->transferStats?->getTransferTime() * 1000 ?? null,
            ];
        } catch (\Exception $e) {
            return [
                'status' => 'unhealthy',
                'error' => $e->getMessage(),
            ];
        }
    }

    public function getCapabilities(): array
    {
        return [
            self::CAPABILITY_SERP_POSITION,
            self::CAPABILITY_SERP_FEATURES,
            self::CAPABILITY_COMPETITOR_DATA,
            self::CAPABILITY_RELATED_KEYWORDS,
            self::CAPABILITY_PEOPLE_ALSO_ASK,
            self::CAPABILITY_FEATURED_SNIPPET,
            self::CAPABILITY_LOCAL_PACK,
            self::CAPABILITY_KNOWLEDGE_GRAPH,
            self::CAPABILITY_SEARCH_VOLUME,  // Estimated from SERP features
        ];
    }

    public function getPriority(): int
    {
        return self::PRIORITY;
    }

    public function getRateLimitConfig(): array
    {
        return [
            'requests_per_minute' => $this->config['rate_limit'] ?? 30,
            'burst_limit' => 10,
            'cooldown_seconds' => 60,
        ];
    }

    /**
     * Get comprehensive keyword data from SERP.
     */
    public function getKeywordData(string $keyword, array $options = []): KeywordData
    {
        $cacheKey = self::CACHE_PREFIX . ':keyword:' . md5($keyword . json_encode($options));

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return KeywordData::fromArray($cached);
        }

        $serpData = $this->fetchSerpData($keyword, $options);

        $keywordData = $this->transformSerpToKeywordData($keyword, $serpData);

        $this->cache->put($cacheKey, $keywordData->toArray(), self::CACHE_TTL);

        return $keywordData;
    }

    /**
     * Batch keyword data retrieval.
     */
    public function getKeywordDataBatch(array $keywords, array $options = []): Collection
    {
        $results = new Collection();

        foreach ($keywords as $keyword) {
            try {
                $results->push($this->getKeywordData($keyword, $options));
            } catch (\Exception $e) {
                Log::warning("SerpAPI batch failed for keyword: {$keyword}", [
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return $results;
    }

    /**
     * Get search volume (estimated from SERP features).
     */
    public function getSearchVolume(string $keyword, array $options = []): array
    {
        $serpData = $this->fetchSerpData($keyword, $options);

        // Estimate search volume from SERP features
        $estimate = $this->estimateSearchVolume($serpData);

        return [
            'volume' => $estimate['volume'],
            'volume_trend' => $estimate['trend'],
            'seasonality' => $estimate['seasonality'],
            'confidence' => $estimate['confidence'],
        ];
    }

    /**
     * Get related keywords from People Also Ask and Related Searches.
     */
    public function getRelatedKeywords(string $seedKeyword, int $limit = 50, array $options = []): Collection
    {
        $serpData = $this->fetchSerpData($seedKeyword, $options);

        $related = new Collection();

        // Extract from Related Searches
        if (!empty($serpData['related_searches'])) {
            foreach ($serpData['related_searches'] as $search) {
                $related->push(new KeywordData(
                    keyword: $search['query'] ?? $search['block_position'] ?? '',
                    searchVolume: null,
                    difficulty: null,
                    dataSource: self::PROVIDER_ID,
                    dataConfidence: 70,
                    fetchedAt: new \DateTimeImmutable(),
                    relatedTo: $seedKeyword,
                ));
            }
        }

        // Extract from People Also Ask
        if (!empty($serpData['related_questions'])) {
            foreach ($serpData['related_questions'] as $question) {
                $related->push(new KeywordData(
                    keyword: $question['question'] ?? '',
                    searchVolume: null,
                    difficulty: null,
                    intent: 'informational',
                    dataSource: self::PROVIDER_ID,
                    dataConfidence: 75,
                    fetchedAt: new \DateTimeImmutable(),
                    relatedTo: $seedKeyword,
                ));
            }
        }

        return $related->take($limit);
    }

    /**
     * Analyze SERP with full feature detection.
     */
    public function analyzeSERP(string $keyword, array $options = []): SerpAnalysis
    {
        $cacheKey = self::CACHE_PREFIX . ':serp:' . md5($keyword . json_encode($options));

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return SerpAnalysis::fromSerpApi($cached);
        }

        $serpData = $this->fetchSerpData($keyword, $options);

        $analysis = SerpAnalysis::fromSerpApi([
            'keyword' => $keyword,
            'organic_results' => $serpData['organic_results'] ?? [],
            'related_questions' => $serpData['related_questions'] ?? [],
            'related_searches' => $serpData['related_searches'] ?? [],
            'knowledge_graph' => $serpData['knowledge_graph'] ?? null,
            'answer_box' => $serpData['answer_box'] ?? null,
            'local_results' => $serpData['local_results'] ?? null,
            'shopping_results' => $serpData['shopping_results'] ?? [],
            'images_results' => $serpData['images_results'] ?? [],
            'videos' => $serpData['inline_videos'] ?? [],
            'news_results' => $serpData['news_results'] ?? [],
            'top_stories' => $serpData['top_stories'] ?? [],
            'search_metadata' => $serpData['search_metadata'] ?? [],
            'search_information' => $serpData['search_information'] ?? [],
        ]);

        $this->cache->put($cacheKey, $serpData, self::CACHE_TTL);

        return $analysis;
    }

    /**
     * Get trending keywords (from related searches).
     */
    public function getTrendingKeywords(string $topic, array $options = []): Collection
    {
        return $this->getRelatedKeywords($topic, 20, $options);
    }

    /**
     * Get competitor data from SERP.
     */
    public function getCompetitorData(string $keyword, array $options = []): array
    {
        $serpData = $this->fetchSerpData($keyword, $options);

        $competitors = [];

        if (!empty($serpData['organic_results'])) {
            foreach ($serpData['organic_results'] as $position => $result) {
                $domain = parse_url($result['link'] ?? '', PHP_URL_HOST);

                $competitors[] = [
                    'position' => $result['position'] ?? ($position + 1),
                    'url' => $result['link'] ?? '',
                    'domain' => $domain,
                    'title' => $result['title'] ?? '',
                    'snippet' => $result['snippet'] ?? '',
                    'displayed_link' => $result['displayed_link'] ?? '',
                    'cached_page_link' => $result['cached_page_link'] ?? null,
                    'rich_snippet' => $result['rich_snippet'] ?? null,
                    'sitelinks' => $result['sitelinks'] ?? null,
                    'date' => $result['date'] ?? null,
                    'thumbnail' => $result['thumbnail'] ?? null,
                ];
            }
        }

        return [
            'keyword' => $keyword,
            'total_results' => $serpData['search_information']['total_results'] ?? 0,
            'time_taken' => $serpData['search_information']['time_taken_displayed'] ?? null,
            'competitors' => $competitors,
            'featured_snippet' => $serpData['answer_box'] ?? null,
            'knowledge_graph' => $serpData['knowledge_graph'] ?? null,
        ];
    }

    /**
     * Get People Also Ask questions.
     */
    public function getPeopleAlsoAsk(string $keyword, array $options = []): array
    {
        $serpData = $this->fetchSerpData($keyword, $options);

        $questions = [];

        if (!empty($serpData['related_questions'])) {
            foreach ($serpData['related_questions'] as $question) {
                $questions[] = [
                    'question' => $question['question'] ?? '',
                    'snippet' => $question['snippet'] ?? '',
                    'title' => $question['title'] ?? '',
                    'link' => $question['link'] ?? '',
                    'displayed_link' => $question['displayed_link'] ?? '',
                    'source_logo' => $question['source_logo'] ?? null,
                ];
            }
        }

        return $questions;
    }

    /**
     * Get featured snippet data.
     */
    public function getFeaturedSnippet(string $keyword, array $options = []): ?array
    {
        $serpData = $this->fetchSerpData($keyword, $options);

        if (empty($serpData['answer_box'])) {
            return null;
        }

        $answerBox = $serpData['answer_box'];

        return [
            'type' => $answerBox['type'] ?? 'unknown',
            'title' => $answerBox['title'] ?? null,
            'snippet' => $answerBox['snippet'] ?? $answerBox['answer'] ?? null,
            'snippet_highlighted_words' => $answerBox['snippet_highlighted_words'] ?? [],
            'link' => $answerBox['link'] ?? null,
            'displayed_link' => $answerBox['displayed_link'] ?? null,
            'date' => $answerBox['date'] ?? null,
            'list' => $answerBox['list'] ?? null,
            'table' => $answerBox['table'] ?? null,
        ];
    }

    /**
     * Get local pack results.
     */
    public function getLocalPack(string $keyword, array $options = []): array
    {
        $serpData = $this->fetchSerpData($keyword, array_merge($options, [
            'location' => $options['location'] ?? $this->config['locations']['default'] ?? 'United States',
        ]));

        if (empty($serpData['local_results'])) {
            return [];
        }

        $localResults = $serpData['local_results'];

        return [
            'places' => array_map(function ($place) {
                return [
                    'position' => $place['position'] ?? null,
                    'title' => $place['title'] ?? '',
                    'place_id' => $place['place_id'] ?? null,
                    'data_id' => $place['data_id'] ?? null,
                    'data_cid' => $place['data_cid'] ?? null,
                    'reviews_link' => $place['reviews_link'] ?? null,
                    'photos_link' => $place['photos_link'] ?? null,
                    'gps_coordinates' => $place['gps_coordinates'] ?? null,
                    'address' => $place['address'] ?? '',
                    'phone' => $place['phone'] ?? null,
                    'website' => $place['website'] ?? null,
                    'rating' => $place['rating'] ?? null,
                    'reviews' => $place['reviews'] ?? null,
                    'type' => $place['type'] ?? null,
                    'hours' => $place['hours'] ?? null,
                    'service_options' => $place['service_options'] ?? null,
                    'thumbnail' => $place['thumbnail'] ?? null,
                ];
            }, $localResults['places'] ?? []),
            'more_locations_link' => $localResults['more_locations_link'] ?? null,
        ];
    }

    /**
     * Get knowledge graph data.
     */
    public function getKnowledgeGraph(string $keyword, array $options = []): ?array
    {
        $serpData = $this->fetchSerpData($keyword, $options);

        if (empty($serpData['knowledge_graph'])) {
            return null;
        }

        $kg = $serpData['knowledge_graph'];

        return [
            'title' => $kg['title'] ?? null,
            'type' => $kg['type'] ?? null,
            'description' => $kg['description'] ?? null,
            'source' => $kg['source'] ?? null,
            'kgmid' => $kg['kgmid'] ?? null,
            'knowledge_graph_search_link' => $kg['knowledge_graph_search_link'] ?? null,
            'serpapi_knowledge_graph_search_link' => $kg['serpapi_knowledge_graph_search_link'] ?? null,
            'image' => $kg['header_images'][0]['image'] ?? $kg['image'] ?? null,
            'attributes' => $kg['known_attributes'] ?? [],
            'profiles' => $kg['profiles'] ?? [],
            'people_also_search_for' => $kg['people_also_search_for'] ?? [],
            'see_results_about' => $kg['see_results_about'] ?? [],
        ];
    }

    /**
     * Fetch raw SERP data from API.
     */
    private function fetchSerpData(string $keyword, array $options = []): array
    {
        $cacheKey = self::CACHE_PREFIX . ':raw:' . md5($keyword . json_encode($options));

        $cached = $this->cache->get($cacheKey);
        if ($cached) {
            return $cached;
        }

        $params = [
            'api_key' => $this->config['api_key'],
            'q' => $keyword,
            'engine' => 'google',
            'google_domain' => $options['google_domain'] ?? $this->config['locations']['google_domain'] ?? 'google.com',
            'gl' => $options['gl'] ?? $this->config['locations']['gl'] ?? 'us',
            'hl' => $options['hl'] ?? $this->config['locations']['hl'] ?? 'en',
            'num' => $options['num'] ?? 100,
            'start' => $options['start'] ?? 0,
        ];

        if (!empty($options['location'])) {
            $params['location'] = $options['location'];
        }

        if (!empty($options['device'])) {
            $params['device'] = $options['device']; // desktop, mobile, tablet
        }

        try {
            $response = Http::timeout(30)->get(self::API_BASE_URL, $params);

            if (!$response->successful()) {
                Log::error('SerpAPI request failed', [
                    'status' => $response->status(),
                    'keyword' => $keyword,
                ]);
                return [];
            }

            $data = $response->json();

            $this->cache->put($cacheKey, $data, self::CACHE_TTL);

            return $data;

        } catch (\Exception $e) {
            Log::error('SerpAPI exception', [
                'keyword' => $keyword,
                'error' => $e->getMessage(),
            ]);
            return [];
        }
    }

    /**
     * Transform SERP data to KeywordData DTO.
     */
    private function transformSerpToKeywordData(string $keyword, array $serpData): KeywordData
    {
        // Detect SERP features
        $serpFeatures = $this->detectSerpFeatures($serpData);

        // Estimate difficulty from competition
        $difficulty = $this->estimateDifficultyFromSerp($serpData);

        // Get our position if we rank
        $ourPosition = $this->findOurPosition($serpData);

        // Estimate search volume
        $volumeEstimate = $this->estimateSearchVolume($serpData);

        return new KeywordData(
            keyword: $keyword,
            searchVolume: $volumeEstimate['volume'],
            difficulty: $difficulty,
            position: $ourPosition,
            serpFeatures: $serpFeatures,
            competitorCount: count($serpData['organic_results'] ?? []),
            dataSource: self::PROVIDER_ID,
            dataConfidence: 85,
            fetchedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Detect SERP features present.
     */
    private function detectSerpFeatures(array $serpData): array
    {
        $features = [];

        if (!empty($serpData['answer_box'])) {
            $features[] = SerpAnalysis::FEATURE_FEATURED_SNIPPET;
        }

        if (!empty($serpData['knowledge_graph'])) {
            $features[] = SerpAnalysis::FEATURE_KNOWLEDGE_PANEL;
        }

        if (!empty($serpData['local_results'])) {
            $features[] = SerpAnalysis::FEATURE_LOCAL_PACK;
        }

        if (!empty($serpData['related_questions'])) {
            $features[] = SerpAnalysis::FEATURE_PEOPLE_ALSO_ASK;
        }

        if (!empty($serpData['shopping_results'])) {
            $features[] = SerpAnalysis::FEATURE_SHOPPING;
        }

        if (!empty($serpData['images_results'])) {
            $features[] = SerpAnalysis::FEATURE_IMAGE_PACK;
        }

        if (!empty($serpData['inline_videos'])) {
            $features[] = SerpAnalysis::FEATURE_VIDEO;
        }

        if (!empty($serpData['news_results']) || !empty($serpData['top_stories'])) {
            $features[] = SerpAnalysis::FEATURE_TOP_STORIES;
        }

        // Check for sitelinks
        foreach ($serpData['organic_results'] ?? [] as $result) {
            if (!empty($result['sitelinks'])) {
                $features[] = SerpAnalysis::FEATURE_SITELINKS;
                break;
            }
        }

        // Check for reviews
        foreach ($serpData['organic_results'] ?? [] as $result) {
            if (!empty($result['rich_snippet']['top']['extensions'])) {
                $features[] = SerpAnalysis::FEATURE_REVIEWS;
                break;
            }
        }

        return array_unique($features);
    }

    /**
     * Estimate keyword difficulty from SERP.
     */
    private function estimateDifficultyFromSerp(array $serpData): int
    {
        $difficulty = 50; // Base

        $organicResults = $serpData['organic_results'] ?? [];

        if (empty($organicResults)) {
            return $difficulty;
        }

        // Check for major brands/domains in top 10
        $majorDomains = [
            'wikipedia.org', 'youtube.com', 'amazon.com', 'facebook.com',
            'linkedin.com', 'twitter.com', 'instagram.com', 'reddit.com',
            'quora.com', 'medium.com', 'forbes.com', 'nytimes.com',
        ];

        $majorDomainsCount = 0;
        foreach (array_slice($organicResults, 0, 10) as $result) {
            $domain = parse_url($result['link'] ?? '', PHP_URL_HOST);
            $domain = preg_replace('/^www\./', '', $domain);

            if (in_array($domain, $majorDomains)) {
                $majorDomainsCount++;
            }
        }

        // More major domains = higher difficulty
        $difficulty += ($majorDomainsCount * 5);

        // SERP features indicate competition
        if (!empty($serpData['answer_box'])) {
            $difficulty += 10;
        }
        if (!empty($serpData['knowledge_graph'])) {
            $difficulty += 5;
        }
        if (!empty($serpData['shopping_results'])) {
            $difficulty += 10;
        }

        // High number of results indicates competition
        $totalResults = $serpData['search_information']['total_results'] ?? 0;
        if ($totalResults > 1000000000) {
            $difficulty += 15;
        } elseif ($totalResults > 100000000) {
            $difficulty += 10;
        } elseif ($totalResults > 10000000) {
            $difficulty += 5;
        }

        return (int) min(100, max(0, $difficulty));
    }

    /**
     * Find our position in SERP.
     */
    private function findOurPosition(array $serpData): ?int
    {
        $ourDomain = parse_url(config('app.url'), PHP_URL_HOST);
        $ourDomain = preg_replace('/^www\./', '', $ourDomain);

        foreach ($serpData['organic_results'] ?? [] as $result) {
            $domain = parse_url($result['link'] ?? '', PHP_URL_HOST);
            $domain = preg_replace('/^www\./', '', $domain);

            if ($domain === $ourDomain) {
                return $result['position'] ?? null;
            }
        }

        return null;
    }

    /**
     * Estimate search volume from SERP features.
     */
    private function estimateSearchVolume(array $serpData): array
    {
        // This is a rough estimate based on SERP features
        // Real volume should come from Google Keyword Planner

        $totalResults = $serpData['search_information']['total_results'] ?? 0;

        // More total results typically correlates with higher search volume
        if ($totalResults > 1000000000) {
            $volumeEstimate = 100000; // Very high
        } elseif ($totalResults > 100000000) {
            $volumeEstimate = 50000;
        } elseif ($totalResults > 10000000) {
            $volumeEstimate = 10000;
        } elseif ($totalResults > 1000000) {
            $volumeEstimate = 5000;
        } elseif ($totalResults > 100000) {
            $volumeEstimate = 1000;
        } else {
            $volumeEstimate = 500;
        }

        // Boost if there are ads/shopping results (indicates commercial value)
        if (!empty($serpData['ads']) || !empty($serpData['shopping_results'])) {
            $volumeEstimate = (int) ($volumeEstimate * 1.5);
        }

        return [
            'volume' => $volumeEstimate,
            'trend' => 'estimated',
            'seasonality' => null,
            'confidence' => 40, // Low confidence for estimates
        ];
    }
}
