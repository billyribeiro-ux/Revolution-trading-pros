<?php

declare(strict_types=1);

namespace App\Http\Resources;

use Carbon\Carbon;
use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Support\Facades\Cache;

/**
 * Subscription API Resource
 * 
 * Transforms subscription model data for API responses with enterprise-grade
 * features including computed fields, caching, and conditional loading.
 * 
 * @property-read int $id
 * @property-read int $user_id
 * @property-read string $status
 * @property-read ?Carbon $trial_ends_at
 * @property-read Carbon $current_period_start
 * @property-read Carbon $current_period_end
 * @property-read ?Carbon $cancelled_at
 * @property-read ?Carbon $paused_at
 * @property-read ?Carbon $expires_at
 * @property-read string $payment_method
 * @property-read float $amount_paid
 * @property-read int $billing_cycles_completed
 * @property-read ?string $notes
 * @property-read Carbon $created_at
 * @property-read Carbon $updated_at
 * @property-read \App\Models\User $user
 * @property-read \App\Models\Plan $plan
 * @property-read \Illuminate\Support\Collection $invoices
 * @property-read \Illuminate\Support\Collection $usageRecords
 * 
 * @mixin \App\Models\Subscription
 */
class SubscriptionResource extends JsonResource
{
    /**
     * Additional data to merge with the resource
     */
    protected array $additional = [];

    /**
     * Cache TTL in seconds (5 minutes default)
     */
    protected const CACHE_TTL = 300;

    /**
     * Transform the resource into an array
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    public function toArray(Request $request): array
    {
        $cacheKey = $this->getCacheKey($request);
        
        // Use cache for expensive computed fields
        $computedData = Cache::remember($cacheKey, self::CACHE_TTL, function () {
            return $this->getComputedFields();
        });

        return array_merge([
            // Primary identifiers
            'id' => $this->id,
            'user_id' => $this->user_id,
            
            // Status and state
            'status' => $this->status,
            'status_label' => $this->getStatusLabel(),
            'is_active' => $this->isActive(),
            'is_trialing' => $this->isTrialing(),
            'is_grace_period' => $this->isInGracePeriod(),
            'is_past_due' => $this->isPastDue(),
            
            // Dates with formatted variants
            'trial_ends_at' => $this->formatDate($this->trial_ends_at),
            'current_period_start' => $this->formatDate($this->current_period_start),
            'current_period_end' => $this->formatDate($this->current_period_end),
            'cancelled_at' => $this->formatDate($this->cancelled_at),
            'paused_at' => $this->formatDate($this->paused_at),
            'expires_at' => $this->formatDate($this->expires_at),
            
            // Billing information
            'payment_method' => $this->payment_method,
            'payment_method_label' => $this->getPaymentMethodLabel(),
            'amount_paid' => $this->formatCurrency($this->amount_paid),
            'billing_cycles_completed' => $this->billing_cycles_completed,
            'next_billing_date' => $this->when(
                $this->isActive() && !$this->cancelled_at,
                fn() => $this->formatDate($this->getNextBillingDate())
            ),
            
            // Computed metrics
            'days_until_expiry' => $computedData['days_until_expiry'],
            'days_remaining_trial' => $computedData['days_remaining_trial'],
            'total_revenue' => $this->when(
                $request->user()?->can('view-revenue', $this->resource),
                fn() => $this->formatCurrency($computedData['total_revenue'])
            ),
            'lifetime_value' => $this->when(
                $request->user()?->can('view-revenue', $this->resource),
                fn() => $this->formatCurrency($computedData['lifetime_value'])
            ),
            
            // Usage and limits
            'usage_percentage' => $this->when(
                $this->plan?->has_usage_limits,
                fn() => $computedData['usage_percentage']
            ),
            'usage_summary' => $this->when(
                $this->plan?->has_usage_limits,
                fn() => $this->getUsageSummary()
            ),
            
            // Related resources (conditionally loaded)
            'user' => UserResource::make($this->whenLoaded('user')),
            'plan' => PlanResource::make($this->whenLoaded('plan')),
            'invoices' => InvoiceResource::collection($this->whenLoaded('invoices')),
            'latest_invoice' => $this->when(
                $this->relationLoaded('latestInvoice'),
                fn() => InvoiceResource::make($this->latestInvoice)
            ),
            'usage_records' => UsageRecordResource::collection(
                $this->whenLoaded('usageRecords')
            ),
            
            // Metadata
            'notes' => $this->when($this->notes, $this->notes),
            'metadata' => $this->when(
                $this->metadata && !empty($this->metadata),
                $this->metadata
            ),
            'tags' => $this->when(
                $this->relationLoaded('tags'),
                fn() => $this->tags->pluck('name')
            ),
            
            // Actions available
            'can' => $this->getPermissions($request),
            
            // Timestamps
            'created_at' => $this->formatDate($this->created_at),
            'updated_at' => $this->formatDate($this->updated_at),
            
            // Human-readable durations
            'subscription_age' => $this->getSubscriptionAge(),
            'time_until_renewal' => $this->when(
                $this->isActive() && !$this->cancelled_at,
                fn() => $this->getTimeUntilRenewal()
            ),
        ], $this->additional);
    }

    /**
     * Get computed fields (expensive operations cached)
     *
     * @return array<string, mixed>
     */
    protected function getComputedFields(): array
    {
        return [
            'days_until_expiry' => $this->getDaysUntilExpiry(),
            'days_remaining_trial' => $this->getDaysRemainingTrial(),
            'total_revenue' => $this->calculateTotalRevenue(),
            'lifetime_value' => $this->calculateLifetimeValue(),
            'usage_percentage' => $this->calculateUsagePercentage(),
        ];
    }

