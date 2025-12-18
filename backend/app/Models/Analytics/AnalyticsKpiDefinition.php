<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Collection;

/**
 * AnalyticsKpiDefinition Model - KPI Configuration
 *
 * Defines KPI metrics with calculation rules, targets, and display settings.
 *
 * @property int $id
 * @property string $kpi_key
 * @property string $name
 * @property string|null $description
 * @property string $category
 * @property string $type
 * @property string $aggregation
 * @property string $format
 */
class AnalyticsKpiDefinition extends Model
{
    protected $table = 'analytics_kpi_definitions';

    /**
     * KPI Categories
     */
    public const CATEGORY_REVENUE = 'revenue';
    public const CATEGORY_ENGAGEMENT = 'engagement';
    public const CATEGORY_CONVERSION = 'conversion';
    public const CATEGORY_RETENTION = 'retention';
    public const CATEGORY_GROWTH = 'growth';
    public const CATEGORY_ACQUISITION = 'acquisition';
    public const CATEGORY_CONTENT = 'content';

    /**
     * KPI Types
     */
    public const TYPE_COUNT = 'count';
    public const TYPE_SUM = 'sum';
    public const TYPE_AVERAGE = 'average';
    public const TYPE_RATIO = 'ratio';
    public const TYPE_PERCENTAGE = 'percentage';
    public const TYPE_RATE = 'rate';

    /**
     * Aggregation Methods
     */
    public const AGG_SUM = 'sum';
    public const AGG_AVG = 'avg';
    public const AGG_MIN = 'min';
    public const AGG_MAX = 'max';
    public const AGG_COUNT = 'count';
    public const AGG_DISTINCT = 'distinct';

    /**
     * Display Formats
     */
    public const FORMAT_NUMBER = 'number';
    public const FORMAT_CURRENCY = 'currency';
    public const FORMAT_PERCENTAGE = 'percentage';
    public const FORMAT_DURATION = 'duration';
    public const FORMAT_DECIMAL = 'decimal';

    protected $fillable = [
        'kpi_key',
        'name',
        'description',
        'category',
        'type',
        'aggregation',
        'metric_field',
        'numerator_event',
        'denominator_event',
        'filters',
        'formula',
        'format',
        'decimal_places',
        'unit',
        'icon',
        'color',
        'target_value',
        'warning_threshold',
        'critical_threshold',
        'comparison_period',
        'sort_order',
        'is_primary',
        'is_active',
        'dashboard_section',
        'default_period',
        'available_periods',
    ];

    protected $casts = [
        'filters' => 'array',
        'formula' => 'array',
        'available_periods' => 'array',
        'target_value' => 'decimal:4',
        'warning_threshold' => 'decimal:4',
        'critical_threshold' => 'decimal:4',
        'decimal_places' => 'integer',
        'sort_order' => 'integer',
        'is_primary' => 'boolean',
        'is_active' => 'boolean',
    ];

