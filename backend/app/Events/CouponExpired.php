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
 * CouponExpired Event - Dispatched when a coupon expires
 * 
 * @version 1.0.0
 */
class CouponExpired implements ShouldBroadcast
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
        return 'coupon.expired';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_expired',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'code' => $this->coupon->code,
                'expiry_date' => $this->coupon->expiry_date?->toIso8601String(),
                'total_uses' => $this->coupon->current_uses,
                'total_revenue' => $this->coupon->total_revenue,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
