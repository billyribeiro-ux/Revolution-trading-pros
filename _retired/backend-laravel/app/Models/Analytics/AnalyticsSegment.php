<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use App\Models\User;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;

/**
 * AnalyticsSegment Model - User Segmentation
 *
 * Defines dynamic and static user segments for targeted analytics.
 *
 * @property int $id
 * @property string $segment_key
 * @property string $name
 * @property string $segment_type
 * @property array|null $rules
 */
class AnalyticsSegment extends Model
{
    protected $table = 'analytics_segments';

    /**
     * Segment Types
     */
    public const TYPE_STATIC = 'static';
    public const TYPE_DYNAMIC = 'dynamic';
    public const TYPE_COMPUTED = 'computed';

    /**
     * Rule Operators
     */
    public const OP_AND = 'AND';
    public const OP_OR = 'OR';

    /**
     * Refresh Frequencies
     */
    public const REFRESH_REALTIME = 'realtime';
    public const REFRESH_HOURLY = 'hourly';
    public const REFRESH_DAILY = 'daily';

    protected $fillable = [
        'segment_key',
        'name',
        'description',
        'segment_type',
        'rules',
        'rule_operator',
        'refresh_frequency',
        'last_computed_at',
        'user_count',
        'percentage_of_total',
        'is_active',
        'is_system',
        'color',
        'icon',
        'sort_order',
    ];

    protected $casts = [
        'rules' => 'array',
        'user_count' => 'integer',
        'percentage_of_total' => 'decimal:4',
        'is_active' => 'boolean',
        'is_system' => 'boolean',
        'sort_order' => 'integer',
        'last_computed_at' => 'datetime',
    ];

