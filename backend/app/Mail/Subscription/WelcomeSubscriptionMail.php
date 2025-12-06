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
 * Welcome Subscription Mail
 */
class WelcomeSubscriptionMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public User $user,
    ) {}

    public function envelope(): Envelope
    {
        $planName = $this->subscription->plan?->name ?? 'Subscription';
        return new Envelope(
            subject: "Welcome to {$planName}!",
        );
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.welcome',
            with: [
                'user' => $this->user,
                'subscription' => $this->subscription,
                'dashboardUrl' => route('dashboard'),
                'planName' => $this->subscription->plan?->name ?? 'Subscription',
                'features' => $this->subscription->plan?->features ?? [],
            ],
        );
    }
}
