<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsFunnel;
use App\Models\Analytics\AnalyticsFunnelStep;
use App\Models\Analytics\AnalyticsFunnelConversion;
use App\Models\Analytics\AnalyticsFunnelAggregate;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;

/**
 * FunnelAnalyticsEngine - Enterprise Funnel Analysis
 *
 * Computes multi-step conversion funnels with drop-off analysis.
 *
 * @package App\Services\Analytics
 */
class FunnelAnalyticsEngine
{
    public function __construct(
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    /**
     * Analyze funnel for a date range
     */
    public function analyzeFunnel(
        string $funnelKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $cacheKey = "funnel:analysis:{$funnelKey}:{$startDate->format('Ymd')}:{$endDate->format('Ymd')}";

        return $this->cacheManager->remember(
            $cacheKey,
            fn () => $this->computeFunnelAnalysis($funnelKey, $startDate, $endDate),
            1800 // 30 minutes
        );
    }

    /**
     * Compute funnel analysis
     */
    protected function computeFunnelAnalysis(
        string $funnelKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)
            ->with('steps')
            ->firstOrFail();

        return $funnel->analyze($startDate, $endDate);
    }

    /**
     * Get funnel breakdown by segment
     */
    public function getFunnelBySegment(
        string $funnelKey,
        string $segmentField,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)
            ->with('steps')
            ->firstOrFail();

        // Get unique segment values
        $segments = AnalyticsEvent::query()
            ->inDateRange($startDate, $endDate)
            ->whereNotNull($segmentField)
            ->distinct($segmentField)
            ->pluck($segmentField);

        $results = [];

        foreach ($segments as $segmentValue) {
            $results[$segmentValue] = $this->computeSegmentedFunnel(
                $funnel,
                $segmentField,
                $segmentValue,
                $startDate,
                $endDate
            );
        }

