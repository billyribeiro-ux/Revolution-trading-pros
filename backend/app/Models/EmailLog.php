<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphTo;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Crypt;
use Illuminate\Support\Str;
use Carbon\Carbon;
use App\Traits\HasUuid;
use App\Traits\Encryptable;
use App\Traits\Searchable;
use App\Contracts\Trackable;
use App\Contracts\Analyzable;
use App\Events\EmailSent;
use App\Events\EmailBounced;
use App\Events\EmailOpened;
use App\Events\EmailClicked;
use App\Events\EmailFailed;
use App\Events\EmailComplained;
use App\Events\EmailUnsubscribed;
use App\Enums\EmailStatus;
use App\Enums\EmailType;
use App\Enums\EmailPriority;
use App\Enums\EmailProvider;
use App\Enums\BounceType;
use App\Enums\ComplaintType;
use App\Services\Email\EmailAnalyzer;
use App\Services\Email\SpamScoreCalculator;
use App\Services\Email\DeliverabilityChecker;

/**
 * EmailLog Model
 * 
 * Enterprise-grade email tracking system with comprehensive analytics,
 * deliverability monitoring, engagement tracking, and compliance features.
 * 
 * @property int $id
 * @property string $uuid
 * @property string $message_id
 * @property string $thread_id
 * @property string $email_type
 * @property string $category
 * @property string $template_id
 * @property string $template_version
 * @property string $campaign_id
 * @property string $batch_id
 * @property string $provider
 * @property string $provider_message_id
 * @property array $provider_response
 * @property string $from_email
 * @property string $from_name
 * @property string $reply_to_email
 * @property string $reply_to_name
 * @property string $recipient_email
 * @property string $recipient_name
 * @property string $recipient_type
 * @property int|null $recipient_id
 * @property array|null $cc_recipients
 * @property array|null $bcc_recipients
 * @property string $subject
 * @property string $preheader
 * @property string $body_html
 * @property string $body_text
 * @property string $body_amp
 * @property array|null $headers
 * @property array|null $attachments
 * @property array|null $embedded_images
 * @property array|null $variables
 * @property array|null $personalization
 * @property array|null $tags
 * @property array|null $metadata
 * @property string $status
 * @property string $priority
 * @property int $attempt_count
 * @property int $max_attempts
 * @property Carbon|null $next_retry_at
 * @property string|null $error_code
 * @property string|null $error_message
 * @property array|null $error_details
 * @property Carbon|null $queued_at
 * @property Carbon|null $sent_at
 * @property Carbon|null $delivered_at
 * @property Carbon|null $opened_at
 * @property Carbon|null $clicked_at
 * @property Carbon|null $bounced_at
 * @property Carbon|null $complained_at
 * @property Carbon|null $unsubscribed_at
 * @property Carbon|null $failed_at
 * @property int $open_count
 * @property int $unique_open_count
 * @property int $click_count
 * @property int $unique_click_count
 * @property array|null $clicked_links
 * @property string|null $bounce_type
 * @property string|null $bounce_subtype
 * @property string|null $bounce_reason
 * @property bool $is_hard_bounce
 * @property string|null $complaint_type
 * @property string|null $complaint_feedback
 * @property string|null $unsubscribe_reason
 * @property float|null $spam_score
 * @property array|null $spam_report
 * @property float|null $engagement_score
 * @property float|null $deliverability_score
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property array|null $device_info
 * @property array|null $geo_location
 * @property string|null $client_type
 * @property string|null $client_name
 * @property string|null $client_version
 * @property string|null $platform
 * @property string|null $platform_version
 * @property bool $is_mobile
 * @property bool $is_desktop
 * @property bool $is_tablet
 * @property bool $is_bot
 * @property int|null $email_size_bytes
 * @property int|null $processing_time_ms
 * @property float|null $delivery_time_seconds
 * @property string|null $smtp_response
 * @property array|null $routing_info
 * @property array|null $dns_records
 * @property bool $has_dkim
 * @property bool $has_spf
 * @property bool $has_dmarc
 * @property bool $is_encrypted
 * @property bool $is_authenticated
 * @property string|null $conversation_id
 * @property int|null $conversation_position
 * @property bool $is_reply
 * @property bool $is_forward
 * @property string|null $in_reply_to
 * @property array|null $references
 * @property string|null $list_unsubscribe
 * @property string|null $list_unsubscribe_post
 * @property bool $is_transactional
 * @property bool $is_marketing
 * @property bool $is_automated
 * @property bool $requires_action
 * @property Carbon|null $action_required_by
 * @property string|null $action_taken
 * @property Carbon|null $action_taken_at
 * @property array|null $ab_test
 * @property string|null $ab_test_variant
 * @property array|null $performance_metrics
 * @property array|null $cost_data
 * @property bool $is_test
 * @property bool $is_archived
 * @property string|null $archive_reason
 * @property Carbon|null $archived_at
 * @property int|null $created_by
 * @property string|null $created_by_type
 * @property Carbon|null $scheduled_for
 * @property Carbon|null $expires_at
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\User|null $creator
 * @property-read \Illuminate\Database\Eloquent\Model|null $recipient
 * @property-read \Illuminate\Database\Eloquent\Model|null $relatable
 * @property-read \App\Models\EmailTemplate|null $template
 * @property-read \App\Models\EmailCampaign|null $campaign
 * @property-read \App\Models\EmailBatch|null $batch
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailEvent[] $events
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailLink[] $links
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailAttachment[] $attachmentRecords
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailInteraction[] $interactions
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailLog[] $thread
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailMetric[] $metrics
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Note[] $notes
 */
