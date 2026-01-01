<?php

declare(strict_types=1);

namespace App\Models;

use App\Enums\ConversationPriority;
use App\Enums\ConversationStatus;
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

/**
 * EmailConversation Model
 *
 * Represents a threaded email conversation in the CRM system.
 * Links email messages to contacts and provides conversation management.
 *
 * @property string $id UUID primary key
 * @property int $contact_id Foreign key to contacts
 * @property int|null $assigned_to User assigned to handle conversation
 * @property string $subject Email subject line
 * @property string $thread_id Unique thread identifier
 * @property string|null $mailbox_hash Hash for routing (e.g., reply+{hash}@domain.com)
 * @property ConversationStatus $status Current conversation status
 * @property ConversationPriority $priority Conversation priority level
 * @property string $channel Communication channel
 * @property int $message_count Total messages in conversation
 * @property int $unread_count Unread message count
 * @property Carbon|null $first_message_at Timestamp of first message
 * @property Carbon|null $latest_message_at Timestamp of latest message
 * @property Carbon|null $first_response_at Timestamp of first response
 * @property Carbon|null $resolved_at Resolution timestamp
 * @property int|null $response_time_seconds Time to first response
 * @property array|null $participants List of email participants
 * @property array|null $tags Conversation tags
 * @property array|null $metadata Additional metadata
 * @property bool $is_starred Starred flag
 * @property bool $is_muted Muted notifications flag
 * @property int|null $created_by User who created conversation
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 *
 * @property-read Contact $contact
 * @property-read User|null $assignee
 * @property-read User|null $creator
 * @property-read \Illuminate\Database\Eloquent\Collection<EmailMessage> $messages
 * @property-read EmailMessage|null $latestMessage
 * @property-read \Illuminate\Database\Eloquent\Collection<EmailThreadMapping> $threadMappings
 *
 * @method static Builder|self open()
 * @method static Builder|self pending()
 * @method static Builder|self resolved()
 * @method static Builder|self active()
 * @method static Builder|self forContact(int $contactId)
 * @method static Builder|self assignedTo(int $userId)
 * @method static Builder|self unassigned()
 * @method static Builder|self withUnread()
 * @method static Builder|self priority(ConversationPriority $priority)
 *
 * @version 1.0.0
 */
class EmailConversation extends Model
{
    use HasFactory;
    use HasUuids;
    use SoftDeletes;

