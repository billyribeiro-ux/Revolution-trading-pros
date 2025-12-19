<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Str;

/**
 * AI Content Analysis Service
 *
 * Provides intelligent content analysis features:
 * - Auto-tagging and categorization
 * - Sentiment analysis
 * - Readability scoring (Flesch-Kincaid, etc.)
 * - SEO optimization suggestions
 * - Content similarity detection
 * - Keyword extraction
 * - Summarization
 * - Language detection
 * - Toxicity/moderation scoring
 * - Entity extraction (people, places, organizations)
 */
class AIContentAnalysisService
{
    private PortableTextService $portableText;

    public function __construct(PortableTextService $portableText)
    {
        $this->portableText = $portableText;
    }

    /**
     * Perform comprehensive content analysis
     */
    public function analyze(array $content, array $options = []): ContentAnalysisResult
    {
        $text = $this->extractText($content);
        $cacheKey = 'content_analysis:' . md5($text . json_encode($options));

        if (!($options['skipCache'] ?? false)) {
            $cached = Cache::get($cacheKey);
            if ($cached) {
                return ContentAnalysisResult::fromArray($cached);
            }
        }

        $result = new ContentAnalysisResult();

        // Run all analyses
        $result->readability = $this->analyzeReadability($text);
        $result->sentiment = $this->analyzeSentiment($text);
        $result->keywords = $this->extractKeywords($text, $options['maxKeywords'] ?? 10);
        $result->entities = $this->extractEntities($text);
        $result->seo = $this->analyzeSEO($content, $text, $options);
        $result->language = $this->detectLanguage($text);
        $result->summary = $this->generateSummary($text, $options['summaryLength'] ?? 150);
        $result->suggestedTags = $this->suggestTags($text, $result->keywords);
        $result->suggestedCategories = $this->suggestCategories($text, $result->entities);
        $result->contentQuality = $this->assessContentQuality($text, $result);
        $result->moderation = $this->moderateContent($text);

        // Cache for 1 hour
        Cache::put($cacheKey, $result->toArray(), 3600);

        return $result;
    }

    /**
     * Analyze readability metrics
     */
    public function analyzeReadability(string $text): array
    {
        $sentences = $this->countSentences($text);
        $words = str_word_count($text);
        $syllables = $this->countSyllables($text);
        $complexWords = $this->countComplexWords($text);
        $characters = strlen(preg_replace('/\s+/', '', $text));

        if ($words === 0 || $sentences === 0) {
            return [
                'score' => 0,
                'grade' => 'N/A',
                'metrics' => [],
            ];
        }

        // Flesch Reading Ease
        $fleschReadingEase = 206.835 - (1.015 * ($words / $sentences)) - (84.6 * ($syllables / $words));
        $fleschReadingEase = max(0, min(100, $fleschReadingEase));

        // Flesch-Kincaid Grade Level
        $fleschKincaidGrade = (0.39 * ($words / $sentences)) + (11.8 * ($syllables / $words)) - 15.59;
        $fleschKincaidGrade = max(0, $fleschKincaidGrade);

        // Gunning Fog Index
        $gunningFog = 0.4 * (($words / $sentences) + (100 * ($complexWords / $words)));

        // SMOG Index
        $smog = 1.043 * sqrt($complexWords * (30 / max(1, $sentences))) + 3.1291;

        // Coleman-Liau Index
        $L = ($characters / $words) * 100;
        $S = ($sentences / $words) * 100;
        $colemanLiau = (0.0588 * $L) - (0.296 * $S) - 15.8;

        // Automated Readability Index
        $ari = (4.71 * ($characters / $words)) + (0.5 * ($words / $sentences)) - 21.43;

        // Average grade level
        $avgGrade = ($fleschKincaidGrade + $gunningFog + $smog + max(0, $colemanLiau) + max(0, $ari)) / 5;

        return [
            'score' => round($fleschReadingEase, 1),
            'grade' => $this->gradeToLabel(round($avgGrade)),
            'gradeLevel' => round($avgGrade, 1),
            'difficulty' => $this->scoreTodifficulty($fleschReadingEase),
            'metrics' => [
                'fleschReadingEase' => round($fleschReadingEase, 1),
                'fleschKincaidGrade' => round($fleschKincaidGrade, 1),
                'gunningFogIndex' => round($gunningFog, 1),
                'smogIndex' => round($smog, 1),
                'colemanLiauIndex' => round($colemanLiau, 1),
                'automatedReadabilityIndex' => round($ari, 1),
            ],
            'stats' => [
                'sentences' => $sentences,
                'words' => $words,
                'syllables' => $syllables,
                'complexWords' => $complexWords,
                'characters' => $characters,
                'avgWordsPerSentence' => round($words / $sentences, 1),
                'avgSyllablesPerWord' => round($syllables / $words, 2),
            ],
        ];
    }

