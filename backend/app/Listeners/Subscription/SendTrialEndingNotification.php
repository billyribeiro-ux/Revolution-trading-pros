<?php

declare(strict_types=1);

namespace App\Listeners\Subscription;

use App\Events\Subscription\TrialWillEnd;
use App\Mail\Subscription\TrialEndingMail;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Log;

/**
 * Send Trial Ending Notification
 */
class SendTrialEndingNotification implements ShouldQueue
{
    use InteractsWithQueue;

    public int $tries = 3;
    public int $backoff = 60;

    public function handle(TrialWillEnd $event): void
    {
        $subscription = $event->subscription;
        $user = $subscription->user;

        try {
            Mail::to($user->email)->queue(
                new TrialEndingMail($subscription, $user, $event->trialEndsAt)
            );

            Log::info('Trial ending email queued', [
                'user_id' => $user->id,
                'subscription_id' => $subscription->id,
                'trial_ends_at' => $event->trialEndsAt,
            ]);
        } catch (\Throwable $e) {
            Log::error('Failed to send trial ending email', [
                'user_id' => $user->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }
}
