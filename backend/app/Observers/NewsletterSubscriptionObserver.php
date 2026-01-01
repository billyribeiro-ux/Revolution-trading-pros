<?php

declare(strict_types=1);

namespace App\Observers;

use App\Models\NewsletterSubscription;
use App\Services\Integration\IntegrationHub;
use Illuminate\Support\Facades\Log;

/**
 * NewsletterSubscriptionObserver - Wires newsletter subscriptions to the ecosystem
 *
 * Triggers:
 * - CRM contact creation/update
 * - Tag application
 * - Welcome automation sequences
 * - Consent recording
 */
class NewsletterSubscriptionObserver
{
    public function __construct(
        private readonly IntegrationHub $integrationHub
    ) {}

    /**
     * Handle the NewsletterSubscription "created" event.
     */
    public function created(NewsletterSubscription $subscription): void
    {
        try {
            $this->integrationHub->processNewsletterSubscription($subscription, 'subscribed');

            Log::info('NewsletterSubscriptionObserver: Processed new subscription', [
                'email' => $subscription->email,
            ]);
        } catch (\Exception $e) {
            Log::error('NewsletterSubscriptionObserver: Failed to process subscription', [
                'email' => $subscription->email,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Handle the NewsletterSubscription "updated" event.
     */
    public function updated(NewsletterSubscription $subscription): void
    {
        try {
            // Handle status changes
            if ($subscription->isDirty('status')) {
                $newStatus = $subscription->status;

                if ($newStatus === 'unsubscribed') {
                    $this->integrationHub->processNewsletterSubscription($subscription, 'unsubscribed');
                } elseif ($newStatus === 'active' && $subscription->getOriginal('status') === 'pending') {
                    // Double opt-in confirmed
                    $this->integrationHub->processNewsletterSubscription($subscription, 'confirmed');
                }
            }
        } catch (\Exception $e) {
            Log::error('NewsletterSubscriptionObserver: Failed to process update', [
                'email' => $subscription->email,
                'error' => $e->getMessage(),
            ]);
        }
    }
}
