<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('membership_plans', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->decimal('price', 10, 2);
            $table->enum('billing_cycle', ['monthly', 'quarterly', 'annual']);
            $table->boolean('is_active')->default(true);
            $table->json('metadata')->nullable();
            $table->timestamps();
        });

        Schema::create('membership_features', function (Blueprint $table) {
            $table->id();
            $table->foreignId('plan_id')->constrained('membership_plans')->onDelete('cascade');
            $table->string('feature_code');
            $table->string('feature_name');
            $table->text('description')->nullable();
            $table->timestamps();

            $table->unique(['plan_id', 'feature_code']);
        });

        Schema::create('user_memberships', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->foreignId('plan_id')->constrained('membership_plans')->onDelete('cascade');
            $table->timestamp('starts_at');
            $table->timestamp('expires_at')->nullable();
            $table->enum('status', ['active', 'cancelled', 'expired', 'pending'])->default('pending');
            $table->string('payment_provider')->nullable();
            $table->string('subscription_id')->nullable();
            $table->timestamps();

            $table->index(['user_id', 'status']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('user_memberships');
        Schema::dropIfExists('membership_features');
        Schema::dropIfExists('membership_plans');
    }
};
