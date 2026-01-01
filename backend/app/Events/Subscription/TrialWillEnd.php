<?php

declare(strict_types=1);

namespace App\Events\Subscription;

use App\Models\Subscription;
use Carbon\Carbon;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * Trial Will End Event
 */
class TrialWillEnd
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public Subscription $subscription,
        public ?Carbon $trialEndsAt,
    ) {}
}
