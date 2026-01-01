<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Enterprise Popup Audit Logging Migration
 *
 * Creates audit trail for all popup-related actions:
 * - Create, update, delete operations
 * - Status changes (activate, pause, archive)
 * - A/B test modifications
 * - Analytics access
 * - Configuration changes
 *
 * Compliant with SOX, GDPR, and enterprise audit requirements.
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('popup_audit_logs', function (Blueprint $table) {
            $table->id();

            // Reference to the popup
            $table->foreignId('popup_id')
                ->nullable()
                ->constrained('popups')
                ->nullOnDelete();

            // Actor information
            $table->foreignId('user_id')
                ->nullable()
                ->constrained('users')
                ->nullOnDelete();
            $table->string('user_email')->nullable();
            $table->string('user_role')->nullable();

            // Action details
            $table->string('action', 50)->index();
            // Actions: create, update, delete, activate, deactivate, pause, archive,
            //          schedule, duplicate, ab_test_start, ab_test_end, analytics_view,
            //          export, restore, version_rollback

            // Change tracking
            $table->json('old_values')->nullable();
            $table->json('new_values')->nullable();
            $table->json('changed_fields')->nullable();

            // Context
            $table->string('ip_address', 45)->nullable();
            $table->string('user_agent')->nullable();
            $table->string('request_id')->nullable()->index();
            $table->string('session_id')->nullable();

            // Additional metadata
            $table->json('metadata')->nullable();
            $table->text('notes')->nullable();

            // Compliance fields
            $table->string('compliance_tag')->nullable()->index();
            $table->boolean('is_sensitive')->default(false);
            $table->timestamp('expires_at')->nullable()->index();

            $table->timestamps();

            // Indexes for efficient querying
            $table->index(['popup_id', 'created_at']);
            $table->index(['user_id', 'created_at']);
            $table->index(['action', 'created_at']);
        });

        // Create popup versions table for rollback capability
        Schema::create('popup_versions', function (Blueprint $table) {
            $table->id();

            $table->foreignId('popup_id')
                ->constrained('popups')
                ->cascadeOnDelete();

            $table->integer('version_number');
            $table->json('config');
            $table->json('design')->nullable();
            $table->json('display_rules')->nullable();

            // Change tracking
            $table->foreignId('created_by')
                ->nullable()
                ->constrained('users')
                ->nullOnDelete();
            $table->string('change_summary')->nullable();

            // Status at time of version
            $table->string('status_at_version', 50)->nullable();

            $table->timestamps();

            // Unique version per popup
            $table->unique(['popup_id', 'version_number']);
            $table->index(['popup_id', 'created_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('popup_versions');
        Schema::dropIfExists('popup_audit_logs');
    }
};
