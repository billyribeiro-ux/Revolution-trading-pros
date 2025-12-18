<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsCohort;
use App\Models\Analytics\AnalyticsCohortResult;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;

/**
 * CohortAnalyticsEngine - Enterprise Cohort Analysis
 *
 * Computes user cohorts and retention metrics for behavioral analysis.
 *
 * @package App\Services\Analytics
 */
class CohortAnalyticsEngine
{
    public function __construct(
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    /**
     * Compute cohort analysis for a cohort definition
     */
    public function computeCohort(
        AnalyticsCohort $cohort,
        Carbon $startDate,
        Carbon $endDate
    ): int {
        $computedPeriods = 0;

        // Get all cohort periods in the date range
        $periods = $this->getCohortPeriods($cohort, $startDate, $endDate);

        foreach ($periods as $periodStart) {
            // Get users who entered the cohort in this period
            $cohortUsers = $this->getCohortUsers($cohort, $periodStart);

            if ($cohortUsers->isEmpty()) {
                continue;
            }

            $cohortSize = $cohortUsers->count();
            $periodKey = $cohort->generatePeriodKey($periodStart);

            // Compute retention for each analysis period
            for ($period = 0; $period <= $cohort->analysis_periods; $period++) {
                $analysisDate = $this->getAnalysisDate($cohort, $periodStart, $period);

                // Skip future dates
                if ($analysisDate->isFuture()) {
                    continue;
                }

                $metrics = $this->computePeriodMetrics(
                    $cohort,
                    $cohortUsers,
                    $periodStart,
                    $period
                );

                AnalyticsCohortResult::updateOrCreate(
                    [
                        'cohort_id' => $cohort->id,
                        'cohort_date' => $periodStart,
                        'period_number' => $period,
                    ],
                    [
                        'cohort_period' => $periodKey,
                        'analysis_date' => $analysisDate,
                        'cohort_size' => $cohortSize,
                        'active_users' => $metrics['active_users'],
                        'retention_rate' => $metrics['retention_rate'],
                        'total_revenue' => $metrics['total_revenue'],
                        'avg_revenue_per_user' => $metrics['avg_revenue_per_user'],
                        'ltv_to_date' => $metrics['ltv_to_date'],
                        'total_events' => $metrics['total_events'],
                        'avg_events_per_user' => $metrics['avg_events_per_user'],
                        'engagement_score' => $metrics['engagement_score'],
                    ]
                );

                $computedPeriods++;
            }
        }

        // Invalidate cache
        $this->cacheManager->invalidateCohort($cohort->cohort_key);

        return $computedPeriods;
    }

    /**
     * Get cohort periods based on grouping
     */
    protected function getCohortPeriods(AnalyticsCohort $cohort, Carbon $start, Carbon $end): array
    {
        $periods = [];
        $current = $start->copy();

        while ($current->lte($end)) {
            $periods[] = $current->copy();

            $current = match ($cohort->grouping_period) {
                AnalyticsCohort::GROUP_DAILY => $current->addDay(),
                AnalyticsCohort::GROUP_WEEKLY => $current->addWeek(),
                AnalyticsCohort::GROUP_MONTHLY => $current->addMonth(),
                default => $current->addDay(),
            };
        }

        return $periods;
    }

    /**
     * Get users who entered cohort in a period
     */
    protected function getCohortUsers(AnalyticsCohort $cohort, Carbon $periodStart): Collection
    {
        $periodEnd = $this->getPeriodEnd($cohort, $periodStart);

        $query = AnalyticsEvent::query()
            ->byEventName($cohort->entry_event)
            ->whereBetween('event_timestamp', [$periodStart, $periodEnd])
            ->whereNotNull('user_id');

        // Apply entry conditions
        if ($cohort->entry_conditions) {
            foreach ($cohort->entry_conditions as $condition) {
                $this->applyCondition($query, $condition);
            }
        }

        return $query->distinct('user_id')->pluck('user_id');
    }

    /**
     * Get end of cohort period
     */
    protected function getPeriodEnd(AnalyticsCohort $cohort, Carbon $periodStart): Carbon
    {
        return match ($cohort->grouping_period) {
            AnalyticsCohort::GROUP_DAILY => $periodStart->copy()->endOfDay(),
            AnalyticsCohort::GROUP_WEEKLY => $periodStart->copy()->endOfWeek(),
            AnalyticsCohort::GROUP_MONTHLY => $periodStart->copy()->endOfMonth(),
            default => $periodStart->copy()->endOfDay(),
        };
    }

    /**
     * Get analysis date for a period number
     */
    protected function getAnalysisDate(AnalyticsCohort $cohort, Carbon $cohortStart, int $period): Carbon
    {
        return match ($cohort->grouping_period) {
            AnalyticsCohort::GROUP_DAILY => $cohortStart->copy()->addDays($period),
            AnalyticsCohort::GROUP_WEEKLY => $cohortStart->copy()->addWeeks($period),
            AnalyticsCohort::GROUP_MONTHLY => $cohortStart->copy()->addMonths($period),
            default => $cohortStart->copy()->addDays($period),
        };
    }

    /**
     * Compute metrics for a cohort period
     */
    protected function computePeriodMetrics(
        AnalyticsCohort $cohort,
        Collection $cohortUsers,
        Carbon $cohortStart,
        int $period
    ): array {
        if ($cohortUsers->isEmpty()) {
            return $this->emptyMetrics();
        }

        $analysisStart = $this->getAnalysisDate($cohort, $cohortStart, $period);
        $analysisEnd = $this->getPeriodEnd($cohort, $analysisStart);
        $cohortSize = $cohortUsers->count();

        // Get active users (users with retention event)
        $retentionEvent = $cohort->retention_event ?? 'page_view';

        $activeUsersQuery = AnalyticsEvent::query()
            ->byEventName($retentionEvent)
            ->whereBetween('event_timestamp', [$analysisStart, $analysisEnd])
            ->whereIn('user_id', $cohortUsers);

        // Apply retention conditions
        if ($cohort->retention_conditions) {
            foreach ($cohort->retention_conditions as $condition) {
                $this->applyCondition($activeUsersQuery, $condition);
            }
        }

        $activeUsers = $activeUsersQuery->distinct('user_id')->count('user_id');

        // Calculate revenue
        $revenue = AnalyticsEvent::query()
            ->whereBetween('event_timestamp', [$analysisStart, $analysisEnd])
            ->whereIn('user_id', $cohortUsers)
            ->withRevenue()
            ->sum('revenue');

        // Calculate cumulative LTV
        $ltvToDate = AnalyticsEvent::query()
            ->whereBetween('event_timestamp', [$cohortStart, $analysisEnd])
            ->whereIn('user_id', $cohortUsers)
            ->withRevenue()
            ->sum('revenue');

        // Calculate events
        $totalEvents = AnalyticsEvent::query()
            ->whereBetween('event_timestamp', [$analysisStart, $analysisEnd])
            ->whereIn('user_id', $cohortUsers)
            ->count();

        // Calculate engagement
        $avgEngagement = AnalyticsEvent::query()
            ->whereBetween('event_timestamp', [$analysisStart, $analysisEnd])
            ->whereIn('user_id', $cohortUsers)
            ->avg('engagement_score') ?? 0;

        return [
            'active_users' => $activeUsers,
            'retention_rate' => round(($activeUsers / $cohortSize) * 100, 4),
            'total_revenue' => (float) $revenue,
            'avg_revenue_per_user' => $activeUsers > 0 ? round($revenue / $activeUsers, 4) : 0,
            'ltv_to_date' => (float) $ltvToDate,
            'total_events' => $totalEvents,
            'avg_events_per_user' => $activeUsers > 0 ? round($totalEvents / $activeUsers, 2) : 0,
            'engagement_score' => round((float) $avgEngagement, 2),
        ];
    }

    /**
     * Apply condition to query
     */
    protected function applyCondition($query, array $condition): void
    {
        $field = $condition['field'] ?? null;
        $operator = $condition['operator'] ?? 'eq';
        $value = $condition['value'] ?? null;

        if (!$field) {
            return;
        }

        match ($operator) {
            'eq' => $query->where($field, $value),
            'neq' => $query->where($field, '!=', $value),
            'gt' => $query->where($field, '>', $value),
            'gte' => $query->where($field, '>=', $value),
            'lt' => $query->where($field, '<', $value),
            'lte' => $query->where($field, '<=', $value),
            'in' => $query->whereIn($field, (array) $value),
            'not_in' => $query->whereNotIn($field, (array) $value),
            default => null,
        };
    }

    /**
     * Get empty metrics array
     */
    protected function emptyMetrics(): array
    {
        return [
            'active_users' => 0,
            'retention_rate' => 0,
            'total_revenue' => 0,
            'avg_revenue_per_user' => 0,
            'ltv_to_date' => 0,
            'total_events' => 0,
            'avg_events_per_user' => 0,
            'engagement_score' => 0,
        ];
    }

    /**
     * Get cohort retention matrix
     */
    public function getRetentionMatrix(
        string $cohortKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $cacheKey = "cohort:matrix:{$cohortKey}:{$startDate->format('Ymd')}:{$endDate->format('Ymd')}";

        return $this->cacheManager->remember(
            $cacheKey,
            function () use ($cohortKey, $startDate, $endDate) {
                $cohort = AnalyticsCohort::where('cohort_key', $cohortKey)->firstOrFail();
                return $cohort->getRetentionMatrix($startDate, $endDate);
            },
            3600 // 1 hour
        );
    }

    /**
     * Get retention curve data
     */
    public function getRetentionCurve(
        string $cohortKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $cohort = AnalyticsCohort::where('cohort_key', $cohortKey)->firstOrFail();

        return AnalyticsCohortResult::getRetentionCurve($cohort->id, $startDate, $endDate);
    }

    /**
     * Get LTV analysis by cohort
     */
    public function getLTVAnalysis(
        string $cohortKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $cohort = AnalyticsCohort::where('cohort_key', $cohortKey)->firstOrFail();

        $results = AnalyticsCohortResult::byCohort($cohort->id)
            ->inDateRange($startDate, $endDate)
            ->orderBy('cohort_date')
            ->orderBy('period_number')
            ->get();

        $ltvByPeriod = $results->groupBy('period_number')->map(function ($group) {
            return [
                'avg_ltv' => round($group->avg('ltv_to_date'), 2),
                'max_ltv' => round($group->max('ltv_to_date'), 2),
                'total_revenue' => round($group->sum('total_revenue'), 2),
            ];
        });

        return [
            'cohort' => $cohortKey,
            'ltv_by_period' => $ltvByPeriod->toArray(),
            'total_cohorts' => $results->unique('cohort_date')->count(),
            'avg_cohort_size' => round($results->where('period_number', 0)->avg('cohort_size'), 0),
        ];
    }

    /**
     * Compare multiple cohorts
     */
    public function compareCohorts(array $cohortKeys, Carbon $startDate, Carbon $endDate): array
    {
        $comparison = [];

        foreach ($cohortKeys as $key) {
            $cohort = AnalyticsCohort::where('cohort_key', $key)->first();

            if (!$cohort) {
                continue;
            }

            $results = $cohort->results()
                ->inDateRange($startDate, $endDate)
                ->get();

            $comparison[$key] = [
                'name' => $cohort->name,
                'avg_retention_week_1' => round($results->where('period_number', 1)->avg('retention_rate'), 2),
                'avg_retention_week_4' => round($results->where('period_number', 4)->avg('retention_rate'), 2),
                'avg_retention_week_12' => round($results->where('period_number', 12)->avg('retention_rate'), 2),
                'avg_ltv' => round($results->max('ltv_to_date'), 2),
                'total_users' => $results->where('period_number', 0)->sum('cohort_size'),
            ];
        }

        return $comparison;
    }
}
