<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;

/**
 * AnalyticsCohort Model - Cohort Analysis Configuration
 *
 * Defines cohort analysis parameters for user retention and behavior tracking.
 *
 * @property int $id
 * @property string $cohort_key
 * @property string $name
 * @property string $cohort_type
 * @property string $grouping_period
 * @property string $entry_event
 */
class AnalyticsCohort extends Model
{
    protected $table = 'analytics_cohorts';

    /**
     * Cohort Types
     */
    public const TYPE_ACQUISITION = 'acquisition';
    public const TYPE_BEHAVIORAL = 'behavioral';
    public const TYPE_VALUE = 'value';
    public const TYPE_CUSTOM = 'custom';

    /**
     * Grouping Periods
     */
    public const GROUP_DAILY = 'daily';
    public const GROUP_WEEKLY = 'weekly';
    public const GROUP_MONTHLY = 'monthly';

    /**
     * Metric Types
     */
    public const METRIC_RETENTION = 'retention';
    public const METRIC_REVENUE = 'revenue';
    public const METRIC_ENGAGEMENT = 'engagement';

    protected $fillable = [
        'cohort_key',
        'name',
        'description',
        'cohort_type',
        'grouping_period',
        'entry_event',
        'entry_conditions',
        'retention_event',
        'retention_conditions',
        'analysis_periods',
        'metric_type',
        'is_active',
        'sort_order',
        'chart_config',
    ];

    protected $casts = [
        'entry_conditions' => 'array',
        'retention_conditions' => 'array',
        'chart_config' => 'array',
        'analysis_periods' => 'integer',
        'is_active' => 'boolean',
        'sort_order' => 'integer',
    ];

    protected $attributes = [
        'grouping_period' => self::GROUP_WEEKLY,
        'analysis_periods' => 12,
        'metric_type' => self::METRIC_RETENTION,
        'is_active' => true,
        'sort_order' => 0,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function results(): HasMany
    {
        return $this->hasMany(AnalyticsCohortResult::class, 'cohort_id');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    public function scopeByType(Builder $query, string $type): Builder
    {
        return $query->where('cohort_type', $type);
    }

    // =========================================================================
    // STATIC METHODS - DEFAULT COHORTS
    // =========================================================================

    /**
     * Get predefined cohort configurations
     */
    public static function getDefaultCohorts(): array
    {
        return [
            [
                'cohort_key' => 'user_signup',
                'name' => 'User Signup Cohort',
                'description' => 'Users grouped by signup date',
                'cohort_type' => self::TYPE_ACQUISITION,
                'grouping_period' => self::GROUP_WEEKLY,
                'entry_event' => 'signup',
                'retention_event' => 'page_view',
                'analysis_periods' => 12,
                'metric_type' => self::METRIC_RETENTION,
            ],
            [
                'cohort_key' => 'first_purchase',
                'name' => 'First Purchase Cohort',
                'description' => 'Users grouped by first purchase date',
                'cohort_type' => self::TYPE_VALUE,
                'grouping_period' => self::GROUP_WEEKLY,
                'entry_event' => 'purchase',
                'retention_event' => 'purchase',
                'analysis_periods' => 12,
                'metric_type' => self::METRIC_REVENUE,
            ],
            [
                'cohort_key' => 'subscription_start',
                'name' => 'Subscription Cohort',
                'description' => 'Users grouped by subscription start date',
                'cohort_type' => self::TYPE_VALUE,
                'grouping_period' => self::GROUP_MONTHLY,
                'entry_event' => 'subscription_created',
                'retention_event' => 'subscription_renewed',
                'analysis_periods' => 12,
                'metric_type' => self::METRIC_RETENTION,
            ],
            [
                'cohort_key' => 'course_enrollment',
                'name' => 'Course Enrollment Cohort',
                'description' => 'Users grouped by course enrollment date',
                'cohort_type' => self::TYPE_BEHAVIORAL,
                'grouping_period' => self::GROUP_WEEKLY,
                'entry_event' => 'course_enrolled',
                'retention_event' => 'lesson_completed',
                'analysis_periods' => 8,
                'metric_type' => self::METRIC_ENGAGEMENT,
            ],
        ];
    }

    /**
     * Seed default cohorts
     */
    public static function seedDefaults(): void
    {
        foreach (self::getDefaultCohorts() as $cohort) {
            self::firstOrCreate(
                ['cohort_key' => $cohort['cohort_key']],
                $cohort
            );
        }
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Get cohort analysis matrix
     */
    public function getRetentionMatrix(Carbon $startDate, Carbon $endDate): array
    {
        $results = $this->results()
            ->where('cohort_date', '>=', $startDate)
            ->where('cohort_date', '<=', $endDate)
            ->orderBy('cohort_date')
            ->orderBy('period_number')
            ->get();

        $matrix = [];

        foreach ($results as $result) {
            $cohortKey = $result->cohort_period;

            if (!isset($matrix[$cohortKey])) {
                $matrix[$cohortKey] = [
                    'cohort_date' => $result->cohort_date->format('Y-m-d'),
                    'cohort_size' => $result->cohort_size,
                    'periods' => [],
                ];
            }

            $matrix[$cohortKey]['periods'][$result->period_number] = [
                'active_users' => $result->active_users,
                'retention_rate' => $result->retention_rate,
                'total_revenue' => $result->total_revenue,
                'avg_revenue_per_user' => $result->avg_revenue_per_user,
            ];
        }

        return array_values($matrix);
    }

    /**
     * Get average retention by period
     */
    public function getAverageRetentionByPeriod(Carbon $startDate, Carbon $endDate): Collection
    {
        return $this->results()
            ->where('cohort_date', '>=', $startDate)
            ->where('cohort_date', '<=', $endDate)
            ->selectRaw('period_number, AVG(retention_rate) as avg_retention, AVG(total_revenue) as avg_revenue')
            ->groupBy('period_number')
            ->orderBy('period_number')
            ->get();
    }

    /**
     * Get cohort size trend
     */
    public function getCohortSizeTrend(Carbon $startDate, Carbon $endDate): Collection
    {
        return $this->results()
            ->where('cohort_date', '>=', $startDate)
            ->where('cohort_date', '<=', $endDate)
            ->where('period_number', 0)
            ->select('cohort_date', 'cohort_period', 'cohort_size')
            ->orderBy('cohort_date')
            ->get();
    }

    /**
     * Generate period key based on grouping
     */
    public function generatePeriodKey(Carbon $date): string
    {
        return match ($this->grouping_period) {
            self::GROUP_DAILY => $date->format('Y-m-d'),
            self::GROUP_WEEKLY => $date->format('Y') . '-W' . $date->format('W'),
            self::GROUP_MONTHLY => $date->format('Y-m'),
            default => $date->format('Y-m-d'),
        };
    }

    /**
     * Get period start date from key
     */
    public function getPeriodStartDate(string $periodKey): Carbon
    {
        return match ($this->grouping_period) {
            self::GROUP_DAILY => Carbon::parse($periodKey),
            self::GROUP_WEEKLY => Carbon::now()->setISODate(
                (int) substr($periodKey, 0, 4),
                (int) substr($periodKey, 6)
            )->startOfWeek(),
            self::GROUP_MONTHLY => Carbon::parse($periodKey . '-01'),
            default => Carbon::parse($periodKey),
        };
    }
}