    /**
     * Format date with ISO 8601 and human-readable formats
     *
     * @param Carbon|string|null $date
     * @return array<string, string>|null
     */
    protected function formatDate($date): ?array
    {
        if (!$date) {
            return null;
        }

        $carbon = $date instanceof Carbon ? $date : Carbon::parse($date);

        return [
            'iso' => $carbon->toIso8601String(),
            'formatted' => $carbon->format('M j, Y g:i A'),
            'relative' => $carbon->diffForHumans(),
            'timestamp' => $carbon->timestamp,
        ];
    }

    /**
     * Format currency values
     *
     * @param float|int $amount
     * @return array<string, mixed>
     */
    protected function formatCurrency($amount): array
    {
        return [
            'raw' => $amount,
            'formatted' => number_format($amount / 100, 2),
            'display' => '$' . number_format($amount / 100, 2),
            'cents' => (int) $amount,
        ];
    }

    /**
     * Get human-readable status label
     *
     * @return string
     */
    protected function getStatusLabel(): string
    {
        return match($this->status) {
            'active' => 'Active',
            'trialing' => 'Trial Period',
            'past_due' => 'Past Due',
            'cancelled' => 'Cancelled',
            'paused' => 'Paused',
            'expired' => 'Expired',
            'pending' => 'Pending Activation',
            default => ucfirst($this->status),
        };
    }

    /**
     * Get payment method label
     *
     * @return string
     */
    protected function getPaymentMethodLabel(): string
    {
        return match($this->payment_method) {
            'card' => 'Credit Card',
            'paypal' => 'PayPal',
            'stripe' => 'Stripe',
            'bank_transfer' => 'Bank Transfer',
            'crypto' => 'Cryptocurrency',
            default => ucfirst(str_replace('_', ' ', $this->payment_method)),
        };
    }

    /**
     * Calculate days until subscription expiry
     *
     * @return int|null
     */
    protected function getDaysUntilExpiry(): ?int
    {
        if (!$this->expires_at) {
            return null;
        }

        return max(0, Carbon::now()->diffInDays($this->expires_at, false));
    }

    /**
     * Calculate days remaining in trial
     *
     * @return int|null
     */
    protected function getDaysRemainingTrial(): ?int
    {
        if (!$this->trial_ends_at || !$this->isTrialing()) {
            return null;
        }

        return max(0, Carbon::now()->diffInDays($this->trial_ends_at, false));
    }

    /**
     * Get next billing date
     *
     * @return Carbon|null
     */
    protected function getNextBillingDate(): ?Carbon
    {
        if (!$this->isActive() || $this->cancelled_at) {
            return null;
        }

        return $this->current_period_end;
    }

    /**
     * Calculate total revenue from subscription
     *
     * @return float
     */
    protected function calculateTotalRevenue(): float
    {
        // This would typically query the database for all payments
        return $this->invoices()
            ->where('status', 'paid')
            ->sum('amount');
    }

    /**
     * Calculate customer lifetime value
     *
     * @return float
     */
    protected function calculateLifetimeValue(): float
    {
        if (!$this->plan) {
            return 0;
        }

        $monthlyRevenue = $this->plan->price;
        $estimatedMonths = $this->billing_cycles_completed + 
                          ($this->isActive() ? 6 : 0); // Estimate 6 more months if active
        
        return $monthlyRevenue * $estimatedMonths;
    }

