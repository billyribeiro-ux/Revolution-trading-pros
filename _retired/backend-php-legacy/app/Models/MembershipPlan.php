<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;

/**
 * Membership Plan Model
 * 
 * Manages membership/subscription plans with tiered pricing, features,
 * trial periods, and comprehensive billing configurations.
 *
 * @property int $id
 * @property string $name Plan name
 * @property string $slug URL-friendly slug
 * @property string|null $description Plan description
 * @property string|null $tagline Short tagline/subtitle
 * @property float $price Monthly/base price
 * @property float|null $annual_price Annual price (if different)
 * @property float|null $setup_fee One-time setup fee
 * @property string $billing_cycle Billing cycle (monthly, yearly, quarterly, lifetime, custom)
 * @property string $currency Currency code (USD, EUR, etc.)
 * @property bool $is_active Whether plan is active/published
 * @property bool $is_featured Featured plan flag
 * @property bool $is_popular Popular/recommended flag
 * @property bool $is_custom Custom/enterprise plan
 * @property int $sort_order Display sort order
 * @property array $metadata Additional plan data
 * @property array $features Plan features/benefits
 * @property array $limitations Plan limitations/restrictions
 * @property array $pricing_tiers Volume/usage-based pricing tiers
 * @property int|null $max_users Maximum users allowed
 * @property int|null $max_projects Maximum projects allowed
 * @property int|null $max_storage Maximum storage (MB)
 * @property int|null $max_api_calls Maximum API calls per month
 * @property array $access_levels Access level permissions
 * @property bool $has_trial Trial period available
 * @property int|null $trial_days Trial period duration (days)
 * @property bool $trial_requires_payment Trial requires payment method
 * @property string|null $stripe_price_id Stripe price ID
 * @property string|null $stripe_product_id Stripe product ID
 * @property string|null $paypal_plan_id PayPal plan ID
 * @property array $payment_providers Enabled payment providers
 * @property bool $allow_upgrades Allow upgrade to higher plans
 * @property bool $allow_downgrades Allow downgrade to lower plans
 * @property bool $allow_cancellation Allow plan cancellation
 * @property string|null $cancellation_policy Cancellation policy text
 * @property bool $prorate_on_change Prorate on plan changes
 * @property int|null $grace_period_days Grace period after cancellation (days)
 * @property array $included_features Included feature IDs/codes
 * @property array $addon_features Available addon features
 * @property int $active_subscriptions_count Active subscription count
 * @property int $total_subscriptions_count Total subscriptions (all time)
 * @property float $total_revenue Total revenue generated
 * @property float $mrr Monthly recurring revenue
 * @property float $arr Annual recurring revenue
 * @property float|null $churn_rate Churn rate percentage
 * @property \Illuminate\Support\Carbon|null $available_from Plan availability start date
 * @property \Illuminate\Support\Carbon|null $available_until Plan availability end date
 * @property string|null $upgrade_to_plan_id Suggested upgrade plan
 * @property string|null $downgrade_to_plan_id Suggested downgrade plan
 * @property array $comparison_highlights Key comparison points
 * @property string|null $cta_text Call-to-action button text
 * @property string|null $cta_url Custom CTA URL
 * @property array $terms_conditions Terms and conditions
 * @property string|null $created_by User who created plan
 * @property string|null $updated_by User who last updated plan
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \Illuminate\Database\Eloquent\Collection $planFeatures Feature details
 * @property-read \Illuminate\Database\Eloquent\Collection $subscriptions User subscriptions
 * @property-read \Illuminate\Database\Eloquent\Collection $addons Available addons
 */
