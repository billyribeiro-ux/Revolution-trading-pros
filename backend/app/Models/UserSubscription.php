<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;

/**
 * UserSubscription Model - Enterprise-grade subscription management
 *
 * Comprehensive subscription lifecycle management with support for:
 * - Active subscriptions with auto-renewal
 * - Trial periods with automatic conversion
 * - Paused/On-hold states with resumption
 * - Cancellation with grace periods
 * - Payment tracking and failure handling
 * - Revenue metrics (MRR, ARR, LTV)
 * - Usage tracking and limits
 * - Dunning management
 *
 * @property int $id
 * @property int $user_id
 * @property int $subscription_plan_id
 * @property int|null $product_id
 * @property string $status Subscription status
 * @property string $interval Billing interval
 * @property float $price Subscription price
 * @property string $currency Currency code
 * @property Carbon $start_date Subscription start date
 * @property Carbon|null $end_date Subscription end date
 * @property Carbon|null $next_payment_date Next payment due date
 * @property Carbon|null $last_payment_date Last successful payment date
 * @property Carbon|null $trial_ends_at Trial end timestamp
 * @property Carbon|null $trial_end_date Trial end date (alias)
 * @property Carbon|null $current_period_start Current billing period start
 * @property Carbon|null $current_period_end Current billing period end
 * @property Carbon|null $cancelled_at Cancellation timestamp
 * @property Carbon|null $paused_at Pause timestamp
 * @property Carbon|null $expires_at Expiration timestamp
 * @property string|null $payment_method Payment method identifier
 * @property string|null $payment_id Payment gateway ID
 * @property float $amount_paid Current period amount paid
 * @property float $total_paid Total amount paid lifetime
 * @property int $failed_payments Failed payment count
 * @property int $successful_payments Successful payment count
 * @property int $billing_cycles_completed Completed billing cycles
 * @property int $renewal_count Renewal count
 * @property string|null $cancellation_reason Cancellation reason
 * @property string|null $pause_reason Pause reason
 * @property bool $auto_renew Auto-renewal flag
 * @property array|null $metadata Additional metadata
 * @property string|null $notes Internal notes
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read User $user
 * @property-read SubscriptionPlan $plan
 * @property-read Product|null $product
 * @property-read Collection<int, SubscriptionPayment> $payments
 * @property-read Collection<int, SubscriptionUsage> $usage
 * @property-read float $mrr Monthly recurring revenue
 * @property-read float $arr Annual recurring revenue
 * @property-read float $ltv Lifetime value
 * @property-read int $days_until_renewal Days until next renewal
 * @property-read bool $is_active
 * @property-read bool $is_trial
 * @property-read bool $is_cancelled
 * @property-read bool $is_paused
 * @property-read bool $is_expired
 * 
 * @method static Builder active()
 * @method static Builder trial()
 * @method static Builder onHold()
 * @method static Builder cancelled()
 * @method static Builder expired()
 * @method static Builder expiringSoon(int $days = 7)
 * @method static Builder byInterval(string $interval)
 * @method static Builder byPlan(int $planId)
 * @method static Builder byUser(int $userId)
 * @method static Builder autoRenewing()
 */
