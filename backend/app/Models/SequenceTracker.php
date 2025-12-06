<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * Sequence Tracker Model (Subscriber progress through sequences)
 *
 * @property string $id
 * @property string $contact_id
 * @property string $sequence_id
 * @property string|null $last_sequence_mail_id
 * @property string|null $next_sequence_mail_id
 * @property string $status
 * @property Carbon $started_at
 * @property Carbon|null $last_executed_at
 * @property Carbon|null $next_execution_at
 * @property Carbon|null $completed_at
 * @property array|null $notes
 * @property int $emails_sent
 * @property int $emails_opened
 * @property int $emails_clicked
 */
class SequenceTracker extends Model
{
    use HasUuids;

    protected $fillable = [
        'contact_id',
        'sequence_id',
        'last_sequence_mail_id',
        'next_sequence_mail_id',
        'status',
        'started_at',
        'last_executed_at',
        'next_execution_at',
        'completed_at',
        'notes',
        'emails_sent',
        'emails_opened',
        'emails_clicked',
    ];

    protected function casts(): array
    {
        return [
            'started_at' => 'datetime',
            'last_executed_at' => 'datetime',
            'next_execution_at' => 'datetime',
            'completed_at' => 'datetime',
            'notes' => 'array',
            'emails_sent' => 'integer',
            'emails_opened' => 'integer',
            'emails_clicked' => 'integer',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $tracker): void {
            $tracker->status ??= 'active';
            $tracker->started_at ??= now();
            $tracker->notes ??= [];
        });
    }

    // Relationships
    public function contact(): BelongsTo
    {
        return $this->belongsTo(Contact::class);
    }

    public function sequence(): BelongsTo
    {
        return $this->belongsTo(EmailSequence::class, 'sequence_id');
    }

    public function lastMail(): BelongsTo
    {
        return $this->belongsTo(SequenceMail::class, 'last_sequence_mail_id');
    }

    public function nextMail(): BelongsTo
    {
        return $this->belongsTo(SequenceMail::class, 'next_sequence_mail_id');
    }

    // Scopes
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', 'active');
    }

    public function scopeCompleted(Builder $query): Builder
    {
        return $query->where('status', 'completed');
    }

    public function scopeDueForExecution(Builder $query): Builder
    {
        return $query->where('status', 'active')
            ->whereNotNull('next_execution_at')
            ->where('next_execution_at', '<=', now());
    }

    // Business Logic
    public function advanceToNextEmail(): void
    {
        $nextMail = $this->nextMail?->getNextMail();

        if ($nextMail) {
            $this->update([
                'last_sequence_mail_id' => $this->next_sequence_mail_id,
                'next_sequence_mail_id' => $nextMail->id,
                'last_executed_at' => now(),
                'next_execution_at' => now()->addSeconds($nextMail->delay),
                'emails_sent' => $this->emails_sent + 1,
            ]);
        } else {
            $this->markAsCompleted();
        }

        $this->addNote('advanced', [
            'from_mail_id' => $this->last_sequence_mail_id,
            'to_mail_id' => $this->next_sequence_mail_id,
        ]);
    }

    public function markAsCompleted(): void
    {
        $this->update([
            'status' => 'completed',
            'last_sequence_mail_id' => $this->next_sequence_mail_id,
            'next_sequence_mail_id' => null,
            'next_execution_at' => null,
            'completed_at' => now(),
            'last_executed_at' => now(),
            'emails_sent' => $this->emails_sent + 1,
        ]);

        $this->addNote('completed', ['total_emails' => $this->emails_sent]);
    }

    public function pause(): void
    {
        $this->update(['status' => 'paused']);
        $this->addNote('paused');
    }

    public function resume(): void
    {
        $this->update([
            'status' => 'active',
            'next_execution_at' => now(),
        ]);
        $this->addNote('resumed');
    }

    public function cancel(string $reason = 'manual'): void
    {
        $this->update(['status' => 'cancelled']);
        $this->addNote('cancelled', ['reason' => $reason]);
    }

    public function recordOpen(): void
    {
        $this->increment('emails_opened');
        $this->addNote('email_opened', ['mail_id' => $this->last_sequence_mail_id]);
    }

    public function recordClick(): void
    {
        $this->increment('emails_clicked');
        $this->addNote('email_clicked', ['mail_id' => $this->last_sequence_mail_id]);
    }

    public function addNote(string $action, array $data = []): void
    {
        $notes = $this->notes ?? [];
        $notes[] = [
            'action' => $action,
            'data' => $data,
            'at' => now()->toIso8601String(),
        ];
        $this->update(['notes' => $notes]);
    }

    public function getProgressPercentage(): float
    {
        $totalEmails = $this->sequence->emails()->count();
        if ($totalEmails === 0) {
            return 0;
        }

        return round(($this->emails_sent / $totalEmails) * 100, 1);
    }
}
