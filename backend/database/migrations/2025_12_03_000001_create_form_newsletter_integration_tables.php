<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Form-Newsletter Integration Migration
 *
 * Creates tables and columns for seamless integration between
 * the form builder and newsletter subscription system.
 *
 * @level ICT11 Principal Engineer
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // Newsletter categories for segmented campaigns
        Schema::create('newsletter_categories', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->string('icon')->nullable();
            $table->string('color', 7)->nullable();
            $table->integer('sort_order')->default(0);
            $table->boolean('is_active')->default(true);
            $table->boolean('is_default')->default(false);
            $table->unsignedInteger('subscriber_count')->default(0);
            $table->unsignedInteger('email_count')->default(0);
            $table->decimal('avg_open_rate', 5, 2)->default(0);
            $table->decimal('avg_click_rate', 5, 2)->default(0);
            $table->json('settings')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamp('last_sent_at')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'sort_order']);
            $table->index('subscriber_count');
        });

        // Enhance newsletter_subscriptions table with additional fields
        Schema::table('newsletter_subscriptions', function (Blueprint $table) {
            // Check and add columns if they don't exist
            if (!Schema::hasColumn('newsletter_subscriptions', 'first_name')) {
                $table->string('first_name')->nullable()->after('name');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'last_name')) {
                $table->string('last_name')->nullable()->after('first_name');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'source')) {
                $table->string('source')->default('direct')->after('status');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'referring_url')) {
                $table->string('referring_url')->nullable()->after('source');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'ip_address')) {
                $table->string('ip_address', 45)->nullable()->after('referring_url');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'user_agent')) {
                $table->text('user_agent')->nullable()->after('ip_address');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'segments')) {
                $table->json('segments')->nullable()->after('user_agent');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'preferences')) {
                $table->json('preferences')->nullable()->after('segments');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'interests')) {
                $table->json('interests')->nullable()->after('preferences');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'metadata')) {
                $table->json('metadata')->nullable()->after('interests');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'double_opt_in')) {
                $table->boolean('double_opt_in')->default(true)->after('metadata');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'consent_given_at')) {
                $table->timestamp('consent_given_at')->nullable()->after('double_opt_in');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'consent_ip')) {
                $table->string('consent_ip', 45)->nullable()->after('consent_given_at');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'consent_method')) {
                $table->string('consent_method')->nullable()->after('consent_ip');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'utm_source')) {
                $table->string('utm_source')->nullable()->after('consent_method');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'utm_medium')) {
                $table->string('utm_medium')->nullable()->after('utm_source');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'utm_campaign')) {
                $table->string('utm_campaign')->nullable()->after('utm_medium');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'open_rate')) {
                $table->decimal('open_rate', 5, 2)->default(0)->after('utm_campaign');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'click_rate')) {
                $table->decimal('click_rate', 5, 2)->default(0)->after('open_rate');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'engagement_score')) {
                $table->decimal('engagement_score', 5, 2)->default(0)->after('click_rate');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'last_opened_at')) {
                $table->timestamp('last_opened_at')->nullable()->after('engagement_score');
            }
            if (!Schema::hasColumn('newsletter_subscriptions', 'last_clicked_at')) {
                $table->timestamp('last_clicked_at')->nullable()->after('last_opened_at');
            }

            // Add indexes for common queries
            $table->index('source');
            $table->index('engagement_score');
        });

        // Add newsletter_subscription_id to form_submissions
        Schema::table('form_submissions', function (Blueprint $table) {
            if (!Schema::hasColumn('form_submissions', 'newsletter_subscription_id')) {
                $table->foreignId('newsletter_subscription_id')
                    ->nullable()
                    ->after('metadata')
                    ->constrained('newsletter_subscriptions')
                    ->onDelete('set null');
            }
        });

        // Create pivot table for category-subscription relationship (optional M2M)
        Schema::create('newsletter_category_subscription', function (Blueprint $table) {
            $table->id();
            $table->foreignId('newsletter_category_id')
                ->constrained('newsletter_categories')
                ->onDelete('cascade');
            $table->foreignId('newsletter_subscription_id')
                ->constrained('newsletter_subscriptions')
                ->onDelete('cascade');
            $table->timestamp('subscribed_at')->useCurrent();
            $table->timestamps();

            $table->unique(
                ['newsletter_category_id', 'newsletter_subscription_id'],
                'category_subscription_unique'
            );
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('newsletter_category_subscription');

        Schema::table('form_submissions', function (Blueprint $table) {
            if (Schema::hasColumn('form_submissions', 'newsletter_subscription_id')) {
                $table->dropForeign(['newsletter_subscription_id']);
                $table->dropColumn('newsletter_subscription_id');
            }
        });

        Schema::table('newsletter_subscriptions', function (Blueprint $table) {
            $columns = [
                'first_name', 'last_name', 'source', 'referring_url',
                'ip_address', 'user_agent', 'segments', 'preferences',
                'interests', 'metadata', 'double_opt_in', 'consent_given_at',
                'consent_ip', 'consent_method', 'utm_source', 'utm_medium',
                'utm_campaign', 'open_rate', 'click_rate', 'engagement_score',
                'last_opened_at', 'last_clicked_at',
            ];

            foreach ($columns as $column) {
                if (Schema::hasColumn('newsletter_subscriptions', $column)) {
                    $table->dropColumn($column);
                }
            }
        });

        Schema::dropIfExists('newsletter_categories');
    }
};
