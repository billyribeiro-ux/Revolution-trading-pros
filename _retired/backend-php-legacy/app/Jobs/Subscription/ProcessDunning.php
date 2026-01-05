<?php

declare(strict_types=1);

namespace App\Jobs\Subscription;

use App\Models\DunningAttempt;
use App\Models\Subscription;
use App\Mail\Subscription\DunningReminderMail;
use App\Mail\Subscription\AccountSuspendedMail;
use App\Mail\Subscription\FinalWarningMail;
use App\Services\Payments\StripeProvider;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Mail;

/**
 * Process Dunning Job (ICT9+ Enterprise Grade)
 *
 * Handles failed payment recovery:
 * - Sends escalating reminder emails
 * - Retries payments
 * - Suspends/cancels accounts after max attempts
 *
 * @version 1.0.0
 */
class ProcessDunning implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    public int $tries = 3;
    public int $backoff = 300;
    public int $timeout = 900;

    /**
     * Maximum dunning attempts before cancellation
     */
    private const MAX_ATTEMPTS = 4;

    /**
     * Days of grace period before final cancellation
     */
    private const GRACE_PERIOD_DAYS = 14;

    public function handle(StripeProvider $stripeProvider): void
    {
        $this->processPendingDunningAttempts($stripeProvider);
        $this->processGracePeriodExpirations();
        $this->sendUpcomingDunningReminders();
    }

    /**
     * Process dunning attempts that are due
     */
    private function processPendingDunningAttempts(StripeProvider $stripeProvider): void
    {
        $pendingAttempts = DunningAttempt::query()
            ->where('status', 'pending')
            ->where('scheduled_at', '<=', now())
            ->with(['user', 'subscription', 'invoice'])
            ->cursor();

        foreach ($pendingAttempts as $attempt) {
            try {
                $this->processAttempt($attempt, $stripeProvider);
            } catch (\Throwable $e) {
                Log::error('Dunning attempt processing failed', [
                    'attempt_id' => $attempt->id,
                    'error' => $e->getMessage(),
                ]);

                $attempt->update([
                    'status' => 'failed',
                    'error_message' => $e->getMessage(),
                ]);
            }
        }
    }

    /**
     * Process a single dunning attempt
     */
    private function processAttempt(DunningAttempt $attempt, StripeProvider $stripeProvider): void
    {
        $user = $attempt->user;
        $subscription = $attempt->subscription;
        $invoice = $attempt->invoice;

        // Try to collect payment
        $paymentSucceeded = false;

        if ($invoice->stripe_invoice_id) {
            try {
                $stripeInvoice = \Stripe\Invoice::retrieve($invoice->stripe_invoice_id);

                if ($stripeInvoice->status === 'open') {
                    $stripeInvoice->pay();
                    $paymentSucceeded = true;
                } elseif ($stripeInvoice->status === 'paid') {
                    $paymentSucceeded = true;
                }
            } catch (\Stripe\Exception\CardException $e) {
                // Payment failed - continue with dunning
                Log::warning('Dunning payment retry failed', [
                    'attempt_id' => $attempt->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        if ($paymentSucceeded) {
            $this->handlePaymentSuccess($attempt, $subscription);
        } else {
            $this->handlePaymentFailure($attempt, $subscription, $user);
        }
    }

    /**
     * Handle successful payment recovery
     */
    private function handlePaymentSuccess(DunningAttempt $attempt, Subscription $subscription): void
    {
        $attempt->update([
            'status' => 'succeeded',
            'processed_at' => now(),
        ]);

        // Clear any past_due status
        if ($subscription->status === 'past_due') {
            $subscription->update([
                'status' => 'active',
                'payment_attempts' => 0,
            ]);
        }

        // Clear remaining dunning attempts for this invoice
        DunningAttempt::where('invoice_id', $attempt->invoice_id)
            ->where('status', 'pending')
            ->update(['status' => 'cancelled']);

        Log::info('Dunning payment recovered', [
            'attempt_id' => $attempt->id,
            'subscription_id' => $subscription->id,
        ]);
    }

    /**
     * Handle continued payment failure
     */
    private function handlePaymentFailure(DunningAttempt $attempt, Subscription $subscription, $user): void
    {
        $attempt->update([
            'status' => 'failed',
            'processed_at' => now(),
        ]);

        $totalAttempts = DunningAttempt::where('subscription_id', $subscription->id)
            ->where('status', 'failed')
            ->count();

        if ($totalAttempts >= self::MAX_ATTEMPTS) {
            // Final attempt failed - suspend account
            $this->suspendSubscription($subscription, $user);
        } else {
            // Schedule next attempt
            $this->scheduleNextAttempt($attempt, $subscription, $user, $totalAttempts);
        }
    }

    /**
     * Suspend subscription after max attempts
     */
    private function suspendSubscription(Subscription $subscription, $user): void
    {
        $subscription->update([
            'status' => 'suspended',
            'suspended_at' => now(),
            'grace_period_ends_at' => now()->addDays(self::GRACE_PERIOD_DAYS),
        ]);

        Mail::to($user->email)->queue(
            new AccountSuspendedMail($subscription, $user, self::GRACE_PERIOD_DAYS)
        );

        Log::warning('Subscription suspended after max dunning attempts', [
            'subscription_id' => $subscription->id,
            'user_id' => $user->id,
        ]);
    }

    /**
     * Schedule next dunning attempt
     */
    private function scheduleNextAttempt(DunningAttempt $attempt, Subscription $subscription, $user, int $attemptNumber): void
    {
        $daysUntilNext = match ($attemptNumber) {
            1 => 3,
            2 => 5,
            3 => 7,
            default => 7,
        };

        DunningAttempt::create([
            'user_id' => $user->id,
            'subscription_id' => $subscription->id,
            'invoice_id' => $attempt->invoice_id,
            'attempt_number' => $attemptNumber + 1,
            'status' => 'pending',
            'scheduled_at' => now()->addDays($daysUntilNext),
        ]);

        // Send appropriate email
        if ($attemptNumber === self::MAX_ATTEMPTS - 1) {
            Mail::to($user->email)->queue(
                new FinalWarningMail($subscription, $user, $daysUntilNext)
            );
        } else {
            Mail::to($user->email)->queue(
                new DunningReminderMail($subscription, $user, $attemptNumber + 1)
            );
        }
    }

    /**
     * Process grace period expirations
     */
    private function processGracePeriodExpirations(): void
    {
        $expiredGracePeriods = Subscription::query()
            ->where('status', 'suspended')
            ->where('grace_period_ends_at', '<', now())
            ->with('user')
            ->cursor();

        foreach ($expiredGracePeriods as $subscription) {
            try {
                $subscription->update([
                    'status' => 'canceled',
                    'canceled_at' => now(),
                    'ends_at' => now(),
                    'cancellation_reason' => 'Payment failure - grace period expired',
                ]);

                // Cancel in Stripe if applicable
                if ($subscription->stripe_subscription_id) {
                    try {
                        $stripe = new \Stripe\StripeClient(config('services.stripe.secret'));
                        $stripe->subscriptions->cancel($subscription->stripe_subscription_id);
                    } catch (\Throwable $e) {
                        Log::error('Failed to cancel Stripe subscription', [
                            'subscription_id' => $subscription->id,
                            'error' => $e->getMessage(),
                        ]);
                    }
                }

                Log::info('Subscription canceled - grace period expired', [
                    'subscription_id' => $subscription->id,
                    'user_id' => $subscription->user_id,
                ]);
            } catch (\Throwable $e) {
                Log::error('Failed to process grace period expiration', [
                    'subscription_id' => $subscription->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }

    /**
     * Send reminders for upcoming dunning attempts
     */
    private function sendUpcomingDunningReminders(): void
    {
        // Send reminder 1 day before scheduled retry
        $upcomingAttempts = DunningAttempt::query()
            ->where('status', 'pending')
            ->whereBetween('scheduled_at', [
                now()->addHours(23),
                now()->addHours(25),
            ])
            ->whereNull('reminder_sent_at')
            ->with(['user', 'subscription'])
            ->cursor();

        foreach ($upcomingAttempts as $attempt) {
            try {
                Mail::to($attempt->user->email)->queue(
                    new DunningReminderMail(
                        $attempt->subscription,
                        $attempt->user,
                        $attempt->attempt_number,
                        true // isUpcoming
                    )
                );

                $attempt->update(['reminder_sent_at' => now()]);

                Log::info('Upcoming dunning reminder sent', [
                    'attempt_id' => $attempt->id,
                    'user_id' => $attempt->user_id,
                ]);
            } catch (\Throwable $e) {
                Log::error('Failed to send upcoming dunning reminder', [
                    'attempt_id' => $attempt->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }
}
