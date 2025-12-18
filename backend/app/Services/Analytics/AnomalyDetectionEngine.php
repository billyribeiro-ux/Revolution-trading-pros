<?php

declare(strict_types=1);

namespace App\Services\Analytics;

use App\Models\Analytics\AnalyticsEvent;
use App\Models\Analytics\AnalyticsKpiValue;
use App\Models\Analytics\AnalyticsAlert;
use App\Models\Analytics\AnalyticsAlertHistory;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * AnomalyDetectionEngine - Intelligent Anomaly Detection
 *
 * Detects anomalies in metrics using statistical methods including
 * z-score analysis, trend detection, and pattern recognition.
 *
 * @package App\Services\Analytics
 */
class AnomalyDetectionEngine
{
    /**
     * Default z-score threshold for anomaly detection
     */
    private const DEFAULT_Z_THRESHOLD = 2.5;

    /**
     * Minimum data points required for analysis
     */
    private const MIN_DATA_POINTS = 7;

    public function __construct(
        private readonly AnalyticsCacheManager $cacheManager,
    ) {}

    /**
     * Detect anomalies in a metric time series
     */
    public function detectInTimeSeries(
        Collection $values,
        float $threshold = self::DEFAULT_Z_THRESHOLD
    ): array {
        if ($values->count() < self::MIN_DATA_POINTS) {
            return [];
        }

        $anomalies = [];
        $mean = $values->avg();
        $stdDev = $this->calculateStdDev($values);

        if ($stdDev == 0) {
            return [];
        }

        foreach ($values as $index => $value) {
            $zScore = abs(($value - $mean) / $stdDev);

            if ($zScore > $threshold) {
                $anomalies[] = [
                    'index' => $index,
                    'value' => $value,
                    'z_score' => round($zScore, 2),
                    'type' => $value > $mean ? 'spike' : 'drop',
                    'deviation_percentage' => round((($value - $mean) / $mean) * 100, 2),
                ];
            }
        }

        return $anomalies;
    }

    /**
     * Detect anomaly in latest KPI value
     */
    public function detectKpiAnomaly(
        int $kpiDefinitionId,
        string $periodType,
        Carbon $currentDate
    ): ?array {
        // Get historical values
        $historical = AnalyticsKpiValue::byKpi($kpiDefinitionId)
            ->byPeriodType($periodType)
            ->noSegment()
            ->where('period_start', '<', $currentDate)
            ->orderByDesc('period_start')
            ->limit(30)
            ->pluck('value');

        if ($historical->count() < self::MIN_DATA_POINTS) {
            return null;
        }

        // Get current value
        $current = AnalyticsKpiValue::byKpi($kpiDefinitionId)
            ->byPeriodType($periodType)
            ->noSegment()
            ->where('period_start', $currentDate)
            ->first();

        if (!$current) {
            return null;
        }

        $mean = $historical->avg();
        $stdDev = $this->calculateStdDev($historical);

        if ($stdDev == 0) {
            return null;
        }

        $zScore = abs(($current->value - $mean) / $stdDev);

        if ($zScore > self::DEFAULT_Z_THRESHOLD) {
            return [
                'kpi_definition_id' => $kpiDefinitionId,
                'value' => $current->value,
                'mean' => round($mean, 2),
                'std_dev' => round($stdDev, 2),
                'z_score' => round($zScore, 2),
                'type' => $current->value > $mean ? 'spike' : 'drop',
                'severity' => $this->getSeverity($zScore),
                'deviation_percentage' => round((($current->value - $mean) / $mean) * 100, 2),
            ];
        }

        return null;
    }

    /**
     * Detect trend breaks in time series
     */
    public function detectTrendBreak(Collection $values, int $windowSize = 7): ?array
    {
        if ($values->count() < ($windowSize * 2)) {
            return null;
        }

        $valuesArray = $values->values()->toArray();
        $n = count($valuesArray);

        // Calculate rolling averages
        $recentAvg = array_sum(array_slice($valuesArray, -$windowSize)) / $windowSize;
        $previousAvg = array_sum(array_slice($valuesArray, -($windowSize * 2), $windowSize)) / $windowSize;

        // Calculate trend change
        $changePercentage = $previousAvg > 0
            ? (($recentAvg - $previousAvg) / $previousAvg) * 100
            : 0;

        // Detect significant trend change (>20% change)
        if (abs($changePercentage) > 20) {
            return [
                'type' => 'trend_break',
                'direction' => $changePercentage > 0 ? 'up' : 'down',
                'change_percentage' => round($changePercentage, 2),
                'previous_avg' => round($previousAvg, 2),
                'recent_avg' => round($recentAvg, 2),
                'severity' => abs($changePercentage) > 50 ? 'critical' : 'warning',
            ];
        }

        return null;
    }

