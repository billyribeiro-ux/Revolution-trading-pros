<?php

/**
 * Revolution Trading Pros - User Sessions Migration
 * Enterprise-grade session management for single-session authentication
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */

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
        Schema::create('user_sessions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->uuid('session_id')->unique();
            $table->string('device_id', 64)->index();
            $table->string('device_name', 255);
            $table->enum('device_type', ['desktop', 'mobile', 'tablet', 'unknown'])->default('unknown');
            $table->string('os_name', 100)->nullable();
            $table->string('os_version', 50)->nullable();
            $table->string('browser_name', 100)->nullable();
            $table->string('browser_version', 50)->nullable();
            $table->string('ip_address', 45); // IPv6 compatible
            $table->text('user_agent');
            $table->string('location_country', 100)->nullable();
            $table->string('location_city', 100)->nullable();
            $table->decimal('location_lat', 10, 8)->nullable();
            $table->decimal('location_lng', 11, 8)->nullable();
            $table->timestamp('last_activity_at');
            $table->timestamp('expires_at');
            $table->boolean('is_active')->default(true);
            $table->enum('logout_reason', [
                'manual',           // User clicked logout
                'expired',          // Session expired
                'revoked_by_user',  // User revoked from settings
                'revoked_by_admin', // Admin revoked
                'new_login',        // New login from another device
                'security',         // Security concern
                'password_change',  // User changed password
            ])->nullable();
            $table->json('metadata')->nullable(); // Additional session data
            $table->timestamps();

            // Indexes for performance
            $table->index(['user_id', 'is_active']);
            $table->index(['user_id', 'device_id']);
            $table->index(['expires_at', 'is_active']);
            $table->index('last_activity_at');
        });

        // Add session tracking to personal_access_tokens
        Schema::table('personal_access_tokens', function (Blueprint $table) {
            $table->foreignId('user_session_id')
                ->nullable()
                ->after('tokenable_id')
                ->constrained('user_sessions')
                ->onDelete('cascade');
            $table->enum('token_type', ['access', 'refresh'])->default('access')->after('name');
        });

        // Create audit_logs table for comprehensive logging
        Schema::create('audit_logs', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->nullable()->constrained()->onDelete('set null');
            $table->foreignId('user_session_id')->nullable()->constrained('user_sessions')->onDelete('set null');
            $table->string('action', 100)->index(); // login, logout, create, update, delete, etc.
            $table->string('entity_type', 100)->nullable()->index(); // User, Post, Product, etc.
            $table->unsignedBigInteger('entity_id')->nullable();
            $table->string('ip_address', 45);
            $table->text('user_agent')->nullable();
            $table->json('old_values')->nullable();
            $table->json('new_values')->nullable();
            $table->json('metadata')->nullable();
            $table->enum('severity', ['info', 'warning', 'critical'])->default('info');
            $table->timestamps();

            // Indexes for querying
            $table->index(['entity_type', 'entity_id']);
            $table->index('created_at');
            $table->index(['user_id', 'action']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('personal_access_tokens', function (Blueprint $table) {
            $table->dropForeign(['user_session_id']);
            $table->dropColumn(['user_session_id', 'token_type']);
        });

        Schema::dropIfExists('audit_logs');
        Schema::dropIfExists('user_sessions');
    }
};
