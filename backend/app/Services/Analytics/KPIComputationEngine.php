<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsSession;
use App\Models\Analytics\AnalyticsKpiDefinition;
use App\Models\Analytics\AnalyticsKpiValue;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * KPIComputationEngine - Enterprise KPI Calculation Engine
 *
 * Computes KPIs across all modules with time-series aggregation,
 * trend analysis, and anomaly detection.
 *
 * @package App\Services\Analytics
 */
class KPIComputationEngine
{
    public function __construct(
        private readonly AnomalyDetectionEngine $anomalyDetector,
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    /**
     * Compute all KPIs for a period
     */
    public function computeAll(
        string $periodType,
        Carbon $periodStart,
        Carbon $periodEnd
    ): int {
        $definitions = AnalyticsKpiDefinition::active()->get();
        $computed = 0;

        foreach ($definitions as $definition) {
            try {
                $this->computeKpi($definition, $periodType, $periodStart, $periodEnd);
                $computed++;
            } catch (\Exception $e) {
                Log::error('KPI computation failed', [
                    'kpi' => $definition->kpi_key,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return $computed;
    }

    /**
     * Compute a single KPI
     */
    public function computeKpi(
        AnalyticsKpiDefinition $definition,
        string $periodType,
        Carbon $periodStart,
        Carbon $periodEnd,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): AnalyticsKpiValue {
        // Calculate the KPI value
        $value = $this->calculateValue($definition, $periodStart, $periodEnd, $segmentType, $segmentValue);

        // Get previous period value for comparison
        $previousPeriod = $this->getPreviousPeriod($periodType, $periodStart);
        $previousValue = $this->getPreviousValue(
            $definition,
            $periodType,
            $previousPeriod['start'],
            $segmentType,
            $segmentValue
        );

        // Generate period key
        $periodKey = $this->generatePeriodKey($periodType, $periodStart);

        // Create or update KPI value
        $kpiValue = AnalyticsKpiValue::updateOrCreate(
            [
                'kpi_definition_id' => $definition->id,
                'period_type' => $periodType,
                'period_start' => $periodStart,
                'segment_type' => $segmentType,
                'segment_value' => $segmentValue,
            ],
            [
                'period_end' => $periodEnd,
                'period_key' => $periodKey,
                'value' => $value,
                'previous_value' => $previousValue,
                'target_value' => $definition->target_value,
            ]
        );

        // Calculate change metrics
        $kpiValue->calculateChange();

        // Evaluate target status
        $kpiValue->evaluateTargetStatus();

        // Detect anomalies
        $this->detectAnomalies($kpiValue, $definition, $periodType);

        $kpiValue->save();

        // Invalidate cache
        $this->cacheManager->invalidateKpi($definition->kpi_key);

        return $kpiValue;
    }

    /**
     * Calculate KPI value based on definition
     */
    protected function calculateValue(
        AnalyticsKpiDefinition $definition,
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        return match ($definition->kpi_key) {
            'total_revenue' => $this->calculateTotalRevenue($start, $end, $segmentType, $segmentValue),
            'avg_order_value' => $this->calculateAvgOrderValue($start, $end, $segmentType, $segmentValue),
            'revenue_per_user' => $this->calculateRevenuePerUser($start, $end, $segmentType, $segmentValue),
            'total_sessions' => $this->calculateTotalSessions($start, $end, $segmentType, $segmentValue),
            'avg_session_duration' => $this->calculateAvgSessionDuration($start, $end, $segmentType, $segmentValue),
            'pages_per_session' => $this->calculatePagesPerSession($start, $end, $segmentType, $segmentValue),
            'bounce_rate' => $this->calculateBounceRate($start, $end, $segmentType, $segmentValue),
            'conversion_rate' => $this->calculateConversionRate($start, $end, $segmentType, $segmentValue),
            'total_conversions' => $this->calculateTotalConversions($start, $end, $segmentType, $segmentValue),
            'new_users' => $this->calculateNewUsers($start, $end, $segmentType, $segmentValue),
            'signups' => $this->calculateSignups($start, $end, $segmentType, $segmentValue),
            'returning_users' => $this->calculateReturningUsers($start, $end, $segmentType, $segmentValue),
            'retention_rate' => $this->calculateRetentionRate($start, $end, $segmentType, $segmentValue),
            default => $this->calculateGenericKpi($definition, $start, $end, $segmentType, $segmentValue),
        };
    }

    /**
     * Calculate total revenue
     */
    protected function calculateTotalRevenue(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsEvent::inDateRange($start, $end)->withRevenue();

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) $query->sum('revenue');
    }

    /**
     * Calculate average order value
     */
    protected function calculateAvgOrderValue(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsEvent::inDateRange($start, $end)
            ->byType(AnalyticsEvent::TYPE_PURCHASE)
            ->withRevenue();

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) ($query->avg('revenue') ?? 0);
    }

    /**
     * Calculate revenue per user
     */
    protected function calculateRevenuePerUser(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $revenue = $this->calculateTotalRevenue($start, $end, $segmentType, $segmentValue);

        $query = AnalyticsEvent::inDateRange($start, $end)->whereNotNull('user_id');

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        $uniqueUsers = $query->distinct('user_id')->count('user_id');

        return $uniqueUsers > 0 ? round($revenue / $uniqueUsers, 4) : 0;
    }

    /**
     * Calculate total sessions
     */
    protected function calculateTotalSessions(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsSession::inDateRange($start, $end);

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) $query->count();
    }

    /**
     * Calculate average session duration
     */
    protected function calculateAvgSessionDuration(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsSession::inDateRange($start, $end);

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) ($query->avg('duration_seconds') ?? 0);
    }

    /**
     * Calculate pages per session
     */
    protected function calculatePagesPerSession(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsSession::inDateRange($start, $end);

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return round((float) ($query->avg('page_views') ?? 0), 2);
    }

    /**
     * Calculate bounce rate
     */
    protected function calculateBounceRate(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $totalQuery = AnalyticsSession::inDateRange($start, $end);
        $bounceQuery = AnalyticsSession::inDateRange($start, $end)->bounces();

        if ($segmentType && $segmentValue) {
            $totalQuery->where($segmentType, $segmentValue);
            $bounceQuery->where($segmentType, $segmentValue);
        }

        $total = $totalQuery->count();
        $bounces = $bounceQuery->count();

        return $total > 0 ? round(($bounces / $total) * 100, 2) : 0;
    }

    /**
     * Calculate conversion rate
     */
    protected function calculateConversionRate(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $totalQuery = AnalyticsSession::inDateRange($start, $end);
        $convertedQuery = AnalyticsSession::inDateRange($start, $end)->withConversion();

        if ($segmentType && $segmentValue) {
            $totalQuery->where($segmentType, $segmentValue);
            $convertedQuery->where($segmentType, $segmentValue);
        }

        $total = $totalQuery->count();
        $converted = $convertedQuery->count();

        return $total > 0 ? round(($converted / $total) * 100, 2) : 0;
    }

    /**
     * Calculate total conversions
     */
    protected function calculateTotalConversions(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsEvent::inDateRange($start, $end)->conversions();

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) $query->count();
    }

