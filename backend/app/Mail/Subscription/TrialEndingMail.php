<?php

declare(strict_types=1);

namespace App\Mail\Subscription;

use App\Models\User;
use App\Models\Subscription;
use Carbon\Carbon;
use Illuminate\Bus\Queueable;
use Illuminate\Mail\Mailable;
use Illuminate\Mail\Mailables\Content;
use Illuminate\Mail\Mailables\Envelope;
use Illuminate\Queue\SerializesModels;

/**
 * Trial Ending Mail
 */
class TrialEndingMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public User $user,
        public ?Carbon $trialEndsAt,
    ) {}

    public function envelope(): Envelope
    {
        $daysRemaining = $this->trialEndsAt?->diffInDays(now()) ?? 0;

        return new Envelope(
            subject: "Your Trial Ends in {$daysRemaining} Days",
        );
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.trial-ending',
            with: [
                'user' => $this->user,
                'subscription' => $this->subscription,
                'trialEndsAt' => $this->trialEndsAt,
                'daysRemaining' => $this->trialEndsAt?->diffInDays(now()) ?? 0,
                'addPaymentUrl' => route('billing.payment-methods'),
                'planName' => $this->subscription->plan?->name ?? 'Trial',
            ],
        );
    }
}
