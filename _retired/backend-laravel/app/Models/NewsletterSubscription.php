<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Hash;
use InvalidArgumentException;

/**
 * Newsletter Subscription Model
 * 
 * Manages newsletter subscriptions with double opt-in, segmentation,
 * engagement tracking, and GDPR compliance features.
 *
 * @property int $id
 * @property string $email Subscriber email address
 * @property string|null $name Subscriber name
 * @property string|null $first_name First name
 * @property string|null $last_name Last name
 * @property string $status Subscription status (pending, active, unsubscribed, bounced, complained)
 * @property string|null $verification_token Email verification token
 * @property string|null $unsubscribe_token Unsubscribe token
 * @property \Illuminate\Support\Carbon|null $verified_at Verification timestamp
 * @property \Illuminate\Support\Carbon|null $subscribed_at Initial subscription timestamp
 * @property \Illuminate\Support\Carbon|null $unsubscribed_at Unsubscription timestamp
 * @property string|null $unsubscribe_reason Reason for unsubscribing
 * @property array $preferences Subscriber preferences
 * @property array $segments Subscriber segments/tags
 * @property array $interests Subscriber interests
 * @property array $custom_fields Custom data fields
 * @property array $metadata Additional subscriber data
 * @property string|null $source Subscription source (website, popup, api, import, etc.)
 * @property string|null $referring_url Referring URL at subscription
 * @property string|null $utm_source UTM source
 * @property string|null $utm_medium UTM medium
 * @property string|null $utm_campaign UTM campaign
 * @property string|null $ip_address Subscriber IP address
 * @property string|null $user_agent Subscriber user agent
 * @property string|null $country Subscriber country
 * @property string|null $city Subscriber city
 * @property string|null $timezone Subscriber timezone
 * @property string|null $language Preferred language
 * @property int $emails_sent Total emails sent
 * @property int $emails_opened Total emails opened
 * @property int $emails_clicked Total emails clicked
 * @property int $emails_bounced Total bounced emails
 * @property int $emails_complained Total spam complaints
 * @property float $open_rate Email open rate percentage
 * @property float $click_rate Email click rate percentage
 * @property float $engagement_score Engagement score (0-100)
 * @property \Illuminate\Support\Carbon|null $last_sent_at Last email sent timestamp
 * @property \Illuminate\Support\Carbon|null $last_opened_at Last email opened timestamp
 * @property \Illuminate\Support\Carbon|null $last_clicked_at Last email clicked timestamp
 * @property bool $is_vip VIP subscriber flag
 * @property bool $is_blocked Whether subscriber is blocked
 * @property string|null $blocked_reason Reason for blocking
 * @property int|null $bounce_count Consecutive bounce count
 * @property int|null $complaint_count Spam complaint count
 * @property \Illuminate\Support\Carbon|null $consent_given_at GDPR consent timestamp
 * @property string|null $consent_ip IP address where consent was given
 * @property string|null $consent_method Consent collection method
 * @property array $consent_data GDPR consent data
 * @property bool $double_opt_in Whether double opt-in is enabled
 * @property string|null $notes Internal notes
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \Illuminate\Database\Eloquent\Collection $campaigns Campaigns sent to subscriber
 * @property-read \Illuminate\Database\Eloquent\Collection $activities Activity history
 */
