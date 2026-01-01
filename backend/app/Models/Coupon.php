<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Carbon\Carbon;
use App\Traits\HasUuid;
use App\Traits\Trackable;
use App\Traits\Versionable;
use App\Traits\HasAnalytics;
use App\Contracts\Discountable;
use App\Contracts\Trackable as TrackableContract;
use App\Events\CouponCreated;
use App\Events\CouponUsed;
use App\Events\CouponExpired;
use App\Events\CouponExhausted;
use App\Events\CouponAbused;
use App\Enums\CouponType;
use App\Enums\CouponStatus;
use App\Enums\DiscountMethod;
use App\Enums\RestrictionType;
use App\Services\Promotions\CouponValidator;
use App\Services\Promotions\DiscountCalculator;
use App\Services\Analytics\CouponAnalyzer;
use App\Exceptions\CouponException;

/**
 * Coupon Model
 * 
 * Enterprise-grade promotional coupon system with advanced validation,
 * analytics, A/B testing, fraud detection, and comprehensive tracking.
 * 
 * @property int $id
 * @property string $uuid
 * @property string $code
 * @property string $type
 * @property float $value
 * @property string $discount_method
 * @property string $display_name
 * @property string|null $description
 * @property string|null $internal_notes
 * @property string|null $terms_and_conditions
 * @property int|null $max_uses
 * @property int $current_uses
 * @property int|null $max_uses_per_customer
 * @property int $unique_customers
 * @property Carbon|null $start_date
 * @property Carbon|null $expiry_date
 * @property float|null $max_discount_amount
 * @property float|null $min_purchase_amount
 * @property array|null $applicable_products
 * @property array|null $applicable_categories
 * @property array|null $applicable_brands
 * @property array|null $applicable_skus
 * @property array|null $excluded_products
 * @property array|null $excluded_categories
 * @property array|null $excluded_brands
 * @property array|null $customer_segments
 * @property array|null $customer_tiers
 * @property array|null $geographic_restrictions
 * @property array|null $device_restrictions
 * @property array|null $payment_method_restrictions
 * @property array|null $shipping_method_restrictions
 * @property array|null $time_restrictions
 * @property array|null $usage_restrictions
 * @property array|null $combination_rules
 * @property bool $is_active
 * @property bool $is_public
 * @property bool $is_stackable
 * @property bool $is_one_time_use
 * @property bool $requires_account
 * @property bool $auto_apply
 * @property bool $is_gift
 * @property int $priority
 * @property string $status
 * @property string|null $campaign_id
 * @property string|null $promotion_id
 * @property string|null $referral_source
 * @property string|null $utm_source
 * @property string|null $utm_medium
 * @property string|null $utm_campaign
 * @property int|null $affiliate_id
 * @property int|null $influencer_id
 * @property int|null $partner_id
 * @property float $commission_rate
 * @property float $total_revenue
 * @property float $total_discount
 * @property float $average_order_value
 * @property float $conversion_rate
 * @property float $roi
 * @property float $fraud_score
 * @property int $fraud_attempts
 * @property array|null $fraud_indicators
 * @property array|null $ab_test
 * @property string|null $ab_test_variant
 * @property array|null $tiers
 * @property array|null $dynamic_rules
 * @property array|null $personalization_rules
 * @property array|null $gamification_rules
 * @property array|null $notification_rules
 * @property array|null $tags
 * @property array|null $metadata
 * @property array|null $analytics_data
 * @property int $created_by
 * @property int|null $updated_by
 * @property int|null $approved_by
 * @property Carbon|null $approved_at
 * @property int $version
 * @property string|null $previous_version_id
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\Campaign|null $campaign
 * @property-read \App\Models\Promotion|null $promotion
 * @property-read \App\Models\Affiliate|null $affiliate
 * @property-read \App\Models\Influencer|null $influencer
 * @property-read \App\Models\Partner|null $partner
 * @property-read \App\Models\User $creator
 * @property-read \App\Models\User|null $updater
 * @property-read \App\Models\User|null $approver
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CouponUsage[] $usages
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Order[] $orders
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Customer[] $customers
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Product[] $products
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Category[] $categories
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CouponVersion[] $versions
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CouponAnalytics[] $analytics
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\FraudAttempt[] $fraudAttempts
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\ABTestResult[] $abTestResults
 */
