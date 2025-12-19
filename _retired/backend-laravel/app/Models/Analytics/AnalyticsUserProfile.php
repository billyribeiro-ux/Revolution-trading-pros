<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use App\Models\User;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * AnalyticsUserProfile Model - Aggregated User Analytics Data
 *
 * Stores computed user-level analytics for fast queries.
 *
 * @property int $id
 * @property int $user_id
 * @property string|null $lifecycle_stage
 * @property float $engagement_score
 * @property float $total_revenue
 */
class AnalyticsUserProfile extends Model
{
    protected $table = 'analytics_user_profiles';

    /**
     * Lifecycle Stages
     */
    public const STAGE_NEW = 'new';
    public const STAGE_ACTIVE = 'active';
    public const STAGE_AT_RISK = 'at_risk';
    public const STAGE_DORMANT = 'dormant';
    public const STAGE_CHURNED = 'churned';

    /**
     * Engagement Levels
     */
    public const ENGAGEMENT_LOW = 'low';
    public const ENGAGEMENT_MEDIUM = 'medium';
    public const ENGAGEMENT_HIGH = 'high';
    public const ENGAGEMENT_POWER = 'power';

    /**
     * Value Tiers
     */
    public const VALUE_LOW = 'low';
    public const VALUE_MEDIUM = 'medium';
    public const VALUE_HIGH = 'high';
    public const VALUE_VIP = 'vip';

    /**
     * Churn Risk Levels
     */
    public const RISK_LOW = 'low';
    public const RISK_MEDIUM = 'medium';
    public const RISK_HIGH = 'high';

    protected $fillable = [
        'user_id',
        'first_seen_at',
        'last_seen_at',
        'days_since_first_seen',
        'days_since_last_seen',
        'lifecycle_stage',
        'total_sessions',
        'total_events',
        'total_page_views',
        'avg_session_duration',
        'avg_pages_per_session',
        'engagement_score',
        'engagement_level',
        'streak_days',
        'longest_streak',
        'has_converted',
        'first_conversion_at',
        'total_conversions',
        'total_revenue',
        'avg_order_value',
        'predicted_ltv',
        'value_tier',
        'acquisition_channel',
        'acquisition_source',
        'acquisition_campaign',
        'preferred_device',
        'preferred_browser',
        'primary_country',
        'interests',
        'segment_ids',
        'churn_risk_score',
        'churn_risk_level',
        'last_event_name',
        'last_event_at',
    ];

    protected $casts = [
        'user_id' => 'integer',
        'first_seen_at' => 'datetime',
        'last_seen_at' => 'datetime',
        'days_since_first_seen' => 'integer',
        'days_since_last_seen' => 'integer',
        'total_sessions' => 'integer',
        'total_events' => 'integer',
        'total_page_views' => 'integer',
        'avg_session_duration' => 'integer',
        'avg_pages_per_session' => 'decimal:2',
        'engagement_score' => 'decimal:2',
        'streak_days' => 'integer',
        'longest_streak' => 'integer',
        'has_converted' => 'boolean',
        'first_conversion_at' => 'datetime',
        'total_conversions' => 'integer',
        'total_revenue' => 'decimal:4',
        'avg_order_value' => 'decimal:4',
        'predicted_ltv' => 'decimal:4',
        'interests' => 'array',
        'segment_ids' => 'array',
        'churn_risk_score' => 'decimal:2',
        'last_event_at' => 'datetime',
    ];

