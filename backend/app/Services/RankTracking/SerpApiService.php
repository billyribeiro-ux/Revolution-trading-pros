<?php

namespace App\Services\RankTracking;

use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use App\Exceptions\SerpApiException;

class SerpApiService
{
    private string $apiKey;
    private string $baseUrl;
    private array $rateLimits = [
        'google' => 1000,  // per day
        'bing' => 500,
        'yahoo' => 300,
        'duckduckgo' => 100,
    ];
    
    public function __construct()
    {
        $this->apiKey = config('services.serpapi.key');
        $this->baseUrl = config('services.serpapi.url', 'https://serpapi.com/search');
    }

    /**
     * Check ranking position for a keyword and URL.
     */
    public function checkRanking(string $keyword, string $url, array $params = []): array
    {
        $searchEngine = $params['search_engine'] ?? 'google';
        $location = $params['location'] ?? 'United States';
        $device = $params['device'] ?? 'desktop';
        $language = $params['language'] ?? 'en';
        $numResults = $params['num_results'] ?? 100;

        // Check rate limits
        $this->checkRateLimit($searchEngine);

        // Build API request
        $apiParams = $this->buildApiParams($keyword, $searchEngine, $location, $device, $language, $numResults);

        try {
            // Make API request
            $response = Http::timeout(30)
                ->retry(3, 1000)
                ->get($this->baseUrl, $apiParams);

            if (!$response->successful()) {
                throw new SerpApiException("SERP API request failed: " . $response->body());
            }

            $data = $response->json();

            // Parse results
            $results = $this->parseResults($data, $searchEngine);
            
            // Find position for the target URL
            $position = $this->findPosition($results, $url);
            
            // Extract SERP features
            $serpFeatures = $this->extractSerpFeatures($data, $searchEngine);
            
            // Get search metrics
            $metrics = $this->extractSearchMetrics($data, $keyword);

            return [
                'position' => $position,
                'total_results' => count($results),
                'top_results' => array_slice($results, 0, 10),
                'serp_features' => $serpFeatures,
                'search_volume' => $metrics['search_volume'] ?? null,
                'cpc' => $metrics['cpc'] ?? null,
                'competition' => $metrics['competition'] ?? null,
                'related_keywords' => $metrics['related_keywords'] ?? [],
                'people_also_ask' => $this->extractPeopleAlsoAsk($data),
                'raw_data' => $params['include_raw'] ?? false ? $data : null,
            ];

        } catch (\Exception $e) {
            Log::error('SERP API error', [
                'keyword' => $keyword,
                'url' => $url,
                'error' => $e->getMessage(),
            ]);
            
            throw new SerpApiException("Failed to check ranking: " . $e->getMessage());
        }
    }

    /**
     * Analyze SERP features for a keyword.
     */
    public function analyzeSerpFeatures(string $keyword, array $params = []): array
    {
        $searchEngine = $params['search_engine'] ?? 'google';
        $location = $params['location'] ?? 'United States';
        
        $cacheKey = "serp_features:{$searchEngine}:{$keyword}:{$location}";
        
        return Cache::remember($cacheKey, 3600, function () use ($keyword, $searchEngine, $location) {
            $apiParams = $this->buildApiParams($keyword, $searchEngine, $location, 'desktop', 'en', 10);
            
            $response = Http::timeout(30)->get($this->baseUrl, $apiParams);
            
            if (!$response->successful()) {
                return [];
            }
            
            $data = $response->json();
            
            return $this->extractDetailedSerpFeatures($data, $searchEngine);
        });
    }