    /**
     * Detect sudden spikes or drops
     */
    public function detectSuddenChange(
        float $currentValue,
        float $previousValue,
        float $threshold = 50
    ): ?array {
        if ($previousValue == 0) {
            return $currentValue > 0 ? [
                'type' => 'new_activity',
                'current' => $currentValue,
                'previous' => 0,
            ] : null;
        }

        $changePercentage = (($currentValue - $previousValue) / $previousValue) * 100;

        if (abs($changePercentage) > $threshold) {
            return [
                'type' => $changePercentage > 0 ? 'spike' : 'drop',
                'current' => $currentValue,
                'previous' => $previousValue,
                'change_percentage' => round($changePercentage, 2),
                'severity' => abs($changePercentage) > 100 ? 'critical' : 'warning',
            ];
        }

        return null;
    }

    /**
     * Run anomaly detection on all active alerts
     */
    public function runAlertChecks(): int
    {
        $alerts = AnalyticsAlert::where('is_active', true)->get();
        $triggered = 0;

        foreach ($alerts as $alert) {
            try {
                if ($this->checkAlert($alert)) {
                    $triggered++;
                }
            } catch (\Exception $e) {
                Log::error('Alert check failed', [
                    'alert_id' => $alert->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return $triggered;
    }

    /**
     * Check a single alert condition
     */
    public function checkAlert(AnalyticsAlert $alert): bool
    {
        // Get current metric value
        $currentValue = $this->getAlertMetricValue($alert);

        if ($currentValue === null) {
            return false;
        }

        $triggered = false;
        $severity = 'info';

        // Check based on alert type
        switch ($alert->alert_type) {
            case 'threshold':
                $triggered = $this->checkThresholdCondition(
                    $currentValue,
                    $alert->operator,
                    $alert->threshold_value
                );
                $severity = $triggered ? 'warning' : 'info';
                break;

            case 'anomaly':
                $anomaly = $this->detectKpiAnomalyByKey($alert->metric_key);
                $triggered = $anomaly !== null;
                $severity = $anomaly['severity'] ?? 'warning';
                break;

            case 'change':
                $previousValue = $this->getPreviousMetricValue($alert);
                if ($previousValue !== null) {
                    $changePercentage = $previousValue > 0
                        ? (($currentValue - $previousValue) / $previousValue) * 100
                        : 0;

                    $triggered = abs($changePercentage) > ($alert->threshold_percentage ?? 20);
                    $severity = $triggered && abs($changePercentage) > 50 ? 'critical' : 'warning';
                }
                break;
        }

        if ($triggered) {
            $this->triggerAlert($alert, $currentValue, $severity);
            return true;
        }

        // Update status to normal if was previously triggered
        if ($alert->current_status !== 'normal') {
            $alert->update(['current_status' => 'normal']);
        }

        return false;
    }

    /**
     * Check threshold condition
     */
    protected function checkThresholdCondition(
        float $value,
        string $operator,
        ?float $threshold
    ): bool {
        if ($threshold === null) {
            return false;
        }

        return match ($operator) {
            'gt' => $value > $threshold,
            'lt' => $value < $threshold,
            'gte' => $value >= $threshold,
            'lte' => $value <= $threshold,
            'eq' => $value == $threshold,
            default => false,
        };
    }

    /**
     * Get metric value for alert
     */
    protected function getAlertMetricValue(AnalyticsAlert $alert): ?float
    {
        // Try to get from KPI values first
        $kpiValue = AnalyticsKpiValue::byKpiKey($alert->metric_key)
            ->byPeriodType('daily')
            ->noSegment()
            ->orderByDesc('period_start')
            ->first();

        if ($kpiValue) {
            return $kpiValue->value;
        }

        // Fall back to event count
        return (float) AnalyticsEvent::today()
            ->byEventName($alert->metric_key)
            ->count();
    }

    /**
     * Get previous metric value for comparison
     */
    protected function getPreviousMetricValue(AnalyticsAlert $alert): ?float
    {
        $kpiValue = AnalyticsKpiValue::byKpiKey($alert->metric_key)
            ->byPeriodType('daily')
            ->noSegment()
            ->where('period_start', '<', now()->startOfDay())
            ->orderByDesc('period_start')
            ->first();

        return $kpiValue?->value;
    }

    /**
     * Detect KPI anomaly by key
     */
    protected function detectKpiAnomalyByKey(string $metricKey): ?array
    {
        $kpiDefinition = \App\Models\Analytics\AnalyticsKpiDefinition::where('kpi_key', $metricKey)->first();

        if (!$kpiDefinition) {
            return null;
        }

        return $this->detectKpiAnomaly(
            $kpiDefinition->id,
            'daily',
            now()->startOfDay()
        );
    }

    /**
     * Trigger alert
     */
    protected function triggerAlert(
        AnalyticsAlert $alert,
        float $currentValue,
        string $severity
    ): void {
        // Create alert history entry
        AnalyticsAlertHistory::create([
            'alert_id' => $alert->id,
            'triggered_at' => now(),
            'severity' => $severity,
            'current_value' => $currentValue,
            'threshold_value' => $alert->threshold_value ?? 0,
            'previous_value' => $this->getPreviousMetricValue($alert),
            'message' => $this->generateAlertMessage($alert, $currentValue, $severity),
            'context_data' => [
                'metric_key' => $alert->metric_key,
                'alert_type' => $alert->alert_type,
            ],
        ]);

        // Update alert status
        $alert->update([
            'current_status' => $severity,
            'last_triggered_at' => now(),
        ]);

        // Send notifications (could dispatch job here)
        $this->sendAlertNotifications($alert, $currentValue, $severity);
    }

    /**
     * Generate alert message
     */
    protected function generateAlertMessage(
        AnalyticsAlert $alert,
        float $currentValue,
        string $severity
    ): string {
        return sprintf(
            '%s alert for %s: Current value %.2f %s threshold %.2f',
            ucfirst($severity),
            $alert->name,
            $currentValue,
            $alert->operator,
            $alert->threshold_value ?? 0
        );
    }

    /**
     * Send alert notifications
     */
    protected function sendAlertNotifications(
        AnalyticsAlert $alert,
        float $currentValue,
        string $severity
    ): void {
        // Implementation would dispatch notification jobs
        // for email, Slack, webhooks, etc.
        Log::info('Alert triggered', [
            'alert' => $alert->name,
            'value' => $currentValue,
            'severity' => $severity,
        ]);
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
     * Get severity level from z-score
     */
    protected function getSeverity(float $zScore): string
    {
        return match (true) {
            $zScore >= 4 => 'critical',
            $zScore >= 3 => 'high',
            $zScore >= 2.5 => 'warning',
            default => 'info',
        };
    }

    /**
     * Get anomaly summary for dashboard
     */
    public function getAnomalySummary(): array
    {
        $recentAnomalies = AnalyticsKpiValue::anomalies()
            ->where('created_at', '>=', now()->subDay())
            ->with('kpiDefinition')
            ->get();

        $recentAlerts = AnalyticsAlertHistory::query()
            ->where('triggered_at', '>=', now()->subDay())
            ->with('alert')
            ->orderByDesc('triggered_at')
            ->limit(10)
            ->get();

        return [
            'anomalies_count' => $recentAnomalies->count(),
            'alerts_triggered' => $recentAlerts->count(),
            'critical_count' => $recentAlerts->where('severity', 'critical')->count(),
            'recent_anomalies' => $recentAnomalies->map(fn ($a) => [
                'kpi' => $a->kpiDefinition->name ?? 'Unknown',
                'value' => $a->value,
                'type' => $a->anomaly_type,
                'score' => $a->anomaly_score,
                'detected_at' => $a->created_at->format('Y-m-d H:i'),
            ])->toArray(),
            'recent_alerts' => $recentAlerts->map(fn ($h) => [
                'alert' => $h->alert->name ?? 'Unknown',
                'severity' => $h->severity,
                'message' => $h->message,
                'triggered_at' => $h->triggered_at->format('Y-m-d H:i'),
            ])->toArray(),
        ];
    }
}
