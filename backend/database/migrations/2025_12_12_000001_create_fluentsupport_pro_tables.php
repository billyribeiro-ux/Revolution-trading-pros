<?php

/**
 * FluentSupport Pro Tables Migration
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Complete FluentSupport Pro L8 - Help Desk & Customer Support System
 * Part of the unified Fluent Ecosystem
 *
 * Features:
 * - Ticket management with priorities and statuses
 * - Support agents with departments and permissions
 * - Canned responses and macros
 * - SLA policies with escalation rules
 * - Customer satisfaction ratings
 * - Knowledge base integration
 * - Integration with FluentCRM for contact linking
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // ═══════════════════════════════════════════════════════════════════════════
        // SUPPORT DEPARTMENTS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('support_departments', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->string('email')->nullable(); // Department email
            $table->string('color', 7)->default('#6366f1');
            $table->boolean('is_active')->default(true);
            $table->boolean('is_default')->default(false);
            $table->unsignedInteger('sort_order')->default(0);
            $table->json('settings')->nullable(); // Auto-assign, notifications, etc.
            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'sort_order']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // SUPPORT AGENTS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('support_agents', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('title')->nullable(); // Job title
            $table->string('avatar_url')->nullable();
            $table->enum('role', ['admin', 'supervisor', 'agent'])->default('agent');
            $table->boolean('is_active')->default(true);
            $table->boolean('is_available')->default(true); // Online status
            $table->unsignedInteger('max_tickets')->default(20); // Max concurrent tickets
            $table->unsignedInteger('current_tickets')->default(0);
            $table->json('permissions')->nullable();
            $table->json('notification_settings')->nullable();
            $table->timestamp('last_activity_at')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'is_available']);
            $table->index('role');
        });

        // Agent-Department pivot
        Schema::create('agent_departments', function (Blueprint $table) {
            $table->id();
            $table->foreignId('agent_id')->constrained('support_agents')->onDelete('cascade');
            $table->foreignId('department_id')->constrained('support_departments')->onDelete('cascade');
            $table->boolean('is_primary')->default(false);
            $table->timestamps();

            $table->unique(['agent_id', 'department_id']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TICKET CATEGORIES & TAGS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('ticket_categories', function (Blueprint $table) {
            $table->id();
            $table->foreignId('parent_id')->nullable()->constrained('ticket_categories')->nullOnDelete();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->string('icon')->nullable();
            $table->string('color', 7)->default('#6b7280');
            $table->unsignedInteger('sort_order')->default(0);
            $table->boolean('is_active')->default(true);
            $table->timestamps();

            $table->index(['parent_id', 'is_active']);
        });

        Schema::create('ticket_tags', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->string('color', 7)->default('#6b7280');
            $table->text('description')->nullable();
            $table->timestamps();
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // SLA POLICIES
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('sla_policies', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->text('description')->nullable();
            $table->boolean('is_active')->default(true);
            $table->boolean('is_default')->default(false);
            $table->unsignedInteger('priority')->default(0); // Higher = more important

            // Response time targets (in minutes)
            $table->unsignedInteger('first_response_urgent')->default(60); // 1 hour
            $table->unsignedInteger('first_response_high')->default(240); // 4 hours
            $table->unsignedInteger('first_response_medium')->default(480); // 8 hours
            $table->unsignedInteger('first_response_low')->default(1440); // 24 hours

            // Resolution time targets (in minutes)
            $table->unsignedInteger('resolution_urgent')->default(240); // 4 hours
            $table->unsignedInteger('resolution_high')->default(480); // 8 hours
            $table->unsignedInteger('resolution_medium')->default(1440); // 24 hours
            $table->unsignedInteger('resolution_low')->default(4320); // 72 hours

            // Business hours
            $table->boolean('use_business_hours')->default(true);
            $table->json('business_hours')->nullable();
            $table->json('holidays')->nullable();

            // Escalation settings
            $table->json('escalation_rules')->nullable();

            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'priority']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TICKETS - Main Table
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('support_tickets', function (Blueprint $table) {
            $table->id();
            $table->string('ticket_number')->unique(); // e.g., TKT-2025-000001
            $table->string('subject');
            $table->text('description');

            // Customer info
            $table->foreignId('customer_id')->nullable()->constrained('users')->nullOnDelete();
            $table->foreignId('contact_id')->nullable(); // FluentCRM contact link
            $table->string('customer_email');
            $table->string('customer_name');

            // Assignment
            $table->foreignId('department_id')->nullable()->constrained('support_departments')->nullOnDelete();
            $table->foreignId('assigned_agent_id')->nullable()->constrained('support_agents')->nullOnDelete();
            $table->foreignId('category_id')->nullable()->constrained('ticket_categories')->nullOnDelete();

            // Status & Priority
            $table->enum('status', [
                'new', 'open', 'pending', 'waiting_customer', 'waiting_third_party',
                'on_hold', 'resolved', 'closed', 'spam'
            ])->default('new');
            $table->enum('priority', ['urgent', 'high', 'medium', 'low'])->default('medium');

            // SLA tracking
            $table->foreignId('sla_policy_id')->nullable()->constrained('sla_policies')->nullOnDelete();
            $table->timestamp('first_response_due_at')->nullable();
            $table->timestamp('resolution_due_at')->nullable();
            $table->timestamp('first_responded_at')->nullable();
            $table->timestamp('resolved_at')->nullable();
            $table->boolean('sla_breached')->default(false);

            // Source tracking
            $table->enum('source', ['web', 'email', 'api', 'chat', 'phone', 'social'])->default('web');
            $table->string('source_reference')->nullable(); // External reference ID

            // Metadata
            $table->json('custom_fields')->nullable();
            $table->json('metadata')->nullable(); // Browser, IP, etc.
            $table->unsignedInteger('response_count')->default(0);
            $table->unsignedInteger('internal_note_count')->default(0);

            // Customer satisfaction
            $table->unsignedTinyInteger('satisfaction_rating')->nullable(); // 1-5
            $table->text('satisfaction_feedback')->nullable();
            $table->timestamp('satisfaction_rated_at')->nullable();

            // Flags
            $table->boolean('is_starred')->default(false);
            $table->boolean('is_escalated')->default(false);
            $table->boolean('is_locked')->default(false);

            $table->timestamps();
            $table->softDeletes();

            // Indexes for performance
            $table->index(['status', 'priority', 'created_at']);
            $table->index(['department_id', 'status']);
            $table->index(['assigned_agent_id', 'status']);
            $table->index(['customer_id', 'status']);
            $table->index('customer_email');
            $table->index('contact_id');
            $table->index(['sla_breached', 'status']);
            $table->index('first_response_due_at');
            $table->index('resolution_due_at');
        });

        // Ticket-Tag pivot
        Schema::create('ticket_tag', function (Blueprint $table) {
            $table->id();
            $table->foreignId('ticket_id')->constrained('support_tickets')->onDelete('cascade');
            $table->foreignId('tag_id')->constrained('ticket_tags')->onDelete('cascade');
            $table->timestamps();

            $table->unique(['ticket_id', 'tag_id']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TICKET RESPONSES (Thread)
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('ticket_responses', function (Blueprint $table) {
            $table->id();
            $table->foreignId('ticket_id')->constrained('support_tickets')->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained()->nullOnDelete(); // Who sent
            $table->foreignId('agent_id')->nullable()->constrained('support_agents')->nullOnDelete();

            $table->text('content');
            $table->text('content_html')->nullable();
            $table->enum('type', ['reply', 'note', 'system'])->default('reply');
            $table->boolean('is_private')->default(false); // Internal note

            // Email tracking
            $table->string('message_id')->nullable(); // Email Message-ID
            $table->json('email_headers')->nullable();

            // Metadata
            $table->json('metadata')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['ticket_id', 'type', 'created_at']);
            $table->index('message_id');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TICKET ATTACHMENTS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('ticket_attachments', function (Blueprint $table) {
            $table->id();
            $table->foreignId('ticket_id')->constrained('support_tickets')->onDelete('cascade');
            $table->foreignId('response_id')->nullable()->constrained('ticket_responses')->onDelete('cascade');

            $table->string('filename');
            $table->string('original_filename');
            $table->string('mime_type');
            $table->unsignedBigInteger('file_size');
            $table->string('storage_path');
            $table->string('storage_disk')->default('local');

            $table->timestamps();

            $table->index(['ticket_id', 'created_at']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // CANNED RESPONSES (Templates)
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('canned_responses', function (Blueprint $table) {
            $table->id();
            $table->foreignId('agent_id')->nullable()->constrained('support_agents')->nullOnDelete(); // Personal or shared
            $table->foreignId('department_id')->nullable()->constrained('support_departments')->nullOnDelete();

            $table->string('title');
            $table->string('shortcut')->nullable(); // Quick access code
            $table->text('content');
            $table->text('content_html')->nullable();

            $table->boolean('is_shared')->default(true);
            $table->boolean('is_active')->default(true);
            $table->unsignedInteger('usage_count')->default(0);
            $table->unsignedInteger('sort_order')->default(0);

            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'is_shared']);
            $table->index('shortcut');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TICKET ACTIVITY LOG
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('ticket_activities', function (Blueprint $table) {
            $table->id();
            $table->foreignId('ticket_id')->constrained('support_tickets')->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained()->nullOnDelete();
            $table->foreignId('agent_id')->nullable()->constrained('support_agents')->nullOnDelete();

            $table->string('action'); // e.g., 'status_changed', 'assigned', 'priority_changed'
            $table->json('old_value')->nullable();
            $table->json('new_value')->nullable();
            $table->text('description')->nullable();

            $table->timestamp('created_at');

            $table->index(['ticket_id', 'created_at']);
            $table->index('action');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // MACROS (Automated Actions)
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('support_macros', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->text('description')->nullable();
            $table->foreignId('created_by')->nullable()->constrained('support_agents')->nullOnDelete();

            $table->json('actions'); // Array of actions to perform
            $table->boolean('is_active')->default(true);
            $table->boolean('is_shared')->default(true);
            $table->unsignedInteger('usage_count')->default(0);

            $table->timestamps();
            $table->softDeletes();

            $table->index('is_active');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // AUTOMATION RULES
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('support_automations', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->text('description')->nullable();
            $table->enum('trigger', ['ticket_created', 'ticket_updated', 'response_added', 'sla_breached', 'scheduled']);
            $table->json('conditions'); // When to run
            $table->json('actions'); // What to do
            $table->boolean('is_active')->default(true);
            $table->unsignedInteger('priority')->default(0);
            $table->unsignedBigInteger('run_count')->default(0);
            $table->timestamp('last_run_at')->nullable();

            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'trigger', 'priority']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // SUPPORT SETTINGS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('support_settings', function (Blueprint $table) {
            $table->id();
            $table->string('key')->unique();
            $table->text('value')->nullable();
            $table->string('group')->default('general');
            $table->timestamps();

            $table->index('group');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('support_settings');
        Schema::dropIfExists('support_automations');
        Schema::dropIfExists('support_macros');
        Schema::dropIfExists('ticket_activities');
        Schema::dropIfExists('canned_responses');
        Schema::dropIfExists('ticket_attachments');
        Schema::dropIfExists('ticket_responses');
        Schema::dropIfExists('ticket_tag');
        Schema::dropIfExists('support_tickets');
        Schema::dropIfExists('sla_policies');
        Schema::dropIfExists('ticket_tags');
        Schema::dropIfExists('ticket_categories');
        Schema::dropIfExists('agent_departments');
        Schema::dropIfExists('support_agents');
        Schema::dropIfExists('support_departments');
    }
};
