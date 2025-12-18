<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * EmailWebhook Model
 *
 * Represents a webhook endpoint for email marketing events.
 *
 * @property int $id
 * @property string $name
 * @property string $url
 * @property array $events
 * @property string $secret
 * @property string|null $previous_secret
 * @property array|null $headers
 * @property string|null $description
 * @property string $status
 * @property int $consecutive_failures
 * @property \Carbon\Carbon|null $last_triggered_at
 * @property \Carbon\Carbon|null $secret_rotated_at
 * @property int|null $created_by
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class EmailWebhook extends Model
{
    use HasFactory, SoftDeletes;

    protected $fillable = [
        'name',
        'url',
        'events',
        'secret',
        'previous_secret',
        'headers',
        'description',
        'status',
        'consecutive_failures',
        'last_triggered_at',
        'secret_rotated_at',
        'created_by',
    ];

    protected $casts = [
        'events' => 'array',
        'headers' => 'array',
        'last_triggered_at' => 'datetime',
        'secret_rotated_at' => 'datetime',
    ];

    protected $hidden = [
        'secret',
        'previous_secret',
    ];

    /**
     * Status constants
     */
    public const STATUS_ACTIVE = 'active';
    public const STATUS_INACTIVE = 'inactive';
    public const STATUS_FAILING = 'failing';

    /**
     * Get the creator of this webhook
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get webhook deliveries
     */
    public function deliveries()
    {
        return $this->hasMany(EmailWebhookDelivery::class, 'webhook_id');
    }

    /**
     * Scope for active webhooks
     */
    public function scopeActive($query)
    {
        return $query->where('status', self::STATUS_ACTIVE);
    }

    /**
     * Scope for webhooks subscribed to an event
     */
    public function scopeForEvent($query, string $event)
    {
        return $query->whereJsonContains('events', $event);
    }

    /**
     * Check if webhook is subscribed to an event
     */
    public function isSubscribedTo(string $event): bool
    {
        return in_array($event, $this->events ?? []);
    }

    /**
     * Record a failure
     */
    public function recordFailure(): void
    {
        $this->increment('consecutive_failures');

        // Auto-disable after too many failures
        if ($this->consecutive_failures >= 10) {
            $this->update(['status' => self::STATUS_FAILING]);
        }
    }

    /**
     * Record a success
     */
    public function recordSuccess(): void
    {
        $this->update([
            'consecutive_failures' => 0,
            'last_triggered_at' => now(),
        ]);

        // Re-enable if it was failing
        if ($this->status === self::STATUS_FAILING) {
            $this->update(['status' => self::STATUS_ACTIVE]);
        }
    }

    /**
     * Get valid secret (current or previous during grace period)
     */
    public function getValidSecret(string $providedSignature): ?string
    {
        // Check current secret
        if ($providedSignature === $this->secret) {
            return $this->secret;
        }

        // Check previous secret if within grace period (24 hours)
        if ($this->previous_secret && $this->secret_rotated_at) {
            if ($this->secret_rotated_at->addHours(24)->isFuture()) {
                if ($providedSignature === $this->previous_secret) {
                    return $this->previous_secret;
                }
            }
        }

        return null;
    }
}