    protected $attributes = [
        'aggregation' => self::AGG_SUM,
        'format' => self::FORMAT_NUMBER,
        'decimal_places' => 2,
        'comparison_period' => 'previous_period',
        'sort_order' => 0,
        'is_primary' => false,
        'is_active' => true,
        'default_period' => '30d',
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function values(): HasMany
    {
        return $this->hasMany(AnalyticsKpiValue::class, 'kpi_definition_id');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    public function scopePrimary(Builder $query): Builder
    {
        return $query->where('is_primary', true);
    }

    public function scopeByCategory(Builder $query, string $category): Builder
    {
        return $query->where('category', $category);
    }

    public function scopeForDashboard(Builder $query, ?string $section = null): Builder
    {
        $query->active()->orderBy('sort_order');

        if ($section) {
            $query->where('dashboard_section', $section);
        }

        return $query;
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    /**
     * Get all predefined KPI definitions
     */
    public static function getDefaultKpis(): array
    {
        return [
            // Revenue KPIs
            [
                'kpi_key' => 'total_revenue',
                'name' => 'Total Revenue',
                'description' => 'Total revenue from all transactions',
                'category' => self::CATEGORY_REVENUE,
                'type' => self::TYPE_SUM,
                'aggregation' => self::AGG_SUM,
                'metric_field' => 'revenue',
                'format' => self::FORMAT_CURRENCY,
                'icon' => 'currency-dollar',
                'color' => 'green',
                'is_primary' => true,
                'dashboard_section' => 'overview',
            ],
            [
                'kpi_key' => 'avg_order_value',
                'name' => 'Average Order Value',
                'description' => 'Average revenue per transaction',
                'category' => self::CATEGORY_REVENUE,
                'type' => self::TYPE_AVERAGE,
                'aggregation' => self::AGG_AVG,
                'metric_field' => 'revenue',
                'filters' => ['event_type' => 'purchase'],
                'format' => self::FORMAT_CURRENCY,
                'icon' => 'shopping-cart',
                'color' => 'blue',
                'dashboard_section' => 'revenue',
            ],
            [
                'kpi_key' => 'revenue_per_user',
                'name' => 'Revenue Per User',
                'description' => 'Average revenue per unique user',
                'category' => self::CATEGORY_REVENUE,
                'type' => self::TYPE_RATIO,
                'numerator_event' => 'total_revenue',
                'denominator_event' => 'unique_users',
                'format' => self::FORMAT_CURRENCY,
                'icon' => 'user-dollar',
                'color' => 'purple',
                'dashboard_section' => 'revenue',
            ],

            // Engagement KPIs
            [
                'kpi_key' => 'total_sessions',
                'name' => 'Total Sessions',
                'description' => 'Total number of user sessions',
                'category' => self::CATEGORY_ENGAGEMENT,
                'type' => self::TYPE_COUNT,
                'aggregation' => self::AGG_COUNT,
                'format' => self::FORMAT_NUMBER,
                'icon' => 'users',
                'color' => 'blue',
                'is_primary' => true,
                'dashboard_section' => 'overview',
            ],
            [
                'kpi_key' => 'avg_session_duration',
                'name' => 'Avg Session Duration',
                'description' => 'Average time spent per session',
                'category' => self::CATEGORY_ENGAGEMENT,
                'type' => self::TYPE_AVERAGE,
                'aggregation' => self::AGG_AVG,
                'metric_field' => 'duration_seconds',
                'format' => self::FORMAT_DURATION,
                'icon' => 'clock',
                'color' => 'orange',
                'dashboard_section' => 'engagement',
            ],
            [
                'kpi_key' => 'pages_per_session',
                'name' => 'Pages Per Session',
                'description' => 'Average pages viewed per session',
                'category' => self::CATEGORY_ENGAGEMENT,
                'type' => self::TYPE_AVERAGE,
                'aggregation' => self::AGG_AVG,
                'metric_field' => 'page_views',
                'format' => self::FORMAT_DECIMAL,
                'decimal_places' => 1,
                'icon' => 'file-text',
                'color' => 'cyan',
                'dashboard_section' => 'engagement',
            ],
            [
                'kpi_key' => 'bounce_rate',
                'name' => 'Bounce Rate',
                'description' => 'Percentage of single-page sessions',
                'category' => self::CATEGORY_ENGAGEMENT,
                'type' => self::TYPE_PERCENTAGE,
                'format' => self::FORMAT_PERCENTAGE,
                'icon' => 'logout',
                'color' => 'red',
                'dashboard_section' => 'engagement',
            ],

            // Conversion KPIs
            [
                'kpi_key' => 'conversion_rate',
                'name' => 'Conversion Rate',
                'description' => 'Percentage of sessions that convert',
                'category' => self::CATEGORY_CONVERSION,
                'type' => self::TYPE_PERCENTAGE,
                'format' => self::FORMAT_PERCENTAGE,
                'icon' => 'target',
                'color' => 'green',
                'is_primary' => true,
                'dashboard_section' => 'overview',
            ],
            [
                'kpi_key' => 'total_conversions',
                'name' => 'Total Conversions',
                'description' => 'Total number of conversions',
                'category' => self::CATEGORY_CONVERSION,
                'type' => self::TYPE_COUNT,
                'aggregation' => self::AGG_COUNT,
                'filters' => ['is_conversion' => true],
                'format' => self::FORMAT_NUMBER,
                'icon' => 'check-circle',
                'color' => 'green',
                'dashboard_section' => 'conversion',
            ],

            // Acquisition KPIs
            [
                'kpi_key' => 'new_users',
                'name' => 'New Users',
                'description' => 'Number of first-time users',
                'category' => self::CATEGORY_ACQUISITION,
                'type' => self::TYPE_COUNT,
                'aggregation' => self::AGG_COUNT,
                'format' => self::FORMAT_NUMBER,
                'icon' => 'user-plus',
                'color' => 'blue',
                'is_primary' => true,
                'dashboard_section' => 'overview',
            ],
            [
                'kpi_key' => 'signups',
                'name' => 'Sign Ups',
                'description' => 'Number of new account registrations',
                'category' => self::CATEGORY_ACQUISITION,
                'type' => self::TYPE_COUNT,
                'aggregation' => self::AGG_COUNT,
                'filters' => ['event_name' => 'signup'],
                'format' => self::FORMAT_NUMBER,
                'icon' => 'user-check',
                'color' => 'purple',
                'dashboard_section' => 'acquisition',
            ],

            // Retention KPIs
            [
                'kpi_key' => 'returning_users',
                'name' => 'Returning Users',
                'description' => 'Users who have visited before',
                'category' => self::CATEGORY_RETENTION,
                'type' => self::TYPE_COUNT,
                'format' => self::FORMAT_NUMBER,
                'icon' => 'refresh',
                'color' => 'orange',
                'dashboard_section' => 'retention',
            ],
            [
                'kpi_key' => 'retention_rate',
                'name' => 'Retention Rate',
                'description' => 'Percentage of users who return',
                'category' => self::CATEGORY_RETENTION,
                'type' => self::TYPE_PERCENTAGE,
                'format' => self::FORMAT_PERCENTAGE,
                'icon' => 'heart',
                'color' => 'pink',
                'dashboard_section' => 'retention',
            ],

            // Growth KPIs
            [
                'kpi_key' => 'user_growth_rate',
                'name' => 'User Growth Rate',
                'description' => 'Period-over-period user growth',
                'category' => self::CATEGORY_GROWTH,
                'type' => self::TYPE_PERCENTAGE,
                'format' => self::FORMAT_PERCENTAGE,
                'icon' => 'trending-up',
                'color' => 'green',
                'dashboard_section' => 'growth',
            ],
            [
                'kpi_key' => 'revenue_growth_rate',
                'name' => 'Revenue Growth Rate',
                'description' => 'Period-over-period revenue growth',
                'category' => self::CATEGORY_GROWTH,
                'type' => self::TYPE_PERCENTAGE,
                'format' => self::FORMAT_PERCENTAGE,
                'icon' => 'trending-up',
                'color' => 'green',
                'dashboard_section' => 'growth',
            ],
        ];
    }

    /**
     * Seed default KPIs
     */
    public static function seedDefaults(): void
    {
        foreach (self::getDefaultKpis() as $kpi) {
            self::firstOrCreate(
                ['kpi_key' => $kpi['kpi_key']],
                $kpi
            );
        }
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Get the latest computed value for this KPI
     */
    public function getLatestValue(string $periodType = 'daily'): ?AnalyticsKpiValue
    {
        return $this->values()
            ->where('period_type', $periodType)
            ->orderByDesc('period_start')
            ->first();
    }

    /**
     * Get values for a date range
     */
    public function getValuesInRange(
        string $periodType,
        \Carbon\Carbon $startDate,
        \Carbon\Carbon $endDate
    ): Collection {
        return $this->values()
            ->where('period_type', $periodType)
            ->whereBetween('period_start', [$startDate, $endDate])
            ->orderBy('period_start')
            ->get();
    }

    /**
     * Format a value according to KPI settings
     */
    public function formatValue(float $value): string
    {
        return match ($this->format) {
            self::FORMAT_CURRENCY => '$' . number_format($value, $this->decimal_places),
            self::FORMAT_PERCENTAGE => number_format($value, $this->decimal_places) . '%',
            self::FORMAT_DURATION => $this->formatDuration((int) $value),
            self::FORMAT_DECIMAL => number_format($value, $this->decimal_places),
            default => number_format($value, 0),
        };
    }

    /**
     * Format seconds as human-readable duration
     */
    protected function formatDuration(int $seconds): string
    {
        if ($seconds < 60) {
            return $seconds . 's';
        }

        if ($seconds < 3600) {
            $minutes = floor($seconds / 60);
            $secs = $seconds % 60;
            return $minutes . 'm ' . $secs . 's';
        }

        $hours = floor($seconds / 3600);
        $minutes = floor(($seconds % 3600) / 60);
        return $hours . 'h ' . $minutes . 'm';
    }

    /**
     * Check if value meets target
     */
    public function meetsTarget(float $value): bool
    {
        if ($this->target_value === null) {
            return true;
        }

        return $value >= $this->target_value;
    }

    /**
     * Get status based on thresholds
     */
    public function getStatus(float $value): string
    {
        if ($this->critical_threshold !== null && $value <= $this->critical_threshold) {
            return 'critical';
        }

        if ($this->warning_threshold !== null && $value <= $this->warning_threshold) {
            return 'warning';
        }

        if ($this->target_value !== null && $value >= $this->target_value) {
            return 'success';
        }

        return 'normal';
    }
}
