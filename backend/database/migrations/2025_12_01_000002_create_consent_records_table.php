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
        Schema::create('consent_records', function (Blueprint $table) {
            $table->id();

            // Identifier (email, cookie_id, user_id, etc.)
            $table->string('identifier')->index();
            $table->string('identifier_type')->default('email'); // email, cookie_id, user_id, deleted

            // Consent details
            $table->string('consent_type')->index(); // category or purpose
            $table->boolean('granted')->default(false);

            // Tracking data for audit
            $table->string('ip_address')->nullable();
            $table->text('user_agent')->nullable();
            $table->string('source')->nullable(); // consent_banner, form, user_settings, api, user_withdrawal
            $table->string('region', 10)->nullable()->index(); // ISO country code

            // Additional metadata
            $table->json('metadata')->nullable();

            $table->timestamps();
            $table->softDeletes();

            // Composite indexes for efficient queries
            $table->index(['identifier', 'consent_type']);
            $table->index(['identifier', 'consent_type', 'created_at']);
            $table->index(['consent_type', 'granted']);
            $table->index(['created_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('consent_records');
    }
};
