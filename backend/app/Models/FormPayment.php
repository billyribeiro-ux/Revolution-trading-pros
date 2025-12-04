<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * FormPayment Model - FluentForm Pro Payment Processing
 *
 * Handles payment transactions from form submissions including
 * one-time payments, subscriptions, donations, and product purchases.
 *
 * @property int $id
 * @property int $form_id
 * @property int $submission_id
 * @property int|null $user_id
 * @property string $payment_provider
 * @property string $transaction_id
 * @property string $status
 * @property float $amount
 * @property string $currency
 * @property string|null $payment_method
 * @property string|null $customer_email
 * @property string|null $customer_name
 * @property array|null $billing_address
 * @property array|null $line_items
 * @property array|null $metadata
 * @property string|null $coupon_code
 * @property float|null $discount_amount
 * @property string|null $subscription_id
 * @property string|null $subscription_status
 * @property Carbon|null $subscription_start
 * @property Carbon|null $subscription_end
 * @property string|null $refund_id
 * @property float|null $refund_amount
 * @property string|null $refund_reason
 * @property Carbon|null $refunded_at
 * @property string|null $failure_reason
 * @property Carbon $created_at
 * @property Carbon $updated_at
 */
class FormPayment extends Model
{
    use HasFactory;

    protected $table = 'form_payments';

    protected $fillable = [
        'form_id',
        'submission_id',
        'user_id',
        'payment_provider',
        'transaction_id',
        'status',
        'amount',
        'currency',
        'payment_method',
        'customer_email',
        'customer_name',
        'billing_address',
        'line_items',
        'metadata',
        'coupon_code',
        'discount_amount',
        'subscription_id',
        'subscription_status',
        'subscription_start',
        'subscription_end',
        'refund_id',
        'refund_amount',
        'refund_reason',
        'refunded_at',
        'failure_reason',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'submission_id' => 'integer',
        'user_id' => 'integer',
        'amount' => 'float',
        'discount_amount' => 'float',
        'refund_amount' => 'float',
        'billing_address' => 'array',
        'line_items' => 'array',
        'metadata' => 'array',
        'subscription_start' => 'datetime',
        'subscription_end' => 'datetime',
        'refunded_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    // Payment Providers
    public const PROVIDER_STRIPE = 'stripe';
    public const PROVIDER_PAYPAL = 'paypal';
    public const PROVIDER_SQUARE = 'square';
    public const PROVIDER_RAZORPAY = 'razorpay';
    public const PROVIDER_MOLLIE = 'mollie';
    public const PROVIDER_OFFLINE = 'offline';

    public const PROVIDERS = [
        self::PROVIDER_STRIPE => 'Stripe',
        self::PROVIDER_PAYPAL => 'PayPal',
        self::PROVIDER_SQUARE => 'Square',
        self::PROVIDER_RAZORPAY => 'Razorpay',
        self::PROVIDER_MOLLIE => 'Mollie',
        self::PROVIDER_OFFLINE => 'Offline Payment',
    ];

    // Payment Status
    public const STATUS_PENDING = 'pending';
    public const STATUS_PROCESSING = 'processing';
    public const STATUS_COMPLETED = 'completed';
    public const STATUS_FAILED = 'failed';
    public const STATUS_CANCELLED = 'cancelled';
    public const STATUS_REFUNDED = 'refunded';
    public const STATUS_PARTIALLY_REFUNDED = 'partially_refunded';
    public const STATUS_DISPUTED = 'disputed';

    public const STATUSES = [
        self::STATUS_PENDING,
        self::STATUS_PROCESSING,
        self::STATUS_COMPLETED,
        self::STATUS_FAILED,
        self::STATUS_CANCELLED,
        self::STATUS_REFUNDED,
        self::STATUS_PARTIALLY_REFUNDED,
        self::STATUS_DISPUTED,
    ];

    // Subscription Status
    public const SUB_STATUS_ACTIVE = 'active';
    public const SUB_STATUS_TRIALING = 'trialing';
    public const SUB_STATUS_PAST_DUE = 'past_due';
    public const SUB_STATUS_CANCELLED = 'cancelled';
    public const SUB_STATUS_PAUSED = 'paused';
    public const SUB_STATUS_EXPIRED = 'expired';

    // Payment Methods
    public const METHOD_CARD = 'card';
    public const METHOD_BANK = 'bank_transfer';
    public const METHOD_PAYPAL = 'paypal';
    public const METHOD_APPLE_PAY = 'apple_pay';
    public const METHOD_GOOGLE_PAY = 'google_pay';
    public const METHOD_SEPA = 'sepa_debit';
    public const METHOD_IDEAL = 'ideal';
    public const METHOD_SOFORT = 'sofort';

    // Currencies
    public const CURRENCIES = [
        'USD' => ['symbol' => '$', 'name' => 'US Dollar'],
        'EUR' => ['symbol' => '€', 'name' => 'Euro'],
        'GBP' => ['symbol' => '£', 'name' => 'British Pound'],
        'CAD' => ['symbol' => 'C$', 'name' => 'Canadian Dollar'],
        'AUD' => ['symbol' => 'A$', 'name' => 'Australian Dollar'],
        'JPY' => ['symbol' => '¥', 'name' => 'Japanese Yen'],
        'INR' => ['symbol' => '₹', 'name' => 'Indian Rupee'],
        'BRL' => ['symbol' => 'R$', 'name' => 'Brazilian Real'],
        'MXN' => ['symbol' => 'MX$', 'name' => 'Mexican Peso'],
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    public function submission(): BelongsTo
    {
        return $this->belongsTo(FormSubmission::class, 'submission_id');
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeCompleted(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_COMPLETED);
    }

    public function scopePending(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_PENDING);
    }

    public function scopeFailed(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_FAILED);
    }

