<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // Behavior Sessions
        Schema::create('behavior_sessions', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('user_id')->nullable();
            $table->uuid('visitor_id');
            $table->string('session_fingerprint');
            
            $table->timestamp('started_at');
            $table->timestamp('ended_at')->nullable();
            $table->integer('duration_seconds')->default(0);
            
            $table->integer('page_count')->default(0);
            $table->integer('event_count')->default(0);
            
            // Scores
            $table->decimal('engagement_score', 5, 2)->default(0);
            $table->decimal('intent_score', 5, 2)->default(0);
            $table->decimal('friction_score', 5, 2)->default(0);
            $table->decimal('churn_risk_score', 5, 2)->default(0);
            
            // Flags
            $table->boolean('has_rage_clicks')->default(false);
            $table->boolean('has_form_abandonment')->default(false);
            $table->boolean('has_speed_scrolls')->default(false);
            $table->boolean('has_exit_intent')->default(false);
            $table->boolean('has_dead_clicks')->default(false);
            
            // Metadata
            $table->enum('device_type', ['mobile', 'tablet', 'desktop'])->nullable();
            $table->string('browser', 100)->nullable();
            $table->integer('viewport_width')->nullable();
            $table->integer('viewport_height')->nullable();
            $table->text('entry_url')->nullable();
            $table->text('exit_url')->nullable();
            $table->text('referrer')->nullable();
            $table->string('utm_source')->nullable();
            $table->string('utm_campaign')->nullable();
            $table->string('utm_medium')->nullable();
            
            $table->timestamps();
            
            $table->index('user_id');
            $table->index('visitor_id');
            $table->index('started_at');
            $table->index('churn_risk_score');
            $table->index('intent_score');
        });

        // Behavior Events
        Schema::create('behavior_events', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('session_id');
            $table->string('event_type', 50);
            $table->timestamp('timestamp');
            $table->text('page_url');
            
            // Event Data
            $table->string('element')->nullable();
            $table->text('element_selector')->nullable();
            $table->integer('coordinates_x')->nullable();
            $table->integer('coordinates_y')->nullable();
            $table->text('event_value')->nullable();
            $table->json('event_metadata')->nullable();
            
            // Scores
            $table->decimal('local_engagement_score', 5, 2)->default(0);
            $table->decimal('local_friction_score', 5, 2)->default(0);
            
            // Sequence
            $table->integer('sequence_number');
            $table->integer('time_since_session_start');
            $table->integer('time_since_last_event')->nullable();
            
            $table->timestamp('created_at')->useCurrent();
            
            $table->foreign('session_id')->references('id')->on('behavior_sessions')->onDelete('cascade');
            $table->index('session_id');
            $table->index('event_type');
            $table->index('timestamp');
            $table->index(['session_id', 'sequence_number']);
        });

        // Friction Points
        Schema::create('friction_points', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('session_id');
            $table->text('page_url');
            
            $table->enum('friction_type', ['rage_click', 'form_abandon', 'dead_click', 'speed_scroll', 'error', 'other']);
            $table->enum('severity', ['mild', 'moderate', 'severe', 'critical']);
            
            $table->string('element')->nullable();
            $table->text('element_selector')->nullable();
            $table->text('description')->nullable();
            
            $table->integer('event_count')->default(1);
            $table->timestamp('first_occurred_at');
            $table->timestamp('last_occurred_at');
            
            // Context
            $table->string('user_segment', 100)->nullable();
            $table->enum('device_type', ['mobile', 'tablet', 'desktop'])->nullable();
            
            // Resolution
            $table->boolean('resolved')->default(false);
            $table->timestamp('resolved_at')->nullable();
            $table->text('resolution_notes')->nullable();
            
            $table->timestamps();
            
            $table->foreign('session_id')->references('id')->on('behavior_sessions')->onDelete('cascade');
            $table->index('friction_type');
            $table->index('severity');
            $table->index('resolved');
        });

        // Intent Signals
        Schema::create('intent_signals', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('session_id');
            $table->uuid('user_id')->nullable();
            
            $table->enum('signal_type', ['cta_hover', 'form_start', 'product_view', 'pricing_view', 'demo_request', 'other']);
            $table->enum('intent_strength', ['weak', 'moderate', 'strong']);
            
            $table->string('element')->nullable();
            $table->text('page_url');
            
            $table->timestamp('timestamp');
            $table->boolean('converted')->default(false);
            $table->timestamp('conversion_timestamp')->nullable();
            
            $table->timestamp('created_at')->useCurrent();
            
            $table->foreign('session_id')->references('id')->on('behavior_sessions')->onDelete('cascade');
            $table->index('session_id');
            $table->index('user_id');
            $table->index('signal_type');
            $table->index('converted');
        });

        // Behavior Triggers
        Schema::create('behavior_triggers', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('session_id');
            $table->enum('trigger_type', ['popup', 'email', 'notification', 'task', 'webhook']);
            
            $table->string('condition_met');
            $table->timestamp('triggered_at');
            
            $table->uuid('target_id')->nullable();
            $table->string('target_type', 50)->nullable();
            
            $table->boolean('executed')->default(false);
            $table->timestamp('executed_at')->nullable();
            
            $table->json('result')->nullable();
            
            $table->timestamp('created_at')->useCurrent();
            
            $table->foreign('session_id')->references('id')->on('behavior_sessions')->onDelete('cascade');
            $table->index('session_id');
            $table->index('trigger_type');
            $table->index('executed');
        });

        // Behavior Aggregates
        Schema::create('behavior_aggregates', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->date('date');
            $table->string('page_url', 500);
            
            // Volume
            $table->integer('session_count')->default(0);
            $table->integer('event_count')->default(0);
            $table->integer('unique_visitors')->default(0);
            
            // Averages
            $table->decimal('avg_engagement_score', 5, 2)->default(0);
            $table->decimal('avg_intent_score', 5, 2)->default(0);
            $table->decimal('avg_friction_score', 5, 2)->default(0);
            $table->integer('avg_time_on_page')->default(0);
            $table->decimal('avg_scroll_depth', 5, 2)->default(0);
            
            // Friction Counts
            $table->integer('rage_click_count')->default(0);
            $table->integer('form_abandon_count')->default(0);
            $table->integer('dead_click_count')->default(0);
            $table->integer('speed_scroll_count')->default(0);
            
            // Conversion
            $table->integer('conversion_count')->default(0);
            $table->decimal('conversion_rate', 5, 2)->default(0);
            
            $table->timestamps();
            
            $table->unique(['date', 'page_url']);
            $table->index('date');
            $table->index('page_url');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('behavior_aggregates');
        Schema::dropIfExists('behavior_triggers');
        Schema::dropIfExists('intent_signals');
        Schema::dropIfExists('friction_points');
        Schema::dropIfExists('behavior_events');
        Schema::dropIfExists('behavior_sessions');
    }
};
