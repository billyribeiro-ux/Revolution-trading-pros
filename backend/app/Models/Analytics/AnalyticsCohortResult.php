<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * AnalyticsCohortResult Model - Pre-computed Cohort Analysis Results
 *
 * Stores cohort retention and metric data for efficient queries.
 *
 * @property int $id
 * @property int $cohort_id
 * @property Carbon $cohort_date
 * @property string $cohort_period
 * @property int $period_number
 * @property int $cohort_size
 * @property int $active_users
 * @property float $retention_rate
 */
class AnalyticsCohortResult extends Model
{
    protected $table = 'analytics_cohort_results';

    protected $fillable = [
        'cohort_id',
        'cohort_date',
        'cohort_period',
        'period_number',
        'analysis_date',
        'cohort_size',
        'active_users',
        'retention_rate',
        'cumulative_retention',
        'total_revenue',
        'avg_revenue_per_user',
        'ltv_to_date',
        'total_events',
        'avg_events_per_user',
        'engagement_score',
    ];

    protected $casts = [
        'cohort_id' => 'integer',
        'cohort_date' => 'date',
        'analysis_date' => 'date',
        'period_number' => 'integer',
        'cohort_size' => 'integer',
        'active_users' => 'integer',
        'retention_rate' => 'decimal:4',
        'cumulative_retention' => 'decimal:4',
        'total_revenue' => 'decimal:4',
        'avg_revenue_per_user' => 'decimal:4',
        'ltv_to_date' => 'decimal:4',
        'total_events' => 'integer',
        'avg_events_per_user' => 'decimal:2',
        'engagement_score' => 'decimal:2',
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function cohort(): BelongsTo
    {
        return $this->belongsTo(AnalyticsCohort::class, 'cohort_id');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeByCohort(Builder $query, int $cohortId): Builder
    {
        return $query->where('cohort_id', $cohortId);
    }

    public function scopeByPeriod(Builder $query, int $periodNumber): Builder
    {
        return $query->where('period_number', $periodNumber);
    }

    public function scopeInDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('cohort_date', [$start, $end]);
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    /**
     * Calculate retention rate
     */
    public static function calculateRetentionRate(int $activeUsers, int $cohortSize): float
    {
        if ($cohortSize === 0) {
            return 0;
        }

        return round(($activeUsers / $cohortSize) * 100, 4);
    }

    /**
     * Get retention curve data
     */
    public static function getRetentionCurve(int $cohortId, Carbon $startDate, Carbon $endDate): array
    {
        $results = static::byCohort($cohortId)
            ->inDateRange($startDate, $endDate)
            ->selectRaw('period_number, AVG(retention_rate) as avg_retention, COUNT(*) as cohort_count')
            ->groupBy('period_number')
            ->orderBy('period_number')
            ->get();

        return $results->map(fn($row) => [
            'period' => $row->period_number,
            'retention' => round($row->avg_retention, 2),
            'cohorts' => $row->cohort_count,
        ])->toArray();
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Calculate derived metrics
     */
    public function calculateDerivedMetrics(): void
    {
        // Calculate retention rate
        $this->retention_rate = self::calculateRetentionRate(
            $this->active_users,
            $this->cohort_size
        );

        // Calculate average revenue per user
        if ($this->active_users > 0) {
            $this->avg_revenue_per_user = round($this->total_revenue / $this->active_users, 4);
        }

        // Calculate average events per user
        if ($this->active_users > 0) {
            $this->avg_events_per_user = round($this->total_events / $this->active_users, 2);
        }
    }

    /**
     * Get churn rate for this period
     */
    public function getChurnRate(): float
    {
        return 100 - $this->retention_rate;
    }

    /**
     * Compare to previous period
     */
    public function compareToPrevoiusPeriod(): ?array
    {
        if ($this->period_number === 0) {
            return null;
        }

        $previous = static::byCohort($this->cohort_id)
            ->where('cohort_date', $this->cohort_date)
            ->byPeriod($this->period_number - 1)
            ->first();

        if (!$previous) {
            return null;
        }

        return [
            'retention_change' => $this->retention_rate - $previous->retention_rate,
            'users_lost' => $previous->active_users - $this->active_users,
            'revenue_change' => $this->total_revenue - $previous->total_revenue,
        ];
    }
}
