<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;

/**
 * Popup Model
 * 
 * Manages marketing popups, modals, and overlays with A/B testing,
 * advanced targeting, conversion tracking, and performance analytics.
 *
 * @property int $id
 * @property string $name Internal popup name
 * @property string $status Popup status (draft, active, paused, archived)
 * @property bool $is_active Whether popup is currently active
 * @property string $type Popup type (modal, slide_in, bar, fullscreen, exit_intent, etc.)
 * @property string|null $template Template name
 * @property int|null $ab_test_id A/B test identifier
 * @property string|null $variant_title A/B test variant name
 * @property int $priority Display priority (higher = shown first)
 * @property array $config General configuration
 * @property array $content Popup content (title, body, CTA, etc.)
 * @property array $attention_animation Animation settings
 * @property array|null $countdown_timer Countdown timer configuration
 * @property array|null $video_embed Embedded video settings
 * @property array $display_rules When/where to show popup
 * @property array $targeting_rules Audience targeting criteria
 * @property array $frequency_rules How often to show popup
 * @property array $form_fields Form field definitions
 * @property array $design Design/style configuration
 * @property array $behavior Behavioral settings (close button, overlay click, etc.)
 * @property array $integration_config Third-party integrations (email, CRM, etc.)
 * @property int $impressions Total impressions count
 * @property int $views Unique views count
 * @property int $conversions Conversion count
 * @property int $closes Close count
 * @property int $form_submissions Form submission count
 * @property float $conversion_rate Conversion rate percentage
 * @property float $close_rate Close rate percentage
 * @property float $avg_time_to_conversion Average time to convert (seconds)
 * @property \Illuminate\Support\Carbon|null $last_impression_at Last impression timestamp
 * @property \Illuminate\Support\Carbon|null $last_conversion_at Last conversion timestamp
 * @property \Illuminate\Support\Carbon|null $active_from Start date for active period
 * @property \Illuminate\Support\Carbon|null $active_until End date for active period
 * @property string|null $success_message Message shown after conversion
 * @property string|null $redirect_url URL to redirect after conversion
 * @property bool $is_test Test mode (doesn't count analytics)
 * @property int|null $max_impressions Maximum impressions limit
 * @property int|null $max_conversions Maximum conversions limit
 * @property string|null $created_by User who created popup
 * @property string|null $updated_by User who last updated popup
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \Illuminate\Database\Eloquent\Collection $interactions User interactions
 * @property-read \App\Models\AbTest|null $abTest A/B test configuration
 */
