<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\MorphTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use InvalidArgumentException;

/**
 * Order Item Model
 * 
 * Represents individual items within an order with pricing, discounts,
 * taxes, and comprehensive tracking for e-commerce transactions.
 *
 * @property int $id
 * @property int $order_id Parent order
 * @property string $item_type Purchasable type (product, subscription, service, etc.)
 * @property int $item_id Purchasable item ID
 * @property string $name Item name (snapshot at purchase time)
 * @property string|null $sku Item SKU
 * @property string|null $description Item description
 * @property float $price Unit price
 * @property float $original_price Original price before discounts
 * @property int $quantity Quantity ordered
 * @property float $subtotal Line subtotal (price * quantity)
 * @property float $discount_amount Total discount amount
 * @property float|null $discount_percentage Discount percentage
 * @property string|null $discount_code Applied coupon/discount code
 * @property float $tax_amount Tax amount
 * @property float|null $tax_rate Tax rate percentage
 * @property float $shipping_amount Shipping cost for this item
 * @property float $total Line total (subtotal - discount + tax + shipping)
 * @property array $metadata Additional item data
 * @property array $options Product options/variants (size, color, etc.)
 * @property array $custom_fields Custom fields
 * @property string|null $image_url Item image snapshot
 * @property float|null $cost Cost of goods sold (COGS)
 * @property float|null $profit Profit margin
 * @property float|null $weight Item weight
 * @property string|null $weight_unit Weight unit (kg, lb, etc.)
 * @property bool $is_digital Whether item is digital
 * @property bool $is_taxable Whether item is taxable
 * @property bool $is_shippable Whether item requires shipping
 * @property bool $is_subscription Whether item is subscription
 * @property string|null $subscription_interval Subscription interval (monthly, yearly, etc.)
 * @property bool $requires_shipping Whether item needs shipping
 * @property string|null $fulfillment_status Fulfillment status (pending, processing, shipped, delivered, canceled)
 * @property \Illuminate\Support\Carbon|null $fulfilled_at Fulfillment timestamp
 * @property string|null $tracking_number Shipping tracking number
 * @property string|null $tracking_url Tracking URL
 * @property bool $inventory_deducted Whether inventory was deducted
 * @property \Illuminate\Support\Carbon|null $inventory_deducted_at Inventory deduction timestamp
 * @property bool $is_refunded Whether item was refunded
 * @property float|null $refund_amount Refund amount
 * @property \Illuminate\Support\Carbon|null $refunded_at Refund timestamp
 * @property string|null $refund_reason Refund reason
 * @property bool $is_returned Whether item was returned
 * @property \Illuminate\Support\Carbon|null $returned_at Return timestamp
 * @property string|null $return_reason Return reason
 * @property array $download_links Download links for digital products
 * @property int $download_count Number of downloads
 * @property int|null $download_limit Maximum allowed downloads
 * @property \Illuminate\Support\Carbon|null $download_expires_at Download expiration
 * @property string|null $notes Internal notes
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \App\Models\Order $order Parent order
 * @property-read \Illuminate\Database\Eloquent\Model $item Purchasable item (polymorphic)
 */
