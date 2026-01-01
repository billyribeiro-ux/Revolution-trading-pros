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
 * ═══════════════════════════════════════════════════════════════════════════
 * CouponRedeemed Event - Real-Time Coupon Redemption Broadcasting
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * Broadcasts when a coupon is redeemed, enabling:
 * - Admin real-time usage monitoring
 * - User redemption confirmation
 * - Analytics updates
 * 
 * @version 1.0.0
 */
class CouponRedeemed implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon,
        public readonly int $userId,
        public readonly float $discountAmount,
        public readonly ?int $orderId = null
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('coupons.admin'),
            new PrivateChannel('coupons.analytics'),
            new PrivateChannel('coupons.user.' . $this->userId),
        ];
    }

    public function broadcastAs(): string
    {
        return 'coupon.redeemed';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'redemption',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'coupon_code' => $this->coupon->code,
                'user_id' => $this->userId,
                'discount_amount' => $this->discountAmount,
                'order_id' => $this->orderId,
                'current_uses' => $this->coupon->current_uses,
                'remaining_uses' => $this->coupon->remaining_uses,
                'usage_percentage' => $this->coupon->usage_percentage,
                'is_exhausted' => $this->coupon->is_exhausted,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
