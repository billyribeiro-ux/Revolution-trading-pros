<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Seo;

use Illuminate\Contracts\Support\Arrayable;
use JsonSerializable;

/**
 * Keyword Data Transfer Object
 *
 * Apple Principal Engineer ICT11+ Architecture
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Immutable value object representing comprehensive keyword data.
 * Aggregates data from multiple providers into a unified structure.
 *
 * @version 2.0.0
 * @level ICT11+ Principal Engineer
 */
final class KeywordData implements Arrayable, JsonSerializable
{
    /**
     * Intent classification constants
     */
    public const INTENT_INFORMATIONAL = 'informational';
    public const INTENT_NAVIGATIONAL = 'navigational';
    public const INTENT_COMMERCIAL = 'commercial';
    public const INTENT_TRANSACTIONAL = 'transactional';

    /**
     * Trend direction constants
     */
    public const TREND_UP = 'up';
    public const TREND_DOWN = 'down';
    public const TREND_STABLE = 'stable';
    public const TREND_VOLATILE = 'volatile';
    public const TREND_SEASONAL = 'seasonal';

    /**
     * Difficulty level constants
     */
    public const DIFFICULTY_EASY = 'easy';
    public const DIFFICULTY_MEDIUM = 'medium';
    public const DIFFICULTY_HARD = 'hard';
    public const DIFFICULTY_VERY_HARD = 'very_hard';

    public function __construct(
        public readonly string $keyword,
        public readonly ?int $searchVolume = null,
        public readonly ?int $difficulty = null,
        public readonly ?string $difficultyLabel = null,
        public readonly ?float $cpc = null,
        public readonly ?float $competition = null,
        public readonly string $intent = self::INTENT_INFORMATIONAL,
        public readonly ?float $intentConfidence = null,
        public readonly ?int $currentPosition = null,
        public readonly ?int $previousPosition = null,
        public readonly ?float $positionChange = null,
        public readonly ?int $clicks = null,
        public readonly ?int $impressions = null,
        public readonly ?float $ctr = null,
        public readonly string $trend = self::TREND_STABLE,
        public readonly ?float $trendScore = null,
        public readonly ?float $opportunityScore = null,
        public readonly ?float $priorityScore = null,
        public readonly array $serpFeatures = [],
        public readonly array $relatedKeywords = [],
        public readonly array $questions = [],
        public readonly array $seasonality = [],
        public readonly array $historicalPositions = [],
        public readonly ?string $url = null,
        public readonly ?string $dataSource = null,
        public readonly ?float $dataConfidence = null,
        public readonly array $metadata = [],
        public readonly ?\DateTimeImmutable $fetchedAt = null,
    ) {}

    /**
     * Create from array data.
     */
    public static function fromArray(array $data): self
    {
        return new self(
            keyword: $data['keyword'] ?? '',
            searchVolume: $data['search_volume'] ?? $data['searchVolume'] ?? null,
            difficulty: $data['difficulty'] ?? $data['difficulty_score'] ?? null,
            difficultyLabel: $data['difficulty_label'] ?? $data['difficultyLabel'] ?? null,
            cpc: isset($data['cpc']) ? (float) $data['cpc'] : null,
            competition: isset($data['competition']) ? (float) $data['competition'] : null,
            intent: $data['intent'] ?? self::INTENT_INFORMATIONAL,
            intentConfidence: isset($data['intent_confidence']) ? (float) $data['intent_confidence'] : null,
            currentPosition: $data['current_position'] ?? $data['position'] ?? null,
            previousPosition: $data['previous_position'] ?? null,
            positionChange: isset($data['position_change']) ? (float) $data['position_change'] : null,
            clicks: $data['clicks'] ?? null,
            impressions: $data['impressions'] ?? null,
            ctr: isset($data['ctr']) ? (float) $data['ctr'] : null,
            trend: $data['trend'] ?? self::TREND_STABLE,
            trendScore: isset($data['trend_score']) ? (float) $data['trend_score'] : null,
            opportunityScore: isset($data['opportunity_score']) ? (float) $data['opportunity_score'] : null,
            priorityScore: isset($data['priority_score']) ? (float) $data['priority_score'] : null,
            serpFeatures: $data['serp_features'] ?? [],
            relatedKeywords: $data['related_keywords'] ?? [],
            questions: $data['questions'] ?? [],
            seasonality: $data['seasonality'] ?? [],
            historicalPositions: $data['historical_positions'] ?? [],
            url: $data['url'] ?? null,
            dataSource: $data['data_source'] ?? null,
            dataConfidence: isset($data['data_confidence']) ? (float) $data['data_confidence'] : null,
            metadata: $data['metadata'] ?? [],
            fetchedAt: isset($data['fetched_at']) ? new \DateTimeImmutable($data['fetched_at']) : new \DateTimeImmutable(),
        );
    }

