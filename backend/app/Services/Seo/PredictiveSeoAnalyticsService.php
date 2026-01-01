<?php

declare(strict_types=1);

namespace App\Services\Seo;

use App\DataTransferObjects\Seo\KeywordData;
use App\DataTransferObjects\Seo\KeywordGap;
use App\Services\CacheService;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

/**
 * Predictive SEO Analytics Service
 *
 * Apple Principal Engineer ICT11+ Implementation
 * ══════════════════════════════════════════════════════════════════════════════
 *
 * Advanced predictive analytics for SEO strategy optimization using
 * statistical modeling, trend analysis, and machine learning-ready algorithms.
 *
 * Core Capabilities:
 * - Traffic Prediction (based on ranking improvements)
 * - Seasonal Trend Analysis (detect and predict cycles)
 * - ROI Forecasting (estimate value of SEO efforts)
 * - Opportunity Scoring (multi-factor ML-ready algorithm)
 * - Competitive Landscape Analysis
 * - Growth Trajectory Modeling
 * - Content Decay Detection
 * - SERP Volatility Prediction
 *
 * Data Sources:
 * - Google Search Console (historical performance)
 * - Google Trends (seasonality signals)
 * - Google Keyword Planner (search volume data)
 *
 * @version 1.0.0
 * @level ICT11+ Principal Engineer
 */
class PredictiveSeoAnalyticsService
{
    private const CACHE_PREFIX = 'seo:predictive';
    private const CACHE_TTL = 3600; // 1 hour

    /**
     * CTR curve by position (based on industry studies).
     * Data from Advanced Web Ranking CTR studies.
     */
    private const CTR_CURVE = [
        1 => 0.319,
        2 => 0.158,
        3 => 0.100,
        4 => 0.068,
        5 => 0.051,
        6 => 0.042,
        7 => 0.033,
        8 => 0.029,
        9 => 0.025,
        10 => 0.022,
        11 => 0.015,
        12 => 0.013,
        13 => 0.011,
        14 => 0.010,
        15 => 0.009,
        16 => 0.008,
        17 => 0.007,
        18 => 0.006,
        19 => 0.005,
        20 => 0.004,
    ];

    /**
     * Seasonality patterns by industry/topic.
     */
    private const SEASONALITY_PATTERNS = [
        'retail' => [
            'peak_months' => [11, 12],
            'low_months' => [1, 2],
            'variance' => 0.4,
        ],
        'travel' => [
            'peak_months' => [5, 6, 7, 8],
            'low_months' => [1, 2, 11],
            'variance' => 0.35,
        ],
        'fitness' => [
            'peak_months' => [1, 9],
            'low_months' => [7, 8, 12],
            'variance' => 0.3,
        ],
        'finance' => [
            'peak_months' => [1, 4, 10],
            'low_months' => [6, 7, 8],
            'variance' => 0.2,
        ],
        'education' => [
            'peak_months' => [8, 9, 1],
            'low_months' => [6, 7, 12],
            'variance' => 0.25,
        ],
        'default' => [
            'peak_months' => [],
            'low_months' => [],
            'variance' => 0.1,
        ],
    ];

    /**
     * Opportunity score weights.
     */
    private const OPPORTUNITY_WEIGHTS = [
        'traffic_potential' => 0.30,
        'difficulty_inverse' => 0.20,
        'trend_signal' => 0.15,
        'commercial_value' => 0.15,
        'position_headroom' => 0.10,
        'ctr_potential' => 0.10,
    ];

    private SeoDataSourceManager $dataSourceManager;
    private CacheService $cache;

