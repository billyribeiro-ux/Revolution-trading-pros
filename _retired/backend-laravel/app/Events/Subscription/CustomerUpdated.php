<?php

declare(strict_types=1);

namespace App\Events\Subscription;

use App\Models\User;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Customer Updated Event
 */
class CustomerUpdated
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public User $user,
        public object $stripeCustomer,
    ) {}
}
