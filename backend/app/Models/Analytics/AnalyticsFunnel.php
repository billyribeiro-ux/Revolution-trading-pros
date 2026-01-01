<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;

/**
 * AnalyticsFunnel Model - Conversion Funnel Configuration
 *
 * Defines multi-step conversion funnels for user journey analysis.
 *
 * @property int $id
 * @property string $funnel_key
 * @property string $name
 * @property string $funnel_type
 * @property int $conversion_window_hours
 * @property bool $is_ordered
 */
class AnalyticsFunnel extends Model
{
    protected $table = 'analytics_funnels';

    /**
     * Funnel Types
     */
    public const TYPE_CONVERSION = 'conversion';
    public const TYPE_ENGAGEMENT = 'engagement';
    public const TYPE_PURCHASE = 'purchase';
    public const TYPE_ONBOARDING = 'onboarding';

    protected $fillable = [
        'funnel_key',
        'name',
        'description',
        'funnel_type',
        'conversion_window_hours',
        'is_ordered',
        'is_active',
        'sort_order',
        'chart_config',
    ];

    protected $casts = [
        'conversion_window_hours' => 'integer',
        'is_ordered' => 'boolean',
        'is_active' => 'boolean',
        'sort_order' => 'integer',
        'chart_config' => 'array',
    ];

