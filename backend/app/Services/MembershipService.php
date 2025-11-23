<?php

namespace App\Services;

use App\Models\Product;
use App\Models\User;
use App\Models\Membership;
use App\Models\MembershipPlan;
use App\Models\MembershipFeature;
use App\Models\MembershipUsage;
use App\Models\BillingHistory;
use App\Models\SubscriptionEvent;
use App\Events\MembershipGranted;
use App\Events\MembershipExpired;
use App\Events\MembershipUpgraded;
use App\Events\UsageLimitReached;
use App\Exceptions\MembershipException;
use App\Exceptions\BillingException;
use App\Enums\MembershipStatus;
use App\Enums\BillingCycle;
use App\Enums\PaymentProvider;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Redis;
use Illuminate\Support\Collection;
use Carbon\Carbon;
use Stripe\Stripe;
use Stripe\Subscription;

/**
 * Membership Service - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ENTERPRISE FEATURES:
 * 
 * 1. ACCESS CONTROL:
 *    - Hierarchical permissions
 *    - Feature flags
 *    - Usage limits
 *    - Time-based access
 *    - Geographic restrictions
 *    - Device limits
 * 
 * 2. BILLING MANAGEMENT:
 *    - Multiple payment providers
 *    - Proration handling
 *    - Dunning management
 *    - Tax calculation
 *    - Invoice generation
 *    - Refund processing
 * 
 * 3. SUBSCRIPTION LIFECYCLE:
 *    - Trial periods
 *    - Grace periods
 *    - Pause/Resume
 *    - Upgrades/Downgrades
 *    - Cancellations
 *    - Renewals
 * 
 * 4. USAGE TRACKING:
 *    - API calls
 *    - Storage usage
 *    - Bandwidth
 *    - Feature usage
 *    - Concurrent users
 *    - Rate limiting
 * 
 * 5. ANALYTICS:
 *    - Churn prediction
 *    - Usage patterns
 *    - Revenue metrics
 *    - Cohort analysis
 *    - LTV calculation
 *    - Growth metrics
 * 
 * @version 3.0.0 (Google L7+ Enterprise)
 */
class MembershipService
{
    /**
     * Cache configuration
     */
    private const CACHE_TTL = 300; // 5 minutes
    private const CACHE_LONG_TTL = 3600; // 1 hour
    private const CACHE_PREFIX = 'membership:';
    
    /**
     * Rate limiting configuration
     */
    private const RATE_LIMIT_WINDOW = 60; // 1 minute
    private const DEFAULT_RATE_LIMIT = 100; // requests per minute
    
    /**
     * Payment providers
     */
    private array $paymentProviders;
    
    /**
     * Constructor
     */
    public function __construct()
    {
        $this->initializePaymentProviders();
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Access Control Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Check if user has access to a specific feature with comprehensive validation
     * 
     * @param User $user
     * @param string $featureCode
     * @param array $context Additional context for validation
     * @return bool
     */
    public function userHasFeature(User $user, string $featureCode, array $context = []): bool
    {
        $cacheKey = $this->getCacheKey($user->id, 'feature', $featureCode);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($user, $featureCode, $context) {
            // Check active membership
            $membership = $this->getActiveMembership($user);
            
            if (!$membership) {
                return false;
            }
            
            // Check feature availability in plan
            $hasFeature = $membership->plan->features()
                ->where('feature_code', $featureCode)
                ->exists();
            
            if (!$hasFeature) {
                return false;
            }
            
            // Check usage limits
            if (!$this->checkUsageLimits($user, $featureCode)) {
                return false;
            }
            
            // Check geographic restrictions
            if (!$this->checkGeographicRestrictions($user, $membership, $context)) {
                return false;
            }
            
            // Check device limits
            if (!$this->checkDeviceLimits($user, $membership, $context)) {
                return false;
            }
            
            // Check time-based restrictions
            if (!$this->checkTimeRestrictions($membership)) {
                return false;
            }
            
            return true;
        });
    }
    
