<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use InvalidArgumentException;

/**
 * Order Model
 * 
 * Comprehensive e-commerce order management with payment processing,
 * fulfillment tracking, multi-currency support, and advanced analytics.
 *
 * @property int $id
 * @property int|null $user_id Customer user ID (null for guest orders)
 * @property string $order_number Unique order number
 * @property string $status Order status (pending, processing, completed, etc.)
 * @property string $payment_status Payment status (pending, paid, failed, refunded, etc.)
 * @property string $fulfillment_status Overall fulfillment status
 * @property float $subtotal Items subtotal
 * @property float $discount_amount Total discount
 * @property string|null $discount_code Applied coupon code
 * @property float $tax Tax amount
 * @property float|null $tax_rate Tax rate percentage
 * @property float $shipping_cost Shipping cost
 * @property float $total Order total
 * @property float|null $refund_amount Total refunded
 * @property string $currency Currency code (USD, EUR, etc.)
 * @property float|null $exchange_rate Exchange rate at time of order
 * @property string|null $payment_provider Payment gateway (stripe, paypal, etc.)
 * @property string|null $payment_intent_id Payment provider transaction ID
 * @property string|null $payment_method Payment method (card, bank, etc.)
 * @property string|null $card_brand Card brand (visa, mastercard, etc.)
 * @property string|null $card_last4 Last 4 digits of card
 * @property array $metadata Additional order data
 * @property array $billing_address Billing address details
 * @property array $shipping_address Shipping address details
 * @property string|null $customer_email Customer email
 * @property string|null $customer_phone Customer phone
 * @property string|null $customer_name Customer name
 * @property string|null $customer_ip Customer IP address
 * @property string|null $customer_user_agent Customer user agent
 * @property string|null $shipping_method Shipping method/carrier
 * @property string|null $tracking_number Shipment tracking number
 * @property string|null $tracking_url Tracking URL
 * @property array $tracking_events Tracking status updates
 * @property bool $is_gift Whether order is a gift
 * @property string|null $gift_message Gift message
 * @property string|null $gift_wrap Gift wrap option
 * @property float|null $gift_wrap_cost Gift wrap cost
 * @property string|null $notes Customer order notes
 * @property string|null $internal_notes Internal staff notes
 * @property array $tags Order tags for organization
 * @property int $items_count Total items count
 * @property float|null $weight Total order weight
 * @property string|null $weight_unit Weight unit (kg, lb)
 * @property bool $requires_shipping Whether order needs shipping
 * @property bool $is_paid Whether order is paid
 * @property bool $is_shipped Whether order is shipped
 * @property bool $is_delivered Whether order is delivered
 * @property bool $is_canceled Whether order is canceled
 * @property bool $is_refunded Whether order is refunded
 * @property string|null $cancellation_reason Cancellation reason
 * @property string|null $refund_reason Refund reason
 * @property \Illuminate\Support\Carbon|null $paid_at Payment timestamp
 * @property \Illuminate\Support\Carbon|null $shipped_at Shipment timestamp
 * @property \Illuminate\Support\Carbon|null $delivered_at Delivery timestamp
 * @property \Illuminate\Support\Carbon|null $canceled_at Cancellation timestamp
 * @property \Illuminate\Support\Carbon|null $refunded_at Refund timestamp
 * @property \Illuminate\Support\Carbon|null $expires_at Order expiration (for pending)
 * @property string|null $source Order source (web, mobile, api, pos, etc.)
 * @property string|null $referrer Referrer URL
 * @property string|null $utm_source UTM source
 * @property string|null $utm_medium UTM medium
 * @property string|null $utm_campaign UTM campaign
 * @property int|null $parent_order_id Parent order (for reorders)
 * @property bool $is_test Test/sandbox order
 * @property string|null $created_by User who created order
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \App\Models\User|null $user Customer
 * @property-read \Illuminate\Database\Eloquent\Collection $items Order items
 * @property-read \Illuminate\Database\Eloquent\Collection $transactions Payment transactions
 * @property-read \Illuminate\Database\Eloquent\Collection $refunds Refund records
 * @property-read \Illuminate\Database\Eloquent\Collection $activities Activity log
 */