    /**
     * Analyze sentiment
     */
    public function analyzeSentiment(string $text): array
    {
        $sentences = $this->splitIntoSentences($text);
        $positiveWords = $this->getPositiveWords();
        $negativeWords = $this->getNegativeWords();

        $positiveCount = 0;
        $negativeCount = 0;
        $neutralCount = 0;
        $sentenceSentiments = [];

        foreach ($sentences as $sentence) {
            $words = str_word_count(strtolower($sentence), 1);
            $posInSentence = count(array_intersect($words, $positiveWords));
            $negInSentence = count(array_intersect($words, $negativeWords));

            $positiveCount += $posInSentence;
            $negativeCount += $negInSentence;

            if ($posInSentence > $negInSentence) {
                $sentenceSentiments[] = ['text' => $sentence, 'sentiment' => 'positive', 'score' => $posInSentence];
            } elseif ($negInSentence > $posInSentence) {
                $sentenceSentiments[] = ['text' => $sentence, 'sentiment' => 'negative', 'score' => -$negInSentence];
            } else {
                $sentenceSentiments[] = ['text' => $sentence, 'sentiment' => 'neutral', 'score' => 0];
                $neutralCount++;
            }
        }

        $total = $positiveCount + $negativeCount + $neutralCount;
        $total = max(1, $total);

        // Calculate overall sentiment score (-1 to 1)
        $score = ($positiveCount - $negativeCount) / max(1, $positiveCount + $negativeCount);

        return [
            'overall' => $score > 0.1 ? 'positive' : ($score < -0.1 ? 'negative' : 'neutral'),
            'score' => round($score, 3),
            'confidence' => min(1, ($positiveCount + $negativeCount) / max(1, str_word_count($text) / 10)),
            'breakdown' => [
                'positive' => round($positiveCount / $total, 3),
                'negative' => round($negativeCount / $total, 3),
                'neutral' => round($neutralCount / count($sentences), 3),
            ],
            'emotionalTone' => $this->detectEmotionalTone($text),
            'topSentences' => array_slice($sentenceSentiments, 0, 5),
        ];
    }

    /**
     * Extract keywords using TF-IDF approximation
     */
    public function extractKeywords(string $text, int $limit = 10): array
    {
        $words = str_word_count(strtolower($text), 1);
        $stopWords = $this->getStopWords();

        // Filter and count words
        $wordFreq = [];
        foreach ($words as $word) {
            if (strlen($word) < 3 || in_array($word, $stopWords, true)) {
                continue;
            }
            $wordFreq[$word] = ($wordFreq[$word] ?? 0) + 1;
        }

        // Calculate TF-IDF-like scores
        $totalWords = count($words);
        $scores = [];

        foreach ($wordFreq as $word => $freq) {
            // TF: term frequency
            $tf = $freq / $totalWords;

            // Approximate IDF using word length and frequency
            // (Real IDF would need a corpus)
            $idf = log(1 + (10 / $freq)) * (strlen($word) / 5);

            $scores[$word] = $tf * $idf * $freq;
        }

        arsort($scores);

        $keywords = [];
        $i = 0;
        foreach ($scores as $word => $score) {
            if ($i >= $limit) break;
            $keywords[] = [
                'word' => $word,
                'score' => round($score, 4),
                'frequency' => $wordFreq[$word],
            ];
            $i++;
        }

        // Also extract key phrases (bigrams)
        $phrases = $this->extractKeyPhrases($text, 5);

        return [
            'words' => $keywords,
            'phrases' => $phrases,
        ];
    }

