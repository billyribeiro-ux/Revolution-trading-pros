<?php

declare(strict_types=1);

namespace App\Jobs\Subscription;

use App\Models\Subscription;
use App\Services\Payments\StripeProvider;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;

/**
 * Process Subscription Renewals Job (ICT9+ Enterprise Grade)
 *
 * Handles subscription renewals that need manual processing:
 * - Renewals with payment issues
 * - Grace period expirations
 * - Status transitions
 *
 * @version 1.0.0
 */
class ProcessSubscriptionRenewals implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    public int $tries = 3;
    public int $backoff = 300;
    public int $timeout = 600;

    public function __construct(
        private ?int $subscriptionId = null
    ) {}

    public function handle(StripeProvider $stripeProvider): void
    {
        $query = Subscription::query()
            ->where('status', 'active')
            ->where('current_period_end', '<=', now())
            ->where('cancel_at_period_end', false);

        if ($this->subscriptionId) {
            $query->where('id', $this->subscriptionId);
        }

        $subscriptions = $query->cursor();

        foreach ($subscriptions as $subscription) {
            try {
                $this->processRenewal($subscription, $stripeProvider);
            } catch (\Throwable $e) {
                Log::error('Subscription renewal failed', [
                    'subscription_id' => $subscription->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
    }

    private function processRenewal(Subscription $subscription, StripeProvider $stripeProvider): void
    {
        // For Stripe, renewals are automatic - we just sync the status
        if ($subscription->stripe_subscription_id) {
            $stripeSubscription = $stripeProvider->getSubscription($subscription->stripe_subscription_id);

            $subscription->update([
                'status' => $this->mapStripeStatus($stripeSubscription['status']),
                'current_period_start' => \Carbon\Carbon::createFromTimestamp($stripeSubscription['current_period_start']),
                'current_period_end' => \Carbon\Carbon::createFromTimestamp($stripeSubscription['current_period_end']),
            ]);

            Log::info('Subscription renewal synced', [
                'subscription_id' => $subscription->id,
                'status' => $subscription->status,
            ]);
        }
    }

    private function mapStripeStatus(string $stripeStatus): string
    {
        return match ($stripeStatus) {
            'trialing' => 'trialing',
            'active' => 'active',
            'past_due' => 'past_due',
            'canceled' => 'canceled',
            'unpaid' => 'unpaid',
            'incomplete' => 'incomplete',
            'incomplete_expired' => 'expired',
            'paused' => 'paused',
            default => 'unknown',
        };
    }
}
