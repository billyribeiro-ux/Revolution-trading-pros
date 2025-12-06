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
 * Account Suspended Mail
 */
class AccountSuspendedMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public User $user,
        public int $gracePeriodDays,
    ) {}

    public function envelope(): Envelope
    {
        return new Envelope(
            subject: 'Your Account Has Been Suspended',
        );
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.account-suspended',
            with: [
                'user' => $this->user,
                'subscription' => $this->subscription,
                'gracePeriodDays' => $this->gracePeriodDays,
                'reactivateUrl' => route('billing.reactivate'),
                'updatePaymentUrl' => route('billing.payment-methods'),
                'planName' => $this->subscription->plan?->name ?? 'Subscription',
            ],
        );
    }
}
