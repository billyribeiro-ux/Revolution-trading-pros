<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\Analytics\EventIngestionService;
use App\Services\Analytics\KPIComputationEngine;
use App\Services\Analytics\CohortAnalyticsEngine;
use App\Services\Analytics\FunnelAnalyticsEngine;
use App\Services\Analytics\AttributionEngine;
use App\Services\Analytics\ForecastingEngine;
use App\Services\Analytics\AnomalyDetectionEngine;
use App\Services\Analytics\AnalyticsCacheManager;
use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsKpiDefinition;
use App\Models\Analytics\AnalyticsCohort;
use App\Models\Analytics\AnalyticsFunnel;
use App\Models\Analytics\AnalyticsSegment;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Carbon;

/**
 * AnalyticsController - Enterprise Analytics API
 *
 * Comprehensive API for analytics dashboard, KPIs, funnels, cohorts,
 * attribution, forecasting, and real-time metrics.
 */
class AnalyticsController extends Controller
{
    public function __construct(
        private readonly EventIngestionService $ingestionService,
        private readonly KPIComputationEngine $kpiEngine,
        private readonly CohortAnalyticsEngine $cohortEngine,
        private readonly FunnelAnalyticsEngine $funnelEngine,
        private readonly AttributionEngine $attributionEngine,
        private readonly ForecastingEngine $forecastingEngine,
        private readonly AnomalyDetectionEngine $anomalyEngine,
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    // =========================================================================
    // EVENT TRACKING
    // =========================================================================

    /**
     * Track an analytics event
     */
    public function trackEvent(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'event_name' => 'required|string|max:100',
            'event_category' => 'nullable|string|max:50',
            'event_type' => 'nullable|string|max:50',
            'entity_type' => 'nullable|string|max:50',
            'entity_id' => 'nullable|integer',
            'properties' => 'nullable|array',
            'event_value' => 'nullable|numeric',
            'revenue' => 'nullable|numeric',
        ]);

        $event = $this->ingestionService->track(
            $validated['event_name'],
            $validated,
            $request
        );