class OrderItem extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Item types (polymorphic)
     */
    public const TYPE_PRODUCT = 'product';
    public const TYPE_SUBSCRIPTION = 'subscription';
    public const TYPE_SERVICE = 'service';
    public const TYPE_COURSE = 'course';
    public const TYPE_DIGITAL = 'digital';
    public const TYPE_BUNDLE = 'bundle';
    public const TYPE_MEMBERSHIP = 'membership';
    public const TYPE_CUSTOM = 'custom';

    /**
     * Fulfillment statuses
     */
    public const FULFILLMENT_PENDING = 'pending';
    public const FULFILLMENT_PROCESSING = 'processing';
    public const FULFILLMENT_SHIPPED = 'shipped';
    public const FULFILLMENT_DELIVERED = 'delivered';
    public const FULFILLMENT_CANCELED = 'canceled';
    public const FULFILLMENT_RETURNED = 'returned';
    public const FULFILLMENT_REFUNDED = 'refunded';
    public const FULFILLMENT_FAILED = 'failed';

    /**
     * Valid fulfillment statuses
     */
    public const VALID_FULFILLMENT_STATUSES = [
        self::FULFILLMENT_PENDING,
        self::FULFILLMENT_PROCESSING,
        self::FULFILLMENT_SHIPPED,
        self::FULFILLMENT_DELIVERED,
        self::FULFILLMENT_CANCELED,
        self::FULFILLMENT_RETURNED,
        self::FULFILLMENT_REFUNDED,
        self::FULFILLMENT_FAILED,
    ];

    /**
     * Subscription intervals
     */
    public const INTERVAL_DAILY = 'daily';
    public const INTERVAL_WEEKLY = 'weekly';
    public const INTERVAL_MONTHLY = 'monthly';
    public const INTERVAL_QUARTERLY = 'quarterly';
    public const INTERVAL_YEARLY = 'yearly';

    protected $fillable = [
        'order_id',
        'item_type',
        'item_id',
        'name',
        'sku',
        'description',
        'price',
        'original_price',
        'quantity',
        'subtotal',
        'discount_amount',
        'discount_percentage',
        'discount_code',
        'tax_amount',
        'tax_rate',
        'shipping_amount',
        'total',
        'metadata',
        'options',
        'custom_fields',
        'image_url',
        'cost',
        'profit',
        'weight',
        'weight_unit',
        'is_digital',
        'is_taxable',
        'is_shippable',
        'is_subscription',
        'subscription_interval',
        'requires_shipping',
        'fulfillment_status',
        'fulfilled_at',
        'tracking_number',
        'tracking_url',
        'inventory_deducted',
        'inventory_deducted_at',
        'is_refunded',
        'refund_amount',
        'refunded_at',
        'refund_reason',
        'is_returned',
        'returned_at',
        'return_reason',
        'download_links',
        'download_count',
        'download_limit',
        'download_expires_at',
        'notes',
    ];

    protected $casts = [
        'metadata' => 'array',
        'options' => 'array',
        'custom_fields' => 'array',
        'download_links' => 'array',
        'price' => 'decimal:2',
        'original_price' => 'decimal:2',
        'subtotal' => 'decimal:2',
        'discount_amount' => 'decimal:2',
        'discount_percentage' => 'decimal:2',
        'tax_amount' => 'decimal:2',
        'tax_rate' => 'decimal:2',
        'shipping_amount' => 'decimal:2',
        'total' => 'decimal:2',
        'cost' => 'decimal:2',
        'profit' => 'decimal:2',
        'weight' => 'decimal:2',
        'refund_amount' => 'decimal:2',
        'quantity' => 'integer',
        'download_count' => 'integer',
        'download_limit' => 'integer',
        'is_digital' => 'boolean',
        'is_taxable' => 'boolean',
        'is_shippable' => 'boolean',
        'is_subscription' => 'boolean',
        'requires_shipping' => 'boolean',
        'inventory_deducted' => 'boolean',
        'is_refunded' => 'boolean',
        'is_returned' => 'boolean',
        'fulfilled_at' => 'datetime',
        'inventory_deducted_at' => 'datetime',
        'refunded_at' => 'datetime',
        'returned_at' => 'datetime',
        'download_expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'quantity' => 1,
        'subtotal' => 0.0,
        'discount_amount' => 0.0,
        'tax_amount' => 0.0,
        'shipping_amount' => 0.0,
        'total' => 0.0,
        'download_count' => 0,
        'is_digital' => false,
        'is_taxable' => true,
        'is_shippable' => true,
        'is_subscription' => false,
        'requires_shipping' => true,
        'inventory_deducted' => false,
        'is_refunded' => false,
        'is_returned' => false,
        'fulfillment_status' => self::FULFILLMENT_PENDING,
        'metadata' => '[]',
        'options' => '[]',
        'custom_fields' => '[]',
        'download_links' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->calculateAmounts();
            $model->setOriginalPrice();
            $model->extractItemData();
        });

        static::updating(function (self $model): void {
            if ($model->isDirty(['price', 'quantity', 'discount_amount', 'tax_amount', 'shipping_amount'])) {
                $model->calculateAmounts();
            }

            if ($model->isDirty('fulfillment_status')) {
                $model->handleFulfillmentStatusChange();
            }
        });
    }

    /**
     * Get parent order
     */
    public function order(): BelongsTo
    {
        return $this->belongsTo(Order::class);
    }

    /**
     * Get purchasable item (polymorphic)
     */
    public function item(): MorphTo
    {
        return $this->morphTo('item');
    }

    /**
     * Calculate all amounts
     */
    protected function calculateAmounts(): void
    {
        // Calculate subtotal
        $this->subtotal = round($this->price * $this->quantity, 2);

        // Calculate total
        $this->total = round(
            $this->subtotal - $this->discount_amount + $this->tax_amount + $this->shipping_amount,
            2
        );

        // Calculate profit if cost is set
        if ($this->cost !== null) {
            $revenue = $this->subtotal - $this->discount_amount;
            $totalCost = $this->cost * $this->quantity;
            $this->profit = round($revenue - $totalCost, 2);
        }

        // Calculate discount percentage
        if ($this->discount_amount > 0 && $this->subtotal > 0) {
            $this->discount_percentage = round(($this->discount_amount / $this->subtotal) * 100, 2);
        }
    }

    /**
     * Set original price if not set
     */
    protected function setOriginalPrice(): void
    {
        if ($this->original_price === null) {
            $this->original_price = $this->price;
        }
    }

    /**
     * Extract data from purchasable item
     */
    protected function extractItemData(): void
    {
        if (!$this->item) {
            return;
        }

        $item = $this->item;

        // Extract common properties
        if (!$this->name && isset($item->name)) {
            $this->name = $item->name;
        }

        if (!$this->sku && isset($item->sku)) {
            $this->sku = $item->sku;
        }

        if (!$this->description && isset($item->description)) {
            $this->description = $item->description;
        }

        if (!$this->image_url && isset($item->thumbnail)) {
            $this->image_url = $item->thumbnail;
        }

        if ($this->cost === null && isset($item->cost)) {
            $this->cost = $item->cost;
        }

        if ($this->weight === null && isset($item->weight)) {
            $this->weight = $item->weight;
        }

        // Set item type flags
        if (isset($item->is_digital)) {
            $this->is_digital = $item->is_digital;
        }

        if (isset($item->requires_shipping)) {
            $this->requires_shipping = $item->requires_shipping;
        }
    }

    /**
     * Handle fulfillment status changes
     */
    protected function handleFulfillmentStatusChange(): void
    {
        if ($this->fulfillment_status === self::FULFILLMENT_DELIVERED && !$this->fulfilled_at) {
            $this->fulfilled_at = now();
        }
    }

    /**
     * Calculate tax amount
     */
    public function calculateTax(float $taxRate): self
    {
        if (!$this->is_taxable) {
            $this->tax_amount = 0.0;
            $this->tax_rate = null;
            return $this;
        }

        $taxableAmount = $this->subtotal - $this->discount_amount;
        $this->tax_amount = round($taxableAmount * ($taxRate / 100), 2);
        $this->tax_rate = $taxRate;
        $this->calculateAmounts();

        return $this;
    }

    /**
     * Apply discount
     */
    public function applyDiscount(float $amount, ?string $code = null): self
    {
        $this->discount_amount = round($amount, 2);
        $this->discount_code = $code;
        $this->calculateAmounts();

        return $this;
    }

    /**
     * Apply percentage discount
     */
    public function applyPercentageDiscount(float $percentage, ?string $code = null): self
    {
        $discountAmount = round($this->subtotal * ($percentage / 100), 2);
        return $this->applyDiscount($discountAmount, $code);
    }

    /**
     * Set shipping amount
     */
    public function setShippingAmount(float $amount): self
    {
        $this->shipping_amount = round($amount, 2);
        $this->calculateAmounts();

        return $this;
    }

    /**
     * Mark as shipped
     */
    public function markAsShipped(?string $trackingNumber = null, ?string $trackingUrl = null): self
    {
        $this->update([
            'fulfillment_status' => self::FULFILLMENT_SHIPPED,
            'tracking_number' => $trackingNumber,
            'tracking_url' => $trackingUrl,
            'fulfilled_at' => now(),
        ]);

        return $this;
    }

    /**
     * Mark as delivered
     */
    public function markAsDelivered(): self
    {
        $this->update([
            'fulfillment_status' => self::FULFILLMENT_DELIVERED,
            'fulfilled_at' => now(),
        ]);

        return $this;
    }

    /**
     * Process refund
     */
    public function refund(?float $amount = null, ?string $reason = null): self
    {
        $refundAmount = $amount ?? $this->total;

        $this->update([
            'is_refunded' => true,
            'refund_amount' => round($refundAmount, 2),
            'refunded_at' => now(),
            'refund_reason' => $reason,
            'fulfillment_status' => self::FULFILLMENT_REFUNDED,
        ]);

        // Restore inventory if deducted
        if ($this->inventory_deducted && $this->item) {
            $this->restoreInventory();
        }

        return $this;
    }

    /**
     * Process return
     */
    public function processReturn(?string $reason = null): self
    {
        $this->update([
            'is_returned' => true,
            'returned_at' => now(),
            'return_reason' => $reason,
            'fulfillment_status' => self::FULFILLMENT_RETURNED,
        ]);

        // Restore inventory
        if ($this->inventory_deducted && $this->item) {
            $this->restoreInventory();
        }

        return $this;
    }

    /**
     * Deduct inventory
     */
    public function deductInventory(): self
    {
        if ($this->inventory_deducted || !$this->item) {
            return $this;
        }

        if (method_exists($this->item, 'decrementStock')) {
            $this->item->decrementStock($this->quantity);
            
            $this->update([
                'inventory_deducted' => true,
                'inventory_deducted_at' => now(),
            ]);
        }

        return $this;
    }

    /**
     * Restore inventory
     */
    public function restoreInventory(): self
    {
        if (!$this->inventory_deducted || !$this->item) {
            return $this;
        }

        if (method_exists($this->item, 'incrementStock')) {
            $this->item->incrementStock($this->quantity);
            
            $this->update([
                'inventory_deducted' => false,
                'inventory_deducted_at' => null,
            ]);
        }

        return $this;
    }

    /**
     * Record download
     */
    public function recordDownload(): self
    {
        if (!$this->is_digital) {
            return $this;
        }

        // Check download limit
        if ($this->download_limit && $this->download_count >= $this->download_limit) {
            throw new InvalidArgumentException('Download limit exceeded');
        }

        // Check expiration
        if ($this->download_expires_at && $this->download_expires_at->isPast()) {
            throw new InvalidArgumentException('Download link expired');
        }

        $this->increment('download_count');

        return $this;
    }

    /**
     * Check if can be downloaded
     */
    public function canBeDownloaded(): bool
    {
        if (!$this->is_digital) {
            return false;
        }

        // Check limit
        if ($this->download_limit && $this->download_count >= $this->download_limit) {
            return false;
        }

        // Check expiration
        if ($this->download_expires_at && $this->download_expires_at->isPast()) {
            return false;
        }

        // Check order payment
        if (!$this->order->isPaid()) {
            return false;
        }

        return true;
    }

    /**
     * Check if fulfilled
     */
    public function isFulfilled(): bool
    {
        return in_array($this->fulfillment_status, [
            self::FULFILLMENT_SHIPPED,
            self::FULFILLMENT_DELIVERED,
        ], true);
    }

    /**
     * Check if pending fulfillment
     */
    public function isPendingFulfillment(): bool
    {
        return in_array($this->fulfillment_status, [
            self::FULFILLMENT_PENDING,
            self::FULFILLMENT_PROCESSING,
        ], true);
    }

    /**
     * Check if can be refunded
     */
    public function canBeRefunded(): bool
    {
        return !$this->is_refunded && $this->order->isPaid();
    }

    /**
     * Check if can be returned
     */
    public function canBeReturned(): bool
    {
        return !$this->is_returned && $this->isFulfilled();
    }

    /**
     * Get unit price after discount
     */
    public function getUnitPriceAfterDiscountAttribute(): float
    {
        if ($this->quantity === 0) {
            return 0.0;
        }

        return round(($this->subtotal - $this->discount_amount) / $this->quantity, 2);
    }

    /**
     * Get discount per unit
     */
    public function getDiscountPerUnitAttribute(): float
    {
        if ($this->quantity === 0) {
            return 0.0;
        }

        return round($this->discount_amount / $this->quantity, 2);
    }

    /**
     * Get profit margin percentage
     */
    public function getProfitMarginPercentageAttribute(): ?float
    {
        if ($this->profit === null || $this->subtotal <= 0) {
            return null;
        }

        return round(($this->profit / $this->subtotal) * 100, 2);
    }

    /**
     * Get total weight
     */
    public function getTotalWeightAttribute(): ?float
    {
        if ($this->weight === null) {
            return null;
        }

        return round($this->weight * $this->quantity, 2);
    }

    /**
     * Get fulfillment status color
     */
    public function getFulfillmentStatusColorAttribute(): string
    {
        return match($this->fulfillment_status) {
            self::FULFILLMENT_DELIVERED => '#22c55e', // green-500
            self::FULFILLMENT_SHIPPED => '#3b82f6', // blue-500
            self::FULFILLMENT_PROCESSING => '#f59e0b', // amber-500
            self::FULFILLMENT_PENDING => '#6b7280', // gray-500
            self::FULFILLMENT_CANCELED => '#ef4444', // red-500
            self::FULFILLMENT_RETURNED => '#f97316', // orange-500
            self::FULFILLMENT_REFUNDED => '#8b5cf6', // purple-500
            self::FULFILLMENT_FAILED => '#dc2626', // red-600
            default => '#6b7280',
        };
    }

    /**
     * Get fulfillment status label
     */
    public function getFulfillmentStatusLabelAttribute(): string
    {
        return match($this->fulfillment_status) {
            self::FULFILLMENT_DELIVERED => 'Delivered',
            self::FULFILLMENT_SHIPPED => 'Shipped',
            self::FULFILLMENT_PROCESSING => 'Processing',
            self::FULFILLMENT_PENDING => 'Pending',
            self::FULFILLMENT_CANCELED => 'Canceled',
            self::FULFILLMENT_RETURNED => 'Returned',
            self::FULFILLMENT_REFUNDED => 'Refunded',
            self::FULFILLMENT_FAILED => 'Failed',
            default => 'Unknown',
        };
    }

    /**
     * Get metadata value
     */
    public function getMetadataValue(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Set metadata value
     */
    public function setMetadataValue(string $key, mixed $value): self
    {
        $metadata = $this->metadata;
        $metadata[$key] = $value;
        $this->metadata = $metadata;

        return $this;
    }

    /**
     * Scope: Filter by item type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('item_type', $type);
    }

    /**
     * Scope: Digital items
     */
    public function scopeDigital(Builder $query): Builder
    {
        return $query->where('is_digital', true);
    }

    /**
     * Scope: Physical items
     */
    public function scopePhysical(Builder $query): Builder
    {
        return $query->where('is_digital', false);
    }

    /**
     * Scope: Subscription items
     */
    public function scopeSubscriptions(Builder $query): Builder
    {
        return $query->where('is_subscription', true);
    }

    /**
     * Scope: Fulfilled items
     */
    public function scopeFulfilled(Builder $query): Builder
    {
        return $query->whereIn('fulfillment_status', [
            self::FULFILLMENT_SHIPPED,
            self::FULFILLMENT_DELIVERED,
        ]);
    }

    /**
     * Scope: Pending fulfillment
     */
    public function scopePendingFulfillment(Builder $query): Builder
    {
        return $query->whereIn('fulfillment_status', [
            self::FULFILLMENT_PENDING,
            self::FULFILLMENT_PROCESSING,
        ]);
    }

    /**
     * Scope: Refunded items
     */
    public function scopeRefunded(Builder $query): Builder
    {
        return $query->where('is_refunded', true);
    }

    /**
     * Scope: Returned items
     */
    public function scopeReturned(Builder $query): Builder
    {
        return $query->where('is_returned', true);
    }

    /**
     * Scope: With tracking
     */
    public function scopeWithTracking(Builder $query): Builder
    {
        return $query->whereNotNull('tracking_number');
    }

    /**
     * Static: Calculate totals for collection
     */
    public static function calculateTotals(iterable $items): array
    {
        $totals = [
            'subtotal' => 0.0,
            'discount' => 0.0,
            'tax' => 0.0,
            'shipping' => 0.0,
            'total' => 0.0,
            'quantity' => 0,
            'profit' => 0.0,
        ];

        foreach ($items as $item) {
            $totals['subtotal'] += $item->subtotal;
            $totals['discount'] += $item->discount_amount;
            $totals['tax'] += $item->tax_amount;
            $totals['shipping'] += $item->shipping_amount;
            $totals['total'] += $item->total;
            $totals['quantity'] += $item->quantity;
            
            if ($item->profit !== null) {
                $totals['profit'] += $item->profit;
            }
        }

        return array_map(fn($value) => is_float($value) ? round($value, 2) : $value, $totals);
    }

    /**
     * Export to array for API
     */
    public function toOrderItemArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'sku' => $this->sku,
            'description' => $this->description,
            'image_url' => $this->image_url ? url($this->image_url) : null,
            'quantity' => $this->quantity,
            'price' => $this->price,
            'original_price' => $this->original_price,
            'subtotal' => $this->subtotal,
            'discount_amount' => $this->discount_amount,
            'discount_percentage' => $this->discount_percentage,
            'discount_code' => $this->discount_code,
            'tax_amount' => $this->tax_amount,
            'tax_rate' => $this->tax_rate,
            'shipping_amount' => $this->shipping_amount,
            'total' => $this->total,
            'unit_price_after_discount' => $this->unit_price_after_discount,
            'options' => $this->options,
            'is_digital' => $this->is_digital,
            'is_subscription' => $this->is_subscription,
            'fulfillment_status' => $this->fulfillment_status,
            'fulfillment_status_label' => $this->fulfillment_status_label,
            'fulfillment_status_color' => $this->fulfillment_status_color,
            'tracking_number' => $this->tracking_number,
            'tracking_url' => $this->tracking_url,
            'can_download' => $this->canBeDownloaded(),
            'download_count' => $this->download_count,
            'download_limit' => $this->download_limit,
            'created_at' => $this->created_at->toIso8601String(),
        ];
    }
}