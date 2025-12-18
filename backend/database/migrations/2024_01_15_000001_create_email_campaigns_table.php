<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        if (Schema::hasTable('email_campaigns')) {
            return; // Table already exists, skip
        }

        Schema::create('email_campaigns', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->foreignId('template_id')->constrained('email_templates')->onDelete('cascade');
            $table->unsignedBigInteger('segment_id')->nullable();
            $table->string('subject');
            $table->string('from_name')->nullable();
            $table->string('from_email')->nullable();
            $table->string('reply_to')->nullable();
            $table->timestamp('scheduled_at')->nullable();
            $table->timestamp('sent_at')->nullable();
            $table->enum('status', ['draft', 'scheduled', 'sending', 'sent', 'paused', 'cancelled'])->default('draft');
            $table->unsignedInteger('total_recipients')->default(0);
            $table->unsignedInteger('sent_count')->default(0);
            $table->unsignedInteger('opened_count')->default(0);
            $table->unsignedInteger('clicked_count')->default(0);
            $table->unsignedInteger('bounced_count')->default(0);
            $table->unsignedInteger('unsubscribed_count')->default(0);
            $table->json('ab_test_config')->nullable();
            $table->foreignId('created_by')->constrained('users')->onDelete('cascade');
            $table->timestamps();

            // Indexes
            $table->index('status');
            $table->index('scheduled_at');
            $table->index('sent_at');
            $table->index('created_by');
        });

        // Create email_logs table for tracking individual email events
        if (Schema::hasTable('email_logs')) {
            return; // Table already exists, skip
        }

        Schema::create('email_logs', function (Blueprint $table) {
            $table->id();
            $table->foreignId('campaign_id')->nullable()->constrained('email_campaigns')->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained('users')->onDelete('set null');
            $table->string('email');
            $table->enum('event_type', ['sent', 'delivered', 'opened', 'clicked', 'bounced', 'complained', 'unsubscribed']);
            $table->string('link_url')->nullable();
            $table->string('device_type')->nullable();
            $table->string('browser')->nullable();
            $table->string('os')->nullable();
            $table->string('country')->nullable();
            $table->string('city')->nullable();
            $table->string('ip_address')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamps();

            // Indexes
            $table->index('campaign_id');
            $table->index('user_id');
            $table->index('event_type');
            $table->index('created_at');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('email_logs');
        Schema::dropIfExists('email_campaigns');
    }
};
