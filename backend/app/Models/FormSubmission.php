<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Str;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;

/**
 * FormSubmission Model - Enterprise-grade form submission management
 * 
 * Handles form submissions with comprehensive status tracking, metadata management,
 * and relationship handling. Includes soft deletes for data retention compliance.
 *
 * @property int $id
 * @property int $form_id
 * @property int|null $user_id
 * @property string $submission_id UUID identifier
 * @property string $status Current submission status
 * @property string|null $ip_address Submitter's IP address
 * @property string|null $user_agent Submitter's user agent
 * @property string|null $referrer HTTP referrer
 * @property array|null $metadata Additional JSON metadata
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read Form $form
 * @property-read User|null $user
 * @property-read Collection<int, FormSubmissionData> $data
 * @property-read int $data_count
 * 
 * @method static Builder unread()
 * @method static Builder read()
 * @method static Builder starred()
 * @method static Builder archived()
 * @method static Builder spam()
 * @method static Builder byStatus(string $status)
 * @method static Builder recent(int $days = 30)
 * @method static Builder forForm(int $formId)
 * @method static Builder forUser(int $userId)
 */
class FormSubmission extends Model
{
    use SoftDeletes, HasFactory;

    /**
     * The table associated with the model.
     */
    protected $table = 'form_submissions';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'form_id',
        'user_id',
        'submission_id',
        'status',
        'ip_address',
        'user_agent',
        'referrer',
        'metadata',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'form_id' => 'integer',
        'user_id' => 'integer',
        'metadata' => 'array',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'status' => self::STATUS_UNREAD,
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<int, string>
     */
    protected $hidden = [
        'ip_address',
        'user_agent',
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        'is_new',
        'status_label',
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const STATUS_UNREAD = 'unread';
    public const STATUS_READ = 'read';
    public const STATUS_STARRED = 'starred';
    public const STATUS_ARCHIVED = 'archived';
    public const STATUS_SPAM = 'spam';

    /**
     * Available submission statuses.
     *
     * @var array<int, string>
     */
    public const STATUSES = [
        self::STATUS_UNREAD,
        self::STATUS_READ,
        self::STATUS_STARRED,
        self::STATUS_ARCHIVED,
        self::STATUS_SPAM,
    ];

    /**
     * Status labels for display.
     *
     * @var array<string, string>
     */
    public const STATUS_LABELS = [
        self::STATUS_UNREAD => 'Unread',
        self::STATUS_READ => 'Read',
        self::STATUS_STARRED => 'Starred',
        self::STATUS_ARCHIVED => 'Archived',
        self::STATUS_SPAM => 'Spam',
    ];

    /**
     * Status colors for UI.
     *
     * @var array<string, string>
     */
    public const STATUS_COLORS = [
        self::STATUS_UNREAD => 'blue',
        self::STATUS_READ => 'gray',
        self::STATUS_STARRED => 'yellow',
        self::STATUS_ARCHIVED => 'green',
        self::STATUS_SPAM => 'red',
    ];

    // =========================================================================
    // BOOT & LIFECYCLE EVENTS
    // =========================================================================

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        // Auto-generate UUID on creation
        static::creating(function (FormSubmission $submission) {
            if (empty($submission->submission_id)) {
                $submission->submission_id = Str::uuid()->toString();
            }

            // Capture request metadata if available
            if (request()) {
                $submission->captureRequestMetadata();
            }
        });

        // Log status changes
        static::updating(function (FormSubmission $submission) {
            if ($submission->isDirty('status')) {
                \Log::info('FormSubmission status changed', [
                    'id' => $submission->id,
                    'old_status' => $submission->getOriginal('status'),
                    'new_status' => $submission->status,
                ]);
            }
        });

        // Cascade delete submission data
        static::deleting(function (FormSubmission $submission) {
            if ($submission->isForceDeleting()) {
                $submission->data()->forceDelete();
            }
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the form that owns the submission.
     */
    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class)
            ->withDefault([
                'name' => 'Unknown Form',
                'active' => false,
            ]);
    }

    /**
     * Get the user that submitted the form.
     * 
     * Returns null for anonymous submissions.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class)
            ->withDefault([
                'name' => 'Anonymous',
                'email' => null,
            ]);
    }

    /**
     * Get the submission data entries.
     */
    public function data(): HasMany
    {
        return $this->hasMany(FormSubmissionData::class, 'submission_id')
            ->orderBy('field_name');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to only include unread submissions.
     */
    public function scopeUnread(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_UNREAD);
    }

    /**
     * Scope a query to only include read submissions.
     */
    public function scopeRead(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_READ);
    }

