<?php

declare(strict_types=1);

namespace App\Models\Analytics;

use App\Models\User;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * AnalyticsEvent Model - Enterprise Global Event Tracking
 *
 * Unified event schema for all platform events. Designed to surpass
 * Mixpanel, Amplitude, GA4, and Heap Analytics in capability.
 *
 * @property int $id
 * @property string $event_id
 * @property string $event_name
 * @property string $event_category
 * @property string $event_type
 * @property string $event_source
 * @property string|null $entity_type
 * @property int|null $entity_id
 * @property int|null $user_id
 * @property string|null $anonymous_id
 * @property string $session_id
 * @property array|null $properties
 * @property array|null $user_properties
 * @property array|null $context
 * @property Carbon $event_timestamp
 * @property bool $is_conversion
 * @property-read User|null $user
 * @property-read AnalyticsSession|null $session
 */
class AnalyticsEvent extends Model
{
    use HasFactory;

    protected $table = 'analytics_events';

    /**
     * Event Categories
     */
    public const CATEGORY_PAGE_VIEW = 'page_view';
    public const CATEGORY_USER_ACTION = 'user_action';
    public const CATEGORY_TRANSACTION = 'transaction';
    public const CATEGORY_SYSTEM = 'system';
    public const CATEGORY_ENGAGEMENT = 'engagement';
    public const CATEGORY_CONVERSION = 'conversion';
    public const CATEGORY_ERROR = 'error';

    /**
     * Event Types
     */
    public const TYPE_VIEW = 'view';
    public const TYPE_CLICK = 'click';
    public const TYPE_SUBMIT = 'submit';
    public const TYPE_PURCHASE = 'purchase';
    public const TYPE_SIGNUP = 'signup';
    public const TYPE_LOGIN = 'login';
    public const TYPE_SUBSCRIBE = 'subscribe';
    public const TYPE_UNSUBSCRIBE = 'unsubscribe';
    public const TYPE_SHARE = 'share';
    public const TYPE_DOWNLOAD = 'download';
    public const TYPE_PLAY = 'play';
    public const TYPE_COMPLETE = 'complete';
    public const TYPE_ERROR = 'error';

    /**
     * Event Sources
     */
    public const SOURCE_WEB = 'web';
    public const SOURCE_API = 'api';
    public const SOURCE_MOBILE = 'mobile';
    public const SOURCE_WEBHOOK = 'webhook';
    public const SOURCE_SYSTEM = 'system';

    /**
     * Channel Types
     */
    public const CHANNEL_ORGANIC = 'organic';
    public const CHANNEL_PAID = 'paid';
    public const CHANNEL_SOCIAL = 'social';
    public const CHANNEL_EMAIL = 'email';
    public const CHANNEL_DIRECT = 'direct';
    public const CHANNEL_REFERRAL = 'referral';
    public const CHANNEL_AFFILIATE = 'affiliate';

    /**
     * Journey Stages
     */
    public const STAGE_AWARENESS = 'awareness';
    public const STAGE_CONSIDERATION = 'consideration';
    public const STAGE_DECISION = 'decision';
    public const STAGE_RETENTION = 'retention';
    public const STAGE_ADVOCACY = 'advocacy';

    protected $fillable = [
        'event_id',
        'event_name',
        'event_category',
        'event_type',
        'event_source',
        'entity_type',
        'entity_id',
        'user_id',
        'anonymous_id',
        'session_id',
        'utm_source',
        'utm_medium',
        'utm_campaign',
        'utm_term',
        'utm_content',
        'referrer',
        'referrer_domain',
        'channel',
        'channel_group',
        'device_type',
        'device_brand',
        'device_model',
        'browser',
        'browser_version',
        'os',
        'os_version',
        'screen_resolution',
        'viewport_size',
        'ip_address',
        'country_code',
        'country',
        'region',
        'city',
        'timezone',
        'latitude',
        'longitude',
        'page_url',
        'page_path',
        'page_title',
        'page_type',
        'event_value',
        'currency',
        'revenue',
        'quantity',
        'properties',
        'user_properties',
        'context',
        'funnel_id',
        'funnel_step',
        'journey_stage',
        'experiment_id',
        'variant_id',
        'page_load_time',
        'time_on_page',
        'scroll_depth',
        'engagement_score',
        'is_bounce',
        'is_conversion',
        'is_goal_completion',
        'event_timestamp',
        'event_day',
        'event_hour',
        'is_processed',
        'processed_at',
    ];

