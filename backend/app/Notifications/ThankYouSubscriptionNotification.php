<?php

namespace App\Notifications;

use App\Models\MembershipPlan;
use App\Models\UserSubscription;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;

/**
 * Thank you notification for new subscription purchases.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class ThankYouSubscriptionNotification extends Notification implements ShouldQueue
{
    use Queueable;

    public int $tries = 3;
    public int $backoff = 60;

    public function __construct(
        protected UserSubscription $subscription,
        protected MembershipPlan $plan
    ) {}

    /**
     * Get the notification's delivery channels.
     */
    public function via(object $notifiable): array
    {
        return ['mail', 'database'];
    }

    /**
     * Get the mail representation of the notification.
     */
    public function toMail(object $notifiable): MailMessage
    {
        return (new MailMessage)
            ->subject("Welcome to {$this->plan->name} - Revolution Trading Pros")
            ->view('emails.purchases.thank-you-subscription', [
                'user' => $notifiable,
                'subscription' => $this->subscription,
                'plan' => $this->plan,
            ]);
    }

    /**
     * Get the array representation of the notification.
     */
    public function toArray(object $notifiable): array
    {
        return [
            'type' => 'subscription_purchase',
            'subscription_id' => $this->subscription->id,
            'plan_id' => $this->plan->id,
            'plan_name' => $this->plan->name,
            'amount' => $this->subscription->price,
            'message' => "Thank you for subscribing to {$this->plan->name}!",
        ];
    }
}