    /**
     * Calculate usage percentage
     *
     * @return float
     */
    protected function calculateUsagePercentage(): float
    {
        if (!$this->plan || !$this->plan->has_usage_limits) {
            return 0;
        }

        $usage = $this->usageRecords()
            ->whereBetween('created_at', [
                $this->current_period_start,
                $this->current_period_end
            ])
            ->sum('quantity');

        $limit = $this->plan->usage_limit ?? 0;
        
        return $limit > 0 ? min(100, ($usage / $limit) * 100) : 0;
    }

    /**
     * Get usage summary
     *
     * @return array<string, mixed>
     */
    protected function getUsageSummary(): array
    {
        $usage = $this->usageRecords()
            ->whereBetween('created_at', [
                $this->current_period_start,
                $this->current_period_end
            ])
            ->selectRaw('type, SUM(quantity) as total')
            ->groupBy('type')
            ->get();

        return $usage->mapWithKeys(function ($item) {
            return [$item->type => [
                'used' => $item->total,
                'limit' => $this->plan->limits[$item->type] ?? null,
                'percentage' => isset($this->plan->limits[$item->type]) 
                    ? ($item->total / $this->plan->limits[$item->type]) * 100
                    : null,
            ]];
        })->toArray();
    }

    /**
     * Get subscription age in human-readable format
     *
     * @return string
     */
    protected function getSubscriptionAge(): string
    {
        return $this->created_at->diffForHumans(null, true);
    }

    /**
     * Get time until renewal in human-readable format
     *
     * @return string|null
     */
    protected function getTimeUntilRenewal(): ?string
    {
        if (!$this->current_period_end) {
            return null;
        }

        return $this->current_period_end->diffForHumans();
    }

    /**
     * Get permissions for current user
     *
     * @param Request $request
     * @return array<string, bool>
     */
    protected function getPermissions(Request $request): array
    {
        $user = $request->user();
        
        if (!$user) {
            return [];
        }

        return [
            'cancel' => $user->can('cancel', $this->resource),
            'pause' => $user->can('pause', $this->resource),
            'resume' => $user->can('resume', $this->resource),
            'upgrade' => $user->can('upgrade', $this->resource),
            'downgrade' => $user->can('downgrade', $this->resource),
            'update_payment' => $user->can('updatePayment', $this->resource),
            'view_invoices' => $user->can('viewInvoices', $this->resource),
            'view_usage' => $user->can('viewUsage', $this->resource),
        ];
    }

    /**
     * Generate cache key for computed fields
     *
     * @param Request $request
     * @return string
     */
    protected function getCacheKey(Request $request): string
    {
        $userId = $request->user()?->id ?? 'guest';
        return "subscription_resource_{$this->id}_{$userId}_" . md5(serialize($request->all()));
    }

    /**
     * Check if subscription is active
     *
     * @return bool
     */
    protected function isActive(): bool
    {
        return in_array($this->status, ['active', 'trialing']);
    }

    /**
     * Check if subscription is in trial period
     *
     * @return bool
     */
    protected function isTrialing(): bool
    {
        return $this->status === 'trialing' && 
               $this->trial_ends_at && 
               $this->trial_ends_at->isFuture();
    }

    /**
     * Check if subscription is in grace period
     *
     * @return bool
     */
    protected function isInGracePeriod(): bool
    {
        return $this->cancelled_at && 
               $this->current_period_end && 
               $this->current_period_end->isFuture();
    }

    /**
     * Check if subscription is past due
     *
     * @return bool
     */
    protected function isPastDue(): bool
    {
        return $this->status === 'past_due';
    }

    /**
     * Add additional data to the resource
     *
     * @param array<string, mixed> $data
     * @return static
     */
    public function additional(array $data): static
    {
        $this->additional = $data;
        return $this;
    }

    /**
     * Customize the response for the resource
     *
     * @param Request $request
     * @param \Illuminate\Http\JsonResponse $response
     * @return void
     */
    public function withResponse($request, $response): void
    {
        $response->header('X-Subscription-Version', '2.0');
        
        // Add cache headers for GET requests
        if ($request->isMethod('GET')) {
            $response->header('Cache-Control', 'private, max-age=60');
            $response->header('ETag', md5($response->getContent()));
        }
    }
}