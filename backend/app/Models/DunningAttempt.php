<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * Dunning Attempt Model
 *
 * Tracks payment retry attempts for failed subscriptions
 */
class DunningAttempt extends Model
{
    protected $fillable = [
        'user_id',
        'subscription_id',
        'invoice_id',
        'attempt_number',
        'status',
        'scheduled_at',
        'processed_at',
        'reminder_sent_at',
        'error_message',
    ];

    protected $casts = [
        'scheduled_at' => 'datetime',
        'processed_at' => 'datetime',
        'reminder_sent_at' => 'datetime',
    ];

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function subscription(): BelongsTo
    {
        return $this->belongsTo(Subscription::class);
    }

    public function invoice(): BelongsTo
    {
        return $this->belongsTo(Invoice::class);
    }
}