    /**
     * Extract named entities
     */
    public function extractEntities(string $text): array
    {
        $entities = [
            'people' => [],
            'organizations' => [],
            'locations' => [],
            'dates' => [],
            'money' => [],
            'percentages' => [],
            'emails' => [],
            'urls' => [],
            'hashtags' => [],
            'mentions' => [],
        ];

        // Extract emails
        preg_match_all('/[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/', $text, $emails);
        $entities['emails'] = array_unique($emails[0]);

        // Extract URLs
        preg_match_all('/https?:\/\/[^\s<>"{}|\\^`\[\]]+/', $text, $urls);
        $entities['urls'] = array_unique($urls[0]);

        // Extract dates
        preg_match_all('/\b\d{1,2}[\/\-]\d{1,2}[\/\-]\d{2,4}\b|\b(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]* \d{1,2},? \d{4}\b/i', $text, $dates);
        $entities['dates'] = array_unique($dates[0]);

        // Extract money
        preg_match_all('/\$[\d,]+(?:\.\d{2})?|\b\d+(?:,\d{3})*(?:\.\d{2})?\s*(?:dollars?|USD|EUR|GBP)\b/i', $text, $money);
        $entities['money'] = array_unique($money[0]);

        // Extract percentages
        preg_match_all('/\b\d+(?:\.\d+)?%/', $text, $percentages);
        $entities['percentages'] = array_unique($percentages[0]);

        // Extract hashtags
        preg_match_all('/#\w+/', $text, $hashtags);
        $entities['hashtags'] = array_unique($hashtags[0]);

        // Extract mentions
        preg_match_all('/@\w+/', $text, $mentions);
        $entities['mentions'] = array_unique($mentions[0]);

        // Simple NER for capitalized sequences (names, organizations, places)
        preg_match_all('/\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)+)\b/', $text, $capitalizedPhrases);
        foreach ($capitalizedPhrases[1] as $phrase) {
            // Heuristic classification
            if ($this->looksLikePerson($phrase)) {
                $entities['people'][] = $phrase;
            } elseif ($this->looksLikeOrganization($phrase)) {
                $entities['organizations'][] = $phrase;
            } elseif ($this->looksLikeLocation($phrase)) {
                $entities['locations'][] = $phrase;
            }
        }

        // Dedupe
        foreach ($entities as $type => $items) {
            $entities[$type] = array_values(array_unique($items));
        }

        return $entities;
    }

