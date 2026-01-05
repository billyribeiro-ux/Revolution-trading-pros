<?php

/**
 * FluentBooking Pro Tables Migration
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Complete FluentBooking Pro L8 - Appointment & Scheduling System
 * Part of the unified Fluent Ecosystem
 *
 * Features:
 * - Calendar management with availability rules
 * - Service types with pricing and duration
 * - Team member assignments and round-robin
 * - Buffer times and booking windows
 * - Payment integration for paid appointments
 * - Video conferencing (Zoom, Google Meet, Teams)
 * - Email/SMS reminders
 * - Integration with FluentCRM for contact syncing
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
        // CALENDARS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_calendars', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade'); // Owner
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->string('timezone')->default('UTC');
            $table->string('color', 7)->default('#6366f1');

            // Availability settings
            $table->json('availability_rules'); // Weekly schedule
            $table->json('date_overrides')->nullable(); // Specific date exceptions
            $table->boolean('is_active')->default(true);

            // Booking settings
            $table->unsignedInteger('min_booking_notice')->default(60); // Minutes before booking
            $table->unsignedInteger('max_booking_advance')->default(60); // Days in advance
            $table->unsignedInteger('slot_duration')->default(30); // Minutes per slot
            $table->unsignedInteger('buffer_before')->default(0); // Buffer before appointment
            $table->unsignedInteger('buffer_after')->default(0); // Buffer after appointment

            // Sync settings
            $table->boolean('sync_google_calendar')->default(false);
            $table->boolean('sync_outlook_calendar')->default(false);
            $table->string('google_calendar_id')->nullable();
            $table->string('outlook_calendar_id')->nullable();

            $table->timestamps();
            $table->softDeletes();

            $table->index(['user_id', 'is_active']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // SERVICE TYPES
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_services', function (Blueprint $table) {
            $table->id();
            $table->foreignId('calendar_id')->constrained('booking_calendars')->onDelete('cascade');
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->string('color', 7)->default('#6366f1');

            // Duration & pricing
            $table->unsignedInteger('duration')->default(30); // Minutes
            $table->enum('pricing_type', ['free', 'fixed', 'variable'])->default('free');
            $table->decimal('price', 10, 2)->default(0);
            $table->string('currency', 3)->default('USD');

            // Booking limits
            $table->unsignedInteger('max_bookings_per_day')->nullable();
            $table->unsignedInteger('max_attendees')->default(1); // For group bookings
            $table->boolean('allow_group_booking')->default(false);

            // Custom settings
            $table->unsignedInteger('buffer_before')->nullable(); // Override calendar buffer
            $table->unsignedInteger('buffer_after')->nullable();
            $table->json('custom_questions')->nullable(); // Intake questions

            // Location
            $table->enum('location_type', ['in_person', 'phone', 'video', 'custom'])->default('video');
            $table->string('location_details')->nullable();
            $table->enum('video_provider', ['zoom', 'google_meet', 'teams', 'custom'])->nullable();

            $table->boolean('is_active')->default(true);
            $table->boolean('requires_confirmation')->default(false);
            $table->boolean('requires_payment')->default(false);
            $table->unsignedInteger('sort_order')->default(0);

            $table->timestamps();
            $table->softDeletes();

            $table->index(['calendar_id', 'is_active']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TEAM MEMBERS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_team_members', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('display_name');
            $table->string('title')->nullable();
            $table->string('avatar_url')->nullable();
            $table->text('bio')->nullable();

            // Availability
            $table->json('availability_rules')->nullable(); // Personal availability
            $table->boolean('is_active')->default(true);
            $table->boolean('is_available')->default(true);

            // Booking settings
            $table->unsignedInteger('max_daily_bookings')->nullable();
            $table->json('services')->nullable(); // Which services they offer

            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'is_available']);
        });

        // Service-Team Member pivot (who can perform which service)
        Schema::create('booking_service_team', function (Blueprint $table) {
            $table->id();
            $table->foreignId('service_id')->constrained('booking_services')->onDelete('cascade');
            $table->foreignId('team_member_id')->constrained('booking_team_members')->onDelete('cascade');
            $table->boolean('is_primary')->default(false);
            $table->unsignedInteger('priority')->default(0); // For round-robin
            $table->timestamps();

            $table->unique(['service_id', 'team_member_id']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // APPOINTMENTS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_appointments', function (Blueprint $table) {
            $table->id();
            $table->string('booking_reference')->unique(); // e.g., APT-2025-000001
            $table->foreignId('service_id')->constrained('booking_services')->onDelete('cascade');
            $table->foreignId('calendar_id')->constrained('booking_calendars')->onDelete('cascade');
            $table->foreignId('team_member_id')->nullable()->constrained('booking_team_members')->nullOnDelete();

            // Customer info
            $table->foreignId('customer_id')->nullable()->constrained('users')->nullOnDelete();
            $table->foreignId('contact_id')->nullable(); // FluentCRM contact link
            $table->string('customer_name');
            $table->string('customer_email');
            $table->string('customer_phone')->nullable();
            $table->string('customer_timezone')->nullable();

            // Timing
            $table->dateTime('starts_at');
            $table->dateTime('ends_at');
            $table->unsignedInteger('duration'); // Minutes

            // Status
            $table->enum('status', [
                'pending', 'confirmed', 'cancelled', 'completed',
                'no_show', 'rescheduled'
            ])->default('pending');
            $table->text('cancellation_reason')->nullable();
            $table->foreignId('cancelled_by')->nullable()->constrained('users')->nullOnDelete();
            $table->timestamp('cancelled_at')->nullable();

            // Location
            $table->enum('location_type', ['in_person', 'phone', 'video', 'custom'])->default('video');
            $table->string('location_details')->nullable();
            $table->string('meeting_url')->nullable(); // Zoom/Meet link
            $table->string('meeting_id')->nullable();
            $table->string('meeting_password')->nullable();

            // Payment
            $table->enum('payment_status', ['not_required', 'pending', 'paid', 'refunded', 'failed'])->default('not_required');
            $table->decimal('amount', 10, 2)->default(0);
            $table->string('currency', 3)->default('USD');
            $table->string('payment_intent_id')->nullable(); // Stripe
            $table->timestamp('paid_at')->nullable();

            // Notes & responses
            $table->text('notes')->nullable();
            $table->text('internal_notes')->nullable();
            $table->json('custom_responses')->nullable(); // Answers to custom questions

            // Reminders
            $table->boolean('reminder_sent_24h')->default(false);
            $table->boolean('reminder_sent_1h')->default(false);

            // Rescheduling
            $table->foreignId('rescheduled_from')->nullable()->constrained('booking_appointments')->nullOnDelete();
            $table->unsignedTinyInteger('reschedule_count')->default(0);

            // External calendar sync
            $table->string('google_event_id')->nullable();
            $table->string('outlook_event_id')->nullable();

            $table->timestamps();
            $table->softDeletes();

            // Indexes
            $table->index(['service_id', 'status', 'starts_at']);
            $table->index(['team_member_id', 'status', 'starts_at']);
            $table->index(['customer_email', 'status']);
            $table->index('contact_id');
            $table->index('starts_at');
            $table->index('status');
        });

        // Group booking attendees (for appointments with max_attendees > 1)
        Schema::create('booking_attendees', function (Blueprint $table) {
            $table->id();
            $table->foreignId('appointment_id')->constrained('booking_appointments')->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained()->nullOnDelete();
            $table->foreignId('contact_id')->nullable(); // FluentCRM contact

            $table->string('name');
            $table->string('email');
            $table->string('phone')->nullable();
            $table->enum('status', ['confirmed', 'cancelled', 'no_show'])->default('confirmed');

            $table->timestamps();

            $table->index(['appointment_id', 'status']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // TIME BLOCKS (Busy/unavailable periods)
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_time_blocks', function (Blueprint $table) {
            $table->id();
            $table->foreignId('calendar_id')->nullable()->constrained('booking_calendars')->onDelete('cascade');
            $table->foreignId('team_member_id')->nullable()->constrained('booking_team_members')->onDelete('cascade');

            $table->string('title')->nullable();
            $table->dateTime('starts_at');
            $table->dateTime('ends_at');
            $table->boolean('is_recurring')->default(false);
            $table->json('recurrence_rules')->nullable();

            $table->timestamps();

            $table->index(['calendar_id', 'starts_at', 'ends_at']);
            $table->index(['team_member_id', 'starts_at', 'ends_at']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // BOOKING PAGES (Public booking links)
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_pages', function (Blueprint $table) {
            $table->id();
            $table->foreignId('calendar_id')->constrained('booking_calendars')->onDelete('cascade');
            $table->string('slug')->unique();
            $table->string('title');
            $table->text('description')->nullable();
            $table->string('logo_url')->nullable();
            $table->string('cover_image_url')->nullable();

            // Design
            $table->string('primary_color', 7)->default('#6366f1');
            $table->string('background_color', 7)->default('#ffffff');
            $table->json('custom_css')->nullable();

            // Content
            $table->text('welcome_message')->nullable();
            $table->text('confirmation_message')->nullable();

            // Settings
            $table->boolean('show_team_member_selection')->default(true);
            $table->boolean('require_account')->default(false);
            $table->boolean('is_active')->default(true);

            // SEO
            $table->string('meta_title')->nullable();
            $table->text('meta_description')->nullable();

            $table->timestamps();
            $table->softDeletes();

            $table->index('is_active');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // REMINDER TEMPLATES
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_reminder_templates', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->enum('type', ['email', 'sms'])->default('email');
            $table->enum('trigger', ['confirmation', 'reminder_24h', 'reminder_1h', 'followup', 'cancellation', 'reschedule']);

            $table->string('subject')->nullable(); // For email
            $table->text('body');
            $table->boolean('is_active')->default(true);
            $table->boolean('is_default')->default(false);

            $table->timestamps();

            $table->index(['type', 'trigger', 'is_active']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // INTEGRATIONS (Video conferencing providers)
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_integrations', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('provider'); // zoom, google_meet, microsoft_teams
            $table->json('credentials'); // Encrypted OAuth tokens
            $table->boolean('is_active')->default(true);
            $table->timestamp('token_expires_at')->nullable();
            $table->timestamp('last_synced_at')->nullable();

            $table->timestamps();

            $table->unique(['user_id', 'provider']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // BOOKING ANALYTICS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_analytics', function (Blueprint $table) {
            $table->id();
            $table->foreignId('service_id')->nullable()->constrained('booking_services')->onDelete('cascade');
            $table->foreignId('team_member_id')->nullable()->constrained('booking_team_members')->onDelete('cascade');
            $table->date('date');

            $table->unsignedInteger('bookings_count')->default(0);
            $table->unsignedInteger('cancellations_count')->default(0);
            $table->unsignedInteger('no_shows_count')->default(0);
            $table->unsignedInteger('completed_count')->default(0);
            $table->decimal('revenue', 12, 2)->default(0);
            $table->unsignedInteger('avg_duration')->default(0);

            $table->timestamps();

            $table->unique(['service_id', 'team_member_id', 'date']);
            $table->index('date');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // BOOKING SETTINGS
        // ═══════════════════════════════════════════════════════════════════════════
        Schema::create('booking_settings', function (Blueprint $table) {
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
        Schema::dropIfExists('booking_settings');
        Schema::dropIfExists('booking_analytics');
        Schema::dropIfExists('booking_integrations');
        Schema::dropIfExists('booking_reminder_templates');
        Schema::dropIfExists('booking_pages');
        Schema::dropIfExists('booking_time_blocks');
        Schema::dropIfExists('booking_attendees');
        Schema::dropIfExists('booking_appointments');
        Schema::dropIfExists('booking_service_team');
        Schema::dropIfExists('booking_team_members');
        Schema::dropIfExists('booking_services');
        Schema::dropIfExists('booking_calendars');
    }
};
