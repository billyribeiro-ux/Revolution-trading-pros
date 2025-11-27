<?php

namespace App\Notifications;

use App\Models\MembershipPlan;
use App\Models\UserMembership;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;
use Illuminate\Support\Facades\Config;

/**
 * Win-back notification for churned members with special offers.
 *
 * @version 1.0.0
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
