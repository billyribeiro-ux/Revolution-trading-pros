<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;

/**
 * AnalyticsKpiValue Model - Computed KPI Values Storage
 *
 * Stores pre-computed KPI values for efficient dashboard queries.
 *
 * @property int $id
 * @property int $kpi_definition_id
 * @property string $period_type
 * @property Carbon $period_start
 * @property Carbon $period_end
 * @property float $value
 * @property float|null $previous_value
 * @property bool $is_anomaly
 */
class AnalyticsKpiValue extends Model
{
    protected $table = 'analytics_kpi_values';

    /**
     * Period Types
     */
    public const PERIOD_HOURLY = 'hourly';
    public const PERIOD_DAILY = 'daily';
    public const PERIOD_WEEKLY = 'weekly';
    public const PERIOD_MONTHLY = 'monthly';
    public const PERIOD_YEARLY = 'yearly';

    /**
     * Trend Directions
     */
    public const TREND_UP = 'up';
    public const TREND_DOWN = 'down';
    public const TREND_STABLE = 'stable';

    /**
     * Target Status
     */
    public const STATUS_ON_TRACK = 'on_track';
    public const STATUS_AT_RISK = 'at_risk';
    public const STATUS_BEHIND = 'behind';

    protected $fillable = [
        'kpi_definition_id',
        'period_type',
        'period_start',
        'period_end',
        'period_key',
        'segment_type',
        'segment_value',
        'value',
        'previous_value',
        'change_value',
        'change_percentage',
        'trend',
        'min_value',
        'max_value',
        'avg_value',
        'std_deviation',
        'sample_size',
        'target_value',
        'target_variance',
        'target_status',
        'is_anomaly',
        'anomaly_score',
        'anomaly_type',
    ];

    protected $casts = [
        'kpi_definition_id' => 'integer',
        'period_start' => 'date',
        'period_end' => 'date',
        'period_key' => 'integer',
        'value' => 'decimal:4',
        'previous_value' => 'decimal:4',
        'change_value' => 'decimal:4',
        'change_percentage' => 'decimal:4',
        'min_value' => 'decimal:4',
        'max_value' => 'decimal:4',
        'avg_value' => 'decimal:4',
        'std_deviation' => 'decimal:4',
        'sample_size' => 'integer',
        'target_value' => 'decimal:4',
        'target_variance' => 'decimal:4',
        'anomaly_score' => 'decimal:2',
        'is_anomaly' => 'boolean',
    ];

