<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * AbandonedCart Model - Track and recover abandoned shopping carts
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * @property int $id
 * @property int|null $user_id
 * @property string $session_id
 * @property string $email
 * @property array $items
 * @property float $subtotal
 * @property float $total
 * @property string $currency
 * @property string $status
 * @property int $recovery_attempts
 * @property Carbon|null $last_recovery_at
 * @property Carbon|null $recovered_at
 * @property string|null $recovery_code
 * @property float|null $recovery_discount
 * @property string|null $source
 * @property array|null $utm_params
 * @property array|null $metadata
 * @property Carbon $abandoned_at
 * @property Carbon $created_at
 * @property Carbon $updated_at
 *
 * @property-read User|null $user
 */
class AbandonedCart extends Model
{
    use HasFactory;

    protected $table = 'abandoned_carts';

    /**
     * Status constants
     */
    public const STATUS_PENDING = 'pending';
    public const STATUS_EMAIL_SENT = 'email_sent';
    public const STATUS_CLICKED = 'clicked';
    public const STATUS_RECOVERED = 'recovered';
    public const STATUS_EXPIRED = 'expired';
    public const STATUS_UNSUBSCRIBED = 'unsubscribed';

    /**
     * Recovery email sequence timing (in hours)
     */
    public const RECOVERY_SEQUENCE = [
        1 => 1,    // First email: 1 hour after abandonment
        2 => 24,   // Second email: 24 hours after abandonment
        3 => 72,   // Third email: 72 hours after abandonment (with discount)
    ];

    protected $fillable = [
        'user_id',
        'session_id',
        'email',
        'items',
        'subtotal',
        'total',
        'currency',
        'status',
        'recovery_attempts',
        'last_recovery_at',
        'recovered_at',
        'recovery_code',
        'recovery_discount',
        'source',
        'utm_params',
        'metadata',
        'abandoned_at',
    ];

    protected $casts = [
        'items' => 'array',
        'utm_params' => 'array',
        'metadata' => 'array',
        'subtotal' => 'decimal:2',
        'total' => 'decimal:2',
        'recovery_discount' => 'decimal:2',
        'recovery_attempts' => 'integer',
        'abandoned_at' => 'datetime',
        'last_recovery_at' => 'datetime',
        'recovered_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => self::STATUS_PENDING,
        'currency' => 'USD',
        'recovery_attempts' => 0,
    ];

    /**
     * Get the user that owns the cart
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Scope: Pending recovery
     */
    public function scopePendingRecovery(Builder $query): Builder
    {
        return $query->whereIn('status', [self::STATUS_PENDING, self::STATUS_EMAIL_SENT, self::STATUS_CLICKED])
            ->where('recovery_attempts', '<', 3)
            ->where('abandoned_at', '>=', now()->subDays(7));
    }

    /**
     * Scope: Ready for first email (1 hour after abandonment)
     */
    public function scopeReadyForFirstEmail(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_PENDING)
            ->where('recovery_attempts', 0)
            ->where('abandoned_at', '<=', now()->subHours(1))
            ->where('abandoned_at', '>=', now()->subDays(7));
    }

    /**
     * Scope: Ready for second email (24 hours after abandonment)
     */
    public function scopeReadyForSecondEmail(Builder $query): Builder
    {
        return $query->whereIn('status', [self::STATUS_PENDING, self::STATUS_EMAIL_SENT])
            ->where('recovery_attempts', 1)
            ->where('last_recovery_at', '<=', now()->subHours(23))
            ->where('abandoned_at', '>=', now()->subDays(7));
    }

    /**
     * Scope: Ready for third email (72 hours after abandonment, with discount)
     */
    public function scopeReadyForThirdEmail(Builder $query): Builder
    {
        return $query->whereIn('status', [self::STATUS_PENDING, self::STATUS_EMAIL_SENT, self::STATUS_CLICKED])
            ->where('recovery_attempts', 2)
            ->where('last_recovery_at', '<=', now()->subHours(48))
            ->where('abandoned_at', '>=', now()->subDays(7));
    }

    /**
     * Scope: Recovered
     */
    public function scopeRecovered(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_RECOVERED);
    }

    /**
     * Scope: By time period
     */
    public function scopeByPeriod(Builder $query, string $period): Builder
    {
        $days = match($period) {
            '24h' => 1,
            '7d' => 7,
            '30d' => 30,
            '90d' => 90,
            default => null,
        };

        if ($days) {
            $query->where('abandoned_at', '>=', now()->subDays($days));
        }

        return $query;
    }

    /**
     * Scope: High value carts
     */
    public function scopeHighValue(Builder $query, float $minValue = 100): Builder
    {
        return $query->where('total', '>=', $minValue);
    }

    /**
     * Check if cart can receive recovery email
     */
    public function canReceiveRecoveryEmail(): bool
    {
        if ($this->status === self::STATUS_RECOVERED) {
            return false;
        }

        if ($this->status === self::STATUS_UNSUBSCRIBED) {
            return false;
        }

        if ($this->status === self::STATUS_EXPIRED) {
            return false;
        }

        if ($this->recovery_attempts >= 3) {
            return false;
        }

        if ($this->abandoned_at->lt(now()->subDays(7))) {
            return false;
        }

        return true;
    }

    /**
     * Get next recovery email number
     */
    public function getNextRecoveryEmail(): ?int
    {
        if (!$this->canReceiveRecoveryEmail()) {
            return null;
        }

        return $this->recovery_attempts + 1;
    }

    /**
     * Record recovery attempt
     */
    public function recordRecoveryAttempt(): void
    {
        $this->recovery_attempts++;
        $this->last_recovery_at = now();
        $this->status = self::STATUS_EMAIL_SENT;
        $this->save();
    }

    /**
     * Mark as recovered
     */
    public function markAsRecovered(?string $orderId = null): void
    {
        $this->status = self::STATUS_RECOVERED;
        $this->recovered_at = now();

        if ($orderId) {
            $metadata = $this->metadata ?? [];
            $metadata['recovered_order_id'] = $orderId;
            $this->metadata = $metadata;
        }

        $this->save();
    }

    /**
     * Mark as clicked (user clicked recovery link)
     */
    public function markAsClicked(): void
    {
        if ($this->status !== self::STATUS_RECOVERED) {
            $this->status = self::STATUS_CLICKED;
            $this->save();
        }
    }

    /**
     * Get item count
     */
    public function getItemCountAttribute(): int
    {
        return collect($this->items)->sum('quantity');
    }

    /**
     * Get hours since abandonment
     */
    public function getHoursSinceAbandonmentAttribute(): float
    {
        return round($this->abandoned_at->diffInMinutes(now()) / 60, 1);
    }

    /**
     * Generate recovery code
     */
    public function generateRecoveryCode(?int $discountPercent = null): string
    {
        $code = 'SAVE' . strtoupper(substr(md5($this->id . now()->timestamp), 0, 6));

        $this->recovery_code = $code;

        if ($discountPercent) {
            $this->recovery_discount = $discountPercent;
        }

        $this->save();

        return $code;
    }
}