    protected $attributes = [
        'segment_type' => self::TYPE_DYNAMIC,
        'rule_operator' => self::OP_AND,
        'refresh_frequency' => self::REFRESH_DAILY,
        'user_count' => 0,
        'is_active' => true,
        'is_system' => false,
        'sort_order' => 0,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function users(): BelongsToMany
    {
        return $this->belongsToMany(User::class, 'analytics_segment_members', 'segment_id', 'user_id')
            ->withPivot(['added_at', 'removed_at', 'is_active', 'properties_snapshot'])
            ->wherePivot('is_active', true);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    public function scopeSystem(Builder $query): Builder
    {
        return $query->where('is_system', true);
    }

    public function scopeCustom(Builder $query): Builder
    {
        return $query->where('is_system', false);
    }

    public function scopeByType(Builder $query, string $type): Builder
    {
        return $query->where('segment_type', $type);
    }

    // =========================================================================
    // STATIC METHODS - DEFAULT SEGMENTS
    // =========================================================================

    /**
     * Get predefined segment configurations
     */
    public static function getDefaultSegments(): array
    {
        return [
            // Engagement Segments
            [
                'segment_key' => 'power_users',
                'name' => 'Power Users',
                'description' => 'Highly engaged users with 10+ sessions',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'total_sessions', 'operator' => 'gte', 'value' => 10],
                    ['field' => 'engagement_score', 'operator' => 'gte', 'value' => 70],
                ],
                'is_system' => true,
                'color' => 'purple',
                'icon' => 'star',
            ],
            [
                'segment_key' => 'at_risk',
                'name' => 'At Risk',
                'description' => 'Users who haven\'t been active in 14+ days',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'days_since_last_seen', 'operator' => 'gte', 'value' => 14],
                    ['field' => 'days_since_last_seen', 'operator' => 'lt', 'value' => 30],
                ],
                'is_system' => true,
                'color' => 'orange',
                'icon' => 'alert-triangle',
            ],
            [
                'segment_key' => 'churned',
                'name' => 'Churned',
                'description' => 'Users inactive for 30+ days',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'days_since_last_seen', 'operator' => 'gte', 'value' => 30],
                ],
                'is_system' => true,
                'color' => 'red',
                'icon' => 'user-x',
            ],
            [
                'segment_key' => 'new_users',
                'name' => 'New Users',
                'description' => 'Users who signed up in the last 7 days',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'days_since_first_seen', 'operator' => 'lte', 'value' => 7],
                ],
                'is_system' => true,
                'color' => 'green',
                'icon' => 'user-plus',
            ],

            // Value Segments
            [
                'segment_key' => 'high_value',
                'name' => 'High Value Customers',
                'description' => 'Customers with $500+ lifetime value',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'total_revenue', 'operator' => 'gte', 'value' => 500],
                ],
                'is_system' => true,
                'color' => 'gold',
                'icon' => 'crown',
            ],
            [
                'segment_key' => 'subscribers',
                'name' => 'Active Subscribers',
                'description' => 'Users with active subscriptions',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'has_active_subscription', 'operator' => 'eq', 'value' => true],
                ],
                'is_system' => true,
                'color' => 'blue',
                'icon' => 'credit-card',
            ],
            [
                'segment_key' => 'free_users',
                'name' => 'Free Users',
                'description' => 'Users who haven\'t made a purchase',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'total_revenue', 'operator' => 'eq', 'value' => 0],
                ],
                'is_system' => true,
                'color' => 'gray',
                'icon' => 'user',
            ],

            // Channel Segments
            [
                'segment_key' => 'organic_users',
                'name' => 'Organic Users',
                'description' => 'Users acquired through organic search',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'acquisition_channel', 'operator' => 'eq', 'value' => 'organic'],
                ],
                'is_system' => true,
                'color' => 'teal',
                'icon' => 'search',
            ],
            [
                'segment_key' => 'paid_users',
                'name' => 'Paid Acquisition',
                'description' => 'Users acquired through paid channels',
                'segment_type' => self::TYPE_DYNAMIC,
                'rules' => [
                    ['field' => 'acquisition_channel', 'operator' => 'eq', 'value' => 'paid'],
                ],
                'is_system' => true,
                'color' => 'pink',
                'icon' => 'target',
            ],
        ];
    }

    /**
     * Seed default segments
     */
    public static function seedDefaults(): void
    {
        foreach (self::getDefaultSegments() as $segment) {
            self::firstOrCreate(
                ['segment_key' => $segment['segment_key']],
                $segment
            );
        }
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Check if a user matches segment rules
     */
    public function matchesUser(AnalyticsUserProfile $profile): bool
    {
        if (empty($this->rules)) {
            return false;
        }

        $results = [];

        foreach ($this->rules as $rule) {
            $fieldValue = $profile->{$rule['field']} ?? null;
            $results[] = $this->evaluateRule($fieldValue, $rule['operator'], $rule['value']);
        }

        return $this->rule_operator === self::OP_AND
            ? !in_array(false, $results, true)
            : in_array(true, $results, true);
    }

    /**
     * Evaluate a single rule
     */
    protected function evaluateRule(mixed $fieldValue, string $operator, mixed $ruleValue): bool
    {
        return match ($operator) {
            'eq' => $fieldValue == $ruleValue,
            'neq' => $fieldValue != $ruleValue,
            'gt' => $fieldValue > $ruleValue,
            'gte' => $fieldValue >= $ruleValue,
            'lt' => $fieldValue < $ruleValue,
            'lte' => $fieldValue <= $ruleValue,
            'in' => in_array($fieldValue, (array) $ruleValue),
            'not_in' => !in_array($fieldValue, (array) $ruleValue),
            'contains' => str_contains((string) $fieldValue, (string) $ruleValue),
            'starts_with' => str_starts_with((string) $fieldValue, (string) $ruleValue),
            'is_null' => $fieldValue === null,
            'is_not_null' => $fieldValue !== null,
            default => false,
        };
    }

    /**
     * Recompute segment membership
     */
    public function recompute(): int
    {
        $profiles = AnalyticsUserProfile::all();
        $matchedUserIds = [];

        foreach ($profiles as $profile) {
            if ($this->matchesUser($profile)) {
                $matchedUserIds[] = $profile->user_id;
            }
        }

        // Update memberships
        DB::table('analytics_segment_members')
            ->where('segment_id', $this->id)
            ->update(['is_active' => false, 'removed_at' => now()]);

        foreach ($matchedUserIds as $userId) {
            DB::table('analytics_segment_members')->updateOrInsert(
                ['segment_id' => $this->id, 'user_id' => $userId],
                ['is_active' => true, 'added_at' => now(), 'removed_at' => null]
            );
        }

        // Update counts
        $totalUsers = User::count();
        $this->user_count = count($matchedUserIds);
        $this->percentage_of_total = $totalUsers > 0
            ? round(($this->user_count / $totalUsers) * 100, 4)
            : 0;
        $this->last_computed_at = now();
        $this->save();

        return $this->user_count;
    }

    /**
     * Get segment comparison data
     */
    public static function getComparison(array $segmentIds): Collection
    {
        return static::whereIn('id', $segmentIds)
            ->select('id', 'segment_key', 'name', 'user_count', 'percentage_of_total', 'color')
            ->get();
    }
}