    /**
     * Get search volume and keyword metrics.
     */
    public function getKeywordMetrics(string $keyword, array $params = []): array
    {
        $location = $params['location'] ?? 'United States';
        $language = $params['language'] ?? 'en';
        
        // This would integrate with Google Ads API or similar service
        // For now, using estimation based on SERP data
        
        $apiParams = [
            'api_key' => $this->apiKey,
            'q' => $keyword,
            'location' => $location,
            'hl' => $language,
            'num' => 1,
        ];
        
        try {
            $response = Http::timeout(30)->get($this->baseUrl, $apiParams);
            
            if (!$response->successful()) {
                return $this->getEstimatedMetrics($keyword);
            }
            
            $data = $response->json();
            
            return [
                'search_volume' => $this->estimateSearchVolume($data),
                'cpc' => $this->estimateCpc($data),
                'competition' => $this->estimateCompetition($data),
                'difficulty' => $this->calculateDifficulty($data),
                'trend' => $this->analyzeTrend($keyword),
            ];
            
        } catch (\Exception $e) {
            return $this->getEstimatedMetrics($keyword);
        }
    }

    /**
     * Build API parameters based on search engine.
     */
    private function buildApiParams(
        string $keyword,
        string $searchEngine,
        string $location,
        string $device,
        string $language,
        int $numResults
    ): array {
        $baseParams = [
            'api_key' => $this->apiKey,
            'q' => $keyword,
            'location' => $location,
            'num' => $numResults,
        ];

        switch ($searchEngine) {
            case 'google':
                return array_merge($baseParams, [
                    'engine' => 'google',
                    'google_domain' => 'google.com',
                    'gl' => $this->getCountryCode($location),
                    'hl' => $language,
                    'device' => $device,
                    'no_cache' => 'true',
                ]);
                
            case 'bing':
                return array_merge($baseParams, [
                    'engine' => 'bing',
                    'cc' => $this->getCountryCode($location),
                    'setlang' => $language,
                    'device' => $device,
                ]);
                
            case 'yahoo':
                return array_merge($baseParams, [
                    'engine' => 'yahoo',
                    'vl' => $location,
                    'lang' => $language,
                ]);
                
            case 'duckduckgo':
                return array_merge($baseParams, [
                    'engine' => 'duckduckgo',
                    'kl' => $this->getCountryCode($location) . '-' . $language,
                ]);
                
            default:
                return $baseParams;
        }
    }

    /**
     * Parse search results based on search engine format.
     */
    private function parseResults(array $data, string $searchEngine): array
    {
        $results = [];
        
        switch ($searchEngine) {
            case 'google':
                $organicResults = $data['organic_results'] ?? [];
                foreach ($organicResults as $index => $result) {
                    $results[] = [
                        'position' => $result['position'] ?? ($index + 1),
                        'title' => $result['title'] ?? '',
                        'url' => $result['link'] ?? '',
                        'display_url' => $result['displayed_link'] ?? '',
                        'snippet' => $result['snippet'] ?? '',
                        'domain' => parse_url($result['link'] ?? '', PHP_URL_HOST),
                        'is_amp' => $result['is_amp'] ?? false,
                        'date' => $result['date'] ?? null,
                    ];
                }
                break;
                
            case 'bing':
                $organicResults = $data['organic_results'] ?? [];
                foreach ($organicResults as $index => $result) {
                    $results[] = [
                        'position' => $index + 1,
                        'title' => $result['title'] ?? '',
                        'url' => $result['link'] ?? '',
                        'display_url' => $result['displayed_link'] ?? '',
                        'snippet' => $result['snippet'] ?? '',
                        'domain' => parse_url($result['link'] ?? '', PHP_URL_HOST),
                    ];
                }
                break;
                
            default:
                // Generic parsing for other search engines
                $organicResults = $data['results'] ?? $data['organic_results'] ?? [];
                foreach ($organicResults as $index => $result) {
                    $results[] = [
                        'position' => $index + 1,
                        'title' => $result['title'] ?? '',
                        'url' => $result['url'] ?? $result['link'] ?? '',
                        'snippet' => $result['snippet'] ?? $result['description'] ?? '',
                        'domain' => parse_url($result['url'] ?? $result['link'] ?? '', PHP_URL_HOST),
                    ];
                }
        }
        
        return $results;
    }

