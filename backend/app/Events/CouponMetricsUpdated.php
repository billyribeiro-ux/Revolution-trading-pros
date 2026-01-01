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
 * CouponMetricsUpdated Event - Real-Time Coupon Analytics Broadcasting
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * Broadcasts coupon performance metrics updates for admin analytics dashboards.
 * 
 * @version 1.0.0
 */
class CouponMetricsUpdated implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('coupons.analytics'),
        ];
    }

    public function broadcastAs(): string
    {
        return 'coupon.metrics_updated';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'metrics_update',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'code' => $this->coupon->code,
                'current_uses' => $this->coupon->current_uses,
                'unique_customers' => $this->coupon->unique_customers,
                'total_revenue' => $this->coupon->total_revenue,
                'total_discount' => $this->coupon->total_discount,
                'average_order_value' => $this->coupon->average_order_value,
                'conversion_rate' => $this->coupon->conversion_rate,
                'roi' => $this->coupon->roi,
                'effectiveness_score' => $this->coupon->effectiveness_score,
                'performance_rating' => $this->coupon->performance_rating,
                'usage_percentage' => $this->coupon->usage_percentage,
                'remaining_uses' => $this->coupon->remaining_uses,
                'days_until_expiry' => $this->coupon->days_until_expiry,
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }
}
