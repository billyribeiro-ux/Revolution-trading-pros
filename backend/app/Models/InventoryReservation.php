<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;

/**
 * InventoryReservation Model
 *
 * Tracks inventory reservations with TTL for cart abandonment handling.
 *
 * @property int $id
 * @property string $reservation_id
 * @property int $product_id
 * @property int $quantity
 * @property string|null $cart_id
 * @property int|null $user_id
 * @property int|null $order_id
 * @property string $status
 * @property \Carbon\Carbon|null $expires_at
 * @property \Carbon\Carbon|null $committed_at
 * @property \Carbon\Carbon|null $released_at
 * @property array|null $metadata
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class InventoryReservation extends Model
{
    use HasFactory;

    protected $table = 'inventory_reservations';

    protected $fillable = [
        'reservation_id',
        'product_id',
        'quantity',
        'cart_id',
        'user_id',
        'order_id',
        'status',
        'expires_at',
        'committed_at',
        'released_at',
        'metadata',
    ];

    protected $casts = [
        'quantity' => 'integer',
        'product_id' => 'integer',
        'user_id' => 'integer',
        'order_id' => 'integer',
        'expires_at' => 'datetime',
        'committed_at' => 'datetime',
        'released_at' => 'datetime',
        'metadata' => 'array',
    ];

    /**
     * Status constants
     */
    public const STATUS_ACTIVE = 'active';
    public const STATUS_COMMITTED = 'committed';
    public const STATUS_RELEASED = 'released';
    public const STATUS_EXPIRED = 'expired';
    public const STATUS_BACKORDER = 'backorder';

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function product(): BelongsTo
    {
        return $this->belongsTo(Product::class);
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function order(): BelongsTo
    {
        return $this->belongsTo(Order::class);
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_ACTIVE);
    }

    public function scopeCommitted(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_COMMITTED);
    }

    public function scopeExpired(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_ACTIVE)
            ->where('expires_at', '<', now());
    }

    public function scopeBackorder(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_BACKORDER);
    }

    public function scopeForProduct(Builder $query, int $productId): Builder
    {
        return $query->where('product_id', $productId);
    }

    public function scopeForCart(Builder $query, string $cartId): Builder
    {
        return $query->where('cart_id', $cartId);
    }

    public function scopeForUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getIsExpiredAttribute(): bool
    {
        return $this->status === self::STATUS_ACTIVE
            && $this->expires_at
            && $this->expires_at->isPast();
    }

    public function getTimeRemainingAttribute(): ?int
    {
        if (!$this->expires_at || $this->status !== self::STATUS_ACTIVE) {
            return null;
        }

        return max(0, now()->diffInSeconds($this->expires_at, false));
    }

    public function getStatusLabelAttribute(): string
    {
        return match ($this->status) {
            self::STATUS_ACTIVE => $this->is_expired ? 'Expired' : 'Active',
            self::STATUS_COMMITTED => 'Committed',
            self::STATUS_RELEASED => 'Released',
            self::STATUS_BACKORDER => 'Backorder',
            default => ucfirst($this->status),
        };
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    public function isActive(): bool
    {
        return $this->status === self::STATUS_ACTIVE && !$this->is_expired;
    }

    public function extend(int $seconds = 900): bool
    {
        if (!$this->isActive()) {
            return false;
        }

        $this->update([
            'expires_at' => now()->addSeconds($seconds),
        ]);

        return true;
    }
}