    /**
     * Calculate new users
     */
    protected function calculateNewUsers(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsEvent::inDateRange($start, $end)
            ->byEventName('signup');

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) $query->count();
    }

    /**
     * Calculate signups
     */
    protected function calculateSignups(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        return $this->calculateNewUsers($start, $end, $segmentType, $segmentValue);
    }

    /**
     * Calculate returning users
     */
    protected function calculateReturningUsers(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        // Users who had a session before the period and have a session in this period
        $query = AnalyticsSession::inDateRange($start, $end)
            ->whereNotNull('user_id')
            ->whereIn('user_id', function ($subQuery) use ($start) {
                $subQuery->select('user_id')
                    ->from('analytics_sessions')
                    ->where('started_at', '<', $start)
                    ->whereNotNull('user_id');
            });

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        return (float) $query->distinct('user_id')->count('user_id');
    }

    /**
     * Calculate retention rate
     */
    protected function calculateRetentionRate(
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        // Get users from previous period
        $previousPeriod = $this->getPreviousPeriod('daily', $start);

        $previousUsers = AnalyticsSession::whereBetween('started_at', [
            $previousPeriod['start'],
            $previousPeriod['end'],
        ])->whereNotNull('user_id')->distinct('user_id')->pluck('user_id');

        if ($previousUsers->isEmpty()) {
            return 0;
        }

        // Count how many returned in current period
        $returnedUsers = AnalyticsSession::inDateRange($start, $end)
            ->whereIn('user_id', $previousUsers)
            ->distinct('user_id')
            ->count('user_id');

        return round(($returnedUsers / $previousUsers->count()) * 100, 2);
    }

    /**
     * Calculate generic KPI based on definition
     */
    protected function calculateGenericKpi(
        AnalyticsKpiDefinition $definition,
        Carbon $start,
        Carbon $end,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): float {
        $query = AnalyticsEvent::inDateRange($start, $end);

        // Apply filters from definition
        if ($definition->filters) {
            foreach ($definition->filters as $field => $value) {
                $query->where($field, $value);
            }
        }

        if ($segmentType && $segmentValue) {
            $query->where($segmentType, $segmentValue);
        }

        // Apply aggregation
        return (float) match ($definition->aggregation) {
            AnalyticsKpiDefinition::AGG_SUM => $query->sum($definition->metric_field ?? 'event_value'),
            AnalyticsKpiDefinition::AGG_AVG => $query->avg($definition->metric_field ?? 'event_value') ?? 0,
            AnalyticsKpiDefinition::AGG_MIN => $query->min($definition->metric_field ?? 'event_value') ?? 0,
            AnalyticsKpiDefinition::AGG_MAX => $query->max($definition->metric_field ?? 'event_value') ?? 0,
            AnalyticsKpiDefinition::AGG_COUNT => $query->count(),
            AnalyticsKpiDefinition::AGG_DISTINCT => $query->distinct($definition->metric_field ?? 'user_id')
                ->count($definition->metric_field ?? 'user_id'),
            default => $query->count(),
        };
    }

    /**
     * Get previous period for comparison
     */
    protected function getPreviousPeriod(string $periodType, Carbon $currentStart): array
    {
        return match ($periodType) {
            AnalyticsKpiValue::PERIOD_HOURLY => [
                'start' => $currentStart->copy()->subHour(),
                'end' => $currentStart->copy(),
            ],
            AnalyticsKpiValue::PERIOD_DAILY => [
                'start' => $currentStart->copy()->subDay(),
                'end' => $currentStart->copy(),
            ],
            AnalyticsKpiValue::PERIOD_WEEKLY => [
                'start' => $currentStart->copy()->subWeek(),
                'end' => $currentStart->copy(),
            ],
            AnalyticsKpiValue::PERIOD_MONTHLY => [
                'start' => $currentStart->copy()->subMonth(),
                'end' => $currentStart->copy(),
            ],
            default => [
                'start' => $currentStart->copy()->subDay(),
                'end' => $currentStart->copy(),
            ],
        };
    }

    /**
     * Get previous period value
     */
    protected function getPreviousValue(
        AnalyticsKpiDefinition $definition,
        string $periodType,
        Carbon $previousStart,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): ?float {
        $query = AnalyticsKpiValue::byKpi($definition->id)
            ->byPeriodType($periodType)
            ->where('period_start', $previousStart);

        if ($segmentType) {
            $query->bySegment($segmentType, $segmentValue);
        } else {
            $query->noSegment();
        }

        return $query->first()?->value;
    }

    /**
     * Generate period key for indexing
     */
    protected function generatePeriodKey(string $periodType, Carbon $start): int
    {
        return match ($periodType) {
            AnalyticsKpiValue::PERIOD_HOURLY => (int) $start->format('YmdH'),
            AnalyticsKpiValue::PERIOD_DAILY => (int) $start->format('Ymd'),
            AnalyticsKpiValue::PERIOD_WEEKLY => (int) ($start->format('Y') . $start->format('W')),
            AnalyticsKpiValue::PERIOD_MONTHLY => (int) $start->format('Ym'),
            AnalyticsKpiValue::PERIOD_YEARLY => (int) $start->format('Y'),
            default => (int) $start->format('Ymd'),
        };
    }

    /**
     * Detect anomalies in KPI value
     */
    protected function detectAnomalies(
        AnalyticsKpiValue $kpiValue,
        AnalyticsKpiDefinition $definition,
        string $periodType
    ): void {
        // Get historical values for baseline
        $historical = AnalyticsKpiValue::byKpi($definition->id)
            ->byPeriodType($periodType)
            ->noSegment()
            ->where('period_start', '<', $kpiValue->period_start)
            ->orderByDesc('period_start')
            ->limit(30)
            ->pluck('value');

        if ($historical->count() < 7) {
            return; // Not enough data for anomaly detection
        }

        $mean = $historical->avg();
        $stdDev = $this->calculateStdDev($historical);

        $kpiValue->detectAnomaly($mean, $stdDev);
    }

    /**
     * Calculate standard deviation
     */
    protected function calculateStdDev(Collection $values): float
    {
        $count = $values->count();
        if ($count < 2) {
            return 0;
        }

        $mean = $values->avg();
        $sumSquaredDiffs = $values->reduce(function ($carry, $value) use ($mean) {
            return $carry + pow($value - $mean, 2);
        }, 0);

        return sqrt($sumSquaredDiffs / ($count - 1));
    }

    /**
     * Get KPI dashboard data
     */
    public function getDashboardKpis(string $periodType = 'daily'): array
    {
        return $this->cacheManager->remember(
            "dashboard:kpis:{$periodType}",
            fn () => $this->computeDashboardKpis($periodType),
            300 // 5 minutes
        );
    }

    /**
     * Compute dashboard KPIs
     */
    protected function computeDashboardKpis(string $periodType): array
    {
        $definitions = AnalyticsKpiDefinition::forDashboard()->get();

        return $definitions->map(function ($definition) use ($periodType) {
            $latestValue = $definition->getLatestValue($periodType);

            return [
                'kpi_key' => $definition->kpi_key,
                'name' => $definition->name,
                'category' => $definition->category,
                'value' => $latestValue?->value ?? 0,
                'formatted_value' => $latestValue
                    ? $definition->formatValue($latestValue->value)
                    : $definition->formatValue(0),
                'change_percentage' => $latestValue?->change_percentage ?? 0,
                'trend' => $latestValue?->trend ?? 'stable',
                'target_status' => $latestValue?->target_status,
                'is_anomaly' => $latestValue?->is_anomaly ?? false,
                'icon' => $definition->icon,
                'color' => $definition->color,
                'is_primary' => $definition->is_primary,
            ];
        })->toArray();
    }
}
