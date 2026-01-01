<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * Recurring Mail Model (Individual sends from recurring campaigns)
 *
 * @property string $id
 * @property string $recurring_campaign_id
 * @property string $email_subject
 * @property string|null $email_pre_header
 * @property string $email_body
 * @property string $status
 * @property int $recipients_count
 * @property int $sent_count
 * @property int $failed_count
 * @property int $open_count
 * @property int $click_count
 * @property int $unsubscribe_count
 * @property float $revenue
 * @property Carbon|null $scheduled_at
 * @property Carbon|null $sent_at
 */
class RecurringMail extends Model
{
    use HasUuids;

    protected $fillable = [
        'recurring_campaign_id',
        'email_subject',
        'email_pre_header',
        'email_body',
        'status',
        'recipients_count',
        'scheduled_at',
        'sent_at',
    ];

    protected function casts(): array
    {
        return [
            'recipients_count' => 'integer',
            'sent_count' => 'integer',
            'failed_count' => 'integer',
            'open_count' => 'integer',
            'click_count' => 'integer',
            'unsubscribe_count' => 'integer',
            'revenue' => 'decimal:2',
            'scheduled_at' => 'datetime',
            'sent_at' => 'datetime',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $mail): void {
            $mail->status ??= 'draft';
        });

        static::updated(function (self $mail): void {
            if ($mail->wasChanged('status') && $mail->status === 'sent') {
                $mail->campaign->increment('total_campaigns_sent');
                $mail->campaign->increment('total_emails_sent', $mail->sent_count);
                $mail->campaign->update(['last_sent_at' => now()]);
                $mail->campaign->scheduleNextSend();
            }
        });
    }

    // Relationships
    public function campaign(): BelongsTo
    {
        return $this->belongsTo(RecurringCampaign::class, 'recurring_campaign_id');
    }

    // Scopes
    public function scopeScheduled(Builder $query): Builder
    {
        return $query->where('status', 'scheduled');
    }

    public function scopeSent(Builder $query): Builder
    {
        return $query->where('status', 'sent');
    }

    public function scopeDueForSending(Builder $query): Builder
    {
        return $query->where('status', 'scheduled')
            ->where('scheduled_at', '<=', now());
    }

    // Business Logic
    public function markAsSending(): void
    {
        $this->update(['status' => 'sending']);
    }

    public function markAsSent(): void
    {
        $this->update([
            'status' => 'sent',
            'sent_at' => now(),
        ]);
    }

    public function markAsFailed(): void
    {
        $this->update(['status' => 'failed']);
    }

    public function recordSend(): void
    {
        $this->increment('sent_count');
    }

    public function recordFailure(): void
    {
        $this->increment('failed_count');
    }

    public function recordOpen(): void
    {
        $this->increment('open_count');
    }

    public function recordClick(): void
    {
        $this->increment('click_count');
    }

    public function recordUnsubscribe(): void
    {
        $this->increment('unsubscribe_count');
    }

    public function getOpenRate(): float
    {
        return $this->sent_count > 0
            ? round(($this->open_count / $this->sent_count) * 100, 2)
            : 0;
    }

    public function getClickRate(): float
    {
        return $this->open_count > 0
            ? round(($this->click_count / $this->open_count) * 100, 2)
            : 0;
    }

    public function getStats(): array
    {
        return [
            'recipients' => $this->recipients_count,
            'sent' => $this->sent_count,
            'failed' => $this->failed_count,
            'opened' => $this->open_count,
            'clicked' => $this->click_count,
            'unsubscribed' => $this->unsubscribe_count,
            'open_rate' => $this->getOpenRate(),
            'click_rate' => $this->getClickRate(),
            'revenue' => $this->revenue,
        ];
    }
}
