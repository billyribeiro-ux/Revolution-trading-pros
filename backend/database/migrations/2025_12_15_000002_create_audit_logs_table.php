<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

/**
 * Create audit logs table for compliance tracking.
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('audit_logs', function (Blueprint $table) {
            $table->uuid('id')->primary();

            // Event classification
            $table->string('event_type', 50)->index();

            // Tracing
            $table->string('correlation_id', 100)->nullable()->index();

            // Actor (who performed the action)
            $table->foreignId('user_id')
                ->nullable()
                ->constrained('users')
                ->nullOnDelete();

            // Request context
            $table->string('ip_address', 45)->nullable();
            $table->string('user_agent', 255)->nullable();

            // Event data (JSON)
            $table->json('data');

            // Timestamp (immutable, no updated_at)
            $table->timestamp('created_at')->useCurrent();

            // Indexes for common queries
            $table->index(['user_id', 'created_at']);
            $table->index(['event_type', 'created_at']);
            $table->index('created_at');
        });

        // Add comment for documentation
        DB::statement("COMMENT ON TABLE audit_logs IS 'Immutable audit trail for compliance and forensics'");
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('audit_logs');
    }
};
