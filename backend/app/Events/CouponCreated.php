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
 * CouponCreated Event - Real-Time Coupon Creation Broadcasting
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * Broadcasts when a new coupon is created, enabling:
 * - Admin dashboard real-time updates
 * - Auto-apply coupon notifications to users
 * - Analytics tracking initialization
 * 
 * @version 1.0.0
 */
class CouponCreated implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon
    ) {}

    /**
     * Get the channels the event should broadcast on.
     *
     * @return array<int, \Illuminate\Broadcasting\Channel>
     */
    public function broadcastOn(): array
    {
        $channels = [
            new PrivateChannel('coupons.admin'),
        ];

        // If coupon is public/auto-apply, broadcast to public channel
        if ($this->coupon->is_public || $this->coupon->auto_apply) {
            $channels[] = new Channel('coupons.public');
        }

        return $channels;
    }

    /**
     * The event's broadcast name.
     */
    public function broadcastAs(): string
    {
        return 'coupon.created';
    }

    /**
     * Get the data to broadcast.
     *
     * @return array<string, mixed>
     */
    public function broadcastWith(): array
    {
        return [
            'type' => 'coupon_created',
            'data' => [
                'id' => $this->coupon->id,
                'uuid' => $this->coupon->uuid,
                'code' => $this->coupon->code,
                'type' => $this->coupon->type,
                'value' => $this->coupon->value,
                'display_name' => $this->coupon->display_name,
                'description' => $this->coupon->description,
                'discount_display' => $this->coupon->discount_display,
                'is_active' => $this->coupon->is_active,
                'is_public' => $this->coupon->is_public,
                'auto_apply' => $this->coupon->auto_apply,
                'start_date' => $this->coupon->start_date?->toIso8601String(),
                'expiry_date' => $this->coupon->expiry_date?->toIso8601String(),
                'min_purchase_amount' => $this->coupon->min_purchase_amount,
                'max_uses' => $this->coupon->max_uses,
                'current_uses' => $this->coupon->current_uses,
                'created_at' => $this->coupon->created_at->toIso8601String(),
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
