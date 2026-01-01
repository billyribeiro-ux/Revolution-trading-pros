<?php

declare(strict_types=1);

namespace Tests\Feature\WebSocket;

use App\Events\CouponCreated;
use App\Events\CouponUpdated;
use App\Events\CouponDeleted;
use App\Events\CouponRedeemed;
use App\Events\CouponMetricsUpdated;
use App\Events\CouponFraudAlert;
use App\Models\Coupon;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\Event;
use Tests\TestCase;

/**
 * ═══════════════════════════════════════════════════════════════════════════
 * CouponWebSocketTest - WebSocket Event Broadcasting Tests
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * COVERAGE:
 * - Event dispatching on CRUD operations
 * - Broadcast channel authorization
 * - Event payload structure validation
 * - Real-time update propagation
 * 
 * @version 1.0.0
 */
class CouponWebSocketTest extends TestCase
{
    use RefreshDatabase;

    private User $adminUser;
    private User $regularUser;

    protected function setUp(): void
    {
        parent::setUp();
        
        // Create test users
        $this->adminUser = User::factory()->create();
        $this->adminUser->assignRole('admin');
        
        $this->regularUser = User::factory()->create();
    }

    /**
     * Test CouponCreated event is dispatched when creating a coupon.
     */
    public function test_coupon_created_event_is_dispatched(): void
    {
        Event::fake([CouponCreated::class]);

        $response = $this->actingAs($this->adminUser)
            ->postJson('/api/coupons', [
                'code' => 'TESTCOUPON',
                'type' => 'percentage',
                'value' => 10,
                'is_active' => true,
            ]);

        $response->assertStatus(201);
        
        Event::assertDispatched(CouponCreated::class, function ($event) {
            return $event->coupon->code === 'TESTCOUPON';
        });
    }

    /**
     * Test CouponCreated event contains correct payload structure.
     */
    public function test_coupon_created_event_has_correct_payload(): void
    {
        $coupon = Coupon::factory()->create([
            'code' => 'PAYLOADTEST',
            'type' => 'percentage',
            'value' => 15,
            'is_public' => true,
        ]);

        $event = new CouponCreated($coupon);
        $payload = $event->broadcastWith();

        $this->assertEquals('coupon_created', $payload['type']);
        $this->assertArrayHasKey('data', $payload);
        $this->assertArrayHasKey('timestamp', $payload);
        $this->assertEquals('PAYLOADTEST', $payload['data']['code']);
        $this->assertEquals(15, $payload['data']['value']);
    }

    /**
     * Test CouponUpdated event is dispatched when updating a coupon.
     */
    public function test_coupon_updated_event_is_dispatched(): void
    {
        Event::fake([CouponUpdated::class]);

        $coupon = Coupon::factory()->create(['code' => 'UPDATETEST']);

        $response = $this->actingAs($this->adminUser)
            ->putJson("/api/coupons/{$coupon->id}", [
                'value' => 20,
            ]);

        $response->assertStatus(200);
        
        Event::assertDispatched(CouponUpdated::class, function ($event) {
            return $event->coupon->code === 'UPDATETEST';
        });
    }

    /**
     * Test CouponDeleted event is dispatched when deleting a coupon.
     */
    public function test_coupon_deleted_event_is_dispatched(): void
    {
        Event::fake([CouponDeleted::class]);

        $coupon = Coupon::factory()->create(['code' => 'DELETETEST']);
        $couponId = $coupon->id;

        $response = $this->actingAs($this->adminUser)
            ->deleteJson("/api/coupons/{$couponId}");

        $response->assertStatus(200);
        
        Event::assertDispatched(CouponDeleted::class, function ($event) use ($couponId) {
            return $event->couponId === $couponId && $event->couponCode === 'DELETETEST';
        });
    }

    /**
     * Test CouponRedeemed event contains user-specific channel.
     */
    public function test_coupon_redeemed_event_broadcasts_to_user_channel(): void
    {
        $coupon = Coupon::factory()->create();
        
        $event = new CouponRedeemed($coupon, $this->regularUser->id, 25.00, 123);
        $channels = $event->broadcastOn();

        $channelNames = array_map(fn($ch) => $ch->name, $channels);
        
        $this->assertContains('private-coupons.admin', $channelNames);
        $this->assertContains('private-coupons.analytics', $channelNames);
        $this->assertContains('private-coupons.user.' . $this->regularUser->id, $channelNames);
    }

    /**
     * Test CouponFraudAlert event calculates severity correctly.
     */
    public function test_coupon_fraud_alert_calculates_severity(): void
    {
        $coupon = Coupon::factory()->create(['fraud_score' => 85]);
        
        $event = new CouponFraudAlert($coupon, ['high_velocity', 'suspicious_ip'], 1, '192.168.1.1');
        $payload = $event->broadcastWith();

        $this->assertEquals('critical', $payload['data']['severity']);
        $this->assertEquals('Immediate suspension recommended', $payload['data']['recommended_action']);
    }

    /**
     * Test CouponMetricsUpdated event contains analytics data.
     */
    public function test_coupon_metrics_updated_event_contains_analytics(): void
    {
        $coupon = Coupon::factory()->create([
            'current_uses' => 50,
            'total_revenue' => 5000.00,
            'conversion_rate' => 0.15,
        ]);

        $event = new CouponMetricsUpdated($coupon);
        $payload = $event->broadcastWith();

        $this->assertEquals('metrics_update', $payload['type']);
        $this->assertEquals(50, $payload['data']['current_uses']);
        $this->assertEquals(5000.00, $payload['data']['total_revenue']);
    }

    /**
     * Test public coupon broadcasts to public channel.
     */
    public function test_public_coupon_broadcasts_to_public_channel(): void
    {
        $coupon = Coupon::factory()->create(['is_public' => true]);
        
        $event = new CouponCreated($coupon);
        $channels = $event->broadcastOn();

        $channelNames = array_map(fn($ch) => $ch->name ?? $ch, $channels);
        
        $this->assertTrue(
            in_array('coupons.public', $channelNames) || 
            in_array('public-coupons.public', $channelNames)
        );
    }

    /**
     * Test private coupon only broadcasts to admin channel.
     */
    public function test_private_coupon_only_broadcasts_to_admin_channel(): void
    {
        $coupon = Coupon::factory()->create([
            'is_public' => false,
            'auto_apply' => false,
        ]);
        
        $event = new CouponCreated($coupon);
        $channels = $event->broadcastOn();

        $this->assertCount(1, $channels);
        $this->assertEquals('private-coupons.admin', $channels[0]->name);
    }

    /**
     * Test broadcast event names follow convention.
     */
    public function test_broadcast_event_names_follow_convention(): void
    {
        $coupon = Coupon::factory()->create();
        
        $this->assertEquals('coupon.created', (new CouponCreated($coupon))->broadcastAs());
        $this->assertEquals('coupon.updated', (new CouponUpdated($coupon))->broadcastAs());
        $this->assertEquals('coupon.deleted', (new CouponDeleted(1, 'TEST'))->broadcastAs());
        $this->assertEquals('coupon.redeemed', (new CouponRedeemed($coupon, 1, 10.00))->broadcastAs());
        $this->assertEquals('coupon.metrics_updated', (new CouponMetricsUpdated($coupon))->broadcastAs());
        $this->assertEquals('coupon.fraud_alert', (new CouponFraudAlert($coupon, []))->broadcastAs());
    }
}
