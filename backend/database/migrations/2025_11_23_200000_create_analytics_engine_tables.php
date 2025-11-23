<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Enterprise Analytics Engine - Core Tables Migration
 *
 * This migration creates the foundational tables for the enterprise-grade
 * analytics system designed to surpass Mixpanel, Amplitude, GA4, and Heap.
 *
 * Architecture: Event-driven with time-series optimization
 * Scale: Designed for billions of events with partitioning support
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // =====================================================================
        // GLOBAL ANALYTICS EVENTS TABLE
        // Core event ingestion table - unified schema for all platform events
        // =====================================================================
        Schema::create('analytics_events', function (Blueprint $table) {
            $table->id();
            $table->uuid('event_id')->unique()->index();

            // Event Classification
            $table->string('event_name', 100)->index();
            $table->string('event_category', 50)->index(); // page_view, user_action, transaction, system
            $table->string('event_type', 50)->index(); // click, view, submit, purchase, etc.
            $table->string('event_source', 50)->index(); // web, api, mobile, webhook

            // Entity References (polymorphic tracking)
            $table->string('entity_type', 50)->nullable()->index(); // post, video, product, form, etc.
            $table->unsignedBigInteger('entity_id')->nullable()->index();
            $table->index(['entity_type', 'entity_id'], 'idx_entity');

            // User Context
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->string('anonymous_id', 64)->nullable()->index(); // For non-authenticated users
            $table->string('session_id', 64)->index();

            // Attribution & Source Tracking
            $table->string('utm_source', 100)->nullable()->index();
            $table->string('utm_medium', 100)->nullable();
            $table->string('utm_campaign', 100)->nullable()->index();
            $table->string('utm_term', 100)->nullable();
            $table->string('utm_content', 100)->nullable();
            $table->string('referrer', 500)->nullable();
            $table->string('referrer_domain', 100)->nullable()->index();

            // Channel Attribution
            $table->string('channel', 50)->nullable()->index(); // organic, paid, social, email, direct
            $table->string('channel_group', 50)->nullable(); // acquisition, engagement, retention

            // Device & Environment
            $table->string('device_type', 20)->nullable()->index(); // desktop, mobile, tablet
            $table->string('device_brand', 50)->nullable();
            $table->string('device_model', 50)->nullable();
            $table->string('browser', 50)->nullable()->index();
            $table->string('browser_version', 20)->nullable();
            $table->string('os', 50)->nullable()->index();
            $table->string('os_version', 20)->nullable();
            $table->string('screen_resolution', 20)->nullable();
            $table->string('viewport_size', 20)->nullable();

            // Geographic Data
            $table->string('ip_address', 45)->nullable();
            $table->string('country_code', 2)->nullable()->index();
            $table->string('country', 100)->nullable();
            $table->string('region', 100)->nullable();
            $table->string('city', 100)->nullable();
            $table->string('timezone', 50)->nullable();
            $table->decimal('latitude', 10, 7)->nullable();
            $table->decimal('longitude', 10, 7)->nullable();

            // Page Context
            $table->string('page_url', 500)->nullable();
            $table->string('page_path', 255)->nullable()->index();
            $table->string('page_title', 255)->nullable();
            $table->string('page_type', 50)->nullable()->index(); // landing, product, checkout, etc.

            // Revenue & Value Tracking
            $table->decimal('event_value', 15, 4)->nullable(); // Monetary value
            $table->string('currency', 3)->default('USD');
            $table->decimal('revenue', 15, 4)->nullable();
            $table->integer('quantity')->nullable();

            // Event Properties (flexible JSON storage)
            $table->json('properties')->nullable(); // Custom event properties
            $table->json('user_properties')->nullable(); // User traits at event time
            $table->json('context')->nullable(); // Additional context data

            // Funnel & Journey Tracking
            $table->string('funnel_id', 64)->nullable()->index();
            $table->integer('funnel_step')->nullable();
            $table->string('journey_stage', 50)->nullable()->index(); // awareness, consideration, decision, retention

            // A/B Testing
            $table->string('experiment_id', 64)->nullable()->index();
            $table->string('variant_id', 64)->nullable();

            // Performance Metrics
            $table->integer('page_load_time')->nullable(); // milliseconds
            $table->integer('time_on_page')->nullable(); // seconds
            $table->integer('scroll_depth')->nullable(); // percentage

            // Engagement Scoring
            $table->decimal('engagement_score', 5, 2)->nullable();
            $table->boolean('is_bounce')->default(false);
            $table->boolean('is_conversion')->default(false)->index();
            $table->boolean('is_goal_completion')->default(false);

            // Timestamps with precision
            $table->timestamp('event_timestamp')->useCurrent()->index();
            $table->integer('event_day')->index(); // YYYYMMDD format for partitioning
            $table->integer('event_hour')->nullable(); // 0-23
            $table->timestamps();

            // Processing Status
            $table->boolean('is_processed')->default(false)->index();
            $table->timestamp('processed_at')->nullable();

            // Composite indexes for common queries
            $table->index(['user_id', 'event_timestamp'], 'idx_user_timeline');
            $table->index(['event_name', 'event_timestamp'], 'idx_event_timeline');
            $table->index(['session_id', 'event_timestamp'], 'idx_session_timeline');
            $table->index(['event_category', 'event_type', 'event_timestamp'], 'idx_category_type_time');
            $table->index(['channel', 'is_conversion', 'event_timestamp'], 'idx_channel_conversion');
            $table->index(['event_day', 'event_name'], 'idx_daily_events');

            // Foreign key (optional - can be disabled for performance)
            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // ANALYTICS SESSIONS TABLE
        // Session-level aggregation for user journey analysis
        // =====================================================================
        Schema::create('analytics_sessions', function (Blueprint $table) {
            $table->id();
            $table->string('session_id', 64)->unique()->index();
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->string('anonymous_id', 64)->nullable()->index();

            // Session Timing
            $table->timestamp('started_at')->index();
            $table->timestamp('ended_at')->nullable();
            $table->integer('duration_seconds')->default(0);
            $table->boolean('is_active')->default(true)->index();

            // Session Metrics
            $table->integer('page_views')->default(0);
            $table->integer('events_count')->default(0);
            $table->integer('interactions_count')->default(0);
            $table->decimal('total_value', 15, 4)->default(0);

            // Entry & Exit
            $table->string('landing_page', 500)->nullable();
            $table->string('exit_page', 500)->nullable();
            $table->string('entry_source', 100)->nullable();

            // Attribution (first touch)
            $table->string('utm_source', 100)->nullable();
            $table->string('utm_medium', 100)->nullable();
            $table->string('utm_campaign', 100)->nullable();
            $table->string('channel', 50)->nullable()->index();
            $table->string('referrer', 500)->nullable();

            // Device Info
            $table->string('device_type', 20)->nullable();
            $table->string('browser', 50)->nullable();
            $table->string('os', 50)->nullable();

            // Geographic
            $table->string('country_code', 2)->nullable()->index();
            $table->string('city', 100)->nullable();

            // Engagement
            $table->decimal('engagement_score', 5, 2)->default(0);
            $table->boolean('is_bounce')->default(false);
            $table->boolean('had_conversion')->default(false)->index();
            $table->integer('goals_completed')->default(0);

            // Journey Stage
            $table->string('journey_stage', 50)->nullable();
            $table->json('pages_visited')->nullable();
            $table->json('events_summary')->nullable();

            $table->timestamps();

            $table->index(['user_id', 'started_at'], 'idx_user_sessions');
            $table->index(['started_at', 'had_conversion'], 'idx_session_conversions');

            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // KPI DEFINITIONS TABLE
        // Configurable KPI metrics definitions
        // =====================================================================
        Schema::create('analytics_kpi_definitions', function (Blueprint $table) {
            $table->id();
            $table->string('kpi_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // KPI Configuration
            $table->string('category', 50)->index(); // revenue, engagement, conversion, retention, growth
            $table->string('type', 30); // count, sum, average, ratio, percentage, rate
            $table->string('aggregation', 30)->default('sum'); // sum, avg, min, max, count, distinct

            // Calculation Definition
            $table->string('metric_field', 100)->nullable(); // Field to aggregate
            $table->string('numerator_event', 100)->nullable(); // For ratio KPIs
            $table->string('denominator_event', 100)->nullable(); // For ratio KPIs
            $table->json('filters')->nullable(); // Additional filters
            $table->json('formula')->nullable(); // Complex calculation formula

            // Display Configuration
            $table->string('format', 30)->default('number'); // number, currency, percentage, duration
            $table->integer('decimal_places')->default(2);
            $table->string('unit', 20)->nullable();
            $table->string('icon', 50)->nullable();
            $table->string('color', 20)->nullable();

            // Targets & Thresholds
            $table->decimal('target_value', 15, 4)->nullable();
            $table->decimal('warning_threshold', 15, 4)->nullable();
            $table->decimal('critical_threshold', 15, 4)->nullable();
            $table->string('comparison_period', 30)->default('previous_period'); // previous_period, previous_year

            // Hierarchy & Grouping
            $table->integer('sort_order')->default(0);
            $table->boolean('is_primary')->default(false);
            $table->boolean('is_active')->default(true);
            $table->string('dashboard_section', 50)->nullable();

            // Time Configuration
            $table->string('default_period', 30)->default('30d'); // 7d, 30d, 90d, 1y
            $table->json('available_periods')->nullable();

            $table->timestamps();
        });

        // =====================================================================
        // KPI VALUES TABLE
        // Computed KPI values with time-series storage
        // =====================================================================
        Schema::create('analytics_kpi_values', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('kpi_definition_id')->index();

            // Time Period
            $table->string('period_type', 20)->index(); // hourly, daily, weekly, monthly, yearly
            $table->date('period_start')->index();
            $table->date('period_end');
            $table->integer('period_key')->index(); // YYYYMMDD or YYYYMM format

            // Segmentation (optional)
            $table->string('segment_type', 50)->nullable()->index();
            $table->string('segment_value', 100)->nullable()->index();

            // Computed Values
            $table->decimal('value', 20, 4);
            $table->decimal('previous_value', 20, 4)->nullable();
            $table->decimal('change_value', 20, 4)->nullable();
            $table->decimal('change_percentage', 10, 4)->nullable();
            $table->string('trend', 20)->nullable(); // up, down, stable

            // Statistical Data
            $table->decimal('min_value', 20, 4)->nullable();
            $table->decimal('max_value', 20, 4)->nullable();
            $table->decimal('avg_value', 20, 4)->nullable();
            $table->decimal('std_deviation', 20, 4)->nullable();
            $table->bigInteger('sample_size')->nullable();

            // Target Comparison
            $table->decimal('target_value', 15, 4)->nullable();
            $table->decimal('target_variance', 10, 4)->nullable();
            $table->string('target_status', 20)->nullable(); // on_track, at_risk, behind

            // Anomaly Detection
            $table->boolean('is_anomaly')->default(false)->index();
            $table->decimal('anomaly_score', 5, 2)->nullable();
            $table->string('anomaly_type', 30)->nullable(); // spike, drop, trend_break

            $table->timestamps();

            $table->unique(['kpi_definition_id', 'period_type', 'period_start', 'segment_type', 'segment_value'], 'uk_kpi_period_segment');
            $table->index(['period_type', 'period_start'], 'idx_kpi_period');

            $table->foreign('kpi_definition_id')->references('id')->on('analytics_kpi_definitions')->onDelete('cascade');
        });

        // =====================================================================
        // COHORT DEFINITIONS TABLE
        // User cohort configuration for cohort analysis
        // =====================================================================
        Schema::create('analytics_cohorts', function (Blueprint $table) {
            $table->id();
            $table->string('cohort_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // Cohort Configuration
            $table->string('cohort_type', 30)->index(); // acquisition, behavioral, value, custom
            $table->string('grouping_period', 20)->default('weekly'); // daily, weekly, monthly

            // Cohort Criteria
            $table->string('entry_event', 100); // Event that defines cohort entry
            $table->json('entry_conditions')->nullable(); // Additional entry conditions
            $table->string('retention_event', 100)->nullable(); // Event that defines retention
            $table->json('retention_conditions')->nullable();

            // Analysis Configuration
            $table->integer('analysis_periods')->default(12); // Number of periods to analyze
            $table->string('metric_type', 30)->default('retention'); // retention, revenue, engagement

            // Display
            $table->boolean('is_active')->default(true);
            $table->integer('sort_order')->default(0);
            $table->json('chart_config')->nullable();

            $table->timestamps();
        });

        // =====================================================================
        // COHORT ANALYSIS RESULTS TABLE
        // Pre-computed cohort analysis data
        // =====================================================================
        Schema::create('analytics_cohort_results', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('cohort_id')->index();

            // Cohort Period
            $table->date('cohort_date')->index(); // The cohort's entry date
            $table->string('cohort_period', 20); // weekly_2024_01, monthly_2024_01

            // Period Analysis
            $table->integer('period_number'); // 0, 1, 2, 3... (periods since entry)
            $table->date('analysis_date');

            // Metrics
            $table->bigInteger('cohort_size'); // Initial cohort size
            $table->bigInteger('active_users'); // Users active in this period
            $table->decimal('retention_rate', 6, 4); // Percentage retained
            $table->decimal('cumulative_retention', 6, 4)->nullable();

            // Value Metrics
            $table->decimal('total_revenue', 15, 4)->default(0);
            $table->decimal('avg_revenue_per_user', 15, 4)->nullable();
            $table->decimal('ltv_to_date', 15, 4)->nullable();

            // Engagement Metrics
            $table->integer('total_events')->default(0);
            $table->decimal('avg_events_per_user', 10, 2)->nullable();
            $table->decimal('engagement_score', 5, 2)->nullable();

            $table->timestamps();

            $table->unique(['cohort_id', 'cohort_date', 'period_number'], 'uk_cohort_period');
            $table->index(['cohort_date', 'period_number'], 'idx_cohort_analysis');

            $table->foreign('cohort_id')->references('id')->on('analytics_cohorts')->onDelete('cascade');
        });

        // =====================================================================
        // FUNNEL DEFINITIONS TABLE
        // Conversion funnel configurations
        // =====================================================================
        Schema::create('analytics_funnels', function (Blueprint $table) {
            $table->id();
            $table->string('funnel_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // Funnel Configuration
            $table->string('funnel_type', 30)->default('conversion'); // conversion, engagement, purchase
            $table->integer('conversion_window_hours')->default(168); // 7 days default
            $table->boolean('is_ordered')->default(true); // Must complete steps in order

            // Display
            $table->boolean('is_active')->default(true);
            $table->integer('sort_order')->default(0);
            $table->json('chart_config')->nullable();

            $table->timestamps();
        });

        // =====================================================================
        // FUNNEL STEPS TABLE
        // Individual steps within funnels
        // =====================================================================
        Schema::create('analytics_funnel_steps', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('funnel_id')->index();

            $table->integer('step_number');
            $table->string('name', 100);
            $table->text('description')->nullable();

            // Step Definition
            $table->string('event_name', 100);
            $table->json('event_conditions')->nullable(); // Additional conditions

            // Optional Step Configuration
            $table->boolean('is_optional')->default(false);
            $table->integer('time_limit_hours')->nullable(); // Max time to complete step

            $table->timestamps();

            $table->unique(['funnel_id', 'step_number'], 'uk_funnel_step');

            $table->foreign('funnel_id')->references('id')->on('analytics_funnels')->onDelete('cascade');
        });

        // =====================================================================
        // FUNNEL CONVERSIONS TABLE
        // Individual user funnel progress tracking
        // =====================================================================
        Schema::create('analytics_funnel_conversions', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('funnel_id')->index();
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->string('anonymous_id', 64)->nullable()->index();
            $table->string('session_id', 64)->index();

            // Funnel Progress
            $table->integer('current_step')->default(0);
            $table->integer('max_step_reached')->default(0);
            $table->boolean('is_converted')->default(false)->index();
            $table->integer('dropped_at_step')->nullable();

            // Timing
            $table->timestamp('started_at')->index();
            $table->timestamp('converted_at')->nullable();
            $table->timestamp('last_activity_at')->nullable();
            $table->integer('total_duration_seconds')->nullable();
            $table->json('step_timestamps')->nullable(); // Timestamp for each step
            $table->json('step_durations')->nullable(); // Duration between steps

            // Attribution
            $table->string('entry_channel', 50)->nullable();
            $table->string('utm_source', 100)->nullable();
            $table->string('utm_campaign', 100)->nullable();

            // Value
            $table->decimal('conversion_value', 15, 4)->nullable();

            // Analysis Period
            $table->date('started_date')->index();

            $table->timestamps();

            $table->index(['funnel_id', 'is_converted', 'started_date'], 'idx_funnel_conversion_analysis');
            $table->index(['funnel_id', 'dropped_at_step'], 'idx_funnel_dropoff');

            $table->foreign('funnel_id')->references('id')->on('analytics_funnels')->onDelete('cascade');
            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // FUNNEL ANALYTICS AGGREGATES TABLE
        // Pre-computed funnel statistics
        // =====================================================================
        Schema::create('analytics_funnel_aggregates', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('funnel_id')->index();

            // Time Period
            $table->string('period_type', 20)->index(); // daily, weekly, monthly
            $table->date('period_start')->index();
            $table->date('period_end');

            // Segmentation
            $table->string('segment_type', 50)->nullable();
            $table->string('segment_value', 100)->nullable();

            // Overall Metrics
            $table->bigInteger('total_entries')->default(0);
            $table->bigInteger('total_conversions')->default(0);
            $table->decimal('conversion_rate', 6, 4)->default(0);
            $table->decimal('avg_conversion_time_hours', 10, 2)->nullable();
            $table->decimal('total_value', 15, 4)->default(0);

            // Step-by-Step Data (JSON for flexibility)
            $table->json('step_metrics')->nullable(); // {step: {entries, exits, conversion_rate, avg_time}}

            // Comparison
            $table->decimal('prev_conversion_rate', 6, 4)->nullable();
            $table->decimal('conversion_rate_change', 8, 4)->nullable();

            $table->timestamps();

            $table->unique(['funnel_id', 'period_type', 'period_start', 'segment_type', 'segment_value'], 'uk_funnel_aggregate');

            $table->foreign('funnel_id')->references('id')->on('analytics_funnels')->onDelete('cascade');
        });

        // =====================================================================
        // SEGMENTS DEFINITIONS TABLE
        // User segmentation configurations
        // =====================================================================
        Schema::create('analytics_segments', function (Blueprint $table) {
            $table->id();
            $table->string('segment_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // Segment Type
            $table->string('segment_type', 30)->index(); // static, dynamic, computed

            // Segment Rules (for dynamic segments)
            $table->json('rules')->nullable(); // Rule conditions
            $table->string('rule_operator', 10)->default('AND'); // AND, OR

            // Refresh Configuration
            $table->string('refresh_frequency', 20)->default('daily'); // realtime, hourly, daily
            $table->timestamp('last_computed_at')->nullable();

            // Metrics
            $table->bigInteger('user_count')->default(0);
            $table->decimal('percentage_of_total', 6, 4)->nullable();

            // Display
            $table->boolean('is_active')->default(true);
            $table->boolean('is_system')->default(false); // System-defined segments
            $table->string('color', 20)->nullable();
            $table->string('icon', 50)->nullable();
            $table->integer('sort_order')->default(0);

            $table->timestamps();
        });

        // =====================================================================
        // SEGMENT MEMBERS TABLE
        // User-segment membership
        // =====================================================================
        Schema::create('analytics_segment_members', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('segment_id')->index();
            $table->unsignedBigInteger('user_id')->index();

            // Membership
            $table->timestamp('added_at');
            $table->timestamp('removed_at')->nullable();
            $table->boolean('is_active')->default(true);

            // Computed Properties at Join Time
            $table->json('properties_snapshot')->nullable();

            $table->timestamps();

            $table->unique(['segment_id', 'user_id'], 'uk_segment_user');

            $table->foreign('segment_id')->references('id')->on('analytics_segments')->onDelete('cascade');
            $table->foreign('user_id')->references('id')->on('users')->onDelete('cascade');
        });

        // =====================================================================
        // TIME SERIES AGGREGATES TABLE
        // Pre-aggregated time-series data for fast queries
        // =====================================================================
        Schema::create('analytics_time_series', function (Blueprint $table) {
            $table->id();

            // Metric Identification
            $table->string('metric_key', 100)->index();
            $table->string('metric_type', 30); // event_count, user_count, revenue, etc.

            // Time Granularity
            $table->string('granularity', 20)->index(); // minute, hour, day, week, month
            $table->timestamp('timestamp')->index();
            $table->integer('timestamp_key')->index(); // YYYYMMDDHH format

            // Dimensions (for grouping)
            $table->string('dimension_1', 100)->nullable()->index();
            $table->string('dimension_1_value', 255)->nullable();
            $table->string('dimension_2', 100)->nullable();
            $table->string('dimension_2_value', 255)->nullable();

            // Aggregated Values
            $table->bigInteger('count_value')->default(0);
            $table->decimal('sum_value', 20, 4)->default(0);
            $table->decimal('avg_value', 20, 4)->nullable();
            $table->decimal('min_value', 20, 4)->nullable();
            $table->decimal('max_value', 20, 4)->nullable();
            $table->bigInteger('distinct_count')->nullable();

            // Statistical
            $table->decimal('variance', 20, 4)->nullable();
            $table->decimal('p50', 20, 4)->nullable(); // Median
            $table->decimal('p90', 20, 4)->nullable();
            $table->decimal('p99', 20, 4)->nullable();

            $table->timestamps();

            $table->unique(['metric_key', 'granularity', 'timestamp', 'dimension_1', 'dimension_1_value'], 'uk_timeseries');
            $table->index(['metric_key', 'granularity', 'timestamp'], 'idx_timeseries_query');
        });

        // =====================================================================
        // ATTRIBUTION MODELS TABLE
        // Attribution model configurations
        // =====================================================================
        Schema::create('analytics_attribution_models', function (Blueprint $table) {
            $table->id();
            $table->string('model_key', 50)->unique()->index();
            $table->string('name', 100);
            $table->text('description')->nullable();

            // Model Type
            $table->string('model_type', 30); // first_touch, last_touch, linear, time_decay, position_based, data_driven

            // Configuration
            $table->json('config')->nullable(); // Model-specific configuration
            $table->integer('lookback_window_days')->default(30);

            $table->boolean('is_active')->default(true);
            $table->boolean('is_default')->default(false);

            $table->timestamps();
        });

        // =====================================================================
        // ATTRIBUTION TOUCHPOINTS TABLE
        // User journey touchpoints for attribution
        // =====================================================================
        Schema::create('analytics_attribution_touchpoints', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->string('anonymous_id', 64)->nullable()->index();
            $table->string('conversion_id', 64)->index(); // Links touchpoints to conversion

            // Touchpoint Details
            $table->string('channel', 50)->index();
            $table->string('source', 100)->nullable();
            $table->string('medium', 100)->nullable();
            $table->string('campaign', 100)->nullable()->index();
            $table->string('content', 100)->nullable();
            $table->string('term', 100)->nullable();

            // Touchpoint Context
            $table->timestamp('touchpoint_at')->index();
            $table->integer('touchpoint_number'); // Order in journey
            $table->boolean('is_first_touch')->default(false);
            $table->boolean('is_last_touch')->default(false);
            $table->boolean('is_converting_touch')->default(false);

            // Attribution Credit (populated after conversion)
            $table->decimal('first_touch_credit', 6, 4)->default(0);
            $table->decimal('last_touch_credit', 6, 4)->default(0);
            $table->decimal('linear_credit', 6, 4)->default(0);
            $table->decimal('time_decay_credit', 6, 4)->default(0);
            $table->decimal('position_credit', 6, 4)->default(0);

            // Event Reference
            $table->unsignedBigInteger('event_id')->nullable();

            $table->timestamps();

            $table->index(['conversion_id', 'touchpoint_number'], 'idx_conversion_journey');
            $table->index(['channel', 'touchpoint_at'], 'idx_channel_touchpoints');

            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // ATTRIBUTION CONVERSIONS TABLE
        // Conversion events for attribution analysis
        // =====================================================================
        Schema::create('analytics_attribution_conversions', function (Blueprint $table) {
            $table->id();
            $table->string('conversion_id', 64)->unique()->index();
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->string('anonymous_id', 64)->nullable();

            // Conversion Details
            $table->string('conversion_type', 50)->index(); // purchase, signup, subscription, lead
            $table->string('conversion_event', 100);
            $table->timestamp('converted_at')->index();

            // Value
            $table->decimal('conversion_value', 15, 4)->default(0);
            $table->string('currency', 3)->default('USD');

            // Journey Summary
            $table->integer('touchpoint_count')->default(0);
            $table->integer('days_to_convert')->nullable();

            // Primary Attribution (most common models)
            $table->string('first_touch_channel', 50)->nullable()->index();
            $table->string('last_touch_channel', 50)->nullable()->index();

            // Entity Reference
            $table->string('entity_type', 50)->nullable();
            $table->unsignedBigInteger('entity_id')->nullable();

            $table->timestamps();

            $table->index(['conversion_type', 'converted_at'], 'idx_conversion_type_time');

            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // ANALYTICS GOALS TABLE
        // Goal tracking configurations
        // =====================================================================
        Schema::create('analytics_goals', function (Blueprint $table) {
            $table->id();
            $table->string('goal_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // Goal Type
            $table->string('goal_type', 30); // destination, duration, pages_per_session, event

            // Goal Definition
            $table->string('event_name', 100)->nullable();
            $table->json('conditions')->nullable();
            $table->decimal('target_value', 15, 4)->nullable();

            // Value
            $table->decimal('goal_value', 15, 4)->default(0); // Default value per completion

            // Status
            $table->boolean('is_active')->default(true);
            $table->integer('sort_order')->default(0);

            $table->timestamps();
        });

        // =====================================================================
        // ANALYTICS GOAL COMPLETIONS TABLE
        // Individual goal completion tracking
        // =====================================================================
        Schema::create('analytics_goal_completions', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('goal_id')->index();
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->string('session_id', 64)->index();

            // Completion Details
            $table->timestamp('completed_at')->index();
            $table->decimal('value', 15, 4)->default(0);

            // Attribution
            $table->string('channel', 50)->nullable();
            $table->string('source', 100)->nullable();

            // Event Reference
            $table->unsignedBigInteger('event_id')->nullable();

            $table->timestamps();

            $table->index(['goal_id', 'completed_at'], 'idx_goal_completions_time');

            $table->foreign('goal_id')->references('id')->on('analytics_goals')->onDelete('cascade');
            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // ANALYTICS ALERTS TABLE
        // Alert configurations for anomaly detection
        // =====================================================================
        Schema::create('analytics_alerts', function (Blueprint $table) {
            $table->id();
            $table->string('alert_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // Alert Type
            $table->string('alert_type', 30); // threshold, anomaly, trend, comparison

            // Metric Reference
            $table->string('metric_type', 50); // kpi, event_count, conversion_rate, etc.
            $table->string('metric_key', 100);

            // Conditions
            $table->string('operator', 20); // gt, lt, gte, lte, eq, change_gt, change_lt
            $table->decimal('threshold_value', 20, 4)->nullable();
            $table->decimal('threshold_percentage', 10, 4)->nullable();

            // Notification Configuration
            $table->json('notification_channels')->nullable(); // email, slack, webhook
            $table->json('recipients')->nullable();

            // Frequency
            $table->string('check_frequency', 20)->default('hourly'); // realtime, hourly, daily
            $table->timestamp('last_checked_at')->nullable();
            $table->timestamp('last_triggered_at')->nullable();

            // Status
            $table->boolean('is_active')->default(true);
            $table->string('current_status', 20)->default('normal'); // normal, warning, critical

            $table->timestamps();
        });

        // =====================================================================
        // ANALYTICS ALERT HISTORY TABLE
        // Alert trigger history
        // =====================================================================
        Schema::create('analytics_alert_history', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('alert_id')->index();

            // Trigger Details
            $table->timestamp('triggered_at')->index();
            $table->string('severity', 20); // info, warning, critical

            // Values at Trigger Time
            $table->decimal('current_value', 20, 4);
            $table->decimal('threshold_value', 20, 4);
            $table->decimal('previous_value', 20, 4)->nullable();

            // Context
            $table->text('message')->nullable();
            $table->json('context_data')->nullable();

            // Resolution
            $table->boolean('is_resolved')->default(false);
            $table->timestamp('resolved_at')->nullable();
            $table->string('resolution_type', 30)->nullable(); // auto, manual, acknowledged

            $table->timestamps();

            $table->index(['alert_id', 'triggered_at'], 'idx_alert_history');

            $table->foreign('alert_id')->references('id')->on('analytics_alerts')->onDelete('cascade');
        });

        // =====================================================================
        // ANALYTICS REPORTS TABLE
        // Saved report configurations
        // =====================================================================
        Schema::create('analytics_reports', function (Blueprint $table) {
            $table->id();
            $table->string('report_key', 100)->unique()->index();
            $table->string('name', 150);
            $table->text('description')->nullable();

            // Report Type
            $table->string('report_type', 30); // dashboard, funnel, cohort, custom

            // Configuration
            $table->json('config')->nullable(); // Report-specific configuration
            $table->json('filters')->nullable();
            $table->json('columns')->nullable();
            $table->json('charts')->nullable();

            // Scheduling
            $table->boolean('is_scheduled')->default(false);
            $table->string('schedule_frequency', 20)->nullable(); // daily, weekly, monthly
            $table->json('schedule_recipients')->nullable();
            $table->timestamp('last_sent_at')->nullable();

            // Access
            $table->unsignedBigInteger('created_by')->nullable();
            $table->boolean('is_public')->default(false);
            $table->boolean('is_system')->default(false);

            $table->timestamps();

            $table->foreign('created_by')->references('id')->on('users')->onDelete('set null');
        });

        // =====================================================================
        // ANALYTICS FORECASTS TABLE
        // Predictive analytics and forecasts
        // =====================================================================
        Schema::create('analytics_forecasts', function (Blueprint $table) {
            $table->id();
            $table->string('metric_key', 100)->index();

            // Forecast Configuration
            $table->string('model_type', 30); // linear, exponential, arima, prophet
            $table->date('forecast_date')->index();
            $table->string('granularity', 20); // daily, weekly, monthly

            // Forecast Values
            $table->decimal('predicted_value', 20, 4);
            $table->decimal('lower_bound', 20, 4)->nullable(); // Confidence interval
            $table->decimal('upper_bound', 20, 4)->nullable();
            $table->decimal('confidence_level', 5, 4)->default(0.95);

            // Accuracy Tracking
            $table->decimal('actual_value', 20, 4)->nullable(); // Populated after date passes
            $table->decimal('error', 20, 4)->nullable();
            $table->decimal('error_percentage', 10, 4)->nullable();

            // Model Metadata
            $table->json('model_params')->nullable();
            $table->decimal('model_accuracy', 6, 4)->nullable(); // Historical accuracy

            $table->timestamp('generated_at');
            $table->timestamps();

            $table->unique(['metric_key', 'forecast_date', 'granularity'], 'uk_forecast');
            $table->index(['forecast_date', 'metric_key'], 'idx_forecast_date');
        });

        // =====================================================================
        // USER ANALYTICS PROFILE TABLE
        // Aggregated user-level analytics data
        // =====================================================================
        Schema::create('analytics_user_profiles', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('user_id')->unique()->index();

            // Lifecycle
            $table->timestamp('first_seen_at')->nullable();
            $table->timestamp('last_seen_at')->nullable();
            $table->integer('days_since_first_seen')->default(0);
            $table->integer('days_since_last_seen')->default(0);
            $table->string('lifecycle_stage', 30)->nullable()->index(); // new, active, at_risk, dormant, churned

            // Activity Metrics
            $table->bigInteger('total_sessions')->default(0);
            $table->bigInteger('total_events')->default(0);
            $table->bigInteger('total_page_views')->default(0);
            $table->integer('avg_session_duration')->default(0);
            $table->decimal('avg_pages_per_session', 8, 2)->default(0);

            // Engagement
            $table->decimal('engagement_score', 5, 2)->default(0)->index();
            $table->string('engagement_level', 20)->nullable(); // low, medium, high, power
            $table->integer('streak_days')->default(0);
            $table->integer('longest_streak')->default(0);

            // Conversion & Revenue
            $table->boolean('has_converted')->default(false)->index();
            $table->timestamp('first_conversion_at')->nullable();
            $table->integer('total_conversions')->default(0);
            $table->decimal('total_revenue', 15, 4)->default(0);
            $table->decimal('avg_order_value', 15, 4)->default(0);
            $table->decimal('predicted_ltv', 15, 4)->nullable();
            $table->string('value_tier', 20)->nullable()->index(); // low, medium, high, vip

            // Attribution
            $table->string('acquisition_channel', 50)->nullable()->index();
            $table->string('acquisition_source', 100)->nullable();
            $table->string('acquisition_campaign', 100)->nullable();

            // Preferences
            $table->string('preferred_device', 20)->nullable();
            $table->string('preferred_browser', 50)->nullable();
            $table->string('primary_country', 2)->nullable();
            $table->json('interests')->nullable(); // Computed interests

            // Segment Membership
            $table->json('segment_ids')->nullable();

            // Risk Indicators
            $table->decimal('churn_risk_score', 5, 2)->nullable();
            $table->string('churn_risk_level', 20)->nullable(); // low, medium, high

            // Last Activity
            $table->string('last_event_name', 100)->nullable();
            $table->timestamp('last_event_at')->nullable();

            $table->timestamps();

            $table->index(['lifecycle_stage', 'engagement_score'], 'idx_user_lifecycle');
            $table->index(['value_tier', 'total_revenue'], 'idx_user_value');

            $table->foreign('user_id')->references('id')->on('users')->onDelete('cascade');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('analytics_user_profiles');
        Schema::dropIfExists('analytics_forecasts');
        Schema::dropIfExists('analytics_reports');
        Schema::dropIfExists('analytics_alert_history');
        Schema::dropIfExists('analytics_alerts');
        Schema::dropIfExists('analytics_goal_completions');
        Schema::dropIfExists('analytics_goals');
        Schema::dropIfExists('analytics_attribution_conversions');
        Schema::dropIfExists('analytics_attribution_touchpoints');
        Schema::dropIfExists('analytics_attribution_models');
        Schema::dropIfExists('analytics_time_series');
        Schema::dropIfExists('analytics_segment_members');
        Schema::dropIfExists('analytics_segments');
        Schema::dropIfExists('analytics_funnel_aggregates');
        Schema::dropIfExists('analytics_funnel_conversions');
        Schema::dropIfExists('analytics_funnel_steps');
        Schema::dropIfExists('analytics_funnels');
        Schema::dropIfExists('analytics_cohort_results');
        Schema::dropIfExists('analytics_cohorts');
        Schema::dropIfExists('analytics_kpi_values');
        Schema::dropIfExists('analytics_kpi_definitions');
        Schema::dropIfExists('analytics_sessions');
        Schema::dropIfExists('analytics_events');
    }
};