    /**
     * Create from Google Search Console data.
     */
    public static function fromGoogleSearchConsole(array $gscData): self
    {
        $positionChange = null;
        if (isset($gscData['position']) && isset($gscData['previous_position'])) {
            $positionChange = $gscData['previous_position'] - $gscData['position'];
        }

        return new self(
            keyword: $gscData['query'] ?? $gscData['keyword'] ?? '',
            currentPosition: isset($gscData['position']) ? (int) round($gscData['position']) : null,
            previousPosition: isset($gscData['previous_position']) ? (int) round($gscData['previous_position']) : null,
            positionChange: $positionChange,
            clicks: $gscData['clicks'] ?? null,
            impressions: $gscData['impressions'] ?? null,
            ctr: isset($gscData['ctr']) ? round($gscData['ctr'] * 100, 2) : null,
            url: $gscData['page'] ?? $gscData['url'] ?? null,
            trend: self::determineTrend($positionChange),
            dataSource: 'google_search_console',
            dataConfidence: 1.0, // GSC data is authoritative for your site
            fetchedAt: new \DateTimeImmutable(),
        );
    }

    /**
     * Merge data from another KeywordData object.
     * Prioritizes non-null values from the other object.
     */
    public function merge(KeywordData $other): self
    {
        return new self(
            keyword: $this->keyword,
            searchVolume: $other->searchVolume ?? $this->searchVolume,
            difficulty: $other->difficulty ?? $this->difficulty,
            difficultyLabel: $other->difficultyLabel ?? $this->difficultyLabel,
            cpc: $other->cpc ?? $this->cpc,
            competition: $other->competition ?? $this->competition,
            intent: $other->intent !== self::INTENT_INFORMATIONAL ? $other->intent : $this->intent,
            intentConfidence: $other->intentConfidence ?? $this->intentConfidence,
            currentPosition: $other->currentPosition ?? $this->currentPosition,
            previousPosition: $other->previousPosition ?? $this->previousPosition,
            positionChange: $other->positionChange ?? $this->positionChange,
            clicks: $other->clicks ?? $this->clicks,
            impressions: $other->impressions ?? $this->impressions,
            ctr: $other->ctr ?? $this->ctr,
            trend: $other->trend !== self::TREND_STABLE ? $other->trend : $this->trend,
            trendScore: $other->trendScore ?? $this->trendScore,
            opportunityScore: $other->opportunityScore ?? $this->opportunityScore,
            priorityScore: $other->priorityScore ?? $this->priorityScore,
            serpFeatures: array_merge($this->serpFeatures, $other->serpFeatures),
            relatedKeywords: array_unique(array_merge($this->relatedKeywords, $other->relatedKeywords)),
            questions: array_unique(array_merge($this->questions, $other->questions)),
            seasonality: $other->seasonality ?: $this->seasonality,
            historicalPositions: array_merge($this->historicalPositions, $other->historicalPositions),
            url: $other->url ?? $this->url,
            dataSource: $this->mergeDataSources($other->dataSource),
            dataConfidence: max($this->dataConfidence ?? 0, $other->dataConfidence ?? 0),
            metadata: array_merge($this->metadata, $other->metadata),
            fetchedAt: $other->fetchedAt ?? $this->fetchedAt,
        );
    }