    protected $attributes = [
        'days_since_first_seen' => 0,
        'days_since_last_seen' => 0,
        'total_sessions' => 0,
        'total_events' => 0,
        'total_page_views' => 0,
        'avg_session_duration' => 0,
        'avg_pages_per_session' => 0,
        'engagement_score' => 0,
        'streak_days' => 0,
        'longest_streak' => 0,
        'has_converted' => false,
        'total_conversions' => 0,
        'total_revenue' => 0,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeByLifecycleStage(Builder $query, string $stage): Builder
    {
        return $query->where('lifecycle_stage', $stage);
    }

    public function scopeByEngagementLevel(Builder $query, string $level): Builder
    {
        return $query->where('engagement_level', $level);
    }

    public function scopeByValueTier(Builder $query, string $tier): Builder
    {
        return $query->where('value_tier', $tier);
    }

    public function scopeAtRisk(Builder $query): Builder
    {
        return $query->where('churn_risk_level', self::RISK_HIGH);
    }

    public function scopeConverted(Builder $query): Builder
    {
        return $query->where('has_converted', true);
    }

    public function scopeHighValue(Builder $query, float $threshold = 500): Builder
    {
        return $query->where('total_revenue', '>=', $threshold);
    }

    public function scopeActiveInLast(Builder $query, int $days): Builder
    {
        return $query->where('last_seen_at', '>=', now()->subDays($days));
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    /**
     * Get or create profile for user
     */
    public static function findOrCreateForUser(int $userId): self
    {
        return static::firstOrCreate(
            ['user_id' => $userId],
            ['first_seen_at' => now()]
        );
    }

    /**
     * Get lifecycle stage distribution
     */
    public static function getLifecycleDistribution(): array
    {
        return static::query()
            ->selectRaw('lifecycle_stage, COUNT(*) as count')
            ->groupBy('lifecycle_stage')
            ->pluck('count', 'lifecycle_stage')
            ->toArray();
    }

    /**
     * Get value tier distribution
     */
    public static function getValueTierDistribution(): array
    {
        return static::query()
            ->selectRaw('value_tier, COUNT(*) as count, SUM(total_revenue) as total_revenue')
            ->groupBy('value_tier')
            ->get()
            ->keyBy('value_tier')
            ->toArray();
    }

    // =========================================================================
    // INSTANCE METHODS
    // =========================================================================

    /**
     * Update profile from event
     */
    public function recordEvent(AnalyticsEvent $event): void
    {
        $this->total_events++;

        if ($event->event_category === AnalyticsEvent::CATEGORY_PAGE_VIEW) {
            $this->total_page_views++;
        }

        $this->last_seen_at = $event->event_timestamp;
        $this->last_event_name = $event->event_name;
        $this->last_event_at = $event->event_timestamp;

        // Update days since
        $this->days_since_last_seen = 0;
        if ($this->first_seen_at) {
            $this->days_since_first_seen = $this->first_seen_at->diffInDays(now());
        }

        // Track conversion
        if ($event->is_conversion) {
            if (!$this->has_converted) {
                $this->has_converted = true;
                $this->first_conversion_at = $event->event_timestamp;
            }
            $this->total_conversions++;
        }

        // Track revenue
        if ($event->revenue) {
            $this->total_revenue += $event->revenue;
            $this->avg_order_value = $this->total_conversions > 0
                ? $this->total_revenue / $this->total_conversions
                : 0;
        }

        // First time setup
        if (!$this->first_seen_at) {
            $this->first_seen_at = $event->event_timestamp;

            // Set acquisition data
            $this->acquisition_channel = $event->channel;
            $this->acquisition_source = $event->utm_source;
            $this->acquisition_campaign = $event->utm_campaign;
        }

        $this->save();
    }

    /**
     * Record new session
     */
    public function recordSession(AnalyticsSession $session): void
    {
        $this->total_sessions++;
        $this->last_seen_at = $session->started_at;

        // Update averages
        if ($session->duration_seconds > 0) {
            $this->avg_session_duration = (int) round(
                (($this->avg_session_duration * ($this->total_sessions - 1)) + $session->duration_seconds) / $this->total_sessions
            );
        }

        if ($session->page_views > 0) {
            $this->avg_pages_per_session = round(
                (($this->avg_pages_per_session * ($this->total_sessions - 1)) + $session->page_views) / $this->total_sessions,
                2
            );
        }

        // Track device preferences
        if ($session->device_type) {
            $this->preferred_device = $session->device_type;
        }
        if ($session->browser) {
            $this->preferred_browser = $session->browser;
        }
        if ($session->country_code) {
            $this->primary_country = $session->country_code;
        }

        // Update conversion tracking
        if ($session->had_conversion) {
            $this->has_converted = true;
            $this->total_revenue += $session->total_value;
        }

        $this->save();
    }

    /**
     * Recalculate all metrics
     */
    public function recalculate(): void
    {
        // Update time-based metrics
        if ($this->first_seen_at) {
            $this->days_since_first_seen = $this->first_seen_at->diffInDays(now());
        }
        if ($this->last_seen_at) {
            $this->days_since_last_seen = $this->last_seen_at->diffInDays(now());
        }

        // Calculate engagement score
        $this->engagement_score = $this->calculateEngagementScore();
        $this->engagement_level = $this->determineEngagementLevel();

        // Determine lifecycle stage
        $this->lifecycle_stage = $this->determineLifecycleStage();

        // Determine value tier
        $this->value_tier = $this->determineValueTier();

        // Calculate churn risk
        $this->churn_risk_score = $this->calculateChurnRiskScore();
        $this->churn_risk_level = $this->determineChurnRiskLevel();

        // Predict LTV
        $this->predicted_ltv = $this->predictLTV();

        $this->save();
    }

    /**
     * Calculate engagement score (0-100)
     */
    protected function calculateEngagementScore(): float
    {
        $score = 0;

        // Recency (up to 30 points)
        $recencyScore = max(0, 30 - ($this->days_since_last_seen * 2));
        $score += $recencyScore;

        // Frequency (up to 30 points)
        $frequencyScore = min(30, $this->total_sessions * 1.5);
        $score += $frequencyScore;

        // Depth (up to 20 points)
        $avgPages = $this->avg_pages_per_session;
        $depthScore = min(20, $avgPages * 4);
        $score += $depthScore;

        // Value (up to 20 points)
        $valueScore = min(20, ($this->total_revenue / 100) * 2);
        $score += $valueScore;

        return round(min(100, $score), 2);
    }

    /**
     * Determine engagement level
     */
    protected function determineEngagementLevel(): string
    {
        return match (true) {
            $this->engagement_score >= 80 => self::ENGAGEMENT_POWER,
            $this->engagement_score >= 50 => self::ENGAGEMENT_HIGH,
            $this->engagement_score >= 25 => self::ENGAGEMENT_MEDIUM,
            default => self::ENGAGEMENT_LOW,
        };
    }

    /**
     * Determine lifecycle stage
     */
    protected function determineLifecycleStage(): string
    {
        return match (true) {
            $this->days_since_first_seen <= 7 => self::STAGE_NEW,
            $this->days_since_last_seen >= 30 => self::STAGE_CHURNED,
            $this->days_since_last_seen >= 14 => self::STAGE_DORMANT,
            $this->days_since_last_seen >= 7 => self::STAGE_AT_RISK,
            default => self::STAGE_ACTIVE,
        };
    }

    /**
     * Determine value tier
     */
    protected function determineValueTier(): string
    {
        return match (true) {
            $this->total_revenue >= 1000 => self::VALUE_VIP,
            $this->total_revenue >= 500 => self::VALUE_HIGH,
            $this->total_revenue >= 100 => self::VALUE_MEDIUM,
            default => self::VALUE_LOW,
        };
    }

    /**
     * Calculate churn risk score
     */
    protected function calculateChurnRiskScore(): float
    {
        $score = 0;

        // Days inactive
        $score += min(40, $this->days_since_last_seen * 2);

        // Low engagement
        if ($this->engagement_score < 30) {
            $score += 20;
        }

        // Decreasing activity (simplified)
        if ($this->total_sessions > 0 && $this->days_since_first_seen > 0) {
            $avgSessionsPerWeek = ($this->total_sessions / $this->days_since_first_seen) * 7;
            if ($avgSessionsPerWeek < 1) {
                $score += 20;
            }
        }

        // Never converted
        if (!$this->has_converted && $this->days_since_first_seen > 14) {
            $score += 20;
        }

        return round(min(100, $score), 2);
    }

    /**
     * Determine churn risk level
     */
    protected function determineChurnRiskLevel(): string
    {
        return match (true) {
            $this->churn_risk_score >= 60 => self::RISK_HIGH,
            $this->churn_risk_score >= 30 => self::RISK_MEDIUM,
            default => self::RISK_LOW,
        };
    }

    /**
     * Simple LTV prediction
     */
    protected function predictLTV(): float
    {
        if (!$this->has_converted) {
            return 0;
        }

        // Simple projection: current revenue * retention factor
        $retentionFactor = match ($this->churn_risk_level) {
            self::RISK_HIGH => 1.2,
            self::RISK_MEDIUM => 1.5,
            default => 2.0,
        };

        return round($this->total_revenue * $retentionFactor, 2);
    }
}
