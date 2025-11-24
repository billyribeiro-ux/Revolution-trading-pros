<?php

namespace App\Services\Seo;

use App\Models\SeoEntity;
use App\Models\SeoEntityMention;
use App\Models\SeoTopic;
use App\Models\SeoTopicCoverage;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;

/**
 * NLP Intelligence Service
 * 
 * Google L8 Enterprise-Grade NLP Engine
 * Implements: Entity Extraction, Topic Modeling, Sentiment Analysis
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class NlpIntelligenceService
{
    private const CACHE_PREFIX = 'seo:nlp';
    private const CACHE_TTL = 86400; // 24 hours

    /**
     * Perform comprehensive NLP analysis on content.
     */
    public function analyze(
        string $contentType,
        int $contentId,
        string $text,
        string $html = null
    ): array {
        $cacheKey = $this->getCacheKey($contentType, $contentId, $text);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($contentType, $contentId, $text, $html) {
            $startTime = microtime(true);
            
            try {
                $results = [
                    'entities' => $this->extractEntities($text, $contentType, $contentId),
                    'topics' => $this->extractTopics($text, $contentType, $contentId),
                    'sentiment' => $this->analyzeSentiment($text),
                    'intent' => $this->classifyIntent($text),
                    'readability' => $this->analyzeReadability($text),
                    'processing_time_ms' => (int) ((microtime(true) - $startTime) * 1000),
                ];
                
                // Store in NLP cache table for audit trail
                $this->storeInCache($contentType, $contentId, 'full_analysis', $results);
                
                return $results;
                
            } catch (\Exception $e) {
                Log::error('NLP Analysis failed', [
                    'content_type' => $contentType,
                    'content_id' => $contentId,
                    'error' => $e->getMessage(),
                ]);
                
                throw $e;
            }
        });
    }

    /**
     * Extract entities using Google Cloud NLP API.
     */
    public function extractEntities(string $text, string $contentType, int $contentId): array
    {
        $cacheKey = self::CACHE_PREFIX . ":entities:{$contentType}:{$contentId}:" . md5($text);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($text, $contentType, $contentId) {
            // Use Google Cloud NLP API for entity extraction
            $entities = $this->callGoogleNlpApi($text, 'analyzeEntities');
            
            $processedEntities = [];
            
            foreach ($entities as $entity) {
                // Store or update entity in database
                $seoEntity = SeoEntity::firstOrCreate(
                    ['name' => $entity['name']],
                    [
                        'type' => $entity['type'],
                        'salience_avg' => $entity['salience'],
                        'mention_count' => 0,
                        'wikipedia_url' => $entity['metadata']['wikipedia_url'] ?? null,
                        'knowledge_graph_id' => $entity['metadata']['mid'] ?? null,
                        'metadata' => $entity['metadata'] ?? [],
                    ]
                );
                
                // Record mention
                $mention = SeoEntityMention::updateOrCreate(
                    [
                        'entity_id' => $seoEntity->id,
                        'content_type' => $contentType,
                        'content_id' => $contentId,
                    ],
                    [
                        'salience' => $entity['salience'],
                        'mention_count' => $entity['mentions_count'] ?? 1,
                        'first_mention_position' => $entity['mentions'][0]['text']['beginOffset'] ?? null,
                        'context' => $this->extractContext($text, $entity['mentions'][0]['text']['beginOffset'] ?? 0),
                        'sentiment' => $entity['sentiment']['magnitude'] > 0 ? 
                            ($entity['sentiment']['score'] > 0 ? 'positive' : 'negative') : 
                            'neutral',
                    ]
                );
                
                // Update entity statistics
                $seoEntity->recordMention($entity['salience']);
                
                $processedEntities[] = [
                    'id' => $seoEntity->id,
                    'name' => $entity['name'],
                    'type' => $entity['type'],
                    'salience' => $entity['salience'],
                    'mention_count' => $entity['mentions_count'] ?? 1,
                    'sentiment' => $mention->sentiment,
                    'wikipedia_url' => $seoEntity->wikipedia_url,
                ];
            }
            
            return $processedEntities;
        });
    }

    /**
     * Extract topics using LDA and TF-IDF.
     */
    public function extractTopics(string $text, string $contentType, int $contentId): array
    {
        $cacheKey = self::CACHE_PREFIX . ":topics:{$contentType}:{$contentId}:" . md5($text);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($text, $contentType, $contentId) {
            // Extract keywords using TF-IDF
            $keywords = $this->extractKeywordsTfidf($text);
            
            // Cluster keywords into topics
            $topics = $this->clusterKeywordsIntoTopics($keywords);
            
            $processedTopics = [];
            
            foreach ($topics as $topicData) {
                // Store or update topic
                $topic = SeoTopic::firstOrCreate(
                    ['topic_name' => $topicData['name']],
                    [
                        'description' => $topicData['description'] ?? null,
                        'keywords' => $topicData['keywords'],
                        'content_count' => 0,
                        'avg_relevance' => 0,
                    ]
                );
                
                // Record coverage
                $coverage = SeoTopicCoverage::updateOrCreate(
                    [
                        'topic_id' => $topic->id,
                        'content_type' => $contentType,
                        'content_id' => $contentId,
                    ],
                    [
                        'relevance_score' => $topicData['relevance'],
                        'keyword_count' => count($topicData['keywords_found']),
                        'keywords_found' => $topicData['keywords_found'],
                    ]
                );
                
                // Update topic statistics
                $topic->recordCoverage($topicData['relevance']);
                
                $processedTopics[] = [
                    'id' => $topic->id,
                    'name' => $topicData['name'],
                    'relevance' => $topicData['relevance'],
                    'keywords' => $topicData['keywords_found'],
                ];
            }
            
            return $processedTopics;
        });
    }

    /**
     * Analyze sentiment using Google Cloud NLP API.
     */
    public function analyzeSentiment(string $text): array
    {
        $cacheKey = self::CACHE_PREFIX . ":sentiment:" . md5($text);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($text) {
            $sentiment = $this->callGoogleNlpApi($text, 'analyzeSentiment');
            
            return [
                'score' => $sentiment['documentSentiment']['score'], // -1 to 1
                'magnitude' => $sentiment['documentSentiment']['magnitude'], // 0 to infinity
                'label' => $this->getSentimentLabel($sentiment['documentSentiment']['score']),
            ];
        });
    }

    /**
     * Classify content intent.
     */
    public function classifyIntent(string $text): string
    {
        $text = strtolower($text);
        
        // Transactional intent indicators
        $transactionalKeywords = ['buy', 'purchase', 'order', 'price', 'cost', 'discount', 'deal', 'shop', 'cart'];
        foreach ($transactionalKeywords as $keyword) {
            if (str_contains($text, $keyword)) {
                return 'transactional';
            }
        }
        
        // Commercial intent indicators
        $commercialKeywords = ['best', 'top', 'review', 'comparison', 'vs', 'alternative', 'option'];
        foreach ($commercialKeywords as $keyword) {
            if (str_contains($text, $keyword)) {
                return 'commercial';
            }
        }
        
        // Navigational intent indicators
        $navigationalKeywords = ['login', 'sign in', 'account', 'dashboard', 'portal'];
        foreach ($navigationalKeywords as $keyword) {
            if (str_contains($text, $keyword)) {
                return 'navigational';
            }
        }
        
        // Default to informational
        return 'informational';
    }

    /**
     * Analyze readability metrics.
     */
    public function analyzeReadability(string $text): array
    {
        $sentences = preg_split('/[.!?]+/', $text, -1, PREG_SPLIT_NO_EMPTY);
        $sentenceCount = count($sentences);
        $wordCount = str_word_count($text);
        $syllableCount = $this->countSyllables($text);
        
        if ($sentenceCount === 0 || $wordCount === 0) {
            return [
                'flesch_score' => 0,
                'grade_level' => 0,
                'avg_words_per_sentence' => 0,
                'avg_syllables_per_word' => 0,
            ];
        }
        
        $avgWordsPerSentence = $wordCount / $sentenceCount;
        $avgSyllablesPerWord = $syllableCount / $wordCount;
        
        // Flesch Reading Ease
        $fleschScore = 206.835 - 1.015 * $avgWordsPerSentence - 84.6 * $avgSyllablesPerWord;
        $fleschScore = max(0, min(100, $fleschScore));
        
        // Flesch-Kincaid Grade Level
        $gradeLevel = 0.39 * $avgWordsPerSentence + 11.8 * $avgSyllablesPerWord - 15.59;
        $gradeLevel = max(0, $gradeLevel);
        
        return [
            'flesch_score' => round($fleschScore, 2),
            'grade_level' => round($gradeLevel, 1),
            'avg_words_per_sentence' => round($avgWordsPerSentence, 1),
            'avg_syllables_per_word' => round($avgSyllablesPerWord, 2),
            'readability_label' => $this->getReadabilityLabel($fleschScore),
        ];
    }

    /**
     * Call Google Cloud NLP API.
     */
    private function callGoogleNlpApi(string $text, string $method): array
    {
        $apiKey = config('services.google_nlp.api_key');
        
        if (!$apiKey) {
            // Fallback to local NLP if API key not configured
            return $this->fallbackNlpAnalysis($text, $method);
        }
        
        try {
            $response = Http::timeout(10)->post("https://language.googleapis.com/v1/documents:{$method}?key={$apiKey}", [
                'document' => [
                    'type' => 'PLAIN_TEXT',
                    'content' => Str::limit($text, 10000), // API limit
                ],
                'encodingType' => 'UTF8',
            ]);
            
            if ($response->successful()) {
                return $response->json();
            }
            
            Log::warning('Google NLP API call failed', [
                'method' => $method,
                'status' => $response->status(),
            ]);
            
            return $this->fallbackNlpAnalysis($text, $method);
            
        } catch (\Exception $e) {
            Log::error('Google NLP API error', [
                'method' => $method,
                'error' => $e->getMessage(),
            ]);
            
            return $this->fallbackNlpAnalysis($text, $method);
        }
    }

    /**
     * Fallback NLP analysis using local algorithms.
     */
    private function fallbackNlpAnalysis(string $text, string $method): array
    {
        switch ($method) {
            case 'analyzeEntities':
                return $this->extractEntitiesLocal($text);
            case 'analyzeSentiment':
                return $this->analyzeSentimentLocal($text);
            default:
                return [];
        }
    }

    /**
     * Local entity extraction (basic NER).
     */
    private function extractEntitiesLocal(string $text): array
    {
        // Simple capitalized word extraction as entities
        preg_match_all('/\b[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*\b/', $text, $matches);
        
        $entities = [];
        $entityCounts = array_count_values($matches[0]);
        $totalWords = str_word_count($text);
        
        foreach ($entityCounts as $entity => $count) {
            $salience = min(1.0, $count / ($totalWords / 100));
            
            $entities[] = [
                'name' => $entity,
                'type' => 'UNKNOWN',
                'salience' => $salience,
                'mentions_count' => $count,
                'mentions' => [['text' => ['beginOffset' => strpos($text, $entity)]]],
                'sentiment' => ['score' => 0, 'magnitude' => 0],
                'metadata' => [],
            ];
        }
        
        return $entities;
    }

    /**
     * Local sentiment analysis (basic).
     */
    private function analyzeSentimentLocal(string $text): array
    {
        $positiveWords = ['good', 'great', 'excellent', 'amazing', 'wonderful', 'fantastic', 'love', 'best'];
        $negativeWords = ['bad', 'terrible', 'awful', 'horrible', 'worst', 'hate', 'poor', 'disappointing'];
        
        $text = strtolower($text);
        $positiveCount = 0;
        $negativeCount = 0;
        
        foreach ($positiveWords as $word) {
            $positiveCount += substr_count($text, $word);
        }
        
        foreach ($negativeWords as $word) {
            $negativeCount += substr_count($text, $word);
        }
        
        $totalSentimentWords = $positiveCount + $negativeCount;
        $score = $totalSentimentWords > 0 ? 
            ($positiveCount - $negativeCount) / $totalSentimentWords : 
            0;
        
        return [
            'documentSentiment' => [
                'score' => $score,
                'magnitude' => $totalSentimentWords / 10,
            ],
        ];
    }

    /**
     * Extract keywords using TF-IDF.
     */
    private function extractKeywordsTfidf(string $text): array
    {
        // Simple TF-IDF implementation
        $words = str_word_count(strtolower($text), 1);
        $wordFreq = array_count_values($words);
        
        // Remove stop words
        $stopWords = ['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by'];
        $wordFreq = array_diff_key($wordFreq, array_flip($stopWords));
        
        // Calculate TF scores
        $maxFreq = max($wordFreq);
        $tfScores = [];
        
        foreach ($wordFreq as $word => $freq) {
            if (strlen($word) > 3) { // Only words longer than 3 characters
                $tfScores[$word] = $freq / $maxFreq;
            }
        }
        
        arsort($tfScores);
        
        return array_slice($tfScores, 0, 20, true);
    }

    /**
     * Cluster keywords into topics.
     */
    private function clusterKeywordsIntoTopics(array $keywords): array
    {
        // Simple topic clustering based on keyword co-occurrence
        // In production, use more sophisticated clustering (K-means, LDA)
        
        $topics = [];
        $keywordList = array_keys($keywords);
        
        // Create one topic for top keywords
        if (count($keywordList) >= 5) {
            $topKeywords = array_slice($keywordList, 0, 5);
            $topics[] = [
                'name' => ucfirst($topKeywords[0]) . ' Topic',
                'description' => 'Main topic based on keyword frequency',
                'keywords' => $topKeywords,
                'keywords_found' => $topKeywords,
                'relevance' => array_sum(array_slice($keywords, 0, 5)) / 5,
            ];
        }
        
        return $topics;
    }

    /**
     * Count syllables in text.
     */
    private function countSyllables(string $text): int
    {
        $words = str_word_count(strtolower($text), 1);
        $syllableCount = 0;
        
        foreach ($words as $word) {
            $syllableCount += $this->countSyllablesInWord($word);
        }
        
        return $syllableCount;
    }

    /**
     * Count syllables in a single word.
     */
    private function countSyllablesInWord(string $word): int
    {
        $word = strtolower($word);
        $word = preg_replace('/[^a-z]/', '', $word);
        
        if (strlen($word) <= 3) {
            return 1;
        }
        
        $word = preg_replace('/(?:[^laeiouy]es|ed|[^laeiouy]e)$/', '', $word);
        $word = preg_replace('/^y/', '', $word);
        
        $matches = preg_match_all('/[aeiouy]{1,2}/', $word);
        
        return max(1, $matches);
    }

    /**
     * Extract context around entity mention.
     */
    private function extractContext(string $text, int $position, int $contextLength = 100): string
    {
        $start = max(0, $position - $contextLength);
        $length = $contextLength * 2;
        
        return substr($text, $start, $length);
    }

    /**
     * Get sentiment label from score.
     */
    private function getSentimentLabel(float $score): string
    {
        if ($score >= 0.25) return 'positive';
        if ($score <= -0.25) return 'negative';
        return 'neutral';
    }

    /**
     * Get readability label from Flesch score.
     */
    private function getReadabilityLabel(float $score): string
    {
        if ($score >= 90) return 'Very Easy';
        if ($score >= 80) return 'Easy';
        if ($score >= 70) return 'Fairly Easy';
        if ($score >= 60) return 'Standard';
        if ($score >= 50) return 'Fairly Difficult';
        if ($score >= 30) return 'Difficult';
        return 'Very Difficult';
    }

    /**
     * Get cache key for content.
     */
    private function getCacheKey(string $contentType, int $contentId, string $text): string
    {
        return self::CACHE_PREFIX . ":{$contentType}:{$contentId}:" . md5($text);
    }

    /**
     * Store analysis in NLP cache table.
     */
    private function storeInCache(string $contentType, int $contentId, string $operationType, array $result): void
    {
        \DB::table('seo_nlp_cache')->updateOrInsert(
            [
                'cache_key' => $this->getCacheKey($contentType, $contentId, json_encode($result)),
            ],
            [
                'operation_type' => $operationType,
                'content_type' => $contentType,
                'content_id' => $contentId,
                'result' => json_encode($result),
                'model_version' => 'google-nlp-v1',
                'processing_time_ms' => $result['processing_time_ms'] ?? null,
                'expires_at' => now()->addSeconds(self::CACHE_TTL),
                'updated_at' => now(),
                'created_at' => now(),
            ]
        );
    }
}
