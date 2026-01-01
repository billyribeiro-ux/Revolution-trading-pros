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
 * CouponAbused Event - Dispatched when fraud is detected on a coupon
 * 
 * @version 1.0.0
 */
class CouponAbused implements ShouldBroadcast
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
        return 'coupon.abused';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_abused',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'code' => $this->coupon->code,
                'fraud_score' => $this->coupon->fraud_score,
                'fraud_attempts' => $this->coupon->fraud_attempts,
                'fraud_indicators' => $this->coupon->fraud_indicators,
                'status' => $this->coupon->status,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
