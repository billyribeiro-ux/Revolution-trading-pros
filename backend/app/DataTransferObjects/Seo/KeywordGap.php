<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Seo;

use Illuminate\Contracts\Support\Arrayable;
use JsonSerializable;

/**
 * Keyword Gap Data Transfer Object
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Represents a keyword gap/opportunity identified through analysis.
 * Supports both Google-only (GSC-based) and third-party data sources.
 *
 * Gap Types (Google-Only Mode):
 * - position_opportunity: Rankings 11-30 with high impressions
 * - ctr_opportunity: Low CTR despite good position
 * - declining_keyword: Position dropping over time
 * - untapped_impression: High impressions, no clicks
 * - cannibalization: Multiple URLs ranking for same keyword
 *
 * Gap Types (With Third-Party Data):
 * - competitor_gap: Competitor ranks, you don't
 * - content_gap: Missing topics/entities vs competitors
 * - serp_feature_gap: Competitors have SERP features, you don't
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
final class KeywordGap implements Arrayable, JsonSerializable
{
    /**
     * Gap type constants
     */
    public const TYPE_POSITION_OPPORTUNITY = 'position_opportunity';
    public const TYPE_CTR_OPPORTUNITY = 'ctr_opportunity';
    public const TYPE_DECLINING_KEYWORD = 'declining_keyword';
    public const TYPE_UNTAPPED_IMPRESSION = 'untapped_impression';
    public const TYPE_CANNIBALIZATION = 'cannibalization';
    public const TYPE_COMPETITOR_GAP = 'competitor_gap';
    public const TYPE_CONTENT_GAP = 'content_gap';
    public const TYPE_SERP_FEATURE_GAP = 'serp_feature_gap';
    public const TYPE_SEASONAL_OPPORTUNITY = 'seasonal_opportunity';
    public const TYPE_TRENDING_OPPORTUNITY = 'trending_opportunity';
    public const TYPE_LONG_TAIL_OPPORTUNITY = 'long_tail_opportunity';

    /**
     * Priority constants
     */
    public const PRIORITY_CRITICAL = 'critical';
    public const PRIORITY_HIGH = 'high';
    public const PRIORITY_MEDIUM = 'medium';
    public const PRIORITY_LOW = 'low';

    /**
     * Effort level constants
     */
    public const EFFORT_QUICK_WIN = 'quick_win';
    public const EFFORT_LOW = 'low';
    public const EFFORT_MEDIUM = 'medium';
    public const EFFORT_HIGH = 'high';

    public function __construct(
        public readonly string $keyword,
        public readonly string $gapType,
        public readonly string $description,
        public readonly string $priority,
        public readonly float $opportunityScore,
        public readonly ?float $estimatedTrafficGain = null,
        public readonly ?float $estimatedRevenueImpact = null,
        public readonly string $effort = self::EFFORT_MEDIUM,
        public readonly array $actions = [],
        public readonly ?int $currentPosition = null,
        public readonly ?int $targetPosition = null,
        public readonly ?int $impressions = null,
        public readonly ?int $clicks = null,
        public readonly ?float $ctr = null,
        public readonly ?int $searchVolume = null,
        public readonly ?string $affectedUrl = null,
        public readonly array $competitorUrls = [],
        public readonly array $missingEntities = [],
        public readonly array $missingTopics = [],
        public readonly array $missingSerpFeatures = [],
        public readonly array $relatedKeywords = [],
        public readonly ?string $dataSource = null,
        public readonly array $metadata = [],
        public readonly ?\DateTimeImmutable $identifiedAt = null,
    ) {}

    /**
     * Create a position opportunity gap (GSC-based).
     */
    public static function createPositionOpportunity(KeywordData $keywordData): self
    {
        $targetPosition = self::calculateTargetPosition($keywordData->currentPosition);
        $trafficGain = self::estimateTrafficGain(
            $keywordData->impressions,
            $keywordData->currentPosition,
            $targetPosition
        );

        return new self(
            keyword: $keywordData->keyword,
            gapType: self::TYPE_POSITION_OPPORTUNITY,
            description: sprintf(
                'Keyword "%s" is ranking at position %d with %d impressions. Moving to position %d could significantly increase clicks.',
                $keywordData->keyword,
                $keywordData->currentPosition,
                $keywordData->impressions ?? 0,
                $targetPosition
            ),
            priority: $keywordData->impressions > 5000 ? self::PRIORITY_HIGH : self::PRIORITY_MEDIUM,
            opportunityScore: $keywordData->opportunityScore ?? self::calculateOpportunityScore($keywordData),
            estimatedTrafficGain: $trafficGain,
            effort: $keywordData->currentPosition <= 20 ? self::EFFORT_LOW : self::EFFORT_MEDIUM,
            actions: [
                'Optimize on-page SEO for the target keyword',
                'Improve content depth and comprehensiveness',
                'Build internal links to the ranking page',
                'Acquire relevant backlinks',
            ],
            currentPosition: $keywordData->currentPosition,
            targetPosition: $targetPosition,
            impressions: $keywordData->impressions,
            clicks: $keywordData->clicks,
            ctr: $keywordData->ctr,
            searchVolume: $keywordData->searchVolume,
            affectedUrl: $keywordData->url,
            dataSource: 'google_search_console',
            identifiedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Create a CTR optimization opportunity (GSC-based).
     */
    public static function createCtrOpportunity(KeywordData $keywordData): self
    {
        $expectedCtr = self::getExpectedCtr($keywordData->currentPosition);
        $ctrGap = $expectedCtr - ($keywordData->ctr ?? 0);
        $potentialExtraClicks = (int) (($keywordData->impressions ?? 0) * ($ctrGap / 100));

        return new self(
            keyword: $keywordData->keyword,
            gapType: self::TYPE_CTR_OPPORTUNITY,
            description: sprintf(
                'Keyword "%s" has a CTR of %.1f%% at position %d. Expected CTR is ~%.1f%%. Optimizing title/meta could gain ~%d extra clicks.',
                $keywordData->keyword,
                $keywordData->ctr ?? 0,
                $keywordData->currentPosition ?? 0,
                $expectedCtr,
                $potentialExtraClicks
            ),
            priority: $potentialExtraClicks > 100 ? self::PRIORITY_HIGH : self::PRIORITY_MEDIUM,
            opportunityScore: min(100, ($ctrGap * 10) + 30),
            estimatedTrafficGain: (float) $potentialExtraClicks,
            effort: self::EFFORT_QUICK_WIN,
            actions: [
                'Rewrite the title tag to be more compelling and include the keyword',
                'Optimize meta description with clear value proposition and CTA',
                'Add structured data for rich snippets (reviews, FAQ, etc.)',
                'Use power words and numbers in titles',
            ],
            currentPosition: $keywordData->currentPosition,
            impressions: $keywordData->impressions,
            clicks: $keywordData->clicks,
            ctr: $keywordData->ctr,
            affectedUrl: $keywordData->url,
            dataSource: 'google_search_console',
            metadata: [
                'expected_ctr' => $expectedCtr,
                'ctr_gap' => $ctrGap,
            ],
            identifiedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Create a declining keyword alert (GSC-based).
     */
    public static function createDecliningKeyword(KeywordData $keywordData): self
    {
        return new self(
            keyword: $keywordData->keyword,
            gapType: self::TYPE_DECLINING_KEYWORD,
            description: sprintf(
                'Keyword "%s" has dropped from position %d to %d (%.1f positions). Immediate action recommended.',
                $keywordData->keyword,
                $keywordData->previousPosition ?? 0,
                $keywordData->currentPosition ?? 0,
                abs($keywordData->positionChange ?? 0)
            ),
            priority: ($keywordData->positionChange ?? 0) < -10 ? self::PRIORITY_CRITICAL : self::PRIORITY_HIGH,
            opportunityScore: 80, // Defending existing rankings is high priority
            effort: self::EFFORT_MEDIUM,
            actions: [
                'Audit the page for any recent changes or issues',
                'Check for new competitors ranking for this keyword',
                'Update and refresh the content',
                'Review and improve internal linking',
                'Check for technical SEO issues (speed, mobile, etc.)',
            ],
            currentPosition: $keywordData->currentPosition,
            targetPosition: $keywordData->previousPosition,
            impressions: $keywordData->impressions,
            clicks: $keywordData->clicks,
            affectedUrl: $keywordData->url,
            dataSource: 'google_search_console',
            metadata: [
                'position_change' => $keywordData->positionChange,
                'previous_position' => $keywordData->previousPosition,
            ],
            identifiedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Create a cannibalization issue (GSC-based).
     */
    public static function createCannibalization(string $keyword, array $competingUrls, array $metrics): self
    {
        return new self(
            keyword: $keyword,
            gapType: self::TYPE_CANNIBALIZATION,
            description: sprintf(
                'Keyword "%s" has %d pages competing for the same rankings. This dilutes ranking potential.',
                $keyword,
                count($competingUrls)
            ),
            priority: count($competingUrls) > 3 ? self::PRIORITY_HIGH : self::PRIORITY_MEDIUM,
            opportunityScore: 70,
            effort: self::EFFORT_MEDIUM,
            actions: [
                'Identify the primary page that should rank for this keyword',
                'Consolidate content from competing pages into the primary page',
                'Set up 301 redirects from secondary pages if appropriate',
                'Use canonical tags to indicate the preferred URL',
                'Differentiate secondary pages to target related but different keywords',
            ],
            impressions: $metrics['total_impressions'] ?? null,
            clicks: $metrics['total_clicks'] ?? null,
            competitorUrls: $competingUrls,
            dataSource: 'google_search_console',
            metadata: [
                'competing_pages_count' => count($competingUrls),
                'page_metrics' => $metrics['per_page'] ?? [],
            ],
            identifiedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Create a trending opportunity (Google Trends-based).
     */
    public static function createTrendingOpportunity(string $keyword, array $trendData): self
    {
        return new self(
            keyword: $keyword,
            gapType: self::TYPE_TRENDING_OPPORTUNITY,
            description: sprintf(
                'Keyword "%s" is trending with %d%% growth. Early content creation could capture rising demand.',
                $keyword,
                $trendData['growth_percentage'] ?? 0
            ),
            priority: ($trendData['growth_percentage'] ?? 0) > 100 ? self::PRIORITY_HIGH : self::PRIORITY_MEDIUM,
            opportunityScore: min(100, 50 + (($trendData['growth_percentage'] ?? 0) / 2)),
            effort: self::EFFORT_MEDIUM,
            actions: [
                'Create comprehensive content targeting this trending keyword',
                'Identify related trending keywords to include',
                'Consider time-sensitive content strategies',
                'Monitor trend direction and adjust strategy accordingly',
            ],
            searchVolume: $trendData['search_volume'] ?? null,
            relatedKeywords: $trendData['related_keywords'] ?? [],
            dataSource: 'google_trends',
            metadata: $trendData,
            identifiedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Calculate target position based on current position.
     */
    private static function calculateTargetPosition(?int $currentPosition): int
    {
        if ($currentPosition === null) {
            return 10;
        }

        if ($currentPosition <= 3) {
            return 1;
        } elseif ($currentPosition <= 10) {
            return 3;
        } elseif ($currentPosition <= 20) {
            return 10;
        }

        return 10;
    }

    /**
     * Estimate traffic gain from position improvement.
     */
    private static function estimateTrafficGain(?int $impressions, ?int $currentPosition, int $targetPosition): float
    {
        if ($impressions === null || $currentPosition === null) {
            return 0;
        }

        $currentCtr = self::getExpectedCtr($currentPosition);
        $targetCtr = self::getExpectedCtr($targetPosition);
        $ctrImprovement = $targetCtr - $currentCtr;

        return ($impressions * $ctrImprovement) / 100;
    }

    /**
     * Get expected CTR for a position.
     * Based on industry CTR curves.
     */
    private static function getExpectedCtr(?int $position): float
    {
        if ($position === null) {
            return 1.0;
        }

        $ctrCurve = [
            1 => 31.7,
            2 => 24.7,
            3 => 18.6,
            4 => 13.6,
            5 => 9.5,
            6 => 6.2,
            7 => 4.2,
            8 => 3.1,
            9 => 2.6,
            10 => 2.4,
        ];

        if (isset($ctrCurve[$position])) {
            return $ctrCurve[$position];
        }

        // Positions 11+ have very low CTR
        return max(0.5, 2.4 - (($position - 10) * 0.2));
    }

    /**
     * Calculate opportunity score from keyword data.
     */
    private static function calculateOpportunityScore(KeywordData $keywordData): float
    {
        return $keywordData->opportunityScore ?? 50;
    }

    /**
     * Check if this is a quick win.
     */
    public function isQuickWin(): bool
    {
        return $this->effort === self::EFFORT_QUICK_WIN
            || ($this->effort === self::EFFORT_LOW && $this->opportunityScore > 60);
    }

    /**
     * Get estimated time to implement.
     */
    public function getEstimatedTime(): string
    {
        return match ($this->effort) {
            self::EFFORT_QUICK_WIN => '1-2 hours',
            self::EFFORT_LOW => '2-4 hours',
            self::EFFORT_MEDIUM => '1-2 days',
            self::EFFORT_HIGH => '1+ week',
            default => 'Variable',
        };
    }

    public function toArray(): array
    {
        return [
            'keyword' => $this->keyword,
            'gap_type' => $this->gapType,
            'description' => $this->description,
            'priority' => $this->priority,
            'opportunity_score' => $this->opportunityScore,
            'estimated_traffic_gain' => $this->estimatedTrafficGain,
            'estimated_revenue_impact' => $this->estimatedRevenueImpact,
            'effort' => $this->effort,
            'estimated_time' => $this->getEstimatedTime(),
            'is_quick_win' => $this->isQuickWin(),
            'actions' => $this->actions,
            'current_position' => $this->currentPosition,
            'target_position' => $this->targetPosition,
            'impressions' => $this->impressions,
            'clicks' => $this->clicks,
            'ctr' => $this->ctr,
            'search_volume' => $this->searchVolume,
            'affected_url' => $this->affectedUrl,
            'competitor_urls' => $this->competitorUrls,
            'missing_entities' => $this->missingEntities,
            'missing_topics' => $this->missingTopics,
            'missing_serp_features' => $this->missingSerpFeatures,
            'related_keywords' => $this->relatedKeywords,
            'data_source' => $this->dataSource,
            'metadata' => $this->metadata,
            'identified_at' => $this->identifiedAt?->format('c'),
        ];
    }

    public function jsonSerialize(): array
    {
        return $this->toArray();
    }
}
