<?php

declare(strict_types=1);

namespace App\Jobs\Subscription;

use App\Models\Subscription;
use App\Mail\Subscription\TrialExpiredMail;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Mail;

/**
 * Process Trial Expirations Job (ICT9+ Enterprise Grade)
 *
 * Handles trial period expirations:
 * - Sends reminder emails before expiration
 * - Converts trials to paid or expired
 * - Cleans up expired trial accounts
 *
 * @version 1.0.0
 */
class ProcessTrialExpirations implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    public int $tries = 3;
    public int $backoff = 300;
    public int $timeout = 600;

    public function handle(): void
    {
        $this->processExpiringTrials();
        $this->processExpiredTrials();
    }

    /**
     * Send reminders for trials expiring in 3 days
     */
    private function processExpiringTrials(): void
    {
        $expiringTrials = Subscription::query()
            ->where('status', 'trialing')
            ->whereBetween('trial_ends_at', [
                now(),
                now()->addDays(3),
            ])
            ->whereNull('trial_reminder_sent_at')
            ->with('user', 'plan')
            ->cursor();

        foreach ($expiringTrials as $subscription) {
            try {
                Mail::to($subscription->user->email)->queue(
                    new \App\Mail\Subscription\TrialEndingMail(
                        $subscription,
                        $subscription->user,
                        $subscription->trial_ends_at
                    )
                );

                $subscription->update(['trial_reminder_sent_at' => now()]);

                Log::info('Trial expiring reminder sent', [
                    'subscription_id' => $subscription->id,
                    'user_id' => $subscription->user_id,
                    'trial_ends_at' => $subscription->trial_ends_at,
                ]);
            } catch (\Throwable $e) {
                Log::error('Failed to send trial expiring reminder', [
                    'subscription_id' => $subscription->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }

    /**
     * Process expired trials
     */
    private function processExpiredTrials(): void
    {
        $expiredTrials = Subscription::query()
            ->where('status', 'trialing')
            ->where('trial_ends_at', '<', now())
            ->with('user', 'plan')
            ->cursor();

        foreach ($expiredTrials as $subscription) {
            try {
                // Check if user has valid payment method
                $hasPaymentMethod = $this->userHasPaymentMethod($subscription->user);

                if ($hasPaymentMethod) {
                    // Trial converts to active paid subscription
                    $subscription->update([
                        'status' => 'active',
                        'trial_ends_at' => null,
                    ]);

                    Log::info('Trial converted to active subscription', [
                        'subscription_id' => $subscription->id,
                        'user_id' => $subscription->user_id,
                    ]);
                } else {
                    // No payment method - expire the trial
                    $subscription->update([
                        'status' => 'expired',
                        'ends_at' => now(),
                    ]);

                    // Send trial expired email
                    Mail::to($subscription->user->email)->queue(
                        new TrialExpiredMail($subscription, $subscription->user)
                    );

                    Log::info('Trial expired - no payment method', [
                        'subscription_id' => $subscription->id,
                        'user_id' => $subscription->user_id,
                    ]);
                }
            } catch (\Throwable $e) {
                Log::error('Failed to process expired trial', [
                    'subscription_id' => $subscription->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }

    /**
     * Check if user has a valid payment method
     */
    private function userHasPaymentMethod($user): bool
    {
        if (!$user->stripe_customer_id) {
            return false;
        }

        try {
            $stripe = new \Stripe\StripeClient(config('services.stripe.secret'));
            $paymentMethods = $stripe->paymentMethods->all([
                'customer' => $user->stripe_customer_id,
                'type' => 'card',
            ]);

            return count($paymentMethods->data) > 0;
        } catch (\Throwable $e) {
            return false;
        }
    }
}