    protected $casts = [
        'entity_id' => 'integer',
        'user_id' => 'integer',
        'event_value' => 'decimal:4',
        'revenue' => 'decimal:4',
        'quantity' => 'integer',
        'latitude' => 'decimal:7',
        'longitude' => 'decimal:7',
        'page_load_time' => 'integer',
        'time_on_page' => 'integer',
        'scroll_depth' => 'integer',
        'engagement_score' => 'decimal:2',
        'is_bounce' => 'boolean',
        'is_conversion' => 'boolean',
        'is_goal_completion' => 'boolean',
        'is_processed' => 'boolean',
        'event_day' => 'integer',
        'event_hour' => 'integer',
        'properties' => 'array',
        'user_properties' => 'array',
        'context' => 'array',
        'event_timestamp' => 'datetime',
        'processed_at' => 'datetime',
    ];

    protected $attributes = [
        'event_source' => self::SOURCE_WEB,
        'currency' => 'USD',
        'is_bounce' => false,
        'is_conversion' => false,
        'is_goal_completion' => false,
        'is_processed' => false,
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function session(): BelongsTo
    {
        return $this->belongsTo(AnalyticsSession::class, 'session_id', 'session_id');
    }

    // =========================================================================
    // BOOT METHODS
    // =========================================================================

    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $event) {
            if (empty($event->event_id)) {
                $event->event_id = Str::uuid()->toString();
            }

            if (empty($event->event_timestamp)) {
                $event->event_timestamp = now();
            }

            // Auto-populate event_day and event_hour
            $timestamp = $event->event_timestamp instanceof Carbon
                ? $event->event_timestamp
                : Carbon::parse($event->event_timestamp);

            $event->event_day = (int) $timestamp->format('Ymd');
            $event->event_hour = (int) $timestamp->format('H');

            // Auto-detect channel from UTM parameters
            if (empty($event->channel)) {
                $event->channel = self::detectChannel($event);
            }

            // Extract referrer domain
            if ($event->referrer && empty($event->referrer_domain)) {
                $event->referrer_domain = parse_url($event->referrer, PHP_URL_HOST);
            }

            // Generate session ID if not provided
            if (empty($event->session_id)) {
                $event->session_id = Str::uuid()->toString();
            }
        });
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    public function scopeByEventName(Builder $query, string $eventName): Builder
    {
        return $query->where('event_name', $eventName);
    }

    public function scopeByCategory(Builder $query, string $category): Builder
    {
        return $query->where('event_category', $category);
    }

    public function scopeByType(Builder $query, string $type): Builder
    {
        return $query->where('event_type', $type);
    }

    public function scopeByUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    public function scopeBySession(Builder $query, string $sessionId): Builder
    {
        return $query->where('session_id', $sessionId);
    }

    public function scopeByEntity(Builder $query, string $entityType, ?int $entityId = null): Builder
    {
        $query->where('entity_type', $entityType);

        if ($entityId !== null) {
            $query->where('entity_id', $entityId);
        }

        return $query;
    }

    public function scopeByChannel(Builder $query, string $channel): Builder
    {
        return $query->where('channel', $channel);
    }

    public function scopeConversions(Builder $query): Builder
    {
        return $query->where('is_conversion', true);
    }

    public function scopeGoalCompletions(Builder $query): Builder
    {
        return $query->where('is_goal_completion', true);
    }

    public function scopeInDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('event_timestamp', [$start, $end]);
    }

    public function scopeToday(Builder $query): Builder
    {
        return $query->where('event_day', (int) now()->format('Ymd'));
    }

    public function scopeLastNDays(Builder $query, int $days): Builder
    {
        $startDay = (int) now()->subDays($days)->format('Ymd');
        return $query->where('event_day', '>=', $startDay);
    }

    public function scopeByCountry(Builder $query, string $countryCode): Builder
    {
        return $query->where('country_code', strtoupper($countryCode));
    }

    public function scopeByDevice(Builder $query, string $deviceType): Builder
    {
        return $query->where('device_type', $deviceType);
    }

    public function scopeUnprocessed(Builder $query): Builder
    {
        return $query->where('is_processed', false);
    }

    public function scopeWithRevenue(Builder $query): Builder
    {
        return $query->whereNotNull('revenue')->where('revenue', '>', 0);
    }

    // =========================================================================
    // STATIC FACTORY METHODS
    // =========================================================================

    /**
     * Create a page view event
     */
    public static function pageView(array $data): self
    {
        return static::create(array_merge($data, [
            'event_name' => 'page_view',
            'event_category' => self::CATEGORY_PAGE_VIEW,
            'event_type' => self::TYPE_VIEW,
        ]));
    }

    /**
     * Create a click event
     */
    public static function click(string $elementName, array $data = []): self
    {
        return static::create(array_merge($data, [
            'event_name' => 'click_' . Str::snake($elementName),
            'event_category' => self::CATEGORY_USER_ACTION,
            'event_type' => self::TYPE_CLICK,
            'properties' => array_merge($data['properties'] ?? [], ['element' => $elementName]),
        ]));
    }

    /**
     * Create a conversion event
     */
    public static function conversion(string $conversionType, array $data = []): self
    {
        return static::create(array_merge($data, [
            'event_name' => 'conversion_' . Str::snake($conversionType),
            'event_category' => self::CATEGORY_CONVERSION,
            'event_type' => $conversionType,
            'is_conversion' => true,
        ]));
    }

    /**
     * Create a purchase event
     */
    public static function purchase(float $revenue, array $data = []): self
    {
        return static::create(array_merge($data, [
            'event_name' => 'purchase',
            'event_category' => self::CATEGORY_TRANSACTION,
            'event_type' => self::TYPE_PURCHASE,
            'revenue' => $revenue,
            'event_value' => $revenue,
            'is_conversion' => true,
        ]));
    }

    /**
     * Create a signup event
     */
    public static function signup(array $data = []): self
    {
        return static::create(array_merge($data, [
            'event_name' => 'signup',
            'event_category' => self::CATEGORY_CONVERSION,
            'event_type' => self::TYPE_SIGNUP,
            'is_conversion' => true,
            'journey_stage' => self::STAGE_DECISION,
        ]));
    }

    /**
     * Create a custom event
     */
    public static function track(string $eventName, array $data = []): self
    {
        return static::create(array_merge($data, [
            'event_name' => $eventName,
            'event_category' => $data['event_category'] ?? self::CATEGORY_USER_ACTION,
            'event_type' => $data['event_type'] ?? self::TYPE_VIEW,
        ]));
    }

    // =========================================================================
    // AGGREGATION METHODS
    // =========================================================================

    /**
     * Get event counts grouped by a dimension
     */
    public static function countByDimension(
        string $dimension,
        ?Carbon $startDate = null,
        ?Carbon $endDate = null
    ): Collection {
        $query = static::query()
            ->select($dimension, DB::raw('COUNT(*) as count'))
            ->groupBy($dimension)
            ->orderByDesc('count');

        if ($startDate && $endDate) {
            $query->inDateRange($startDate, $endDate);
        }

        return $query->get();
    }

    /**
     * Get time-series event counts
     */
    public static function timeSeries(
        string $granularity = 'day',
        ?Carbon $startDate = null,
        ?Carbon $endDate = null,
        ?string $eventName = null
    ): Collection {
        $dateFormat = match ($granularity) {
            'hour' => '%Y-%m-%d %H:00:00',
            'day' => '%Y-%m-%d',
            'week' => '%Y-%u',
            'month' => '%Y-%m',
            default => '%Y-%m-%d',
        };

        $query = static::query()
            ->select(
                DB::raw("DATE_FORMAT(event_timestamp, '{$dateFormat}') as period"),
                DB::raw('COUNT(*) as event_count'),
                DB::raw('COUNT(DISTINCT user_id) as unique_users'),
                DB::raw('COUNT(DISTINCT session_id) as sessions'),
                DB::raw('SUM(CASE WHEN is_conversion = 1 THEN 1 ELSE 0 END) as conversions'),
                DB::raw('COALESCE(SUM(revenue), 0) as total_revenue')
            )
            ->groupBy('period')
            ->orderBy('period');

        if ($startDate && $endDate) {
            $query->inDateRange($startDate, $endDate);
        }

        if ($eventName) {
            $query->byEventName($eventName);
        }

        return $query->get();
    }

    /**
     * Get conversion funnel metrics
     */
    public static function getFunnelMetrics(array $steps, Carbon $startDate, Carbon $endDate): array
    {
        $results = [];
        $previousCount = null;

        foreach ($steps as $index => $stepEvent) {
            $count = static::query()
                ->byEventName($stepEvent)
                ->inDateRange($startDate, $endDate)
                ->count(DB::raw('DISTINCT COALESCE(user_id, anonymous_id)'));

            $conversionRate = $previousCount !== null && $previousCount > 0
                ? round(($count / $previousCount) * 100, 2)
                : 100;

            $results[] = [
                'step' => $index + 1,
                'event' => $stepEvent,
                'count' => $count,
                'conversion_rate' => $conversionRate,
                'drop_off' => $previousCount !== null ? $previousCount - $count : 0,
                'drop_off_rate' => $previousCount !== null && $previousCount > 0
                    ? round((($previousCount - $count) / $previousCount) * 100, 2)
                    : 0,
            ];

            $previousCount = $count;
        }

        return $results;
    }

    /**
     * Get real-time metrics (last N minutes)
     */
    public static function getRealTimeMetrics(int $minutes = 30): array
    {
        $since = now()->subMinutes($minutes);

        return Cache::remember("analytics:realtime:{$minutes}", 60, function () use ($since) {
            return [
                'active_users' => static::where('event_timestamp', '>=', $since)
                    ->distinct('session_id')
                    ->count('session_id'),
                'events' => static::where('event_timestamp', '>=', $since)->count(),
                'page_views' => static::where('event_timestamp', '>=', $since)
                    ->byCategory(self::CATEGORY_PAGE_VIEW)
                    ->count(),
                'conversions' => static::where('event_timestamp', '>=', $since)
                    ->conversions()
                    ->count(),
                'revenue' => static::where('event_timestamp', '>=', $since)
                    ->sum('revenue') ?? 0,
                'top_pages' => static::where('event_timestamp', '>=', $since)
                    ->byCategory(self::CATEGORY_PAGE_VIEW)
                    ->select('page_path', DB::raw('COUNT(*) as views'))
                    ->groupBy('page_path')
                    ->orderByDesc('views')
                    ->limit(10)
                    ->get(),
                'top_events' => static::where('event_timestamp', '>=', $since)
                    ->select('event_name', DB::raw('COUNT(*) as count'))
                    ->groupBy('event_name')
                    ->orderByDesc('count')
                    ->limit(10)
                    ->get(),
            ];
        });
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /**
     * Detect marketing channel from UTM parameters and referrer
     */
    protected static function detectChannel(self $event): string
    {
        // Check UTM source first
        if ($event->utm_source) {
            $source = strtolower($event->utm_source);

            if (in_array($source, ['google', 'bing', 'yahoo', 'duckduckgo'])) {
                return $event->utm_medium === 'cpc' ? self::CHANNEL_PAID : self::CHANNEL_ORGANIC;
            }

            if (in_array($source, ['facebook', 'instagram', 'twitter', 'linkedin', 'tiktok', 'youtube'])) {
                return self::CHANNEL_SOCIAL;
            }

            if (str_contains($source, 'email') || str_contains($source, 'newsletter')) {
                return self::CHANNEL_EMAIL;
            }

            if (str_contains($source, 'affiliate')) {
                return self::CHANNEL_AFFILIATE;
            }
        }

        // Check UTM medium
        if ($event->utm_medium) {
            $medium = strtolower($event->utm_medium);

            if (in_array($medium, ['cpc', 'ppc', 'paid', 'display', 'banner'])) {
                return self::CHANNEL_PAID;
            }

            if (in_array($medium, ['email', 'newsletter'])) {
                return self::CHANNEL_EMAIL;
            }

            if (in_array($medium, ['social', 'social-media'])) {
                return self::CHANNEL_SOCIAL;
            }

            if ($medium === 'referral') {
                return self::CHANNEL_REFERRAL;
            }

            if ($medium === 'organic') {
                return self::CHANNEL_ORGANIC;
            }
        }

        // Check referrer domain
        if ($event->referrer_domain) {
            $domain = strtolower($event->referrer_domain);

            // Search engines
            $searchEngines = ['google.', 'bing.', 'yahoo.', 'duckduckgo.', 'baidu.', 'yandex.'];
            foreach ($searchEngines as $engine) {
                if (str_contains($domain, $engine)) {
                    return self::CHANNEL_ORGANIC;
                }
            }

            // Social networks
            $socialNetworks = ['facebook.', 'instagram.', 'twitter.', 'linkedin.', 't.co', 'youtube.', 'tiktok.'];
            foreach ($socialNetworks as $network) {
                if (str_contains($domain, $network)) {
                    return self::CHANNEL_SOCIAL;
                }
            }

            // If referrer exists but not identified
            return self::CHANNEL_REFERRAL;
        }

        // No referrer = direct traffic
        return self::CHANNEL_DIRECT;
    }

    /**
     * Get property value from properties JSON
     */
    public function getProperty(string $key, mixed $default = null): mixed
    {
        return data_get($this->properties, $key, $default);
    }

    /**
     * Set property value in properties JSON
     */
    public function setProperty(string $key, mixed $value): self
    {
        $properties = $this->properties ?? [];
        data_set($properties, $key, $value);
        $this->properties = $properties;

        return $this;
    }

    /**
     * Check if event is from mobile device
     */
    public function isMobile(): bool
    {
        return $this->device_type === 'mobile';
    }

    /**
     * Check if event is from returning user
     */
    public function isReturningUser(): bool
    {
        if (!$this->user_id) {
            return false;
        }

        return static::where('user_id', $this->user_id)
            ->where('id', '<', $this->id)
            ->exists();
    }

    /**
     * Get all events in the same session
     */
    public function getSessionEvents(): Collection
    {
        return static::bySession($this->session_id)
            ->orderBy('event_timestamp')
            ->get();
    }

    /**
     * Calculate engagement score based on event properties
     */
    public function calculateEngagementScore(): float
    {
        $score = 0;

        // Base score for event type
        $eventScores = [
            self::TYPE_PURCHASE => 100,
            self::TYPE_SUBSCRIBE => 80,
            self::TYPE_SIGNUP => 70,
            self::TYPE_COMPLETE => 50,
            self::TYPE_SUBMIT => 40,
            self::TYPE_SHARE => 30,
            self::TYPE_CLICK => 10,
            self::TYPE_VIEW => 5,
        ];

        $score += $eventScores[$this->event_type] ?? 5;

        // Time on page bonus
        if ($this->time_on_page > 60) {
            $score += min(20, ($this->time_on_page / 60) * 2);
        }

        // Scroll depth bonus
        if ($this->scroll_depth > 50) {
            $score += ($this->scroll_depth / 100) * 10;
        }

        // Revenue bonus
        if ($this->revenue > 0) {
            $score += min(50, $this->revenue / 10);
        }

        return round(min(100, $score), 2);
    }
}