        return response()->json([
            'success' => true,
            'event_id' => $event->event_id,
        ]);
    }

    /**
     * Track a page view
     */
    public function trackPageView(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'page_url' => 'nullable|string|max:500',
            'page_title' => 'nullable|string|max:255',
            'page_type' => 'nullable|string|max:50',
            'referrer' => 'nullable|string|max:500',
        ]);

        $event = $this->ingestionService->trackPageView($validated, $request);

        return response()->json([
            'success' => true,
            'event_id' => $event->event_id,
        ]);
    }

    /**
     * Track batch events
     */
    public function trackBatch(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'events' => 'required|array|min:1|max:100',
            'events.*.event_name' => 'required|string|max:100',
            'events.*.event_category' => 'nullable|string|max:50',
            'events.*.properties' => 'nullable|array',
        ]);

        $processed = $this->ingestionService->ingestBatch($validated['events'], $request);

        return response()->json([
            'success' => true,
            'processed' => $processed,
        ]);
    }

    // =========================================================================
    // DASHBOARD & KPIs
    // =========================================================================

    /**
     * Get dashboard overview
     */
    public function getDashboard(Request $request): JsonResponse
    {
        $period = $request->input('period', '30d');

        $kpis = $this->kpiEngine->getDashboardKpis();
        $realtime = AnalyticsEvent::getRealTimeMetrics();
        $anomalies = $this->anomalyEngine->getAnomalySummary();

        return response()->json([
            'kpis' => $kpis,
            'realtime' => $realtime,
            'anomalies' => $anomalies,
            'period' => $period,
            'generated_at' => now()->toIso8601String(),
        ]);
    }

    /**
     * Get KPI details
     */
    public function getKpi(Request $request, string $kpiKey): JsonResponse
    {
        $period = $request->input('period', '30d');
        $granularity = $request->input('granularity', 'daily');

        [$startDate, $endDate] = $this->parsePeriod($period);

        $definition = AnalyticsKpiDefinition::where('kpi_key', $kpiKey)->firstOrFail();

        $values = $definition->getValuesInRange($granularity, $startDate, $endDate);

        $latestValue = $definition->getLatestValue($granularity);

        return response()->json([
            'kpi' => [
                'key' => $definition->kpi_key,
                'name' => $definition->name,
                'category' => $definition->category,
                'format' => $definition->format,
            ],
            'current_value' => $latestValue?->value ?? 0,
            'formatted_value' => $latestValue
                ? $definition->formatValue($latestValue->value)
                : $definition->formatValue(0),
            'change_percentage' => $latestValue?->change_percentage ?? 0,
            'trend' => $latestValue?->trend ?? 'stable',
            'time_series' => $values->map(fn ($v) => [
                'date' => $v->period_start->format('Y-m-d'),
                'value' => $v->value,
                'is_anomaly' => $v->is_anomaly,
            ]),
        ]);
    }

    /**
     * Get all KPI definitions
     */
    public function getKpiDefinitions(): JsonResponse
    {
        $definitions = AnalyticsKpiDefinition::active()
            ->orderBy('sort_order')
            ->get();

        return response()->json([
            'definitions' => $definitions->map(fn ($d) => [
                'key' => $d->kpi_key,
                'name' => $d->name,
                'description' => $d->description,
                'category' => $d->category,
                'type' => $d->type,
                'format' => $d->format,
                'icon' => $d->icon,
                'color' => $d->color,
                'is_primary' => $d->is_primary,
            ]),
        ]);
    }

    // =========================================================================
    // FUNNEL ANALYTICS
    // =========================================================================

    /**
     * Get funnel analysis
     */
    public function getFunnelAnalysis(Request $request, string $funnelKey): JsonResponse
    {
        $period = $request->input('period', '30d');
        [$startDate, $endDate] = $this->parsePeriod($period);

        $analysis = $this->funnelEngine->analyzeFunnel($funnelKey, $startDate, $endDate);

        return response()->json($analysis);
    }

    /**
     * Get funnel drop-off analysis
     */
    public function getFunnelDropOff(Request $request, string $funnelKey): JsonResponse
    {
        $period = $request->input('period', '30d');
        [$startDate, $endDate] = $this->parsePeriod($period);

        $dropOff = $this->funnelEngine->getDropOffAnalysis($funnelKey, $startDate, $endDate);

        return response()->json($dropOff);
    }

    /**
     * Get funnel by segment
     */
    public function getFunnelBySegment(Request $request, string $funnelKey): JsonResponse
    {
        $validated = $request->validate([
            'segment_field' => 'required|string',
            'period' => 'nullable|string',
        ]);

        $period = $validated['period'] ?? '30d';
        [$startDate, $endDate] = $this->parsePeriod($period);

        $segmented = $this->funnelEngine->getFunnelBySegment(
            $funnelKey,
            $validated['segment_field'],
            $startDate,
            $endDate
        );

        return response()->json($segmented);
    }

    /**
     * Get all funnels
     */
    public function getFunnels(): JsonResponse
    {
        $funnels = AnalyticsFunnel::active()
            ->with('steps')
            ->orderBy('sort_order')
            ->get();

        return response()->json([
            'funnels' => $funnels->map(fn ($f) => [
                'key' => $f->funnel_key,
                'name' => $f->name,
                'description' => $f->description,
                'type' => $f->funnel_type,
                'steps_count' => $f->steps->count(),
                'steps' => $f->steps->map(fn ($s) => [
                    'number' => $s->step_number,
                    'name' => $s->name,
                    'event' => $s->event_name,
                ]),
            ]),
        ]);
    }

    // =========================================================================
    // COHORT ANALYTICS
    // =========================================================================

    /**
     * Get cohort retention matrix
     */
    public function getCohortMatrix(Request $request, string $cohortKey): JsonResponse
    {
        $period = $request->input('period', '90d');
        [$startDate, $endDate] = $this->parsePeriod($period);

        $matrix = $this->cohortEngine->getRetentionMatrix($cohortKey, $startDate, $endDate);

        return response()->json([
            'cohort_key' => $cohortKey,
            'matrix' => $matrix,
        ]);
    }

    /**
     * Get cohort retention curve
     */
    public function getCohortCurve(Request $request, string $cohortKey): JsonResponse
    {
        $period = $request->input('period', '90d');
        [$startDate, $endDate] = $this->parsePeriod($period);

        $curve = $this->cohortEngine->getRetentionCurve($cohortKey, $startDate, $endDate);

        return response()->json([
            'cohort_key' => $cohortKey,
            'curve' => $curve,
        ]);
    }

    /**
     * Get cohort LTV analysis
     */
    public function getCohortLTV(Request $request, string $cohortKey): JsonResponse
    {
        $period = $request->input('period', '90d');
        [$startDate, $endDate] = $this->parsePeriod($period);

        $ltv = $this->cohortEngine->getLTVAnalysis($cohortKey, $startDate, $endDate);

        return response()->json($ltv);
    }

    /**
     * Get all cohort definitions
     */
    public function getCohorts(): JsonResponse
    {
        $cohorts = AnalyticsCohort::active()
            ->orderBy('sort_order')
            ->get();

        return response()->json([
            'cohorts' => $cohorts->map(fn ($c) => [
                'key' => $c->cohort_key,
                'name' => $c->name,
                'description' => $c->description,
                'type' => $c->cohort_type,
                'grouping' => $c->grouping_period,
                'metric_type' => $c->metric_type,
            ]),
        ]);
    }

    // =========================================================================
    // ATTRIBUTION ANALYTICS
    // =========================================================================

    /**
     * Get channel attribution
     */
    public function getChannelAttribution(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'model' => 'nullable|string|in:first_touch,last_touch,linear,time_decay,position_based',
            'period' => 'nullable|string',
        ]);

        $model = $validated['model'] ?? 'linear';
        $period = $validated['period'] ?? '30d';
        [$startDate, $endDate] = $this->parsePeriod($period);

        $attribution = $this->attributionEngine->getChannelAttribution($model, $startDate, $endDate);

        return response()->json($attribution);
    }

    /**
     * Get campaign attribution
     */
    public function getCampaignAttribution(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'model' => 'nullable|string|in:first_touch,last_touch,linear,time_decay,position_based',
            'period' => 'nullable|string',
        ]);

        $model = $validated['model'] ?? 'linear';
        $period = $validated['period'] ?? '30d';
        [$startDate, $endDate] = $this->parsePeriod($period);

        $attribution = $this->attributionEngine->getCampaignAttribution($model, $startDate, $endDate);

        return response()->json(['campaigns' => $attribution]);
    }

    /**
     * Get conversion paths
     */
    public function getConversionPaths(Request $request): JsonResponse
    {
        $period = $request->input('period', '30d');
        $limit = $request->input('limit', 20);
        [$startDate, $endDate] = $this->parsePeriod($period);

        $paths = $this->attributionEngine->getConversionPaths($startDate, $endDate, $limit);

        return response()->json(['paths' => $paths]);
    }

    /**
     * Compare attribution models
     */
    public function compareAttributionModels(Request $request): JsonResponse
    {
        $period = $request->input('period', '30d');
        [$startDate, $endDate] = $this->parsePeriod($period);

        $comparison = $this->attributionEngine->compareModels($startDate, $endDate);

        return response()->json(['models' => $comparison]);
    }

    // =========================================================================
    // FORECASTING
    // =========================================================================

    /**
     * Get forecast for a KPI
     */
    public function getForecast(Request $request, string $kpiKey): JsonResponse
    {
        $validated = $request->validate([
            'periods_ahead' => 'nullable|integer|min:1|max:90',
            'model' => 'nullable|string|in:linear,exponential,moving_average,seasonal',
            'granularity' => 'nullable|string|in:daily,weekly,monthly',
        ]);

        $forecast = $this->forecastingEngine->forecast(
            $kpiKey,
            $validated['granularity'] ?? 'daily',
            $validated['periods_ahead'] ?? 14,
            $validated['model'] ?? 'linear'
        );

        return response()->json($forecast);
    }

    /**
     * Get forecast accuracy report
     */
    public function getForecastAccuracy(Request $request, string $kpiKey): JsonResponse
    {
        $granularity = $request->input('granularity', 'daily');

        $accuracy = $this->forecastingEngine->getAccuracyReport($kpiKey, $granularity);

        return response()->json($accuracy);
    }

    // =========================================================================
    // SEGMENTS
    // =========================================================================

    /**
     * Get all segments
     */
    public function getSegments(): JsonResponse
    {
        $segments = AnalyticsSegment::active()
            ->orderBy('sort_order')
            ->get();

        return response()->json([
            'segments' => $segments->map(fn ($s) => [
                'key' => $s->segment_key,
                'name' => $s->name,
                'description' => $s->description,
                'type' => $s->segment_type,
                'user_count' => $s->user_count,
                'percentage' => $s->percentage_of_total,
                'color' => $s->color,
                'icon' => $s->icon,
                'is_system' => $s->is_system,
            ]),
        ]);
    }

    /**
     * Get segment details
     */
    public function getSegment(Request $request, string $segmentKey): JsonResponse
    {
        $segment = AnalyticsSegment::where('segment_key', $segmentKey)->firstOrFail();

        return response()->json([
            'segment' => [
                'key' => $segment->segment_key,
                'name' => $segment->name,
                'description' => $segment->description,
                'type' => $segment->segment_type,
                'rules' => $segment->rules,
                'user_count' => $segment->user_count,
                'percentage' => $segment->percentage_of_total,
                'last_computed' => $segment->last_computed_at?->toIso8601String(),
            ],
        ]);
    }

    // =========================================================================
    // EVENT EXPLORER
    // =========================================================================

    /**
     * Get event time series
     */
    public function getEventTimeSeries(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'event_name' => 'nullable|string',
            'granularity' => 'nullable|string|in:hour,day,week,month',
            'period' => 'nullable|string',
        ]);

        $period = $validated['period'] ?? '30d';
        $granularity = $validated['granularity'] ?? 'day';
        [$startDate, $endDate] = $this->parsePeriod($period);

        $timeSeries = AnalyticsEvent::timeSeries(
            $granularity,
            $startDate,
            $endDate,
            $validated['event_name'] ?? null
        );

        return response()->json(['time_series' => $timeSeries]);
    }

    /**
     * Get event breakdown by dimension
     */
    public function getEventBreakdown(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'dimension' => 'required|string|in:channel,device_type,country_code,browser,event_name',
            'period' => 'nullable|string',
        ]);

        $period = $validated['period'] ?? '30d';
        [$startDate, $endDate] = $this->parsePeriod($period);

        $breakdown = AnalyticsEvent::countByDimension(
            $validated['dimension'],
            $startDate,
            $endDate
        );

        return response()->json(['breakdown' => $breakdown]);
    }

    /**
     * Get real-time metrics
     */
    public function getRealTimeMetrics(): JsonResponse
    {
        $metrics = AnalyticsEvent::getRealTimeMetrics(30);
        $cacheMetrics = $this->cacheManager->getRealTimeMetrics();

        return response()->json([
            'metrics' => $metrics,
            'counters' => $cacheMetrics,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    // =========================================================================
    // HELPERS
    // =========================================================================

    /**
     * Parse period string to date range
     */
    protected function parsePeriod(string $period): array
    {
        $endDate = now();

        $startDate = match (true) {
            str_ends_with($period, 'd') => now()->subDays((int) rtrim($period, 'd')),
            str_ends_with($period, 'w') => now()->subWeeks((int) rtrim($period, 'w')),
            str_ends_with($period, 'm') => now()->subMonths((int) rtrim($period, 'm')),
            str_ends_with($period, 'y') => now()->subYears((int) rtrim($period, 'y')),
            default => now()->subDays(30),
        };

        return [$startDate->startOfDay(), $endDate->endOfDay()];
    }
}