    /**
     * Find position of target URL in results.
     */
    private function findPosition(array $results, string $targetUrl): ?int
    {
        $targetDomain = parse_url($targetUrl, PHP_URL_HOST);
        $targetPath = parse_url($targetUrl, PHP_URL_PATH);
        
        foreach ($results as $result) {
            // Check exact URL match
            if ($result['url'] === $targetUrl) {
                return $result['position'];
            }
            
            // Check domain match with path similarity
            if ($result['domain'] === $targetDomain) {
                $resultPath = parse_url($result['url'], PHP_URL_PATH);
                
                // If paths match or one contains the other
                if ($resultPath === $targetPath || 
                    str_contains($resultPath, $targetPath) || 
                    str_contains($targetPath, $resultPath)) {
                    return $result['position'];
                }
            }
        }
        
        return null; // Not found in results
    }

    /**
     * Extract SERP features from results.
     */
    private function extractSerpFeatures(array $data, string $searchEngine): array
    {
        $features = [];
        
        if ($searchEngine === 'google') {
            // Featured Snippet
            if (isset($data['answer_box'])) {
                $features['featured_snippet'] = [
                    'type' => $data['answer_box']['type'] ?? 'unknown',
                    'title' => $data['answer_box']['title'] ?? null,
                    'answer' => $data['answer_box']['answer'] ?? null,
                    'source' => $data['answer_box']['link'] ?? null,
                ];
            }
            
            // Knowledge Panel
            if (isset($data['knowledge_graph'])) {
                $features['knowledge_panel'] = [
                    'title' => $data['knowledge_graph']['title'] ?? null,
                    'type' => $data['knowledge_graph']['type'] ?? null,
                    'description' => $data['knowledge_graph']['description'] ?? null,
                ];
            }
            
            // Local Pack
            if (isset($data['local_results'])) {
                $features['local_pack'] = [
                    'count' => count($data['local_results']),
                    'places' => array_map(function ($place) {
                        return [
                            'title' => $place['title'] ?? null,
                            'rating' => $place['rating'] ?? null,
                            'reviews' => $place['reviews'] ?? null,
                        ];
                    }, array_slice($data['local_results'], 0, 3)),
                ];
            }
            
            // Shopping Results
            if (isset($data['shopping_results'])) {
                $features['shopping'] = [
                    'count' => count($data['shopping_results']),
                ];
            }
            
            // Video Carousel
            if (isset($data['videos_results'])) {
                $features['videos'] = [
                    'count' => count($data['videos_results']),
                ];
            }
            
            // Image Pack
            if (isset($data['images_results'])) {
                $features['images'] = [
                    'count' => count($data['images_results']),
                ];
            }
            
            // Top Stories
            if (isset($data['top_stories'])) {
                $features['top_stories'] = [
                    'count' => count($data['top_stories']),
                ];
            }
            
            // Ads
            if (isset($data['ads'])) {
                $features['ads'] = [
                    'top' => count(array_filter($data['ads'], fn($ad) => ($ad['block_position'] ?? '') === 'top')),
                    'bottom' => count(array_filter($data['ads'], fn($ad) => ($ad['block_position'] ?? '') === 'bottom')),
                ];
            }
            
            // Site Links
            if (isset($data['organic_results'][0]['sitelinks'])) {
                $features['sitelinks'] = true;
            }
            
            // Rich Snippets
            foreach ($data['organic_results'] ?? [] as $result) {
                if (isset($result['rich_snippet'])) {
                    $features['rich_snippets'] = true;
                    break;
                }
            }
        }
        
        return $features;
    }

    /**
     * Extract detailed SERP features for analysis.
     */
    private function extractDetailedSerpFeatures(array $data, string $searchEngine): array
    {
        $features = $this->extractSerpFeatures($data, $searchEngine);
        
        // Add opportunity analysis
        foreach ($features as $feature => &$details) {
            if (is_array($details)) {
                $details['opportunity'] = $this->assessFeatureOpportunity($feature, $details);
            }
        }
        
        return $features;
    }

