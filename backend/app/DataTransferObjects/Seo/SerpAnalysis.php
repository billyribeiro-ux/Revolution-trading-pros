<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Seo;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Collection;
use JsonSerializable;

/**
 * SERP Analysis Data Transfer Object
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive SERP analysis result containing all available data points
 * from search engine results pages.
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
final class SerpAnalysis implements Arrayable, JsonSerializable
{
    /**
     * SERP feature constants
     */
    public const FEATURE_FEATURED_SNIPPET = 'featured_snippet';
    public const FEATURE_KNOWLEDGE_PANEL = 'knowledge_panel';
    public const FEATURE_LOCAL_PACK = 'local_pack';
    public const FEATURE_IMAGE_PACK = 'image_pack';
    public const FEATURE_VIDEO_CAROUSEL = 'video_carousel';
    public const FEATURE_TOP_STORIES = 'top_stories';
    public const FEATURE_PEOPLE_ALSO_ASK = 'people_also_ask';
    public const FEATURE_RELATED_SEARCHES = 'related_searches';
    public const FEATURE_SHOPPING_RESULTS = 'shopping_results';
    public const FEATURE_SITELINKS = 'sitelinks';
    public const FEATURE_REVIEWS = 'reviews';
    public const FEATURE_FAQ = 'faq';
    public const FEATURE_ADS_TOP = 'ads_top';
    public const FEATURE_ADS_BOTTOM = 'ads_bottom';

    public function __construct(
        public readonly string $keyword,
        public readonly ?int $totalResults = null,
        public readonly array $organicResults = [],
        public readonly array $serpFeatures = [],
        public readonly array $featuredSnippet = [],
        public readonly array $knowledgePanel = [],
        public readonly array $localPack = [],
        public readonly array $peopleAlsoAsk = [],
        public readonly array $relatedSearches = [],
        public readonly array $adsTop = [],
        public readonly array $adsBottom = [],
        public readonly ?int $adsCount = null,
        public readonly ?float $avgDomainAuthority = null,
        public readonly ?float $avgWordCount = null,
        public readonly array $topDomains = [],
        public readonly array $contentTypes = [],
        public readonly array $commonEntities = [],
        public readonly array $commonTopics = [],
        public readonly ?float $commercialIntent = null,
        public readonly ?string $dominantIntent = null,
        public readonly array $schemaTypes = [],
        public readonly ?string $dataSource = null,
        public readonly ?float $dataConfidence = null,
        public readonly ?\DateTimeImmutable $analyzedAt = null,
    ) {}

    /**
     * Create from Google Search Console derived data.
     * Note: GSC doesn't provide full SERP data, so this is limited.
     */
    public static function fromGoogleSearchConsole(string $keyword, array $gscData): self
    {
        $organicResults = [];

        // If we have your site's ranking, that's one data point
        if (isset($gscData['position']) && isset($gscData['url'])) {
            $organicResults[] = [
                'position' => (int) round($gscData['position']),
                'url' => $gscData['url'],
                'is_own_site' => true,
                'clicks' => $gscData['clicks'] ?? null,
                'impressions' => $gscData['impressions'] ?? null,
                'ctr' => $gscData['ctr'] ?? null,
            ];
        }

        return new self(
            keyword: $keyword,
            organicResults: $organicResults,
            dataSource: 'google_search_console',
            dataConfidence: 0.7, // Limited data
            analyzedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Create from SerpAPI data (full SERP data).
     */
    public static function fromSerpApi(array $data): self
    {
        $organicResults = [];
        foreach ($data['organic_results'] ?? [] as $result) {
            $organicResults[] = [
                'position' => $result['position'] ?? null,
                'title' => $result['title'] ?? '',
                'url' => $result['link'] ?? '',
                'domain' => parse_url($result['link'] ?? '', PHP_URL_HOST),
                'snippet' => $result['snippet'] ?? '',
                'is_own_site' => false,
            ];
        }

        $serpFeatures = [];
        if (isset($data['answer_box'])) {
            $serpFeatures[] = self::FEATURE_FEATURED_SNIPPET;
        }
        if (isset($data['knowledge_graph'])) {
            $serpFeatures[] = self::FEATURE_KNOWLEDGE_PANEL;
        }
        if (isset($data['local_results'])) {
            $serpFeatures[] = self::FEATURE_LOCAL_PACK;
        }
        if (isset($data['related_questions'])) {
            $serpFeatures[] = self::FEATURE_PEOPLE_ALSO_ASK;
        }
        if (isset($data['related_searches'])) {
            $serpFeatures[] = self::FEATURE_RELATED_SEARCHES;
        }
        if (isset($data['shopping_results'])) {
            $serpFeatures[] = self::FEATURE_SHOPPING_RESULTS;
        }
        if (isset($data['ads'])) {
            $serpFeatures[] = self::FEATURE_ADS_TOP;
        }

        $peopleAlsoAsk = [];
        foreach ($data['related_questions'] ?? [] as $question) {
            $peopleAlsoAsk[] = [
                'question' => $question['question'] ?? '',
                'answer' => $question['answer'] ?? null,
                'source' => $question['link'] ?? null,
            ];
        }

        $relatedSearches = [];
        foreach ($data['related_searches'] ?? [] as $search) {
            $relatedSearches[] = $search['query'] ?? $search;
        }

        return new self(
            keyword: $data['search_parameters']['q'] ?? '',
            totalResults: $data['search_information']['total_results'] ?? null,
            organicResults: $organicResults,
            serpFeatures: $serpFeatures,
            featuredSnippet: $data['answer_box'] ?? [],
            knowledgePanel: $data['knowledge_graph'] ?? [],
            localPack: $data['local_results'] ?? [],
            peopleAlsoAsk: $peopleAlsoAsk,
            relatedSearches: $relatedSearches,
            adsTop: $data['ads'] ?? [],
            adsCount: count($data['ads'] ?? []),
            dataSource: 'serpapi',
            dataConfidence: 1.0,
            analyzedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Get opportunities based on SERP analysis.
     */
    public function getOpportunities(): array
    {
        $opportunities = [];

        // Featured snippet opportunity
        if (!in_array(self::FEATURE_FEATURED_SNIPPET, $this->serpFeatures)) {
            $opportunities[] = [
                'type' => 'featured_snippet',
                'description' => 'No featured snippet present - opportunity to capture position zero',
                'priority' => 'high',
                'action' => 'Create concise, direct answers to the query in your content',
            ];
        }

        // People Also Ask opportunity
        if (!empty($this->peopleAlsoAsk)) {
            $opportunities[] = [
                'type' => 'people_also_ask',
                'description' => 'PAA questions found - answer these in your content',
                'priority' => 'high',
                'questions' => array_column($this->peopleAlsoAsk, 'question'),
                'action' => 'Add FAQ section addressing these questions',
            ];
        }

        // Low ad competition
        if (($this->adsCount ?? 0) < 2) {
            $opportunities[] = [
                'type' => 'low_competition',
                'description' => 'Low paid competition - organic rankings may be easier',
                'priority' => 'medium',
                'action' => 'Focus on comprehensive content to dominate organically',
            ];
        }

        // Schema opportunity
        if (empty($this->schemaTypes)) {
            $opportunities[] = [
                'type' => 'schema_markup',
                'description' => 'Implement rich schema markup for enhanced SERP appearance',
                'priority' => 'medium',
                'action' => 'Add appropriate schema types (Article, FAQ, HowTo, etc.)',
            ];
        }

        return $opportunities;
    }

    /**
     * Calculate SERP difficulty based on analysis.
     */
    public function calculateDifficulty(): int
    {
        $difficulty = 50; // Base difficulty

        // More ads = more commercial, potentially harder
        $difficulty += min(20, ($this->adsCount ?? 0) * 5);

        // Average domain authority impact
        if ($this->avgDomainAuthority !== null) {
            $difficulty += (int) (($this->avgDomainAuthority / 100) * 30);
        }

        // Total results impact
        if ($this->totalResults !== null) {
            if ($this->totalResults > 1000000000) {
                $difficulty += 15;
            } elseif ($this->totalResults > 100000000) {
                $difficulty += 10;
            } elseif ($this->totalResults > 10000000) {
                $difficulty += 5;
            }
        }

        // SERP features make it harder to get clicks
        $difficulty += count($this->serpFeatures) * 3;

        return min(100, max(0, $difficulty));
    }

    /**
     * Check if SERP has a specific feature.
     */
    public function hasFeature(string $feature): bool
    {
        return in_array($feature, $this->serpFeatures);
    }

    /**
     * Get click potential estimation.
     * Returns percentage of clicks organic results might receive.
     */
    public function getOrganicClickPotential(): float
    {
        $potential = 70.0; // Base organic click potential

        // Ads reduce organic clicks
        $potential -= min(20, ($this->adsCount ?? 0) * 5);

        // Featured snippet captures clicks
        if ($this->hasFeature(self::FEATURE_FEATURED_SNIPPET)) {
            $potential -= 15;
        }

        // Knowledge panel captures clicks
        if ($this->hasFeature(self::FEATURE_KNOWLEDGE_PANEL)) {
            $potential -= 10;
        }

        // Local pack captures clicks for local queries
        if ($this->hasFeature(self::FEATURE_LOCAL_PACK)) {
            $potential -= 10;
        }

        return max(20, $potential);
    }

    public function toArray(): array
    {
        return [
            'keyword' => $this->keyword,
            'total_results' => $this->totalResults,
            'organic_results_count' => count($this->organicResults),
            'organic_results' => $this->organicResults,
            'serp_features' => $this->serpFeatures,
            'featured_snippet' => $this->featuredSnippet,
            'knowledge_panel' => $this->knowledgePanel,
            'local_pack' => $this->localPack,
            'people_also_ask' => $this->peopleAlsoAsk,
            'related_searches' => $this->relatedSearches,
            'ads_count' => $this->adsCount,
            'avg_domain_authority' => $this->avgDomainAuthority,
            'avg_word_count' => $this->avgWordCount,
            'top_domains' => $this->topDomains,
            'content_types' => $this->contentTypes,
            'common_entities' => $this->commonEntities,
            'common_topics' => $this->commonTopics,
            'commercial_intent' => $this->commercialIntent,
            'dominant_intent' => $this->dominantIntent,
            'schema_types' => $this->schemaTypes,
            'difficulty' => $this->calculateDifficulty(),
            'organic_click_potential' => $this->getOrganicClickPotential(),
            'opportunities' => $this->getOpportunities(),
            'data_source' => $this->dataSource,
            'data_confidence' => $this->dataConfidence,
            'analyzed_at' => $this->analyzedAt?->format('c'),
        ];
    }

    public function jsonSerialize(): array
    {
        return $this->toArray();
    }
}
