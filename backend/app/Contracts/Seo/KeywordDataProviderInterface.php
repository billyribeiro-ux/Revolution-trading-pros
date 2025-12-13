<?php

declare(strict_types=1);

namespace App\Contracts\Seo;

use App\DataTransferObjects\Seo\KeywordData;
use App\DataTransferObjects\Seo\KeywordOpportunity;
use App\DataTransferObjects\Seo\KeywordGap;
use App\DataTransferObjects\Seo\SerpAnalysis;
use Illuminate\Support\Collection;

/**
 * Keyword Data Provider Interface
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Defines the contract for all keyword data sources. Implementations include:
 * - Google Search Console (Primary - Your Site Data)
 * - Google Keyword Planner (Search Volume)
 * - Google Trends (Trending & Related)
 * - SerpAPI (Third-party SERP - Optional Fallback)
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
interface KeywordDataProviderInterface extends SeoDataProviderInterface
{
    /**
     * Capability constants for keyword providers
     */
    public const CAPABILITY_SEARCH_VOLUME = 'search_volume';
    public const CAPABILITY_KEYWORD_DIFFICULTY = 'keyword_difficulty';
    public const CAPABILITY_CPC_DATA = 'cpc_data';
    public const CAPABILITY_SERP_POSITION = 'serp_position';
    public const CAPABILITY_SERP_FEATURES = 'serp_features';
    public const CAPABILITY_COMPETITOR_DATA = 'competitor_data';
    public const CAPABILITY_TRENDING = 'trending';
    public const CAPABILITY_RELATED_KEYWORDS = 'related_keywords';
    public const CAPABILITY_QUESTIONS = 'questions';
    public const CAPABILITY_HISTORICAL_DATA = 'historical_data';
    public const CAPABILITY_CLICK_DATA = 'click_data';
    public const CAPABILITY_IMPRESSION_DATA = 'impression_data';
    public const CAPABILITY_CTR_DATA = 'ctr_data';
    public const CAPABILITY_INTENT_CLASSIFICATION = 'intent_classification';

    /**
     * Get keyword metrics and data.
     *
     * @param string $keyword The keyword to analyze
     * @param array $options Additional options (location, language, device, etc.)
     * @return KeywordData Comprehensive keyword data
     */
    public function getKeywordData(string $keyword, array $options = []): KeywordData;

    /**
     * Get keyword metrics for multiple keywords (batch).
     *
     * @param array<string> $keywords List of keywords
     * @param array $options Additional options
     * @return Collection<KeywordData>
     */
    public function getKeywordDataBatch(array $keywords, array $options = []): Collection;

    /**
     * Get search volume for a keyword.
     *
     * @param string $keyword The keyword
     * @param array $options Location, language, etc.
     * @return array{
     *     volume: int|null,
     *     volume_trend: string,
     *     seasonality: array|null,
     *     confidence: float
     * }
     */
    public function getSearchVolume(string $keyword, array $options = []): array;

    /**
     * Get keyword difficulty score.
     *
     * @param string $keyword The keyword
     * @return array{
     *     difficulty: int,
     *     difficulty_label: string,
     *     factors: array,
     *     confidence: float
     * }
     */
    public function getKeywordDifficulty(string $keyword): array;

    /**
     * Get related keywords with metrics.
     *
     * @param string $seedKeyword The seed keyword
     * @param int $limit Maximum number of results
     * @param array $options Filters and options
     * @return Collection<KeywordData>
     */
    public function getRelatedKeywords(string $seedKeyword, int $limit = 50, array $options = []): Collection;

    /**
     * Get question-based keywords (People Also Ask style).
     *
     * @param string $seedKeyword The seed keyword
     * @param int $limit Maximum number of questions
     * @return Collection<KeywordData>
     */
    public function getQuestionKeywords(string $seedKeyword, int $limit = 20): Collection;

    /**
     * Get long-tail keyword variations.
     *
     * @param string $seedKeyword The seed keyword
     * @param int $limit Maximum number of results
     * @return Collection<KeywordData>
     */
    public function getLongTailVariations(string $seedKeyword, int $limit = 50): Collection;

    /**
     * Analyze SERP for a keyword.
     *
     * @param string $keyword The keyword
     * @param array $options Search options (location, device, etc.)
     * @return SerpAnalysis Complete SERP analysis
     */
    public function analyzeSERP(string $keyword, array $options = []): SerpAnalysis;

    /**
     * Get keyword ranking position for a URL.
     *
     * @param string $keyword The keyword
     * @param string $url The URL to check
     * @param array $options Search options
     * @return array{
     *     position: int|null,
     *     page: int|null,
     *     url_found: string|null,
     *     serp_features: array,
     *     competitors: array
     * }
     */
    public function getKeywordPosition(string $keyword, string $url, array $options = []): array;

    /**
     * Get trending keywords in a topic/niche.
     *
     * @param string $topic The topic or niche
     * @param array $options Timeframe, location, etc.
     * @return Collection<KeywordData>
     */
    public function getTrendingKeywords(string $topic, array $options = []): Collection;

    /**
     * Get keyword intent classification.
     *
     * @param string $keyword The keyword
     * @return array{
     *     primary_intent: string,
     *     secondary_intent: string|null,
     *     confidence: float,
     *     signals: array
     * }
     */
    public function classifyIntent(string $keyword): array;

    /**
     * Get historical keyword performance data.
     *
     * @param string $keyword The keyword
     * @param int $days Number of days of history
     * @return array{
     *     data_points: array,
     *     trend: string,
     *     average_position: float|null,
     *     position_change: float|null,
     *     volatility: float
     * }
     */
    public function getHistoricalData(string $keyword, int $days = 90): array;
}
