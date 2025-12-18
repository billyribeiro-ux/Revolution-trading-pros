<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * EmailEvent Model
 * 
 * Tracks email events (sent, opened, clicked, bounced, etc.)
 * 
 * @property int $id
 * @property int $email_log_id
 * @property string $event_type
 * @property array|null $event_data
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property string|null $device_type
 * @property array|null $location
 * @property Carbon $event_timestamp
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read EmailLog $emailLog
 */
class EmailEvent extends Model
{
    use HasFactory;

    protected $fillable = [
        'email_log_id',
        'event_type',
        'event_data',
        'ip_address',
        'user_agent',
        'device_type',
        'location',
        'event_timestamp',
    ];

    protected $casts = [
        'event_data' => 'array',
        'location' => 'array',
        'event_timestamp' => 'datetime',
    ];

    /**
     * Relationships
     */
    public function emailLog(): BelongsTo
    {
        return $this->belongsTo(EmailLog::class);
    }

    /**
     * Scope: Filter by event type
     */
    public function scopeByType(Builder $query, string $type): Builder
    {
        return $query->where('event_type', $type);
    }

    /**
     * Scope: Recent events
     */
    public function scopeRecent(Builder $query, int $days = 7): Builder
    {
        return $query->where('event_timestamp', '>=', now()->subDays($days));
    }

    /**
     * Scope: Opened emails
     */
    public function scopeOpened(Builder $query): Builder
    {
        return $query->where('event_type', 'opened');
    }

    /**
     * Scope: Clicked emails
     */
    public function scopeClicked(Builder $query): Builder
    {
        return $query->where('event_type', 'clicked');
    }

    /**
     * Scope: Bounced emails
     */
    public function scopeBounced(Builder $query): Builder
    {
        return $query->where('event_type', 'bounced');
    }

    /**
     * Check if event is engagement (open or click)
     */
    public function isEngagement(): bool
    {
        return in_array($this->event_type, ['opened', 'clicked']);
    }

    /**
     * Check if event is negative (bounce or complaint)
     */
    public function isNegative(): bool
    {
        return in_array($this->event_type, ['bounced', 'complained']);
    }
}
