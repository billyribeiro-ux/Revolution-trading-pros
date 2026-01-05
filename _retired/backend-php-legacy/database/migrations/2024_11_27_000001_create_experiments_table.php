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
        Schema::create('experiments', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('key', 100)->unique();
            $table->string('name', 255);
            $table->text('description')->nullable();
            $table->json('variants'); // Default set in model
            $table->json('variant_weights')->nullable();
            $table->string('default_variant', 50)->default('control');
            $table->unsignedTinyInteger('rollout_percentage')->default(100);
            $table->boolean('enabled')->default(false);
            $table->timestamp('starts_at')->nullable();
            $table->timestamp('ends_at')->nullable();
            $table->timestamps();

            $table->index(['enabled', 'starts_at', 'ends_at']);
        });

        Schema::create('experiment_assignments', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('experiment_key', 100);
            $table->string('user_id', 100);
            $table->string('variant', 50);
            $table->string('session_id', 100)->nullable();
            $table->json('context')->nullable();
            $table->json('conversions')->nullable();
            $table->timestamp('exposed_at')->nullable();
            $table->timestamp('converted_at')->nullable();
            $table->timestamps();

            $table->unique(['experiment_key', 'user_id']);
            $table->index(['experiment_key', 'variant']);
            $table->index('user_id');
        });

        Schema::create('feature_flags', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('key', 100)->unique();
            $table->string('name', 255);
            $table->text('description')->nullable();
            $table->boolean('enabled')->default(false);
            $table->unsignedTinyInteger('rollout_percentage')->default(100);
            $table->json('targeting_rules')->nullable();
            $table->timestamps();

            $table->index('enabled');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('feature_flags');
        Schema::dropIfExists('experiment_assignments');
        Schema::dropIfExists('experiments');
    }
};