        return [
            'funnel_key' => $funnelKey,
            'segment_field' => $segmentField,
            'segments' => $results,
        ];
    }

    /**
     * Compute funnel for a segment
     */
    protected function computeSegmentedFunnel(
        AnalyticsFunnel $funnel,
        string $segmentField,
        string $segmentValue,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $steps = $funnel->steps;
        $results = [];
        $previousCount = null;

        foreach ($steps as $step) {
            $count = AnalyticsEvent::query()
                ->byEventName($step->event_name)
                ->inDateRange($startDate, $endDate)
                ->where($segmentField, $segmentValue)
                ->count(DB::raw('DISTINCT COALESCE(user_id, anonymous_id)'));

            $conversionRate = $previousCount !== null && $previousCount > 0
                ? round(($count / $previousCount) * 100, 2)
                : 100;

            $results[] = [
                'step' => $step->step_number,
                'name' => $step->name,
                'count' => $count,
                'conversion_rate' => $conversionRate,
            ];

            $previousCount = $count;
        }

        return $results;
    }

    /**
     * Get funnel time series
     */
    public function getFunnelTimeSeries(
        string $funnelKey,
        string $granularity,
        Carbon $startDate,
        Carbon $endDate
    ): Collection {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)->firstOrFail();

        return $funnel->getTimeSeries($granularity, $startDate, $endDate);
    }

    /**
     * Get drop-off analysis
     */
    public function getDropOffAnalysis(
        string $funnelKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)->firstOrFail();

        return $funnel->getDropOffAnalysis($startDate, $endDate);
    }

    /**
     * Compute funnel aggregates for a period
     */
    public function computeAggregates(
        string $funnelKey,
        string $periodType,
        Carbon $periodStart,
        Carbon $periodEnd
    ): AnalyticsFunnelAggregate {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)->firstOrFail();
        $analysis = $funnel->analyze($periodStart, $periodEnd);

        $stepMetrics = [];
        foreach ($analysis['steps'] as $step) {
            $stepMetrics[$step['step_number']] = [
                'name' => $step['name'],
                'entries' => $step['count'],
                'conversion_rate' => $step['conversion_rate'],
                'drop_off' => $step['drop_off'],
            ];
        }

        // Get previous period for comparison
        $previousStart = match ($periodType) {
            'daily' => $periodStart->copy()->subDay(),
            'weekly' => $periodStart->copy()->subWeek(),
            'monthly' => $periodStart->copy()->subMonth(),
            default => $periodStart->copy()->subDay(),
        };

        $previousAggregate = AnalyticsFunnelAggregate::where('funnel_id', $funnel->id)
            ->where('period_type', $periodType)
            ->where('period_start', $previousStart)
            ->first();

        return AnalyticsFunnelAggregate::updateOrCreate(
            [
                'funnel_id' => $funnel->id,
                'period_type' => $periodType,
                'period_start' => $periodStart,
            ],
            [
                'period_end' => $periodEnd,
                'total_entries' => $analysis['summary']['total_entries'] ?? 0,
                'total_conversions' => $analysis['summary']['total_conversions'] ?? 0,
                'conversion_rate' => $analysis['summary']['overall_conversion_rate'] ?? 0,
                'step_metrics' => $stepMetrics,
                'prev_conversion_rate' => $previousAggregate?->conversion_rate,
                'conversion_rate_change' => $previousAggregate
                    ? ($analysis['summary']['overall_conversion_rate'] ?? 0) - $previousAggregate->conversion_rate
                    : null,
            ]
        );
    }

    /**
     * Track user through funnel
     */
    public function trackFunnelProgress(
        string $funnelKey,
        AnalyticsEvent $event
    ): ?AnalyticsFunnelConversion {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)
            ->with('steps')
            ->first();

        if (!$funnel) {
            return null;
        }

        // Find matching step
        $matchingStep = $funnel->steps->firstWhere('event_name', $event->event_name);

        if (!$matchingStep) {
            return null;
        }

        // Get or create conversion record
        $conversion = AnalyticsFunnelConversion::firstOrCreate(
            [
                'funnel_id' => $funnel->id,
                'user_id' => $event->user_id,
                'anonymous_id' => $event->anonymous_id,
                'session_id' => $event->session_id,
                'is_converted' => false,
            ],
            [
                'started_at' => $event->event_timestamp,
                'started_date' => $event->event_timestamp->toDateString(),
                'entry_channel' => $event->channel,
                'utm_source' => $event->utm_source,
                'utm_campaign' => $event->utm_campaign,
            ]
        );

        // Check if step should be advanced
        if ($funnel->is_ordered && $matchingStep->step_number > $conversion->current_step + 1) {
            // Ordered funnel - user skipped steps
            return $conversion;
        }

        // Advance to step
        $conversion->advanceToStep($matchingStep->step_number);

        // Check if completed
        $totalSteps = $funnel->steps->count();
        if ($matchingStep->step_number >= $totalSteps) {
            $conversion->markConverted($event->revenue);
        }

        return $conversion;
    }

    /**
     * Get funnel comparison
     */
    public function compareFunnels(
        array $funnelKeys,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $comparison = [];

        foreach ($funnelKeys as $key) {
            $analysis = $this->analyzeFunnel($key, $startDate, $endDate);

            $comparison[$key] = [
                'name' => $analysis['funnel']['name'],
                'total_entries' => $analysis['summary']['total_entries'] ?? 0,
                'total_conversions' => $analysis['summary']['total_conversions'] ?? 0,
                'conversion_rate' => $analysis['summary']['overall_conversion_rate'] ?? 0,
                'steps' => count($analysis['steps']),
                'biggest_drop_off' => $analysis['summary']['biggest_drop_off_step'] ?? null,
            ];
        }

        return $comparison;
    }

    /**
     * Get funnel conversion time analysis
     */
    public function getConversionTimeAnalysis(
        string $funnelKey,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)->firstOrFail();

        $conversions = AnalyticsFunnelConversion::byFunnel($funnel->id)
            ->converted()
            ->inDateRange($startDate, $endDate)
            ->get();

        if ($conversions->isEmpty()) {
            return [
                'avg_time_hours' => 0,
                'median_time_hours' => 0,
                'min_time_hours' => 0,
                'max_time_hours' => 0,
                'distribution' => [],
            ];
        }

        $times = $conversions->map(fn ($c) => $c->getTimeToConvertHours())->filter();

        // Calculate time buckets
        $buckets = [
            '< 1 hour' => 0,
            '1-6 hours' => 0,
            '6-24 hours' => 0,
            '1-3 days' => 0,
            '3-7 days' => 0,
            '7+ days' => 0,
        ];

        foreach ($times as $time) {
            if ($time < 1) {
                $buckets['< 1 hour']++;
            } elseif ($time < 6) {
                $buckets['1-6 hours']++;
            } elseif ($time < 24) {
                $buckets['6-24 hours']++;
            } elseif ($time < 72) {
                $buckets['1-3 days']++;
            } elseif ($time < 168) {
                $buckets['3-7 days']++;
            } else {
                $buckets['7+ days']++;
            }
        }

        $sorted = $times->sort()->values();
        $count = $sorted->count();

        return [
            'avg_time_hours' => round($times->avg(), 2),
            'median_time_hours' => $count > 0 ? round($sorted[$count / 2], 2) : 0,
            'min_time_hours' => round($times->min(), 2),
            'max_time_hours' => round($times->max(), 2),
            'distribution' => $buckets,
        ];
    }

    /**
     * Get step-level analysis
     */
    public function getStepAnalysis(
        string $funnelKey,
        int $stepNumber,
        Carbon $startDate,
        Carbon $endDate
    ): array {
        $funnel = AnalyticsFunnel::where('funnel_key', $funnelKey)
            ->with('steps')
            ->firstOrFail();

        $step = $funnel->steps->firstWhere('step_number', $stepNumber);

        if (!$step) {
            return [];
        }

        // Get events for this step
        $events = AnalyticsEvent::query()
            ->byEventName($step->event_name)
            ->inDateRange($startDate, $endDate)
            ->get();

        // Breakdown by channel
        $byChannel = $events->groupBy('channel')->map(fn ($group) => [
            'count' => $group->count(),
            'unique_users' => $group->unique('user_id')->count(),
        ]);

        // Breakdown by device
        $byDevice = $events->groupBy('device_type')->map(fn ($group) => [
            'count' => $group->count(),
            'unique_users' => $group->unique('user_id')->count(),
        ]);

        // Time distribution
        $byHour = $events->groupBy(fn ($e) => $e->event_timestamp->format('H'))
            ->map(fn ($group) => $group->count());

        return [
            'step_number' => $stepNumber,
            'step_name' => $step->name,
            'event_name' => $step->event_name,
            'total_events' => $events->count(),
            'unique_users' => $events->unique('user_id')->count(),
            'by_channel' => $byChannel->toArray(),
            'by_device' => $byDevice->toArray(),
            'by_hour' => $byHour->toArray(),
        ];
    }
}
