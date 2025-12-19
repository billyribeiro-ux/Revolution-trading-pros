<?php

declare(strict_types=1);

namespace App\Models;

use App\Enums\AttachmentScanStatus;
use App\Enums\EmailDirection;
use App\Enums\EmailMessageStatus;
use App\Enums\SecurityVerdict;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Carbon;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * EmailMessage Model
 *
 * Represents an individual email message (inbound or outbound) within a conversation.
 * Handles spam analysis, security verification, and attachment tracking.
 *
 * @property string $id UUID primary key
 * @property string $conversation_id Foreign key to email_conversations
 * @property string $message_id RFC 2822 Message-ID header
 * @property string|null $provider_message_id Provider-specific message ID
 * @property string|null $in_reply_to In-Reply-To header value
 * @property array|null $references References header values
 * @property string $from_email Sender email address
 * @property string|null $from_name Sender display name
 * @property string $to_email Recipient email address
 * @property string|null $to_name Recipient display name
 * @property array|null $cc CC recipients
 * @property array|null $bcc BCC recipients
 * @property string|null $reply_to_email Reply-To email address
 * @property string|null $reply_to_name Reply-To display name
 * @property string $subject Email subject
 * @property string|null $body_text Plain text body
 * @property string|null $body_html HTML body
 * @property string|null $body_preview First 500 characters preview
 * @property EmailDirection $direction Inbound or outbound
 * @property EmailMessageStatus $status Processing status
 * @property bool $is_read Read status
 * @property bool $is_starred Starred status
 * @property bool $is_draft Draft status
 * @property bool $is_automated Automated email flag
 * @property float|null $spam_score Spam score (0-10)
 * @property SecurityVerdict|null $spam_verdict Spam check result
 * @property SecurityVerdict|null $virus_verdict Virus check result
 * @property SecurityVerdict|null $spf_verdict SPF check result
 * @property SecurityVerdict|null $dkim_verdict DKIM check result
 * @property SecurityVerdict|null $dmarc_verdict DMARC check result
 * @property array|null $authentication_results Full auth results
 * @property array|null $spam_report Spam analysis report
 * @property array|null $headers Raw email headers
 * @property int|null $size_bytes Email size in bytes
 * @property int $attachment_count Number of attachments
 * @property string|null $content_type Email content type
 * @property string|null $charset Email charset
 * @property string|null $ip_address Sender IP address
 * @property string|null $user_agent Mail client user agent
 * @property array|null $geo_location Sender geolocation data
 * @property string $provider Email provider (postmark, ses, etc.)
 * @property array|null $provider_data Provider-specific data
 * @property int|null $processing_time_ms Processing time in milliseconds
 * @property string|null $error_code Error code if failed
 * @property string|null $error_message Error message if failed
 * @property int $retry_count Number of processing retries
 * @property Carbon|null $processed_at When processing completed
 * @property Carbon|null $sent_at When email was sent
 * @property Carbon|null $received_at When email was received
 * @property Carbon|null $read_at When email was read
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 *
 * @property-read EmailConversation $conversation
 * @property-read Contact|null $contact
 * @property-read \Illuminate\Database\Eloquent\Collection<EmailAttachment> $attachments
 *
 * @method static Builder|self inbound()
 * @method static Builder|self outbound()
 * @method static Builder|self unread()
 * @method static Builder|self processed()
 * @method static Builder|self failed()
 * @method static Builder|self spam()
 * @method static Builder|self clean()
 *
 * @version 1.0.0
 */
class EmailMessage extends Model
{
    use HasFactory;
    use HasUuids;
    use SoftDeletes;

    /**
     * The table associated with the model.
     */
    protected $table = 'email_messages';

    /**
     * The primary key type.
     */
    protected $keyType = 'string';