    /**
     * Calculate opportunity score based on available data.
     */
    public function withCalculatedOpportunityScore(): self
    {
        $score = $this->calculateOpportunityScore();

        return new self(
            keyword: $this->keyword,
            searchVolume: $this->searchVolume,
            difficulty: $this->difficulty,
            difficultyLabel: $this->difficultyLabel,
            cpc: $this->cpc,
            competition: $this->competition,
            intent: $this->intent,
            intentConfidence: $this->intentConfidence,
            currentPosition: $this->currentPosition,
            previousPosition: $this->previousPosition,
            positionChange: $this->positionChange,
            clicks: $this->clicks,
            impressions: $this->impressions,
            ctr: $this->ctr,
            trend: $this->trend,
            trendScore: $this->trendScore,
            opportunityScore: $score,
            priorityScore: $this->calculatePriorityScore($score),
            serpFeatures: $this->serpFeatures,
            relatedKeywords: $this->relatedKeywords,
            questions: $this->questions,
            seasonality: $this->seasonality,
            historicalPositions: $this->historicalPositions,
            url: $this->url,
            dataSource: $this->dataSource,
            dataConfidence: $this->dataConfidence,
            metadata: $this->metadata,
            fetchedAt: $this->fetchedAt,
        );
    }

    /**
     * Calculate opportunity score (0-100).
     * Higher score = better opportunity.
     */
    private function calculateOpportunityScore(): float
    {
        $score = 50.0; // Base score

        // Position factor (positions 11-30 are best opportunities)
        if ($this->currentPosition !== null) {
            if ($this->currentPosition >= 11 && $this->currentPosition <= 20) {
                $score += 25; // Great opportunity - just off first page
            } elseif ($this->currentPosition >= 4 && $this->currentPosition <= 10) {
                $score += 20; // Good - can push to top 3
            } elseif ($this->currentPosition >= 21 && $this->currentPosition <= 30) {
                $score += 15; // Moderate opportunity
            } elseif ($this->currentPosition <= 3) {
                $score += 10; // Already ranking well, maintain
            } else {
                $score -= 10; // Too far down, harder to improve
            }
        }

        // Impressions factor (high impressions = proven demand)
        if ($this->impressions !== null) {
            if ($this->impressions > 10000) {
                $score += 20;
            } elseif ($this->impressions > 1000) {
                $score += 15;
            } elseif ($this->impressions > 100) {
                $score += 10;
            }
        }

        // CTR factor (low CTR with high impressions = optimization opportunity)
        if ($this->ctr !== null && $this->impressions !== null && $this->impressions > 100) {
            if ($this->ctr < 2) {
                $score += 15; // Low CTR = title/meta optimization needed
            } elseif ($this->ctr < 5) {
                $score += 10;
            }
        }

        // Difficulty factor (lower difficulty = easier win)
        if ($this->difficulty !== null) {
            if ($this->difficulty < 30) {
                $score += 15;
            } elseif ($this->difficulty < 50) {
                $score += 10;
            } elseif ($this->difficulty < 70) {
                $score += 5;
            } else {
                $score -= 10;
            }
        }

        // Trend factor
        if ($this->trend === self::TREND_UP) {
            $score += 10;
        } elseif ($this->trend === self::TREND_DOWN) {
            $score -= 5;
        }

        // Intent factor (commercial/transactional = higher value)
        if ($this->intent === self::INTENT_TRANSACTIONAL) {
            $score += 15;
        } elseif ($this->intent === self::INTENT_COMMERCIAL) {
            $score += 10;
        }

        return min(100, max(0, $score));
    }

    /**
     * Calculate priority score combining opportunity and effort.
     */
    private function calculatePriorityScore(float $opportunityScore): float
    {
        // Estimate effort (inverse of opportunity in some ways)
        $effortScore = 50.0;

        if ($this->difficulty !== null) {
            $effortScore = $this->difficulty;
        }

        // Priority = Opportunity / Effort (higher = better priority)
        $priority = $effortScore > 0
            ? ($opportunityScore / $effortScore) * 50
            : $opportunityScore;

        return min(100, max(0, $priority));
    }

