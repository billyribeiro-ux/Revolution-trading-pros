<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Str;

/**
 * FormDraft Model - FluentForm Pro Save & Resume Feature
 *
 * Allows users to save form progress and resume later.
 * Supports both authenticated and anonymous users.
 *
 * @property int $id
 * @property int $form_id
 * @property int|null $user_id
 * @property string $hash Unique resume token
 * @property string|null $email User email for resume link
 * @property array $form_data Saved form field values
 * @property int $current_step Current step (for multi-step forms)
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property Carbon|null $email_sent_at
 * @property Carbon|null $resumed_at
 * @property Carbon|null $completed_at
 * @property int|null $submission_id Final submission ID
 * @property Carbon|null $expires_at
 * @property Carbon $created_at
 * @property Carbon $updated_at
 */
class FormDraft extends Model
{
    use HasFactory;

    protected $table = 'form_drafts';

    protected $fillable = [
        'form_id',
        'user_id',
        'hash',
        'email',
        'form_data',
        'current_step',
        'ip_address',
        'user_agent',
        'email_sent_at',
        'resumed_at',
        'completed_at',
        'submission_id',
        'expires_at',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'user_id' => 'integer',
        'form_data' => 'encrypted:array',
        'current_step' => 'integer',
        'submission_id' => 'integer',
        'email_sent_at' => 'datetime',
        'resumed_at' => 'datetime',
        'completed_at' => 'datetime',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $hidden = [
        'form_data', // Encrypted, don't expose raw
    ];

    // =========================================================================
    // BOOT
    // =========================================================================

    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (FormDraft $draft) {
            if (empty($draft->hash)) {
                $draft->hash = Str::random(64);
            }
            if (empty($draft->expires_at)) {
                // Default 30 days expiration
                $draft->expires_at = now()->addDays(30);
            }
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function submission(): BelongsTo
    {
        return $this->belongsTo(FormSubmission::class, 'submission_id');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->whereNull('completed_at')
            ->where('expires_at', '>', now());
    }

    public function scopeExpired(Builder $query): Builder
    {
        return $query->where('expires_at', '<=', now());
    }

    public function scopeCompleted(Builder $query): Builder
    {
        return $query->whereNotNull('completed_at');
    }

    public function scopeForUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    public function scopeForEmail(Builder $query, string $email): Builder
    {
        return $query->where('email', $email);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getIsExpiredAttribute(): bool
    {
        return $this->expires_at && $this->expires_at->isPast();
    }

    public function getIsCompletedAttribute(): bool
    {
        return !is_null($this->completed_at);
    }

    public function getResumeUrlAttribute(): string
    {
        return url("/forms/{$this->form->slug}/resume/{$this->hash}");
    }

    public function getProgressPercentageAttribute(): int
    {
        if (!$this->form_data || !$this->form) {
            return 0;
        }

        $totalFields = $this->form->fields()->where('active', true)->count();
        if ($totalFields === 0) {
            return 0;
        }

        $filledFields = count(array_filter($this->form_data, fn($v) => !empty($v)));
        return (int) round(($filledFields / $totalFields) * 100);
    }

    public function getTimeRemainingAttribute(): ?string
    {
        if (!$this->expires_at || $this->is_expired) {
            return null;
        }

        return $this->expires_at->diffForHumans();
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Update saved form data.
     */
    public function saveProgress(array $data, int $step = null): void
    {
        $existingData = $this->form_data ?? [];
        $mergedData = array_merge($existingData, $data);

        $this->update([
            'form_data' => $mergedData,
            'current_step' => $step ?? $this->current_step,
        ]);
    }

    /**
     * Mark draft as resumed.
     */
    public function markResumed(): void
    {
        if (!$this->resumed_at) {
            $this->update(['resumed_at' => now()]);
        }
    }

    /**
     * Mark draft as completed with submission.
     */
    public function markCompleted(int $submissionId): void
    {
        $this->update([
            'completed_at' => now(),
            'submission_id' => $submissionId,
        ]);
    }

    /**
     * Extend expiration date.
     */
    public function extendExpiration(int $days = 30): void
    {
        $this->update([
            'expires_at' => now()->addDays($days),
        ]);
    }

    /**
     * Send resume link email.
     */
    public function sendResumeEmail(): bool
    {
        if (empty($this->email)) {
            return false;
        }

        // In real implementation, dispatch email job
        // Mail::to($this->email)->send(new FormResumeMail($this));

        $this->update(['email_sent_at' => now()]);
        return true;
    }

    /**
     * Get decrypted form data safely.
     */
    public function getFormData(): array
    {
        return $this->form_data ?? [];
    }

    /**
     * Find draft by hash.
     */
    public static function findByHash(string $hash): ?self
    {
        return static::where('hash', $hash)->active()->first();
    }

    /**
     * Find or create draft for user.
     */
    public static function findOrCreateForUser(int $formId, ?int $userId, ?string $email = null, ?string $ip = null): self
    {
        $query = static::where('form_id', $formId)->active();

        if ($userId) {
            $query->where('user_id', $userId);
        } elseif ($email) {
            $query->where('email', $email);
        } else {
            return static::create([
                'form_id' => $formId,
                'user_id' => $userId,
                'email' => $email,
                'ip_address' => $ip,
                'current_step' => 1,
            ]);
        }

        $existing = $query->first();

        if ($existing) {
            return $existing;
        }

        return static::create([
            'form_id' => $formId,
            'user_id' => $userId,
            'email' => $email,
            'ip_address' => $ip,
            'current_step' => 1,
        ]);
    }

    /**
     * Clean up expired drafts.
     */
    public static function cleanupExpired(): int
    {
        return static::expired()->whereNull('completed_at')->delete();
    }

    /**
     * Get partial entries statistics for a form.
     */
    public static function getPartialStats(int $formId): array
    {
        $drafts = static::where('form_id', $formId)->get();

        return [
            'total_drafts' => $drafts->count(),
            'active_drafts' => $drafts->filter(fn($d) => !$d->is_expired && !$d->is_completed)->count(),
            'completed_drafts' => $drafts->whereNotNull('completed_at')->count(),
            'expired_drafts' => $drafts->filter(fn($d) => $d->is_expired)->count(),
            'conversion_rate' => $drafts->count() > 0
                ? round(($drafts->whereNotNull('completed_at')->count() / $drafts->count()) * 100, 2)
                : 0,
            'avg_progress' => $drafts->avg(fn($d) => $d->progress_percentage) ?? 0,
        ];
    }
}