    /**
     * Analyze SEO factors
     */
    public function analyzeSEO(array $content, string $text, array $options = []): array
    {
        $title = $options['title'] ?? ($content['title'] ?? '');
        $metaDescription = $options['metaDescription'] ?? '';
        $focusKeyword = $options['focusKeyword'] ?? '';

        $issues = [];
        $suggestions = [];
        $score = 100;

        $wordCount = str_word_count($text);
        $titleLength = strlen($title);
        $descriptionLength = strlen($metaDescription);

        // Title analysis
        if ($titleLength === 0) {
            $issues[] = ['type' => 'error', 'message' => 'Missing title'];
            $score -= 20;
        } elseif ($titleLength < 30) {
            $issues[] = ['type' => 'warning', 'message' => 'Title is too short (< 30 chars)'];
            $score -= 5;
        } elseif ($titleLength > 60) {
            $issues[] = ['type' => 'warning', 'message' => 'Title is too long (> 60 chars)'];
            $score -= 5;
        }

        // Meta description
        if ($descriptionLength === 0) {
            $suggestions[] = 'Add a meta description for better search visibility';
            $score -= 10;
        } elseif ($descriptionLength < 120) {
            $suggestions[] = 'Meta description is short, aim for 120-160 characters';
            $score -= 3;
        } elseif ($descriptionLength > 160) {
            $issues[] = ['type' => 'warning', 'message' => 'Meta description may be truncated (> 160 chars)'];
            $score -= 3;
        }

        // Content length
        if ($wordCount < 300) {
            $issues[] = ['type' => 'warning', 'message' => 'Content is thin (< 300 words)'];
            $score -= 15;
        } elseif ($wordCount >= 1500) {
            $suggestions[] = 'Long-form content (1500+ words) performs well in search';
        }

        // Focus keyword
        if ($focusKeyword) {
            $keywordInTitle = stripos($title, $focusKeyword) !== false;
            $keywordInContent = stripos($text, $focusKeyword) !== false;
            $keywordDensity = substr_count(strtolower($text), strtolower($focusKeyword)) / max(1, $wordCount) * 100;

            if (!$keywordInTitle) {
                $suggestions[] = "Add focus keyword '{$focusKeyword}' to the title";
                $score -= 10;
            }

            if (!$keywordInContent) {
                $issues[] = ['type' => 'error', 'message' => 'Focus keyword not found in content'];
                $score -= 15;
            } elseif ($keywordDensity < 0.5) {
                $suggestions[] = 'Increase focus keyword usage (aim for 1-2% density)';
                $score -= 5;
            } elseif ($keywordDensity > 3) {
                $issues[] = ['type' => 'warning', 'message' => 'Keyword stuffing detected (> 3% density)'];
                $score -= 10;
            }
        }

        // Headings analysis
        $hasH1 = preg_match('/<h1[^>]*>/', $this->portableText->toHtml($content['body'] ?? [])) ||
                 isset($content['title']);
        $headingCount = preg_match_all('/<h[2-6][^>]*>/', $this->portableText->toHtml($content['body'] ?? []));

        if (!$hasH1) {
            $suggestions[] = 'Add an H1 heading';
            $score -= 10;
        }

        if ($headingCount === 0 && $wordCount > 300) {
            $suggestions[] = 'Add subheadings (H2, H3) to improve readability';
            $score -= 5;
        }

        // Links
        $internalLinks = preg_match_all('/href=["\']\/?[^"\']+["\']/', $this->portableText->toHtml($content['body'] ?? []));
        $externalLinks = preg_match_all('/href=["\']https?:\/\//', $this->portableText->toHtml($content['body'] ?? []));

        if ($internalLinks === 0) {
            $suggestions[] = 'Add internal links to related content';
        }

        if ($externalLinks === 0 && $wordCount > 500) {
            $suggestions[] = 'Consider adding external links to authoritative sources';
        }

        // Images
        $images = preg_match_all('/_type.*?image/i', json_encode($content));
        if ($images === 0 && $wordCount > 300) {
            $suggestions[] = 'Add images to make content more engaging';
            $score -= 5;
        }

        return [
            'score' => max(0, min(100, $score)),
            'grade' => $this->seoScoreToGrade($score),
            'issues' => $issues,
            'suggestions' => $suggestions,
            'metrics' => [
                'titleLength' => $titleLength,
                'metaDescriptionLength' => $descriptionLength,
                'wordCount' => $wordCount,
                'headingCount' => $headingCount,
                'internalLinks' => $internalLinks,
                'externalLinks' => $externalLinks,
                'imageCount' => $images,
            ],
        ];
    }

    /**
     * Detect content language
     */
    public function detectLanguage(string $text): array
    {
        $sample = substr($text, 0, 1000);
        $languages = [
            'en' => $this->getEnglishIndicators(),
            'es' => ['el', 'la', 'de', 'que', 'y', 'en', 'un', 'es', 'por', 'con'],
            'fr' => ['le', 'la', 'de', 'et', 'les', 'des', 'en', 'un', 'une', 'que'],
            'de' => ['der', 'die', 'und', 'in', 'den', 'von', 'zu', 'das', 'mit', 'ist'],
            'pt' => ['de', 'a', 'o', 'que', 'e', 'do', 'da', 'em', 'um', 'para'],
            'it' => ['di', 'che', 'e', 'la', 'il', 'un', 'a', 'per', 'in', 'non'],
        ];

        $words = str_word_count(strtolower($sample), 1);
        $scores = [];

        foreach ($languages as $lang => $indicators) {
            $matches = count(array_intersect($words, $indicators));
            $scores[$lang] = $matches / max(1, count($words));
        }

        arsort($scores);
        $detected = array_key_first($scores);

        return [
            'detected' => $detected,
            'confidence' => round($scores[$detected], 3),
            'alternatives' => array_slice($scores, 1, 3, true),
        ];
    }

