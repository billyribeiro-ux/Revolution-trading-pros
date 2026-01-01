<?php

declare(strict_types=1);

namespace Tests\Feature\WebSocket;

use App\Broadcasting\CouponChannel;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

/**
 * ═══════════════════════════════════════════════════════════════════════════
 * CouponChannelTest - WebSocket Channel Authorization Tests
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11 Principal Engineer Implementation
 * 
 * @version 1.0.0
 */
class CouponChannelTest extends TestCase
{
    use RefreshDatabase;

    private CouponChannel $channel;
    private User $adminUser;
    private User $superAdminUser;
    private User $regularUser;

    protected function setUp(): void
    {
        parent::setUp();
        
        $this->channel = new CouponChannel();
        
        $this->adminUser = User::factory()->create();
        $this->adminUser->assignRole('admin');
        
        $this->superAdminUser = User::factory()->create();
        $this->superAdminUser->assignRole('super-admin');
        
        $this->regularUser = User::factory()->create();
    }

    /**
     * Test any authenticated user can join public channel.
     */
    public function test_any_user_can_join_public_channel(): void
    {
        $result = $this->channel->joinPublic($this->regularUser);
        
        $this->assertIsArray($result);
        $this->assertEquals($this->regularUser->id, $result['id']);
        $this->assertEquals('subscriber', $result['role']);
    }

    /**
     * Test user can join their own user channel.
     */
    public function test_user_can_join_own_channel(): void
    {
        $result = $this->channel->joinUser($this->regularUser, (string) $this->regularUser->id);
        
        $this->assertIsArray($result);
        $this->assertEquals($this->regularUser->id, $result['id']);
        $this->assertEquals('owner', $result['role']);
    }

    /**
     * Test user cannot join another user's channel.
     */
    public function test_user_cannot_join_other_user_channel(): void
    {
        $result = $this->channel->joinUser($this->regularUser, (string) $this->adminUser->id);
        
        $this->assertFalse($result);
    }

    /**
     * Test admin can join admin channel.
     */
    public function test_admin_can_join_admin_channel(): void
    {
        $result = $this->channel->joinAdmin($this->adminUser);
        
        $this->assertIsArray($result);
        $this->assertEquals($this->adminUser->id, $result['id']);
        $this->assertEquals('admin', $result['role']);
    }

    /**
     * Test super admin can join admin channel with super-admin role.
     */
    public function test_super_admin_gets_super_admin_role(): void
    {
        $result = $this->channel->joinAdmin($this->superAdminUser);
        
        $this->assertIsArray($result);
        $this->assertEquals('super-admin', $result['role']);
    }

    /**
     * Test regular user cannot join admin channel.
     */
    public function test_regular_user_cannot_join_admin_channel(): void
    {
        $result = $this->channel->joinAdmin($this->regularUser);
        
        $this->assertFalse($result);
    }

    /**
     * Test admin can join analytics channel.
     */
    public function test_admin_can_join_analytics_channel(): void
    {
        $result = $this->channel->joinAnalytics($this->adminUser);
        
        $this->assertIsArray($result);
        $this->assertEquals('analyst', $result['role']);
    }

    /**
     * Test regular user cannot join analytics channel.
     */
    public function test_regular_user_cannot_join_analytics_channel(): void
    {
        $result = $this->channel->joinAnalytics($this->regularUser);
        
        $this->assertFalse($result);
    }
}
