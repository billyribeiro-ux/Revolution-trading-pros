<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * FluentForm Pro Feature Tables Migration
 *
 * Creates tables for:
 * - Payments (Stripe, PayPal, etc.)
 * - Quizzes & Surveys
 * - Draft Submissions (Save & Resume)
 * - Coupons
 * - Quiz Results
 * - Inventory Tracking
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // =====================================================================
        // PAYMENTS TABLE
        // =====================================================================
        Schema::create('form_payments', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->foreignId('submission_id')->nullable()->constrained('form_submissions')->onDelete('set null');
            $table->foreignId('user_id')->nullable()->constrained('users')->onDelete('set null');
            $table->string('payment_provider', 50)->index(); // stripe, paypal, etc.
            $table->string('transaction_id')->nullable()->index();
            $table->string('status', 50)->default('pending')->index();
            $table->decimal('amount', 12, 2);
            $table->string('currency', 3)->default('USD');
            $table->string('payment_method', 50)->nullable(); // card, bank, etc.
            $table->string('customer_email')->nullable();
            $table->string('customer_name')->nullable();
            $table->json('billing_address')->nullable();
            $table->json('line_items')->nullable();
            $table->json('metadata')->nullable();
            $table->string('coupon_code', 50)->nullable();
            $table->decimal('discount_amount', 12, 2)->nullable();
            $table->string('subscription_id')->nullable()->index();
            $table->string('subscription_status', 50)->nullable();
            $table->timestamp('subscription_start')->nullable();
            $table->timestamp('subscription_end')->nullable();
            $table->string('refund_id')->nullable();
            $table->decimal('refund_amount', 12, 2)->nullable();
            $table->text('refund_reason')->nullable();
            $table->timestamp('refunded_at')->nullable();
            $table->text('failure_reason')->nullable();
            $table->timestamps();

            $table->index(['form_id', 'status']);
            $table->index(['customer_email', 'created_at']);
        });

        // =====================================================================
        // QUIZZES TABLE
        // =====================================================================
        Schema::create('form_quizzes', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->string('quiz_type', 50)->default('quiz'); // quiz, survey, assessment
            $table->boolean('enabled')->default(true);
            $table->string('scoring_type', 50)->default('points'); // points, percentage, letter_grade
            $table->decimal('passing_score', 5, 2)->default(70);
            $table->boolean('show_score')->default(true);
            $table->boolean('show_correct_answers')->default(false);
            $table->boolean('randomize_questions')->default(false);
            $table->boolean('randomize_options')->default(false);
            $table->unsignedInteger('time_limit')->nullable(); // seconds
            $table->unsignedSmallInteger('attempts_allowed')->nullable();
            $table->json('grade_settings')->nullable();
            $table->json('result_messages')->nullable();
            $table->boolean('instant_feedback')->default(false);
            $table->boolean('allow_retake')->default(true);
            $table->timestamps();

            $table->unique('form_id');
        });

        // =====================================================================
        // QUIZ RESULTS TABLE
        // =====================================================================
        Schema::create('form_quiz_results', function (Blueprint $table) {
            $table->id();
            $table->foreignId('quiz_id')->constrained('form_quizzes')->onDelete('cascade');
            $table->foreignId('submission_id')->nullable()->constrained('form_submissions')->onDelete('set null');
            $table->foreignId('user_id')->nullable()->constrained('users')->onDelete('set null');
            $table->decimal('percentage', 5, 2);
            $table->unsignedInteger('earned_points');
            $table->unsignedInteger('total_points');
            $table->unsignedSmallInteger('correct_answers');
            $table->unsignedSmallInteger('total_questions');
            $table->string('grade', 10)->nullable();
            $table->boolean('passed')->default(false);
            $table->json('answers')->nullable(); // Detailed answer breakdown
            $table->unsignedInteger('time_taken')->nullable(); // seconds
            $table->unsignedSmallInteger('attempt_number')->default(1);
            $table->string('ip_address', 45)->nullable();
            $table->timestamps();

            $table->index(['quiz_id', 'user_id']);
            $table->index(['quiz_id', 'passed']);
        });

        // =====================================================================
        // DRAFTS TABLE (Save & Resume)
        // =====================================================================
        Schema::create('form_drafts', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained('users')->onDelete('set null');
            $table->string('hash', 64)->unique();
            $table->string('email')->nullable()->index();
            $table->text('form_data'); // Encrypted JSON
            $table->unsignedSmallInteger('current_step')->default(1);
            $table->string('ip_address', 45)->nullable();
            $table->text('user_agent')->nullable();
            $table->timestamp('email_sent_at')->nullable();
            $table->timestamp('resumed_at')->nullable();
            $table->timestamp('completed_at')->nullable();
            $table->foreignId('submission_id')->nullable()->constrained('form_submissions')->onDelete('set null');
            $table->timestamp('expires_at')->nullable()->index();
            $table->timestamps();

            $table->index(['form_id', 'user_id']);
            $table->index(['form_id', 'email']);
        });

        // =====================================================================
        // COUPONS TABLE
        // =====================================================================
        Schema::create('form_coupons', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->nullable()->constrained('forms')->onDelete('cascade');
            $table->string('code', 50)->unique();
            $table->string('discount_type', 50)->default('percentage');
            $table->decimal('discount_value', 10, 2);
            $table->decimal('min_amount', 12, 2)->nullable();
            $table->decimal('max_discount', 12, 2)->nullable();
            $table->unsignedInteger('usage_limit')->nullable();
            $table->unsignedSmallInteger('usage_limit_per_user')->nullable();
            $table->unsignedInteger('usage_count')->default(0);
            $table->boolean('stackable')->default(false);
            $table->timestamp('starts_at')->nullable();
            $table->timestamp('expires_at')->nullable()->index();
            $table->json('conditions')->nullable();
            $table->boolean('active')->default(true)->index();
            $table->timestamps();

            $table->index(['code', 'active']);
        });

        // =====================================================================
        // INVENTORY TABLE
        // =====================================================================
        Schema::create('form_inventory', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->string('field_name'); // Form field this tracks
            $table->string('item_name'); // Option/item identifier
            $table->unsignedInteger('total_stock');
            $table->unsignedInteger('reserved_stock')->default(0);
            $table->unsignedInteger('sold_stock')->default(0);
            $table->boolean('track_inventory')->default(true);
            $table->boolean('allow_backorders')->default(false);
            $table->unsignedSmallInteger('low_stock_threshold')->nullable();
            $table->timestamps();

            $table->unique(['form_id', 'field_name', 'item_name']);
            $table->index(['form_id', 'field_name']);
        });

        // =====================================================================
        // FORM SCHEDULING TABLE
        // =====================================================================
        Schema::create('form_schedules', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->boolean('scheduling_enabled')->default(false);
            $table->timestamp('start_date')->nullable();
            $table->timestamp('end_date')->nullable();
            $table->text('pending_message')->nullable(); // Before start
            $table->text('expired_message')->nullable(); // After end
            $table->unsignedInteger('submission_limit')->nullable();
            $table->text('limit_reached_message')->nullable();
            $table->json('restrict_days')->nullable(); // Days of week
            $table->json('restrict_hours')->nullable(); // Hours of day
            $table->timestamps();

            $table->unique('form_id');
        });

        // =====================================================================
        // ADMIN APPROVAL TABLE
        // =====================================================================
        Schema::create('form_approvals', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->foreignId('submission_id')->constrained('form_submissions')->onDelete('cascade');
            $table->string('status', 50)->default('pending'); // pending, approved, rejected
            $table->foreignId('reviewed_by')->nullable()->constrained('users')->onDelete('set null');
            $table->timestamp('reviewed_at')->nullable();
            $table->text('review_notes')->nullable();
            $table->text('rejection_reason')->nullable();
            $table->boolean('notify_user')->default(true);
            $table->timestamps();

            $table->unique('submission_id');
            $table->index(['form_id', 'status']);
        });

        // =====================================================================
        // DOUBLE OPT-IN TABLE
        // =====================================================================
        Schema::create('form_confirmations', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->foreignId('submission_id')->constrained('form_submissions')->onDelete('cascade');
            $table->string('email');
            $table->string('token', 64)->unique();
            $table->timestamp('sent_at')->nullable();
            $table->timestamp('confirmed_at')->nullable();
            $table->timestamp('expires_at');
            $table->string('ip_address', 45)->nullable();
            $table->timestamps();

            $table->index(['email', 'confirmed_at']);
            $table->index('token');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('form_confirmations');
        Schema::dropIfExists('form_approvals');
        Schema::dropIfExists('form_schedules');
        Schema::dropIfExists('form_inventory');
        Schema::dropIfExists('form_coupons');
        Schema::dropIfExists('form_drafts');
        Schema::dropIfExists('form_quiz_results');
        Schema::dropIfExists('form_quizzes');
        Schema::dropIfExists('form_payments');
    }
};