class MembershipPlan extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Billing cycles
     */
    public const BILLING_MONTHLY = 'monthly';
    public const BILLING_QUARTERLY = 'quarterly';
    public const BILLING_SEMI_ANNUAL = 'semi_annual';
    public const BILLING_YEARLY = 'yearly';
    public const BILLING_LIFETIME = 'lifetime';
    public const BILLING_CUSTOM = 'custom';

    /**
     * Valid billing cycles
     */
    public const VALID_BILLING_CYCLES = [
        self::BILLING_MONTHLY,
        self::BILLING_QUARTERLY,
        self::BILLING_SEMI_ANNUAL,
        self::BILLING_YEARLY,
        self::BILLING_LIFETIME,
        self::BILLING_CUSTOM,
    ];

    /**
     * Plan tiers
     */
    public const TIER_FREE = 'free';
    public const TIER_STARTER = 'starter';
    public const TIER_PROFESSIONAL = 'professional';
    public const TIER_BUSINESS = 'business';
    public const TIER_ENTERPRISE = 'enterprise';

    /**
     * Default currency
     */
    public const DEFAULT_CURRENCY = 'USD';

    /**
     * Cache TTL
     */
    public const CACHE_TTL = 3600;

    protected $fillable = [
        'name',
        'slug',
        'description',
        'tagline',
        'price',
        'annual_price',
        'setup_fee',
        'billing_cycle',
        'currency',
        'is_active',
        'is_featured',
        'is_popular',
        'is_custom',
        'sort_order',
        'metadata',
        'features',
        'limitations',
        'pricing_tiers',
        'max_users',
        'max_projects',
        'max_storage',
        'max_api_calls',
        'access_levels',
        'has_trial',
        'trial_days',
        'trial_requires_payment',
        'stripe_price_id',
        'stripe_product_id',
        'paypal_plan_id',
        'payment_providers',
        'allow_upgrades',
        'allow_downgrades',
        'allow_cancellation',
        'cancellation_policy',
        'prorate_on_change',
        'grace_period_days',
        'included_features',
        'addon_features',
        'active_subscriptions_count',
        'total_subscriptions_count',
        'total_revenue',
        'mrr',
        'arr',
        'churn_rate',
        'available_from',
        'available_until',
        'upgrade_to_plan_id',
        'downgrade_to_plan_id',
        'comparison_highlights',
        'cta_text',
        'cta_url',
        'terms_conditions',
        'created_by',
        'updated_by',
    ];

    protected $casts = [
        'metadata' => 'array',
        'features' => 'array',
        'limitations' => 'array',
        'pricing_tiers' => 'array',
        'access_levels' => 'array',
        'payment_providers' => 'array',
        'included_features' => 'array',
        'addon_features' => 'array',
        'comparison_highlights' => 'array',
        'terms_conditions' => 'array',
        'price' => 'decimal:2',
        'annual_price' => 'decimal:2',
        'setup_fee' => 'decimal:2',
        'total_revenue' => 'decimal:2',
        'mrr' => 'decimal:2',
        'arr' => 'decimal:2',
        'churn_rate' => 'decimal:2',
        'max_users' => 'integer',
        'max_projects' => 'integer',
        'max_storage' => 'integer',
        'max_api_calls' => 'integer',
        'trial_days' => 'integer',
        'grace_period_days' => 'integer',
        'active_subscriptions_count' => 'integer',
        'total_subscriptions_count' => 'integer',
        'sort_order' => 'integer',
        'is_active' => 'boolean',
        'is_featured' => 'boolean',
        'is_popular' => 'boolean',
        'is_custom' => 'boolean',
        'has_trial' => 'boolean',
        'trial_requires_payment' => 'boolean',
        'allow_upgrades' => 'boolean',
        'allow_downgrades' => 'boolean',
        'allow_cancellation' => 'boolean',
        'prorate_on_change' => 'boolean',
        'available_from' => 'datetime',
        'available_until' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'billing_cycle' => self::BILLING_MONTHLY,
        'currency' => self::DEFAULT_CURRENCY,
        'is_active' => true,
        'is_featured' => false,
        'is_popular' => false,
        'is_custom' => false,
        'sort_order' => 0,
        'has_trial' => false,
        'trial_requires_payment' => false,
        'allow_upgrades' => true,
        'allow_downgrades' => true,
        'allow_cancellation' => true,
        'prorate_on_change' => true,
        'active_subscriptions_count' => 0,
        'total_subscriptions_count' => 0,
        'total_revenue' => 0.0,
        'mrr' => 0.0,
        'arr' => 0.0,
        'metadata' => '[]',
        'features' => '[]',
        'limitations' => '[]',
        'pricing_tiers' => '[]',
        'access_levels' => '[]',
        'payment_providers' => '[]',
        'included_features' => '[]',
        'addon_features' => '[]',
        'comparison_highlights' => '[]',
        'terms_conditions' => '[]',
        'cta_text' => 'Get Started',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->generateSlugIfMissing();
            $model->validateBillingCycle();
            $model->calculateAnnualMetrics();
        });

        static::updating(function (self $model): void {
            $model->validateBillingCycle();
            
            if ($model->isDirty(['price', 'billing_cycle', 'active_subscriptions_count'])) {
                $model->calculateAnnualMetrics();
            }

            $model->updated_by = auth()->id();
        });

        static::saved(function (self $model): void {
            $model->clearPlanCache();
        });

        static::deleted(function (self $model): void {
            $model->clearPlanCache();
        });
    }

    /**
     * Get route key name
     */
    public function getRouteKeyName(): string
    {
        return 'slug';
    }

    /**
     * Get plan features (detailed)
     */
    public function planFeatures(): HasMany
    {
        return $this->hasMany(MembershipFeature::class, 'plan_id');
    }

    /**
     * Get user subscriptions/memberships
     */
    public function subscriptions(): HasMany
    {
        return $this->hasMany(UserMembership::class, 'plan_id');
    }

    /**
     * Get active subscriptions
     */
    public function activeSubscriptions(): HasMany
    {
        return $this->subscriptions()->where('status', 'active');
    }

    /**
     * Get upgrade plan
     */
    public function upgradePlan(): \Illuminate\Database\Eloquent\Relations\BelongsTo
    {
        return $this->belongsTo(MembershipPlan::class, 'upgrade_to_plan_id');
    }

    /**
     * Get downgrade plan
     */
    public function downgradePlan(): \Illuminate\Database\Eloquent\Relations\BelongsTo
    {
        return $this->belongsTo(MembershipPlan::class, 'downgrade_to_plan_id');
    }

    /**
     * Get available addons
     */
    public function addons(): BelongsToMany
    {
        return $this->belongsToMany(PlanAddon::class, 'plan_addon_availability')
            ->withPivot(['price', 'is_included'])
            ->withTimestamps();
    }

    /**
     * Generate slug from name if missing
     */
    protected function generateSlugIfMissing(): void
    {
        if (empty($this->slug)) {
            $this->slug = Str::slug($this->name);
        }

        // Ensure uniqueness
        $originalSlug = $this->slug;
        $counter = 1;

        while (static::where('slug', $this->slug)
            ->where('id', '!=', $this->id ?? 0)
            ->exists()) {
            $this->slug = $originalSlug . '-' . $counter++;
        }
    }

    /**
     * Validate billing cycle
     */
    protected function validateBillingCycle(): void
    {
        if (!in_array($this->billing_cycle, self::VALID_BILLING_CYCLES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid billing cycle: %s', $this->billing_cycle)
            );
        }
    }

    /**
     * Calculate MRR and ARR
     */
    protected function calculateAnnualMetrics(): void
    {
        // Calculate MRR (Monthly Recurring Revenue)
        $monthlyPrice = $this->getMonthlyEquivalentPrice();
        $this->mrr = round($monthlyPrice * $this->active_subscriptions_count, 2);

        // Calculate ARR (Annual Recurring Revenue)
        $this->arr = round($this->mrr * 12, 2);
    }

    /**
     * Get monthly equivalent price
     */
    public function getMonthlyEquivalentPrice(): float
    {
        return match($this->billing_cycle) {
            self::BILLING_MONTHLY => $this->price,
            self::BILLING_QUARTERLY => round($this->price / 3, 2),
            self::BILLING_SEMI_ANNUAL => round($this->price / 6, 2),
            self::BILLING_YEARLY => round(($this->annual_price ?? $this->price) / 12, 2),
            self::BILLING_LIFETIME => 0.0,
            default => $this->price,
        };
    }

    /**
     * Clear plan cache
     */
    protected function clearPlanCache(): void
    {
        Cache::tags(['membership_plans'])->flush();
        Cache::forget("plan:slug:{$this->slug}");
        Cache::forget("plan:id:{$this->id}");
        Cache::forget('plans:active');
    }

    /**
     * Check if plan is available
     */
    public function isAvailable(): bool
    {
        if (!$this->is_active) {
            return false;
        }

        $now = now();

        if ($this->available_from && $now->isBefore($this->available_from)) {
            return false;
        }

        if ($this->available_until && $now->isAfter($this->available_until)) {
            return false;
        }

        return true;
    }

    /**
     * Check if plan is free
     */
    public function isFree(): bool
    {
        return $this->price <= 0 && !$this->setup_fee;
    }

    /**
     * Check if plan is lifetime
     */
    public function isLifetime(): bool
    {
        return $this->billing_cycle === self::BILLING_LIFETIME;
    }

    /**
     * Check if plan has trial
     */
    public function hasTrial(): bool
    {
        return $this->has_trial && $this->trial_days > 0;
    }

    /**
     * Check if feature is included
     */
    public function hasFeature(string $featureCode): bool
    {
        return in_array($featureCode, $this->included_features, true);
    }

    /**
     * Check if within limit
     */
    public function isWithinLimit(string $limitType, int $currentValue): bool
    {
        $limitField = "max_{$limitType}";
        
        if (!property_exists($this, $limitField)) {
            return true;
        }

        $limit = $this->{$limitField};

        // Null means unlimited
        if ($limit === null) {
            return true;
        }

        return $currentValue < $limit;
    }

    /**
     * Get limit value
     */
    public function getLimit(string $limitType): ?int
    {
        $limitField = "max_{$limitType}";
        return $this->{$limitField} ?? null;
    }

    /**
     * Calculate price for billing cycle
     */
    public function getPriceForCycle(string $cycle = null): float
    {
        $cycle = $cycle ?? $this->billing_cycle;

        if ($cycle === self::BILLING_YEARLY && $this->annual_price) {
            return $this->annual_price;
        }

        return match($cycle) {
            self::BILLING_QUARTERLY => round($this->price * 3, 2),
            self::BILLING_SEMI_ANNUAL => round($this->price * 6, 2),
            self::BILLING_YEARLY => round($this->price * 12, 2),
            default => $this->price,
        };
    }

    /**
     * Calculate savings compared to monthly
     */
    public function getSavingsPercentage(): ?float
    {
        if ($this->billing_cycle === self::BILLING_MONTHLY || !$this->annual_price) {
            return null;
        }

        $monthlyTotal = $this->price * 12;
        $savings = $monthlyTotal - $this->annual_price;

        return round(($savings / $monthlyTotal) * 100, 2);
    }

    /**
     * Get price per month for display
     */
    public function getPricePerMonthAttribute(): float
    {
        return $this->getMonthlyEquivalentPrice();
    }

    /**
     * Get formatted price
     */
    public function getFormattedPriceAttribute(): string
    {
        return $this->formatMoney($this->price);
    }

    /**
     * Get formatted annual price
     */
    public function getFormattedAnnualPriceAttribute(): ?string
    {
        return $this->annual_price ? $this->formatMoney($this->annual_price) : null;
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
     * Get billing cycle label
     */
    public function getBillingCycleLabelAttribute(): string
    {
        return match($this->billing_cycle) {
            self::BILLING_MONTHLY => 'Monthly',
            self::BILLING_QUARTERLY => 'Quarterly',
            self::BILLING_SEMI_ANNUAL => 'Semi-Annual',
            self::BILLING_YEARLY => 'Yearly',
            self::BILLING_LIFETIME => 'Lifetime',
            self::BILLING_CUSTOM => 'Custom',
            default => ucfirst($this->billing_cycle),
        };
    }

    /**
     * Get feature list as array
     */
    public function getFeatureListAttribute(): array
    {
        return $this->features;
    }

    /**
     * Get limitation list as array
     */
    public function getLimitationListAttribute(): array
    {
        return $this->limitations;
    }

    /**
     * Calculate lifetime value
     */
    public function getLifetimeValueAttribute(): float
    {
        if ($this->active_subscriptions_count === 0) {
            return 0.0;
        }

        return round($this->total_revenue / $this->total_subscriptions_count, 2);
    }

    /**
     * Get retention rate
     */
    public function getRetentionRateAttribute(): float
    {
        if ($this->total_subscriptions_count === 0) {
            return 0.0;
        }

        return round(($this->active_subscriptions_count / $this->total_subscriptions_count) * 100, 2);
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
     * Increment subscription count
     */
    public function incrementSubscriptions(): self
    {
        $this->increment('active_subscriptions_count');
        $this->increment('total_subscriptions_count');
        $this->calculateAnnualMetrics();
        $this->save();

        return $this;
    }

    /**
     * Decrement subscription count
     */
    public function decrementSubscriptions(): self
    {
        $this->decrement('active_subscriptions_count');
        $this->calculateAnnualMetrics();
        $this->save();

        return $this;
    }

    /**
     * Add revenue
     */
    public function addRevenue(float $amount): self
    {
        $this->increment('total_revenue', round($amount, 2));
        return $this;
    }

    /**
     * Scope: Active plans
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    /**
     * Scope: Available plans
     */
    public function scopeAvailable(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->where(function($q) {
                $q->whereNull('available_from')
                  ->orWhere('available_from', '<=', now());
            })
            ->where(function($q) {
                $q->whereNull('available_until')
                  ->orWhere('available_until', '>=', now());
            });
    }

    /**
     * Scope: Featured plans
     */
    public function scopeFeatured(Builder $query): Builder
    {
        return $query->where('is_featured', true);
    }

    /**
     * Scope: Popular plans
     */
    public function scopePopular(Builder $query): Builder
    {
        return $query->where('is_popular', true);
    }

    /**
     * Scope: Free plans
     */
    public function scopeFree(Builder $query): Builder
    {
        return $query->where('price', '<=', 0)
            ->where(function($q) {
                $q->whereNull('setup_fee')
                  ->orWhere('setup_fee', '<=', 0);
            });
    }

    /**
     * Scope: Paid plans
     */
    public function scopePaid(Builder $query): Builder
    {
        return $query->where('price', '>', 0);
    }

    /**
     * Scope: By billing cycle
     */
    public function scopeBillingCycle(Builder $query, string $cycle): Builder
    {
        return $query->where('billing_cycle', $cycle);
    }

    /**
     * Scope: Order by sort order
     */
    public function scopeOrderBySortOrder(Builder $query, string $direction = 'asc'): Builder
    {
        return $query->orderBy('sort_order', $direction)
            ->orderBy('price', $direction);
    }

    /**
     * Scope: Order by price
     */
    public function scopeOrderByPrice(Builder $query, string $direction = 'asc'): Builder
    {
        return $query->orderBy('price', $direction);
    }

    /**
     * Scope: Order by popularity
     */
    public function scopeOrderByPopularity(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('active_subscriptions_count', $direction);
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total_plans' => static::count(),
            'active_plans' => static::active()->count(),
            'total_subscriptions' => static::sum('active_subscriptions_count'),
            'total_mrr' => round(static::sum('mrr'), 2),
            'total_arr' => round(static::sum('arr'), 2),
            'total_revenue' => round(static::sum('total_revenue'), 2),
            'avg_price' => round(static::paid()->avg('price'), 2),
            'avg_retention_rate' => round(static::where('total_subscriptions_count', '>', 0)->avg(\DB::raw('(active_subscriptions_count / total_subscriptions_count) * 100')), 2),
        ];
    }

    /**
     * Static: Get pricing table data
     */
    public static function getPricingTable(): Collection
    {
        return Cache::tags(['membership_plans'])->remember(
            'plans:pricing_table',
            self::CACHE_TTL,
            fn() => static::available()
                ->orderBySortOrder()
                ->get()
        );
    }

    /**
     * Static: Get comparison data
     */
    public static function getComparisonData(): array
    {
        $plans = static::available()->orderByPrice()->get();

        $features = [];
        foreach ($plans as $plan) {
            foreach ($plan->features as $feature) {
                if (!in_array($feature, $features, true)) {
                    $features[] = $feature;
                }
            }
        }

        return [
            'plans' => $plans,
            'features' => $features,
        ];
    }

    /**
     * Export to array for API
     */
    public function toPlanArray(bool $detailed = false): array
    {
        $data = [
            'id' => $this->id,
            'name' => $this->name,
            'slug' => $this->slug,
            'description' => $this->description,
            'tagline' => $this->tagline,
            'price' => $this->price,
            'price_per_month' => $this->price_per_month,
            'formatted_price' => $this->formatted_price,
            'billing_cycle' => $this->billing_cycle,
            'billing_cycle_label' => $this->billing_cycle_label,
            'currency' => $this->currency,
            'is_featured' => $this->is_featured,
            'is_popular' => $this->is_popular,
            'is_free' => $this->isFree(),
            'features' => $this->features,
            'limitations' => $this->limitations,
            'has_trial' => $this->hasTrial(),
            'trial_days' => $this->trial_days,
            'cta_text' => $this->cta_text,
        ];

        if ($detailed) {
            $data = array_merge($data, [
                'annual_price' => $this->annual_price,
                'formatted_annual_price' => $this->formatted_annual_price,
                'setup_fee' => $this->setup_fee,
                'savings_percentage' => $this->getSavingsPercentage(),
                'max_users' => $this->max_users,
                'max_projects' => $this->max_projects,
                'max_storage' => $this->max_storage,
                'max_api_calls' => $this->max_api_calls,
                'pricing_tiers' => $this->pricing_tiers,
                'comparison_highlights' => $this->comparison_highlights,
                'allow_upgrades' => $this->allow_upgrades,
                'allow_downgrades' => $this->allow_downgrades,
                'allow_cancellation' => $this->allow_cancellation,
                'active_subscriptions' => $this->active_subscriptions_count,
                'retention_rate' => $this->retention_rate,
            ]);
        }

        return $data;
    }
}