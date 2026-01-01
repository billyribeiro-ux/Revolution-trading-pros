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
 * Trial Expired Mail
 */
class TrialExpiredMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public User $user,
    ) {}

    public function envelope(): Envelope
    {
        return new Envelope(
            subject: 'Your Trial Has Expired - Upgrade to Continue',
        );
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.trial-expired',
            with: [
                'user' => $this->user,
                'subscription' => $this->subscription,
                'upgradeUrl' => route('billing.plans'),
                'planName' => $this->subscription->plan?->name ?? 'Subscription',
            ],
        );
    }
}
