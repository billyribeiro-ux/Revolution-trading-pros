<?php

declare(strict_types=1);

namespace App\Mail\Subscription;

use App\Models\User;
use App\Models\Invoice;
use Illuminate\Bus\Queueable;
use Illuminate\Mail\Mailable;
use Illuminate\Mail\Mailables\Content;
use Illuminate\Mail\Mailables\Envelope;
use Illuminate\Queue\SerializesModels;

/**
 * Payment Failed Mail
 */
class PaymentFailedMail extends Mailable
{
    use Queueable, SerializesModels;

    public function __construct(
        public User $user,
        public Invoice $invoice,
        public int $attemptCount,
    ) {}

    public function envelope(): Envelope
    {
        $subject = match ($this->attemptCount) {
            1 => 'Action Required: Payment Failed',
            2 => 'Reminder: Please Update Your Payment Method',
            3 => 'Urgent: Your Account is at Risk',
            default => 'Important: Payment Issue with Your Account',
        };

        return new Envelope(subject: $subject);
    }

    public function content(): Content
    {
        return new Content(
            markdown: 'emails.subscription.payment-failed',
            with: [
                'user' => $this->user,
                'invoice' => $this->invoice,
                'attemptCount' => $this->attemptCount,
                'updatePaymentUrl' => route('billing.payment-methods'),
                'amount' => number_format($this->invoice->amount / 100, 2),
                'currency' => strtoupper($this->invoice->currency ?? 'USD'),
            ],
        );
    }
}