    /**
     * The table associated with the model.
     */
    protected $table = 'email_conversations';

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
        'contact_id',
        'assigned_to',
        'subject',
        'thread_id',
        'mailbox_hash',
        'status',
        'priority',
        'channel',
        'message_count',
        'unread_count',
        'first_message_at',
        'latest_message_at',
        'first_response_at',
        'resolved_at',
        'response_time_seconds',
        'participants',
        'tags',
        'metadata',
        'is_starred',
        'is_muted',
        'created_by',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'status' => ConversationStatus::class,
        'priority' => ConversationPriority::class,
        'participants' => 'array',
        'tags' => 'array',
        'metadata' => 'array',
        'is_starred' => 'boolean',
        'is_muted' => 'boolean',
        'message_count' => 'integer',
        'unread_count' => 'integer',
        'response_time_seconds' => 'integer',
        'first_message_at' => 'datetime',
        'latest_message_at' => 'datetime',
        'first_response_at' => 'datetime',
        'resolved_at' => 'datetime',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_unread',
        'sla_status',
        'subject_preview',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'status' => ConversationStatus::OPEN,
        'priority' => ConversationPriority::NORMAL,
        'channel' => 'email',
        'message_count' => 0,
        'unread_count' => 0,
        'is_starred' => false,
        'is_muted' => false,
    ];

    /**
     * Cache configuration.
     */
    protected static array $cacheConfig = [
        'prefix' => 'email_conversation',
        'ttl' => 1800, // 30 minutes
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $conversation): void {
            $conversation->first_message_at = $conversation->first_message_at ?? now();
            $conversation->latest_message_at = $conversation->latest_message_at ?? now();
        });

        static::updating(function (self $conversation): void {
            if ($conversation->isDirty('status')) {
                $conversation->handleStatusChange();
            }
        });

        static::updated(function (self $conversation): void {
            $conversation->clearCache();
            $conversation->updateContactStats();
        });

        static::deleted(function (self $conversation): void {
            $conversation->clearCache();
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the contact that owns this conversation.
     */
    public function contact(): BelongsTo
    {
        return $this->belongsTo(Contact::class);
    }

    /**
     * Get the user assigned to this conversation.
     */
    public function assignee(): BelongsTo
    {
        return $this->belongsTo(User::class, 'assigned_to');
    }

    /**
     * Get the user who created this conversation.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get all messages in this conversation.
     */
    public function messages(): HasMany
    {
        return $this->hasMany(EmailMessage::class, 'conversation_id')
            ->orderBy('created_at', 'asc');
    }

    /**
     * Get the latest message in this conversation.
     */
    public function latestMessage(): HasMany
    {
        return $this->hasMany(EmailMessage::class, 'conversation_id')
            ->latest('created_at')
            ->limit(1);
    }

    /**
     * Get thread mappings for this conversation.
     */
    public function threadMappings(): HasMany
    {
        return $this->hasMany(EmailThreadMapping::class, 'conversation_id');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    /**
     * Scope to open conversations.
     */
    public function scopeOpen(Builder $query): Builder
    {
        return $query->where('status', ConversationStatus::OPEN);
    }

    /**
     * Scope to pending conversations.
     */
    public function scopePending(Builder $query): Builder
    {
        return $query->where('status', ConversationStatus::PENDING);
    }

    /**
     * Scope to resolved conversations.
     */
    public function scopeResolved(Builder $query): Builder
    {
        return $query->where('status', ConversationStatus::RESOLVED);
    }

    /**
     * Scope to active (open or pending) conversations.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->whereIn('status', ConversationStatus::activeStatuses());
    }

    /**
     * Scope to conversations for a specific contact.
     */
    public function scopeForContact(Builder $query, int $contactId): Builder
    {
        return $query->where('contact_id', $contactId);
    }

    /**
     * Scope to conversations assigned to a user.
     */
    public function scopeAssignedTo(Builder $query, int $userId): Builder
    {
        return $query->where('assigned_to', $userId);
    }

    /**
     * Scope to unassigned conversations.
     */
    public function scopeUnassigned(Builder $query): Builder
    {
        return $query->whereNull('assigned_to');
    }

    /**
     * Scope to conversations with unread messages.
     */
    public function scopeWithUnread(Builder $query): Builder
    {
        return $query->where('unread_count', '>', 0);
    }

    /**
     * Scope to conversations with specific priority.
     */
    public function scopePriority(Builder $query, ConversationPriority $priority): Builder
    {
        return $query->where('priority', $priority);
    }

    /**
     * Scope to starred conversations.
     */
    public function scopeStarred(Builder $query): Builder
    {
        return $query->where('is_starred', true);
    }

    /**
     * Scope to spam conversations.
     */
    public function scopeSpam(Builder $query): Builder
    {
        return $query->where('status', ConversationStatus::SPAM);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    /**
     * Check if conversation has unread messages.
     */
    protected function isUnread(): Attribute
    {
        return Attribute::make(
            get: fn (): bool => $this->unread_count > 0
        );
    }

    /**
     * Get SLA status based on response time and priority.
     */
    protected function slaStatus(): Attribute
    {
        return Attribute::make(
            get: function (): string {
                if ($this->status->isClosed()) {
                    return 'completed';
                }

                $slaMinutes = $this->priority->slaMinutes();
                $elapsedMinutes = $this->first_message_at
                    ? $this->first_message_at->diffInMinutes(now())
                    : 0;

                if ($this->first_response_at !== null) {
                    return 'met';
                }

                if ($elapsedMinutes > $slaMinutes) {
                    return 'breached';
                }

                if ($elapsedMinutes > ($slaMinutes * 0.8)) {
                    return 'at_risk';
                }

                return 'on_track';
            }
        );
    }

    /**
     * Get truncated subject for preview.
     */
    protected function subjectPreview(): Attribute
    {
        return Attribute::make(
            get: fn (): string => mb_strlen($this->subject) > 100
                ? mb_substr($this->subject, 0, 100) . '...'
                : $this->subject
        );
    }

    // =========================================================================
    // BUSINESS LOGIC
    // =========================================================================

    /**
     * Add a new message to this conversation.
     */
    public function addMessage(array $attributes): EmailMessage
    {
        $message = $this->messages()->create($attributes);

        $this->increment('message_count');
        $this->update([
            'latest_message_at' => $message->created_at,
        ]);

        if ($message->direction->isFromExternal()) {
            $this->increment('unread_count');

            // Update first response time if this is our response
            if ($this->first_response_at === null && !$message->direction->isFromExternal()) {
                $this->update([
                    'first_response_at' => $message->created_at,
                    'response_time_seconds' => $this->first_message_at
                        ? $this->first_message_at->diffInSeconds($message->created_at)
                        : null,
                ]);
            }
        }

        $this->clearCache();

        return $message;
    }

    /**
     * Mark all messages as read.
     */
    public function markAsRead(): self
    {
        $this->messages()
            ->where('is_read', false)
            ->update([
                'is_read' => true,
                'read_at' => now(),
            ]);

        $this->update(['unread_count' => 0]);
        $this->clearCache();

        return $this;
    }

    /**
     * Assign conversation to a user.
     */
    public function assignTo(?int $userId): self
    {
        $this->update([
            'assigned_to' => $userId,
            'status' => $userId ? ConversationStatus::PENDING : ConversationStatus::OPEN,
        ]);

        return $this;
    }

    /**
     * Resolve the conversation.
     */
    public function resolve(): self
    {
        $this->update([
            'status' => ConversationStatus::RESOLVED,
            'resolved_at' => now(),
        ]);

        return $this;
    }

    /**
     * Reopen the conversation.
     */
    public function reopen(): self
    {
        $this->update([
            'status' => $this->assigned_to ? ConversationStatus::PENDING : ConversationStatus::OPEN,
            'resolved_at' => null,
        ]);

        return $this;
    }

    /**
     * Mark conversation as spam.
     */
    public function markAsSpam(): self
    {
        $this->update(['status' => ConversationStatus::SPAM]);

        return $this;
    }

    /**
     * Archive the conversation.
     */
    public function archive(): self
    {
        $this->update(['status' => ConversationStatus::ARCHIVED]);

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
     * Toggle muted status.
     */
    public function toggleMute(): self
    {
        $this->update(['is_muted' => !$this->is_muted]);

        return $this;
    }

    /**
     * Add a participant to the conversation.
     */
    public function addParticipant(string $email, string $name = null): self
    {
        $participants = $this->participants ?? [];

        if (!collect($participants)->contains('email', $email)) {
            $participants[] = [
                'email' => $email,
                'name' => $name,
                'added_at' => now()->toIso8601String(),
            ];

            $this->update(['participants' => $participants]);
        }

        return $this;
    }

    /**
     * Add tags to the conversation.
     */
    public function addTags(array $newTags): self
    {
        $tags = array_unique(array_merge($this->tags ?? [], $newTags));
        $this->update(['tags' => array_values($tags)]);

        return $this;
    }

    /**
     * Remove a tag from the conversation.
     */
    public function removeTag(string $tag): self
    {
        $tags = array_filter($this->tags ?? [], fn ($t) => $t !== $tag);
        $this->update(['tags' => array_values($tags)]);

        return $this;
    }

    /**
     * Update conversation priority.
     */
    public function setPriority(ConversationPriority $priority): self
    {
        $this->update(['priority' => $priority]);

        return $this;
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /**
     * Handle status change side effects.
     */
    protected function handleStatusChange(): void
    {
        $newStatus = $this->status;

        if ($newStatus === ConversationStatus::RESOLVED) {
            $this->resolved_at = $this->resolved_at ?? now();
        }

        if ($newStatus === ConversationStatus::SPAM) {
            // Update contact's spam flag
            $this->contact?->update([
                'metadata' => array_merge(
                    $this->contact->metadata ?? [],
                    ['has_spam_conversations' => true]
                ),
            ]);
        }
    }

    /**
     * Update contact statistics.
     */
    protected function updateContactStats(): void
    {
        if ($this->contact) {
            $this->contact->update([
                'conversations_count' => $this->contact->conversations()->count(),
                'last_activity_at' => now(),
            ]);
        }
    }

    /**
     * Clear conversation cache.
     */
    protected function clearCache(): void
    {
        $prefix = self::$cacheConfig['prefix'];

        Cache::forget("{$prefix}:{$this->id}");
        Cache::forget("{$prefix}:contact:{$this->contact_id}");

        if ($this->assigned_to) {
            Cache::forget("{$prefix}:user:{$this->assigned_to}");
        }
    }

    /**
     * Get conversation statistics.
     */
    public function getStatistics(): array
    {
        return Cache::remember(
            self::$cacheConfig['prefix'] . ":{$this->id}:stats",
            self::$cacheConfig['ttl'],
            fn () => [
                'message_count' => $this->message_count,
                'unread_count' => $this->unread_count,
                'inbound_count' => $this->messages()->where('direction', 'inbound')->count(),
                'outbound_count' => $this->messages()->where('direction', 'outbound')->count(),
                'attachment_count' => $this->messages()
                    ->sum('attachment_count'),
                'first_message_at' => $this->first_message_at?->toIso8601String(),
                'latest_message_at' => $this->latest_message_at?->toIso8601String(),
                'first_response_at' => $this->first_response_at?->toIso8601String(),
                'response_time_seconds' => $this->response_time_seconds,
                'duration_hours' => $this->first_message_at
                    ? $this->first_message_at->diffInHours($this->latest_message_at ?? now())
                    : 0,
                'sla_status' => $this->sla_status,
            ]
        );
    }

    /**
     * Find conversation by thread ID or create new one.
     */
    public static function findOrCreateByThreadId(
        string $threadId,
        int $contactId,
        array $attributes = []
    ): self {
        return static::firstOrCreate(
            ['thread_id' => $threadId],
            array_merge([
                'contact_id' => $contactId,
                'status' => ConversationStatus::OPEN,
                'priority' => ConversationPriority::NORMAL,
            ], $attributes)
        );
    }
}