    protected $attributes = [
        'is_anomaly' => false,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function kpiDefinition(): BelongsTo
    {
        return $this->belongsTo(AnalyticsKpiDefinition::class, 'kpi_definition_id');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeByPeriodType(Builder $query, string $periodType): Builder
    {
        return $query->where('period_type', $periodType);
    }

    public function scopeByKpi(Builder $query, int $kpiDefinitionId): Builder
    {
        return $query->where('kpi_definition_id', $kpiDefinitionId);
    }

    public function scopeByKpiKey(Builder $query, string $kpiKey): Builder
    {
        return $query->whereHas('kpiDefinition', function ($q) use ($kpiKey) {
            $q->where('kpi_key', $kpiKey);
        });
    }

    public function scopeBySegment(Builder $query, string $type, ?string $value = null): Builder
    {
        $query->where('segment_type', $type);

        if ($value !== null) {
            $query->where('segment_value', $value);
        }

        return $query;
    }

    public function scopeNoSegment(Builder $query): Builder
    {
        return $query->whereNull('segment_type');
    }

    public function scopeInDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->where('period_start', '>=', $start)
            ->where('period_end', '<=', $end);
    }

    public function scopeAnomalies(Builder $query): Builder
    {
        return $query->where('is_anomaly', true);
    }

    public function scopeWithTrend(Builder $query, string $trend): Builder
    {
        return $query->where('trend', $trend);
    }

    public function scopeLatest(Builder $query): Builder
    {
        return $query->orderByDesc('period_start');
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    /**
     * Get time-series data for a KPI
     */
    public static function getTimeSeries(
        int $kpiDefinitionId,
        string $periodType,
        Carbon $startDate,
        Carbon $endDate,
        ?string $segmentType = null,
        ?string $segmentValue = null
    ): Collection {
        $query = static::byKpi($kpiDefinitionId)
            ->byPeriodType($periodType)
            ->inDateRange($startDate, $endDate)
            ->orderBy('period_start');

        if ($segmentType) {
            $query->bySegment($segmentType, $segmentValue);
        } else {
            $query->noSegment();
        }

        return $query->get();
    }

    /**
     * Get comparison between two periods
     */
    public static function comparePeriods(
        int $kpiDefinitionId,
        string $periodType,
        Carbon $currentStart,
        Carbon $currentEnd,
        Carbon $previousStart,
        Carbon $previousEnd
    ): array {
        $current = static::byKpi($kpiDefinitionId)
            ->byPeriodType($periodType)
            ->noSegment()
            ->where('period_start', '>=', $currentStart)
            ->where('period_end', '<=', $currentEnd)
            ->sum('value');

        $previous = static::byKpi($kpiDefinitionId)
            ->byPeriodType($periodType)
            ->noSegment()
            ->where('period_start', '>=', $previousStart)
            ->where('period_end', '<=', $previousEnd)
            ->sum('value');

        $change = $current - $previous;
        $changePercentage = $previous > 0 ? (($change / $previous) * 100) : 0;

        return [
            'current_value' => $current,
            'previous_value' => $previous,
            'change' => $change,
            'change_percentage' => round($changePercentage, 2),
            'trend' => $change > 0 ? self::TREND_UP : ($change < 0 ? self::TREND_DOWN : self::TREND_STABLE),
        ];
    }

    /**
     * Get segmented breakdown
     */
    public static function getSegmentedBreakdown(
        int $kpiDefinitionId,
        string $periodType,
        Carbon $periodStart,
        string $segmentType
    ): Collection {
        return static::byKpi($kpiDefinitionId)
            ->byPeriodType($periodType)
            ->where('period_start', $periodStart)
            ->where('segment_type', $segmentType)
            ->orderByDesc('value')
            ->get();
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Calculate change from previous value
     */
    public function calculateChange(): void
    {
        if ($this->previous_value !== null) {
            $this->change_value = $this->value - $this->previous_value;

            if ($this->previous_value > 0) {
                $this->change_percentage = round(
                    ($this->change_value / $this->previous_value) * 100,
                    4
                );
            }

            $this->trend = match (true) {
                $this->change_percentage > 1 => self::TREND_UP,
                $this->change_percentage < -1 => self::TREND_DOWN,
                default => self::TREND_STABLE,
            };
        }
    }

    /**
     * Check if value is an anomaly using z-score
     */
    public function detectAnomaly(float $mean, float $stdDev, float $threshold = 2.5): bool
    {
        if ($stdDev == 0) {
            return false;
        }

        $zScore = abs(($this->value - $mean) / $stdDev);
        $this->anomaly_score = round($zScore, 2);
        $this->is_anomaly = $zScore > $threshold;

        if ($this->is_anomaly) {
            $this->anomaly_type = $this->value > $mean ? 'spike' : 'drop';
        }

        return $this->is_anomaly;
    }

    /**
     * Evaluate target status
     */
    public function evaluateTargetStatus(): void
    {
        if ($this->target_value === null) {
            return;
        }

        $variance = (($this->value - $this->target_value) / $this->target_value) * 100;
        $this->target_variance = round($variance, 4);

        $this->target_status = match (true) {
            $variance >= 0 => self::STATUS_ON_TRACK,
            $variance >= -10 => self::STATUS_AT_RISK,
            default => self::STATUS_BEHIND,
        };
    }

    /**
     * Get formatted display data
     */
    public function getDisplayData(): array
    {
        $definition = $this->kpiDefinition;

        return [
            'kpi_key' => $definition->kpi_key,
            'name' => $definition->name,
            'value' => $this->value,
            'formatted_value' => $definition->formatValue($this->value),
            'change' => $this->change_value,
            'change_percentage' => $this->change_percentage,
            'trend' => $this->trend,
            'target_status' => $this->target_status,
            'is_anomaly' => $this->is_anomaly,
            'period' => [
                'type' => $this->period_type,
                'start' => $this->period_start->format('Y-m-d'),
                'end' => $this->period_end->format('Y-m-d'),
            ],
        ];
    }
}
