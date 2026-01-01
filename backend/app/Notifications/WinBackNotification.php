<?php

namespace App\Notifications;

use App\Models\EmailTemplate;
use App\Models\MembershipPlan;
use App\Models\UserMembership;
use App\Services\Email\EmailTemplateRenderService;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;
use Illuminate\Support\Facades\Config;

/**
 * Win-back notification for churned members with special offers.
 * Supports admin-customizable templates with Blade fallback.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
class WinBackNotification extends Notification implements ShouldQueue
{
    use Queueable;

    public int $tries = 3;
    public int $backoff = 60;

    public function __construct(
        protected UserMembership $membership,
        protected MembershipPlan $previousPlan,
        protected ?object $offer = null,
        protected ?object $testimonial = null,
        protected ?int $communitySize = null
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
        $template = EmailTemplate::where('slug', 'win-back')
            ->where('status', 'published')
            ->first();

        if ($template) {
            try {
                $renderService = app(EmailTemplateRenderService::class);
                $rendered = $renderService->render($template, [
                    'user_name' => $notifiable->name,
                    'previous_plan' => $this->previousPlan->name,
                    'expired_date' => $this->membership->expires_at?->format('F d, Y'),
                    'discount_percent' => $this->offer?->discount_percent,
                    'offer_code' => $this->offer?->code,
                    'expires_in_days' => $this->offer?->expires_in_days,
                    'community_size' => $this->communitySize,
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
            ->subject("We Miss You, {$notifiable->name}! Special Offer Inside")
            ->view('emails.membership.win-back', [
                'user' => $notifiable,
                'membership' => $this->membership,
                'previousPlan' => $this->previousPlan,
                'offer' => $this->offer,
                'testimonial' => $this->testimonial,
                'communitySize' => $this->communitySize,
            ]);
    }

    /**
     * Get the array representation of the notification.
     */
    public function toArray(object $notifiable): array
    {
        return [
            'type' => 'win_back_campaign',
            'membership_id' => $this->membership->id,
            'previous_plan' => $this->previousPlan->name,
            'offer_code' => $this->offer?->code,
            'offer_discount' => $this->offer?->discount_percent,
            'message' => "We miss you! Come back with {$this->offer?->discount_percent}% off.",
        ];
    }
}
