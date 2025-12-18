<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * EmailCampaign Model
 * 
 * Manages email marketing campaigns
 * 
 * @property int $id
 * @property string $name
 * @property int $template_id
 * @property int|null $segment_id
 * @property string $subject
 * @property string $from_name
 * @property string $from_email
 * @property string|null $reply_to
 * @property Carbon|null $scheduled_at
 * @property Carbon|null $sent_at
 * @property string $status
 * @property int $total_recipients
 * @property int $sent_count
 * @property int $opened_count
 * @property int $clicked_count
 * @property int $bounced_count
 * @property int $unsubscribed_count
 * @property array|null $ab_test_config
 * @property int $created_by
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read EmailTemplate $template
 * @property-read User $creator
 * @property-read \Illuminate\Database\Eloquent\Collection|EmailLog[] $emailLogs
 */
class EmailCampaign extends Model
{
    use HasFactory;

    protected $fillable = [
        'name',
        'template_id',
        'segment_id',
        'subject',
        'from_name',
        'from_email',
        'reply_to',
        'scheduled_at',
        'sent_at',
        'status',
        'total_recipients',
        'sent_count',
        'opened_count',
        'clicked_count',
        'bounced_count',
        'unsubscribed_count',
        'ab_test_config',
        'created_by',
    ];

    protected $casts = [
        'scheduled_at' => 'datetime',
        'sent_at' => 'datetime',
        'ab_test_config' => 'array',
    ];

    /**
     * Relationships
     */
    public function template(): BelongsTo
    {
        return $this->belongsTo(EmailTemplate::class);
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    public function emailLogs(): HasMany
    {
        return $this->hasMany(EmailLog::class, 'campaign_id');
    }

    /**
     * Scopes
     */
    public function scopeDraft(Builder $query): Builder
    {
        return $query->where('status', 'draft');
    }

    public function scopeScheduled(Builder $query): Builder
    {
        return $query->where('status', 'scheduled');
    }

    public function scopeSending(Builder $query): Builder
    {
        return $query->where('status', 'sending');
    }

    public function scopeSent(Builder $query): Builder
    {
        return $query->where('status', 'sent');
    }

    public function scopeReadyToSend(Builder $query): Builder
    {
        return $query->where('status', 'scheduled')
            ->where('scheduled_at', '<=', now());
    }

    /**
     * Calculated attributes
     */
    public function getOpenRateAttribute(): float
    {
        if ($this->sent_count === 0) {
            return 0;
        }
        return ($this->opened_count / $this->sent_count) * 100;
    }

    public function getClickRateAttribute(): float
    {
        if ($this->sent_count === 0) {
            return 0;
        }
        return ($this->clicked_count / $this->sent_count) * 100;
    }

    public function getBounceRateAttribute(): float
    {
        if ($this->sent_count === 0) {
            return 0;
        }
        return ($this->bounced_count / $this->sent_count) * 100;
    }

    public function getUnsubscribeRateAttribute(): float
    {
        if ($this->sent_count === 0) {
            return 0;
        }
        return ($this->unsubscribed_count / $this->sent_count) * 100;
    }

    public function getProgressPercentageAttribute(): float
    {
        if ($this->total_recipients === 0) {
            return 0;
        }
        return ($this->sent_count / $this->total_recipients) * 100;
    }

    /**
     * Check if campaign can be sent
     */
    public function canBeSent(): bool
    {
        return in_array($this->status, ['draft', 'scheduled']) 
            && $this->total_recipients > 0;
    }

    /**
     * Check if campaign is complete
     */
    public function isComplete(): bool
    {
        return $this->status === 'sent' 
            && $this->sent_count >= $this->total_recipients;
    }

    /**
     * Mark campaign as sending
     */
    public function markAsSending(): void
    {
        $this->update(['status' => 'sending']);
    }

    /**
     * Mark campaign as sent
     */
    public function markAsSent(): void
    {
        $this->update([
            'status' => 'sent',
            'sent_at' => now(),
        ]);
    }

    /**
     * Increment sent count
     */
    public function incrementSent(): void
    {
        $this->increment('sent_count');
    }

    /**
     * Increment opened count
     */
    public function incrementOpened(): void
    {
        $this->increment('opened_count');
    }

    /**
     * Increment clicked count
     */
    public function incrementClicked(): void
    {
        $this->increment('clicked_count');
    }

    /**
     * Increment bounced count
     */
    public function incrementBounced(): void
    {
        $this->increment('bounced_count');
    }

    /**
     * Increment unsubscribed count
     */
    public function incrementUnsubscribed(): void
    {
        $this->increment('unsubscribed_count');
    }
}
