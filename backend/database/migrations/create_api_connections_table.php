<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * API Connections Table Migration
 *
 * Centralized storage for all third-party API connections with
 * encrypted credentials, status tracking, and health monitoring.
 *
 * @level L11 Principal Engineer - Apple-grade implementation
 */
return new class extends Migration
{
    public function up(): void
    {
        Schema::create('api_connections', function (Blueprint $table) {
            $table->uuid('id')->primary();

            // Connection identification
            $table->string('service_key', 50)->unique(); // e.g., 'stripe', 'google_analytics'
            $table->string('service_name', 100); // Display name
            $table->string('category', 50); // payments, analytics, email, storage, etc.

            // Status tracking
            $table->enum('status', ['connected', 'disconnected', 'error', 'expired', 'pending'])->default('disconnected');
            $table->timestamp('connected_at')->nullable();
            $table->timestamp('last_verified_at')->nullable();
            $table->timestamp('expires_at')->nullable();

            // Encrypted credentials (stored as encrypted JSON)
            $table->text('credentials')->nullable(); // Encrypted JSON: {api_key, secret, token, etc.}

            // Configuration
            $table->json('config')->nullable(); // Service-specific settings
            $table->json('metadata')->nullable(); // Additional data (property_id, account_id, etc.)

            // Health monitoring
            $table->integer('health_score')->default(100); // 0-100
            $table->timestamp('last_health_check')->nullable();
            $table->text('last_error')->nullable();
            $table->integer('error_count')->default(0);
            $table->integer('success_count')->default(0);

            // Usage tracking
            $table->bigInteger('api_calls_today')->default(0);
            $table->bigInteger('api_calls_total')->default(0);
            $table->timestamp('last_api_call')->nullable();

            // OAuth specific
            $table->boolean('is_oauth')->default(false);
            $table->text('access_token')->nullable();
            $table->text('refresh_token')->nullable();
            $table->string('oauth_scopes')->nullable();

            // Webhook support
            $table->string('webhook_url')->nullable();
            $table->string('webhook_secret')->nullable();
            $table->boolean('webhooks_enabled')->default(false);

            // Environment & version
            $table->enum('environment', ['production', 'sandbox', 'test'])->default('production');
            $table->string('api_version')->nullable();

            $table->timestamps();
            $table->softDeletes();

            // Indexes
            $table->index('category');
            $table->index('status');
            $table->index(['status', 'category']);
        });

        // Create connection logs table for audit trail
        Schema::create('api_connection_logs', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('connection_id');
            $table->string('action', 50); // connected, disconnected, verified, error, api_call
            $table->string('status', 20)->nullable();
            $table->json('details')->nullable();
            $table->string('ip_address', 45)->nullable();
            $table->string('user_agent')->nullable();
            $table->foreignId('user_id')->nullable()->constrained()->nullOnDelete();
            $table->timestamps();

            $table->foreign('connection_id')
                ->references('id')
                ->on('api_connections')
                ->onDelete('cascade');

            $table->index(['connection_id', 'created_at']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('api_connection_logs');
        Schema::dropIfExists('api_connections');
    }
};
