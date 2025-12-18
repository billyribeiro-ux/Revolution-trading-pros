<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Laravel Pulse Tables Migration
 *
 * Creates tables for performance monitoring and analytics.
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // Pulse aggregates table - stores aggregated metrics
        Schema::create('pulse_aggregates', function (Blueprint $table) {
            $table->id();
            $table->unsignedInteger('bucket');
            $table->unsignedMediumInteger('period');
            $table->string('type');
            $table->mediumText('key');
            $table->string('key_hash');
            $table->string('aggregate');
            $table->decimal('value', 20, 2);
            $table->unsignedInteger('count')->nullable();

            $table->unique(['bucket', 'period', 'type', 'aggregate', 'key_hash'], 'pulse_aggregates_unique');
            $table->index(['period', 'bucket'], 'pulse_aggregates_period_bucket_index');
            $table->index(['type', 'period', 'bucket'], 'pulse_aggregates_type_period_bucket_index');
            $table->index('bucket', 'pulse_aggregates_bucket_index');
        });

        // Pulse entries table - stores raw event entries
        Schema::create('pulse_entries', function (Blueprint $table) {
            $table->id();
            $table->unsignedInteger('timestamp');
            $table->string('type');
            $table->mediumText('key');
            $table->string('key_hash');
            $table->bigInteger('value')->nullable();

            $table->index('timestamp', 'pulse_entries_timestamp_index');
            $table->index(['type', 'timestamp'], 'pulse_entries_type_timestamp_index');
            $table->index(['type', 'key_hash', 'timestamp'], 'pulse_entries_type_key_hash_timestamp_index');
        });

        // Pulse values table - stores key-value pairs
        Schema::create('pulse_values', function (Blueprint $table) {
            $table->id();
            $table->unsignedInteger('timestamp');
            $table->string('type');
            $table->mediumText('key');
            $table->string('key_hash');
            $table->mediumText('value');

            $table->unique(['type', 'key_hash'], 'pulse_values_type_key_hash_unique');
            $table->index('timestamp', 'pulse_values_timestamp_index');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('pulse_aggregates');
        Schema::dropIfExists('pulse_entries');
        Schema::dropIfExists('pulse_values');
    }
};