    /**
     * Indicates if the IDs are auto-incrementing.
     */
    public $incrementing = false;

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'conversation_id',
        'message_id',
        'provider_message_id',
        'in_reply_to',
        'references',
        'from_email',
        'from_name',
        'to_email',
        'to_name',
        'cc',
        'bcc',
        'reply_to_email',
        'reply_to_name',
        'subject',
        'body_text',
        'body_html',
        'body_preview',
        'direction',
        'status',
        'is_read',
        'is_starred',
        'is_draft',
        'is_automated',
        'spam_score',
        'spam_verdict',
        'virus_verdict',
        'spf_verdict',
        'dkim_verdict',
        'dmarc_verdict',
        'authentication_results',
        'spam_report',
        'headers',
        'size_bytes',
        'attachment_count',
        'content_type',
        'charset',
        'ip_address',
        'user_agent',
        'geo_location',
        'provider',
        'provider_data',
        'processing_time_ms',
        'error_code',
        'error_message',
        'retry_count',
        'processed_at',
        'sent_at',
        'received_at',
        'read_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'direction' => EmailDirection::class,
        'status' => EmailMessageStatus::class,
        'spam_verdict' => SecurityVerdict::class,
        'virus_verdict' => SecurityVerdict::class,
        'spf_verdict' => SecurityVerdict::class,
        'dkim_verdict' => SecurityVerdict::class,
        'dmarc_verdict' => SecurityVerdict::class,
        'references' => 'array',
        'cc' => 'array',
        'bcc' => 'array',
        'authentication_results' => 'array',
        'spam_report' => 'array',
        'headers' => 'array',
        'geo_location' => 'array',
        'provider_data' => 'array',
        'is_read' => 'boolean',
        'is_starred' => 'boolean',
        'is_draft' => 'boolean',
        'is_automated' => 'boolean',
        'spam_score' => 'float',
        'size_bytes' => 'integer',
        'attachment_count' => 'integer',
        'processing_time_ms' => 'integer',
        'retry_count' => 'integer',
        'processed_at' => 'datetime',
        'sent_at' => 'datetime',
        'received_at' => 'datetime',
        'read_at' => 'datetime',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_secure',
        'security_score',
        'from_display',
        'to_display',
    ];

    /**
     * The attributes that should be hidden.
     *
     * @var array<string>
     */
    protected $hidden = [
        'body_html',
        'body_text',
        'headers',
        'provider_data',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'direction' => EmailDirection::INBOUND,
        'status' => EmailMessageStatus::PENDING,
        'is_read' => false,
        'is_starred' => false,
        'is_draft' => false,
        'is_automated' => false,
        'attachment_count' => 0,
        'retry_count' => 0,
        'provider' => 'postmark',
    ];

    /**
     * Maximum spam score before auto-marking as spam.
     */
    protected const SPAM_THRESHOLD = 5.0;

    /**
     * Maximum retry attempts for failed messages.
     */
    protected const MAX_RETRIES = 3;

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $message): void {
            $message->message_id = $message->message_id ?? $message->generateMessageId();
            $message->received_at = $message->received_at ?? now();
            $message->body_preview = $message->body_preview ?? $message->generateBodyPreview();
        });

        static::created(function (self $message): void {
            $message->updateConversationTimestamps();
        });

        static::updating(function (self $message): void {
            if ($message->isDirty('is_read') && $message->is_read) {
                $message->read_at = $message->read_at ?? now();
            }
        });

        static::updated(function (self $message): void {
            if ($message->wasChanged('is_read')) {
                $message->updateConversationUnreadCount();
            }
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the conversation this message belongs to.
     */
    public function conversation(): BelongsTo
    {
        return $this->belongsTo(EmailConversation::class);
    }

    /**
     * Get the contact associated with this message.
     */
    public function contact(): BelongsTo
    {
        return $this->hasOneThrough(
            Contact::class,
            EmailConversation::class,
            'id',
            'id',
            'conversation_id',
            'contact_id'
        );
    }

    /**
     * Get all attachments for this message.
     */
    public function attachments(): HasMany
    {
        return $this->hasMany(EmailAttachment::class, 'message_id');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    /**
     * Scope to inbound messages.
     */
    public function scopeInbound(Builder $query): Builder
    {
        return $query->where('direction', EmailDirection::INBOUND);
    }

    /**
     * Scope to outbound messages.
     */
    public function scopeOutbound(Builder $query): Builder
    {
        return $query->where('direction', EmailDirection::OUTBOUND);
    }

    /**
     * Scope to unread messages.
     */
    public function scopeUnread(Builder $query): Builder
    {
        return $query->where('is_read', false);
    }

    /**
     * Scope to read messages.
     */
    public function scopeRead(Builder $query): Builder
    {
        return $query->where('is_read', true);
    }

    /**
     * Scope to processed messages.
     */
    public function scopeProcessed(Builder $query): Builder
    {
        return $query->where('status', EmailMessageStatus::PROCESSED);
    }

    /**
     * Scope to failed messages.
     */
    public function scopeFailed(Builder $query): Builder
    {
        return $query->where('status', EmailMessageStatus::FAILED);
    }

    /**
     * Scope to pending messages.
     */
    public function scopePending(Builder $query): Builder
    {
        return $query->where('status', EmailMessageStatus::PENDING);
    }

    /**
     * Scope to spam messages.
     */
    public function scopeSpam(Builder $query): Builder
    {
        return $query->where('status', EmailMessageStatus::SPAM);
    }

    /**
     * Scope to clean (non-spam) messages.
     */
    public function scopeClean(Builder $query): Builder
    {
        return $query->where('status', '!=', EmailMessageStatus::SPAM)
            ->where(function (Builder $q): void {
                $q->whereNull('spam_score')
                    ->orWhere('spam_score', '<', self::SPAM_THRESHOLD);
            });
    }

    /**
     * Scope to messages with attachments.
     */
    public function scopeWithAttachments(Builder $query): Builder
    {
        return $query->where('attachment_count', '>', 0);
    }

    /**
     * Scope to starred messages.
     */
    public function scopeStarred(Builder $query): Builder
    {
        return $query->where('is_starred', true);
    }

    /**
     * Scope to retryable failed messages.
     */
    public function scopeRetryable(Builder $query): Builder
    {
        return $query->where('status', EmailMessageStatus::FAILED)
            ->where('retry_count', '<', self::MAX_RETRIES);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    /**
     * Check if message passed all security checks.
     */
    protected function isSecure(): Attribute
    {
        return Attribute::make(
            get: function (): bool {
                // Check spam verdict
                if ($this->spam_verdict === SecurityVerdict::FAIL) {
                    return false;
                }

                // Check virus verdict
                if ($this->virus_verdict === SecurityVerdict::FAIL) {
                    return false;
                }

                // At least SPF or DKIM should pass
                $spfPass = $this->spf_verdict === SecurityVerdict::PASS;
                $dkimPass = $this->dkim_verdict === SecurityVerdict::PASS;

                return $spfPass || $dkimPass;
            }
        );
    }

    /**
     * Calculate security score (0-100).
     */
    protected function securityScore(): Attribute
    {
        return Attribute::make(
            get: function (): int {
                $score = 50; // Base score

                // SPF contribution (max 15 points)
                $score += match ($this->spf_verdict) {
                    SecurityVerdict::PASS => 15,
                    SecurityVerdict::NEUTRAL => 5,
                    SecurityVerdict::FAIL => -15,
                    default => 0,
                };

                // DKIM contribution (max 15 points)
                $score += match ($this->dkim_verdict) {
                    SecurityVerdict::PASS => 15,
                    SecurityVerdict::NEUTRAL => 5,
                    SecurityVerdict::FAIL => -15,
                    default => 0,
                };

                // DMARC contribution (max 10 points)
                $score += match ($this->dmarc_verdict) {
                    SecurityVerdict::PASS => 10,
                    SecurityVerdict::NEUTRAL => 3,
                    SecurityVerdict::FAIL => -10,
                    default => 0,
                };

                // Spam score contribution (max 10 points)
                if ($this->spam_score !== null) {
                    $spamDeduction = min(10, (int) ($this->spam_score * 2));
                    $score -= $spamDeduction;
                }

                // Virus check (critical)
                if ($this->virus_verdict === SecurityVerdict::FAIL) {
                    $score -= 50;
                }

                return max(0, min(100, $score));
            }
        );
    }

    /**
     * Get formatted from display string.
     */
    protected function fromDisplay(): Attribute
    {
        return Attribute::make(
            get: fn (): string => $this->from_name
                ? "{$this->from_name} <{$this->from_email}>"
                : $this->from_email
        );
    }

    /**
     * Get formatted to display string.
     */
    protected function toDisplay(): Attribute
    {
        return Attribute::make(
            get: fn (): string => $this->to_name
                ? "{$this->to_name} <{$this->to_email}>"
                : $this->to_email
        );
    }

    // =========================================================================
    // BUSINESS LOGIC
    // =========================================================================

    /**
     * Mark message as read.
     */
    public function markAsRead(): self
    {
        if (!$this->is_read) {
            $this->update([
                'is_read' => true,
                'read_at' => now(),
            ]);
        }

        return $this;
    }

    /**
     * Mark message as unread.
     */
    public function markAsUnread(): self
    {
        $this->update([
            'is_read' => false,
            'read_at' => null,
        ]);

        return $this;
    }

    /**
     * Toggle starred status.
     */
    public function toggleStar(): self
    {
        $this->update(['is_starred' => !$this->is_starred]);

        return $this;
    }

    /**
     * Mark message as spam.
     */
    public function markAsSpam(): self
    {
        $this->update(['status' => EmailMessageStatus::SPAM]);

        return $this;
    }

    /**
     * Mark message as not spam.
     */
    public function markAsNotSpam(): self
    {
        $this->update(['status' => EmailMessageStatus::PROCESSED]);

        return $this;
    }

    /**
     * Mark message as processed.
     */
    public function markAsProcessed(): self
    {
        $this->update([
            'status' => EmailMessageStatus::PROCESSED,
            'processed_at' => now(),
        ]);

        return $this;
    }

    /**
     * Mark message as failed.
     */
    public function markAsFailed(string $errorCode, string $errorMessage): self
    {
        $this->update([
            'status' => EmailMessageStatus::FAILED,
            'error_code' => $errorCode,
            'error_message' => $errorMessage,
        ]);

        return $this;
    }

    /**
     * Retry processing failed message.
     */
    public function retry(): bool
    {
        if ($this->retry_count >= self::MAX_RETRIES) {
            return false;
        }

        $this->update([
            'status' => EmailMessageStatus::PENDING,
            'retry_count' => $this->retry_count + 1,
            'error_code' => null,
            'error_message' => null,
        ]);

        return true;
    }

    /**
     * Set spam analysis results.
     */
    public function setSpamAnalysis(float $score, array $report = []): self
    {
        $verdict = $score >= self::SPAM_THRESHOLD
            ? SecurityVerdict::FAIL
            : SecurityVerdict::PASS;

        $this->update([
            'spam_score' => $score,
            'spam_verdict' => $verdict,
            'spam_report' => $report,
        ]);

        // Auto-mark as spam if score exceeds threshold
        if ($score >= self::SPAM_THRESHOLD) {
            $this->markAsSpam();
        }

        return $this;
    }

    /**
     * Set authentication results.
     */
    public function setAuthenticationResults(array $results): self
    {
        $this->update([
            'spf_verdict' => SecurityVerdict::tryFrom($results['spf'] ?? 'none'),
            'dkim_verdict' => SecurityVerdict::tryFrom($results['dkim'] ?? 'none'),
            'dmarc_verdict' => SecurityVerdict::tryFrom($results['dmarc'] ?? 'none'),
            'authentication_results' => $results,
        ]);

        return $this;
    }

    /**
     * Check if message is spam.
     */
    public function isSpam(): bool
    {
        return $this->status === EmailMessageStatus::SPAM
            || ($this->spam_score !== null && $this->spam_score >= self::SPAM_THRESHOLD);
    }

    /**
     * Check if message can be retried.
     */
    public function canRetry(): bool
    {
        return $this->status === EmailMessageStatus::FAILED
            && $this->retry_count < self::MAX_RETRIES;
    }

    /**
     * Get sanitized HTML body.
     */
    public function getSanitizedHtml(): string
    {
        if (empty($this->body_html)) {
            return nl2br(e($this->body_text ?? ''));
        }

        // Use HTMLPurifier or similar for production
        // This is a simplified version
        $html = $this->body_html;

        // Remove potentially dangerous tags
        $dangerousTags = ['script', 'iframe', 'object', 'embed', 'form', 'input', 'button'];
        foreach ($dangerousTags as $tag) {
            $html = preg_replace("/<{$tag}[^>]*>.*?<\/{$tag}>/is", '', $html);
            $html = preg_replace("/<{$tag}[^>]*>/is", '', $html);
        }

        // Remove event handlers
        $html = preg_replace('/\s+on\w+\s*=\s*["\'][^"\']*["\']/i', '', $html);

        // Remove javascript: URLs
        $html = preg_replace('/href\s*=\s*["\']javascript:[^"\']*["\']/i', 'href="#"', $html);

        return $html;
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /**
     * Generate RFC 2822 compliant Message-ID.
     */
    protected function generateMessageId(): string
    {
        $domain = parse_url(config('app.url'), PHP_URL_HOST) ?? 'localhost';
        $unique = Str::random(32);
        $timestamp = now()->timestamp;

        return "<{$unique}.{$timestamp}@{$domain}>";
    }

    /**
     * Generate body preview from text or HTML body.
     */
    protected function generateBodyPreview(): ?string
    {
        $text = $this->body_text;

        if (empty($text) && !empty($this->body_html)) {
            // Strip HTML tags and decode entities
            $text = html_entity_decode(strip_tags($this->body_html));
        }

        if (empty($text)) {
            return null;
        }

        // Normalize whitespace
        $text = preg_replace('/\s+/', ' ', $text);
        $text = trim($text);

        // Truncate to 500 characters
        if (mb_strlen($text) > 500) {
            $text = mb_substr($text, 0, 497) . '...';
        }

        return $text;
    }

    /**
     * Update conversation timestamps when message is created.
     */
    protected function updateConversationTimestamps(): void
    {
        if ($this->conversation) {
            $updates = [
                'latest_message_at' => $this->created_at,
            ];

            // Set first_message_at if this is the first message
            if ($this->conversation->message_count === 0) {
                $updates['first_message_at'] = $this->created_at;
            }

            // Increment message count
            $this->conversation->increment('message_count');

            // Increment unread count for inbound messages
            if ($this->direction === EmailDirection::INBOUND && !$this->is_read) {
                $this->conversation->increment('unread_count');
            }

            $this->conversation->update($updates);
        }
    }

    /**
     * Update conversation unread count when read status changes.
     */
    protected function updateConversationUnreadCount(): void
    {
        if ($this->conversation && $this->direction === EmailDirection::INBOUND) {
            $unreadCount = $this->conversation->messages()
                ->inbound()
                ->unread()
                ->count();

            $this->conversation->update(['unread_count' => $unreadCount]);
        }
    }

    /**
     * Get header value by name.
     */
    public function getHeader(string $name): ?string
    {
        if (empty($this->headers)) {
            return null;
        }

        $normalizedName = strtolower($name);

        foreach ($this->headers as $header) {
            if (is_array($header) && isset($header['Name'], $header['Value'])) {
                if (strtolower($header['Name']) === $normalizedName) {
                    return $header['Value'];
                }
            }
        }

        return null;
    }

    /**
     * Extract all email addresses from headers.
     */
    public function getAllRecipients(): array
    {
        $recipients = [$this->to_email];

        if (!empty($this->cc)) {
            foreach ($this->cc as $cc) {
                $recipients[] = is_array($cc) ? ($cc['email'] ?? $cc['Email'] ?? null) : $cc;
            }
        }

        if (!empty($this->bcc)) {
            foreach ($this->bcc as $bcc) {
                $recipients[] = is_array($bcc) ? ($bcc['email'] ?? $bcc['Email'] ?? null) : $bcc;
            }
        }

        return array_filter(array_unique($recipients));
    }
}
