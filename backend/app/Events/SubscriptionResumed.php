<?php

namespace App\Events;

use App\Models\UserSubscription;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

class SubscriptionResumed
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(public UserSubscription $subscription)
    {
    }
}
