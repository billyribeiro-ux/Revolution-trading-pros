<?php

namespace Tests\Feature;

use App\Enums\SubscriptionInterval;
use App\Enums\SubscriptionStatus;
use App\Models\SubscriptionPlan;
use App\Models\User;
use App\Models\UserSubscription;
use App\Services\SubscriptionService;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class SubscriptionServiceTest extends TestCase
{
    use RefreshDatabase;

    protected SubscriptionService $service;

    protected function setUp(): void
    {
        parent::setUp();
        $this->service = new SubscriptionService();
    }

    public function test_it_can_create_subscription()
    {
        $user = User::factory()->create();
        $plan = SubscriptionPlan::factory()->create([
            'billing_period' => SubscriptionInterval::Monthly,
            'billing_interval' => 1,
            'trial_days' => 0,
        ]);

        $subscription = $this->service->create($user, $plan);

        $this->assertDatabaseHas('user_subscriptions', [
            'user_id' => $user->id,
            'subscription_plan_id' => $plan->id,
            'status' => SubscriptionStatus::Active,
        ]);
    }

    public function test_it_can_create_subscription_with_trial()
    {
        $user = User::factory()->create();
        $plan = SubscriptionPlan::factory()->create([
            'trial_days' => 14,
        ]);

        $subscription = $this->service->create($user, $plan);

        $this->assertEquals(SubscriptionStatus::Trial, $subscription->status);
        $this->assertNotNull($subscription->trial_ends_at);
    }

    public function test_it_can_cancel_subscription()
    {
        $subscription = UserSubscription::factory()->create([
            'status' => SubscriptionStatus::Active,
        ]);

        $this->service->cancel($subscription, immediate: true, reason: 'Test reason');

        $this->assertEquals(SubscriptionStatus::Cancelled, $subscription->fresh()->status);
        $this->assertStringContainsString('Test reason', $subscription->fresh()->notes);
    }

    public function test_it_can_pause_subscription()
    {
        $subscription = UserSubscription::factory()->create([
            'status' => SubscriptionStatus::Active,
        ]);

        $this->service->pause($subscription, reason: 'Taking a break');

        $this->assertEquals(SubscriptionStatus::OnHold, $subscription->fresh()->status);
    }

    public function test_it_can_resume_subscription()
    {
        $subscription = UserSubscription::factory()->create([
            'status' => SubscriptionStatus::OnHold,
        ]);

        $this->service->resume($subscription);

        $this->assertEquals(SubscriptionStatus::Active, $subscription->fresh()->status);
    }
}
