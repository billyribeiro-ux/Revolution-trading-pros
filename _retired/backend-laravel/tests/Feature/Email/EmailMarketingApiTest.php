<?php

namespace Tests\Feature\Email;

use Tests\TestCase;
use Illuminate\Foundation\Testing\RefreshDatabase;
use App\Models\User;
use App\Models\EmailCampaign;
use App\Models\EmailTemplate;
use App\Models\NewsletterSubscription;
use App\Models\EmailWebhook;
use App\Models\EmailDomain;

/**
 * Email Marketing API Integration Tests
 *
 * Tests all 8 sections of the email marketing system:
 * 1. Broadcasts (Campaigns)
 * 2. Templates
 * 3. Audience (Subscribers)
 * 4. Metrics
 * 5. Domains
 * 6. Logs
 * 7. Webhooks
 * 8. Settings
 */
class EmailMarketingApiTest extends TestCase
{
    use RefreshDatabase;

    protected User $adminUser;

    protected function setUp(): void
    {
        parent::setUp();

        // Create admin role if using spatie/permissions
        if (class_exists(\Spatie\Permission\Models\Role::class)) {
            \Spatie\Permission\Models\Role::firstOrCreate(
                ['name' => 'admin', 'guard_name' => 'web']
            );
        }

        // Create admin user
        $this->adminUser = User::factory()->create([
            'email' => 'admin@test.com',
        ]);

        // Assign admin role (if using spatie/permissions)
        if (method_exists($this->adminUser, 'assignRole')) {
            $this->adminUser->assignRole('admin');
        }
    }

    /**
     * Test unauthenticated access is blocked
     */
    public function test_unauthenticated_access_returns_401(): void
    {
        $response = $this->getJson('/api/admin/email/campaigns');
        $response->assertStatus(401);
    }

    // ================================================================
    // CAMPAIGNS (BROADCASTS) TESTS
    // ================================================================

    public function test_can_list_campaigns(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/campaigns');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
            'meta' => ['current_page', 'total'],
        ]);
    }

    public function test_can_get_campaign_stats(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/campaigns/stats');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'total_campaigns',
                'total_sent',
                'total_opened',
            ],
        ]);
    }

    // ================================================================
    // TEMPLATES TESTS
    // ================================================================

    public function test_can_list_templates(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/templates');
        $response->assertOk();
    }

    // ================================================================
    // SUBSCRIBERS (AUDIENCE) TESTS
    // ================================================================

    public function test_can_list_subscribers(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/subscribers');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
            'meta',
        ]);
    }

    public function test_can_get_subscriber_stats(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/subscribers/stats');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'total',
                'active',
                'pending',
            ],
        ]);
    }

    public function test_can_get_subscriber_tags(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/subscribers/tags');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
        ]);
    }

    // ================================================================
    // METRICS (ANALYTICS) TESTS
    // ================================================================

    public function test_can_get_metrics_dashboard(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/metrics/dashboard');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'overview',
                'campaigns',
                'subscribers',
            ],
        ]);
    }

    public function test_can_get_realtime_metrics(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/metrics/realtime');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'timestamp',
            ],
        ]);
    }

    // ================================================================
    // DOMAINS TESTS
    // ================================================================

    public function test_can_list_domains(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/domains');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
        ]);
    }

    public function test_can_get_domain_stats(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/domains/stats');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'total',
                'verified',
            ],
        ]);
    }

    // ================================================================
    // AUDIT LOGS TESTS
    // ================================================================

    public function test_can_list_audit_logs(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/logs');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
            'meta',
        ]);
    }

    public function test_can_get_log_filter_options(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/logs/filters');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'actions',
                'resource_types',
                'statuses',
            ],
        ]);
    }

    // ================================================================
    // WEBHOOKS TESTS
    // ================================================================

    public function test_can_list_webhooks(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/webhooks');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
            'meta',
        ]);
    }

    public function test_can_get_webhook_events(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/webhooks/events');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data',
        ]);
    }

    public function test_can_get_webhook_stats(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/webhooks/stats');
        $response->assertOk();
        $response->assertJsonStructure([
            'success',
            'data' => [
                'total',
                'active',
            ],
        ]);
    }

    // ================================================================
    // SETTINGS TESTS
    // ================================================================

    public function test_can_get_email_settings(): void
    {
        $this->actingAs($this->adminUser);

        $response = $this->getJson('/api/admin/email/settings');
        $response->assertOk();
    }

    // ================================================================
    // PUBLIC NEWSLETTER TESTS
    // ================================================================

    public function test_newsletter_subscribe_requires_email(): void
    {
        $response = $this->postJson('/api/newsletter/subscribe', []);
        $response->assertStatus(422);
        $response->assertJsonValidationErrors(['email']);
    }

    public function test_newsletter_subscribe_requires_consent(): void
    {
        $response = $this->postJson('/api/newsletter/subscribe', [
            'email' => 'test@example.com',
        ]);
        $response->assertStatus(422);
        $response->assertJsonValidationErrors(['consent']);
    }
}
