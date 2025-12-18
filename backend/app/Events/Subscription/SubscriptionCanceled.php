<?php

declare(strict_types=1);

namespace App\Events\Subscription;

use App\Models\Subscription;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Subscription Canceled Event
 */
class SubscriptionCanceled
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public Subscription $subscription,
    ) {}
}