class Order extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Order statuses
     */
    public const STATUS_PENDING = 'pending';
    public const STATUS_PROCESSING = 'processing';
    public const STATUS_COMPLETED = 'completed';
    public const STATUS_SHIPPED = 'shipped';
    public const STATUS_DELIVERED = 'delivered';
    public const STATUS_CANCELED = 'canceled';
    public const STATUS_REFUNDED = 'refunded';
    public const STATUS_FAILED = 'failed';
    public const STATUS_ON_HOLD = 'on_hold';
    public const STATUS_AWAITING_PAYMENT = 'awaiting_payment';

    /**
     * Payment statuses
     */
    public const PAYMENT_PENDING = 'pending';
    public const PAYMENT_AUTHORIZED = 'authorized';
    public const PAYMENT_PAID = 'paid';
    public const PAYMENT_FAILED = 'failed';
    public const PAYMENT_REFUNDED = 'refunded';
    public const PAYMENT_PARTIALLY_REFUNDED = 'partially_refunded';
    public const PAYMENT_CANCELED = 'canceled';
    public const PAYMENT_EXPIRED = 'expired';

    /**
     * Fulfillment statuses
     */
    public const FULFILLMENT_UNFULFILLED = 'unfulfilled';
    public const FULFILLMENT_PARTIAL = 'partial';
    public const FULFILLMENT_FULFILLED = 'fulfilled';
    public const FULFILLMENT_SHIPPED = 'shipped';
    public const FULFILLMENT_DELIVERED = 'delivered';
    public const FULFILLMENT_RETURNED = 'returned';

    /**
     * Valid statuses
     */
    public const VALID_STATUSES = [
        self::STATUS_PENDING,
        self::STATUS_PROCESSING,
        self::STATUS_COMPLETED,
        self::STATUS_SHIPPED,
        self::STATUS_DELIVERED,
        self::STATUS_CANCELED,
        self::STATUS_REFUNDED,
        self::STATUS_FAILED,
        self::STATUS_ON_HOLD,
        self::STATUS_AWAITING_PAYMENT,
    ];

    /**
     * Valid payment statuses
     */
    public const VALID_PAYMENT_STATUSES = [
        self::PAYMENT_PENDING,
        self::PAYMENT_AUTHORIZED,
        self::PAYMENT_PAID,
        self::PAYMENT_FAILED,
        self::PAYMENT_REFUNDED,
        self::PAYMENT_PARTIALLY_REFUNDED,
        self::PAYMENT_CANCELED,
        self::PAYMENT_EXPIRED,
    ];

    /**
     * Order number prefix
     */
    public const ORDER_NUMBER_PREFIX = 'ORD';

    /**
     * Default currency
     */
    public const DEFAULT_CURRENCY = 'USD';

    /**
     * Cache TTL
     */
    public const CACHE_TTL = 3600;

    protected $fillable = [
        'user_id',
        'order_number',
        'status',
        'payment_status',
        'fulfillment_status',
        'subtotal',
        'discount_amount',
        'discount_code',
        'tax',
        'tax_rate',
        'shipping_cost',
        'total',
        'refund_amount',
        'currency',
        'exchange_rate',
        'payment_provider',
        'payment_intent_id',
        'payment_method',
        'card_brand',
        'card_last4',
        'metadata',
        'billing_address',
        'shipping_address',
        'customer_email',
        'customer_phone',
        'customer_name',
        'customer_ip',
        'customer_user_agent',
        'shipping_method',
        'tracking_number',
        'tracking_url',
        'tracking_events',
        'is_gift',
        'gift_message',
        'gift_wrap',
        'gift_wrap_cost',
        'notes',
        'internal_notes',
        'tags',
        'items_count',
        'weight',
        'weight_unit',
        'requires_shipping',
        'is_paid',
        'is_shipped',
        'is_delivered',
        'is_canceled',
        'is_refunded',
        'cancellation_reason',
        'refund_reason',
        'paid_at',
        'shipped_at',
        'delivered_at',
        'canceled_at',
        'refunded_at',
        'expires_at',
        'source',
        'referrer',
        'utm_source',
        'utm_medium',
        'utm_campaign',
        'parent_order_id',
        'is_test',
        'created_by',
    ];

    protected $casts = [
        'metadata' => 'array',
        'billing_address' => 'array',
        'shipping_address' => 'array',
        'tracking_events' => 'array',
        'tags' => 'array',
        'subtotal' => 'decimal:2',
        'discount_amount' => 'decimal:2',
        'tax' => 'decimal:2',
        'tax_rate' => 'decimal:2',
        'shipping_cost' => 'decimal:2',
        'total' => 'decimal:2',
        'refund_amount' => 'decimal:2',
        'exchange_rate' => 'decimal:4',
        'gift_wrap_cost' => 'decimal:2',
        'weight' => 'decimal:2',
        'items_count' => 'integer',
        'is_gift' => 'boolean',
        'requires_shipping' => 'boolean',
        'is_paid' => 'boolean',
        'is_shipped' => 'boolean',
        'is_delivered' => 'boolean',
        'is_canceled' => 'boolean',
        'is_refunded' => 'boolean',
        'is_test' => 'boolean',
        'paid_at' => 'datetime',
        'shipped_at' => 'datetime',
        'delivered_at' => 'datetime',
        'canceled_at' => 'datetime',
        'refunded_at' => 'datetime',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => self::STATUS_PENDING,
        'payment_status' => self::PAYMENT_PENDING,
        'fulfillment_status' => self::FULFILLMENT_UNFULFILLED,
        'currency' => self::DEFAULT_CURRENCY,
        'subtotal' => 0.0,
        'discount_amount' => 0.0,
        'tax' => 0.0,
        'shipping_cost' => 0.0,
        'total' => 0.0,
        'items_count' => 0,
        'is_gift' => false,
        'requires_shipping' => true,
        'is_paid' => false,
        'is_shipped' => false,
        'is_delivered' => false,
        'is_canceled' => false,
        'is_refunded' => false,
        'is_test' => false,
        'metadata' => '[]',
        'billing_address' => '[]',
        'shipping_address' => '[]',
        'tracking_events' => '[]',
        'tags' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->generateOrderNumber();
            $model->validateStatus();
            $model->setExchangeRate();
            $model->extractCustomerData();
        });

        static::updating(function (self $model): void {
            $model->validateStatus();
            
            if ($model->isDirty('status')) {
                $model->handleStatusChange();
            }

            if ($model->isDirty('payment_status')) {
                $model->handlePaymentStatusChange();
            }
        });

        static::saved(function (self $model): void {
            $model->clearOrderCache();
        });

        static::deleted(function (self $model): void {
            $model->clearOrderCache();
        });
    }

    /**
     * Get route key name
     */
    public function getRouteKeyName(): string
    {
        return 'order_number';
    }

    /**
     * Get customer
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Get order items
     */
    public function items(): HasMany
    {
        return $this->hasMany(OrderItem::class);
    }

    /**
     * Get parent order (for reorders)
     */
    public function parentOrder(): BelongsTo
    {
        return $this->belongsTo(Order::class, 'parent_order_id');
    }

    /**
     * Get child orders (reorders)
     */
    public function reorders(): HasMany
    {
        return $this->hasMany(Order::class, 'parent_order_id');
    }

    /**
     * Get payment transactions
     */
    public function transactions(): HasMany
    {
        return $this->hasMany(OrderTransaction::class);
    }

    /**
     * Get refunds
     */
    public function refunds(): HasMany
    {
        return $this->hasMany(OrderRefund::class);
    }

    /**
     * Get shipping details
     */
    public function shipping(): HasOne
    {
        return $this->hasOne(OrderShipping::class);
    }

    /**
     * Get activity log
     */
    public function activities(): MorphMany
    {
        return $this->morphMany(Activity::class, 'subject');
    }

    /**
     * Generate unique order number
     */
    protected function generateOrderNumber(): void
    {
        if (!empty($this->order_number)) {
            return;
        }

        do {
            $number = self::ORDER_NUMBER_PREFIX . '-' 
                . date('Ymd') . '-' 
                . strtoupper(Str::random(6));
        } while (static::where('order_number', $number)->exists());

        $this->order_number = $number;
    }

    /**
     * Validate order status
     */
    protected function validateStatus(): void
    {
        if (!in_array($this->status, self::VALID_STATUSES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid order status: %s', $this->status)
            );
        }

        if (!in_array($this->payment_status, self::VALID_PAYMENT_STATUSES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid payment status: %s', $this->payment_status)
            );
        }
    }

    /**
     * Set exchange rate for currency conversion
     */
    protected function setExchangeRate(): void
    {
        if ($this->currency === self::DEFAULT_CURRENCY) {
            $this->exchange_rate = 1.0;
            return;
        }

        // Fetch exchange rate from service (cached)
        $this->exchange_rate = $this->fetchExchangeRate($this->currency);
    }

    /**
     * Fetch exchange rate (implement with your currency service)
     */
    protected function fetchExchangeRate(string $currency): float
    {
        return Cache::remember(
            "exchange_rate:{$currency}",
            3600,
            fn() => 1.0 // Replace with actual API call
        );
    }

    /**
     * Extract customer data from user or request
     */
    protected function extractCustomerData(): void
    {
        if ($this->user) {
            if (!$this->customer_email) {
                $this->customer_email = $this->user->email;
            }
            if (!$this->customer_name) {
                $this->customer_name = $this->user->name;
            }
        }

        if (!$this->customer_ip) {
            $this->customer_ip = request()->ip();
        }

        if (!$this->customer_user_agent) {
            $this->customer_user_agent = request()->userAgent();
        }
    }

    /**
     * Handle status changes
     */
    protected function handleStatusChange(): void
    {
        $newStatus = $this->status;
        $oldStatus = $this->getOriginal('status');

        match($newStatus) {
            self::STATUS_SHIPPED => $this->markAsShipped(),
            self::STATUS_DELIVERED => $this->markAsDelivered(),
            self::STATUS_CANCELED => $this->markAsCanceled(),
            self::STATUS_REFUNDED => $this->markAsRefunded(),
            default => null,
        };

        $this->logActivity("Order status changed from {$oldStatus} to {$newStatus}");
    }

    /**
     * Handle payment status changes
     */
    protected function handlePaymentStatusChange(): void
    {
        if ($this->payment_status === self::PAYMENT_PAID && !$this->is_paid) {
            $this->markAsPaid();
        }
    }

    /**
     * Clear order cache
     */
    protected function clearOrderCache(): void
    {
        Cache::tags(['orders'])->flush();
        Cache::forget("order:number:{$this->order_number}");
        Cache::forget("order:id:{$this->id}");
    }

    /**
     * Calculate totals from items
     */
    public function calculateTotals(): self
    {
        $itemTotals = OrderItem::calculateTotals($this->items);

        $this->update([
            'subtotal' => $itemTotals['subtotal'],
            'discount_amount' => $itemTotals['discount'],
            'tax' => $itemTotals['tax'],
            'shipping_cost' => $itemTotals['shipping'],
            'total' => $itemTotals['total'],
            'items_count' => $itemTotals['quantity'],
        ]);

        return $this;
    }

    /**
     * Add item to order
     */
    public function addItem(array $itemData): OrderItem
    {
        $item = $this->items()->create($itemData);
        
        $this->calculateTotals();
        
        return $item;
    }

    /**
     * Apply discount
     */
    public function applyDiscount(float $amount, ?string $code = null): self
    {
        $this->discount_amount = round($amount, 2);
        $this->discount_code = $code;
        $this->calculateOrderTotal();
        $this->save();

        return $this;
    }

    /**
     * Calculate order total
     */
    protected function calculateOrderTotal(): void
    {
        $this->total = round(
            $this->subtotal - $this->discount_amount + $this->tax + $this->shipping_cost + ($this->gift_wrap_cost ?? 0),
            2
        );
    }

    /**
     * Mark order as paid
     */
    public function markAsPaid(): self
    {
        $this->update([
            'payment_status' => self::PAYMENT_PAID,
            'is_paid' => true,
            'paid_at' => now(),
            'status' => $this->status === self::STATUS_AWAITING_PAYMENT 
                ? self::STATUS_PROCESSING 
                : $this->status,
        ]);

        // Deduct inventory for all items
        foreach ($this->items as $item) {
            $item->deductInventory();
        }

        $this->logActivity('Order marked as paid');

        return $this;
    }

    /**
     * Mark order as shipped
     */
    public function markAsShipped(?string $trackingNumber = null, ?string $trackingUrl = null): self
    {
        $updates = [
            'status' => self::STATUS_SHIPPED,
            'fulfillment_status' => self::FULFILLMENT_SHIPPED,
            'is_shipped' => true,
            'shipped_at' => now(),
        ];

        if ($trackingNumber) {
            $updates['tracking_number'] = $trackingNumber;
        }

        if ($trackingUrl) {
            $updates['tracking_url'] = $trackingUrl;
        }

        $this->update($updates);

        // Update all items
        foreach ($this->items as $item) {
            $item->markAsShipped($trackingNumber, $trackingUrl);
        }

        $this->logActivity('Order marked as shipped');

        return $this;
    }

    /**
     * Mark order as delivered
     */
    public function markAsDelivered(): self
    {
        $this->update([
            'status' => self::STATUS_DELIVERED,
            'fulfillment_status' => self::FULFILLMENT_DELIVERED,
            'is_delivered' => true,
            'delivered_at' => now(),
        ]);

        // Update all items
        foreach ($this->items as $item) {
            $item->markAsDelivered();
        }

        $this->logActivity('Order marked as delivered');

        return $this;
    }

    /**
     * Cancel order
     */
    public function cancel(?string $reason = null): self
    {
        if (!$this->canBeCanceled()) {
            throw new InvalidArgumentException('Order cannot be canceled');
        }

        $this->update([
            'status' => self::STATUS_CANCELED,
            'is_canceled' => true,
            'canceled_at' => now(),
            'cancellation_reason' => $reason,
        ]);

        // Restore inventory
        foreach ($this->items as $item) {
            if ($item->inventory_deducted) {
                $item->restoreInventory();
            }
        }

        $this->logActivity("Order canceled: {$reason}");

        return $this;
    }

    /**
     * Mark as canceled (internal)
     */
    protected function markAsCanceled(): void
    {
        if (!$this->is_canceled) {
            $this->is_canceled = true;
            $this->canceled_at = $this->canceled_at ?? now();
        }
    }

    /**
     * Process full refund
     */
    public function refund(?string $reason = null): self
    {
        return $this->refundAmount($this->total, $reason);
    }

    /**
     * Process partial refund
     */
    public function refundAmount(float $amount, ?string $reason = null): self
    {
        if (!$this->canBeRefunded()) {
            throw new InvalidArgumentException('Order cannot be refunded');
        }

        $amount = round($amount, 2);

        if ($amount > ($this->total - $this->refund_amount)) {
            throw new InvalidArgumentException('Refund amount exceeds order total');
        }

        $this->refund_amount = round(($this->refund_amount ?? 0) + $amount, 2);
        
        $isFullRefund = $this->refund_amount >= $this->total;

        $this->update([
            'refund_amount' => $this->refund_amount,
            'payment_status' => $isFullRefund 
                ? self::PAYMENT_REFUNDED 
                : self::PAYMENT_PARTIALLY_REFUNDED,
            'is_refunded' => $isFullRefund,
            'refunded_at' => $isFullRefund ? now() : $this->refunded_at,
            'refund_reason' => $reason,
            'status' => $isFullRefund ? self::STATUS_REFUNDED : $this->status,
        ]);

        // Create refund record
        $this->refunds()->create([
            'amount' => $amount,
            'reason' => $reason,
            'processed_at' => now(),
            'processed_by' => auth()->id(),
        ]);

        $this->logActivity("Refunded {$this->formatMoney($amount)}: {$reason}");

        return $this;
    }

    /**
     * Mark as refunded (internal)
     */
    protected function markAsRefunded(): void
    {
        if (!$this->is_refunded) {
            $this->is_refunded = true;
            $this->refunded_at = $this->refunded_at ?? now();
        }
    }

    /**
     * Check if order can be canceled
     */
    public function canBeCanceled(): bool
    {
        return !$this->is_canceled 
            && !$this->is_shipped 
            && !$this->is_delivered
            && $this->status !== self::STATUS_COMPLETED;
    }

    /**
     * Check if order can be refunded
     */
    public function canBeRefunded(): bool
    {
        return $this->is_paid 
            && !$this->is_canceled
            && ($this->refund_amount < $this->total);
    }

    /**
     * Check if order is paid
     */
    public function isPaid(): bool
    {
        return $this->is_paid || $this->payment_status === self::PAYMENT_PAID;
    }

    /**
     * Check if order is completed
     */
    public function isCompleted(): bool
    {
        return in_array($this->status, [
            self::STATUS_COMPLETED,
            self::STATUS_DELIVERED,
        ], true);
    }

    /**
     * Check if order is pending
     */
    public function isPending(): bool
    {
        return $this->status === self::STATUS_PENDING;
    }

    /**
     * Check if order requires action
     */
    public function requiresAction(): bool
    {
        return in_array($this->status, [
            self::STATUS_PENDING,
            self::STATUS_AWAITING_PAYMENT,
            self::STATUS_ON_HOLD,
        ], true);
    }

    /**
     * Get order age in days
     */
    public function getAgeInDaysAttribute(): int
    {
        return $this->created_at->diffInDays(now());
    }

    /**
     * Get revenue (total minus refunds)
     */
    public function getRevenueAttribute(): float
    {
        return round($this->total - ($this->refund_amount ?? 0), 2);
    }

    /**
     * Get profit (revenue minus costs)
     */
    public function getProfitAttribute(): float
    {
        $itemTotals = OrderItem::calculateTotals($this->items);
        return round($itemTotals['profit'], 2);
    }

    /**
     * Get status color
     */
    public function getStatusColorAttribute(): string
    {
        return match($this->status) {
            self::STATUS_COMPLETED, self::STATUS_DELIVERED => '#22c55e', // green-500
            self::STATUS_PROCESSING, self::STATUS_SHIPPED => '#3b82f6', // blue-500
            self::STATUS_PENDING, self::STATUS_AWAITING_PAYMENT => '#f59e0b', // amber-500
            self::STATUS_ON_HOLD => '#f97316', // orange-500
            self::STATUS_CANCELED => '#6b7280', // gray-500
            self::STATUS_REFUNDED => '#8b5cf6', // purple-500
            self::STATUS_FAILED => '#ef4444', // red-500
            default => '#6b7280',
        };
    }

    /**
     * Format money with currency
     */
    public function formatMoney(float $amount): string
    {
        return match($this->currency) {
            'USD' => '$' . number_format($amount, 2),
            'EUR' => '€' . number_format($amount, 2),
            'GBP' => '£' . number_format($amount, 2),
            default => $this->currency . ' ' . number_format($amount, 2),
        };
    }

    /**
     * Get formatted total
     */
    public function getFormattedTotalAttribute(): string
    {
        return $this->formatMoney($this->total);
    }

    /**
     * Log activity
     */
    protected function logActivity(string $description, array $properties = []): void
    {
        $this->activities()->create([
            'description' => $description,
            'properties' => $properties,
            'causer_id' => auth()->id(),
            'causer_type' => User::class,
        ]);
    }

    /**
     * Scope: Filter by status
     */
    public function scopeWithStatus(Builder $query, string $status): Builder
    {
        return $query->where('status', $status);
    }

    /**
     * Scope: Paid orders
     */
    public function scopePaid(Builder $query): Builder
    {
        return $query->where('is_paid', true);
    }

    /**
     * Scope: Pending orders
     */
    public function scopePending(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_PENDING);
    }

    /**
     * Scope: Completed orders
     */
    public function scopeCompleted(Builder $query): Builder
    {
        return $query->whereIn('status', [
            self::STATUS_COMPLETED,
            self::STATUS_DELIVERED,
        ]);
    }

    /**
     * Scope: Requires action
     */
    public function scopeRequiresAction(Builder $query): Builder
    {
        return $query->whereIn('status', [
            self::STATUS_PENDING,
            self::STATUS_AWAITING_PAYMENT,
            self::STATUS_ON_HOLD,
        ]);
    }

    /**
     * Scope: Recent orders
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    /**
     * Scope: By date range
     */
    public function scopeBetweenDates(Builder $query, string $start, string $end): Builder
    {
        return $query->whereBetween('created_at', [$start, $end]);
    }

    /**
     * Scope: Guest orders
     */
    public function scopeGuest(Builder $query): Builder
    {
        return $query->whereNull('user_id');
    }

    /**
     * Scope: High value orders
     */
    public function scopeHighValue(Builder $query, float $minValue = 1000): Builder
    {
        return $query->where('total', '>=', $minValue);
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(?string $period = null): array
    {
        $query = static::query();

        if ($period) {
            $query->recent((int) filter_var($period, FILTER_SANITIZE_NUMBER_INT));
        }

        return [
            'total_orders' => $query->count(),
            'total_revenue' => round($query->sum('total'), 2),
            'paid_orders' => $query->clone()->paid()->count(),
            'pending_orders' => $query->clone()->pending()->count(),
            'completed_orders' => $query->clone()->completed()->count(),
            'average_order_value' => round($query->avg('total'), 2),
            'total_items_sold' => $query->sum('items_count'),
            'refunded_amount' => round($query->sum('refund_amount'), 2),
        ];
    }

    /**
     * Export to array for API
     */
    public function toOrderArray(bool $includeItems = false): array
    {
        $data = [
            'id' => $this->id,
            'order_number' => $this->order_number,
            'status' => $this->status,
            'status_color' => $this->status_color,
            'payment_status' => $this->payment_status,
            'fulfillment_status' => $this->fulfillment_status,
            'customer' => [
                'name' => $this->customer_name,
                'email' => $this->customer_email,
                'phone' => $this->customer_phone,
            ],
            'amounts' => [
                'subtotal' => $this->subtotal,
                'discount' => $this->discount_amount,
                'tax' => $this->tax,
                'shipping' => $this->shipping_cost,
                'total' => $this->total,
                'formatted_total' => $this->formatted_total,
                'revenue' => $this->revenue,
            ],
            'currency' => $this->currency,
            'items_count' => $this->items_count,
            'is_paid' => $this->is_paid,
            'is_shipped' => $this->is_shipped,
            'tracking_number' => $this->tracking_number,
            'created_at' => $this->created_at->toIso8601String(),
        ];

        if ($includeItems) {
            $data['items'] = $this->items->map->toOrderItemArray();
        }

        return $data;
    }
}