    /**
     * Generate content summary
     */
    public function generateSummary(string $text, int $maxLength = 150): string
    {
        $sentences = $this->splitIntoSentences($text);

        if (empty($sentences)) {
            return '';
        }

        // Score sentences by position and keyword density
        $keywords = $this->extractKeywords($text, 5)['words'] ?? [];
        $keywordList = array_column($keywords, 'word');

        $scored = [];
        foreach ($sentences as $index => $sentence) {
            $words = str_word_count(strtolower($sentence), 1);
            $keywordHits = count(array_intersect($words, $keywordList));

            // Position score (first sentences are important)
            $positionScore = 1 / ($index + 1);

            // Length score (prefer medium-length sentences)
            $lengthScore = min(1, strlen($sentence) / 100);

            $scored[] = [
                'sentence' => $sentence,
                'score' => $keywordHits + $positionScore + $lengthScore,
            ];
        }

        usort($scored, fn($a, $b) => $b['score'] <=> $a['score']);

        $summary = '';
        foreach ($scored as $item) {
            if (strlen($summary) + strlen($item['sentence']) > $maxLength) {
                break;
            }
            $summary .= $item['sentence'] . ' ';
        }

        return trim($summary) ?: substr($text, 0, $maxLength) . '...';
    }

    /**
     * Suggest tags based on content
     */
    public function suggestTags(string $text, array $keywords): array
    {
        $tags = [];

        // Add top keywords as tags
        foreach ($keywords['words'] ?? [] as $keyword) {
            if ($keyword['score'] > 0.001) {
                $tags[] = [
                    'tag' => ucfirst($keyword['word']),
                    'confidence' => min(1, $keyword['score'] * 100),
                    'source' => 'keyword',
                ];
            }
        }

        // Add key phrases as tags
        foreach ($keywords['phrases'] ?? [] as $phrase) {
            $tags[] = [
                'tag' => ucwords($phrase['phrase']),
                'confidence' => min(1, $phrase['score'] * 50),
                'source' => 'phrase',
            ];
        }

        // Sort by confidence
        usort($tags, fn($a, $b) => $b['confidence'] <=> $a['confidence']);

        return array_slice($tags, 0, 10);
    }

    /**
     * Suggest categories
     */
    public function suggestCategories(string $text, array $entities): array
    {
        // Simple topic classification based on keywords and entities
        $categories = [];
        $textLower = strtolower($text);

        $categoryPatterns = [
            'Technology' => ['technology', 'software', 'app', 'digital', 'computer', 'ai', 'machine learning', 'data'],
            'Business' => ['business', 'company', 'market', 'revenue', 'profit', 'investment', 'startup'],
            'Health' => ['health', 'medical', 'doctor', 'patient', 'treatment', 'disease', 'wellness'],
            'Science' => ['science', 'research', 'study', 'experiment', 'discovery', 'scientist'],
            'Finance' => ['finance', 'money', 'bank', 'stock', 'investment', 'economy', 'budget'],
            'Sports' => ['sports', 'game', 'team', 'player', 'score', 'championship', 'athlete'],
            'Entertainment' => ['movie', 'music', 'celebrity', 'film', 'actor', 'entertainment', 'show'],
            'Politics' => ['politics', 'government', 'election', 'policy', 'president', 'congress'],
            'Education' => ['education', 'school', 'university', 'student', 'learning', 'teacher'],
            'Travel' => ['travel', 'destination', 'vacation', 'hotel', 'flight', 'tourism'],
        ];

        foreach ($categoryPatterns as $category => $patterns) {
            $matches = 0;
            foreach ($patterns as $pattern) {
                $matches += substr_count($textLower, $pattern);
            }
            if ($matches > 0) {
                $categories[] = [
                    'category' => $category,
                    'confidence' => min(1, $matches / 10),
                    'matches' => $matches,
                ];
            }
        }

        usort($categories, fn($a, $b) => $b['confidence'] <=> $a['confidence']);

        return array_slice($categories, 0, 3);
    }

