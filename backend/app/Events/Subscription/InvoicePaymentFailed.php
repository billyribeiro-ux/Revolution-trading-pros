<?php

declare(strict_types=1);

namespace App\Events\Subscription;

use App\Models\Invoice;
use App\Models\User;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Invoice Payment Failed Event
 */
class InvoicePaymentFailed
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public Invoice $invoice,
        public User $user,
        public int $attemptCount,
    ) {}
}