    /**
     * Extract People Also Ask questions.
     */
    private function extractPeopleAlsoAsk(array $data): array
    {
        $questions = [];
        
        if (isset($data['related_questions'])) {
            foreach ($data['related_questions'] as $question) {
                $questions[] = [
                    'question' => $question['question'] ?? '',
                    'answer' => $question['answer'] ?? null,
                    'source' => $question['link'] ?? null,
                ];
            }
        }
        
        return $questions;
    }

    /**
     * Extract search metrics from SERP data.
     */
    private function extractSearchMetrics(array $data, string $keyword): array
    {
        // This would normally come from a keyword research API
        // Using estimation for demonstration
        
        $totalResults = $data['search_information']['total_results'] ?? 0;
        
        return [
            'search_volume' => $this->estimateVolumeFromResults($totalResults),
            'cpc' => $this->estimateCpcFromAds($data['ads'] ?? []),
            'competition' => $this->estimateCompetitionScore($data),
            'related_keywords' => $this->extractRelatedKeywords($data),
        ];
    }

    /**
     * Estimate search volume from total results.
     */
    private function estimateVolumeFromResults(int $totalResults): int
    {
        // Rough estimation based on result count
        if ($totalResults > 100000000) return 100000;
        if ($totalResults > 10000000) return 50000;
        if ($totalResults > 1000000) return 10000;
        if ($totalResults > 100000) return 5000;
        if ($totalResults > 10000) return 1000;
        
        return 100;
    }

    /**
     * Estimate CPC from ad presence.
     */
    private function estimateCpcFromAds(array $ads): ?float
    {
        if (empty($ads)) {
            return null;
        }
        
        // More ads typically indicate higher commercial value
        $adCount = count($ads);
        
        if ($adCount > 8) return 5.00;
        if ($adCount > 4) return 2.50;
        if ($adCount > 2) return 1.00;
        
        return 0.50;
    }

    /**
     * Estimate competition score.
     */
    private function estimateCompetitionScore(array $data): float
    {
        $score = 0;
        
        // Ads presence
        if (isset($data['ads'])) {
            $score += min(count($data['ads']) * 0.1, 0.3);
        }
        
        // SERP features
        $features = ['shopping_results', 'local_results', 'answer_box'];
        foreach ($features as $feature) {
            if (isset($data[$feature])) {
                $score += 0.1;
            }
        }
        
        // Domain authority of top results
        $topResults = array_slice($data['organic_results'] ?? [], 0, 5);
        $authorityDomains = ['wikipedia.org', 'amazon.com', 'youtube.com', 'facebook.com'];
        
        foreach ($topResults as $result) {
            $domain = parse_url($result['link'] ?? '', PHP_URL_HOST);
            if (in_array($domain, $authorityDomains)) {
                $score += 0.1;
            }
        }
        
        return min($score, 1.0);
    }

    /**
     * Extract related keywords from SERP data.
     */
    private function extractRelatedKeywords(array $data): array
    {
        $keywords = [];
        
        // From related searches
        if (isset($data['related_searches'])) {
            foreach ($data['related_searches'] as $search) {
                $keywords[] = $search['query'] ?? $search;
            }
        }
        
        // From People Also Ask
        if (isset($data['related_questions'])) {
            foreach ($data['related_questions'] as $question) {
                // Extract keywords from questions
                $keywords[] = $this->extractKeywordsFromQuestion($question['question'] ?? '');
            }
        }
        
        return array_unique(array_filter($keywords));
    }

    /**
     * Extract keywords from a question.
     */
    private function extractKeywordsFromQuestion(string $question): string
    {
        // Remove common question words
        $stopWords = ['what', 'how', 'why', 'when', 'where', 'who', 'is', 'are', 'do', 'does', 'can', 'will'];
        
        $words = explode(' ', strtolower($question));
        $keywords = array_diff($words, $stopWords);
        
        return implode(' ', $keywords);
    }

