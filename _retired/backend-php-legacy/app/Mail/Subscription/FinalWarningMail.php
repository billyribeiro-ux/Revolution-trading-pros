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
 * Final Warning Mail - Last chance before suspension
 */
class FinalWarningMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public User $user,
        public int $daysUntilSuspension,
    ) {}

    public function envelope(): Envelope
    {
        return new Envelope(
            subject: '⚠️ Final Warning: Your Account Will Be Suspended',
        );
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.final-warning',
            with: [
                'user' => $this->user,
                'subscription' => $this->subscription,
                'daysUntilSuspension' => $this->daysUntilSuspension,
                'updatePaymentUrl' => route('billing.payment-methods'),
                'planName' => $this->subscription->plan?->name ?? 'Subscription',
            ],
        );
    }
}
