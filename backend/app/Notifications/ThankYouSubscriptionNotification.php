<?php

namespace App\Notifications;

use App\Models\EmailTemplate;
use App\Models\MembershipPlan;
use App\Models\UserSubscription;
use App\Services\Email\EmailTemplateRenderService;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;
use Illuminate\Support\Facades\Config;

/**
 * Thank you notification for new subscription purchases.
 * Supports admin-customizable templates with Blade fallback.
 *
 * @version 2.0.0
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
        // Try to use admin-customizable database template
        $template = EmailTemplate::where('slug', 'thank-you-subscription')
            ->where('status', 'published')
            ->first();

        if ($template) {
            try {
                $renderService = app(EmailTemplateRenderService::class);
                $rendered = $renderService->render($template, [
                    'user_name' => $notifiable->name,
                    'plan_name' => $this->plan->name,
                    'plan_price' => number_format($this->plan->price, 2),
                    'billing_cycle' => $this->plan->billing_cycle ?? 'monthly',
                    'start_date' => $this->subscription->starts_at?->format('F d, Y'),
                    'next_billing_date' => $this->subscription->ends_at?->format('F d, Y'),
                    'dashboard_url' => Config::get('app.frontend_url', Config::get('app.url')) . '/dashboard',
                    'app_url' => Config::get('app.url'),
                ]);

                return (new MailMessage)
                    ->subject($rendered['subject'])
                    ->html($rendered['html']);
            } catch (\Exception $e) {
                report($e);
            }
        }

        // Fallback to Blade template
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
