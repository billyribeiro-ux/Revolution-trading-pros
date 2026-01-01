<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Coupon;
use App\Models\CouponUsage;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * CouponUsed Event - Dispatched when a coupon is used/redeemed
 * 
 * @version 1.0.0
 */
class CouponUsed implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon,
        public readonly CouponUsage $usage
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('coupons.admin'),
            new PrivateChannel('coupons.analytics'),
        ];
    }

    public function broadcastAs(): string
    {
        return 'coupon.used';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_used',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'code' => $this->coupon->code,
                'usage_id' => $this->usage->id,
                'customer_id' => $this->usage->customer_id,
                'order_id' => $this->usage->order_id,
                'discount_amount' => $this->usage->discount_amount,
                'current_uses' => $this->coupon->current_uses,
                'remaining_uses' => $this->coupon->remaining_uses,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