class UserSubscription extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * The table associated with the model.
     */
    protected $table = 'user_subscriptions';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'user_id',
        'subscription_plan_id',
        'product_id',
        'status',
        'interval',
        'price',
        'currency',
        'start_date',
        'end_date',
        'next_payment_date',
        'last_payment_date',
        'trial_ends_at',
        'trial_end_date',
        'current_period_start',
        'current_period_end',
        'cancelled_at',
        'paused_at',
        'expires_at',
        'payment_method',
        'payment_id',
        'amount_paid',
        'total_paid',
        'failed_payments',
        'successful_payments',
        'billing_cycles_completed',
        'renewal_count',
        'cancellation_reason',
        'pause_reason',
        'auto_renew',
        'metadata',
        'notes',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'user_id' => 'integer',
        'subscription_plan_id' => 'integer',
        'product_id' => 'integer',
        'start_date' => 'datetime',
        'end_date' => 'datetime',
        'next_payment_date' => 'datetime',
        'last_payment_date' => 'datetime',
        'trial_ends_at' => 'datetime',
        'trial_end_date' => 'datetime',
        'current_period_start' => 'datetime',
        'current_period_end' => 'datetime',
        'cancelled_at' => 'datetime',
        'paused_at' => 'datetime',
        'expires_at' => 'datetime',
        'price' => 'decimal:2',
        'amount_paid' => 'decimal:2',
        'total_paid' => 'decimal:2',
        'failed_payments' => 'integer',
        'successful_payments' => 'integer',
        'billing_cycles_completed' => 'integer',
        'renewal_count' => 'integer',
        'auto_renew' => 'boolean',
        'metadata' => 'array',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'status' => self::STATUS_PENDING,
        'interval' => self::INTERVAL_MONTHLY,
        'currency' => 'USD',
        'total_paid' => 0,
        'amount_paid' => 0,
        'failed_payments' => 0,
        'successful_payments' => 0,
        'billing_cycles_completed' => 0,
        'renewal_count' => 0,
        'auto_renew' => true,
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        'mrr',
        'arr',
        'ltv',
        'days_until_renewal',
        'is_active',
        'is_trial',
        'is_cancelled',
        'is_paused',
        'is_expired',
    ];

    // =========================================================================
    // CONSTANTS - STATUS
    // =========================================================================

    public const STATUS_ACTIVE = 'active';
    public const STATUS_PENDING = 'pending';
    public const STATUS_ON_HOLD = 'on-hold';
    public const STATUS_CANCELLED = 'cancelled';
    public const STATUS_EXPIRED = 'expired';
    public const STATUS_PENDING_CANCEL = 'pending-cancel';
    public const STATUS_TRIAL = 'trial';
    public const STATUS_PAUSED = 'paused';
    public const STATUS_PAST_DUE = 'past-due';

    /**
     * Available subscription statuses.
     *
     * @var array<int, string>
     */
    public const STATUSES = [
        self::STATUS_ACTIVE,
        self::STATUS_PENDING,
        self::STATUS_ON_HOLD,
        self::STATUS_CANCELLED,
        self::STATUS_EXPIRED,
        self::STATUS_PENDING_CANCEL,
        self::STATUS_TRIAL,
        self::STATUS_PAUSED,
        self::STATUS_PAST_DUE,
    ];

    /**
     * Status labels for display.
     *
     * @var array<string, string>
     */
    public const STATUS_LABELS = [
        self::STATUS_ACTIVE => 'Active',
        self::STATUS_PENDING => 'Pending',
        self::STATUS_ON_HOLD => 'On Hold',
        self::STATUS_CANCELLED => 'Cancelled',
        self::STATUS_EXPIRED => 'Expired',
        self::STATUS_PENDING_CANCEL => 'Pending Cancellation',
        self::STATUS_TRIAL => 'Trial',
        self::STATUS_PAUSED => 'Paused',
        self::STATUS_PAST_DUE => 'Past Due',
    ];

    /**
     * Status colors for UI.
     *
     * @var array<string, string>
     */
    public const STATUS_COLORS = [
        self::STATUS_ACTIVE => 'green',
        self::STATUS_PENDING => 'yellow',
        self::STATUS_ON_HOLD => 'orange',
        self::STATUS_CANCELLED => 'red',
        self::STATUS_EXPIRED => 'gray',
        self::STATUS_PENDING_CANCEL => 'orange',
        self::STATUS_TRIAL => 'blue',
        self::STATUS_PAUSED => 'orange',
        self::STATUS_PAST_DUE => 'red',
    ];

    // =========================================================================
    // CONSTANTS - INTERVALS
    // =========================================================================

    public const INTERVAL_DAILY = 'daily';
    public const INTERVAL_WEEKLY = 'weekly';
    public const INTERVAL_MONTHLY = 'monthly';
    public const INTERVAL_QUARTERLY = 'quarterly';
    public const INTERVAL_YEARLY = 'yearly';
    public const INTERVAL_ANNUAL = 'annual';

    /**
     * Available billing intervals.
     *
     * @var array<int, string>
     */
    public const INTERVALS = [
        self::INTERVAL_DAILY,
        self::INTERVAL_WEEKLY,
        self::INTERVAL_MONTHLY,
        self::INTERVAL_QUARTERLY,
        self::INTERVAL_YEARLY,
        self::INTERVAL_ANNUAL,
    ];

    /**
     * Interval labels.
     *
     * @var array<string, string>
     */
    public const INTERVAL_LABELS = [
        self::INTERVAL_DAILY => 'Daily',
        self::INTERVAL_WEEKLY => 'Weekly',
        self::INTERVAL_MONTHLY => 'Monthly',
        self::INTERVAL_QUARTERLY => 'Quarterly',
        self::INTERVAL_YEARLY => 'Yearly',
        self::INTERVAL_ANNUAL => 'Annual',
    ];

    // =========================================================================
    // CONSTANTS - CONFIGURATION
    // =========================================================================

    public const DUNNING_MAX_ATTEMPTS = 3;
    public const GRACE_PERIOD_DAYS = 3;
    public const TRIAL_DEFAULT_DAYS = 14;
    public const EXPIRING_SOON_DAYS = 7;

    // =========================================================================
    // BOOT & LIFECYCLE EVENTS
    // =========================================================================

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        // Set initial dates on creation
        static::creating(function (UserSubscription $subscription) {
            if (empty($subscription->start_date)) {
                $subscription->start_date = now();
            }

            if (empty($subscription->current_period_start)) {
                $subscription->current_period_start = $subscription->start_date;
            }

            if (empty($subscription->current_period_end)) {
                $subscription->current_period_end = $subscription->calculateNextPeriodEnd(
                    $subscription->current_period_start,
                    $subscription->plan
                );
            }

            if (empty($subscription->next_payment_date) && $subscription->auto_renew) {
                $subscription->next_payment_date = $subscription->current_period_end;
            }
        });

        // Log status changes
        static::updating(function (UserSubscription $subscription) {
            if ($subscription->isDirty('status')) {
                Log::info('Subscription status changed', [
                    'id' => $subscription->id,
                    'user_id' => $subscription->user_id,
                    'old_status' => $subscription->getOriginal('status'),
                    'new_status' => $subscription->status,
                ]);
            }
        });

        // Clear cache on changes
        static::saved(function (UserSubscription $subscription) {
            Cache::forget("user_subscription_metrics_{$subscription->user_id}");
            Cache::forget("subscription_mrr_{$subscription->id}");
        });

        static::deleted(function (UserSubscription $subscription) {
            Cache::forget("user_subscription_metrics_{$subscription->user_id}");
            Cache::forget("subscription_mrr_{$subscription->id}");
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the user that owns the subscription.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class)
            ->withDefault([
                'name' => 'Unknown User',
            ]);
    }

    /**
     * Get the subscription plan.
     */
    public function plan(): BelongsTo
    {
        return $this->belongsTo(SubscriptionPlan::class, 'subscription_plan_id')
            ->withDefault([
                'name' => 'Unknown Plan',
                'price' => 0,
            ]);
    }

    /**
     * Get the product associated with the subscription.
     */
    public function product(): BelongsTo
    {
        return $this->belongsTo(Product::class)
            ->withDefault([
                'name' => 'Unknown Product',
            ]);
    }

    /**
     * Get all payments for this subscription.
     */
    public function payments(): HasMany
    {
        return $this->hasMany(SubscriptionPayment::class);
    }

    /**
     * Get usage records for this subscription.
     */
    public function usage(): HasMany
    {
        return $this->hasMany(SubscriptionUsage::class);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope to active subscriptions.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_ACTIVE);
    }

    /**
     * Scope to trial subscriptions.
     */
    public function scopeTrial(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_TRIAL)
            ->orWhere(function (Builder $q) {
                $q->whereNotNull('trial_ends_at')
                    ->where('trial_ends_at', '>', now());
            });
    }

    /**
     * Scope to on-hold subscriptions.
     */
    public function scopeOnHold(Builder $query): Builder
    {
        return $query->whereIn('status', [self::STATUS_ON_HOLD, self::STATUS_PAUSED]);
    }

    /**
     * Scope to cancelled subscriptions.
     */
    public function scopeCancelled(Builder $query): Builder
    {
        return $query->whereIn('status', [self::STATUS_CANCELLED, self::STATUS_PENDING_CANCEL]);
    }

    /**
     * Scope to expired subscriptions.
     */
    public function scopeExpired(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_EXPIRED);
    }

    /**
     * Scope to past due subscriptions.
     */
    public function scopePastDue(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_PAST_DUE);
    }

    /**
     * Scope to expiring soon subscriptions.
     */
    public function scopeExpiringSoon(Builder $query, int $days = self::EXPIRING_SOON_DAYS): Builder
    {
        return $query->where('status', self::STATUS_ACTIVE)
            ->where(function (Builder $q) use ($days) {
                $q->whereBetween('next_payment_date', [now(), now()->addDays($days)])
                    ->orWhereBetween('current_period_end', [now(), now()->addDays($days)]);
            });
    }

    /**
     * Scope by billing interval.
     */
    public function scopeByInterval(Builder $query, string $interval): Builder
    {
        return $query->where('interval', $interval);
    }

    /**
     * Scope by subscription plan.
     */
    public function scopeByPlan(Builder $query, int $planId): Builder
    {
        return $query->where('subscription_plan_id', $planId);
    }

    /**
     * Scope by user.
     */
    public function scopeByUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope to auto-renewing subscriptions.
     */
    public function scopeAutoRenewing(Builder $query): Builder
    {
        return $query->where('auto_renew', true)
            ->whereIn('status', [self::STATUS_ACTIVE, self::STATUS_TRIAL]);
    }

    /**
     * Scope to revenue-generating subscriptions.
     */
    public function scopeRevenueGenerating(Builder $query): Builder
    {
        return $query->whereIn('status', [
            self::STATUS_ACTIVE,
            self::STATUS_TRIAL,
            self::STATUS_PAST_DUE,
            self::STATUS_PENDING_CANCEL,
        ]);
    }

    /**
     * Scope by currency.
     */
    public function scopeByCurrency(Builder $query, string $currency): Builder
    {
        return $query->where('currency', strtoupper($currency));
    }

    /**
     * Scope to recent subscriptions.
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('start_date', '>=', now()->subDays($days));
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Check if subscription is active.
     */
    public function getIsActiveAttribute(): bool
    {
        return $this->isActive();
    }

    /**
     * Check if subscription is on trial.
     */
    public function getIsTrialAttribute(): bool
    {
        return $this->onTrial();
    }

    /**
     * Check if subscription is cancelled.
     */
    public function getIsCancelledAttribute(): bool
    {
        return $this->isCancelled();
    }

    /**
     * Check if subscription is paused.
     */
    public function getIsPausedAttribute(): bool
    {
        return $this->isPaused();
    }

    /**
     * Check if subscription is expired.
     */
    public function getIsExpiredAttribute(): bool
    {
        return $this->isExpired();
    }

    /**
     * Get monthly recurring revenue.
     */
    public function getMrrAttribute(): float
    {
        return $this->getMrr();
    }

    /**
     * Get annual recurring revenue.
     */
    public function getArrAttribute(): float
    {
        return $this->getArr();
    }

    /**
     * Get lifetime value.
     */
    public function getLtvAttribute(): float
    {
        return $this->getLtv();
    }

    /**
     * Get days until renewal.
     */
    public function getDaysUntilRenewalAttribute(): int
    {
        return $this->getDaysUntilRenewal();
    }

    /**
     * Get status label.
     */
    public function getStatusLabelAttribute(): string
    {
        return self::STATUS_LABELS[$this->status] ?? ucfirst($this->status);
    }

    /**
     * Get status color.
     */
    public function getStatusColorAttribute(): string
    {
        return self::STATUS_COLORS[$this->status] ?? 'gray';
    }

    /**
     * Get interval label.
     */
    public function getIntervalLabelAttribute(): string
    {
        return self::INTERVAL_LABELS[$this->interval] ?? ucfirst($this->interval);
    }

    /**
     * Normalize currency code.
     */
    public function setCurrencyAttribute(?string $value): void
    {
        $this->attributes['currency'] = $value ? strtoupper($value) : 'USD';
    }

    // =========================================================================
    // STATUS CHECK METHODS
    // =========================================================================

    /**
     * Check if subscription is active.
     */
    public function isActive(): bool
    {
        return $this->status === self::STATUS_ACTIVE;
    }

    /**
     * Check if subscription is on trial.
     */
    public function onTrial(): bool
    {
        if ($this->status === self::STATUS_TRIAL) {
            return true;
        }

        $trialEnd = $this->trial_end_date ?? $this->trial_ends_at;
        return $trialEnd && $trialEnd->isFuture();
    }

    /**
     * Check if subscription is cancelled.
     */
    public function isCancelled(): bool
    {
        return in_array($this->status, [self::STATUS_CANCELLED, self::STATUS_PENDING_CANCEL], true);
    }

    /**
     * Check if subscription is paused.
     */
    public function isPaused(): bool
    {
        return in_array($this->status, [self::STATUS_PAUSED, self::STATUS_ON_HOLD], true);
    }

    /**
     * Check if subscription is expired.
     */
    public function isExpired(): bool
    {
        if ($this->status === self::STATUS_EXPIRED) {
            return true;
        }

        $endDate = $this->end_date ?? $this->expires_at;
        return $endDate && $endDate->isPast();
    }

    /**
     * Check if subscription is past due.
     */
    public function isPastDue(): bool
    {
        return $this->status === self::STATUS_PAST_DUE;
    }

    /**
     * Check if subscription is pending cancellation.
     */
    public function isPendingCancellation(): bool
    {
        return $this->status === self::STATUS_PENDING_CANCEL;
    }

    /**
     * Check if subscription can be renewed.
     */
    public function canRenew(): bool
    {
        return $this->auto_renew 
            && in_array($this->status, [self::STATUS_ACTIVE, self::STATUS_TRIAL, self::STATUS_PAST_DUE], true);
    }

    /**
     * Check if subscription is ending soon.
     */
    public function isEndingSoon(int $days = self::EXPIRING_SOON_DAYS): bool
    {
        if (!$this->next_payment_date) {
            return false;
        }

        return $this->next_payment_date->between(now(), now()->addDays($days));
    }

    /**
     * Check if subscription is in grace period.
     */
    public function isInGracePeriod(): bool
    {
        if (!$this->current_period_end) {
            return false;
        }

        return now()->between(
            $this->current_period_end,
            $this->current_period_end->copy()->addDays(self::GRACE_PERIOD_DAYS)
        );
    }

    // =========================================================================
    // LIFECYCLE MANAGEMENT METHODS
    // =========================================================================

    /**
     * Cancel subscription immediately.
     */
    public function cancel(?string $reason = null): self
    {
        $this->update([
            'status' => self::STATUS_CANCELLED,
            'cancelled_at' => now(),
            'cancellation_reason' => $reason,
            'end_date' => now(),
            'auto_renew' => false,
        ]);

        Log::info('Subscription cancelled', [
            'id' => $this->id,
            'user_id' => $this->user_id,
            'reason' => $reason,
        ]);

        return $this;
    }

    /**
     * Cancel subscription at end of current period.
     */
    public function cancelAtPeriodEnd(?string $reason = null): self
    {
        $endDate = $this->next_payment_date ?? $this->current_period_end ?? now()->addMonth();

        $this->update([
            'status' => self::STATUS_PENDING_CANCEL,
            'cancelled_at' => now(),
            'cancellation_reason' => $reason,
            'end_date' => $endDate,
            'auto_renew' => false,
        ]);

        Log::info('Subscription pending cancellation', [
            'id' => $this->id,
            'user_id' => $this->user_id,
            'end_date' => $endDate->toDateTimeString(),
            'reason' => $reason,
        ]);

        return $this;
    }

    /**
     * Pause subscription.
     */
    public function pause(?string $reason = null): self
    {
        $this->update([
            'status' => self::STATUS_ON_HOLD,
            'paused_at' => now(),
            'pause_reason' => $reason,
        ]);

        Log::info('Subscription paused', [
            'id' => $this->id,
            'user_id' => $this->user_id,
            'reason' => $reason,
        ]);

        return $this;
    }

    /**
     * Resume paused subscription.
     */
    public function resume(): self
    {
        $this->update([
            'status' => self::STATUS_ACTIVE,
            'paused_at' => null,
            'pause_reason' => null,
        ]);

        Log::info('Subscription resumed', [
            'id' => $this->id,
            'user_id' => $this->user_id,
        ]);

        return $this;
    }

    /**
     * Reactivate cancelled subscription.
     */
    public function reactivate(): self
    {
        $nextPaymentDate = $this->calculateNextPeriodEnd(now(), $this->plan);

        $this->update([
            'status' => self::STATUS_ACTIVE,
            'cancelled_at' => null,
            'cancellation_reason' => null,
            'end_date' => null,
            'auto_renew' => true,
            'next_payment_date' => $nextPaymentDate,
            'current_period_start' => now(),
            'current_period_end' => $nextPaymentDate,
        ]);

        Log::info('Subscription reactivated', [
            'id' => $this->id,
            'user_id' => $this->user_id,
        ]);

        return $this;
    }

    /**
     * Renew subscription for next billing cycle.
     */
    public function renew(): self
    {
        return DB::transaction(function () {
            $plan = $this->plan;
            $nextPeriodStart = $this->next_payment_date ?? $this->current_period_end ?? now();
            $nextPeriodEnd = $this->calculateNextPeriodEnd($nextPeriodStart, $plan);
            $price = $this->price ?? $plan->price ?? 0;

            $this->update([
                'status' => self::STATUS_ACTIVE,
                'current_period_start' => $nextPeriodStart,
                'current_period_end' => $nextPeriodEnd,
                'next_payment_date' => $nextPeriodEnd,
                'last_payment_date' => now(),
                'billing_cycles_completed' => $this->billing_cycles_completed + 1,
                'renewal_count' => $this->renewal_count + 1,
                'successful_payments' => $this->successful_payments + 1,
                'total_paid' => $this->total_paid + $price,
                'amount_paid' => $price,
            ]);

            Log::info('Subscription renewed', [
                'id' => $this->id,
                'user_id' => $this->user_id,
                'billing_cycle' => $this->billing_cycles_completed,
            ]);

            return $this;
        });
    }

    /**
     * Convert trial to active subscription.
     */
    public function convertFromTrial(): self
    {
        $this->update([
            'status' => self::STATUS_ACTIVE,
            'trial_ends_at' => now(),
            'trial_end_date' => now(),
        ]);

        Log::info('Trial converted to active', [
            'id' => $this->id,
            'user_id' => $this->user_id,
        ]);

        return $this;
    }

    /**
     * Mark subscription as expired.
     */
    public function markAsExpired(): self
    {
        $this->update([
            'status' => self::STATUS_EXPIRED,
            'expires_at' => now(),
            'end_date' => now(),
        ]);

        return $this;
    }

    // =========================================================================
    // PAYMENT MANAGEMENT METHODS
    // =========================================================================

    /**
     * Record successful payment.
     */
    public function recordPaymentSuccess(float $amount, ?string $paymentId = null): self
    {
        $this->update([
            'last_payment_date' => now(),
            'amount_paid' => $amount,
            'total_paid' => $this->total_paid + $amount,
            'successful_payments' => $this->successful_payments + 1,
            'failed_payments' => 0, // Reset failed payment counter
            'payment_id' => $paymentId,
        ]);

        return $this;
    }

    /**
     * Record payment failure with dunning management.
     */
    public function recordPaymentFailure(?string $reason = null): self
    {
        $failedCount = $this->failed_payments + 1;
        $updates = ['failed_payments' => $failedCount];

        // Implement dunning logic
        if ($failedCount >= self::DUNNING_MAX_ATTEMPTS) {
            $updates['status'] = self::STATUS_ON_HOLD;
            $updates['paused_at'] = now();
            $updates['pause_reason'] = 'Multiple payment failures: ' . ($reason ?? 'Unknown');

            Log::warning('Subscription on hold due to payment failures', [
                'id' => $this->id,
                'user_id' => $this->user_id,
                'failed_count' => $failedCount,
            ]);
        } elseif ($failedCount === 1) {
            $updates['status'] = self::STATUS_PAST_DUE;
        }

        $this->update($updates);

        return $this;
    }

    /**
     * Update payment method.
     */
    public function updatePaymentMethod(string $paymentMethod, ?string $paymentId = null): self
    {
        $this->update([
            'payment_method' => $paymentMethod,
            'payment_id' => $paymentId,
        ]);

        return $this;
    }

    // =========================================================================
    // FINANCIAL CALCULATION METHODS
    // =========================================================================

    /**
     * Get effective subscription price.
     */
    public function getEffectivePrice(): float
    {
        return $this->price ?? $this->plan?->price ?? 0.0;
    }

    /**
     * Calculate monthly recurring revenue (MRR).
     */
    public function getMrr(): float
    {
        if (!in_array($this->status, [self::STATUS_ACTIVE, self::STATUS_TRIAL, self::STATUS_PENDING_CANCEL], true)) {
            return 0.0;
        }

        $price = $this->getEffectivePrice();

        return match ($this->interval) {
            self::INTERVAL_DAILY => $price * 30,
            self::INTERVAL_WEEKLY => $price * 4.33,
            self::INTERVAL_MONTHLY => $price,
            self::INTERVAL_QUARTERLY => $price / 3,
            self::INTERVAL_YEARLY, self::INTERVAL_ANNUAL => $price / 12,
            default => $price,
        };
    }

    /**
     * Calculate annual recurring revenue (ARR).
     */
    public function getArr(): float
    {
        return $this->getMrr() * 12;
    }

    /**
     * Calculate lifetime value (LTV).
     */
    public function getLtv(): float
    {
        return $this->total_paid ?? 0.0;
    }

    /**
     * Get average revenue per billing cycle.
     */
    public function getAverageRevenuePerCycle(): float
    {
        if ($this->billing_cycles_completed === 0) {
            return 0.0;
        }

        return round($this->total_paid / $this->billing_cycles_completed, 2);
    }

    /**
     * Calculate customer lifetime in days.
     */
    public function getLifetimeDays(): int
    {
        return $this->start_date->diffInDays($this->end_date ?? now());
    }

    /**
     * Get days until renewal.
     */
    public function getDaysUntilRenewal(): int
    {
        if (!$this->next_payment_date) {
            return 0;
        }

        return max(0, now()->diffInDays($this->next_payment_date, false));
    }

    // =========================================================================
    // DATE CALCULATION METHODS
    // =========================================================================

    /**
     * Calculate next period end date based on interval.
     */
    protected function calculateNextPeriodEnd(Carbon $start, ?SubscriptionPlan $plan = null): Carbon
    {
        $interval = $this->interval ?? $plan?->billing_period ?? self::INTERVAL_MONTHLY;
        $count = $plan?->billing_interval ?? 1;

        return match ($interval) {
            self::INTERVAL_DAILY => $start->copy()->addDays($count),
            self::INTERVAL_WEEKLY => $start->copy()->addWeeks($count),
            self::INTERVAL_MONTHLY => $start->copy()->addMonths($count),
            self::INTERVAL_QUARTERLY => $start->copy()->addMonths($count * 3),
            self::INTERVAL_YEARLY, self::INTERVAL_ANNUAL => $start->copy()->addYears($count),
            default => $start->copy()->addMonth(),
        };
    }

    /**
     * Extend subscription by interval.
     */
    public function extend(int $periods = 1): self
    {
        $currentEnd = $this->current_period_end ?? now();
        $newEnd = $this->calculateNextPeriodEnd($currentEnd, $this->plan);

        for ($i = 1; $i < $periods; $i++) {
            $newEnd = $this->calculateNextPeriodEnd($newEnd, $this->plan);
        }

        $this->update([
            'current_period_end' => $newEnd,
            'next_payment_date' => $newEnd,
        ]);

        return $this;
    }

    // =========================================================================
    // METADATA MANAGEMENT
    // =========================================================================

    /**
     * Add metadata entry.
     */
    public function addMetadata(string $key, mixed $value): bool
    {
        $metadata = $this->metadata ?? [];
        $metadata[$key] = $value;
        $this->metadata = $metadata;
        return $this->save();
    }

    /**
     * Get metadata value by key.
     */
    public function getMetadata(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Remove metadata entry.
     */
    public function removeMetadata(string $key): bool
    {
        $metadata = $this->metadata ?? [];
        unset($metadata[$key]);
        $this->metadata = $metadata;
        return $this->save();
    }

    // =========================================================================
    // UTILITY & EXPORT METHODS
    // =========================================================================

    /**
     * Get subscription summary.
     */
    public function toSummary(): array
    {
        return [
            'id' => $this->id,
            'user_id' => $this->user_id,
            'plan_name' => $this->plan->name,
            'status' => $this->status_label,
            'status_color' => $this->status_color,
            'interval' => $this->interval_label,
            'price' => $this->price,
            'currency' => $this->currency,
            'mrr' => $this->mrr,
            'arr' => $this->arr,
            'ltv' => $this->ltv,
            'start_date' => $this->start_date->toDateString(),
            'next_payment_date' => $this->next_payment_date?->toDateString(),
            'days_until_renewal' => $this->days_until_renewal,
            'billing_cycles' => $this->billing_cycles_completed,
            'is_active' => $this->is_active,
            'is_trial' => $this->is_trial,
            'auto_renew' => $this->auto_renew,
        ];
    }

    // =========================================================================
    // STATIC FACTORY & ANALYTICS METHODS
    // =========================================================================

    /**
     * Get total MRR for all active subscriptions.
     */
    public static function getTotalMrr(?string $currency = null): float
    {
        $query = self::revenueGenerating();

        if ($currency) {
            $query->byCurrency($currency);
        }

        return $query->get()->sum(fn($sub) => $sub->getMrr());
    }

    /**
     * Get total ARR for all active subscriptions.
     */
    public static function getTotalArr(?string $currency = null): float
    {
        return self::getTotalMrr($currency) * 12;
    }

    /**
     * Get churn rate for a period.
     */
    public static function getChurnRate(int $days = 30): float
    {
        $startOfPeriod = now()->subDays($days);
        
        $activeAtStart = self::where('start_date', '<', $startOfPeriod)
            ->whereNotIn('status', [self::STATUS_CANCELLED, self::STATUS_EXPIRED])
            ->count();

        if ($activeAtStart === 0) {
            return 0.0;
        }

        $cancelledInPeriod = self::whereBetween('cancelled_at', [$startOfPeriod, now()])
            ->count();

        return round(($cancelledInPeriod / $activeAtStart) * 100, 2);
    }

    /**
     * Get retention rate for a period.
     */
    public static function getRetentionRate(int $days = 30): float
    {
        return 100 - self::getChurnRate($days);
    }

    /**
     * Bulk cancel subscriptions.
     *
     * @param array<int> $ids
     */
    public static function bulkCancel(array $ids, ?string $reason = null): int
    {
        return self::whereIn('id', $ids)->update([
            'status' => self::STATUS_CANCELLED,
            'cancelled_at' => now(),
            'cancellation_reason' => $reason,
            'end_date' => now(),
            'auto_renew' => false,
        ]);
    }

    /**
     * Process renewals for subscriptions due today.
     */
    public static function processRenewals(): int
    {
        $subscriptions = self::autoRenewing()
            ->whereDate('next_payment_date', '<=', now())
            ->get();

        $renewed = 0;
        foreach ($subscriptions as $subscription) {
            try {
                $subscription->renew();
                $renewed++;
            } catch (\Exception $e) {
                Log::error('Subscription renewal failed', [
                    'id' => $subscription->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return $renewed;
    }

    /**
     * Process trial expirations.
     */
    public static function processTrialExpirations(): int
    {
        $subscriptions = self::trial()
            ->whereDate('trial_ends_at', '<=', now())
            ->get();

        $converted = 0;
        foreach ($subscriptions as $subscription) {
            $subscription->convertFromTrial();
            $converted++;
        }

        return $converted;
    }
}