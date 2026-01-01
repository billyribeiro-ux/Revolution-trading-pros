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
 * CouponFraudAlert Event - Real-Time Fraud Detection Broadcasting
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * Broadcasts when suspicious coupon usage is detected, enabling:
 * - Real-time fraud monitoring for admins
 * - Automated fraud response triggers
 * - Security audit logging
 * 
 * @version 1.0.0
 */
class CouponFraudAlert implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly Coupon $coupon,
        public readonly array $indicators,
        public readonly ?int $userId = null,
        public readonly ?string $ipAddress = null
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('coupons.admin'),
        ];
    }

    public function broadcastAs(): string
    {
        return 'coupon.fraud_alert';
    }

    public function broadcastWith(): array
    {
        return [
            'type' => 'fraud_alert',
            'data' => [
                'coupon_id' => $this->coupon->id,
                'coupon_code' => $this->coupon->code,
                'fraud_score' => $this->coupon->fraud_score,
                'fraud_attempts' => $this->coupon->fraud_attempts,
                'indicators' => $this->indicators,
                'user_id' => $this->userId,
                'ip_address' => $this->ipAddress,
                'severity' => $this->calculateSeverity(),
                'recommended_action' => $this->getRecommendedAction(),
            ],
            'timestamp' => now()->toIso8601String(),
        ];
    }

    private function calculateSeverity(): string
    {
        $score = $this->coupon->fraud_score;
        return match(true) {
            $score >= 80 => 'critical',
            $score >= 60 => 'high',
            $score >= 40 => 'medium',
            default => 'low',
        };
    }

    private function getRecommendedAction(): string
    {
        $score = $this->coupon->fraud_score;
        return match(true) {
            $score >= 80 => 'Immediate suspension recommended',
            $score >= 60 => 'Manual review required',
            $score >= 40 => 'Monitor closely',
            default => 'Log for analysis',
        };
    }
}