    /**
     * Assess overall content quality
     */
    public function assessContentQuality(string $text, ContentAnalysisResult $analysis): array
    {
        $score = 50; // Start at neutral
        $factors = [];

        // Readability factor
        $readabilityScore = $analysis->readability['score'] ?? 50;
        if ($readabilityScore >= 60) {
            $score += 10;
            $factors['readability'] = ['status' => 'good', 'impact' => '+10'];
        } elseif ($readabilityScore < 30) {
            $score -= 10;
            $factors['readability'] = ['status' => 'poor', 'impact' => '-10'];
        } else {
            $factors['readability'] = ['status' => 'average', 'impact' => '0'];
        }

        // Length factor
        $wordCount = str_word_count($text);
        if ($wordCount >= 1000) {
            $score += 15;
            $factors['length'] = ['status' => 'comprehensive', 'impact' => '+15'];
        } elseif ($wordCount >= 500) {
            $score += 10;
            $factors['length'] = ['status' => 'good', 'impact' => '+10'];
        } elseif ($wordCount < 200) {
            $score -= 10;
            $factors['length'] = ['status' => 'thin', 'impact' => '-10'];
        }

        // Keyword diversity
        $keywordCount = count($analysis->keywords['words'] ?? []);
        if ($keywordCount >= 8) {
            $score += 10;
            $factors['keywords'] = ['status' => 'diverse', 'impact' => '+10'];
        }

        // Sentiment consistency
        $sentimentConfidence = $analysis->sentiment['confidence'] ?? 0;
        if ($sentimentConfidence > 0.7) {
            $score += 5;
            $factors['sentiment'] = ['status' => 'clear', 'impact' => '+5'];
        }

        // SEO score
        $seoScore = $analysis->seo['score'] ?? 50;
        if ($seoScore >= 80) {
            $score += 15;
            $factors['seo'] = ['status' => 'optimized', 'impact' => '+15'];
        } elseif ($seoScore < 50) {
            $score -= 10;
            $factors['seo'] = ['status' => 'needs_work', 'impact' => '-10'];
        }

        return [
            'score' => max(0, min(100, $score)),
            'grade' => $this->qualityScoreToGrade($score),
            'factors' => $factors,
            'recommendations' => $this->generateQualityRecommendations($factors, $analysis),
        ];
    }

