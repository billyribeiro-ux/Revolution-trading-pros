<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * EmailWebhookDelivery Model
 *
 * Represents a webhook delivery attempt.
 *
 * @property int $id
 * @property int $webhook_id
 * @property string $event
 * @property string $payload
 * @property string $status
 * @property int|null $response_code
 * @property string|null $response_body
 * @property int|null $response_time_ms
 * @property int|null $retry_of
 * @property \Carbon\Carbon $created_at
 */
class EmailWebhookDelivery extends Model
{
    public $timestamps = false;

    protected $table = 'email_webhook_deliveries';

    protected $fillable = [
        'webhook_id',
        'event',
        'payload',
        'status',
        'response_code',
        'response_body',
        'response_time_ms',
        'retry_of',
        'created_at',
    ];

    protected $casts = [
        'created_at' => 'datetime',
    ];

    /**
     * Get the webhook
     */
    public function webhook(): BelongsTo
    {
        return $this->belongsTo(EmailWebhook::class, 'webhook_id');
    }

    /**
     * Check if delivery was successful
     */
    public function isSuccessful(): bool
    {
        return $this->status === 'success';
    }
}
