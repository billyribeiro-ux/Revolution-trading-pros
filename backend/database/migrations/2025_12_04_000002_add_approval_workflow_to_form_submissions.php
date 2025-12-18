<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * FluentForms 6.1.8 (December 2025) - Admin Approval Workflow
 *
 * Adds approval workflow columns to form_submissions table
 * and creates the approval_logs table for audit trail.
 */
return new class extends Migration
{
    public function up(): void
    {
        // Add approval columns to form_submissions
        Schema::table('form_submissions', function (Blueprint $table) {
            $table->string('approval_status', 30)->default('none')->after('status');
            $table->timestamp('approval_required_at')->nullable()->after('approval_status');
            $table->timestamp('approved_at')->nullable()->after('approval_required_at');
            $table->unsignedBigInteger('approved_by')->nullable()->after('approved_at');
            $table->text('approval_note')->nullable()->after('approved_by');

            $table->index('approval_status');
            $table->foreign('approved_by')->references('id')->on('users')->nullOnDelete();
        });

        // Create approval logs table
        Schema::create('form_submission_approval_logs', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('submission_id');
            $table->unsignedBigInteger('admin_id');
            $table->string('status', 30);
            $table->string('previous_status', 30)->nullable();
            $table->text('note')->nullable();
            $table->string('admin_name')->nullable();
            $table->string('admin_email')->nullable();
            $table->ipAddress('ip_address')->nullable();
            $table->timestamps();

            $table->foreign('submission_id')
                ->references('id')
                ->on('form_submissions')
                ->cascadeOnDelete();

            $table->foreign('admin_id')
                ->references('id')
                ->on('users')
                ->cascadeOnDelete();

            $table->index(['submission_id', 'created_at']);
            $table->index('admin_id');
        });

        // Create form approval settings table
        Schema::create('form_approval_settings', function (Blueprint $table) {
            $table->id();
            $table->unsignedBigInteger('form_id')->unique();
            $table->boolean('approval_required')->default(false);
            $table->json('approver_ids')->nullable(); // Array of user IDs who can approve
            $table->json('approval_rules')->nullable(); // Conditional approval rules
            $table->boolean('notify_approvers')->default(true);
            $table->boolean('notify_submitter')->default(true);
            $table->string('approval_email_template')->nullable();
            $table->string('rejection_email_template')->nullable();
            $table->string('revision_email_template')->nullable();
            $table->integer('auto_approve_after_hours')->nullable(); // Auto-approve if no action
            $table->timestamps();

            $table->foreign('form_id')
                ->references('id')
                ->on('forms')
                ->cascadeOnDelete();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('form_approval_settings');
        Schema::dropIfExists('form_submission_approval_logs');

        Schema::table('form_submissions', function (Blueprint $table) {
            $table->dropForeign(['approved_by']);
            $table->dropIndex(['approval_status']);
            $table->dropColumn([
                'approval_status',
                'approval_required_at',
                'approved_at',
                'approved_by',
                'approval_note',
            ]);
        });
    }
};
