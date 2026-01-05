<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Create email marketing tables for:
 * - Webhooks
 * - Webhook deliveries
 * - Domains
 */
return new class extends Migration
{
    public function up(): void
    {
        // Email Webhooks Table
        if (!Schema::hasTable('email_webhooks')) {
            Schema::create('email_webhooks', function (Blueprint $table) {
                $table->id();
                $table->string('name');
                $table->string('url', 500);
                $table->json('events');
                $table->string('secret', 128);
                $table->string('previous_secret', 128)->nullable();
                $table->json('headers')->nullable();
                $table->text('description')->nullable();
                $table->enum('status', ['active', 'inactive', 'failing'])->default('active');
                $table->unsignedInteger('consecutive_failures')->default(0);
                $table->timestamp('last_triggered_at')->nullable();
                $table->timestamp('secret_rotated_at')->nullable();
                $table->foreignId('created_by')->nullable()->constrained('users')->nullOnDelete();
                $table->timestamps();
                $table->softDeletes();

                $table->index('status');
                $table->index('created_by');
            });
        }

        // Email Webhook Deliveries Table
        if (!Schema::hasTable('email_webhook_deliveries')) {
            Schema::create('email_webhook_deliveries', function (Blueprint $table) {
                $table->id();
                $table->foreignId('webhook_id')->constrained('email_webhooks')->cascadeOnDelete();
                $table->string('event');
                $table->longText('payload');
                $table->enum('status', ['success', 'failed', 'pending'])->default('pending');
                $table->unsignedSmallInteger('response_code')->nullable();
                $table->text('response_body')->nullable();
                $table->unsignedInteger('response_time_ms')->nullable();
                $table->unsignedBigInteger('retry_of')->nullable();
                $table->timestamp('created_at');

                $table->index(['webhook_id', 'status']);
                $table->index(['webhook_id', 'created_at']);
                $table->index('event');
            });
        }

        // Email Domains Table
        if (!Schema::hasTable('email_domains')) {
            Schema::create('email_domains', function (Blueprint $table) {
                $table->id();
                $table->string('domain')->unique();
                $table->string('verification_token', 100);
                $table->boolean('verified')->default(false);
                $table->timestamp('verified_at')->nullable();
                $table->string('dkim_selector', 50)->nullable();
                $table->boolean('dkim_verified')->default(false);
                $table->boolean('spf_verified')->default(false);
                $table->boolean('dmarc_verified')->default(false);
                $table->timestamp('last_checked_at')->nullable();
                $table->foreignId('created_by')->nullable()->constrained('users')->nullOnDelete();
                $table->timestamps();
                $table->softDeletes();

                $table->index('verified');
                $table->index('created_by');
            });
        }

        // Email Logs Table (for tracking events)
        if (!Schema::hasTable('email_logs')) {
            Schema::create('email_logs', function (Blueprint $table) {
                $table->id();
                $table->foreignId('campaign_id')->nullable()->constrained('email_campaigns')->nullOnDelete();
                $table->foreignId('subscriber_id')->nullable();
                $table->string('event_type', 50); // sent, delivered, opened, clicked, bounced, complained
                $table->string('email', 255)->nullable();
                $table->string('from_email', 255)->nullable();
                $table->string('link_url', 500)->nullable();
                $table->string('device_type', 20)->nullable();
                $table->string('browser', 50)->nullable();
                $table->string('os', 50)->nullable();
                $table->string('country', 2)->nullable();
                $table->string('city', 100)->nullable();
                $table->string('ip_address', 45)->nullable();
                $table->string('user_agent', 500)->nullable();
                $table->json('metadata')->nullable();
                $table->timestamp('created_at');

                $table->index(['campaign_id', 'event_type']);
                $table->index(['subscriber_id', 'event_type']);
                $table->index(['event_type', 'created_at']);
                $table->index('created_at');
            });
        }

        // Add missing columns to newsletter_subscriptions if needed
        if (Schema::hasTable('newsletter_subscriptions')) {
            Schema::table('newsletter_subscriptions', function (Blueprint $table) {
                if (!Schema::hasColumn('newsletter_subscriptions', 'engagement_level')) {
                    $table->enum('engagement_level', ['high', 'medium', 'low', 'inactive'])
                        ->nullable()
                        ->after('status');
                }
                if (!Schema::hasColumn('newsletter_subscriptions', 'tags')) {
                    $table->json('tags')->nullable()->after('preferences');
                }
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('email_webhook_deliveries');
        Schema::dropIfExists('email_webhooks');
        Schema::dropIfExists('email_domains');
        Schema::dropIfExists('email_logs');

        if (Schema::hasTable('newsletter_subscriptions')) {
            Schema::table('newsletter_subscriptions', function (Blueprint $table) {
                if (Schema::hasColumn('newsletter_subscriptions', 'engagement_level')) {
                    $table->dropColumn('engagement_level');
                }
                if (Schema::hasColumn('newsletter_subscriptions', 'tags')) {
                    $table->dropColumn('tags');
                }
            });
        }
    }
};