    protected $attributes = [
        'funnel_type' => self::TYPE_CONVERSION,
        'conversion_window_hours' => 168, // 7 days
        'is_ordered' => true,
        'is_active' => true,
        'sort_order' => 0,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function steps(): HasMany
    {
        return $this->hasMany(AnalyticsFunnelStep::class, 'funnel_id')->orderBy('step_number');
    }

    public function conversions(): HasMany
    {
        return $this->hasMany(AnalyticsFunnelConversion::class, 'funnel_id');
    }

    public function aggregates(): HasMany
    {
        return $this->hasMany(AnalyticsFunnelAggregate::class, 'funnel_id');
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
        return $query->where('funnel_type', $type);
    }

    // =========================================================================
    // STATIC METHODS - DEFAULT FUNNELS
    // =========================================================================

    /**
     * Get predefined funnel configurations
     */
    public static function getDefaultFunnels(): array
    {
        return [
            [
                'funnel_key' => 'signup_to_purchase',
                'name' => 'Signup to Purchase',
                'description' => 'Track user journey from signup to first purchase',
                'funnel_type' => self::TYPE_PURCHASE,
                'conversion_window_hours' => 720, // 30 days
                'steps' => [
                    ['step_number' => 1, 'name' => 'Visit Site', 'event_name' => 'page_view'],
                    ['step_number' => 2, 'name' => 'Sign Up', 'event_name' => 'signup'],
                    ['step_number' => 3, 'name' => 'View Product', 'event_name' => 'product_view'],
                    ['step_number' => 4, 'name' => 'Add to Cart', 'event_name' => 'add_to_cart'],
                    ['step_number' => 5, 'name' => 'Complete Purchase', 'event_name' => 'purchase'],
                ],
            ],
            [
                'funnel_key' => 'course_enrollment',
                'name' => 'Course Enrollment',
                'description' => 'Track user journey to course enrollment',
                'funnel_type' => self::TYPE_CONVERSION,
                'conversion_window_hours' => 168, // 7 days
                'steps' => [
                    ['step_number' => 1, 'name' => 'View Course', 'event_name' => 'course_view'],
                    ['step_number' => 2, 'name' => 'View Curriculum', 'event_name' => 'curriculum_view'],
                    ['step_number' => 3, 'name' => 'Start Checkout', 'event_name' => 'checkout_start'],
                    ['step_number' => 4, 'name' => 'Complete Enrollment', 'event_name' => 'course_enrolled'],
                ],
            ],
            [
                'funnel_key' => 'subscription_flow',
                'name' => 'Subscription Flow',
                'description' => 'Track subscription conversion funnel',
                'funnel_type' => self::TYPE_CONVERSION,
                'conversion_window_hours' => 336, // 14 days
                'steps' => [
                    ['step_number' => 1, 'name' => 'View Pricing', 'event_name' => 'pricing_view'],
                    ['step_number' => 2, 'name' => 'Select Plan', 'event_name' => 'plan_selected'],
                    ['step_number' => 3, 'name' => 'Enter Payment', 'event_name' => 'payment_started'],
                    ['step_number' => 4, 'name' => 'Subscribe', 'event_name' => 'subscription_created'],
                ],
            ],
            [
                'funnel_key' => 'user_onboarding',
                'name' => 'User Onboarding',
                'description' => 'Track new user onboarding completion',
                'funnel_type' => self::TYPE_ONBOARDING,
                'conversion_window_hours' => 168, // 7 days
                'steps' => [
                    ['step_number' => 1, 'name' => 'Complete Registration', 'event_name' => 'signup'],
                    ['step_number' => 2, 'name' => 'Verify Email', 'event_name' => 'email_verified'],
                    ['step_number' => 3, 'name' => 'Complete Profile', 'event_name' => 'profile_completed'],
                    ['step_number' => 4, 'name' => 'First Action', 'event_name' => 'first_engagement'],
                ],
            ],
        ];
    }

    /**
     * Seed default funnels with steps
     */
    public static function seedDefaults(): void
    {
        foreach (self::getDefaultFunnels() as $funnelData) {
            $steps = $funnelData['steps'] ?? [];
            unset($funnelData['steps']);

            $funnel = self::firstOrCreate(
                ['funnel_key' => $funnelData['funnel_key']],
                $funnelData
            );

            foreach ($steps as $stepData) {
                AnalyticsFunnelStep::firstOrCreate(
                    [
                        'funnel_id' => $funnel->id,
                        'step_number' => $stepData['step_number'],
                    ],
                    $stepData
                );
            }
        }
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Get funnel analysis for date range
     */
    public function analyze(Carbon $startDate, Carbon $endDate): array
    {
        $steps = $this->steps()->get();
        $results = [];
        $previousCount = null;

        foreach ($steps as $step) {
            $count = $this->getStepCount($step, $startDate, $endDate);

            $conversionRate = $previousCount !== null && $previousCount > 0
                ? round(($count / $previousCount) * 100, 2)
                : 100;

            $overallConversion = $results[0]['count'] ?? $count;
            $overallRate = $overallConversion > 0
                ? round(($count / $overallConversion) * 100, 2)
                : 100;

            $results[] = [
                'step_number' => $step->step_number,
                'name' => $step->name,
                'event_name' => $step->event_name,
                'count' => $count,
                'conversion_rate' => $conversionRate,
                'overall_conversion_rate' => $overallRate,
                'drop_off' => $previousCount !== null ? $previousCount - $count : 0,
                'drop_off_rate' => $previousCount !== null && $previousCount > 0
                    ? round((($previousCount - $count) / $previousCount) * 100, 2)
                    : 0,
            ];

            $previousCount = $count;
        }

        return [
            'funnel' => [
                'key' => $this->funnel_key,
                'name' => $this->name,
                'total_steps' => count($steps),
            ],
            'period' => [
                'start' => $startDate->format('Y-m-d'),
                'end' => $endDate->format('Y-m-d'),
            ],
            'steps' => $results,
            'summary' => $this->calculateSummary($results),
        ];
    }

    /**
     * Get unique user count for a funnel step
     */
    protected function getStepCount(AnalyticsFunnelStep $step, Carbon $startDate, Carbon $endDate): int
    {
        return AnalyticsEvent::query()
            ->byEventName($step->event_name)
            ->inDateRange($startDate, $endDate)
            ->count(DB::raw('DISTINCT COALESCE(user_id, anonymous_id)'));
    }

    /**
     * Calculate funnel summary metrics
     */
    protected function calculateSummary(array $steps): array
    {
        if (empty($steps)) {
            return [];
        }

        $firstStep = $steps[0];
        $lastStep = end($steps);

        return [
            'total_entries' => $firstStep['count'],
            'total_conversions' => $lastStep['count'],
            'overall_conversion_rate' => $lastStep['overall_conversion_rate'],
            'biggest_drop_off_step' => collect($steps)
                ->sortByDesc('drop_off_rate')
                ->first()['name'] ?? null,
            'avg_conversion_rate' => round(
                collect($steps)->avg('conversion_rate'),
                2
            ),
        ];
    }

    /**
     * Get time-series funnel data
     */
    public function getTimeSeries(string $granularity, Carbon $startDate, Carbon $endDate): Collection
    {
        return $this->aggregates()
            ->where('period_type', $granularity)
            ->whereBetween('period_start', [$startDate, $endDate])
            ->orderBy('period_start')
            ->get();
    }

    /**
     * Get drop-off analysis
     */
    public function getDropOffAnalysis(Carbon $startDate, Carbon $endDate): array
    {
        $analysis = $this->analyze($startDate, $endDate);
        $steps = $analysis['steps'];

        $dropOffs = [];
        foreach ($steps as $index => $step) {
            if ($index === 0) {
                continue;
            }

            $dropOffs[] = [
                'from_step' => $steps[$index - 1]['name'],
                'to_step' => $step['name'],
                'users_lost' => $step['drop_off'],
                'drop_off_rate' => $step['drop_off_rate'],
                'potential_revenue_lost' => $this->estimateRevenueLost($step['drop_off']),
            ];
        }

        return [
            'drop_offs' => $dropOffs,
            'critical_step' => collect($dropOffs)->sortByDesc('drop_off_rate')->first(),
            'total_lost' => collect($dropOffs)->sum('users_lost'),
        ];
    }

    /**
     * Estimate revenue lost from drop-offs
     */
    protected function estimateRevenueLost(int $droppedUsers): float
    {
        // Get average conversion value from completed conversions
        $avgValue = $this->conversions()
            ->where('is_converted', true)
            ->whereNotNull('conversion_value')
            ->avg('conversion_value') ?? 0;

        return round($droppedUsers * $avgValue * 0.5, 2); // 50% potential conversion
    }
}