    /**
     * Scope a query to only include starred submissions.
     */
    public function scopeStarred(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_STARRED);
    }

    /**
     * Scope a query to only include archived submissions.
     */
    public function scopeArchived(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_ARCHIVED);
    }

    /**
     * Scope a query to only include spam submissions.
     */
    public function scopeSpam(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_SPAM);
    }

    /**
     * Scope a query by status.
     */
    public function scopeByStatus(Builder $query, string $status): Builder
    {
        if (!in_array($status, self::STATUSES, true)) {
            throw new \InvalidArgumentException("Invalid status: {$status}");
        }

        return $query->where('status', $status);
    }

    /**
     * Scope a query to recent submissions.
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    /**
     * Scope a query to submissions for a specific form.
     */
    public function scopeForForm(Builder $query, int $formId): Builder
    {
        return $query->where('form_id', $formId);
    }

    /**
     * Scope a query to submissions by a specific user.
     */
    public function scopeForUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope to eager load common relationships.
     */
    public function scopeWithCommonRelations(Builder $query): Builder
    {
        return $query->with(['form:id,name', 'user:id,name,email', 'data']);
    }

    /**
     * Scope to search by submission ID or data values.
     */
    public function scopeSearch(Builder $query, string $searchTerm): Builder
    {
        return $query->where(function (Builder $q) use ($searchTerm) {
            $q->where('submission_id', 'like', "%{$searchTerm}%")
                ->orWhereHas('data', function (Builder $dataQuery) use ($searchTerm) {
                    $dataQuery->where('value', 'like', "%{$searchTerm}%");
                });
        });
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Check if submission is new (created within last 24 hours).
     */
    public function getIsNewAttribute(): bool
    {
        return $this->created_at->diffInHours(now()) < 24;
    }

    /**
     * Get human-readable status label.
     */
    public function getStatusLabelAttribute(): string
    {
        return self::STATUS_LABELS[$this->status] ?? ucfirst($this->status);
    }

    /**
     * Get status color for UI.
     */
    public function getStatusColorAttribute(): string
    {
        return self::STATUS_COLORS[$this->status] ?? 'gray';
    }

    /**
     * Get formatted submission date.
     */
    public function getFormattedDateAttribute(): string
    {
        return $this->created_at->format('M d, Y \a\t g:i A');
    }

    // =========================================================================
    // STATUS MANAGEMENT METHODS
    // =========================================================================

    /**
     * Mark submission as read.
     */
    public function markAsRead(): bool
    {
        if ($this->status === self::STATUS_UNREAD) {
            return $this->updateStatus(self::STATUS_READ);
        }
        return true;
    }

    /**
     * Mark submission as unread.
     */
    public function markAsUnread(): bool
    {
        if ($this->status !== self::STATUS_UNREAD) {
            return $this->updateStatus(self::STATUS_UNREAD);
        }
        return true;
    }

    /**
     * Toggle star status.
     * 
     * If starred, reverts to read. If not starred, marks as starred.
     */
    public function toggleStar(): bool
    {
        $newStatus = $this->status === self::STATUS_STARRED 
            ? self::STATUS_READ 
            : self::STATUS_STARRED;

        return $this->updateStatus($newStatus);
    }

    /**
     * Archive the submission.
     */
    public function archive(): bool
    {
        return $this->updateStatus(self::STATUS_ARCHIVED);
    }

    /**
     * Unarchive the submission.
     */
    public function unarchive(): bool
    {
        if ($this->status === self::STATUS_ARCHIVED) {
            return $this->updateStatus(self::STATUS_READ);
        }
        return true;
    }

    /**
     * Mark submission as spam.
     */
    public function markAsSpam(): bool
    {
        return $this->updateStatus(self::STATUS_SPAM);
    }

    /**
     * Mark submission as not spam.
     */
    public function markAsNotSpam(): bool
    {
        if ($this->status === self::STATUS_SPAM) {
            return $this->updateStatus(self::STATUS_UNREAD);
        }
        return true;
    }

    /**
     * Update submission status with validation.
     */
    protected function updateStatus(string $status): bool
    {
        if (!in_array($status, self::STATUSES, true)) {
            throw new \InvalidArgumentException("Invalid status: {$status}");
        }

        $this->status = $status;
        return $this->save();
    }

    /**
     * Check if submission has specific status.
     */
    public function hasStatus(string $status): bool
    {
        return $this->status === $status;
    }

    /**
     * Check if submission is unread.
     */
    public function isUnread(): bool
    {
        return $this->status === self::STATUS_UNREAD;
    }

    /**
     * Check if submission is starred.
     */
    public function isStarred(): bool
    {
        return $this->status === self::STATUS_STARRED;
    }

    /**
     * Check if submission is archived.
     */
    public function isArchived(): bool
    {
        return $this->status === self::STATUS_ARCHIVED;
    }

    /**
     * Check if submission is spam.
     */
    public function isSpam(): bool
    {
        return $this->status === self::STATUS_SPAM;
    }

    // =========================================================================
    // DATA MANAGEMENT METHODS
    // =========================================================================

    /**
     * Get submission data as key-value array.
     */
    public function getDataArray(): array
    {
        return $this->data->pluck('value', 'field_name')->toArray();
    }

    /**
     * Get specific field value by field name.
     */
    public function getFieldValue(string $fieldName): mixed
    {
        $field = $this->data->firstWhere('field_name', $fieldName);
        return $field?->value;
    }

    /**
     * Check if submission has a specific field.
     */
    public function hasField(string $fieldName): bool
    {
        return $this->data->contains('field_name', $fieldName);
    }

    /**
     * Get all non-empty field values.
     */
    public function getNonEmptyData(): Collection
    {
        return $this->data->filter(function (FormSubmissionData $data) {
            return !$data->isEmpty();
        });
    }

    /**
     * Update or create submission data field.
     */
    public function updateFieldValue(string $fieldName, mixed $value, ?int $fieldId = null): FormSubmissionData
    {
        return $this->data()->updateOrCreate(
            ['field_name' => $fieldName],
            [
                'field_id' => $fieldId,
                'value' => $value,
            ]
        );
    }

    // =========================================================================
    // METADATA MANAGEMENT
    // =========================================================================

    /**
     * Capture request metadata automatically.
     */
    protected function captureRequestMetadata(): void
    {
        if (!request()) {
            return;
        }

        $this->ip_address = $this->ip_address ?? request()->ip();
        $this->user_agent = $this->user_agent ?? request()->userAgent();
        $this->referrer = $this->referrer ?? request()->header('referer');
    }

    /**
     * Add metadata entry.
     */
    public function addMetadata(string $key, mixed $value): bool
    {
        $metadata = $this->metadata ?? [];
        $metadata[$key] = $value;
        $this->metadata = $metadata;
        return $this->save();
    }

    /**
     * Get metadata value by key.
     */
    public function getMetadata(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Remove metadata entry.
     */
    public function removeMetadata(string $key): bool
    {
        $metadata = $this->metadata ?? [];
        unset($metadata[$key]);
        $this->metadata = $metadata;
        return $this->save();
    }

    // =========================================================================
    // UTILITY METHODS
    // =========================================================================

    /**
     * Export submission as array with all data.
     */
    public function toExportArray(): array
    {
        return [
            'submission_id' => $this->submission_id,
            'form_name' => $this->form->name,
            'status' => $this->status_label,
            'submitted_at' => $this->created_at->toIso8601String(),
            'submitter' => $this->user?->name ?? 'Anonymous',
            'data' => $this->getDataArray(),
            'metadata' => $this->metadata,
        ];
    }

    /**
     * Get submission summary for notifications.
     */
    public function getSummary(): array
    {
        return [
            'id' => $this->id,
            'submission_id' => $this->submission_id,
            'form' => $this->form->name,
            'status' => $this->status_label,
            'created_at' => $this->formatted_date,
            'data_count' => $this->data_count ?? $this->data->count(),
        ];
    }

    // =========================================================================
    // STATIC FACTORY METHODS
    // =========================================================================

    /**
     * Create a new submission with data in one transaction.
     */
    public static function createWithData(int $formId, array $fieldsData, ?int $userId = null): self
    {
        return \DB::transaction(function () use ($formId, $fieldsData, $userId) {
            $submission = self::create([
                'form_id' => $formId,
                'user_id' => $userId,
                'status' => self::STATUS_UNREAD,
            ]);

            foreach ($fieldsData as $fieldName => $value) {
                FormSubmissionData::create([
                    'submission_id' => $submission->id,
                    'field_name' => $fieldName,
                    'value' => $value,
                ]);
            }

            return $submission->load('data');
        });
    }

    /**
     * Bulk update status for multiple submissions.
     * 
     * @param array<int> $ids
     */
    public static function bulkUpdateStatus(array $ids, string $status): int
    {
        if (!in_array($status, self::STATUSES, true)) {
            throw new \InvalidArgumentException("Invalid status: {$status}");
        }

        return self::whereIn('id', $ids)->update(['status' => $status]);
    }

    /**
     * Bulk archive submissions.
     * 
     * @param array<int> $ids
     */
    public static function bulkArchive(array $ids): int
    {
        return self::bulkUpdateStatus($ids, self::STATUS_ARCHIVED);
    }

    /**
     * Bulk delete submissions.
     * 
     * @param array<int> $ids
     */
    public static function bulkDelete(array $ids): int
    {
        return self::whereIn('id', $ids)->delete();
    }
}