    public function scopeForProvider(Builder $query, string $provider): Builder
    {
        return $query->where('payment_provider', $provider);
    }

    public function scopeSubscriptions(Builder $query): Builder
    {
        return $query->whereNotNull('subscription_id');
    }

    public function scopeOneTime(Builder $query): Builder
    {
        return $query->whereNull('subscription_id');
    }

    public function scopeInDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('created_at', [$start, $end]);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getFormattedAmountAttribute(): string
    {
        $currency = self::CURRENCIES[$this->currency] ?? ['symbol' => '$'];
        return $currency['symbol'] . number_format($this->amount, 2);
    }

    public function getNetAmountAttribute(): float
    {
        return $this->amount - ($this->discount_amount ?? 0) - ($this->refund_amount ?? 0);
    }

    public function getIsSubscriptionAttribute(): bool
    {
        return !empty($this->subscription_id);
    }

    public function getIsRefundedAttribute(): bool
    {
        return in_array($this->status, [self::STATUS_REFUNDED, self::STATUS_PARTIALLY_REFUNDED]);
    }

    public function getProviderLabelAttribute(): string
    {
        return self::PROVIDERS[$this->payment_provider] ?? ucfirst($this->payment_provider);
    }

    public function getStatusLabelAttribute(): string
    {
        return ucfirst(str_replace('_', ' ', $this->status));
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Mark payment as completed.
     */
    public function markCompleted(string $transactionId = null): void
    {
        $this->update([
            'status' => self::STATUS_COMPLETED,
            'transaction_id' => $transactionId ?? $this->transaction_id,
        ]);
    }

    /**
     * Mark payment as failed.
     */
    public function markFailed(string $reason = null): void
    {
        $this->update([
            'status' => self::STATUS_FAILED,
            'failure_reason' => $reason,
        ]);
    }

    /**
     * Process refund.
     */
    public function processRefund(float $amount, string $refundId, string $reason = null): void
    {
        $isPartial = $amount < $this->amount;

        $this->update([
            'status' => $isPartial ? self::STATUS_PARTIALLY_REFUNDED : self::STATUS_REFUNDED,
            'refund_id' => $refundId,
            'refund_amount' => ($this->refund_amount ?? 0) + $amount,
            'refund_reason' => $reason,
            'refunded_at' => now(),
        ]);
    }

    /**
     * Update subscription status.
     */
    public function updateSubscriptionStatus(string $status): void
    {
        $this->update(['subscription_status' => $status]);
    }

    /**
     * Apply coupon discount.
     */
    public function applyCoupon(string $code, float $discountAmount): void
    {
        $this->update([
            'coupon_code' => $code,
            'discount_amount' => $discountAmount,
        ]);
    }

    /**
     * Get payment statistics for a form.
     */
    public static function getFormStats(int $formId): array
    {
        $payments = self::where('form_id', $formId)->get();

        return [
            'total_payments' => $payments->count(),
            'completed_payments' => $payments->where('status', self::STATUS_COMPLETED)->count(),
            'total_revenue' => $payments->where('status', self::STATUS_COMPLETED)->sum('amount'),
            'total_refunded' => $payments->sum('refund_amount'),
            'net_revenue' => $payments->where('status', self::STATUS_COMPLETED)->sum('amount') - $payments->sum('refund_amount'),
            'average_payment' => $payments->where('status', self::STATUS_COMPLETED)->avg('amount') ?? 0,
            'subscription_count' => $payments->whereNotNull('subscription_id')->count(),
            'by_provider' => $payments->groupBy('payment_provider')->map->count(),
            'by_status' => $payments->groupBy('status')->map->count(),
        ];
    }
}
