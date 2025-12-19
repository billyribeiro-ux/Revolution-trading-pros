<?php

namespace App\Services;

use App\Enums\SubscriptionInterval;
use App\Enums\SubscriptionStatus;
use App\Events\SubscriptionCancelled;
use App\Events\SubscriptionCreated;
use App\Events\SubscriptionPaused;
use App\Events\SubscriptionRenewed;
use App\Events\SubscriptionResumed;
use App\Models\SubscriptionPlan;
use App\Models\User;
use App\Models\UserSubscription;
use Carbon\Carbon;
use Illuminate\Support\Facades\DB;

class SubscriptionService
{
    /**
     * Create a new subscription for a user.
     */
    public function create(User $user, SubscriptionPlan $plan, array $data = []): UserSubscription
    {
        return DB::transaction(function () use ($user, $plan, $data) {
            $startDate = $data['start_immediately'] ?? true ? now() : null;
            $trialDays = $data['trial_days'] ?? $plan->trial_days;
            
            // Determine initial status
            $status = SubscriptionStatus::Active;
            if ($trialDays > 0) {
                $status = SubscriptionStatus::Trial;
            }
            if (isset($data['status'])) {
                $status = SubscriptionStatus::from($data['status']);
            }

            $subscription = UserSubscription::create([
                'user_id' => $user->id,
                'subscription_plan_id' => $plan->id,
                'status' => $status,
                'trial_ends_at' => $trialDays > 0 ? now()->addDays($trialDays) : null,
                'current_period_start' => $startDate,
                'current_period_end' => $startDate ? $this->calculatePeriodEnd($startDate, $plan) : null,
                'payment_method' => $data['payment_method'] ?? 'manual',
                'payment_id' => $data['payment_id'] ?? null,
                'amount_paid' => $data['amount_paid'] ?? 0,
                'notes' => $data['notes'] ?? null,
            ]);

            event(new SubscriptionCreated($subscription));

            return $subscription;
        });
    }

    /**
     * Cancel a subscription.
     */
    public function cancel(UserSubscription $subscription, bool $immediate = false, ?string $reason = null): UserSubscription
    {
        return DB::transaction(function () use ($subscription, $immediate, $reason) {
            $updateData = [
                'cancelled_at' => now(),
                'notes' => $reason ? ($subscription->notes . "\nCancellation Reason: " . $reason) : $subscription->notes,
            ];

            if ($immediate) {
                $updateData['status'] = SubscriptionStatus::Cancelled;
                $updateData['expires_at'] = now();
            } else {
                $updateData['status'] = SubscriptionStatus::PendingCancel;
                // Expires at end of current period
                $updateData['expires_at'] = $subscription->current_period_end;
            }

            $subscription->update($updateData);

            event(new SubscriptionCancelled($subscription));

            return $subscription;
        });
    }

    /**
     * Pause a subscription.
     */
    public function pause(UserSubscription $subscription, ?string $reason = null): UserSubscription
    {
        return DB::transaction(function () use ($subscription, $reason) {
            $subscription->update([
                'status' => SubscriptionStatus::OnHold,
                'paused_at' => now(),
                'notes' => $reason ? ($subscription->notes . "\nPause Reason: " . $reason) : $subscription->notes,
            ]);

            event(new SubscriptionPaused($subscription));

            return $subscription;
        });
    }

    /**
     * Resume a subscription.
     */
    public function resume(UserSubscription $subscription): UserSubscription
    {
        return DB::transaction(function () use ($subscription) {
            // Calculate how long it was paused to adjust period end if needed
            $pausedDuration = $subscription->paused_at ? now()->diffInSeconds($subscription->paused_at) : 0;
            
            $updateData = [
                'paused_at' => null,
            ];

            // Restore status based on trial validity
            if ($subscription->trial_ends_at && $subscription->trial_ends_at->isFuture()) {
                $updateData['status'] = SubscriptionStatus::Trial;
            } else {
                $updateData['status'] = SubscriptionStatus::Active;
            }

            // Extend period end by paused duration if applicable
            if ($subscription->current_period_end && $pausedDuration > 0) {
                $updateData['current_period_end'] = $subscription->current_period_end->addSeconds($pausedDuration);
            }
            
            // Extend trial end if it was paused during trial? 
            // User said "free trial days should never be deleted". 
            // If we pause during trial, we should probably extend the trial end date by the paused duration.
            if ($subscription->trial_ends_at && $pausedDuration > 0) {
                $updateData['trial_ends_at'] = $subscription->trial_ends_at->addSeconds($pausedDuration);
            }

            $subscription->update($updateData);

            event(new SubscriptionResumed($subscription));

            return $subscription;
        });
    }

    /**
     * Renew a subscription.
     */
    public function renew(UserSubscription $subscription): UserSubscription
    {
        return DB::transaction(function () use ($subscription) {
            $plan = $subscription->plan;
            $nextPeriodStart = $subscription->current_period_end ?? now();
            
            $subscription->update([
                'status' => SubscriptionStatus::Active,
                'current_period_start' => $nextPeriodStart,
                'current_period_end' => $this->calculatePeriodEnd($nextPeriodStart, $plan),
                'billing_cycles_completed' => $subscription->billing_cycles_completed + 1,
            ]);

            event(new SubscriptionRenewed($subscription));

            return $subscription;
        });
    }

    /**
     * Reactivate a cancelled or pending-cancel subscription.
     */
    public function reactivate(UserSubscription $subscription): UserSubscription
    {
        return DB::transaction(function () use ($subscription) {
            $updateData = [
                'cancelled_at' => null,
                'expires_at' => null,
            ];

            // If subscription hasn't expired yet, restore to active
            if ($subscription->current_period_end && $subscription->current_period_end->isFuture()) {
                $updateData['status'] = SubscriptionStatus::Active;
            } else {
                // If expired, need to renew
                $plan = $subscription->plan;
                $updateData['status'] = SubscriptionStatus::Active;
                $updateData['current_period_start'] = now();
                $updateData['current_period_end'] = $this->calculatePeriodEnd(now(), $plan);
            }

            $subscription->update($updateData);

            event(new SubscriptionResumed($subscription)); // Reuse resumed event or create new

            return $subscription;
        });
    }

    /**
     * Calculate period end date based on plan.
     */
    protected function calculatePeriodEnd(Carbon $start, SubscriptionPlan $plan): Carbon
    {
        $interval = $plan->billing_interval;
        $period = $plan->billing_period instanceof SubscriptionInterval 
            ? $plan->billing_period 
            : (SubscriptionInterval::tryFrom($plan->billing_period) ?? SubscriptionInterval::Monthly);
        
        return match($period) {
            SubscriptionInterval::Daily => $start->copy()->addDays($interval),
            SubscriptionInterval::Weekly => $start->copy()->addWeeks($interval),
            SubscriptionInterval::Monthly => $start->copy()->addMonths($interval),
            SubscriptionInterval::Quarterly => $start->copy()->addMonths($interval * 3),
            SubscriptionInterval::Yearly => $start->copy()->addYears($interval),
        };
    }
}
