<?php

declare(strict_types=1);

namespace App\Mail\Subscription;

use App\Models\User;
use App\Models\Subscription;
use Illuminate\Bus\Queueable;
use Illuminate\Mail\Mailable;
use Illuminate\Mail\Mailables\Content;
use Illuminate\Mail\Mailables\Envelope;
use Illuminate\Queue\SerializesModels;

/**
 * Dunning Reminder Mail
 */
class DunningReminderMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public User $user,
        public int $attemptNumber,
        public bool $isUpcoming = false,
    ) {}

    public function envelope(): Envelope
    {
        $subject = $this->isUpcoming
            ? 'Payment Retry Scheduled Tomorrow'
            : "Payment Retry Attempt #{$this->attemptNumber}";

        return new Envelope(subject: $subject);
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.dunning-reminder',
            with: [
                'user' => $this->user,
                'subscription' => $this->subscription,
                'attemptNumber' => $this->attemptNumber,
                'isUpcoming' => $this->isUpcoming,
                'updatePaymentUrl' => route('billing.payment-methods'),
                'planName' => $this->subscription->plan?->name ?? 'Subscription',
            ],
        );
    }
}
