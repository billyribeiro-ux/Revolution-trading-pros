<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // Dunning attempts table (skip if exists)
        if (!Schema::hasTable('dunning_attempts')) {
            Schema::create('dunning_attempts', function (Blueprint $table) {
                $table->id();
                $table->foreignId('user_id')->constrained()->cascadeOnDelete();
                $table->foreignId('subscription_id')->nullable()->constrained()->nullOnDelete();
                $table->foreignId('invoice_id')->nullable()->constrained()->nullOnDelete();
                $table->integer('attempt_number')->default(1);
                $table->string('status')->default('pending'); // pending, succeeded, failed, cancelled
                $table->timestamp('scheduled_at')->nullable();
                $table->timestamp('processed_at')->nullable();
                $table->timestamp('reminder_sent_at')->nullable();
                $table->text('error_message')->nullable();
                $table->timestamps();

                $table->index(['subscription_id', 'status']);
                $table->index(['scheduled_at', 'status']);
            });
        }

        // Usage records table (skip if exists)
        if (!Schema::hasTable('usage_records')) {
            Schema::create('usage_records', function (Blueprint $table) {
                $table->id();
                $table->foreignId('user_id')->constrained()->cascadeOnDelete();
                $table->foreignId('subscription_id')->nullable()->constrained()->nullOnDelete();
                $table->string('feature', 100);
                $table->integer('quantity')->default(1);
                $table->json('metadata')->nullable();
                $table->timestamp('recorded_at');
                $table->timestamps();

                $table->index(['user_id', 'feature', 'recorded_at']);
                $table->index(['subscription_id', 'feature']);
            });
        }

        // Add payment provider columns to users if not exists
        if (!Schema::hasColumn('users', 'stripe_customer_id')) {
            Schema::table('users', function (Blueprint $table) {
                $table->string('stripe_customer_id')->nullable()->after('email');
                $table->string('paypal_customer_id')->nullable()->after('stripe_customer_id');
                $table->string('paddle_customer_id')->nullable()->after('paypal_customer_id');
                $table->string('tax_id')->nullable();
                $table->string('billing_address')->nullable();
                $table->string('billing_city')->nullable();
                $table->string('billing_state')->nullable();
                $table->string('billing_zip')->nullable();
                $table->string('billing_country', 2)->nullable();
            });
        }

        // Add columns to subscriptions if table exists and column not exists
        if (Schema::hasTable('subscriptions') && !Schema::hasColumn('subscriptions', 'stripe_subscription_id')) {
            Schema::table('subscriptions', function (Blueprint $table) {
                $table->string('stripe_subscription_id')->nullable()->after('id');
                $table->string('paypal_subscription_id')->nullable();
                $table->string('paddle_subscription_id')->nullable();
                $table->integer('payment_attempts')->default(0);
                $table->timestamp('suspended_at')->nullable();
                $table->timestamp('grace_period_ends_at')->nullable();
                $table->timestamp('trial_reminder_sent_at')->nullable();
            });
        }

        // Add columns to invoices if table exists and column not exists
        if (Schema::hasTable('invoices') && !Schema::hasColumn('invoices', 'stripe_invoice_id')) {
            Schema::table('invoices', function (Blueprint $table) {
                $table->string('stripe_invoice_id')->nullable()->after('id');
                $table->string('invoice_number')->nullable();
                $table->integer('subtotal')->nullable();
                $table->integer('tax_amount')->nullable();
                $table->decimal('tax_rate', 5, 2)->nullable();
                $table->string('tax_type', 20)->nullable();
                $table->string('tax_description')->nullable();
                $table->boolean('reverse_charge')->default(false);
                $table->integer('discount_amount')->nullable();
                $table->string('billing_reason')->nullable();
                $table->integer('attempt_count')->default(0);
                $table->timestamp('next_payment_attempt')->nullable();
                $table->string('invoice_pdf')->nullable();
                $table->string('hosted_invoice_url')->nullable();
                $table->string('pdf_path')->nullable();
                $table->json('line_items')->nullable();
                $table->string('card_last_four', 4)->nullable();
                $table->string('card_brand', 20)->nullable();
            });
        }

        // Add columns to payments if table exists and column not exists
        if (Schema::hasTable('payments') && !Schema::hasColumn('payments', 'stripe_payment_intent_id')) {
            Schema::table('payments', function (Blueprint $table) {
                $table->string('stripe_payment_intent_id')->nullable()->after('id');
                $table->string('payment_method', 50)->nullable();
                $table->integer('refunded_amount')->nullable();
                $table->string('failure_reason')->nullable();
            });
        }

        // Add columns to plans if table exists and column not exists
        if (Schema::hasTable('plans') && !Schema::hasColumn('plans', 'stripe_price_id')) {
            Schema::table('plans', function (Blueprint $table) {
                $table->string('stripe_price_id')->nullable();
                $table->string('stripe_yearly_price_id')->nullable();
                $table->string('paddle_price_id')->nullable();
                $table->string('paypal_plan_id')->nullable();
                $table->integer('yearly_price')->nullable();
                $table->integer('sort_order')->default(0);
                $table->boolean('is_featured')->default(false);
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('usage_records');
        Schema::dropIfExists('dunning_attempts');

        // Note: Not dropping added columns as that could cause data loss
    }
};
