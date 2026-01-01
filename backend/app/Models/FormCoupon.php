<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * FormCoupon Model - FluentForm Pro Coupon System
 *
 * Manages discount coupons for form payments.
 *
 * @property int $id
 * @property int|null $form_id Null means applies to all forms
 * @property string $code Coupon code
 * @property string $discount_type
 * @property float $discount_value
 * @property float|null $min_amount Minimum order amount
 * @property float|null $max_discount Maximum discount amount
 * @property int|null $usage_limit Total usage limit
 * @property int|null $usage_limit_per_user Per user limit
 * @property int $usage_count Current usage count
 * @property bool $stackable Can combine with other coupons
 * @property Carbon|null $starts_at
 * @property Carbon|null $expires_at
 * @property array|null $conditions Additional conditions
 * @property bool $active
 * @property Carbon $created_at
 * @property Carbon $updated_at
 */
class FormCoupon extends Model
{
    use HasFactory;

    protected $table = 'form_coupons';

    protected $fillable = [
        'form_id',
        'code',
        'discount_type',
        'discount_value',
        'min_amount',
        'max_discount',
        'usage_limit',
        'usage_limit_per_user',
        'usage_count',
        'stackable',
        'starts_at',
        'expires_at',
        'conditions',
        'active',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'discount_value' => 'float',
        'min_amount' => 'float',
        'max_discount' => 'float',
        'usage_limit' => 'integer',
        'usage_limit_per_user' => 'integer',
        'usage_count' => 'integer',
        'stackable' => 'boolean',
        'starts_at' => 'datetime',
        'expires_at' => 'datetime',
        'conditions' => 'array',
        'active' => 'boolean',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'discount_type' => 'percentage',
        'usage_count' => 0,
        'stackable' => false,
        'active' => true,
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const TYPE_PERCENTAGE = 'percentage';
    public const TYPE_FIXED = 'fixed';
    public const TYPE_FREE_SHIPPING = 'free_shipping';
    public const TYPE_FREE_TRIAL = 'free_trial';

    public const DISCOUNT_TYPES = [
        self::TYPE_PERCENTAGE => 'Percentage Off',
        self::TYPE_FIXED => 'Fixed Amount Off',
        self::TYPE_FREE_SHIPPING => 'Free Shipping',
        self::TYPE_FREE_TRIAL => 'Free Trial',
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('active', true);
    }

    public function scopeValid(Builder $query): Builder
    {
        return $query->active()
            ->where(function ($q) {
                $q->whereNull('starts_at')->orWhere('starts_at', '<=', now());
            })
            ->where(function ($q) {
                $q->whereNull('expires_at')->orWhere('expires_at', '>', now());
            })
            ->where(function ($q) {
                $q->whereNull('usage_limit')->orWhereRaw('usage_count < usage_limit');
            });
    }

    public function scopeForForm(Builder $query, ?int $formId): Builder
    {
        return $query->where(function ($q) use ($formId) {
            $q->whereNull('form_id')->orWhere('form_id', $formId);
        });
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getIsExpiredAttribute(): bool
    {
        return $this->expires_at && $this->expires_at->isPast();
    }

    public function getIsStartedAttribute(): bool
    {
        return !$this->starts_at || $this->starts_at->isPast();
    }

    public function getIsUsageLimitReachedAttribute(): bool
    {
        return $this->usage_limit && $this->usage_count >= $this->usage_limit;
    }

    public function getIsValidAttribute(): bool
    {
        return $this->active
            && $this->is_started
            && !$this->is_expired
            && !$this->is_usage_limit_reached;
    }

    public function getRemainingUsesAttribute(): ?int
    {
        if (!$this->usage_limit) {
            return null; // Unlimited
        }
        return max(0, $this->usage_limit - $this->usage_count);
    }

    public function getDiscountLabelAttribute(): string
    {
        if ($this->discount_type === self::TYPE_PERCENTAGE) {
            return $this->discount_value . '% Off';
        }
        if ($this->discount_type === self::TYPE_FIXED) {
            return '$' . number_format($this->discount_value, 2) . ' Off';
        }
        return self::DISCOUNT_TYPES[$this->discount_type] ?? 'Discount';
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Validate coupon for an order.
     */
    public function validate(float $orderAmount, ?int $userId = null, ?int $formId = null): array
    {
        $errors = [];

        if (!$this->active) {
            $errors[] = 'This coupon is no longer active.';
        }

        if (!$this->is_started) {
            $errors[] = 'This coupon is not yet active.';
        }

        if ($this->is_expired) {
            $errors[] = 'This coupon has expired.';
        }

        if ($this->is_usage_limit_reached) {
            $errors[] = 'This coupon has reached its usage limit.';
        }

        if ($this->min_amount && $orderAmount < $this->min_amount) {
            $errors[] = "Minimum order amount is $" . number_format($this->min_amount, 2);
        }

        if ($formId && $this->form_id && $this->form_id !== $formId) {
            $errors[] = 'This coupon is not valid for this form.';
        }

        if ($userId && $this->usage_limit_per_user) {
            $userUsage = FormPayment::where('coupon_code', $this->code)
                ->where('user_id', $userId)
                ->whereIn('status', [FormPayment::STATUS_COMPLETED, FormPayment::STATUS_PROCESSING])
                ->count();

            if ($userUsage >= $this->usage_limit_per_user) {
                $errors[] = 'You have already used this coupon the maximum number of times.';
            }
        }

        return [
            'valid' => empty($errors),
            'errors' => $errors,
            'discount' => empty($errors) ? $this->calculateDiscount($orderAmount) : 0,
        ];
    }

    /**
     * Calculate discount amount.
     */
    public function calculateDiscount(float $orderAmount): float
    {
        $discount = 0;

        switch ($this->discount_type) {
            case self::TYPE_PERCENTAGE:
                $discount = $orderAmount * ($this->discount_value / 100);
                break;

            case self::TYPE_FIXED:
                $discount = $this->discount_value;
                break;

            case self::TYPE_FREE_SHIPPING:
            case self::TYPE_FREE_TRIAL:
                $discount = 0; // Handled separately
                break;
        }

        // Apply max discount cap
        if ($this->max_discount && $discount > $this->max_discount) {
            $discount = $this->max_discount;
        }

        // Don't exceed order amount
        return min($discount, $orderAmount);
    }

    /**
     * Record coupon usage.
     */
    public function recordUsage(): void
    {
        $this->increment('usage_count');
    }

    /**
     * Find valid coupon by code.
     */
    public static function findValidByCode(string $code, ?int $formId = null): ?self
    {
        return static::where('code', strtoupper(trim($code)))
            ->valid()
            ->forForm($formId)
            ->first();
    }

    /**
     * Generate unique coupon code.
     */
    public static function generateCode(int $length = 8, string $prefix = ''): string
    {
        do {
            $code = $prefix . strtoupper(substr(str_shuffle('ABCDEFGHJKLMNPQRSTUVWXYZ23456789'), 0, $length));
        } while (static::where('code', $code)->exists());

        return $code;
    }
}
