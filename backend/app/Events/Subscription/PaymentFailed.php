<?php

declare(strict_types=1);

namespace App\Events\Subscription;

use App\Models\User;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Payment Failed Event
 */
class PaymentFailed
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public User $user,
        public int $amount,
        public ?string $failureMessage = null,
    ) {}
}