class NewsletterSubscription extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Subscription statuses
     */
    public const STATUS_PENDING = 'pending';
    public const STATUS_ACTIVE = 'active';
    public const STATUS_UNSUBSCRIBED = 'unsubscribed';
    public const STATUS_BOUNCED = 'bounced';
    public const STATUS_COMPLAINED = 'complained';
    public const STATUS_BLOCKED = 'blocked';
    public const STATUS_INACTIVE = 'inactive';

    /**
     * Valid statuses
     */
    public const VALID_STATUSES = [
        self::STATUS_PENDING,
        self::STATUS_ACTIVE,
        self::STATUS_UNSUBSCRIBED,
        self::STATUS_BOUNCED,
        self::STATUS_COMPLAINED,
        self::STATUS_BLOCKED,
        self::STATUS_INACTIVE,
    ];

    /**
     * Subscription sources
     */
    public const SOURCE_WEBSITE = 'website';
    public const SOURCE_POPUP = 'popup';
    public const SOURCE_API = 'api';
    public const SOURCE_IMPORT = 'import';
    public const SOURCE_MANUAL = 'manual';
    public const SOURCE_CHECKOUT = 'checkout';
    public const SOURCE_LANDING_PAGE = 'landing_page';
    public const SOURCE_WIDGET = 'widget';

    /**
     * Consent methods
     */
    public const CONSENT_CHECKBOX = 'checkbox';
    public const CONSENT_DOUBLE_OPT_IN = 'double_opt_in';
    public const CONSENT_IMPORTED = 'imported';
    public const CONSENT_API = 'api';

    /**
     * Engagement score thresholds
     */
    public const ENGAGEMENT_HIGH = 70.0;
    public const ENGAGEMENT_MEDIUM = 40.0;
    public const ENGAGEMENT_LOW = 0.0;

    /**
     * Bounce threshold before auto-unsubscribe
     */
    public const MAX_BOUNCE_COUNT = 5;

    /**
     * Complaint threshold before blocking
     */
    public const MAX_COMPLAINT_COUNT = 2;

    /**
     * Inactive threshold (days without engagement)
     */
    public const INACTIVE_DAYS = 180;

    protected $fillable = [
        'email',
        'name',
        'first_name',
        'last_name',
        'status',
        'verification_token',
        'unsubscribe_token',
        'verified_at',
        'subscribed_at',
        'unsubscribed_at',
        'unsubscribe_reason',
        'preferences',
        'segments',
        'interests',
        'custom_fields',
        'metadata',
        'source',
        'referring_url',
        'utm_source',
        'utm_medium',
        'utm_campaign',
        'ip_address',
        'user_agent',
        'country',
        'city',
        'timezone',
        'language',
        'emails_sent',
        'emails_opened',
        'emails_clicked',
        'emails_bounced',
        'emails_complained',
        'open_rate',
        'click_rate',
        'engagement_score',
        'last_sent_at',
        'last_opened_at',
        'last_clicked_at',
        'is_vip',
        'is_blocked',
        'blocked_reason',
        'bounce_count',
        'complaint_count',
        'consent_given_at',
        'consent_ip',
        'consent_method',
        'consent_data',
        'double_opt_in',
        'notes',
    ];

    protected $casts = [
        'preferences' => 'array',
        'segments' => 'array',
        'interests' => 'array',
        'custom_fields' => 'array',
        'metadata' => 'array',
        'consent_data' => 'array',
        'emails_sent' => 'integer',
        'emails_opened' => 'integer',
        'emails_clicked' => 'integer',
        'emails_bounced' => 'integer',
        'emails_complained' => 'integer',
        'bounce_count' => 'integer',
        'complaint_count' => 'integer',
        'open_rate' => 'decimal:2',
        'click_rate' => 'decimal:2',
        'engagement_score' => 'decimal:2',
        'is_vip' => 'boolean',
        'is_blocked' => 'boolean',
        'double_opt_in' => 'boolean',
        'verified_at' => 'datetime',
        'subscribed_at' => 'datetime',
        'unsubscribed_at' => 'datetime',
        'last_sent_at' => 'datetime',
        'last_opened_at' => 'datetime',
        'last_clicked_at' => 'datetime',
        'consent_given_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => self::STATUS_PENDING,
        'double_opt_in' => true,
        'emails_sent' => 0,
        'emails_opened' => 0,
        'emails_clicked' => 0,
        'emails_bounced' => 0,
        'emails_complained' => 0,
        'bounce_count' => 0,
        'complaint_count' => 0,
        'open_rate' => 0.0,
        'click_rate' => 0.0,
        'engagement_score' => 0.0,
        'is_vip' => false,
        'is_blocked' => false,
        'preferences' => '[]',
        'segments' => '[]',
        'interests' => '[]',
        'custom_fields' => '[]',
        'metadata' => '[]',
        'consent_data' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->normalizeEmail();
            $model->extractNameParts();
            $model->generateTokens();
            $model->captureConsentData();
            $model->detectLocation();
            
            if (!$model->subscribed_at) {
                $model->subscribed_at = now();
            }
        });

        static::updating(function (self $model): void {
            $model->normalizeEmail();
            
            if ($model->isDirty(['emails_sent', 'emails_opened', 'emails_clicked'])) {
                $model->calculateEngagementMetrics();
            }

            if ($model->isDirty('status')) {
                $model->handleStatusChange();
            }

            if ($model->isDirty('bounce_count') && $model->bounce_count >= self::MAX_BOUNCE_COUNT) {
                $model->autoUnsubscribeDueToBounces();
            }

            if ($model->isDirty('complaint_count') && $model->complaint_count >= self::MAX_COMPLAINT_COUNT) {
                $model->autoBlockDueToComplaints();
            }
        });

        static::saved(function (self $model): void {
            $model->clearSubscriberCache();
        });

        static::deleted(function (self $model): void {
            $model->clearSubscriberCache();
        });
    }

    /**
     * Get campaigns sent to subscriber
     */
    public function campaigns(): BelongsToMany
    {
        return $this->belongsToMany(NewsletterCampaign::class, 'campaign_subscribers')
            ->withPivot(['sent_at', 'opened_at', 'clicked_at', 'bounced_at'])
            ->withTimestamps();
    }

    /**
     * Get subscriber activities
     */
    public function activities(): HasMany
    {
        return $this->hasMany(NewsletterActivity::class, 'subscription_id');
    }

    /**
     * Normalize email address
     */
    protected function normalizeEmail(): void
    {
        $this->email = strtolower(trim($this->email));
    }

    /**
     * Extract name parts
     */
    protected function extractNameParts(): void
    {
        if ($this->name && (!$this->first_name || !$this->last_name)) {
            $parts = explode(' ', $this->name, 2);
            $this->first_name = $this->first_name ?? $parts[0];
            $this->last_name = $this->last_name ?? ($parts[1] ?? '');
        }
    }

    /**
     * Generate verification and unsubscribe tokens
     */
    protected function generateTokens(): void
    {
        if (!$this->verification_token) {
            $this->verification_token = Str::random(64);
        }

        if (!$this->unsubscribe_token) {
            $this->unsubscribe_token = Str::random(64);
        }
    }

    /**
     * Capture consent data for GDPR
     */
    protected function captureConsentData(): void
    {
        if (!$this->consent_given_at) {
            $this->consent_given_at = now();
            $this->consent_ip = request()->ip();
            $this->consent_method = $this->double_opt_in 
                ? self::CONSENT_DOUBLE_OPT_IN 
                : self::CONSENT_CHECKBOX;

            $this->consent_data = [
                'timestamp' => now()->toIso8601String(),
                'ip' => request()->ip(),
                'user_agent' => request()->userAgent(),
                'method' => $this->consent_method,
                'url' => request()->fullUrl(),
            ];
        }
    }

    /**
     * Detect location from IP
     */
    protected function detectLocation(): void
    {
        if (!$this->ip_address) {
            $this->ip_address = request()->ip();
        }

        if (!$this->user_agent) {
            $this->user_agent = request()->userAgent();
        }

        // Implement with IP geolocation service
        // $location = app(GeoLocationService::class)->lookup($this->ip_address);
        // $this->country = $location['country'] ?? null;
        // $this->city = $location['city'] ?? null;
    }

    /**
     * Calculate engagement metrics
     */
    protected function calculateEngagementMetrics(): void
    {
        // Calculate open rate
        if ($this->emails_sent > 0) {
            $this->open_rate = round(($this->emails_opened / $this->emails_sent) * 100, 2);
        }

        // Calculate click rate
        if ($this->emails_sent > 0) {
            $this->click_rate = round(($this->emails_clicked / $this->emails_sent) * 100, 2);
        }

        // Calculate engagement score
        $this->calculateEngagementScore();
    }

    /**
     * Calculate engagement score (0-100)
     */
    protected function calculateEngagementScore(): void
    {
        if ($this->emails_sent === 0) {
            $this->engagement_score = 0.0;
            return;
        }

        $weights = [
            'open_rate' => 0.4,
            'click_rate' => 0.5,
            'recency' => 0.1,
        ];

        // Open score (0-100)
        $openScore = min($this->open_rate, 100);

        // Click score (clicks are more valuable, so scale differently)
        $clickScore = min($this->click_rate * 2, 100);

        // Recency score (days since last engagement)
        $daysSinceLastEngagement = $this->last_opened_at 
            ? now()->diffInDays($this->last_opened_at) 
            : 999;
        $recencyScore = max(100 - ($daysSinceLastEngagement / self::INACTIVE_DAYS * 100), 0);

        $score = (
            ($openScore * $weights['open_rate']) +
            ($clickScore * $weights['click_rate']) +
            ($recencyScore * $weights['recency'])
        );

        $this->engagement_score = round($score, 2);
    }

    /**
     * Handle status changes
     */
    protected function handleStatusChange(): void
    {
        $newStatus = $this->status;
        $oldStatus = $this->getOriginal('status');

        if ($newStatus === self::STATUS_ACTIVE && $oldStatus === self::STATUS_PENDING) {
            $this->verified_at = now();
        }

        if ($newStatus === self::STATUS_UNSUBSCRIBED && !$this->unsubscribed_at) {
            $this->unsubscribed_at = now();
        }
    }

    /**
     * Auto-unsubscribe due to excessive bounces
     */
    protected function autoUnsubscribeDueToBounces(): void
    {
        if ($this->status !== self::STATUS_UNSUBSCRIBED) {
            $this->status = self::STATUS_BOUNCED;
            $this->unsubscribed_at = now();
            $this->unsubscribe_reason = 'Automatically unsubscribed due to excessive bounce rate';
        }
    }

    /**
     * Auto-block due to spam complaints
     */
    protected function autoBlockDueToComplaints(): void
    {
        $this->is_blocked = true;
        $this->blocked_reason = 'Automatically blocked due to spam complaints';
        $this->status = self::STATUS_COMPLAINED;
    }

    /**
     * Clear subscriber cache
     */
    protected function clearSubscriberCache(): void
    {
        Cache::tags(['newsletter'])->flush();
        Cache::forget("subscriber:email:{$this->email}");
        Cache::forget("subscriber:id:{$this->id}");
    }

    /**
     * Verify email address
     */
    public function verify(): self
    {
        $this->update([
            'status' => self::STATUS_ACTIVE,
            'verified_at' => now(),
            'verification_token' => null,
        ]);

        $this->logActivity('email_verified');

        return $this;
    }

    /**
     * Unsubscribe
     */
    public function unsubscribe(?string $reason = null): self
    {
        $this->update([
            'status' => self::STATUS_UNSUBSCRIBED,
            'unsubscribed_at' => now(),
            'unsubscribe_reason' => $reason,
        ]);

        $this->logActivity('unsubscribed', ['reason' => $reason]);

        return $this;
    }

    /**
     * Resubscribe
     */
    public function resubscribe(): self
    {
        if ($this->is_blocked) {
            throw new InvalidArgumentException('Blocked subscribers cannot resubscribe');
        }

        $this->update([
            'status' => self::STATUS_ACTIVE,
            'unsubscribed_at' => null,
            'unsubscribe_reason' => null,
            'subscribed_at' => now(),
        ]);

        $this->logActivity('resubscribed');

        return $this;
    }

    /**
     * Block subscriber
     */
    public function block(?string $reason = null): self
    {
        $this->update([
            'is_blocked' => true,
            'blocked_reason' => $reason,
            'status' => self::STATUS_BLOCKED,
        ]);

        $this->logActivity('blocked', ['reason' => $reason]);

        return $this;
    }

    /**
     * Unblock subscriber
     */
    public function unblock(): self
    {
        $this->update([
            'is_blocked' => false,
            'blocked_reason' => null,
            'status' => self::STATUS_ACTIVE,
        ]);

        $this->logActivity('unblocked');

        return $this;
    }

    /**
     * Record email sent
     */
    public function recordSent(): self
    {
        $this->increment('emails_sent');
        $this->update(['last_sent_at' => now()]);
        $this->logActivity('email_sent');

        return $this;
    }

    /**
     * Record email opened
     */
    public function recordOpened(): self
    {
        $this->increment('emails_opened');
        $this->update(['last_opened_at' => now()]);
        $this->logActivity('email_opened');

        return $this;
    }

    /**
     * Record email clicked
     */
    public function recordClicked(?string $url = null): self
    {
        $this->increment('emails_clicked');
        $this->update(['last_clicked_at' => now()]);
        $this->logActivity('email_clicked', ['url' => $url]);

        return $this;
    }

    /**
     * Record bounce
     */
    public function recordBounce(string $type = 'hard', ?string $reason = null): self
    {
        $this->increment('emails_bounced');
        $this->increment('bounce_count');
        
        $this->logActivity('email_bounced', [
            'type' => $type,
            'reason' => $reason,
        ]);

        if ($type === 'hard') {
            $this->update(['status' => self::STATUS_BOUNCED]);
        }

        return $this;
    }

    /**
     * Record spam complaint
     */
    public function recordComplaint(?string $reason = null): self
    {
        $this->increment('emails_complained');
        $this->increment('complaint_count');
        
        $this->logActivity('spam_complaint', ['reason' => $reason]);

        return $this;
    }

    /**
     * Add to segment
     */
    public function addToSegment(string $segment): self
    {
        $segments = $this->segments;
        
        if (!in_array($segment, $segments, true)) {
            $segments[] = $segment;
            $this->segments = $segments;
            $this->save();
        }

        return $this;
    }

    /**
     * Remove from segment
     */
    public function removeFromSegment(string $segment): self
    {
        $segments = array_filter($this->segments, fn($s) => $s !== $segment);
        $this->segments = array_values($segments);
        $this->save();

        return $this;
    }

    /**
     * Check if in segment
     */
    public function isInSegment(string $segment): bool
    {
        return in_array($segment, $this->segments, true);
    }

    /**
     * Update preference
     */
    public function updatePreference(string $key, mixed $value): self
    {
        $preferences = $this->preferences;
        $preferences[$key] = $value;
        $this->preferences = $preferences;
        $this->save();

        return $this;
    }

    /**
     * Get preference
     */
    public function getPreference(string $key, mixed $default = null): mixed
    {
        return $this->preferences[$key] ?? $default;
    }

    /**
     * Log activity
     */
    protected function logActivity(string $type, array $data = []): void
    {
        $this->activities()->create([
            'type' => $type,
            'data' => $data,
            'ip_address' => request()->ip(),
            'user_agent' => request()->userAgent(),
        ]);
    }

    /**
     * Check if verified
     */
    public function isVerified(): bool
    {
        return $this->verified_at !== null;
    }

    /**
     * Check if active
     */
    public function isActive(): bool
    {
        return $this->status === self::STATUS_ACTIVE 
            && !$this->is_blocked 
            && $this->isVerified();
    }

    /**
     * Check if can receive emails
     */
    public function canReceiveEmails(): bool
    {
        return $this->isActive() && !$this->isInactive();
    }

    /**
     * Check if inactive (no engagement for long period)
     */
    public function isInactive(): bool
    {
        if (!$this->last_opened_at && !$this->last_clicked_at) {
            return $this->subscribed_at->addDays(self::INACTIVE_DAYS)->isPast();
        }

        $lastEngagement = max(
            $this->last_opened_at ?? $this->subscribed_at,
            $this->last_clicked_at ?? $this->subscribed_at
        );

        return $lastEngagement->addDays(self::INACTIVE_DAYS)->isPast();
    }

    /**
     * Check if highly engaged
     */
    public function isHighlyEngaged(): bool
    {
        return $this->engagement_score >= self::ENGAGEMENT_HIGH;
    }

    /**
     * Get engagement level
     */
    public function getEngagementLevelAttribute(): string
    {
        return match(true) {
            $this->engagement_score >= self::ENGAGEMENT_HIGH => 'high',
            $this->engagement_score >= self::ENGAGEMENT_MEDIUM => 'medium',
            default => 'low',
        };
    }

    /**
     * Get verification URL
     */
    public function getVerificationUrl(): string
    {
        return url("/newsletter/verify/{$this->verification_token}");
    }

    /**
     * Get unsubscribe URL
     */
    public function getUnsubscribeUrl(): string
    {
        return url("/newsletter/unsubscribe/{$this->unsubscribe_token}");
    }

    /**
     * Get status color
     */
    public function getStatusColorAttribute(): string
    {
        return match($this->status) {
            self::STATUS_ACTIVE => '#22c55e', // green-500
            self::STATUS_PENDING => '#f59e0b', // amber-500
            self::STATUS_UNSUBSCRIBED => '#6b7280', // gray-500
            self::STATUS_BOUNCED => '#ef4444', // red-500
            self::STATUS_COMPLAINED => '#dc2626', // red-600
            self::STATUS_BLOCKED => '#991b1b', // red-900
            self::STATUS_INACTIVE => '#9ca3af', // gray-400
            default => '#6b7280',
        };
    }

    /**
     * Scope: Active subscribers
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_ACTIVE)
            ->where('is_blocked', false)
            ->whereNotNull('verified_at');
    }

    /**
     * Scope: Verified subscribers
     */
    public function scopeVerified(Builder $query): Builder
    {
        return $query->whereNotNull('verified_at');
    }

    /**
     * Scope: Pending verification
     */
    public function scopePending(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_PENDING)
            ->whereNull('verified_at');
    }

    /**
     * Scope: Unsubscribed
     */
    public function scopeUnsubscribed(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_UNSUBSCRIBED);
    }

    /**
     * Scope: Blocked
     */
    public function scopeBlocked(Builder $query): Builder
    {
        return $query->where('is_blocked', true);
    }

    /**
     * Scope: In segment
     */
    public function scopeInSegment(Builder $query, string $segment): Builder
    {
        return $query->whereJsonContains('segments', $segment);
    }

    /**
     * Scope: Highly engaged
     */
    public function scopeHighlyEngaged(Builder $query): Builder
    {
        return $query->where('engagement_score', '>=', self::ENGAGEMENT_HIGH);
    }

    /**
     * Scope: Inactive subscribers
     */
    public function scopeInactive(Builder $query): Builder
    {
        return $query->where(function($q) {
            $q->where('last_opened_at', '<', now()->subDays(self::INACTIVE_DAYS))
              ->orWhere(function($sq) {
                  $sq->whereNull('last_opened_at')
                     ->where('subscribed_at', '<', now()->subDays(self::INACTIVE_DAYS));
              });
        });
    }

    /**
     * Scope: By source
     */
    public function scopeFromSource(Builder $query, string $source): Builder
    {
        return $query->where('source', $source);
    }

    /**
     * Scope: Recent subscribers
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('subscribed_at', '>=', now()->subDays($days));
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total' => static::count(),
            'active' => static::active()->count(),
            'pending' => static::pending()->count(),
            'unsubscribed' => static::unsubscribed()->count(),
            'blocked' => static::blocked()->count(),
            'highly_engaged' => static::highlyEngaged()->count(),
            'inactive' => static::inactive()->count(),
            'avg_open_rate' => round(static::where('emails_sent', '>', 0)->avg('open_rate'), 2),
            'avg_click_rate' => round(static::where('emails_sent', '>', 0)->avg('click_rate'), 2),
            'avg_engagement_score' => round(static::avg('engagement_score'), 2),
            'total_emails_sent' => static::sum('emails_sent'),
            'total_opens' => static::sum('emails_opened'),
            'total_clicks' => static::sum('emails_clicked'),
        ];
    }

    /**
     * Static: Get subscribers by segment
     */
    public static function getBySegment(string $segment): Collection
    {
        return static::active()->inSegment($segment)->get();
    }

    /**
     * Static: Clean inactive subscribers
     */
    public static function cleanInactive(int $days = 365, bool $dryRun = true): array
    {
        $inactive = static::where('status', self::STATUS_ACTIVE)
            ->inactive()
            ->where('subscribed_at', '<', now()->subDays($days))
            ->get();

        if ($dryRun) {
            return [
                'would_mark_inactive' => $inactive->count(),
                'emails' => $inactive->pluck('email'),
            ];
        }

        foreach ($inactive as $subscriber) {
            $subscriber->update(['status' => self::STATUS_INACTIVE]);
        }

        return [
            'marked_inactive' => $inactive->count(),
            'emails' => $inactive->pluck('email'),
        ];
    }

    /**
     * Export to array for API
     */
    public function toSubscriberArray(): array
    {
        return [
            'id' => $this->id,
            'email' => $this->email,
            'name' => $this->name,
            'first_name' => $this->first_name,
            'last_name' => $this->last_name,
            'status' => $this->status,
            'status_color' => $this->status_color,
            'is_active' => $this->isActive(),
            'is_verified' => $this->isVerified(),
            'segments' => $this->segments,
            'interests' => $this->interests,
            'source' => $this->source,
            'engagement' => [
                'score' => $this->engagement_score,
                'level' => $this->engagement_level,
                'open_rate' => $this->open_rate,
                'click_rate' => $this->click_rate,
                'emails_sent' => $this->emails_sent,
                'emails_opened' => $this->emails_opened,
                'emails_clicked' => $this->emails_clicked,
            ],
            'subscribed_at' => $this->subscribed_at?->toIso8601String(),
            'verified_at' => $this->verified_at?->toIso8601String(),
            'last_opened_at' => $this->last_opened_at?->toIso8601String(),
        ];
    }
}