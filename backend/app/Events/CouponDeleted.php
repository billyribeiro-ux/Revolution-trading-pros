<?php

declare(strict_types=1);

namespace App\Events;

use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * ═══════════════════════════════════════════════════════════════════════════
 * CouponDeleted Event - Real-Time Coupon Deletion Broadcasting
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * @version 1.0.0
 */
class CouponDeleted implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly int $couponId,
        public readonly string $couponCode
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('coupons.admin'),
        ];
    }

    public function broadcastAs(): string
    {
        return 'coupon.deleted';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_deleted',
            'data' => [
                'id' => $this->couponId,
                'code' => $this->couponCode,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
