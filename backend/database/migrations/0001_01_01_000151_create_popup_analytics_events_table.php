<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Server-Side Analytics Events Table
 *
 * Stores backup analytics events when client-side tracking is blocked.
 * Privacy-preserving design with no PII storage.
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('popup_analytics_events', function (Blueprint $table) {
            $table->id();

            // Reference to popup
            $table->foreignId('popup_id')
                ->constrained('popups')
                ->cascadeOnDelete();

            // Event details
            $table->string('event_type', 50)->index();
            $table->string('session_id', 100)->index();
            $table->string('visitor_hash', 64)->index(); // SHA-256 hash

            // Page context
            $table->string('page_url', 2048)->nullable();
            $table->string('referrer', 2048)->nullable();

            // Device information
            $table->string('device_type', 20)->nullable()->index();
            $table->string('browser', 50)->nullable();
            $table->string('os', 50)->nullable();

            // Geographic (no PII - country only)
            $table->string('country', 2)->nullable()->index();

            // Campaign tracking
            $table->string('utm_source', 100)->nullable()->index();
            $table->string('utm_medium', 100)->nullable();
            $table->string('utm_campaign', 100)->nullable();

            // Event metrics
            $table->decimal('event_value', 10, 2)->nullable();
            $table->integer('engagement_time_ms')->nullable();
            $table->tinyInteger('scroll_depth')->nullable();
            $table->tinyInteger('form_fields_filled')->nullable();

            // A/B testing
            $table->string('variant_id', 50)->nullable()->index();

            // Additional data
            $table->json('metadata')->nullable();

            // Bot detection
            $table->boolean('is_bot')->default(false)->index();

            $table->timestamp('created_at')->useCurrent()->index();

            // Composite indexes for common queries
            $table->index(['popup_id', 'event_type', 'created_at']);
            $table->index(['popup_id', 'variant_id', 'event_type']);
            $table->index(['visitor_hash', 'created_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('popup_analytics_events');
    }
};