    /**
     * Content moderation
     */
    public function moderateContent(string $text): array
    {
        $flags = [];
        $textLower = strtolower($text);

        // Check for potentially inappropriate content patterns
        $toxicPatterns = $this->getToxicPatterns();

        foreach ($toxicPatterns as $category => $patterns) {
            foreach ($patterns as $pattern) {
                if (preg_match($pattern, $textLower)) {
                    $flags[] = [
                        'category' => $category,
                        'severity' => $this->getCategorySeverity($category),
                    ];
                    break;
                }
            }
        }

        $isSafe = empty($flags);
        $highestSeverity = $isSafe ? 'none' : max(array_column($flags, 'severity'));

        return [
            'safe' => $isSafe,
            'flags' => $flags,
            'severity' => $highestSeverity,
            'recommendation' => $isSafe ? 'Content appears safe' : 'Review flagged content before publishing',
        ];
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════

    private function extractText(array $content): string
    {
        if (isset($content['_type']) && $content['_type'] === 'block') {
            return $this->portableText->toPlainText([$content]);
        }

        if (isset($content['body'])) {
            return $this->portableText->toPlainText($content['body']);
        }

        if (isset($content['content'])) {
            if (is_array($content['content'])) {
                return $this->portableText->toPlainText($content['content']);
            }
            return $content['content'];
        }

        // Try to extract from any text fields
        $text = '';
        foreach (['title', 'excerpt', 'description', 'text'] as $field) {
            if (isset($content[$field]) && is_string($content[$field])) {
                $text .= $content[$field] . ' ';
            }
        }

        return trim($text);
    }

    private function countSentences(string $text): int
    {
        return max(1, preg_match_all('/[.!?]+/', $text));
    }

    private function splitIntoSentences(string $text): array
    {
        $sentences = preg_split('/(?<=[.!?])\s+/', $text, -1, PREG_SPLIT_NO_EMPTY);
        return array_filter($sentences, fn($s) => strlen(trim($s)) > 10);
    }

    private function countSyllables(string $text): int
    {
        $words = str_word_count(strtolower($text), 1);
        $count = 0;

        foreach ($words as $word) {
            $count += $this->countWordSyllables($word);
        }

        return max(1, $count);
    }

    private function countWordSyllables(string $word): int
    {
        $word = preg_replace('/[^a-z]/', '', strtolower($word));

        if (strlen($word) <= 3) {
            return 1;
        }

        $word = preg_replace('/(?:[^laeiouy]es|ed|[^laeiouy]e)$/', '', $word);
        $word = preg_replace('/^y/', '', $word);

        preg_match_all('/[aeiouy]+/', $word, $matches);

        return max(1, count($matches[0]));
    }

    private function countComplexWords(string $text): int
    {
        $words = str_word_count(strtolower($text), 1);
        $count = 0;

        foreach ($words as $word) {
            if ($this->countWordSyllables($word) >= 3) {
                $count++;
            }
        }

        return $count;
    }

    private function gradeToLabel(int|float $grade): string
    {
        return match (true) {
            $grade <= 5 => '5th grade or below',
            $grade <= 8 => '6th-8th grade',
            $grade <= 12 => 'High school',
            $grade <= 16 => 'College',
            default => 'Graduate level',
        };
    }

    private function scoreTodifficulty(float $score): string
    {
        return match (true) {
            $score >= 80 => 'very_easy',
            $score >= 60 => 'easy',
            $score >= 40 => 'moderate',
            $score >= 20 => 'difficult',
            default => 'very_difficult',
        };
    }

    private function seoScoreToGrade(int $score): string
    {
        return match (true) {
            $score >= 90 => 'A',
            $score >= 80 => 'B',
            $score >= 70 => 'C',
            $score >= 60 => 'D',
            default => 'F',
        };
    }

    private function qualityScoreToGrade(int $score): string
    {
        return match (true) {
            $score >= 85 => 'excellent',
            $score >= 70 => 'good',
            $score >= 50 => 'average',
            $score >= 30 => 'needs_improvement',
            default => 'poor',
        };
    }

    private function extractKeyPhrases(string $text, int $limit): array
    {
        $words = str_word_count(strtolower($text), 1);
        $stopWords = $this->getStopWords();
        $bigrams = [];

        for ($i = 0; $i < count($words) - 1; $i++) {
            $w1 = $words[$i];
            $w2 = $words[$i + 1];

            if (in_array($w1, $stopWords, true) || in_array($w2, $stopWords, true)) {
                continue;
            }

            if (strlen($w1) < 3 || strlen($w2) < 3) {
                continue;
            }

            $phrase = "{$w1} {$w2}";
            $bigrams[$phrase] = ($bigrams[$phrase] ?? 0) + 1;
        }

        arsort($bigrams);

        $phrases = [];
        $i = 0;
        foreach ($bigrams as $phrase => $freq) {
            if ($i >= $limit) break;
            if ($freq >= 2) {
                $phrases[] = ['phrase' => $phrase, 'frequency' => $freq, 'score' => $freq / count($words)];
                $i++;
            }
        }

        return $phrases;
    }

    private function detectEmotionalTone(string $text): string
    {
        $textLower = strtolower($text);

        $tones = [
            'formal' => ['therefore', 'consequently', 'furthermore', 'accordingly', 'notwithstanding'],
            'casual' => ["you'll", "we're", 'gonna', 'wanna', 'kinda', 'btw', 'lol'],
            'urgent' => ['immediately', 'urgent', 'asap', 'critical', 'now', 'hurry'],
            'persuasive' => ['must', 'should', 'need', 'important', 'essential', 'crucial'],
            'informative' => ['explains', 'describes', 'details', 'shows', 'demonstrates'],
        ];

        $scores = [];
        foreach ($tones as $tone => $indicators) {
            $matches = 0;
            foreach ($indicators as $word) {
                $matches += substr_count($textLower, $word);
            }
            $scores[$tone] = $matches;
        }

        arsort($scores);
        $topTone = array_key_first($scores);

        return $scores[$topTone] > 0 ? $topTone : 'neutral';
    }

    private function looksLikePerson(string $phrase): bool
    {
        $words = explode(' ', $phrase);
        return count($words) === 2 || count($words) === 3;
    }

    private function looksLikeOrganization(string $phrase): bool
    {
        $orgIndicators = ['Inc', 'Corp', 'LLC', 'Ltd', 'Company', 'Group', 'Foundation', 'Institute'];
        foreach ($orgIndicators as $indicator) {
            if (str_contains($phrase, $indicator)) {
                return true;
            }
        }
        return false;
    }

    private function looksLikeLocation(string $phrase): bool
    {
        $locationIndicators = ['City', 'State', 'County', 'Island', 'Mountain', 'River', 'Lake', 'Park'];
        foreach ($locationIndicators as $indicator) {
            if (str_contains($phrase, $indicator)) {
                return true;
            }
        }
        return false;
    }

    private function generateQualityRecommendations(array $factors, ContentAnalysisResult $analysis): array
    {
        $recommendations = [];

        if (($factors['readability']['status'] ?? '') === 'poor') {
            $recommendations[] = 'Simplify sentences and use more common words to improve readability';
        }

        if (($factors['length']['status'] ?? '') === 'thin') {
            $recommendations[] = 'Add more depth and detail to your content';
        }

        if (($factors['seo']['status'] ?? '') === 'needs_work') {
            $recommendations[] = 'Review SEO suggestions to improve search visibility';
        }

        if (empty($analysis->keywords['phrases'] ?? [])) {
            $recommendations[] = 'Use more descriptive phrases to strengthen your content';
        }

        return $recommendations;
    }

    private function getStopWords(): array
    {
        return ['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by', 'from', 'as', 'is', 'was', 'are', 'were', 'been', 'be', 'have', 'has', 'had', 'do', 'does', 'did', 'will', 'would', 'could', 'should', 'may', 'might', 'must', 'shall', 'can', 'need', 'dare', 'ought', 'used', 'it', 'its', 'this', 'that', 'these', 'those', 'i', 'you', 'he', 'she', 'we', 'they', 'what', 'which', 'who', 'whom', 'when', 'where', 'why', 'how', 'all', 'each', 'every', 'both', 'few', 'more', 'most', 'other', 'some', 'such', 'no', 'nor', 'not', 'only', 'own', 'same', 'so', 'than', 'too', 'very', 'just'];
    }

    private function getPositiveWords(): array
    {
        return ['good', 'great', 'excellent', 'amazing', 'wonderful', 'fantastic', 'outstanding', 'superb', 'brilliant', 'awesome', 'love', 'happy', 'joy', 'beautiful', 'best', 'perfect', 'success', 'successful', 'positive', 'win', 'winning', 'benefit', 'beneficial', 'improve', 'improvement', 'growth', 'growing', 'innovative', 'exciting', 'impressive'];
    }

    private function getNegativeWords(): array
    {
        return ['bad', 'terrible', 'awful', 'horrible', 'poor', 'worst', 'hate', 'sad', 'angry', 'upset', 'fail', 'failure', 'problem', 'issue', 'error', 'wrong', 'negative', 'loss', 'losing', 'decline', 'declining', 'difficult', 'hard', 'disappointing', 'disappointed', 'frustrating', 'frustrated', 'annoying', 'annoyed'];
    }

    private function getEnglishIndicators(): array
    {
        return ['the', 'and', 'is', 'are', 'was', 'were', 'have', 'has', 'been', 'will', 'would', 'could', 'should'];
    }

    private function getToxicPatterns(): array
    {
        // Return safe placeholder patterns - real implementation would have proper content moderation
        return [
            'spam' => ['/\b(buy now|click here|free money|act now)\b/i'],
            'inappropriate' => ['/\b(placeholder_for_moderation)\b/i'],
        ];
    }

    private function getCategorySeverity(string $category): string
    {
        return match ($category) {
            'inappropriate', 'hate' => 'high',
            'spam', 'misleading' => 'medium',
            default => 'low',
        };
    }
}

/**
 * Content Analysis Result DTO
 */
class ContentAnalysisResult
{
    public array $readability = [];
    public array $sentiment = [];
    public array $keywords = [];
    public array $entities = [];
    public array $seo = [];
    public array $language = [];
    public string $summary = '';
    public array $suggestedTags = [];
    public array $suggestedCategories = [];
    public array $contentQuality = [];
    public array $moderation = [];

    public function toArray(): array
    {
        return [
            'readability' => $this->readability,
            'sentiment' => $this->sentiment,
            'keywords' => $this->keywords,
            'entities' => $this->entities,
            'seo' => $this->seo,
            'language' => $this->language,
            'summary' => $this->summary,
            'suggestedTags' => $this->suggestedTags,
            'suggestedCategories' => $this->suggestedCategories,
            'contentQuality' => $this->contentQuality,
            'moderation' => $this->moderation,
        ];
    }

    public static function fromArray(array $data): self
    {
        $result = new self();
        foreach ($data as $key => $value) {
            if (property_exists($result, $key)) {
                $result->{$key} = $value;
            }
        }
        return $result;
    }
}
