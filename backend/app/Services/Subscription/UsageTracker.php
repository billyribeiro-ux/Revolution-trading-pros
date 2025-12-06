<?php

declare(strict_types=1);

namespace App\Services\Subscription;

use App\Models\User;
use App\Models\Subscription;
use App\Models\UsageRecord;
use App\Models\Plan;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * Usage Tracker Service (ICT9+ Enterprise Grade)
 *
 * Tracks and manages subscription usage:
 * - API calls
 * - Storage usage
 * - Feature usage
 * - Resource consumption
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class UsageTracker
{
    /**
     * Cache TTL for usage data (5 minutes)
     */
    private const CACHE_TTL = 300;

    /**
     * Record usage for a feature
     */
    public function record(
        User $user,
        string $feature,
        int $quantity = 1,
        array $metadata = []
    ): UsageRecord {
        $subscription = $user->activeSubscription();

        $record = UsageRecord::create([
            'user_id' => $user->id,
            'subscription_id' => $subscription?->id,
            'feature' => $feature,
            'quantity' => $quantity,
            'metadata' => $metadata,
            'recorded_at' => now(),
        ]);

        // Update cached totals
        $this->incrementCachedUsage($user->id, $feature, $quantity);

        // Check if approaching limit
        $this->checkUsageLimits($user, $feature);

        return $record;
    }

    /**
     * Get current usage for a feature
     */
    public function getUsage(User $user, string $feature, ?string $period = null): int
    {
        $period = $period ?? $this->getCurrentPeriod($user);
        $cacheKey = "usage:{$user->id}:{$feature}:{$period}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($user, $feature, $period) {
            [$start, $end] = $this->getPeriodDates($period, $user);

            return UsageRecord::where('user_id', $user->id)
                ->where('feature', $feature)
                ->whereBetween('recorded_at', [$start, $end])
                ->sum('quantity');
        });
    }

    /**
     * Get all usage for a user
     */
    public function getAllUsage(User $user, ?string $period = null): array
    {
        $period = $period ?? $this->getCurrentPeriod($user);
        $cacheKey = "usage:all:{$user->id}:{$period}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($user, $period) {
            [$start, $end] = $this->getPeriodDates($period, $user);

            return UsageRecord::where('user_id', $user->id)
                ->whereBetween('recorded_at', [$start, $end])
                ->select('feature', DB::raw('SUM(quantity) as total'))
                ->groupBy('feature')
                ->pluck('total', 'feature')
                ->toArray();
        });
    }

    /**
     * Get usage limit for a feature
     */
    public function getLimit(User $user, string $feature): ?int
    {
        $subscription = $user->activeSubscription();

        if (!$subscription) {
            // Free tier limits
            return config("subscription.free_limits.{$feature}", 0);
        }

        $plan = $subscription->plan;

        if (!$plan) {
            return null;
        }

        $limits = $plan->limits ?? [];
        return $limits[$feature] ?? null;
    }

    /**
     * Check if user has remaining quota
     */
    public function hasQuota(User $user, string $feature, int $required = 1): bool
    {
        $limit = $this->getLimit($user, $feature);

        // No limit means unlimited
        if ($limit === null) {
            return true;
        }

        $usage = $this->getUsage($user, $feature);
        return ($usage + $required) <= $limit;
    }

    /**
     * Get remaining quota for a feature
     */
    public function getRemainingQuota(User $user, string $feature): ?int
    {
        $limit = $this->getLimit($user, $feature);

        if ($limit === null) {
            return null; // Unlimited
        }

        $usage = $this->getUsage($user, $feature);
        return max(0, $limit - $usage);
    }

    /**
     * Get usage percentage
     */
    public function getUsagePercentage(User $user, string $feature): ?float
    {
        $limit = $this->getLimit($user, $feature);

        if ($limit === null || $limit === 0) {
            return null;
        }

        $usage = $this->getUsage($user, $feature);
        return round(($usage / $limit) * 100, 2);
    }

    /**
     * Get comprehensive usage report
     */
    public function getUsageReport(User $user): array
    {
        $subscription = $user->activeSubscription();
        $plan = $subscription?->plan;
        $limits = $plan?->limits ?? [];
        $currentUsage = $this->getAllUsage($user);

        $report = [
            'period' => $this->getCurrentPeriod($user),
            'period_start' => $subscription?->current_period_start,
            'period_end' => $subscription?->current_period_end,
            'plan' => $plan?->name ?? 'Free',
            'features' => [],
        ];

        // Get all tracked features
        $features = array_unique(array_merge(array_keys($limits), array_keys($currentUsage)));

        foreach ($features as $feature) {
            $limit = $limits[$feature] ?? null;
            $usage = $currentUsage[$feature] ?? 0;

            $report['features'][$feature] = [
                'usage' => $usage,
                'limit' => $limit,
                'remaining' => $limit !== null ? max(0, $limit - $usage) : null,
                'percentage' => $limit !== null && $limit > 0
                    ? round(($usage / $limit) * 100, 2)
                    : null,
                'unlimited' => $limit === null,
            ];
        }

        return $report;
    }

    /**
     * Get usage history
     */
    public function getUsageHistory(
        User $user,
        string $feature,
        int $periods = 12,
        string $granularity = 'day'
    ): array {
        $format = match ($granularity) {
            'hour' => '%Y-%m-%d %H:00',
            'day' => '%Y-%m-%d',
            'week' => '%Y-%W',
            'month' => '%Y-%m',
            default => '%Y-%m-%d',
        };

        return UsageRecord::where('user_id', $user->id)
            ->where('feature', $feature)
            ->where('recorded_at', '>=', $this->getHistoryStartDate($granularity, $periods))
            ->select(
                DB::raw("DATE_FORMAT(recorded_at, '{$format}') as period"),
                DB::raw('SUM(quantity) as total')
            )
            ->groupBy('period')
            ->orderBy('period')
            ->pluck('total', 'period')
            ->toArray();
    }

    /**
     * Reset usage for a new billing period
     */
    public function resetPeriodUsage(Subscription $subscription): void
    {
        $cachePattern = "usage:{$subscription->user_id}:*";

        // Clear cached usage
        Cache::flush(); // In production, use tagged cache or pattern deletion

        Log::info('Usage reset for new billing period', [
            'subscription_id' => $subscription->id,
            'user_id' => $subscription->user_id,
        ]);
    }

    /**
     * Sync usage with Stripe metered billing
     */
    public function syncWithStripe(Subscription $subscription, string $feature): void
    {
        if (!$subscription->stripe_subscription_id) {
            return;
        }

        $usage = $this->getUsage($subscription->user, $feature);

        try {
            $stripe = new \Stripe\StripeClient(config('services.stripe.secret'));

            // Find the subscription item for this feature
            $stripeSubscription = $stripe->subscriptions->retrieve($subscription->stripe_subscription_id);

            foreach ($stripeSubscription->items->data as $item) {
                $price = $stripe->prices->retrieve($item->price->id);

                if ($price->metadata['feature'] === $feature && $price->recurring->usage_type === 'metered') {
                    $stripe->subscriptionItems->createUsageRecord($item->id, [
                        'quantity' => $usage,
                        'action' => 'set',
                        'timestamp' => time(),
                    ]);

                    Log::info('Usage synced with Stripe', [
                        'subscription_id' => $subscription->id,
                        'feature' => $feature,
                        'quantity' => $usage,
                    ]);
                }
            }
        } catch (\Throwable $e) {
            Log::error('Failed to sync usage with Stripe', [
                'subscription_id' => $subscription->id,
                'feature' => $feature,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Increment cached usage
     */
    private function incrementCachedUsage(int $userId, string $feature, int $quantity): void
    {
        $period = $this->getCurrentPeriodFromDate(now());
        $cacheKey = "usage:{$userId}:{$feature}:{$period}";

        if (Cache::has($cacheKey)) {
            Cache::increment($cacheKey, $quantity);
        }

        // Also update the all-usage cache
        $allCacheKey = "usage:all:{$userId}:{$period}";
        Cache::forget($allCacheKey);
    }

    /**
     * Check usage limits and send alerts
     */
    private function checkUsageLimits(User $user, string $feature): void
    {
        $percentage = $this->getUsagePercentage($user, $feature);

        if ($percentage === null) {
            return;
        }

        $thresholds = [80, 90, 100];

        foreach ($thresholds as $threshold) {
            if ($percentage >= $threshold) {
                $alertKey = "usage:alert:{$user->id}:{$feature}:{$threshold}";

                if (!Cache::has($alertKey)) {
                    // Send alert (email/notification)
                    event(new \App\Events\UsageLimitApproaching($user, $feature, $percentage, $threshold));

                    // Don't send this alert again for 24 hours
                    Cache::put($alertKey, true, 86400);

                    Log::info('Usage limit alert sent', [
                        'user_id' => $user->id,
                        'feature' => $feature,
                        'percentage' => $percentage,
                        'threshold' => $threshold,
                    ]);
                }

                break; // Only send highest threshold alert
            }
        }
    }

    /**
     * Get current billing period identifier
     */
    private function getCurrentPeriod(User $user): string
    {
        $subscription = $user->activeSubscription();

        if ($subscription && $subscription->current_period_start) {
            return $subscription->current_period_start->format('Y-m-d');
        }

        // Default to current month for free users
        return now()->startOfMonth()->format('Y-m-d');
    }

    /**
     * Get period identifier from date
     */
    private function getCurrentPeriodFromDate($date): string
    {
        return \Carbon\Carbon::parse($date)->startOfMonth()->format('Y-m-d');
    }

    /**
     * Get period start and end dates
     */
    private function getPeriodDates(string $period, User $user): array
    {
        $subscription = $user->activeSubscription();

        if ($subscription) {
            return [
                $subscription->current_period_start ?? now()->startOfMonth(),
                $subscription->current_period_end ?? now()->endOfMonth(),
            ];
        }

        // Free users: monthly periods
        $start = \Carbon\Carbon::parse($period)->startOfMonth();
        return [$start, $start->copy()->endOfMonth()];
    }

    /**
     * Get history start date based on granularity
     */
    private function getHistoryStartDate(string $granularity, int $periods): \Carbon\Carbon
    {
        return match ($granularity) {
            'hour' => now()->subHours($periods),
            'day' => now()->subDays($periods),
            'week' => now()->subWeeks($periods),
            'month' => now()->subMonths($periods),
            default => now()->subDays($periods),
        };
    }
}