    /**
     * Determine trend from position change.
     */
    private static function determineTrend(?float $positionChange): string
    {
        if ($positionChange === null) {
            return self::TREND_STABLE;
        }

        if ($positionChange > 5) {
            return self::TREND_UP;
        } elseif ($positionChange < -5) {
            return self::TREND_DOWN;
        }

        return self::TREND_STABLE;
    }

    /**
     * Merge data source identifiers.
     */
    private function mergeDataSources(?string $otherSource): string
    {
        if (!$this->dataSource && !$otherSource) {
            return 'unknown';
        }

        if (!$this->dataSource) {
            return $otherSource;
        }

        if (!$otherSource) {
            return $this->dataSource;
        }

        if ($this->dataSource === $otherSource) {
            return $this->dataSource;
        }

        return $this->dataSource . '+' . $otherSource;
    }

    /**
     * Check if this keyword is a quick win opportunity.
     */
    public function isQuickWin(): bool
    {
        // Quick win: Position 11-20 with high impressions
        return $this->currentPosition !== null
            && $this->currentPosition >= 11
            && $this->currentPosition <= 20
            && ($this->impressions ?? 0) > 500;
    }

    /**
     * Check if this keyword needs CTR optimization.
     */
    public function needsCtrOptimization(): bool
    {
        return $this->ctr !== null
            && $this->ctr < 3
            && ($this->impressions ?? 0) > 1000
            && ($this->currentPosition ?? 100) <= 10;
    }

    /**
     * Check if this keyword is declining.
     */
    public function isDeclining(): bool
    {
        return $this->trend === self::TREND_DOWN
            || ($this->positionChange !== null && $this->positionChange < -3);
    }

    /**
     * Get difficulty label from score.
     */
    public function getDifficultyLabel(): string
    {
        if ($this->difficultyLabel) {
            return $this->difficultyLabel;
        }

        if ($this->difficulty === null) {
            return 'unknown';
        }

        if ($this->difficulty < 30) {
            return self::DIFFICULTY_EASY;
        } elseif ($this->difficulty < 50) {
            return self::DIFFICULTY_MEDIUM;
        } elseif ($this->difficulty < 70) {
            return self::DIFFICULTY_HARD;
        }

        return self::DIFFICULTY_VERY_HARD;
    }

    public function toArray(): array
    {
        return [
            'keyword' => $this->keyword,
            'search_volume' => $this->searchVolume,
            'difficulty' => $this->difficulty,
            'difficulty_label' => $this->getDifficultyLabel(),
            'cpc' => $this->cpc,
            'competition' => $this->competition,
            'intent' => $this->intent,
            'intent_confidence' => $this->intentConfidence,
            'current_position' => $this->currentPosition,
            'previous_position' => $this->previousPosition,
            'position_change' => $this->positionChange,
            'clicks' => $this->clicks,
            'impressions' => $this->impressions,
            'ctr' => $this->ctr,
            'trend' => $this->trend,
            'trend_score' => $this->trendScore,
            'opportunity_score' => $this->opportunityScore,
            'priority_score' => $this->priorityScore,
            'is_quick_win' => $this->isQuickWin(),
            'needs_ctr_optimization' => $this->needsCtrOptimization(),
            'is_declining' => $this->isDeclining(),
            'serp_features' => $this->serpFeatures,
            'related_keywords' => $this->relatedKeywords,
            'questions' => $this->questions,
            'seasonality' => $this->seasonality,
            'url' => $this->url,
            'data_source' => $this->dataSource,
            'data_confidence' => $this->dataConfidence,
            'metadata' => $this->metadata,
            'fetched_at' => $this->fetchedAt?->format('c'),
        ];
    }

    public function jsonSerialize(): array
    {
        return $this->toArray();
    }
}
