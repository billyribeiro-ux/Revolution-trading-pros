<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Coupon;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * CouponExhausted Event - Dispatched when a coupon reaches its usage limit
 * 
 * @version 1.0.0
 */
class CouponExhausted implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('coupons.admin'),
        ];
    }

    public function broadcastAs(): string
    {
        return 'coupon.exhausted';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_exhausted',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'code' => $this->coupon->code,
                'max_uses' => $this->coupon->max_uses,
                'current_uses' => $this->coupon->current_uses,
                'total_revenue' => $this->coupon->total_revenue,
                'unique_customers' => $this->coupon->unique_customers,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