class Popup extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Popup statuses
     */
    public const STATUS_DRAFT = 'draft';
    public const STATUS_ACTIVE = 'active';
    public const STATUS_PAUSED = 'paused';
    public const STATUS_ARCHIVED = 'archived';
    public const STATUS_SCHEDULED = 'scheduled';
    public const STATUS_EXPIRED = 'expired';

    /**
     * Popup types
     */
    public const TYPE_MODAL = 'modal';
    public const TYPE_SLIDE_IN = 'slide_in';
    public const TYPE_BAR = 'bar';
    public const TYPE_FULLSCREEN = 'fullscreen';
    public const TYPE_EXIT_INTENT = 'exit_intent';
    public const TYPE_INLINE = 'inline';
    public const TYPE_STICKY_BAR = 'sticky_bar';
    public const TYPE_SIDEBAR = 'sidebar';
    public const TYPE_CORNER_POPUP = 'corner_popup';
    public const TYPE_GAMIFIED = 'gamified';

    /**
     * Animation types
     */
    public const ANIMATION_FADE = 'fade';
    public const ANIMATION_SLIDE = 'slide';
    public const ANIMATION_ZOOM = 'zoom';
    public const ANIMATION_BOUNCE = 'bounce';
    public const ANIMATION_SHAKE = 'shake';
    public const ANIMATION_PULSE = 'pulse';
    public const ANIMATION_NONE = 'none';

    /**
     * Trigger types
     */
    public const TRIGGER_TIME_DELAY = 'time_delay';
    public const TRIGGER_SCROLL_DEPTH = 'scroll_depth';
    public const TRIGGER_EXIT_INTENT = 'exit_intent';
    public const TRIGGER_CLICK = 'click';
    public const TRIGGER_HOVER = 'hover';
    public const TRIGGER_IMMEDIATE = 'immediate';
    public const TRIGGER_INACTIVITY = 'inactivity';
    public const TRIGGER_SCROLL_TO_ELEMENT = 'scroll_to_element';

    /**
     * Position options
     */
    public const POSITION_CENTER = 'center';
    public const POSITION_TOP = 'top';
    public const POSITION_BOTTOM = 'bottom';
    public const POSITION_LEFT = 'left';
    public const POSITION_RIGHT = 'right';
    public const POSITION_TOP_LEFT = 'top_left';
    public const POSITION_TOP_RIGHT = 'top_right';
    public const POSITION_BOTTOM_LEFT = 'bottom_left';
    public const POSITION_BOTTOM_RIGHT = 'bottom_right';

    /**
     * Valid statuses
     */
    public const VALID_STATUSES = [
        self::STATUS_DRAFT,
        self::STATUS_ACTIVE,
        self::STATUS_PAUSED,
        self::STATUS_ARCHIVED,
        self::STATUS_SCHEDULED,
        self::STATUS_EXPIRED,
    ];

    /**
     * Valid types
     */
    public const VALID_TYPES = [
        self::TYPE_MODAL,
        self::TYPE_SLIDE_IN,
        self::TYPE_BAR,
        self::TYPE_FULLSCREEN,
        self::TYPE_EXIT_INTENT,
        self::TYPE_INLINE,
        self::TYPE_STICKY_BAR,
        self::TYPE_SIDEBAR,
        self::TYPE_CORNER_POPUP,
        self::TYPE_GAMIFIED,
    ];

    /**
     * Cache TTL
     */
    public const CACHE_TTL = 1800; // 30 minutes

    /**
     * Default conversion rate benchmark
     */
    public const BENCHMARK_CONVERSION_RATE = 5.0;

    protected $fillable = [
        'name',
        'status',
        'is_active',
        'type',
        'template',
        'ab_test_id',
        'variant_title',
        'priority',
        'config',
        'content',
        'attention_animation',
        'countdown_timer',
        'video_embed',
        'display_rules',
        'targeting_rules',
        'frequency_rules',
        'form_fields',
        'design',
        'behavior',
        'integration_config',
        'impressions',
        'views',
        'conversions',
        'closes',
        'form_submissions',
        'conversion_rate',
        'close_rate',
        'avg_time_to_conversion',
        'last_impression_at',
        'last_conversion_at',
        'active_from',
        'active_until',
        'success_message',
        'redirect_url',
        'is_test',
        'max_impressions',
        'max_conversions',
        'created_by',
        'updated_by',
    ];

    protected $casts = [
        'config' => 'array',
        'content' => 'array',
        'attention_animation' => 'array',
        'countdown_timer' => 'array',
        'video_embed' => 'array',
        'display_rules' => 'array',
        'targeting_rules' => 'array',
        'frequency_rules' => 'array',
        'form_fields' => 'array',
        'design' => 'array',
        'behavior' => 'array',
        'integration_config' => 'array',
        'is_active' => 'boolean',
        'is_test' => 'boolean',
        'impressions' => 'integer',
        'views' => 'integer',
        'conversions' => 'integer',
        'closes' => 'integer',
        'form_submissions' => 'integer',
        'max_impressions' => 'integer',
        'max_conversions' => 'integer',
        'priority' => 'integer',
        'conversion_rate' => 'decimal:2',
        'close_rate' => 'decimal:2',
        'avg_time_to_conversion' => 'decimal:2',
        'last_impression_at' => 'datetime',
        'last_conversion_at' => 'datetime',
        'active_from' => 'datetime',
        'active_until' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => self::STATUS_DRAFT,
        'type' => self::TYPE_MODAL,
        'is_active' => false,
        'is_test' => false,
        'priority' => 0,
        'impressions' => 0,
        'views' => 0,
        'conversions' => 0,
        'closes' => 0,
        'form_submissions' => 0,
        'conversion_rate' => 0.0,
        'close_rate' => 0.0,
        'avg_time_to_conversion' => 0.0,
        'config' => '[]',
        'content' => '[]',
        'attention_animation' => '[]',
        'display_rules' => '[]',
        'targeting_rules' => '[]',
        'frequency_rules' => '[]',
        'form_fields' => '[]',
        'design' => '[]',
        'behavior' => '[]',
        'integration_config' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->validateStatus();
            $model->validateType();
            $model->setDefaultDisplayRules();
            $model->setDefaultBehavior();
        });

        static::updating(function (self $model): void {
            $model->validateStatus();
            $model->validateType();
            
            if ($model->isDirty(['impressions', 'views', 'conversions', 'closes'])) {
                $model->calculateMetrics();
            }

            if ($model->isDirty('status') && $model->status === self::STATUS_ACTIVE) {
                $model->is_active = true;
            }

            $model->updated_by = auth()->id();
        });

        static::saved(function (self $model): void {
            $model->clearPopupCache();
        });

        static::deleted(function (self $model): void {
            $model->clearPopupCache();
        });
    }

    /**
     * Get popup interactions
     */
    public function interactions(): HasMany
    {
        return $this->hasMany(PopupInteraction::class);
    }

    /**
     * Get A/B test
     */
    public function abTest(): BelongsTo
    {
        return $this->belongsTo(AbTest::class);
    }

    /**
     * Validate popup status
     */
    protected function validateStatus(): void
    {
        if (!in_array($this->status, self::VALID_STATUSES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid popup status: %s', $this->status)
            );
        }
    }

    /**
     * Validate popup type
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid popup type: %s', $this->type)
            );
        }
    }

    /**
     * Set default display rules
     */
    protected function setDefaultDisplayRules(): void
    {
        if (empty($this->display_rules)) {
            $this->display_rules = [
                'trigger' => self::TRIGGER_TIME_DELAY,
                'delay' => 3000, // 3 seconds
                'pages' => ['all'],
                'devices' => ['desktop', 'mobile', 'tablet'],
            ];
        }
    }

    /**
     * Set default behavior
     */
    protected function setDefaultBehavior(): void
    {
        if (empty($this->behavior)) {
            $this->behavior = [
                'show_close_button' => true,
                'close_on_overlay_click' => true,
                'close_on_esc' => true,
                'prevent_scroll' => true,
                'show_only_once' => false,
                'cookie_lifetime' => 30, // days
            ];
        }
    }

    /**
     * Calculate conversion and close rates
     */
    protected function calculateMetrics(): void
    {
        // Conversion rate
        if ($this->views > 0) {
            $this->conversion_rate = round(($this->conversions / $this->views) * 100, 2);
        }

        // Close rate
        if ($this->views > 0) {
            $this->close_rate = round(($this->closes / $this->views) * 100, 2);
        }
    }

    /**
     * Clear popup cache
     */
    protected function clearPopupCache(): void
    {
        Cache::tags(['popups'])->flush();
        Cache::forget("popup:id:{$this->id}");
        Cache::forget('popups:active');
    }

    /**
     * Check if popup is currently active
     */
    public function isCurrentlyActive(): bool
    {
        if (!$this->is_active || $this->status !== self::STATUS_ACTIVE) {
            return false;
        }

        $now = now();

        if ($this->active_from && $now->isBefore($this->active_from)) {
            return false;
        }

        if ($this->active_until && $now->isAfter($this->active_until)) {
            return false;
        }

        // Check impression limit
        if ($this->max_impressions && $this->impressions >= $this->max_impressions) {
            return false;
        }

        // Check conversion limit
        if ($this->max_conversions && $this->conversions >= $this->max_conversions) {
            return false;
        }

        return true;
    }

    /**
     * Check if popup is scheduled
     */
    public function isScheduled(): bool
    {
        return $this->status === self::STATUS_SCHEDULED
            && $this->active_from
            && $this->active_from->isFuture();
    }

    /**
     * Check if popup is expired
     */
    public function isExpired(): bool
    {
        if ($this->active_until && $this->active_until->isPast()) {
            return true;
        }

        if ($this->max_impressions && $this->impressions >= $this->max_impressions) {
            return true;
        }

        if ($this->max_conversions && $this->conversions >= $this->max_conversions) {
            return true;
        }

        return false;
    }

    /**
     * Check if popup should show for user
     */
    public function shouldShowForUser(?User $user, array $context = []): bool
    {
        if (!$this->isCurrentlyActive()) {
            return false;
        }

        // Check targeting rules
        if (!$this->matchesTargetingRules($user, $context)) {
            return false;
        }

        // Check frequency rules
        if (!$this->matchesFrequencyRules($user)) {
            return false;
        }

        // Check display rules
        if (!$this->matchesDisplayRules($context)) {
            return false;
        }

        return true;
    }

    /**
     * Check if targeting rules match
     */
    protected function matchesTargetingRules(?User $user, array $context): bool
    {
        if (empty($this->targeting_rules)) {
            return true;
        }

        $rules = $this->targeting_rules;

        // User authentication
        if (isset($rules['authenticated'])) {
            if ($rules['authenticated'] && !$user) {
                return false;
            }
            if (!$rules['authenticated'] && $user) {
                return false;
            }
        }

        // User segments
        if (isset($rules['segments']) && $user) {
            if (!in_array($user->segment, $rules['segments'], true)) {
                return false;
            }
        }

        // Geographic targeting
        if (isset($rules['countries']) && !empty($rules['countries'])) {
            $userCountry = $context['country'] ?? null;
            if (!in_array($userCountry, $rules['countries'], true)) {
                return false;
            }
        }

        // Device targeting
        if (isset($rules['devices']) && !empty($rules['devices'])) {
            $device = $context['device'] ?? 'desktop';
            if (!in_array($device, $rules['devices'], true)) {
                return false;
            }
        }

        // Traffic source
        if (isset($rules['traffic_sources']) && !empty($rules['traffic_sources'])) {
            $source = $context['traffic_source'] ?? null;
            if (!in_array($source, $rules['traffic_sources'], true)) {
                return false;
            }
        }

        return true;
    }

    /**
     * Check if frequency rules match
     */
    protected function matchesFrequencyRules(?User $user): bool
    {
        if (empty($this->frequency_rules)) {
            return true;
        }

        $rules = $this->frequency_rules;

        // Show only once
        if ($rules['show_once'] ?? false) {
            if ($this->hasBeenSeenBy($user)) {
                return false;
            }
        }

        // Maximum impressions per session
        if (isset($rules['max_per_session'])) {
            $sessionCount = session("popup:{$this->id}:count", 0);
            if ($sessionCount >= $rules['max_per_session']) {
                return false;
            }
        }

        // Minimum time between displays
        if (isset($rules['min_time_between']) && $user) {
            $lastSeen = Cache::get("popup:{$this->id}:user:{$user->id}:last_seen");
            if ($lastSeen && now()->diffInSeconds($lastSeen) < $rules['min_time_between']) {
                return false;
            }
        }

        return true;
    }

    /**
     * Check if display rules match
     */
    protected function matchesDisplayRules(array $context): bool
    {
        if (empty($this->display_rules)) {
            return true;
        }

        $rules = $this->display_rules;

        // Page targeting
        if (isset($rules['pages']) && !empty($rules['pages'])) {
            $currentPage = $context['page'] ?? null;
            if (!in_array('all', $rules['pages'], true) && !in_array($currentPage, $rules['pages'], true)) {
                return false;
            }
        }

        // URL patterns
        if (isset($rules['url_patterns']) && !empty($rules['url_patterns'])) {
            $currentUrl = $context['url'] ?? '';
            $matches = false;
            foreach ($rules['url_patterns'] as $pattern) {
                if (fnmatch($pattern, $currentUrl)) {
                    $matches = true;
                    break;
                }
            }
            if (!$matches) {
                return false;
            }
        }

        return true;
    }

    /**
     * Check if popup has been seen by user
     */
    protected function hasBeenSeenBy(?User $user): bool
    {
        if (!$user) {
            return (bool) session("popup:{$this->id}:seen", false);
        }

        return Cache::has("popup:{$this->id}:user:{$user->id}:seen");
    }

    /**
     * Record impression
     */
    public function recordImpression(?User $user = null, bool $unique = false): self
    {
        if ($this->is_test) {
            return $this;
        }

        $this->increment('impressions');
        
        if ($unique) {
            $this->increment('views');
        }

        $this->update(['last_impression_at' => now()]);

        // Track in session
        session()->increment("popup:{$this->id}:count");
        session(["popup:{$this->id}:seen" => true]);

        // Track for user
        if ($user) {
            Cache::put("popup:{$this->id}:user:{$user->id}:seen", true, 86400 * 30);
            Cache::put("popup:{$this->id}:user:{$user->id}:last_seen", now(), 86400);
        }

        return $this;
    }

    /**
     * Record conversion
     */
    public function recordConversion(?User $user = null, array $data = [], float $timeToConvert = null): self
    {
        if ($this->is_test) {
            return $this;
        }

        $this->increment('conversions');
        
        $updates = ['last_conversion_at' => now()];

        // Update average time to conversion
        if ($timeToConvert !== null) {
            $totalTime = ($this->avg_time_to_conversion * $this->conversions) + $timeToConvert;
            $updates['avg_time_to_conversion'] = round($totalTime / ($this->conversions + 1), 2);
        }

        $this->update($updates);

        // Log interaction
        $this->interactions()->create([
            'user_id' => $user?->id,
            'type' => 'conversion',
            'data' => $data,
            'time_to_action' => $timeToConvert,
        ]);

        return $this;
    }

    /**
     * Record close
     */
    public function recordClose(?User $user = null, float $timeToClose = null): self
    {
        if ($this->is_test) {
            return $this;
        }

        $this->increment('closes');

        // Log interaction
        $this->interactions()->create([
            'user_id' => $user?->id,
            'type' => 'close',
            'time_to_action' => $timeToClose,
        ]);

        return $this;
    }

    /**
     * Record form submission
     */
    public function recordFormSubmission(?User $user = null, array $formData = []): self
    {
        if ($this->is_test) {
            return $this;
        }

        $this->increment('form_submissions');

        // Log interaction
        $this->interactions()->create([
            'user_id' => $user?->id,
            'type' => 'form_submission',
            'data' => $formData,
        ]);

        return $this;
    }

    /**
     * Activate popup
     */
    public function activate(): self
    {
        $this->update([
            'status' => self::STATUS_ACTIVE,
            'is_active' => true,
        ]);

        return $this;
    }

    /**
     * Pause popup
     */
    public function pause(): self
    {
        $this->update([
            'status' => self::STATUS_PAUSED,
            'is_active' => false,
        ]);

        return $this;
    }

    /**
     * Archive popup
     */
    public function archive(): self
    {
        $this->update([
            'status' => self::STATUS_ARCHIVED,
            'is_active' => false,
        ]);

        return $this;
    }

    /**
     * Schedule popup
     */
    public function schedule(\DateTimeInterface $from, ?\DateTimeInterface $until = null): self
    {
        $this->update([
            'status' => self::STATUS_SCHEDULED,
            'active_from' => $from,
            'active_until' => $until,
        ]);

        return $this;
    }

    /**
     * Duplicate popup
     */
    public function duplicate(string $newName = null): self
    {
        $attributes = $this->toArray();
        
        unset($attributes['id'], $attributes['created_at'], $attributes['updated_at'], $attributes['deleted_at']);
        
        $attributes['name'] = $newName ?? ($this->name . ' (Copy)');
        $attributes['status'] = self::STATUS_DRAFT;
        $attributes['is_active'] = false;
        $attributes['impressions'] = 0;
        $attributes['views'] = 0;
        $attributes['conversions'] = 0;
        $attributes['closes'] = 0;
        $attributes['form_submissions'] = 0;
        $attributes['conversion_rate'] = 0.0;
        $attributes['close_rate'] = 0.0;

        return static::create($attributes);
    }

    /**
     * Reset analytics
     */
    public function resetAnalytics(): self
    {
        $this->update([
            'impressions' => 0,
            'views' => 0,
            'conversions' => 0,
            'closes' => 0,
            'form_submissions' => 0,
            'conversion_rate' => 0.0,
            'close_rate' => 0.0,
            'avg_time_to_conversion' => 0.0,
            'last_impression_at' => null,
            'last_conversion_at' => null,
        ]);

        return $this;
    }

    /**
     * Get performance score (0-100)
     */
    public function getPerformanceScoreAttribute(): float
    {
        $weights = [
            'conversion_rate' => 0.5,
            'engagement' => 0.3,
            'efficiency' => 0.2,
        ];

        // Conversion rate score (benchmark = 5%)
        $conversionScore = min(($this->conversion_rate / self::BENCHMARK_CONVERSION_RATE) * 100, 100);

        // Engagement score (inverse of close rate)
        $engagementScore = max(100 - $this->close_rate, 0);

        // Efficiency score (based on time to conversion)
        $targetTime = 30; // 30 seconds target
        $efficiencyScore = $this->avg_time_to_conversion > 0
            ? max(100 - (($this->avg_time_to_conversion / $targetTime) * 100), 0)
            : 50;

        $score = (
            ($conversionScore * $weights['conversion_rate']) +
            ($engagementScore * $weights['engagement']) +
            ($efficiencyScore * $weights['efficiency'])
        );

        return round($score, 2);
    }

    /**
     * Get health status
     */
    public function getHealthStatusAttribute(): string
    {
        if ($this->views < 100) {
            return 'insufficient_data';
        }

        $score = $this->performance_score;

        return match(true) {
            $score >= 80 => 'excellent',
            $score >= 60 => 'good',
            $score >= 40 => 'fair',
            default => 'poor',
        };
    }

    /**
     * Get status color
     */
    public function getStatusColorAttribute(): string
    {
        return match($this->status) {
            self::STATUS_ACTIVE => '#22c55e', // green-500
            self::STATUS_DRAFT => '#6b7280', // gray-500
            self::STATUS_PAUSED => '#f59e0b', // amber-500
            self::STATUS_SCHEDULED => '#3b82f6', // blue-500
            self::STATUS_EXPIRED => '#ef4444', // red-500
            self::STATUS_ARCHIVED => '#9ca3af', // gray-400
            default => '#6b7280',
        };
    }

    /**
     * Get config value
     */
    public function getConfigValue(string $key, mixed $default = null): mixed
    {
        return $this->config[$key] ?? $default;
    }

    /**
     * Set config value
     */
    public function setConfigValue(string $key, mixed $value): self
    {
        $config = $this->config;
        $config[$key] = $value;
        $this->config = $config;

        return $this;
    }

    /**
     * Scope: Active popups
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_ACTIVE)
            ->where('is_active', true);
    }

    /**
     * Scope: Scheduled popups
     */
    public function scopeScheduled(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_SCHEDULED)
            ->whereNotNull('active_from')
            ->where('active_from', '>', now());
    }

    /**
     * Scope: Ready to activate
     */
    public function scopeReadyToActivate(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_SCHEDULED)
            ->whereNotNull('active_from')
            ->where('active_from', '<=', now());
    }

    /**
     * Scope: Expired popups
     */
    public function scopeExpired(Builder $query): Builder
    {
        return $query->where(function($q) {
            $q->whereNotNull('active_until')
              ->where('active_until', '<=', now())
              ->orWhere(function($sq) {
                  $sq->whereNotNull('max_impressions')
                     ->whereColumn('impressions', '>=', 'max_impressions');
              })
              ->orWhere(function($sq) {
                  $sq->whereNotNull('max_conversions')
                     ->whereColumn('conversions', '>=', 'max_conversions');
              });
        });
    }

    /**
     * Scope: Filter by type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: High performing
     */
    public function scopeHighPerforming(Builder $query, float $minConversionRate = 5.0): Builder
    {
        return $query->where('conversion_rate', '>=', $minConversionRate)
            ->where('views', '>=', 100);
    }

    /**
     * Scope: Low performing
     */
    public function scopeLowPerforming(Builder $query, float $maxConversionRate = 2.0): Builder
    {
        return $query->where('conversion_rate', '<=', $maxConversionRate)
            ->where('views', '>=', 100);
    }

    /**
     * Scope: Order by priority
     */
    public function scopeOrderByPriority(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('priority', $direction);
    }

    /**
     * Scope: Order by performance
     */
    public function scopeOrderByPerformance(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('conversion_rate', $direction);
    }

    /**
     * Static: Get active popups for context
     */
    public static function getActiveForContext(array $context, ?User $user = null): Collection
    {
        return Cache::tags(['popups'])->remember(
            'popups:active:' . md5(json_encode($context)),
            self::CACHE_TTL,
            fn() => static::active()
                ->orderByPriority()
                ->get()
                ->filter(fn($popup) => $popup->shouldShowForUser($user, $context))
        );
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total' => static::count(),
            'active' => static::active()->count(),
            'scheduled' => static::scheduled()->count(),
            'expired' => static::expired()->count(),
            'total_impressions' => static::sum('impressions'),
            'total_views' => static::sum('views'),
            'total_conversions' => static::sum('conversions'),
            'avg_conversion_rate' => round(static::where('views', '>', 0)->avg('conversion_rate'), 2),
            'avg_close_rate' => round(static::where('views', '>', 0)->avg('close_rate'), 2),
            'high_performers' => static::highPerforming()->count(),
            'low_performers' => static::lowPerforming()->count(),
        ];
    }

    /**
     * Static: Get best performing popups
     */
    public static function getBestPerformers(int $limit = 10): Collection
    {
        return static::active()
            ->where('views', '>=', 100)
            ->orderByPerformance()
            ->limit($limit)
            ->get();
    }

    /**
     * Static: Process scheduled popups
     */
    public static function processScheduled(): int
    {
        $popups = static::readyToActivate()->get();
        
        foreach ($popups as $popup) {
            $popup->activate();
        }

        return $popups->count();
    }

    /**
     * Export to array for API
     */
    public function toPopupArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'type' => $this->type,
            'status' => $this->status,
            'status_color' => $this->status_color,
            'is_active' => $this->isCurrentlyActive(),
            'priority' => $this->priority,
            'content' => $this->content,
            'design' => $this->design,
            'behavior' => $this->behavior,
            'animation' => $this->attention_animation,
            'countdown_timer' => $this->countdown_timer,
            'video_embed' => $this->video_embed,
            'form_fields' => $this->form_fields,
            'display_rules' => $this->display_rules,
            'analytics' => [
                'impressions' => $this->impressions,
                'views' => $this->views,
                'conversions' => $this->conversions,
                'conversion_rate' => $this->conversion_rate,
                'close_rate' => $this->close_rate,
                'performance_score' => $this->performance_score,
                'health_status' => $this->health_status,
            ],
            'created_at' => $this->created_at->toIso8601String(),
        ];
    }
}