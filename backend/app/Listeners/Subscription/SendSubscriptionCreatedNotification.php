<?php

declare(strict_types=1);

namespace App\Listeners\Subscription;

use App\Events\Subscription\SubscriptionCreated;
use App\Mail\Subscription\WelcomeSubscriptionMail;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Log;

/**
 * Send Subscription Created Notification
 */
class SendSubscriptionCreatedNotification implements ShouldQueue
{
    use InteractsWithQueue;

    public int $tries = 3;
    public int $backoff = 60;

    public function handle(SubscriptionCreated $event): void
    {
        $subscription = $event->subscription;
        $user = $event->user;

        try {
            Mail::to($user->email)->queue(new WelcomeSubscriptionMail($subscription, $user));

            Log::info('Subscription welcome email queued', [
                'user_id' => $user->id,
                'subscription_id' => $subscription->id,
            ]);
        } catch (\Throwable $e) {
            Log::error('Failed to send subscription welcome email', [
                'user_id' => $user->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }
}