class EmailLog extends Model implements Trackable, Analyzable
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Encryptable;
    use Searchable;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'email_logs';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'message_id',
        'thread_id',
        'email_type',
        'category',
        'template_id',
        'template_version',
        'campaign_id',
        'batch_id',
        'provider',
        'provider_message_id',
        'provider_response',
        'from_email',
        'from_name',
        'reply_to_email',
        'reply_to_name',
        'recipient_email',
        'recipient_name',
        'recipient_type',
        'recipient_id',
        'cc_recipients',
        'bcc_recipients',
        'subject',
        'preheader',
        'body_html',
        'body_text',
        'body_amp',
        'headers',
        'attachments',
        'embedded_images',
        'variables',
        'personalization',
        'tags',
        'metadata',
        'status',
        'priority',
        'attempt_count',
        'max_attempts',
        'next_retry_at',
        'error_code',
        'error_message',
        'error_details',
        'queued_at',
        'sent_at',
        'delivered_at',
        'opened_at',
        'clicked_at',
        'bounced_at',
        'complained_at',
        'unsubscribed_at',
        'failed_at',
        'open_count',
        'unique_open_count',
        'click_count',
        'unique_click_count',
        'clicked_links',
        'bounce_type',
        'bounce_subtype',
        'bounce_reason',
        'is_hard_bounce',
        'complaint_type',
        'complaint_feedback',
        'unsubscribe_reason',
        'spam_score',
        'spam_report',
        'engagement_score',
        'deliverability_score',
        'ip_address',
        'user_agent',
        'device_info',
        'geo_location',
        'client_type',
        'client_name',
        'client_version',
        'platform',
        'platform_version',
        'is_mobile',
        'is_desktop',
        'is_tablet',
        'is_bot',
        'email_size_bytes',
        'processing_time_ms',
        'delivery_time_seconds',
        'smtp_response',
        'routing_info',
        'dns_records',
        'has_dkim',
        'has_spf',
        'has_dmarc',
        'is_encrypted',
        'is_authenticated',
        'conversation_id',
        'conversation_position',
        'is_reply',
        'is_forward',
        'in_reply_to',
        'references',
        'list_unsubscribe',
        'list_unsubscribe_post',
        'is_transactional',
        'is_marketing',
        'is_automated',
        'requires_action',
        'action_required_by',
        'action_taken',
        'action_taken_at',
        'ab_test',
        'ab_test_variant',
        'performance_metrics',
        'cost_data',
        'is_test',
        'is_archived',
        'archive_reason',
        'archived_at',
        'created_by',
        'created_by_type',
        'scheduled_for',
        'expires_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'email_type' => EmailType::class,
        'status' => EmailStatus::class,
        'priority' => EmailPriority::class,
        'provider' => EmailProvider::class,
        'bounce_type' => BounceType::class,
        'complaint_type' => ComplaintType::class,
        'provider_response' => 'array',
        'cc_recipients' => 'array',
        'bcc_recipients' => 'array',
        'headers' => 'array',
        'attachments' => 'array',
        'embedded_images' => 'array',
        'variables' => 'array',
        'personalization' => 'array',
        'tags' => 'array',
        'metadata' => 'array',
        'error_details' => 'array',
        'clicked_links' => 'array',
        'spam_report' => 'array',
        'device_info' => 'array',
        'geo_location' => 'array',
        'routing_info' => 'array',
        'dns_records' => 'array',
        'references' => 'array',
        'ab_test' => 'array',
        'performance_metrics' => 'array',
        'cost_data' => 'array',
        'attempt_count' => 'integer',
        'max_attempts' => 'integer',
        'open_count' => 'integer',
        'unique_open_count' => 'integer',
        'click_count' => 'integer',
        'unique_click_count' => 'integer',
        'email_size_bytes' => 'integer',
        'processing_time_ms' => 'integer',
        'conversation_position' => 'integer',
        'spam_score' => 'float',
        'engagement_score' => 'float',
        'deliverability_score' => 'float',
        'delivery_time_seconds' => 'float',
        'is_hard_bounce' => 'boolean',
        'is_mobile' => 'boolean',
        'is_desktop' => 'boolean',
        'is_tablet' => 'boolean',
        'is_bot' => 'boolean',
        'has_dkim' => 'boolean',
        'has_spf' => 'boolean',
        'has_dmarc' => 'boolean',
        'is_encrypted' => 'boolean',
        'is_authenticated' => 'boolean',
        'is_reply' => 'boolean',
        'is_forward' => 'boolean',
        'is_transactional' => 'boolean',
        'is_marketing' => 'boolean',
        'is_automated' => 'boolean',
        'requires_action' => 'boolean',
        'is_test' => 'boolean',
        'is_archived' => 'boolean',
        'queued_at' => 'datetime',
        'sent_at' => 'datetime',
        'delivered_at' => 'datetime',
        'opened_at' => 'datetime',
        'clicked_at' => 'datetime',
        'bounced_at' => 'datetime',
        'complained_at' => 'datetime',
        'unsubscribed_at' => 'datetime',
        'failed_at' => 'datetime',
        'next_retry_at' => 'datetime',
        'action_required_by' => 'datetime',
        'action_taken_at' => 'datetime',
        'archived_at' => 'datetime',
        'scheduled_for' => 'datetime',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be encrypted.
     *
     * @var array<string>
     */
    protected $encryptable = [
        'body_html',
        'body_text',
        'body_amp',
        'personalization',
    ];

    /**
     * The attributes that should be searchable.
     *
     * @var array<string>
     */
    protected $searchable = [
        'message_id',
        'recipient_email',
        'from_email',
        'subject',
        'campaign_id',
        'template_id',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'body_html',
        'body_text',
        'body_amp',
        'headers',
        'deleted_at',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_delivered',
        'is_opened',
        'is_clicked',
        'is_bounced',
        'is_complained',
        'is_unsubscribed',
        'delivery_status',
        'engagement_status',
        'time_to_open',
        'time_to_click',
        'success_rate',
        'health_score',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'status' => EmailStatus::PENDING,
        'priority' => EmailPriority::NORMAL,
        'provider' => EmailProvider::DEFAULT,
        'attempt_count' => 0,
        'max_attempts' => 3,
        'open_count' => 0,
        'unique_open_count' => 0,
        'click_count' => 0,
        'unique_click_count' => 0,
        'is_hard_bounce' => false,
        'is_transactional' => false,
        'is_marketing' => false,
        'is_automated' => false,
        'is_test' => false,
        'is_archived' => false,
    ];

    /**
     * Cache configuration.
     */
    protected static array $cacheConfig = [
        'tags' => ['email_logs'],
        'ttl' => 3600,
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $emailLog) {
            $emailLog->uuid = $emailLog->uuid ?? (string) Str::uuid();
            $emailLog->message_id = $emailLog->message_id ?? $emailLog->generateMessageId();
            $emailLog->thread_id = $emailLog->thread_id ?? $emailLog->generateThreadId();
            $emailLog->created_by = $emailLog->created_by ?? auth()->id();
            $emailLog->created_by_type = $emailLog->created_by_type ?? 'user';
            
            $emailLog->calculateEmailSize();
            $emailLog->analyzeContent();
            $emailLog->determineEmailCategory();
        });

        static::created(function (self $emailLog) {
            $emailLog->createInitialEvent('created');
            $emailLog->scheduleDelivery();
        });

        static::updating(function (self $emailLog) {
            if ($emailLog->isDirty('status')) {
                $emailLog->handleStatusChange();
            }

            if ($emailLog->isDirty(['open_count', 'click_count'])) {
                $emailLog->updateEngagementScore();
            }
        });

        static::updated(function (self $emailLog) {
            $emailLog->clearCache();
            
            if ($emailLog->wasChanged('status')) {
                $emailLog->fireStatusEvent();
            }
        });
    }

    /**
     * Get the creator.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get the recipient polymorphic relation.
     */
    public function recipient(): MorphTo
    {
        return $this->morphTo('recipient');
    }

    /**
     * Get the relatable polymorphic relation.
     */
    public function relatable(): MorphTo
    {
        return $this->morphTo();
    }

    /**
     * Get the email template.
     */
    public function template(): BelongsTo
    {
        return $this->belongsTo(EmailTemplate::class, 'template_id');
    }

    /**
     * Get the email campaign.
     */
    public function campaign(): BelongsTo
    {
        return $this->belongsTo(EmailCampaign::class, 'campaign_id');
    }

    /**
     * Get the email batch.
     */
    public function batch(): BelongsTo
    {
        return $this->belongsTo(EmailBatch::class, 'batch_id');
    }

    /**
     * Get email events.
     */
    public function events(): HasMany
    {
        return $this->hasMany(EmailEvent::class)
            ->orderBy('occurred_at', 'desc');
    }

    /**
     * Get tracked links.
     */
    public function links(): HasMany
    {
        return $this->hasMany(EmailLink::class);
    }

    /**
     * Get attachment records.
     */
    public function attachmentRecords(): HasMany
    {
        return $this->hasMany(EmailAttachment::class);
    }

    /**
     * Get interactions.
     */
    public function interactions(): HasMany
    {
        return $this->hasMany(EmailInteraction::class)
            ->orderBy('interacted_at', 'desc');
    }

    /**
     * Get email thread.
     */
    public function thread(): HasMany
    {
        return $this->hasMany(self::class, 'thread_id', 'thread_id')
            ->orderBy('created_at');
    }

    /**
     * Get metrics.
     */
    public function metrics(): HasMany
    {
        return $this->hasMany(EmailMetric::class)
            ->orderBy('recorded_at', 'desc');
    }

    /**
     * Get notes.
     */
    public function notes(): MorphMany
    {
        return $this->morphMany(Note::class, 'notable');
    }

    /**
     * Scope for sent emails.
     */
    public function scopeSent(Builder $query): Builder
    {
        return $query->where('status', EmailStatus::SENT)
            ->whereNotNull('sent_at');
    }

    /**
     * Scope for delivered emails.
     */
    public function scopeDelivered(Builder $query): Builder
    {
        return $query->where('status', EmailStatus::DELIVERED)
            ->whereNotNull('delivered_at');
    }

    /**
     * Scope for opened emails.
     */
    public function scopeOpened(Builder $query): Builder
    {
        return $query->whereNotNull('opened_at')
            ->where('open_count', '>', 0);
    }

    /**
     * Scope for clicked emails.
     */
    public function scopeClicked(Builder $query): Builder
    {
        return $query->whereNotNull('clicked_at')
            ->where('click_count', '>', 0);
    }

    /**
     * Scope for bounced emails.
     */
    public function scopeBounced(Builder $query): Builder
    {
        return $query->where('status', EmailStatus::BOUNCED)
            ->whereNotNull('bounced_at');
    }

    /**
     * Scope for failed emails.
     */
    public function scopeFailed(Builder $query): Builder
    {
        return $query->where('status', EmailStatus::FAILED)
            ->whereNotNull('failed_at');
    }

    /**
     * Scope for transactional emails.
     */
    public function scopeTransactional(Builder $query): Builder
    {
        return $query->where('is_transactional', true);
    }

    /**
     * Scope for marketing emails.
     */
    public function scopeMarketing(Builder $query): Builder
    {
        return $query->where('is_marketing', true);
    }

    /**
     * Scope for emails in date range.
     */
    public function scopeDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('created_at', [$start, $end]);
    }

    /**
     * Get delivery status.
     */
    protected function isDelivered(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->status === EmailStatus::DELIVERED
        );
    }

    /**
     * Get opened status.
     */
    protected function isOpened(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->open_count > 0
        );
    }

    /**
     * Get clicked status.
     */
    protected function isClicked(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->click_count > 0
        );
    }

    /**
     * Get bounced status.
     */
    protected function isBounced(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->status === EmailStatus::BOUNCED
        );
    }

    /**
     * Get complained status.
     */
    protected function isComplained(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->complained_at !== null
        );
    }

    /**
     * Get unsubscribed status.
     */
    protected function isUnsubscribed(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->unsubscribed_at !== null
        );
    }

    /**
     * Get delivery status summary.
     */
    protected function deliveryStatus(): Attribute
    {
        return Attribute::make(
            get: function () {
                return match($this->status) {
                    EmailStatus::DELIVERED => 'Delivered',
                    EmailStatus::BOUNCED => $this->is_hard_bounce ? 'Hard Bounced' : 'Soft Bounced',
                    EmailStatus::FAILED => 'Failed',
                    EmailStatus::SENT => 'Sent',
                    EmailStatus::QUEUED => 'Queued',
                    EmailStatus::PROCESSING => 'Processing',
                    default => 'Pending',
                };
            }
        );
    }

    /**
     * Get engagement status.
     */
    protected function engagementStatus(): Attribute
    {
        return Attribute::make(
            get: function () {
                if ($this->is_unsubscribed) return 'Unsubscribed';
                if ($this->is_complained) return 'Complained';
                if ($this->is_clicked) return 'Clicked';
                if ($this->is_opened) return 'Opened';
                if ($this->is_delivered) return 'Delivered';
                return 'No Engagement';
            }
        );
    }

    /**
     * Get time to open.
     */
    protected function timeToOpen(): Attribute
    {
        return Attribute::make(
            get: function () {
                if (!$this->sent_at || !$this->opened_at) return null;
                return $this->sent_at->diffInSeconds($this->opened_at);
            }
        );
    }

    /**
     * Get time to click.
     */
    protected function timeToClick(): Attribute
    {
        return Attribute::make(
            get: function () {
                if (!$this->opened_at || !$this->clicked_at) return null;
                return $this->opened_at->diffInSeconds($this->clicked_at);
            }
        );
    }

    /**
     * Get success rate.
     */
    protected function successRate(): Attribute
    {
        return Attribute::make(
            get: function () {
                $score = 0;
                if ($this->is_delivered) $score += 25;
                if ($this->is_opened) $score += 25;
                if ($this->is_clicked) $score += 25;
                if (!$this->is_bounced && !$this->is_complained) $score += 25;
                return $score;
            }
        );
    }

    /**
     * Get health score.
     */
    protected function healthScore(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculateHealthScore()
        );
    }

    /**
     * Generate message ID.
     */
    protected function generateMessageId(): string
    {
        return sprintf(
            '<%s.%s@%s>',
            Str::random(16),
            time(),
            parse_url(config('app.url'), PHP_URL_HOST)
        );
    }

    /**
     * Generate thread ID.
     */
    protected function generateThreadId(): string
    {
        if ($this->is_reply && $this->in_reply_to) {
            $parent = self::where('message_id', $this->in_reply_to)->first();
            if ($parent) {
                return $parent->thread_id;
            }
        }
        
        return (string) Str::uuid();
    }

    /**
     * Calculate email size.
     */
    protected function calculateEmailSize(): void
    {
        $size = 0;
        $size += strlen($this->subject ?? '');
        $size += strlen($this->body_html ?? '');
        $size += strlen($this->body_text ?? '');
        $size += strlen($this->body_amp ?? '');
        
        if ($this->attachments) {
            foreach ($this->attachments as $attachment) {
                $size += $attachment['size'] ?? 0;
            }
        }
        
        $this->email_size_bytes = $size;
    }

    /**
     * Analyze content for spam and quality.
     */
    protected function analyzeContent(): void
    {
        $analyzer = app(SpamScoreCalculator::class);
        
        $this->spam_score = $analyzer->calculate($this);
        $this->spam_report = $analyzer->getReport();
        
        // Check for spam triggers
        if ($this->spam_score > 5) {
            $this->status = EmailStatus::HELD;
            $this->metadata = array_merge($this->metadata ?? [], [
                'held_reason' => 'High spam score',
            ]);
        }
    }

    /**
     * Determine email category.
     */
    protected function determineEmailCategory(): void
    {
        if (!$this->category) {
            $this->category = match($this->email_type) {
                EmailType::WELCOME => 'onboarding',
                EmailType::RESET_PASSWORD => 'security',
                EmailType::INVOICE => 'billing',
                EmailType::NEWSLETTER => 'marketing',
                EmailType::NOTIFICATION => 'updates',
                default => 'general',
            };
        }
    }

    /**
     * Handle status change.
     */
    protected function handleStatusChange(): void
    {
        $oldStatus = $this->getOriginal('status');
        $newStatus = $this->status;
        
        // Update timestamps based on status
        match($newStatus) {
            EmailStatus::SENT => $this->sent_at = $this->sent_at ?? now(),
            EmailStatus::DELIVERED => $this->delivered_at = $this->delivered_at ?? now(),
            EmailStatus::BOUNCED => $this->bounced_at = $this->bounced_at ?? now(),
            EmailStatus::FAILED => $this->failed_at = $this->failed_at ?? now(),
            default => null,
        };
        
        // Record status transition
        $this->recordEvent('status_changed', [
            'from' => $oldStatus,
            'to' => $newStatus,
        ]);
    }

    /**
     * Fire status event.
     */
    protected function fireStatusEvent(): void
    {
        match($this->status) {
            EmailStatus::SENT => event(new EmailSent($this)),
            EmailStatus::BOUNCED => event(new EmailBounced($this)),
            EmailStatus::FAILED => event(new EmailFailed($this)),
            default => null,
        };
    }

    /**
     * Record email event.
     */
    public function recordEvent(string $type, array $data = []): EmailEvent
    {
        return $this->events()->create([
            'type' => $type,
            'data' => $data,
            'occurred_at' => now(),
            'ip_address' => request()->ip(),
            'user_agent' => request()->userAgent(),
        ]);
    }

    /**
     * Record open event.
     */
    public function recordOpen(array $data = []): void
    {
        $this->increment('open_count');
        
        if (!$this->opened_at) {
            $this->opened_at = now();
            $this->increment('unique_open_count');
        }
        
        $this->recordEvent('opened', $data);
        event(new EmailOpened($this));
    }

    /**
     * Record click event.
     */
    public function recordClick(string $url, array $data = []): void
    {
        $this->increment('click_count');
        
        if (!$this->clicked_at) {
            $this->clicked_at = now();
            $this->increment('unique_click_count');
        }
        
        $clicks = $this->clicked_links ?? [];
        $clicks[] = [
            'url' => $url,
            'clicked_at' => now(),
            'data' => $data,
        ];
        $this->clicked_links = $clicks;
        
        $this->save();
        
        $this->recordEvent('clicked', array_merge($data, ['url' => $url]));
        event(new EmailClicked($this, $url));
    }

    /**
     * Record bounce.
     */
    public function recordBounce(string $type, string $reason, array $data = []): void
    {
        $this->update([
            'status' => EmailStatus::BOUNCED,
            'bounce_type' => $type,
            'bounce_reason' => $reason,
            'is_hard_bounce' => $type === BounceType::HARD,
            'bounced_at' => now(),
        ]);
        
        $this->recordEvent('bounced', array_merge($data, [
            'type' => $type,
            'reason' => $reason,
        ]));
    }

    /**
     * Record complaint.
     */
    public function recordComplaint(string $type, string $feedback = null): void
    {
        $this->update([
            'complaint_type' => $type,
            'complaint_feedback' => $feedback,
            'complained_at' => now(),
        ]);
        
        $this->recordEvent('complained', [
            'type' => $type,
            'feedback' => $feedback,
        ]);
        
        event(new EmailComplained($this));
    }

    /**
     * Record unsubscribe.
     */
    public function recordUnsubscribe(string $reason = null): void
    {
        $this->update([
            'unsubscribe_reason' => $reason,
            'unsubscribed_at' => now(),
        ]);
        
        $this->recordEvent('unsubscribed', ['reason' => $reason]);
        
        event(new EmailUnsubscribed($this));
    }

    /**
     * Calculate health score.
     */
    protected function calculateHealthScore(): float
    {
        $score = 100;
        
        // Deduct for bounces
        if ($this->is_bounced) {
            $score -= $this->is_hard_bounce ? 50 : 20;
        }
        
        // Deduct for complaints
        if ($this->is_complained) {
            $score -= 40;
        }
        
        // Deduct for unsubscribes
        if ($this->is_unsubscribed) {
            $score -= 20;
        }
        
        // Add for engagement
        if ($this->is_opened) $score += 10;
        if ($this->is_clicked) $score += 20;
        
        // Deduct for high spam score
        if ($this->spam_score > 5) {
            $score -= ($this->spam_score - 5) * 5;
        }
        
        return max(0, min(100, $score));
    }

    /**
     * Update engagement score.
     */
    protected function updateEngagementScore(): void
    {
        $score = 0;
        
        // Base score from opens
        if ($this->unique_open_count > 0) {
            $score += 30;
            $score += min(20, $this->open_count * 2); // Multiple opens
        }
        
        // Score from clicks
        if ($this->unique_click_count > 0) {
            $score += 40;
            $score += min(10, $this->click_count); // Multiple clicks
        }
        
        // Time-based scoring
        if ($this->time_to_open && $this->time_to_open < 3600) {
            $score += 10; // Quick open
        }
        
        $this->engagement_score = min(100, $score);
    }

    /**
     * Schedule delivery.
     */
    protected function scheduleDelivery(): void
    {
        if ($this->scheduled_for && $this->scheduled_for->isFuture()) {
            dispatch(new SendEmailJob($this))->delay($this->scheduled_for);
        } elseif ($this->priority === EmailPriority::HIGH) {
            dispatch(new SendEmailJob($this))->onQueue('high');
        } else {
            dispatch(new SendEmailJob($this));
        }
    }

    /**
     * Retry sending.
     */
    public function retry(): void
    {
        if ($this->attempt_count >= $this->max_attempts) {
            $this->update(['status' => EmailStatus::FAILED]);
            return;
        }
        
        $this->increment('attempt_count');
        $this->update([
            'status' => EmailStatus::QUEUED,
            'next_retry_at' => now()->addMinutes(5 * $this->attempt_count),
        ]);
        
        $this->scheduleDelivery();
    }

    /**
     * Get analytics summary.
     */
    public function getAnalytics(): array
    {
        return Cache::tags(self::$cacheConfig['tags'])
            ->remember(
                "email_{$this->id}_analytics",
                self::$cacheConfig['ttl'],
                fn () => $this->calculateAnalytics()
            );
    }

    /**
     * Calculate analytics.
     */
    protected function calculateAnalytics(): array
    {
        $analyzer = app(EmailAnalyzer::class);
        return $analyzer->analyze($this);
    }

    /**
     * Clear cache.
     */
    protected function clearCache(): void
    {
        Cache::tags(self::$cacheConfig['tags'])
            ->forget("email_{$this->id}_analytics");
    }

    /**
     * Create initial event.
     */
    protected function createInitialEvent(string $type): void
    {
        $this->recordEvent($type, [
            'created_by' => $this->created_by,
            'template_id' => $this->template_id,
            'campaign_id' => $this->campaign_id,
        ]);
    }
}