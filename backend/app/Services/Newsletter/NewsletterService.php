<?php

declare(strict_types=1);

namespace App\Services\Newsletter;

use App\Models\NewsletterSubscription;

/**
 * Newsletter Service
 *
 * Handles newsletter-related business logic.
 */
class NewsletterService
{
    /**
     * Trigger welcome email sequence.
     */
    public function triggerWelcomeSequence(NewsletterSubscription $subscription): void
    {
        // Welcome sequence implementation
    }

    /**
     * Send newsletter to subscribers.
     */
    public function sendNewsletter(int $campaignId): array
    {
        return ['sent' => 0, 'failed' => 0];
    }

    /**
     * Get subscriber count.
     */
    public function getSubscriberCount(): int
    {
        return NewsletterSubscription::where('status', 'active')->count();
    }
}
