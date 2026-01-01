<?php

declare(strict_types=1);

namespace App\Listeners\Subscription;

use App\Events\Subscription\InvoicePaymentFailed;
use App\Mail\Subscription\PaymentFailedMail;
use App\Models\DunningAttempt;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Log;

/**
 * Handle Payment Failure - Initiate Dunning Process
 */
class HandlePaymentFailure implements ShouldQueue
{
    use InteractsWithQueue;

    public int $tries = 3;
    public int $backoff = 60;

    public function handle(InvoicePaymentFailed $event): void
    {
        $invoice = $event->invoice;
        $user = $event->user;
        $attemptCount = $event->attemptCount;

        try {
            // Create dunning attempt record
            DunningAttempt::create([
                'user_id' => $user->id,
                'invoice_id' => $invoice->id,
                'subscription_id' => $invoice->subscription_id,
                'attempt_number' => $attemptCount,
                'status' => 'pending',
                'scheduled_at' => $this->getNextRetryDate($attemptCount),
            ]);

            // Send appropriate email based on attempt count
            $this->sendDunningEmail($user, $invoice, $attemptCount);

            Log::info('Dunning process initiated', [
                'user_id' => $user->id,
                'invoice_id' => $invoice->id,
                'attempt_count' => $attemptCount,
            ]);
        } catch (\Throwable $e) {
            Log::error('Failed to initiate dunning process', [
                'user_id' => $user->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    private function getNextRetryDate(int $attemptCount): \Carbon\Carbon
    {
        // Exponential backoff for retries
        $daysUntilRetry = match ($attemptCount) {
            1 => 3,  // First retry after 3 days
            2 => 5,  // Second retry after 5 days
            3 => 7,  // Third retry after 7 days
            default => 14, // Further retries every 2 weeks
        };

        return now()->addDays($daysUntilRetry);
    }

    private function sendDunningEmail($user, $invoice, int $attemptCount): void
    {
        Mail::to($user->email)->queue(
            new PaymentFailedMail($user, $invoice, $attemptCount)
        );
    }
}