    public function __construct(
        SeoDataSourceManager $dataSourceManager,
        CacheService $cache
    ) {
        $this->dataSourceManager = $dataSourceManager;
        $this->cache = $cache;
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * TRAFFIC PREDICTION
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Predict traffic gain from ranking improvement.
     *
     * Uses CTR curve modeling and search volume data to estimate
     * monthly traffic if keyword moves to target position.
     */
    public function predictTrafficGain(KeywordData $keywordData, int $targetPosition): array
    {
        $currentPosition = $keywordData->position ?? 100;
        $searchVolume = $keywordData->searchVolume ?? ($keywordData->impressions ?? 0);

        // Get CTR for current and target positions
        $currentCtr = $this->getPositionCtr($currentPosition);
        $targetCtr = $this->getPositionCtr($targetPosition);

        // Calculate traffic
        $currentTraffic = $searchVolume * $currentCtr;
        $predictedTraffic = $searchVolume * $targetCtr;
        $trafficGain = $predictedTraffic - $currentTraffic;

        // Calculate percentage improvement
        $percentageGain = $currentTraffic > 0
            ? (($predictedTraffic - $currentTraffic) / $currentTraffic) * 100
            : 100;

        return [
            'keyword' => $keywordData->keyword,
            'current_position' => $currentPosition,
            'target_position' => $targetPosition,
            'current_ctr' => round($currentCtr * 100, 2),
            'target_ctr' => round($targetCtr * 100, 2),
            'search_volume' => $searchVolume,
            'current_traffic' => (int) round($currentTraffic),
            'predicted_traffic' => (int) round($predictedTraffic),
            'traffic_gain' => (int) round($trafficGain),
            'percentage_gain' => round($percentageGain, 1),
            'confidence' => $this->calculatePredictionConfidence($keywordData),
        ];
    }

    /**
     * Predict traffic for multiple keywords with ranking improvements.
     */
    public function predictPortfolioTraffic(Collection $keywords, array $targetPositions = []): array
    {
        $totalCurrentTraffic = 0;
        $totalPredictedTraffic = 0;
        $predictions = [];

        foreach ($keywords as $keywordData) {
            $keyword = $keywordData->keyword;
            $targetPosition = $targetPositions[$keyword] ?? $this->suggestTargetPosition($keywordData);

            $prediction = $this->predictTrafficGain($keywordData, $targetPosition);
            $predictions[] = $prediction;

            $totalCurrentTraffic += $prediction['current_traffic'];
            $totalPredictedTraffic += $prediction['predicted_traffic'];
        }

        return [
            'keywords_analyzed' => count($predictions),
            'total_current_traffic' => $totalCurrentTraffic,
            'total_predicted_traffic' => $totalPredictedTraffic,
            'total_traffic_gain' => $totalPredictedTraffic - $totalCurrentTraffic,
            'percentage_gain' => $totalCurrentTraffic > 0
                ? round((($totalPredictedTraffic - $totalCurrentTraffic) / $totalCurrentTraffic) * 100, 1)
                : 0,
            'predictions' => $predictions,
        ];
    }

    /**
     * Suggest realistic target position based on current ranking.
     */
    private function suggestTargetPosition(KeywordData $keywordData): int
    {
        $currentPosition = $keywordData->position ?? 100;

        // Realistic improvement targets
        if ($currentPosition <= 3) {
            return 1; // Already top 3, aim for #1
        } elseif ($currentPosition <= 10) {
            return max(1, $currentPosition - 3); // Move up 3 positions
        } elseif ($currentPosition <= 20) {
            return 10; // Aim for page 1
        } elseif ($currentPosition <= 50) {
            return 15; // Aim for page 2
        } else {
            return 30; // Aim for page 3
        }
    }

    /**
     * Get expected CTR for a given position.
     */
    private function getPositionCtr(int $position): float
    {
        if ($position <= 0) {
            return 0;
        }

        if (isset(self::CTR_CURVE[$position])) {
            return self::CTR_CURVE[$position];
        }

        // Extrapolate for positions beyond 20
        if ($position > 20 && $position <= 100) {
            return 0.004 * (100 - $position) / 80;
        }

        return 0.001;
    }

    /**
     * Calculate prediction confidence based on data quality.
     */
    private function calculatePredictionConfidence(KeywordData $keywordData): float
    {
        $confidence = 50; // Base confidence

        // Boost for actual position data
        if ($keywordData->position !== null && $keywordData->position > 0) {
            $confidence += 20;
        }

        // Boost for search volume data
        if ($keywordData->searchVolume !== null && $keywordData->searchVolume > 0) {
            $confidence += 15;
        }

        // Boost for click/impression data
        if ($keywordData->clicks !== null && $keywordData->impressions !== null) {
            $confidence += 15;
        }

        return min(100, $confidence);
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * ROI FORECASTING
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Calculate estimated ROI for SEO efforts on a keyword.
     *
     * @param float $averageOrderValue Average revenue per conversion
     * @param float $conversionRate Website conversion rate (0-1)
     */
    public function calculateKeywordRoi(
        KeywordData $keywordData,
        int $targetPosition,
        float $averageOrderValue = 50.0,
        float $conversionRate = 0.02
    ): array {
        $trafficPrediction = $this->predictTrafficGain($keywordData, $targetPosition);

        // Calculate potential revenue
        $monthlyTrafficGain = $trafficPrediction['traffic_gain'];
        $monthlyConversions = $monthlyTrafficGain * $conversionRate;
        $monthlyRevenue = $monthlyConversions * $averageOrderValue;
        $yearlyRevenue = $monthlyRevenue * 12;

        // Estimate effort/cost to achieve ranking
        $effortScore = $this->estimateRankingEffort($keywordData, $targetPosition);
        $estimatedHours = $effortScore * 10; // Convert to hours
        $estimatedCost = $estimatedHours * 100; // Assuming $100/hour

        // Calculate ROI
        $roi = $estimatedCost > 0
            ? (($yearlyRevenue - $estimatedCost) / $estimatedCost) * 100
            : 0;

        return [
            'keyword' => $keywordData->keyword,
            'current_position' => $keywordData->position,
            'target_position' => $targetPosition,
            'monthly_traffic_gain' => $monthlyTrafficGain,
            'monthly_conversions' => round($monthlyConversions, 1),
            'monthly_revenue' => round($monthlyRevenue, 2),
            'yearly_revenue' => round($yearlyRevenue, 2),
            'estimated_effort_hours' => round($estimatedHours, 1),
            'estimated_cost' => round($estimatedCost, 2),
            'roi_percentage' => round($roi, 1),
            'payback_months' => $monthlyRevenue > 0 ? round($estimatedCost / $monthlyRevenue, 1) : null,
            'cpc_equivalent' => $keywordData->cpc ?? null,
            'ppc_savings_yearly' => $keywordData->cpc !== null
                ? round($monthlyTrafficGain * $keywordData->cpc * 12, 2)
                : null,
        ];
    }

    /**
     * Estimate effort required to achieve ranking improvement.
     *
     * Returns effort score (1-100) based on:
     * - Current position
     * - Target position
     * - Keyword difficulty
     * - Competition level
     */
    private function estimateRankingEffort(KeywordData $keywordData, int $targetPosition): float
    {
        $currentPosition = $keywordData->position ?? 100;
        $positionDelta = $currentPosition - $targetPosition;

        // Base effort from position delta
        $effort = $positionDelta * 0.5;

        // Adjust for difficulty
        $difficulty = $keywordData->difficulty ?? 50;
        $effort += $difficulty * 0.3;

        // Adjust for target position (harder to reach top spots)
        if ($targetPosition <= 3) {
            $effort *= 1.5;
        } elseif ($targetPosition <= 10) {
            $effort *= 1.2;
        }

        return min(100, max(1, $effort));
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * OPPORTUNITY SCORING
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Calculate comprehensive opportunity score for a keyword.
     *
     * Uses multi-factor algorithm with configurable weights:
     * - Traffic potential (30%)
     * - Difficulty inverse (20%)
     * - Trend signal (15%)
     * - Commercial value (15%)
     * - Position headroom (10%)
     * - CTR potential (10%)
     */
    public function calculateOpportunityScore(KeywordData $keywordData): array
    {
        $factors = [];

        // 1. Traffic Potential (0-100)
        $trafficPotential = $this->calculateTrafficPotential($keywordData);
        $factors['traffic_potential'] = $trafficPotential;

        // 2. Difficulty Inverse (0-100)
        $difficulty = $keywordData->difficulty ?? 50;
        $difficultyInverse = 100 - $difficulty;
        $factors['difficulty_inverse'] = $difficultyInverse;

        // 3. Trend Signal (0-100)
        $trendSignal = $this->calculateTrendSignal($keywordData);
        $factors['trend_signal'] = $trendSignal;

        // 4. Commercial Value (0-100)
        $commercialValue = $this->calculateCommercialValue($keywordData);
        $factors['commercial_value'] = $commercialValue;

        // 5. Position Headroom (0-100)
        $positionHeadroom = $this->calculatePositionHeadroom($keywordData);
        $factors['position_headroom'] = $positionHeadroom;

        // 6. CTR Potential (0-100)
        $ctrPotential = $this->calculateCtrPotential($keywordData);
        $factors['ctr_potential'] = $ctrPotential;

        // Calculate weighted score
        $score = 0;
        foreach (self::OPPORTUNITY_WEIGHTS as $factor => $weight) {
            $score += ($factors[$factor] ?? 0) * $weight;
        }

        return [
            'keyword' => $keywordData->keyword,
            'opportunity_score' => (int) round($score),
            'factors' => $factors,
            'weights' => self::OPPORTUNITY_WEIGHTS,
            'recommendation' => $this->getScoreRecommendation($score),
        ];
    }

    /**
     * Calculate traffic potential score.
     */
    private function calculateTrafficPotential(KeywordData $keywordData): float
    {
        $searchVolume = $keywordData->searchVolume ?? ($keywordData->impressions ?? 0);

        // Normalize to 0-100 scale
        // 0 volume = 0, 10000+ volume = 100
        return min(100, ($searchVolume / 10000) * 100);
    }

    /**
     * Calculate trend signal score.
     */
    private function calculateTrendSignal(KeywordData $keywordData): float
    {
        $trend = $keywordData->trend ?? KeywordData::TREND_STABLE;
        $trendScore = $keywordData->trendScore ?? 50;

        $baseScore = match ($trend) {
            KeywordData::TREND_UP => 75,
            KeywordData::TREND_STABLE => 50,
            KeywordData::TREND_DOWN => 25,
            default => 50,
        };

        // Adjust with trend score
        if ($trendScore !== null) {
            return ($baseScore + $trendScore) / 2;
        }

        return $baseScore;
    }

    /**
     * Calculate commercial value score.
     */
    private function calculateCommercialValue(KeywordData $keywordData): float
    {
        $cpc = $keywordData->cpc ?? 0;
        $intent = $keywordData->intent ?? 'informational';

        // CPC-based value (0 = 0, $5+ = 50)
        $cpcScore = min(50, ($cpc / 5) * 50);

        // Intent-based value
        $intentScore = match ($intent) {
            'transactional' => 50,
            'commercial' => 40,
            'local' => 35,
            'navigational' => 20,
            'informational' => 10,
            default => 15,
        };

        return $cpcScore + $intentScore;
    }

    /**
     * Calculate position headroom score.
     * Higher score = more room for improvement.
     */
    private function calculatePositionHeadroom(KeywordData $keywordData): float
    {
        $position = $keywordData->position ?? 100;

        // Already #1 = 0 headroom, Position 100+ = 100 headroom
        // But positions 11-20 are "sweet spot" (high headroom, achievable)
        if ($position === 1) {
            return 0;
        } elseif ($position <= 3) {
            return 20;
        } elseif ($position <= 10) {
            return 40;
        } elseif ($position <= 20) {
            return 100; // Sweet spot - page 2 keywords
        } elseif ($position <= 50) {
            return 60;
        } else {
            return 30; // Too far to be realistic quick win
        }
    }

    /**
     * Calculate CTR improvement potential.
     */
    private function calculateCtrPotential(KeywordData $keywordData): float
    {
        $position = $keywordData->position ?? 100;
        $actualCtr = $keywordData->ctr ?? 0;
        $expectedCtr = $this->getPositionCtr($position) * 100;

        if ($expectedCtr <= 0) {
            return 50; // Unknown potential
        }

        // If actual CTR is below expected, there's optimization potential
        $ctrGap = $expectedCtr - $actualCtr;

        if ($ctrGap > 5) {
            return 100; // Significant CTR optimization potential
        } elseif ($ctrGap > 2) {
            return 75;
        } elseif ($ctrGap > 0) {
            return 50;
        } else {
            return 25; // Already performing above expected
        }
    }

    /**
     * Get recommendation based on opportunity score.
     */
    private function getScoreRecommendation(float $score): string
    {
        if ($score >= 80) {
            return 'Excellent opportunity - prioritize immediately';
        } elseif ($score >= 60) {
            return 'Good opportunity - include in content calendar';
        } elseif ($score >= 40) {
            return 'Moderate opportunity - consider if resources allow';
        } elseif ($score >= 20) {
            return 'Low opportunity - monitor for changes';
        } else {
            return 'Minimal opportunity - not recommended';
        }
    }

    /**
     * Rank opportunities across multiple keywords.
     *
     * @return Collection<array>
     */
    public function rankOpportunities(Collection $keywords): Collection
    {
        return $keywords
            ->map(fn(KeywordData $kw) => $this->calculateOpportunityScore($kw))
            ->sortByDesc(fn($score) => $score['opportunity_score'])
            ->values();
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * SEASONALITY ANALYSIS
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Analyze seasonality for a keyword.
     *
     * @param string $industry Industry/vertical for seasonality patterns
     */
    public function analyzeSeasonality(KeywordData $keywordData, string $industry = 'default'): array
    {
        $pattern = self::SEASONALITY_PATTERNS[$industry] ?? self::SEASONALITY_PATTERNS['default'];
        $currentMonth = (int) date('n');

        $isPeakSeason = in_array($currentMonth, $pattern['peak_months']);
        $isLowSeason = in_array($currentMonth, $pattern['low_months']);

        // Calculate seasonal multiplier
        $baseMultiplier = 1.0;
        if ($isPeakSeason) {
            $baseMultiplier = 1 + $pattern['variance'];
        } elseif ($isLowSeason) {
            $baseMultiplier = 1 - $pattern['variance'];
        }

        // Estimate seasonal traffic adjustment
        $baseVolume = $keywordData->searchVolume ?? 0;
        $adjustedVolume = (int) round($baseVolume * $baseMultiplier);

        return [
            'keyword' => $keywordData->keyword,
            'industry' => $industry,
            'current_month' => $currentMonth,
            'is_peak_season' => $isPeakSeason,
            'is_low_season' => $isLowSeason,
            'seasonal_multiplier' => round($baseMultiplier, 2),
            'base_search_volume' => $baseVolume,
            'adjusted_search_volume' => $adjustedVolume,
            'peak_months' => $pattern['peak_months'],
            'low_months' => $pattern['low_months'],
            'recommendations' => $this->getSeasonalRecommendations($isPeakSeason, $isLowSeason, $pattern),
        ];
    }

    /**
     * Get recommendations based on seasonality.
     */
    private function getSeasonalRecommendations(bool $isPeak, bool $isLow, array $pattern): array
    {
        $recommendations = [];

        if ($isPeak) {
            $recommendations[] = 'Maximize content exposure - high traffic period';
            $recommendations[] = 'Avoid major site changes during peak';
            $recommendations[] = 'Focus on conversion optimization';
        } elseif ($isLow) {
            $recommendations[] = 'Good time for content updates and technical SEO';
            $recommendations[] = 'Prepare content for upcoming peak season';
            $recommendations[] = 'Focus on building backlinks';
        } else {
            $recommendations[] = 'Standard optimization period';
            $recommendations[] = 'Balance content creation and technical improvements';
        }

        if (!empty($pattern['peak_months'])) {
            $peakNames = array_map(fn($m) => date('F', mktime(0, 0, 0, $m, 1)), $pattern['peak_months']);
            $recommendations[] = 'Peak months: ' . implode(', ', $peakNames);
        }

        return $recommendations;
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * CONTENT DECAY DETECTION
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Detect content decay based on performance trends.
     *
     * Identifies pages/keywords that were performing well but are declining.
     */
    public function detectContentDecay(Collection $keywordData, int $declineThreshold = 20): Collection
    {
        return $keywordData->filter(function (KeywordData $kw) use ($declineThreshold) {
            // Check for declining trend
            if ($kw->trend === KeywordData::TREND_DOWN) {
                return true;
            }

            // Check for significant position drop
            if ($kw->previousPosition !== null && $kw->position !== null) {
                $positionDrop = $kw->position - $kw->previousPosition;
                if ($positionDrop >= $declineThreshold) {
                    return true;
                }
            }

            // Check for CTR decline (if we had good CTR before)
            $expectedCtr = $this->getPositionCtr($kw->position ?? 100) * 100;
            if ($kw->ctr !== null && $expectedCtr > 0) {
                $ctrGap = $expectedCtr - $kw->ctr;
                if ($ctrGap > 5) {
                    return true;
                }
            }

            return false;
        })->map(function (KeywordData $kw) {
            return [
                'keyword' => $kw->keyword,
                'url' => $kw->url,
                'current_position' => $kw->position,
                'previous_position' => $kw->previousPosition,
                'trend' => $kw->trend,
                'decay_type' => $this->classifyDecayType($kw),
                'urgency' => $this->calculateDecayUrgency($kw),
                'recommendations' => $this->getDecayRecommendations($kw),
            ];
        })->values();
    }

    /**
     * Classify the type of content decay.
     */
    private function classifyDecayType(KeywordData $keywordData): string
    {
        if ($keywordData->previousPosition !== null && $keywordData->position !== null) {
            $positionDrop = $keywordData->position - $keywordData->previousPosition;
            if ($positionDrop >= 20) {
                return 'severe_ranking_drop';
            } elseif ($positionDrop >= 10) {
                return 'moderate_ranking_drop';
            }
        }

        if ($keywordData->trend === KeywordData::TREND_DOWN) {
            return 'traffic_decline';
        }

        $expectedCtr = $this->getPositionCtr($keywordData->position ?? 100) * 100;
        if ($keywordData->ctr !== null && $expectedCtr - $keywordData->ctr > 5) {
            return 'ctr_erosion';
        }

        return 'gradual_decline';
    }

    /**
     * Calculate urgency of content decay.
     */
    private function calculateDecayUrgency(KeywordData $keywordData): string
    {
        $searchVolume = $keywordData->searchVolume ?? 0;
        $positionDrop = 0;

        if ($keywordData->previousPosition !== null && $keywordData->position !== null) {
            $positionDrop = $keywordData->position - $keywordData->previousPosition;
        }

        // High-volume keywords with significant drops are urgent
        if ($searchVolume > 1000 && $positionDrop >= 10) {
            return 'critical';
        } elseif ($searchVolume > 500 || $positionDrop >= 10) {
            return 'high';
        } elseif ($keywordData->trend === KeywordData::TREND_DOWN) {
            return 'medium';
        }

        return 'low';
    }

    /**
     * Get recommendations for decaying content.
     */
    private function getDecayRecommendations(KeywordData $keywordData): array
    {
        $recommendations = [];
        $decayType = $this->classifyDecayType($keywordData);

        switch ($decayType) {
            case 'severe_ranking_drop':
                $recommendations[] = 'Audit content for quality issues immediately';
                $recommendations[] = 'Check for technical problems (crawl errors, 404s)';
                $recommendations[] = 'Analyze competitor content that replaced you';
                $recommendations[] = 'Consider complete content refresh';
                break;

            case 'moderate_ranking_drop':
                $recommendations[] = 'Update content with fresh information';
                $recommendations[] = 'Add missing topics identified in competitor analysis';
                $recommendations[] = 'Build additional internal links';
                break;

            case 'traffic_decline':
                $recommendations[] = 'Check for search volume changes in niche';
                $recommendations[] = 'Update publication date and content freshness';
                $recommendations[] = 'Expand content with related subtopics';
                break;

            case 'ctr_erosion':
                $recommendations[] = 'Rewrite title tag for better CTR';
                $recommendations[] = 'Update meta description with value proposition';
                $recommendations[] = 'Add structured data for rich snippets';
                break;

            default:
                $recommendations[] = 'Monitor for continued decline';
                $recommendations[] = 'Consider content refresh in next content cycle';
        }

        return $recommendations;
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * GROWTH TRAJECTORY MODELING
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Model growth trajectory for SEO program.
     *
     * Provides month-by-month projections for:
     * - Traffic growth
     * - Ranking improvements
     * - Revenue impact
     */
    public function modelGrowthTrajectory(
        Collection $keywords,
        int $monthsToProject = 12,
        float $monthlyEffortHours = 40
    ): array {
        $trajectory = [];
        $currentTraffic = $keywords->sum(fn(KeywordData $kw) => $kw->clicks ?? 0);

        // Estimate monthly improvement rate based on effort
        // More hours = faster improvement, but diminishing returns
        $monthlyImprovementRate = min(0.15, $monthlyEffortHours / 400);

        for ($month = 1; $month <= $monthsToProject; $month++) {
            // Calculate cumulative improvement
            $cumulativeGrowth = pow(1 + $monthlyImprovementRate, $month) - 1;
            $projectedTraffic = $currentTraffic * (1 + $cumulativeGrowth);

            // Estimate rankings improvement
            $avgCurrentPosition = $keywords->avg(fn(KeywordData $kw) => $kw->position ?? 50);
            $projectedPosition = max(1, $avgCurrentPosition * (1 - ($cumulativeGrowth * 0.3)));

            $trajectory[] = [
                'month' => $month,
                'projected_traffic' => (int) round($projectedTraffic),
                'traffic_growth_percentage' => round($cumulativeGrowth * 100, 1),
                'projected_avg_position' => round($projectedPosition, 1),
                'estimated_keywords_page1' => $this->estimateKeywordsOnPage1($keywords, $cumulativeGrowth),
            ];
        }

        return [
            'starting_traffic' => $currentTraffic,
            'monthly_effort_hours' => $monthlyEffortHours,
            'projected_monthly_improvement' => round($monthlyImprovementRate * 100, 1) . '%',
            'trajectory' => $trajectory,
            'summary' => [
                'month_3_traffic' => $trajectory[2]['projected_traffic'] ?? 0,
                'month_6_traffic' => $trajectory[5]['projected_traffic'] ?? 0,
                'month_12_traffic' => $trajectory[11]['projected_traffic'] ?? 0,
            ],
        ];
    }

    /**
     * Estimate number of keywords ranking on page 1.
     */
    private function estimateKeywordsOnPage1(Collection $keywords, float $growthFactor): int
    {
        $currentOnPage1 = $keywords->filter(fn(KeywordData $kw) => ($kw->position ?? 100) <= 10)->count();
        $almostOnPage1 = $keywords->filter(fn(KeywordData $kw) =>
            ($kw->position ?? 100) > 10 && ($kw->position ?? 100) <= 20
        )->count();

        // Estimate how many "almost there" keywords will make it
        $convertedKeywords = (int) round($almostOnPage1 * min(1, $growthFactor * 2));

        return $currentOnPage1 + $convertedKeywords;
    }

    /**
     * ═══════════════════════════════════════════════════════════════════════════
     * COMPETITIVE ANALYSIS
     * ═══════════════════════════════════════════════════════════════════════════
     */

    /**
     * Analyze competitive position for keywords.
     * Uses GSC data to understand where we stand vs competition.
     */
    public function analyzeCompetitivePosition(Collection $keywords): array
    {
        $totalKeywords = $keywords->count();

        // Position distribution
        $positionDistribution = [
            'top_3' => $keywords->filter(fn($kw) => ($kw->position ?? 100) <= 3)->count(),
            'top_10' => $keywords->filter(fn($kw) => ($kw->position ?? 100) <= 10)->count(),
            'page_2' => $keywords->filter(fn($kw) => ($kw->position ?? 100) > 10 && ($kw->position ?? 100) <= 20)->count(),
            'page_3_plus' => $keywords->filter(fn($kw) => ($kw->position ?? 100) > 20)->count(),
        ];

        // Calculate competitive strength score
        $strengthScore = $this->calculateCompetitiveStrengthScore($positionDistribution, $totalKeywords);

        // Identify competitive gaps
        $competitiveGaps = $keywords->filter(function (KeywordData $kw) {
            // High-value keywords where we're not competitive
            return ($kw->searchVolume ?? 0) > 500 && ($kw->position ?? 100) > 20;
        });

        return [
            'total_keywords_tracked' => $totalKeywords,
            'position_distribution' => $positionDistribution,
            'competitive_strength_score' => $strengthScore,
            'page_1_percentage' => $totalKeywords > 0
                ? round(($positionDistribution['top_10'] / $totalKeywords) * 100, 1)
                : 0,
            'top_3_percentage' => $totalKeywords > 0
                ? round(($positionDistribution['top_3'] / $totalKeywords) * 100, 1)
                : 0,
            'competitive_gaps_count' => $competitiveGaps->count(),
            'competitive_gaps' => $competitiveGaps->take(10)->map(fn($kw) => [
                'keyword' => $kw->keyword,
                'search_volume' => $kw->searchVolume,
                'current_position' => $kw->position,
            ])->values()->toArray(),
            'recommendations' => $this->getCompetitiveRecommendations($strengthScore, $positionDistribution),
        ];
    }

    /**
     * Calculate competitive strength score (0-100).
     */
    private function calculateCompetitiveStrengthScore(array $distribution, int $total): int
    {
        if ($total === 0) {
            return 0;
        }

        // Weighted score based on position distribution
        $score = 0;
        $score += ($distribution['top_3'] / $total) * 100 * 0.4;
        $score += ($distribution['top_10'] / $total) * 100 * 0.3;
        $score += ($distribution['page_2'] / $total) * 100 * 0.2;
        $score += ($distribution['page_3_plus'] / $total) * 100 * 0.1;

        return (int) round($score);
    }

    /**
     * Get competitive recommendations.
     */
    private function getCompetitiveRecommendations(int $strengthScore, array $distribution): array
    {
        $recommendations = [];

        if ($strengthScore >= 70) {
            $recommendations[] = 'Strong competitive position - focus on maintaining rankings';
            $recommendations[] = 'Target featured snippets for top-ranking keywords';
        } elseif ($strengthScore >= 40) {
            $recommendations[] = 'Moderate competitive position - focus on page 2 keywords';
            $recommendations[] = 'Prioritize quick wins (positions 11-20)';
        } else {
            $recommendations[] = 'Building competitive presence - focus on long-tail keywords';
            $recommendations[] = 'Create more comprehensive content to compete';
        }

        if ($distribution['page_2'] > 0) {
            $recommendations[] = "{$distribution['page_2']} keywords on page 2 - highest priority for improvement";
        }

        return $recommendations;
    }
}
