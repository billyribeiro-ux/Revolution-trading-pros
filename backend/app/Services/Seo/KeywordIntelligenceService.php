<?php

namespace App\Services\Seo;

use App\Models\SeoKeyword;
use App\Models\SeoKeywordCluster;
use App\Models\SeoSerpResult;
use App\Models\SeoSerpCompetitor;
use App\Models\SeoContentGap;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

/**
 * Keyword Intelligence Service
 * 
 * Google L8 Enterprise-Grade Keyword Research & Analysis
 * Implements: SERP Analysis, Difficulty Scoring, Intent Classification, Clustering
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class KeywordIntelligenceService
{
    private const CACHE_PREFIX = 'seo:keywords';
    private const SERP_CACHE_TTL = 21600; // 6 hours
    private const DIFFICULTY_CACHE_TTL = 43200; // 12 hours

    /**
     * Research and expand keywords.
     */
    public function researchKeywords(
        string $seedKeyword,
        int $limit = 100
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":research:" . md5($seedKeyword);
        
        return Cache::remember($cacheKey, self::DIFFICULTY_CACHE_TTL, function () use ($seedKeyword, $limit) {
            $keywords = [];
            
            // Get related keywords
            $keywords = array_merge($keywords, $this->getRelatedKeywords($seedKeyword));
            
            // Get question-based keywords
            $keywords = array_merge($keywords, $this->getQuestionKeywords($seedKeyword));
            
            // Get long-tail variations
            $keywords = array_merge($keywords, $this->getLongTailKeywords($seedKeyword));
            
            // Remove duplicates
            $keywords = array_unique($keywords);
            
            // Limit results
            $keywords = array_slice($keywords, 0, $limit);
            
            // Enrich with metrics
            $enrichedKeywords = [];
            foreach ($keywords as $keyword) {
                $enrichedKeywords[] = $this->enrichKeyword($keyword);
            }
            
            return $enrichedKeywords;
        });
    }

    /**
     * Calculate keyword difficulty score.
     */
    public function calculateDifficulty(string $keyword): int
    {
        $cacheKey = self::CACHE_PREFIX . ":difficulty:" . md5($keyword);
        
        return Cache::remember($cacheKey, self::DIFFICULTY_CACHE_TTL, function () use ($keyword) {
            // Get SERP results
            $serpResults = $this->analyzeSERP($keyword);
            
            if (empty($serpResults)) {
                return 50; // Default medium difficulty
            }
            
            $factors = [
                'avg_domain_authority' => 0,
                'avg_page_authority' => 0,
                'avg_word_count' => 0,
                'has_featured_snippet' => false,
                'top_domains_count' => 0,
            ];
            
            $totalDA = 0;
            $totalPA = 0;
            $totalWords = 0;
            $topDomains = [];
            
            foreach ($serpResults as $result) {
                $totalDA += $result['domain_authority'] ?? 0;
                $totalPA += $result['page_authority'] ?? 0;
                $totalWords += $result['word_count'] ?? 0;
                
                if (!empty($result['serp_features']) && in_array('featured_snippet', $result['serp_features'])) {
                    $factors['has_featured_snippet'] = true;
                }
                
                if (!empty($result['domain']) && ($result['domain_authority'] ?? 0) > 70) {
                    $topDomains[] = $result['domain'];
                }
            }
            
            $count = count($serpResults);
            $factors['avg_domain_authority'] = $count > 0 ? $totalDA / $count : 0;
            $factors['avg_page_authority'] = $count > 0 ? $totalPA / $count : 0;
            $factors['avg_word_count'] = $count > 0 ? $totalWords / $count : 0;
            $factors['top_domains_count'] = count(array_unique($topDomains));
            
            // Calculate difficulty (0-100)
            $difficulty = 0;
            
            // Domain authority weight (40%)
            $difficulty += ($factors['avg_domain_authority'] / 100) * 40;
            
            // Page authority weight (30%)
            $difficulty += ($factors['avg_page_authority'] / 100) * 30;
            
            // Top domains weight (20%)
            $difficulty += ($factors['top_domains_count'] / 10) * 20;
            
            // Featured snippet weight (10%)
            if ($factors['has_featured_snippet']) {
                $difficulty += 10;
            }
            
            return (int) min(100, max(0, $difficulty));
        });
    }

    /**
     * Classify keyword intent.
     */
    public function classifyIntent(string $keyword): string
    {
        $keyword = strtolower($keyword);
        
        // Transactional indicators
        $transactional = ['buy', 'purchase', 'order', 'price', 'cost', 'cheap', 'discount', 'deal', 'coupon', 'shop', 'store'];
        foreach ($transactional as $indicator) {
            if (str_contains($keyword, $indicator)) {
                return 'transactional';
            }
        }
        
        // Commercial indicators
        $commercial = ['best', 'top', 'review', 'comparison', 'vs', 'versus', 'alternative', 'compare', 'option'];
        foreach ($commercial as $indicator) {
            if (str_contains($keyword, $indicator)) {
                return 'commercial';
            }
        }
        
        // Navigational indicators
        $navigational = ['login', 'sign in', 'account', 'dashboard', 'portal', 'official', 'website'];
        foreach ($navigational as $indicator) {
            if (str_contains($keyword, $indicator)) {
                return 'navigational';
            }
        }
        
        // Default to informational
        return 'informational';
    }

    /**
     * Analyze SERP for keyword.
     */
    public function analyzeSERP(string $keyword, bool $forceRefresh = false): array
    {
        $cacheKey = self::CACHE_PREFIX . ":serp:" . md5($keyword);
        
        if ($forceRefresh) {
            Cache::forget($cacheKey);
        }
        
        return Cache::remember($cacheKey, self::SERP_CACHE_TTL, function () use ($keyword) {
            // Find or create keyword record
            $keywordModel = SeoKeyword::firstOrCreate(
                ['keyword' => $keyword],
                [
                    'search_volume' => 0,
                    'difficulty_score' => 0,
                    'opportunity_score' => 0,
                    'intent' => $this->classifyIntent($keyword),
                ]
            );
            
            // Fetch SERP data (using SerpAPI or similar)
            $serpData = $this->fetchSerpData($keyword);
            
            $results = [];
            $competitors = [];
            
            foreach ($serpData as $position => $result) {
                // Store SERP result
                $serpResult = SeoSerpResult::updateOrCreate(
                    [
                        'keyword_id' => $keywordModel->id,
                        'position' => $position + 1,
                    ],
                    [
                        'url' => $result['url'],
                        'title' => $result['title'] ?? '',
                        'description' => $result['description'] ?? '',
                        'domain' => parse_url($result['url'], PHP_URL_HOST),
                        'domain_authority' => $result['domain_authority'] ?? 0,
                        'page_authority' => $result['page_authority'] ?? 0,
                        'word_count' => $result['word_count'] ?? null,
                        'entities_found' => $result['entities'] ?? [],
                        'topics_covered' => $result['topics'] ?? [],
                        'schema_types' => $result['schema_types'] ?? [],
                        'serp_features' => $result['serp_features'] ?? [],
                        'analyzed_at' => now(),
                    ]
                );
                
                $results[] = $serpResult->toArray();
                
                // Track competitors
                $domain = parse_url($result['url'], PHP_URL_HOST);
                if (!isset($competitors[$domain])) {
                    $competitors[$domain] = [
                        'domain' => $domain,
                        'appearances' => 0,
                        'positions' => [],
                        'domain_authority' => $result['domain_authority'] ?? 0,
                        'urls' => [],
                    ];
                }
                
                $competitors[$domain]['appearances']++;
                $competitors[$domain]['positions'][] = $position + 1;
                $competitors[$domain]['urls'][] = $result['url'];
            }
            
            // Store competitor data
            foreach ($competitors as $domain => $data) {
                SeoSerpCompetitor::updateOrCreate(
                    [
                        'keyword_id' => $keywordModel->id,
                        'domain' => $domain,
                    ],
                    [
                        'appearances' => $data['appearances'],
                        'avg_position' => array_sum($data['positions']) / count($data['positions']),
                        'domain_authority' => $data['domain_authority'],
                        'ranking_urls' => $data['urls'],
                    ]
                );
            }
            
            return $results;
        });
    }

    /**
     * Create keyword clusters.
     */
    public function createClusters(array $keywords): array
    {
        // Group keywords by semantic similarity
        $clusters = [];
        
        foreach ($keywords as $keyword) {
            $placed = false;
            
            // Try to place in existing cluster
            foreach ($clusters as $clusterName => &$cluster) {
                if ($this->areKeywordsSimilar($keyword, $cluster['pillar_keyword'])) {
                    $cluster['keywords'][] = $keyword;
                    $placed = true;
                    break;
                }
            }
            
            // Create new cluster if not placed
            if (!$placed) {
                $clusters[$keyword] = [
                    'pillar_keyword' => $keyword,
                    'keywords' => [$keyword],
                ];
            }
        }
        
        // Store clusters in database
        $storedClusters = [];
        
        foreach ($clusters as $clusterData) {
            if (count($clusterData['keywords']) < 2) {
                continue; // Skip single-keyword clusters
            }
            
            $pillarKeyword = SeoKeyword::firstOrCreate(
                ['keyword' => $clusterData['pillar_keyword']],
                [
                    'search_volume' => 0,
                    'difficulty_score' => 0,
                    'opportunity_score' => 0,
                    'intent' => $this->classifyIntent($clusterData['pillar_keyword']),
                ]
            );
            
            $keywordIds = [];
            $totalVolume = 0;
            $totalDifficulty = 0;
            
            foreach ($clusterData['keywords'] as $kw) {
                $kwModel = SeoKeyword::firstOrCreate(
                    ['keyword' => $kw],
                    [
                        'search_volume' => 0,
                        'difficulty_score' => 0,
                        'opportunity_score' => 0,
                        'intent' => $this->classifyIntent($kw),
                    ]
                );
                
                $keywordIds[] = $kwModel->id;
                $totalVolume += $kwModel->search_volume;
                $totalDifficulty += $kwModel->difficulty_score;
            }
            
            $cluster = SeoKeywordCluster::updateOrCreate(
                ['pillar_keyword_id' => $pillarKeyword->id],
                [
                    'cluster_name' => ucfirst($clusterData['pillar_keyword']) . ' Cluster',
                    'cluster_keywords' => $keywordIds,
                    'total_search_volume' => $totalVolume,
                    'avg_difficulty' => count($keywordIds) > 0 ? $totalDifficulty / count($keywordIds) : 0,
                    'content_recommendations' => $this->generateClusterRecommendations($clusterData['keywords']),
                ]
            );
            
            $storedClusters[] = $cluster;
        }
        
        return $storedClusters;
    }

    /**
     * Identify content gaps.
     */
    public function identifyContentGaps(
        string $contentType,
        int $contentId,
        string $targetKeyword,
        array $ourEntities,
        array $ourTopics
    ): array {
        // Analyze SERP
        $serpResults = $this->analyzeSERP($targetKeyword);
        
        // Extract entities and topics from competitors
        $competitorEntities = [];
        $competitorTopics = [];
        
        foreach ($serpResults as $result) {
            if (!empty($result['entities_found'])) {
                $competitorEntities = array_merge($competitorEntities, $result['entities_found']);
            }
            if (!empty($result['topics_covered'])) {
                $competitorTopics = array_merge($competitorTopics, $result['topics_covered']);
            }
        }
        
        // Find missing entities
        $missingEntities = array_diff($competitorEntities, $ourEntities);
        
        // Find missing topics
        $missingTopics = array_diff($competitorTopics, $ourTopics);
        
        // Store content gaps
        $gaps = [];
        
        foreach ($missingEntities as $entity) {
            $gap = SeoContentGap::create([
                'our_content_type' => $contentType,
                'our_content_id' => $contentId,
                'target_keyword' => $targetKeyword,
                'gap_type' => 'entity',
                'gap_description' => "Missing entity: {$entity}",
                'competitor_examples' => $this->findEntityExamples($serpResults, $entity),
                'priority' => 'medium',
                'estimated_impact' => 60,
            ]);
            
            $gaps[] = $gap;
        }
        
        foreach ($missingTopics as $topic) {
            $gap = SeoContentGap::create([
                'our_content_type' => $contentType,
                'our_content_id' => $contentId,
                'target_keyword' => $targetKeyword,
                'gap_type' => 'topic',
                'gap_description' => "Missing topic: {$topic}",
                'competitor_examples' => $this->findTopicExamples($serpResults, $topic),
                'priority' => 'high',
                'estimated_impact' => 75,
            ]);
            
            $gaps[] = $gap;
        }
        
        return $gaps;
    }

    /**
     * Enrich keyword with metrics.
     */
    private function enrichKeyword(string $keyword): array
    {
        $keywordModel = SeoKeyword::firstOrCreate(
            ['keyword' => $keyword],
            [
                'search_volume' => 0,
                'difficulty_score' => 0,
                'opportunity_score' => 0,
                'intent' => $this->classifyIntent($keyword),
            ]
        );
        
        // Calculate difficulty if not set
        if ($keywordModel->difficulty_score === 0) {
            $keywordModel->difficulty_score = $this->calculateDifficulty($keyword);
            $keywordModel->save();
        }
        
        // Calculate opportunity score
        $keywordModel->opportunity_score = $keywordModel->calculateOpportunityScore();
        $keywordModel->save();
        
        return $keywordModel->toArray();
    }

    /**
     * Get related keywords.
     */
    private function getRelatedKeywords(string $seedKeyword): array
    {
        // In production, use Google Keyword Planner API or similar
        // For now, generate simple variations
        
        $related = [];
        $modifiers = ['best', 'top', 'how to', 'what is', 'guide', 'tips', 'examples', 'vs'];
        
        foreach ($modifiers as $modifier) {
            $related[] = $modifier . ' ' . $seedKeyword;
            $related[] = $seedKeyword . ' ' . $modifier;
        }
        
        return $related;
    }

    /**
     * Get question-based keywords.
     */
    private function getQuestionKeywords(string $seedKeyword): array
    {
        $questions = [];
        $questionWords = ['how', 'what', 'why', 'when', 'where', 'who', 'which'];
        
        foreach ($questionWords as $word) {
            $questions[] = $word . ' ' . $seedKeyword;
            $questions[] = $word . ' is ' . $seedKeyword;
            $questions[] = $word . ' to ' . $seedKeyword;
        }
        
        return $questions;
    }

    /**
     * Get long-tail keyword variations.
     */
    private function getLongTailKeywords(string $seedKeyword): array
    {
        $longTail = [];
        $suffixes = ['for beginners', 'in 2024', 'step by step', 'complete guide', 'tutorial', 'explained'];
        
        foreach ($suffixes as $suffix) {
            $longTail[] = $seedKeyword . ' ' . $suffix;
        }
        
        return $longTail;
    }

    /**
     * Fetch SERP data from API.
     */
    private function fetchSerpData(string $keyword): array
    {
        // In production, use SerpAPI, DataForSEO, or similar
        // For now, return mock data
        
        return [
            [
                'url' => 'https://example.com/page1',
                'title' => 'Example Title 1',
                'description' => 'Example description',
                'domain_authority' => 75,
                'page_authority' => 65,
                'word_count' => 2000,
                'entities' => ['Entity1', 'Entity2'],
                'topics' => ['Topic1', 'Topic2'],
                'schema_types' => ['Article'],
                'serp_features' => [],
            ],
        ];
    }

    /**
     * Check if keywords are semantically similar.
     */
    private function areKeywordsSimilar(string $kw1, string $kw2): bool
    {
        // Simple similarity check based on common words
        $words1 = explode(' ', strtolower($kw1));
        $words2 = explode(' ', strtolower($kw2));
        
        $common = array_intersect($words1, $words2);
        
        return count($common) >= 2 || similar_text($kw1, $kw2) > 60;
    }

    /**
     * Generate cluster recommendations.
     */
    private function generateClusterRecommendations(array $keywords): string
    {
        return "Create pillar content covering: " . implode(', ', array_slice($keywords, 0, 5));
    }

    /**
     * Find entity usage examples in SERP results.
     */
    private function findEntityExamples(array $serpResults, string $entity): array
    {
        $examples = [];
        
        foreach ($serpResults as $result) {
            if (!empty($result['entities_found']) && in_array($entity, $result['entities_found'])) {
                $examples[] = [
                    'url' => $result['url'],
                    'title' => $result['title'],
                ];
            }
        }
        
        return $examples;
    }

    /**
     * Find topic coverage examples in SERP results.
     */
    private function findTopicExamples(array $serpResults, string $topic): array
    {
        $examples = [];
        
        foreach ($serpResults as $result) {
            if (!empty($result['topics_covered']) && in_array($topic, $result['topics_covered'])) {
                $examples[] = [
                    'url' => $result['url'],
                    'title' => $result['title'],
                ];
            }
        }
        
        return $examples;
    }
}
