<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Create Email Analytics Tables
 *
 * Dedicated analytics and reporting tables for high-performance
 * email marketing metrics and insights.
 */
return new class extends Migration
{
    public function up(): void
    {
        // Campaign performance metrics (hourly aggregation)
        Schema::create('email_campaign_metrics', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('campaign_id')->index();
            $table->timestamp('hour')->index();
            $table->integer('sent')->default(0);
            $table->integer('delivered')->default(0);
            $table->integer('opened')->default(0);
            $table->integer('unique_opens')->default(0);
            $table->integer('clicked')->default(0);
            $table->integer('unique_clicks')->default(0);
            $table->integer('bounced')->default(0);
            $table->integer('soft_bounced')->default(0);
            $table->integer('hard_bounced')->default(0);
            $table->integer('complained')->default(0);
            $table->integer('unsubscribed')->default(0);
            $table->decimal('revenue', 12, 2)->default(0);
            $table->integer('conversions')->default(0);
            $table->timestamps();

            $table->unique(['campaign_id', 'hour']);
            $table->index(['hour', 'campaign_id']);
        });

        // Link click tracking
        Schema::create('email_link_clicks', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('campaign_id')->index();
            $table->unsignedBigInteger('subscriber_id')->nullable()->index();
            $table->string('link_url', 2048);
            $table->string('link_hash', 64)->index();
            $table->string('link_text')->nullable();
            $table->integer('position')->nullable();
            $table->string('device_type', 20)->nullable();
            $table->string('email_client', 100)->nullable();
            $table->string('os', 50)->nullable();
            $table->string('browser', 50)->nullable();
            $table->string('country', 2)->nullable();
            $table->string('city', 100)->nullable();
            $table->string('ip_address', 45)->nullable();
            $table->timestamp('clicked_at')->useCurrent()->index();

            $table->index(['campaign_id', 'link_hash']);
            $table->index(['clicked_at', 'campaign_id']);
        });

        // Email open tracking
        Schema::create('email_opens', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('campaign_id')->index();
            $table->unsignedBigInteger('subscriber_id')->nullable()->index();
            $table->string('device_type', 20)->nullable();
            $table->string('email_client', 100)->nullable();
            $table->string('os', 50)->nullable();
            $table->string('country', 2)->nullable();
            $table->string('city', 100)->nullable();
            $table->string('ip_address', 45)->nullable();
            $table->boolean('is_first_open')->default(true);
            $table->integer('open_count')->default(1);
            $table->timestamp('first_opened_at')->useCurrent();
            $table->timestamp('last_opened_at')->useCurrent();
            $table->timestamps();

            $table->index(['campaign_id', 'subscriber_id']);
            $table->index(['first_opened_at', 'campaign_id']);
        });

        // Subscriber engagement scores (daily snapshot)
        Schema::create('email_engagement_scores', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('subscriber_id')->index();
            $table->date('date')->index();
            $table->decimal('score', 5, 2)->default(0);
            $table->decimal('open_score', 5, 2)->default(0);
            $table->decimal('click_score', 5, 2)->default(0);
            $table->decimal('recency_score', 5, 2)->default(0);
            $table->decimal('frequency_score', 5, 2)->default(0);
            $table->integer('emails_received_30d')->default(0);
            $table->integer('emails_opened_30d')->default(0);
            $table->integer('emails_clicked_30d')->default(0);
            $table->timestamp('last_engagement_at')->nullable();
            $table->string('engagement_level', 20)->default('new'); // new, cold, warm, hot
            $table->json('factors')->nullable();
            $table->timestamps();

            $table->unique(['subscriber_id', 'date']);
            $table->index(['date', 'score']);
            $table->index(['engagement_level', 'date']);
        });

        // Campaign comparison metrics
        Schema::create('email_campaign_comparisons', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('campaign_a_id')->index();
            $table->unsignedBigInteger('campaign_b_id')->index();
            $table->string('comparison_type', 50); // ab_test, historical, segment
            $table->json('metrics_a');
            $table->json('metrics_b');
            $table->json('statistical_significance')->nullable();
            $table->string('winner')->nullable();
            $table->decimal('improvement_percent', 8, 2)->nullable();
            $table->timestamps();

            $table->index(['campaign_a_id', 'campaign_b_id']);
        });

        // Deliverability metrics (daily)
        Schema::create('email_deliverability_metrics', function (Blueprint $table) {
            $table->id();
            $table->date('date')->index();
            $table->string('domain', 255)->nullable()->index();
            $table->string('provider', 50)->nullable(); // gmail, outlook, yahoo, etc.
            $table->integer('sent')->default(0);
            $table->integer('delivered')->default(0);
            $table->integer('bounced')->default(0);
            $table->integer('soft_bounced')->default(0);
            $table->integer('hard_bounced')->default(0);
            $table->integer('deferred')->default(0);
            $table->integer('spam_reports')->default(0);
            $table->decimal('delivery_rate', 5, 2)->default(0);
            $table->decimal('bounce_rate', 5, 2)->default(0);
            $table->decimal('spam_rate', 5, 2)->default(0);
            $table->decimal('reputation_score', 5, 2)->nullable();
            $table->json('bounce_reasons')->nullable();
            $table->timestamps();

            $table->unique(['date', 'domain', 'provider']);
            $table->index(['date', 'delivery_rate']);
        });

        // Revenue attribution
        Schema::create('email_revenue_attribution', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('campaign_id')->index();
            $table->unsignedBigInteger('subscriber_id')->nullable()->index();
            $table->unsignedBigInteger('order_id')->nullable();
            $table->string('order_number')->nullable();
            $table->decimal('revenue', 12, 2);
            $table->string('currency', 3)->default('USD');
            $table->string('attribution_model', 50)->default('last_click'); // last_click, first_click, linear
            $table->decimal('attribution_weight', 5, 4)->default(1);
            $table->integer('days_to_conversion')->nullable();
            $table->timestamp('email_sent_at')->nullable();
            $table->timestamp('email_opened_at')->nullable();
            $table->timestamp('email_clicked_at')->nullable();
            $table->timestamp('converted_at')->useCurrent();
            $table->timestamps();

            $table->index(['campaign_id', 'converted_at']);
            $table->index(['subscriber_id', 'converted_at']);
        });

        // Segment performance
        Schema::create('email_segment_performance', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('segment_id')->index();
            $table->date('date')->index();
            $table->integer('subscriber_count')->default(0);
            $table->integer('campaigns_sent')->default(0);
            $table->integer('emails_sent')->default(0);
            $table->integer('emails_opened')->default(0);
            $table->integer('emails_clicked')->default(0);
            $table->decimal('avg_open_rate', 5, 2)->default(0);
            $table->decimal('avg_click_rate', 5, 2)->default(0);
            $table->decimal('avg_engagement_score', 5, 2)->default(0);
            $table->decimal('total_revenue', 12, 2)->default(0);
            $table->integer('unsubscribes')->default(0);
            $table->timestamps();

            $table->unique(['segment_id', 'date']);
            $table->index(['date', 'avg_engagement_score']);
        });

        // Send time optimization data
        Schema::create('email_send_time_analytics', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('subscriber_id')->index();
            $table->tinyInteger('hour_of_day'); // 0-23
            $table->tinyInteger('day_of_week'); // 0-6 (Sunday = 0)
            $table->integer('emails_sent')->default(0);
            $table->integer('emails_opened')->default(0);
            $table->integer('emails_clicked')->default(0);
            $table->decimal('open_rate', 5, 2)->default(0);
            $table->decimal('click_rate', 5, 2)->default(0);
            $table->timestamp('last_updated_at')->useCurrent();

            $table->unique(['subscriber_id', 'hour_of_day', 'day_of_week']);
            $table->index(['subscriber_id', 'open_rate']);
        });

        // Aggregate daily stats for quick dashboard access
        Schema::create('email_daily_stats', function (Blueprint $table) {
            $table->id();
            $table->date('date')->unique();
            $table->integer('total_sent')->default(0);
            $table->integer('total_delivered')->default(0);
            $table->integer('total_opened')->default(0);
            $table->integer('total_clicked')->default(0);
            $table->integer('total_bounced')->default(0);
            $table->integer('total_complained')->default(0);
            $table->integer('total_unsubscribed')->default(0);
            $table->integer('new_subscribers')->default(0);
            $table->integer('campaigns_sent')->default(0);
            $table->decimal('total_revenue', 12, 2)->default(0);
            $table->decimal('avg_open_rate', 5, 2)->default(0);
            $table->decimal('avg_click_rate', 5, 2)->default(0);
            $table->decimal('avg_bounce_rate', 5, 2)->default(0);
            $table->json('top_campaigns')->nullable();
            $table->json('device_breakdown')->nullable();
            $table->json('geo_breakdown')->nullable();
            $table->timestamps();

            $table->index(['date']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('email_daily_stats');
        Schema::dropIfExists('email_send_time_analytics');
        Schema::dropIfExists('email_segment_performance');
        Schema::dropIfExists('email_revenue_attribution');
        Schema::dropIfExists('email_deliverability_metrics');
        Schema::dropIfExists('email_campaign_comparisons');
        Schema::dropIfExists('email_engagement_scores');
        Schema::dropIfExists('email_opens');
        Schema::dropIfExists('email_link_clicks');
        Schema::dropIfExists('email_campaign_metrics');
    }
};
