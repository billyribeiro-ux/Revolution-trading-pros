<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Coupon;
use Illuminate\Broadcasting\Channel;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * ═══════════════════════════════════════════════════════════════════════════
 * CouponUpdated Event - Real-Time Coupon Update Broadcasting
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * @version 1.0.0
 */
class CouponUpdated implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon,
        public readonly array $changedAttributes = []
    ) {}

    public function broadcastOn(): array
    {
        $channels = [
            new PrivateChannel('coupons.admin'),
        ];

        if ($this->coupon->is_public || $this->coupon->auto_apply) {
            $channels[] = new Channel('coupons.public');
        }

        return $channels;
    }

    public function broadcastAs(): string
    {
        return 'coupon.updated';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_updated',
            'data' => [
                'id' => $this->coupon->id,
                'uuid' => $this->coupon->uuid,
                'code' => $this->coupon->code,
                'type' => $this->coupon->type,
                'value' => $this->coupon->value,
                'display_name' => $this->coupon->display_name,
                'discount_display' => $this->coupon->discount_display,
                'is_active' => $this->coupon->is_active,
                'is_public' => $this->coupon->is_public,
                'auto_apply' => $this->coupon->auto_apply,
                'start_date' => $this->coupon->start_date?->toIso8601String(),
                'expiry_date' => $this->coupon->expiry_date?->toIso8601String(),
                'max_uses' => $this->coupon->max_uses,
                'current_uses' => $this->coupon->current_uses,
                'remaining_uses' => $this->coupon->remaining_uses,
                'usage_percentage' => $this->coupon->usage_percentage,
                'updated_at' => $this->coupon->updated_at->toIso8601String(),
            ],
            'changed_attributes' => $this->changedAttributes,
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