    /**
     * Get estimated metrics when API fails.
     */
    private function getEstimatedMetrics(string $keyword): array
    {
        $wordCount = str_word_count($keyword);
        
        return [
            'search_volume' => $wordCount === 1 ? 10000 : ($wordCount === 2 ? 5000 : 1000),
            'cpc' => 1.00,
            'competition' => 0.5,
            'difficulty' => 50,
            'trend' => 'stable',
        ];
    }

    /**
     * Calculate keyword difficulty.
     */
    private function calculateDifficulty(array $data): int
    {
        $difficulty = 0;
        
        // Based on total results
        $totalResults = $data['search_information']['total_results'] ?? 0;
        if ($totalResults > 1000000) $difficulty += 20;
        if ($totalResults > 10000000) $difficulty += 20;
        
        // Based on SERP features
        if (isset($data['answer_box'])) $difficulty += 15;
        if (isset($data['knowledge_graph'])) $difficulty += 10;
        if (isset($data['ads']) && count($data['ads']) > 4) $difficulty += 15;
        
        // Based on domain authority in top results
        $topResults = array_slice($data['organic_results'] ?? [], 0, 10);
        $authorityCount = 0;
        
        foreach ($topResults as $result) {
            $domain = parse_url($result['link'] ?? '', PHP_URL_HOST);
            if ($this->isHighAuthorityDomain($domain)) {
                $authorityCount++;
            }
        }
        
        $difficulty += ($authorityCount * 5);
        
        return min($difficulty, 100);
    }

    /**
     * Analyze keyword trend.
     */
    private function analyzeTrend(string $keyword): string
    {
        // This would integrate with Google Trends API
        // For now, returning a static value
        return 'stable';
    }

    /**
     * Check if domain is high authority.
     */
    private function isHighAuthorityDomain(string $domain): bool
    {
        $highAuthorityDomains = [
            'wikipedia.org', 'amazon.com', 'youtube.com', 'facebook.com',
            'twitter.com', 'linkedin.com', 'instagram.com', 'pinterest.com',
            'reddit.com', 'quora.com', 'medium.com', 'github.com',
            'stackoverflow.com', 'apple.com', 'microsoft.com', 'google.com'
        ];
        
        return in_array($domain, $highAuthorityDomains);
    }

    /**
     * Assess opportunity for a SERP feature.
     */
    private function assessFeatureOpportunity(string $feature, array $details): string
    {
        switch ($feature) {
            case 'featured_snippet':
                return isset($details['source']) ? 'occupied' : 'available';
                
            case 'local_pack':
                return $details['count'] < 3 ? 'available' : 'competitive';
                
            case 'videos':
                return $details['count'] > 0 ? 'saturated' : 'opportunity';
                
            default:
                return 'neutral';
        }
    }

    /**
     * Get country code from location string.
     */
    private function getCountryCode(string $location): string
    {
        $countryCodes = [
            'United States' => 'us',
            'United Kingdom' => 'uk',
            'Canada' => 'ca',
            'Australia' => 'au',
            'Germany' => 'de',
            'France' => 'fr',
            'Spain' => 'es',
            'Italy' => 'it',
            'Japan' => 'jp',
            'Brazil' => 'br',
            'India' => 'in',
            'Mexico' => 'mx',
            // Add more as needed
        ];
        
        return $countryCodes[$location] ?? 'us';
    }

    /**
     * Check and enforce rate limits.
     */
    private function checkRateLimit(string $searchEngine): void
    {
        $key = "serp_api_rate:{$searchEngine}:" . now()->format('Y-m-d');
        $count = Cache::get($key, 0);
        
        if ($count >= $this->rateLimits[$searchEngine]) {
            throw new SerpApiException("Rate limit exceeded for {$searchEngine}");
        }
        
        Cache::put($key, $count + 1, now()->endOfDay());
    }
}