    /**
     * Check if user has a specific plan with enhanced validation
     * 
     * @param User $user
     * @param string $planSlug
     * @return bool
     */
    public function userHasPlan(User $user, string $planSlug): bool
    {
        $cacheKey = $this->getCacheKey($user->id, 'plan', $planSlug);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($user, $planSlug) {
            return $user->memberships()
                ->where('status', MembershipStatus::ACTIVE)
                ->where(function ($query) {
                    $query->whereNull('expires_at')
                        ->orWhere('expires_at', '>', now());
                })
                ->whereHas('plan', function ($query) use ($planSlug) {
                    $query->where('slug', $planSlug)
                        ->where('is_active', true);
                })
                ->exists();
        });
    }
    
    /**
     * Check if user owns a specific product
     * 
     * @param User $user
     * @param string $productSlug
     * @return bool
     */
    public function userOwnsProduct(User $user, string $productSlug): bool
    {
        $cacheKey = $this->getCacheKey($user->id, 'product', $productSlug);
        
        return Cache::remember($cacheKey, self::CACHE_LONG_TTL, function () use ($user, $productSlug) {
            return $user->products()
                ->where('slug', $productSlug)
                ->wherePivot('revoked_at', null)
                ->exists();
        });
    }
    
    /**
     * Get user's permission level
     * 
     * @param User $user
     * @return string
     */
    public function getUserPermissionLevel(User $user): string
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return 'free';
        }
        
        return $membership->plan->permission_level ?? 'basic';
    }
    
    /**
     * Check if user can perform action
     * 
     * @param User $user
     * @param string $action
     * @param mixed $resource
     * @return bool
     */
    public function userCanPerform(User $user, string $action, $resource = null): bool
    {
        // Get user's permissions
        $permissions = $this->getUserPermissions($user);
        
        // Check if action is allowed
        if (!in_array($action, $permissions)) {
            return false;
        }
        
        // Check resource-specific permissions
        if ($resource) {
            return $this->checkResourcePermission($user, $action, $resource);
        }
        
        return true;
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Membership Management Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Grant membership to user with comprehensive setup
     * 
     * @param User $user
     * @param int $planId
     * @param array $data
     * @return Membership
     * @throws MembershipException
     */
    public function grantMembership(User $user, int $planId, array $data = []): Membership
    {
        DB::beginTransaction();
        
        try {
            // Validate plan
            $plan = MembershipPlan::findOrFail($planId);
            
            if (!$plan->is_active) {
                throw new MembershipException('Plan is not active');
            }
            
            // Check for existing active membership
            $existingMembership = $this->getActiveMembership($user);
            
            if ($existingMembership) {
                // Handle upgrade/downgrade
                return $this->handleMembershipChange($user, $existingMembership, $plan, $data);
            }
            
            // Calculate dates
            $startsAt = $data['starts_at'] ?? now();
            $expiresAt = $this->calculateExpiryDate($plan, $startsAt, $data);
            
            // Create membership
            $membership = $user->memberships()->create([
                'plan_id' => $planId,
                'starts_at' => $startsAt,
                'expires_at' => $expiresAt,
                'trial_ends_at' => $this->calculateTrialEndDate($plan, $startsAt),
                'status' => MembershipStatus::ACTIVE,
                'payment_provider' => $data['payment_provider'] ?? null,
                'subscription_id' => $data['subscription_id'] ?? null,
                'stripe_subscription_id' => $data['stripe_subscription_id'] ?? null,
                'paypal_subscription_id' => $data['paypal_subscription_id'] ?? null,
                'meta' => $data['meta'] ?? [],
                'cancelled_at' => null,
                'cancellation_reason' => null,
            ]);
            
            // Set up usage tracking
            $this->initializeUsageTracking($membership);
            
            // Set up billing
            if (isset($data['payment_method'])) {
                $this->setupBilling($membership, $data['payment_method']);
            }
            
            // Grant initial benefits
            $this->grantMembershipBenefits($membership);
            
            // Clear cache
            $this->clearUserCache($user->id);
            
            // Fire event
            event(new MembershipGranted($membership));
            
            // Log activity
            $this->logMembershipActivity($membership, 'granted', $data);
            
            DB::commit();
            
            return $membership;
            
        } catch (\Exception $e) {
            DB::rollBack();
            
            Log::error('Failed to grant membership', [
                'user_id' => $user->id,
                'plan_id' => $planId,
                'error' => $e->getMessage()
            ]);
            
            throw new MembershipException('Failed to grant membership: ' . $e->getMessage());
        }
    }
    
    /**
     * Cancel membership
     * 
     * @param User $user
     * @param string $reason
     * @param bool $immediate
     * @return bool
     */
    public function cancelMembership(User $user, string $reason = '', bool $immediate = false): bool
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return false;
        }
        
        DB::beginTransaction();
        
        try {
            if ($immediate) {
                // Immediate cancellation
                $membership->update([
                    'status' => MembershipStatus::CANCELLED,
                    'expires_at' => now(),
                    'cancelled_at' => now(),
                    'cancellation_reason' => $reason
                ]);
                
                // Cancel subscription with provider
                $this->cancelProviderSubscription($membership);
                
            } else {
                // Cancel at end of period
                $membership->update([
                    'cancelled_at' => now(),
                    'cancellation_reason' => $reason,
                    'auto_renew' => false
                ]);
                
                // Schedule cancellation with provider
                $this->scheduleProviderCancellation($membership);
            }
            
            // Clear cache
            $this->clearUserCache($user->id);
            
            // Log activity
            $this->logMembershipActivity($membership, 'cancelled', ['reason' => $reason]);
            
            DB::commit();
            
            return true;
            
        } catch (\Exception $e) {
            DB::rollBack();
            
            Log::error('Failed to cancel membership', [
                'membership_id' => $membership->id,
                'error' => $e->getMessage()
            ]);
            
            return false;
        }
    }
    
    /**
     * Pause membership
     * 
     * @param User $user
     * @param Carbon $resumeDate
     * @return bool
     */
    public function pauseMembership(User $user, Carbon $resumeDate = null): bool
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return false;
        }
        
        // Check if plan allows pausing
        if (!$membership->plan->allow_pause) {
            throw new MembershipException('This plan does not allow pausing');
        }
        
        DB::beginTransaction();
        
        try {
            $membership->update([
                'status' => MembershipStatus::PAUSED,
                'paused_at' => now(),
                'resume_at' => $resumeDate,
                'pause_count' => $membership->pause_count + 1
            ]);
            
            // Pause provider subscription
            $this->pauseProviderSubscription($membership);
            
            // Clear cache
            $this->clearUserCache($user->id);
            
            DB::commit();
            
            return true;
            
        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('Failed to pause membership', ['error' => $e->getMessage()]);
            return false;
        }
    }
    
    /**
     * Resume membership
     * 
     * @param User $user
     * @return bool
     */
    public function resumeMembership(User $user): bool
    {
        $membership = $user->memberships()
            ->where('status', MembershipStatus::PAUSED)
            ->first();
        
        if (!$membership) {
            return false;
        }
        
        DB::beginTransaction();
        
        try {
            // Calculate new expiry date
            $pauseDuration = now()->diffInDays($membership->paused_at);
            $newExpiryDate = $membership->expires_at->addDays($pauseDuration);
            
            $membership->update([
                'status' => MembershipStatus::ACTIVE,
                'expires_at' => $newExpiryDate,
                'paused_at' => null,
                'resume_at' => null
            ]);
            
            // Resume provider subscription
            $this->resumeProviderSubscription($membership);
            
            // Clear cache
            $this->clearUserCache($user->id);
            
            DB::commit();
            
            return true;
            
        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('Failed to resume membership', ['error' => $e->getMessage()]);
            return false;
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Product Management Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Grant product to user with enhanced tracking
     * 
     * @param User $user
     * @param int $productId
     * @param int $orderId
     * @param array $data
     * @return void
     */
    public function grantProduct(User $user, int $productId, int $orderId, array $data = []): void
    {
        DB::beginTransaction();
        
        try {
            $product = Product::findOrFail($productId);
            
            // Check if user already owns product
            if ($this->userOwnsProduct($user, $product->slug)) {
                throw new MembershipException('User already owns this product');
            }
            
            // Attach product with metadata
            $user->products()->attach($productId, [
                'purchased_at' => now(),
                'order_id' => $orderId,
                'license_key' => $this->generateLicenseKey($product),
                'download_count' => 0,
                'last_download_at' => null,
                'expires_at' => $this->calculateProductExpiry($product),
                'meta' => json_encode($data),
                'revoked_at' => null
            ]);
            
            // Grant associated features
            $this->grantProductFeatures($user, $product);
            
            // Send product delivery
            $this->deliverProduct($user, $product);
            
            // Clear cache
            $this->clearProductCache($user->id, $product->slug);
            
            // Log activity
            $this->logProductActivity($user, $product, 'granted');
            
            DB::commit();
            
        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('Failed to grant product', ['error' => $e->getMessage()]);
            throw $e;
        }
    }
    
    /**
     * Revoke product from user
     * 
     * @param User $user
     * @param string $productSlug
     * @param string $reason
     * @return bool
     */
    public function revokeProduct(User $user, string $productSlug, string $reason = ''): bool
    {
        $product = Product::where('slug', $productSlug)->first();
        
        if (!$product) {
            return false;
        }
        
        DB::beginTransaction();
        
        try {
            $user->products()->updateExistingPivot($product->id, [
                'revoked_at' => now(),
                'revoke_reason' => $reason
            ]);
            
            // Revoke associated features
            $this->revokeProductFeatures($user, $product);
            
            // Clear cache
            $this->clearProductCache($user->id, $productSlug);
            
            // Log activity
            $this->logProductActivity($user, $product, 'revoked', ['reason' => $reason]);
            
            DB::commit();
            
            return true;
            
        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('Failed to revoke product', ['error' => $e->getMessage()]);
            return false;
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Usage Tracking Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Track feature usage
     * 
     * @param User $user
     * @param string $featureCode
     * @param int $amount
     * @return bool
     */
    public function trackUsage(User $user, string $featureCode, int $amount = 1): bool
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return false;
        }
        
        // Get feature limits
        $feature = $membership->plan->features()
            ->where('feature_code', $featureCode)
            ->first();
        
        if (!$feature) {
            return false;
        }
        
        // Track usage
        $usage = MembershipUsage::firstOrCreate([
            'membership_id' => $membership->id,
            'feature_code' => $featureCode,
            'period' => now()->format('Y-m')
        ]);
        
        $usage->increment('usage_count', $amount);
        
        // Check if limit exceeded
        if ($feature->pivot->limit && $usage->usage_count > $feature->pivot->limit) {
            event(new UsageLimitReached($membership, $featureCode, $usage->usage_count));
            return false;
        }
        
        return true;
    }
    
    /**
     * Get usage statistics for user
     * 
     * @param User $user
     * @param Carbon $startDate
     * @param Carbon $endDate
     * @return Collection
     */
    public function getUserUsageStats(User $user, Carbon $startDate = null, Carbon $endDate = null): Collection
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return collect();
        }
        
        $query = MembershipUsage::where('membership_id', $membership->id);
        
        if ($startDate) {
            $query->where('created_at', '>=', $startDate);
        }
        
        if ($endDate) {
            $query->where('created_at', '<=', $endDate);
        }
        
        return $query->get()->map(function ($usage) use ($membership) {
            $feature = $membership->plan->features()
                ->where('feature_code', $usage->feature_code)
                ->first();
            
            return [
                'feature' => $usage->feature_code,
                'usage' => $usage->usage_count,
                'limit' => $feature->pivot->limit ?? null,
                'percentage' => $feature->pivot->limit 
                    ? round(($usage->usage_count / $feature->pivot->limit) * 100, 2)
                    : null,
                'period' => $usage->period
            ];
        });
    }
    
    /**
     * Check rate limits
     * 
     * @param User $user
     * @param string $action
     * @return bool
     */
    public function checkRateLimit(User $user, string $action): bool
    {
        $membership = $this->getActiveMembership($user);
        $limit = $membership ? $membership->plan->rate_limit : self::DEFAULT_RATE_LIMIT;
        
        $key = "rate_limit:{$user->id}:{$action}";
        $attempts = Redis::incr($key);
        
        if ($attempts === 1) {
            Redis::expire($key, self::RATE_LIMIT_WINDOW);
        }
        
        return $attempts <= $limit;
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Billing Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Process subscription payment
     * 
     * @param Membership $membership
     * @param array $paymentData
     * @return bool
     */
    public function processPayment(Membership $membership, array $paymentData): bool
    {
        DB::beginTransaction();
        
        try {
            // Get payment provider
            $provider = $this->getPaymentProvider($membership->payment_provider);
            
            // Process payment
            $result = $provider->charge(
                $membership->plan->price,
                $paymentData
            );
            
            if (!$result['success']) {
                throw new BillingException($result['message']);
            }
            
            // Record billing history
            BillingHistory::create([
                'membership_id' => $membership->id,
                'amount' => $membership->plan->price,
                'currency' => $membership->plan->currency,
                'status' => 'success',
                'transaction_id' => $result['transaction_id'],
                'payment_method' => $paymentData['method'],
                'provider' => $membership->payment_provider,
                'billed_at' => now(),
                'meta' => $result['meta'] ?? []
            ]);
            
            // Update membership
            $membership->update([
                'last_payment_at' => now(),
                'next_payment_at' => $this->calculateNextPaymentDate($membership),
                'payment_failures' => 0
            ]);
            
            DB::commit();
            
            return true;
            
        } catch (\Exception $e) {
            DB::rollBack();
            
            // Record failed payment
            BillingHistory::create([
                'membership_id' => $membership->id,
                'amount' => $membership->plan->price,
                'currency' => $membership->plan->currency,
                'status' => 'failed',
                'error_message' => $e->getMessage(),
                'payment_method' => $paymentData['method'],
                'provider' => $membership->payment_provider,
                'billed_at' => now()
            ]);
            
            // Increment failure count
            $membership->increment('payment_failures');
            
            // Handle dunning
            $this->handlePaymentFailure($membership);
            
            return false;
        }
    }
    
    /**
     * Calculate proration for plan change
     * 
     * @param Membership $membership
     * @param MembershipPlan $newPlan
     * @return float
     */
    public function calculateProration(Membership $membership, MembershipPlan $newPlan): float
    {
        $currentPlan = $membership->plan;
        $daysRemaining = now()->diffInDays($membership->expires_at);
        $totalDays = $membership->starts_at->diffInDays($membership->expires_at);
        
        // Calculate unused portion of current plan
        $unusedAmount = ($currentPlan->price / $totalDays) * $daysRemaining;
        
        // Calculate cost of new plan for remaining period
        $newPlanCost = ($newPlan->price / $totalDays) * $daysRemaining;
        
        // Return difference
        return round($newPlanCost - $unusedAmount, 2);
    }
    
    /**
     * Generate invoice
     * 
     * @param Membership $membership
     * @param Carbon $billingPeriod
     * @return array
     */
    public function generateInvoice(Membership $membership, Carbon $billingPeriod): array
    {
        $lineItems = [];
        $subtotal = 0;
        
        // Add subscription fee
        $lineItems[] = [
            'description' => $membership->plan->name . ' Subscription',
            'quantity' => 1,
            'unit_price' => $membership->plan->price,
            'total' => $membership->plan->price
        ];
        $subtotal += $membership->plan->price;
        
        // Add usage charges
        $usageCharges = $this->calculateUsageCharges($membership, $billingPeriod);
        foreach ($usageCharges as $charge) {
            $lineItems[] = $charge;
            $subtotal += $charge['total'];
        }
        
        // Calculate tax
        $taxRate = $this->getTaxRate($membership->user);
        $tax = round($subtotal * $taxRate, 2);
        
        // Calculate total
        $total = $subtotal + $tax;
        
        return [
            'invoice_number' => $this->generateInvoiceNumber($membership),
            'date' => now()->toDateString(),
            'due_date' => now()->addDays(30)->toDateString(),
            'customer' => [
                'name' => $membership->user->name,
                'email' => $membership->user->email,
                'address' => $membership->user->billing_address
            ],
            'line_items' => $lineItems,
            'subtotal' => $subtotal,
            'tax_rate' => $taxRate,
            'tax' => $tax,
            'total' => $total,
            'currency' => $membership->plan->currency,
            'payment_terms' => 'Net 30'
        ];
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Analytics Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Get membership analytics
     * 
     * @param Carbon $startDate
     * @param Carbon $endDate
     * @return array
     */
    public function getMembershipAnalytics(Carbon $startDate, Carbon $endDate): array
    {
        return Cache::remember(
            "analytics:membership:{$startDate->format('Y-m-d')}:{$endDate->format('Y-m-d')}",
            3600,
            function () use ($startDate, $endDate) {
                return [
                    'summary' => $this->getAnalyticsSummary($startDate, $endDate),
                    'revenue' => $this->getRevenueMetrics($startDate, $endDate),
                    'churn' => $this->getChurnMetrics($startDate, $endDate),
                    'growth' => $this->getGrowthMetrics($startDate, $endDate),
                    'usage' => $this->getUsageMetrics($startDate, $endDate),
                    'cohorts' => $this->getCohortAnalysis($startDate, $endDate)
                ];
            }
        );
    }
    
    /**
     * Calculate customer lifetime value
     * 
     * @param User $user
     * @return float
     */
    public function calculateLTV(User $user): float
    {
        $totalRevenue = BillingHistory::whereHas('membership', function ($query) use ($user) {
            $query->where('user_id', $user->id);
        })
        ->where('status', 'success')
        ->sum('amount');
        
        $membershipMonths = $user->memberships()
            ->selectRaw('SUM(TIMESTAMPDIFF(MONTH, starts_at, COALESCE(expires_at, NOW()))) as months')
            ->value('months') ?? 1;
        
        $averageMonthlyRevenue = $totalRevenue / max($membershipMonths, 1);
        
        // Estimate based on average retention
        $averageRetentionMonths = $this->getAverageRetentionMonths();
        
        return round($averageMonthlyRevenue * $averageRetentionMonths, 2);
    }
    
    /**
     * Predict churn probability
     * 
     * @param User $user
     * @return float
     */
    public function predictChurnProbability(User $user): float
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return 0;
        }
        
        $factors = [];
        
        // Usage frequency
        $lastUsage = MembershipUsage::where('membership_id', $membership->id)
            ->orderBy('created_at', 'desc')
            ->first();
        
        $daysSinceLastUsage = $lastUsage 
            ? now()->diffInDays($lastUsage->created_at)
            : 999;
        
        $factors['usage'] = min($daysSinceLastUsage / 30, 1); // Normalize to 0-1
        
        // Payment failures
        $factors['payment'] = min($membership->payment_failures / 3, 1);
        
        // Support tickets
        $openTickets = $user->supportTickets()
            ->where('status', 'open')
            ->count();
        
        $factors['support'] = min($openTickets / 5, 1);
        
        // Engagement score
        $engagementScore = $this->calculateEngagementScore($user);
        $factors['engagement'] = 1 - ($engagementScore / 100);
        
        // Calculate weighted average
        $weights = [
            'usage' => 0.4,
            'payment' => 0.3,
            'support' => 0.15,
            'engagement' => 0.15
        ];
        
        $probability = 0;
        foreach ($factors as $key => $value) {
            $probability += $value * $weights[$key];
        }
        
        return round($probability * 100, 2);
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Helper Methods
    // ═══════════════════════════════════════════════════════════════════════════
    
    /**
     * Get active membership for user
     * 
     * @param User $user
     * @return Membership|null
     */
    private function getActiveMembership(User $user): ?Membership
    {
        return $user->memberships()
            ->where('status', MembershipStatus::ACTIVE)
            ->where(function ($query) {
                $query->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            })
            ->orderBy('expires_at', 'desc')
            ->first();
    }
    
    /**
     * Clear user cache
     * 
     * @param int $userId
     * @return void
     */
    private function clearUserCache(int $userId): void
    {
        // Clear specific cache patterns
        $patterns = [
            self::CACHE_PREFIX . "user.{$userId}.*",
            self::CACHE_PREFIX . "permissions.{$userId}",
            self::CACHE_PREFIX . "features.{$userId}.*"
        ];
        
        // If using Redis
        if (Cache::getStore() instanceof \Illuminate\Cache\RedisStore) {
            foreach ($patterns as $pattern) {
                $keys = Redis::keys($pattern);
                if (!empty($keys)) {
                    Redis::del($keys);
                }
            }
        } else {
            // For other cache drivers, we'll need to track keys manually
            // or use tags if supported
            Cache::flush(); // Not ideal but works
        }
    }
    
    /**
     * Clear product cache
     * 
     * @param int $userId
     * @param string $productSlug
     * @return void
     */
    private function clearProductCache(int $userId, string $productSlug): void
    {
        $cacheKey = $this->getCacheKey($userId, 'product', $productSlug);
        Cache::forget($cacheKey);
    }
    
    /**
     * Generate cache key
     * 
     * @param int $userId
     * @param string $type
     * @param string $identifier
     * @return string
     */
    private function getCacheKey(int $userId, string $type, string $identifier): string
    {
        return self::CACHE_PREFIX . "user.{$userId}.{$type}.{$identifier}";
    }
    
    /**
     * Calculate expiry date
     * 
     * @param MembershipPlan $plan
     * @param Carbon $startsAt
     * @param array $data
     * @return Carbon|null
     */
    private function calculateExpiryDate(MembershipPlan $plan, Carbon $startsAt, array $data): ?Carbon
    {
        if (isset($data['expires_at'])) {
            return Carbon::parse($data['expires_at']);
        }
        
        switch ($plan->billing_cycle) {
            case BillingCycle::MONTHLY:
                return $startsAt->copy()->addMonth();
            case BillingCycle::QUARTERLY:
                return $startsAt->copy()->addMonths(3);
            case BillingCycle::YEARLY:
                return $startsAt->copy()->addYear();
            case BillingCycle::LIFETIME:
                return null;
            default:
                return $startsAt->copy()->addMonth();
        }
    }
    
    /**
     * Calculate trial end date
     * 
     * @param MembershipPlan $plan
     * @param Carbon $startsAt
     * @return Carbon|null
     */
    private function calculateTrialEndDate(MembershipPlan $plan, Carbon $startsAt): ?Carbon
    {
        if ($plan->trial_days > 0) {
            return $startsAt->copy()->addDays($plan->trial_days);
        }
        
        return null;
    }
    
    /**
     * Initialize payment providers
     * 
     * @return void
     */
    private function initializePaymentProviders(): void
    {
        $this->paymentProviders = [
            'stripe' => app(\App\Services\Payments\StripeProvider::class),
            'paypal' => app(\App\Services\Payments\PayPalProvider::class),
            'paddle' => app(\App\Services\Payments\PaddleProvider::class),
        ];
    }
    
    /**
     * Get payment provider
     * 
     * @param string $provider
     * @return mixed
     */
    private function getPaymentProvider(string $provider)
    {
        if (!isset($this->paymentProviders[$provider])) {
            throw new BillingException("Payment provider {$provider} not found");
        }
        
        return $this->paymentProviders[$provider];
    }
    
    /**
     * Log membership activity
     * 
     * @param Membership $membership
     * @param string $action
     * @param array $data
     * @return void
     */
    private function logMembershipActivity(Membership $membership, string $action, array $data = []): void
    {
        Log::info("Membership {$action}", [
            'membership_id' => $membership->id,
            'user_id' => $membership->user_id,
            'plan_id' => $membership->plan_id,
            'action' => $action,
            'data' => $data
        ]);
    }
    
    /**
     * Check usage limits
     * 
     * @param User $user
     * @param string $featureCode
     * @return bool
     */
    private function checkUsageLimits(User $user, string $featureCode): bool
    {
        $membership = $this->getActiveMembership($user);
        
        if (!$membership) {
            return false;
        }
        
        $feature = $membership->plan->features()
            ->where('feature_code', $featureCode)
            ->first();
        
        if (!$feature || !$feature->pivot->limit) {
            return true;
        }
        
        $usage = MembershipUsage::where('membership_id', $membership->id)
            ->where('feature_code', $featureCode)
            ->where('period', now()->format('Y-m'))
            ->first();
        
        if (!$usage) {
            return true;
        }
        
        return $usage->usage_count < $feature->pivot->limit;
    }
    
    /**
     * Initialize usage tracking for new membership
     * 
     * @param Membership $membership
     * @return void
     */
    private function initializeUsageTracking(Membership $membership): void
    {
        foreach ($membership->plan->features as $feature) {
            if ($feature->pivot->limit) {
                MembershipUsage::create([
                    'membership_id' => $membership->id,
                    'feature_code' => $feature->feature_code,
                    'period' => now()->format('Y-m'),
                    'usage_count' => 0,
                    'limit' => $feature->pivot->limit
                ]);
            }
        }
    }
    
    /**
     * Handle membership upgrade/downgrade
     * 
     * @param User $user
     * @param Membership $currentMembership
     * @param MembershipPlan $newPlan
     * @param array $data
     * @return Membership
     */
    private function handleMembershipChange(
        User $user,
        Membership $currentMembership,
        MembershipPlan $newPlan,
        array $data
    ): Membership {
        // Calculate proration
        $prorationAmount = $this->calculateProration($currentMembership, $newPlan);
        
        // Cancel current membership
        $currentMembership->update([
            'status' => MembershipStatus::UPGRADED,
            'upgraded_at' => now(),
            'upgraded_to' => $newPlan->id
        ]);
        
        // Create new membership
        $newMembership = $this->grantMembership($user, $newPlan->id, array_merge($data, [
            'proration_credit' => $prorationAmount < 0 ? abs($prorationAmount) : 0,
            'proration_charge' => $prorationAmount > 0 ? $prorationAmount : 0,
            'upgraded_from' => $currentMembership->id
        ]));
        
        // Fire event
        event(new MembershipUpgraded($currentMembership, $newMembership));
        
        return $newMembership;
    }
}