class Coupon extends Model implements Discountable, TrackableContract
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Trackable;
    use Versionable;
    use HasAnalytics;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'coupons';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'code',
        'type',
        'value',
        'discount_method',
        'display_name',
        'description',
        'internal_notes',
        'terms_and_conditions',
        'max_uses',
        'current_uses',
        'max_uses_per_customer',
        'unique_customers',
        'start_date',
        'expiry_date',
        'max_discount_amount',
        'min_purchase_amount',
        'applicable_products',
        'applicable_categories',
        'applicable_brands',
        'applicable_skus',
        'excluded_products',
        'excluded_categories',
        'excluded_brands',
        'customer_segments',
        'customer_tiers',
        'geographic_restrictions',
        'device_restrictions',
        'payment_method_restrictions',
        'shipping_method_restrictions',
        'time_restrictions',
        'usage_restrictions',
        'combination_rules',
        'is_active',
        'is_public',
        'is_stackable',
        'is_one_time_use',
        'requires_account',
        'auto_apply',
        'is_gift',
        'priority',
        'status',
        'campaign_id',
        'promotion_id',
        'referral_source',
        'utm_source',
        'utm_medium',
        'utm_campaign',
        'affiliate_id',
        'influencer_id',
        'partner_id',
        'commission_rate',
        'total_revenue',
        'total_discount',
        'average_order_value',
        'conversion_rate',
        'roi',
        'fraud_score',
        'fraud_attempts',
        'fraud_indicators',
        'ab_test',
        'ab_test_variant',
        'tiers',
        'dynamic_rules',
        'personalization_rules',
        'gamification_rules',
        'notification_rules',
        'tags',
        'metadata',
        'analytics_data',
        'created_by',
        'updated_by',
        'approved_by',
        'approved_at',
        'version',
        'previous_version_id',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'value' => 'decimal:2',
        'max_discount_amount' => 'decimal:2',
        'min_purchase_amount' => 'decimal:2',
        'commission_rate' => 'decimal:2',
        'total_revenue' => 'decimal:2',
        'total_discount' => 'decimal:2',
        'average_order_value' => 'decimal:2',
        'conversion_rate' => 'decimal:4',
        'roi' => 'decimal:2',
        'fraud_score' => 'decimal:2',
        'max_uses' => 'integer',
        'current_uses' => 'integer',
        'max_uses_per_customer' => 'integer',
        'unique_customers' => 'integer',
        'fraud_attempts' => 'integer',
        'priority' => 'integer',
        'version' => 'integer',
        'is_active' => 'boolean',
        'is_public' => 'boolean',
        'is_stackable' => 'boolean',
        'is_one_time_use' => 'boolean',
        'requires_account' => 'boolean',
        'auto_apply' => 'boolean',
        'is_gift' => 'boolean',
        'type' => CouponType::class,
        'status' => CouponStatus::class,
        'discount_method' => DiscountMethod::class,
        'start_date' => 'datetime',
        'expiry_date' => 'datetime',
        'approved_at' => 'datetime',
        'applicable_products' => 'array',
        'applicable_categories' => 'array',
        'applicable_brands' => 'array',
        'applicable_skus' => 'array',
        'excluded_products' => 'array',
        'excluded_categories' => 'array',
        'excluded_brands' => 'array',
        'customer_segments' => 'array',
        'customer_tiers' => 'array',
        'geographic_restrictions' => 'array',
        'device_restrictions' => 'array',
        'payment_method_restrictions' => 'array',
        'shipping_method_restrictions' => 'array',
        'time_restrictions' => 'array',
        'usage_restrictions' => 'array',
        'combination_rules' => 'array',
        'fraud_indicators' => 'array',
        'ab_test' => 'array',
        'tiers' => 'array',
        'dynamic_rules' => 'array',
        'personalization_rules' => 'array',
        'gamification_rules' => 'array',
        'notification_rules' => 'array',
        'tags' => 'array',
        'metadata' => 'array',
        'analytics_data' => 'array',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'internal_notes',
        'fraud_indicators',
        'deleted_at',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_valid',
        'is_expired',
        'is_exhausted',
        'remaining_uses',
        'usage_percentage',
        'days_until_expiry',
        'effectiveness_score',
        'discount_display',
        'restrictions_summary',
        'performance_rating',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'type' => CouponType::STANDARD,
        'discount_method' => DiscountMethod::PERCENTAGE,
        'current_uses' => 0,
        'unique_customers' => 0,
        'total_revenue' => 0,
        'total_discount' => 0,
        'fraud_score' => 0,
        'fraud_attempts' => 0,
        'priority' => 0,
        'version' => 1,
        'is_active' => true,
        'is_public' => false,
        'is_stackable' => false,
        'is_one_time_use' => false,
        'requires_account' => false,
        'auto_apply' => false,
        'is_gift' => false,
        'status' => CouponStatus::ACTIVE,
    ];

    /**
     * Cache configuration.
     *
     * @var array
     */
    protected static array $cacheConfig = [
        'tags' => ['coupons'],
        'ttl' => 3600,
    ];

    /**
     * Boot the model.
     *
     * @return void
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $coupon) {
            $coupon->uuid = $coupon->uuid ?? (string) Str::uuid();
            $coupon->created_by = $coupon->created_by ?? auth()->id();
            
            if (!$coupon->code) {
                $coupon->code = $coupon->generateUniqueCode();
            }
            
            $coupon->validateConfiguration();
            $coupon->calculatePriority();
        });

        static::created(function (self $coupon) {
            event(new CouponCreated($coupon));
            $coupon->createInitialAnalytics();
            $coupon->scheduleExpirationCheck();
        });

        static::updating(function (self $coupon) {
            $coupon->updated_by = auth()->id();
            
            if ($coupon->isDirty(['type', 'value', 'applicable_products', 'applicable_categories'])) {
                $coupon->incrementVersion();
            }
            
            if ($coupon->isDirty('current_uses') && $coupon->isExhausted()) {
                $coupon->status = CouponStatus::EXHAUSTED;
            }
            
            $coupon->validateConfiguration();
        });

        static::updated(function (self $coupon) {
            $coupon->clearCache();
            
            if ($coupon->wasChanged('current_uses')) {
                $coupon->updateAnalytics();
                
                if ($coupon->isExhausted()) {
                    event(new CouponExhausted($coupon));
                }
            }
            
            if ($coupon->wasChanged('status') && $coupon->status === CouponStatus::EXPIRED) {
                event(new CouponExpired($coupon));
            }
        });

        static::deleting(function (self $coupon) {
            if ($coupon->hasActiveUsages()) {
                throw new CouponException('Cannot delete coupon with active usages');
            }
        });
    }

    /**
     * Get the campaign relationship.
     */
    public function campaign(): BelongsTo
    {
        return $this->belongsTo(Campaign::class);
    }

    /**
     * Get the promotion relationship.
     */
    public function promotion(): BelongsTo
    {
        return $this->belongsTo(Promotion::class);
    }

    /**
     * Get the affiliate relationship.
     */
    public function affiliate(): BelongsTo
    {
        return $this->belongsTo(Affiliate::class);
    }

    /**
     * Get the influencer relationship.
     */
    public function influencer(): BelongsTo
    {
        return $this->belongsTo(Influencer::class);
    }

    /**
     * Get the partner relationship.
     */
    public function partner(): BelongsTo
    {
        return $this->belongsTo(Partner::class);
    }

    /**
     * Get the creator relationship.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get the updater relationship.
     */
    public function updater(): BelongsTo
    {
        return $this->belongsTo(User::class, 'updated_by');
    }

    /**
     * Get the approver relationship.
     */
    public function approver(): BelongsTo
    {
        return $this->belongsTo(User::class, 'approved_by');
    }

    /**
     * Get the coupon usages.
     */
    public function usages(): HasMany
    {
        return $this->hasMany(CouponUsage::class)
            ->orderBy('used_at', 'desc');
    }

    /**
     * Get the orders that used this coupon.
     */
    public function orders(): HasManyThrough
    {
        return $this->hasManyThrough(
            Order::class,
            CouponUsage::class,
            'coupon_id',
            'id',
            'id',
            'order_id'
        );
    }

    /**
     * Get the customers who used this coupon.
     */
    public function customers(): BelongsToMany
    {
        return $this->belongsToMany(Customer::class, 'coupon_usages')
            ->withTimestamps()
            ->withPivot(['order_id', 'discount_amount', 'used_at']);
    }

    /**
     * Get applicable products.
     */
    public function products(): BelongsToMany
    {
        return $this->belongsToMany(Product::class, 'coupon_products')
            ->withTimestamps();
    }

    /**
     * Get applicable categories.
     */
    public function categories(): BelongsToMany
    {
        return $this->belongsToMany(Category::class, 'coupon_categories')
            ->withTimestamps();
    }

    /**
     * Get coupon versions.
     */
    public function versions(): HasMany
    {
        return $this->hasMany(CouponVersion::class)
            ->orderBy('version', 'desc');
    }

    /**
     * Get analytics records.
     */
    public function analytics(): HasMany
    {
        return $this->hasMany(CouponAnalytics::class)
            ->orderBy('recorded_at', 'desc');
    }

    /**
     * Get fraud attempts.
     */
    public function fraudAttempts(): HasMany
    {
        return $this->hasMany(FraudAttempt::class)
            ->orderBy('attempted_at', 'desc');
    }

    /**
     * Get A/B test results.
     */
    public function abTestResults(): HasMany
    {
        return $this->hasMany(ABTestResult::class);
    }

    /**
     * Get activity logs.
     */
    public function activities(): MorphMany
    {
        return $this->morphMany(Activity::class, 'subject');
    }

    /**
     * Scope for active coupons.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->where('status', CouponStatus::ACTIVE)
            ->where(function ($q) {
                $q->whereNull('start_date')
                    ->orWhere('start_date', '<=', now());
            })
            ->where(function ($q) {
                $q->whereNull('expiry_date')
                    ->orWhere('expiry_date', '>', now());
            });
    }

    /**
     * Scope for expired coupons.
     */
    public function scopeExpired(Builder $query): Builder
    {
        return $query->where('expiry_date', '<=', now())
            ->orWhere('status', CouponStatus::EXPIRED);
    }

    /**
     * Scope for exhausted coupons.
     */
    public function scopeExhausted(Builder $query): Builder
    {
        return $query->whereNotNull('max_uses')
            ->whereColumn('current_uses', '>=', 'max_uses')
            ->orWhere('status', CouponStatus::EXHAUSTED);
    }

    /**
     * Scope for public coupons.
     */
    public function scopePublic(Builder $query): Builder
    {
        return $query->where('is_public', true)
            ->active();
    }

    /**
     * Scope for stackable coupons.
     */
    public function scopeStackable(Builder $query): Builder
    {
        return $query->where('is_stackable', true);
    }

    /**
     * Scope for auto-apply coupons.
     */
    public function scopeAutoApply(Builder $query): Builder
    {
        return $query->where('auto_apply', true)
            ->active();
    }

    /**
     * Scope for high-performing coupons.
     */
    public function scopeHighPerforming(Builder $query, float $minRoi = 2.0): Builder
    {
        return $query->where('roi', '>=', $minRoi)
            ->where('conversion_rate', '>=', 0.1)
            ->orderBy('roi', 'desc');
    }

    /**
     * Get validity status.
     */
    protected function isValid(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->validateCoupon()
        );
    }

    /**
     * Get expiration status.
     */
    protected function isExpired(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->expiry_date && $this->expiry_date->isPast()
        );
    }

    /**
     * Get exhaustion status.
     */
    protected function isExhausted(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->max_uses && $this->current_uses >= $this->max_uses
        );
    }

    /**
     * Get remaining uses.
     */
    protected function remainingUses(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->max_uses ? max(0, $this->max_uses - $this->current_uses) : null
        );
    }

    /**
     * Get usage percentage.
     */
    protected function usagePercentage(): Attribute
    {
        return Attribute::make(
            get: function () {
                if (!$this->max_uses) return 0;
                return round(($this->current_uses / $this->max_uses) * 100, 2);
            }
        );
    }

    /**
     * Get days until expiry.
     */
    protected function daysUntilExpiry(): Attribute
    {
        return Attribute::make(
            get: function () {
                if (!$this->expiry_date) return null;
                return max(0, now()->diffInDays($this->expiry_date, false));
            }
        );
    }

    /**
     * Get effectiveness score.
     */
    protected function effectivenessScore(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculateEffectivenessScore()
        );
    }

    /**
     * Get discount display string.
     */
    protected function discountDisplay(): Attribute
    {
        return Attribute::make(
            get: function () {
                if ($this->discount_method === DiscountMethod::PERCENTAGE) {
                    return $this->value . '% off';
                } elseif ($this->discount_method === DiscountMethod::FIXED) {
                    return '$' . number_format($this->value, 2) . ' off';
                } elseif ($this->discount_method === DiscountMethod::BUY_X_GET_Y) {
                    return $this->metadata['buy_x_get_y_text'] ?? 'Special offer';
                } else {
                    return 'Free shipping';
                }
            }
        );
    }

    /**
     * Get restrictions summary.
     */
    protected function restrictionsSummary(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->generateRestrictionsSummary()
        );
    }

    /**
     * Get performance rating.
     */
    protected function performanceRating(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculatePerformanceRating()
        );
    }

    /**
     * Validate coupon for use.
     */
    public function validateCoupon(?Customer $customer = null, ?Cart $cart = null): array
    {
        $validator = app(CouponValidator::class);
        return $validator->validate($this, $customer, $cart);
    }

    /**
     * Apply coupon to cart.
     */
    public function applyToCart(Cart $cart, Customer $customer): float
    {
        if (!$this->is_valid) {
            throw new CouponException('Coupon is not valid');
        }

        $calculator = app(DiscountCalculator::class);
        $discount = $calculator->calculate($this, $cart);

        // Record usage
        $this->recordUsage($customer, $cart->order_id, $discount);

        return $discount;
    }

    /**
     * Record coupon usage.
     */
    public function recordUsage(Customer $customer, int $orderId, float $discountAmount): CouponUsage
    {
        DB::transaction(function () use ($customer, $orderId, $discountAmount) {
            $usage = $this->usages()->create([
                'customer_id' => $customer->id,
                'order_id' => $orderId,
                'discount_amount' => $discountAmount,
                'used_at' => now(),
                'ip_address' => request()->ip(),
                'user_agent' => request()->userAgent(),
                'device_type' => $this->detectDeviceType(),
                'location' => $this->getLocationFromIp(),
            ]);

            $this->increment('current_uses');
            
            if (!$this->customers()->where('customer_id', $customer->id)->exists()) {
                $this->increment('unique_customers');
            }

            $this->updateRevenue($discountAmount, $orderId);
            
            event(new CouponUsed($this, $usage));

            return $usage;
        });
    }

    /**
     * Generate unique code.
     */
    protected function generateUniqueCode(): string
    {
        do {
            $code = match ($this->type) {
                CouponType::GIFT => 'GIFT-' . Str::random(8),
                CouponType::REFERRAL => 'REF-' . Str::random(6),
                CouponType::SEASONAL => 'SEASON-' . Str::random(6),
                CouponType::FLASH => 'FLASH-' . Str::random(4),
                default => Str::upper(Str::random(8)),
            };
        } while (static::where('code', $code)->exists());

        return $code;
    }

    /**
     * Validate configuration.
     */
    protected function validateConfiguration(): void
    {
        if ($this->start_date && $this->expiry_date && $this->start_date->gt($this->expiry_date)) {
            throw new CouponException('Start date cannot be after expiry date');
        }

        if ($this->max_discount_amount && $this->discount_method === DiscountMethod::PERCENTAGE) {
            if ($this->value > 100) {
                throw new CouponException('Percentage discount cannot exceed 100%');
            }
        }

        if ($this->is_stackable && $this->priority === 0) {
            throw new CouponException('Stackable coupons must have a priority');
        }
    }

    /**
     * Calculate priority based on rules.
     */
    protected function calculatePriority(): void
    {
        if ($this->priority > 0) return;

        $priority = 0;

        // Higher value coupons get higher priority
        $priority += min(10, $this->value / 10);

        // Limited use coupons get priority
        if ($this->max_uses && $this->max_uses < 100) {
            $priority += 5;
        }

        // Time-limited coupons get priority
        if ($this->expiry_date && $this->days_until_expiry < 7) {
            $priority += 3;
        }

        // Exclusive coupons get highest priority
        if (!$this->is_stackable) {
            $priority += 20;
        }

        $this->priority = (int) $priority;
    }

    /**
     * Calculate effectiveness score.
     */
    protected function calculateEffectivenessScore(): float
    {
        $score = 0;

        // Conversion rate (40% weight)
        $score += ($this->conversion_rate * 100) * 0.4;

        // ROI (30% weight)
        $roiScore = min(100, ($this->roi / 5) * 100);
        $score += $roiScore * 0.3;

        // Usage rate (20% weight)
        if ($this->max_uses) {
            $usageRate = ($this->current_uses / $this->max_uses) * 100;
            $score += $usageRate * 0.2;
        } else {
            $score += 20; // Full points if unlimited
        }

        // Average order value impact (10% weight)
        $aovImpact = min(100, ($this->average_order_value / 100) * 100);
        $score += $aovImpact * 0.1;

        return round(min(100, $score), 2);
    }

    /**
     * Generate restrictions summary.
     */
    protected function generateRestrictionsSummary(): array
    {
        $summary = [];

        if ($this->min_purchase_amount) {
            $summary[] = 'Minimum purchase: $' . number_format($this->min_purchase_amount, 2);
        }

        if ($this->applicable_products) {
            $summary[] = 'Valid for ' . count($this->applicable_products) . ' products';
        }

        if ($this->applicable_categories) {
            $summary[] = 'Valid for ' . count($this->applicable_categories) . ' categories';
        }

        if ($this->customer_segments) {
            $summary[] = 'Customer segments: ' . implode(', ', $this->customer_segments);
        }

        if ($this->geographic_restrictions) {
            $summary[] = 'Geographic restrictions apply';
        }

        if ($this->max_uses_per_customer) {
            $summary[] = 'Limit ' . $this->max_uses_per_customer . ' per customer';
        }

        if ($this->requires_account) {
            $summary[] = 'Account required';
        }

        return $summary;
    }

    /**
     * Calculate performance rating.
     */
    protected function calculatePerformanceRating(): string
    {
        $score = $this->effectiveness_score;

        return match(true) {
            $score >= 80 => 'Excellent',
            $score >= 60 => 'Good',
            $score >= 40 => 'Average',
            $score >= 20 => 'Below Average',
            default => 'Poor',
        };
    }

    /**
     * Check for fraud indicators.
     */
    public function checkForFraud(Customer $customer, string $ipAddress): bool
    {
        $indicators = [];
        
        // Check usage velocity
        $recentUsages = $this->usages()
            ->where('customer_id', $customer->id)
            ->where('used_at', '>=', now()->subHours(1))
            ->count();
        
        if ($recentUsages > 3) {
            $indicators[] = 'high_velocity';
        }

        // Check IP patterns
        $ipUsages = $this->usages()
            ->where('ip_address', $ipAddress)
            ->where('customer_id', '!=', $customer->id)
            ->count();
        
        if ($ipUsages > 5) {
            $indicators[] = 'suspicious_ip';
        }

        // Check for account age
        if ($customer->created_at->diffInDays() < 1) {
            $indicators[] = 'new_account';
        }

        if (!empty($indicators)) {
            $this->recordFraudAttempt($customer, $indicators);
            return true;
        }

        return false;
    }

    /**
     * Record fraud attempt.
     */
    protected function recordFraudAttempt(Customer $customer, array $indicators): void
    {
        $this->fraudAttempts()->create([
            'customer_id' => $customer->id,
            'indicators' => $indicators,
            'ip_address' => request()->ip(),
            'attempted_at' => now(),
        ]);

        $this->increment('fraud_attempts');
        
        $this->fraud_score = min(100, $this->fraud_score + (count($indicators) * 10));
        $this->fraud_indicators = array_merge($this->fraud_indicators ?? [], $indicators);
        $this->save();

        if ($this->fraud_score > 70) {
            event(new CouponAbused($this));
        }
    }

    /**
     * Update revenue metrics.
     */
    protected function updateRevenue(float $discountAmount, int $orderId): void
    {
        $order = Order::find($orderId);
        
        if ($order) {
            $this->total_revenue += $order->total;
            $this->total_discount += $discountAmount;
            
            // Recalculate average order value
            $this->average_order_value = $this->orders()->avg('total') ?? 0;
            
            // Calculate conversion rate
            $views = $this->metadata['view_count'] ?? 1;
            $this->conversion_rate = $this->current_uses / max(1, $views);
            
            // Calculate ROI
            $this->roi = $this->total_discount > 0 
                ? ($this->total_revenue - $this->total_discount) / $this->total_discount
                : 0;
            
            $this->save();
        }
    }

    /**
     * Create initial analytics.
     */
    protected function createInitialAnalytics(): void
    {
        $this->analytics()->create([
            'recorded_at' => now(),
            'metrics' => [
                'status' => 'created',
                'potential_value' => $this->estimatePotentialValue(),
                'target_audience_size' => $this->estimateAudienceSize(),
            ],
        ]);
    }

    /**
     * Update analytics.
     */
    protected function updateAnalytics(): void
    {
        $analyzer = app(CouponAnalyzer::class);
        $metrics = $analyzer->analyze($this);
        
        $this->analytics()->create([
            'recorded_at' => now(),
            'metrics' => $metrics,
        ]);
    }

    /**
     * Schedule expiration check.
     */
    protected function scheduleExpirationCheck(): void
    {
        if ($this->expiry_date) {
            dispatch(function () {
                if ($this->expiry_date->isPast()) {
                    $this->expire();
                }
            })->delay($this->expiry_date);
        }
    }

    /**
     * Expire the coupon.
     */
    public function expire(): void
    {
        $this->update([
            'status' => CouponStatus::EXPIRED,
            'is_active' => false,
        ]);
    }

    /**
     * Clone coupon for A/B testing.
     */
    public function cloneForABTest(array $variations): Collection
    {
        return collect($variations)->map(function ($variation, $variant) {
            $clone = $this->replicate();
            $clone->fill($variation);
            $clone->code = $this->code . '-' . strtoupper($variant);
            $clone->ab_test_variant = $variant;
            $clone->ab_test = [
                'original_id' => $this->id,
                'variant' => $variant,
                'started_at' => now(),
            ];
            $clone->save();
            
            return $clone;
        });
    }

    /**
     * Get best performing variant.
     */
    public function getBestPerformingVariant(): ?self
    {
        if (!$this->ab_test) return null;
        
        return static::where('ab_test->original_id', $this->id)
            ->orWhere('id', $this->id)
            ->orderBy('effectiveness_score', 'desc')
            ->first();
    }

    /**
     * Clear cache.
     */
    protected function clearCache(): void
    {
        Cache::tags(self::$cacheConfig['tags'])->forget("coupon_{$this->id}");
        Cache::tags(self::$cacheConfig['tags'])->forget("coupon_code_{$this->code}");
    }
}