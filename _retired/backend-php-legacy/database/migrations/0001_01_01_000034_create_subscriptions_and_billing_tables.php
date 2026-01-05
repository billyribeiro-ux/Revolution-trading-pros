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
        // Subscription Plans Table
        Schema::create('subscription_plans', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->decimal('price', 10, 2);
            $table->enum('billing_period', ['daily', 'weekly', 'monthly', 'quarterly', 'yearly'])->default('monthly');
            $table->integer('billing_interval')->default(1); // e.g., every 2 months
            $table->integer('trial_days')->default(0);
            $table->integer('signup_fee')->default(0);
            $table->integer('max_users')->nullable(); // null = unlimited
            $table->json('features')->nullable(); // JSON array of features
            $table->boolean('is_active')->default(true);
            $table->boolean('is_featured')->default(false);
            $table->integer('sort_order')->default(0);
            $table->timestamps();
            $table->softDeletes();
        });

        // User Subscriptions Table
        Schema::create('user_subscriptions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->foreignId('subscription_plan_id')->constrained()->onDelete('cascade');
            $table->enum('status', ['active', 'paused', 'on-hold', 'cancelled', 'expired', 'pending', 'pending-cancel', 'trial'])->default('pending');
            $table->string('interval')->nullable(); // monthly, yearly, etc.
            $table->string('currency')->default('USD');
            $table->decimal('total_paid', 10, 2)->default(0);
            $table->integer('failed_payments')->default(0);
            $table->integer('successful_payments')->default(0);
            $table->integer('renewal_count')->default(0);
            $table->boolean('auto_renew')->default(true);
            $table->timestamp('start_date')->nullable();
            $table->timestamp('next_payment_date')->nullable();
            $table->timestamp('trial_ends_at')->nullable();
            $table->timestamp('current_period_start')->nullable();
            $table->timestamp('current_period_end')->nullable();
            $table->timestamp('cancelled_at')->nullable();
            $table->timestamp('paused_at')->nullable();
            $table->timestamp('expires_at')->nullable();
            $table->string('payment_method')->nullable(); // stripe, paypal, manual, etc.
            $table->string('payment_id')->nullable(); // External payment ID
            $table->decimal('amount_paid', 10, 2)->default(0);
            $table->integer('billing_cycles_completed')->default(0);
            $table->json('metadata')->nullable(); // Additional data
            $table->text('notes')->nullable(); // Admin notes
            $table->timestamps();
            $table->softDeletes();

            $table->index(['user_id', 'status']);
            $table->index('status');
        });

        // Subscription Payments Table
        Schema::create('subscription_payments', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_subscription_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->decimal('amount', 10, 2);
            $table->enum('status', ['pending', 'completed', 'failed', 'refunded'])->default('pending');
            $table->string('payment_method')->nullable();
            $table->string('transaction_id')->nullable();
            $table->timestamp('paid_at')->nullable();
            $table->text('failure_reason')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamps();

            $table->index(['user_subscription_id', 'status']);
        });

        // Subscription Features Table (for plan features)
        Schema::create('subscription_features', function (Blueprint $table) {
            $table->id();
            $table->foreignId('subscription_plan_id')->constrained()->onDelete('cascade');
            $table->string('name');
            $table->string('value')->nullable(); // e.g., "unlimited", "10", "yes"
            $table->string('type')->default('boolean'); // boolean, number, text
            $table->integer('sort_order')->default(0);
            $table->timestamps();
        });

        // Subscription Usage Tracking (optional - for metered billing)
        Schema::create('subscription_usage', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_subscription_id')->constrained()->onDelete('cascade');
            $table->string('metric'); // e.g., 'api_calls', 'storage_gb'
            $table->integer('quantity')->default(0);
            $table->timestamp('recorded_at');
            $table->timestamps();

            $table->index(['user_subscription_id', 'metric', 'recorded_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('subscription_usage');
        Schema::dropIfExists('subscription_features');
        Schema::dropIfExists('subscription_payments');
        Schema::dropIfExists('user_subscriptions');
        Schema::dropIfExists('subscription_plans');
    }
};
