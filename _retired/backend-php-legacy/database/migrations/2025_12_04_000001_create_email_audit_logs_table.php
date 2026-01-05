<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Create Email Audit Logs Table
 *
 * Enterprise-grade audit logging for all email marketing admin actions.
 * Provides comprehensive tracking for compliance, debugging, and analytics.
 */
return new class extends Migration
{
    public function up(): void
    {
        Schema::create('email_audit_logs', function (Blueprint $table) {
            $table->id();
            $table->uuid('uuid')->unique();

            // Actor information
            $table->unsignedBigInteger('user_id')->nullable();
            $table->string('user_email')->nullable();
            $table->string('user_name')->nullable();
            $table->string('user_role')->nullable();

            // Action details
            $table->string('action', 100)->index(); // create, update, delete, send, schedule, etc.
            $table->string('resource_type', 100)->index(); // campaign, template, subscriber, etc.
            $table->unsignedBigInteger('resource_id')->nullable()->index();
            $table->string('resource_name')->nullable();

            // Change tracking
            $table->json('old_values')->nullable();
            $table->json('new_values')->nullable();
            $table->json('changed_fields')->nullable();

            // Request context
            $table->string('ip_address', 45)->nullable();
            $table->text('user_agent')->nullable();
            $table->string('request_method', 10)->nullable();
            $table->text('request_url')->nullable();
            $table->string('request_id', 64)->nullable()->index();

            // Session tracking
            $table->string('session_id', 64)->nullable();
            $table->string('device_type', 50)->nullable();
            $table->string('browser', 100)->nullable();
            $table->string('os', 100)->nullable();

            // Geographic data
            $table->string('country', 2)->nullable();
            $table->string('city', 100)->nullable();
            $table->string('timezone', 50)->nullable();

            // Status and metadata
            $table->string('status', 20)->default('success'); // success, failed, pending
            $table->text('error_message')->nullable();
            $table->json('metadata')->nullable();
            $table->json('tags')->nullable();

            // Timing
            $table->integer('duration_ms')->nullable(); // Action duration in milliseconds
            $table->timestamp('created_at')->useCurrent()->index();

            // Indexes for common queries
            $table->index(['user_id', 'created_at']);
            $table->index(['resource_type', 'resource_id']);
            $table->index(['action', 'created_at']);
            $table->index(['status', 'created_at']);

            // Foreign key
            $table->foreign('user_id')->references('id')->on('users')->onDelete('set null');
        });

        // Create summary table for quick aggregations
        Schema::create('email_audit_summaries', function (Blueprint $table) {
            $table->id();
            $table->date('date')->index();
            $table->string('action', 100)->index();
            $table->string('resource_type', 100)->index();
            $table->unsignedBigInteger('user_id')->nullable()->index();
            $table->integer('count')->default(0);
            $table->integer('success_count')->default(0);
            $table->integer('failed_count')->default(0);
            $table->float('avg_duration_ms')->nullable();
            $table->timestamps();

            $table->unique(['date', 'action', 'resource_type', 'user_id'], 'audit_summary_unique');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('email_audit_summaries');
        Schema::dropIfExists('email_audit_logs');
    }
};
