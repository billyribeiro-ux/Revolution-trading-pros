<?php

namespace Tests\Feature;

use App\Models\Coupon;
use App\Models\Order;
use App\Models\Product;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Laravel\Sanctum\Sanctum;
use Tests\TestCase;

/**
 * Checkout Flow E2E Test
 *
 * Apple ICT 11 Principal Engineer Level Testing
 * Tests the complete checkout flow with 100% discount coupon
 */
class CheckoutFlowTest extends TestCase
{
    use RefreshDatabase;

    private User $user;
    private User $admin;
    private Product $product;
    private Coupon $coupon;

    protected function setUp(): void
    {
        parent::setUp();

        // Create admin user
        $this->admin = User::factory()->create([
            'email' => 'admin@test.com',
            'role' => 'admin',
            'is_admin' => true,
        ]);

        // Create regular user for checkout
        $this->user = User::factory()->create([
            'email' => 'user@test.com',
            'name' => 'Test User',
        ]);

        // Create test product (Options Scanner Pro)
        $this->product = Product::factory()->create([
            'name' => 'Options Scanner Pro',
            'slug' => 'options-scanner-pro',
            'type' => 'indicator',
            'price' => 99.00,
            'is_active' => true,
            'is_taxable' => true,
        ]);

        // Create 100% discount coupon
        $this->coupon = Coupon::factory()->create([
            'code' => 'FREETEST100',
            'type' => 'percentage',
            'value' => 100,
            'is_active' => true,
            'max_uses' => 10,
            'uses_count' => 0,
        ]);
    }

    /**
     * Test: Create a 100% discount coupon via admin API
     */
    public function test_admin_can_create_100_percent_coupon(): void
    {
        Sanctum::actingAs($this->admin);

        $response = $this->postJson('/api/admin/coupons', [
            'code' => 'NEWFREE100',
            'type' => 'percentage',
            'value' => 100,
            'is_active' => true,
            'max_uses' => 5,
            'description' => 'Test 100% discount coupon',
        ]);

        $response->assertStatus(201)
            ->assertJsonStructure([
                'id',
                'code',
                'type',
                'value',
                'is_active',
            ]);

        $this->assertDatabaseHas('coupons', [
            'code' => 'NEWFREE100',
            'type' => 'percentage',
            'value' => 100,
        ]);
    }

    /**
     * Test: Validate coupon endpoint returns valid response
     */
    public function test_coupon_validation_returns_valid(): void
    {
        Sanctum::actingAs($this->user);

        $response = $this->postJson('/api/coupon/validate', [
            'code' => 'FREETEST100',
        ]);

        $response->assertStatus(200)
            ->assertJson([
                'valid' => true,
            ]);
    }

    /**
     * Test: Checkout with 100% discount coupon results in $0 total
     */
    public function test_checkout_with_100_percent_coupon_is_free(): void
    {
        Sanctum::actingAs($this->user);

        $response = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'FREETEST100',
            'billing_country' => 'US',
            'billing_state' => 'CA',
        ]);

        $response->assertStatus(200)
            ->assertJsonStructure([
                'order_id',
                'order_number',
                'total',
            ]);

        // Verify total is $0 (or very close due to rounding)
        $total = $response->json('total');
        $this->assertLessThanOrEqual(0.01, abs($total), 'Total should be $0 with 100% discount');

        // Verify order was created in database
        $orderId = $response->json('order_id');
        $this->assertDatabaseHas('orders', [
            'id' => $orderId,
            'user_id' => $this->user->id,
        ]);

        // Verify coupon usage was incremented
        $this->coupon->refresh();
        $this->assertEquals(1, $this->coupon->uses_count);
    }

    /**
     * Test: Checkout flow - complete end-to-end with $0 payment
     */
    public function test_complete_checkout_flow_with_free_order(): void
    {
        Sanctum::actingAs($this->user);

        // Step 1: Validate coupon
        $validateResponse = $this->postJson('/api/coupon/validate', [
            'code' => 'FREETEST100',
        ]);
        $validateResponse->assertStatus(200);

        // Step 2: Calculate tax (should be 0 since subtotal after discount is 0)
        $taxResponse = $this->postJson('/api/cart/calculate-tax', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'FREETEST100',
            'billing_country' => 'US',
            'billing_state' => 'CA',
        ]);
        $taxResponse->assertStatus(200);

        // Step 3: Checkout
        $checkoutResponse = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'FREETEST100',
            'billing_country' => 'US',
            'billing_state' => 'CA',
        ]);

        $checkoutResponse->assertStatus(200);

        $orderNumber = $checkoutResponse->json('order_number');
        $total = $checkoutResponse->json('total');

        // Verify the order
        $this->assertNotNull($orderNumber, 'Order number should be set');
        $this->assertLessThanOrEqual(0.01, abs($total), 'Total should be $0');

        // Step 4: Verify order exists and is properly configured
        $order = Order::where('order_number', $orderNumber)->first();
        $this->assertNotNull($order);
        $this->assertEquals($this->user->id, $order->user_id);
        $this->assertEquals('FREETEST100', $order->coupon_code);
    }

    /**
     * Test: Coupon usage limit is enforced atomically
     */
    public function test_coupon_usage_limit_enforced(): void
    {
        // Create a coupon with max 1 use
        $limitedCoupon = Coupon::factory()->create([
            'code' => 'ONCEONLY',
            'type' => 'percentage',
            'value' => 100,
            'is_active' => true,
            'max_uses' => 1,
            'uses_count' => 0,
        ]);

        Sanctum::actingAs($this->user);

        // First checkout should succeed
        $response1 = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'ONCEONLY',
            'billing_country' => 'US',
            'billing_state' => 'TX',
        ]);

        $response1->assertStatus(200);

        // Second checkout should fail (coupon exhausted)
        $response2 = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'ONCEONLY',
            'billing_country' => 'US',
            'billing_state' => 'TX',
        ]);

        $response2->assertStatus(400)
            ->assertJson([
                'error' => 'Coupon has reached its usage limit',
            ]);
    }

    /**
     * Test: Invalid coupon is rejected
     */
    public function test_invalid_coupon_is_rejected(): void
    {
        Sanctum::actingAs($this->user);

        $response = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'INVALIDCODE',
            'billing_country' => 'US',
            'billing_state' => 'CA',
        ]);

        $response->assertStatus(400)
            ->assertJson([
                'error' => 'Invalid or expired coupon code',
            ]);
    }

    /**
     * Test: Expired coupon is rejected
     */
    public function test_expired_coupon_is_rejected(): void
    {
        // Create an expired coupon
        $expiredCoupon = Coupon::factory()->create([
            'code' => 'EXPIRED100',
            'type' => 'percentage',
            'value' => 100,
            'is_active' => true,
            'expires_at' => now()->subDay(),
        ]);

        Sanctum::actingAs($this->user);

        $response = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
            'coupon_code' => 'EXPIRED100',
            'billing_country' => 'US',
            'billing_state' => 'CA',
        ]);

        $response->assertStatus(400)
            ->assertJson([
                'error' => 'Invalid or expired coupon code',
            ]);
    }

    /**
     * Test: Checkout requires authentication
     */
    public function test_checkout_requires_authentication(): void
    {
        $response = $this->postJson('/api/cart/checkout', [
            'items' => [
                [
                    'product_id' => $this->product->id,
                    'quantity' => 1,
                    'price' => 99.00,
                ],
            ],
        ]);

        $response->assertStatus(401);
    }
}
