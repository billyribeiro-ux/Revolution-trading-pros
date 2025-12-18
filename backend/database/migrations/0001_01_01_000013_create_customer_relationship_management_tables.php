<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        // Contacts
        Schema::create('contacts', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('user_id')->nullable();
            $table->string('email')->unique();
            $table->string('first_name', 100);
            $table->string('last_name', 100);
            $table->string('phone', 50)->nullable();
            $table->string('mobile', 50)->nullable();
            
            // Company
            $table->uuid('company_id')->nullable();
            $table->string('job_title', 100)->nullable();
            $table->string('department', 100)->nullable();
            
            // Address
            $table->string('address_line1')->nullable();
            $table->string('address_line2')->nullable();
            $table->string('city', 100)->nullable();
            $table->string('state', 100)->nullable();
            $table->string('postal_code', 20)->nullable();
            $table->string('country', 100)->nullable();
            $table->string('timezone', 50)->nullable();
            
            // Social
            $table->string('website')->nullable();
            $table->string('linkedin_url')->nullable();
            $table->string('twitter_handle', 100)->nullable();
            
            // CRM
            $table->enum('source', ['website', 'form', 'import', 'api', 'manual', 'referral', 'event'])->default('website');
            $table->string('source_details')->nullable();
            $table->uuid('owner_id')->nullable();
            $table->enum('status', ['lead', 'prospect', 'customer', 'churned', 'unqualified'])->default('lead');
            $table->enum('lifecycle_stage', ['subscriber', 'lead', 'mql', 'sql', 'opportunity', 'customer', 'evangelist'])->default('lead');
            
            // Scoring
            $table->integer('lead_score')->default(0);
            $table->integer('health_score')->default(0);
            $table->integer('engagement_score')->default(0);
            $table->decimal('value_score', 10, 2)->default(0);
            
            // Flags
            $table->boolean('is_verified')->default(false);
            $table->boolean('is_unsubscribed')->default(false);
            $table->boolean('do_not_contact')->default(false);
            $table->boolean('is_vip')->default(false);
            
            // Enrichment
            $table->json('custom_fields')->nullable();
            $table->json('tags')->nullable();
            
            // Attribution
            $table->string('first_touch_channel', 100)->nullable();
            $table->string('first_touch_campaign', 100)->nullable();
            $table->string('last_touch_channel', 100)->nullable();
            $table->string('last_touch_campaign', 100)->nullable();
            
            // Behavior Integration
            $table->integer('total_sessions')->default(0);
            $table->timestamp('last_session_at')->nullable();
            $table->decimal('avg_engagement_score', 5, 2)->default(0);
            $table->decimal('avg_intent_score', 5, 2)->default(0);
            $table->integer('friction_events_count')->default(0);
            
            // Email Integration
            $table->integer('email_opens')->default(0);
            $table->integer('email_clicks')->default(0);
            $table->timestamp('last_email_opened_at')->nullable();
            $table->timestamp('last_email_clicked_at')->nullable();
            
            // Subscription Integration
            $table->enum('subscription_status', ['none', 'trial', 'active', 'paused', 'cancelled', 'expired'])->default('none');
            $table->uuid('subscription_plan_id')->nullable();
            $table->decimal('subscription_mrr', 10, 2)->default(0);
            $table->decimal('lifetime_value', 10, 2)->default(0);
            
            // Activity Metrics
            $table->timestamp('last_activity_at')->nullable();
            $table->timestamp('last_contacted_at')->nullable();
            $table->timestamp('next_followup_at')->nullable();
            $table->integer('activities_count')->default(0);
            $table->integer('notes_count')->default(0);
            $table->integer('tasks_count')->default(0);
            $table->integer('deals_count')->default(0);
            
            $table->timestamps();
            $table->timestamp('last_seen_at')->nullable();
            $table->timestamp('converted_at')->nullable();
            
            $table->index('email');
            $table->index('user_id');
            $table->index('owner_id');
            $table->index('status');
            $table->index('lifecycle_stage');
            $table->index('lead_score');
            $table->index('health_score');
            $table->index('last_activity_at');
            $table->index('created_at');
        });

        // Pipelines
        Schema::create('pipelines', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('name');
            $table->text('description')->nullable();
            $table->boolean('is_default')->default(false);
            $table->boolean('is_active')->default(true);
            $table->integer('deals_count')->default(0);
            $table->decimal('total_value', 12, 2)->default(0);
            $table->decimal('win_rate', 5, 2)->default(0);
            $table->decimal('avg_deal_size', 12, 2)->default(0);
            $table->integer('avg_sales_cycle')->default(0);
            $table->string('color', 7)->default('#3B82F6');
            $table->string('icon', 50)->nullable();
            $table->integer('position')->default(0);
            $table->timestamps();
            
            $table->index('is_default');
            $table->index('is_active');
        });

        // Stages
        Schema::create('stages', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('pipeline_id');
            $table->string('name');
            $table->text('description')->nullable();
            $table->integer('position')->default(0);
            $table->integer('probability')->default(0);
            $table->boolean('is_closed_won')->default(false);
            $table->boolean('is_closed_lost')->default(false);
            $table->integer('auto_advance_after_days')->nullable();
            $table->json('required_activities')->nullable();
            $table->integer('deals_count')->default(0);
            $table->decimal('total_value', 12, 2)->default(0);
            $table->integer('avg_time_in_stage')->default(0);
            $table->decimal('conversion_rate', 5, 2)->default(0);
            $table->string('color', 7)->default('#10B981');
            $table->timestamps();
            
            $table->foreign('pipeline_id')->references('id')->on('pipelines')->onDelete('cascade');
            $table->index('pipeline_id');
            $table->index('position');
        });

        // Deals
        Schema::create('deals', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('name');
            $table->uuid('contact_id');
            $table->uuid('company_id')->nullable();
            $table->uuid('pipeline_id');
            $table->uuid('stage_id');
            $table->decimal('amount', 12, 2);
            $table->string('currency', 3)->default('USD');
            $table->integer('probability')->default(0);
            $table->uuid('owner_id');
            $table->enum('status', ['open', 'won', 'lost', 'abandoned'])->default('open');
            $table->date('close_date')->nullable();
            $table->date('expected_close_date');
            $table->string('lost_reason')->nullable();
            $table->text('won_details')->nullable();
            $table->integer('stage_changes_count')->default(0);
            $table->json('custom_fields')->nullable();
            $table->json('tags')->nullable();
            $table->enum('priority', ['low', 'normal', 'high', 'urgent'])->default('normal');
            $table->string('source_channel', 100)->nullable();
            $table->string('source_campaign', 100)->nullable();
            $table->timestamps();
            $table->timestamp('stage_entered_at')->useCurrent();
            $table->timestamp('closed_at')->nullable();
            
            $table->foreign('contact_id')->references('id')->on('contacts')->onDelete('cascade');
            $table->foreign('pipeline_id')->references('id')->on('pipelines')->onDelete('restrict');
            $table->foreign('stage_id')->references('id')->on('stages')->onDelete('restrict');
            $table->index('contact_id');
            $table->index('pipeline_id');
            $table->index('stage_id');
            $table->index('owner_id');
            $table->index('status');
            $table->index('expected_close_date');
        });

        // Activities
        Schema::create('crm_activities', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('subject_type');
            $table->uuid('subject_id');
            $table->enum('type', [
                'note', 'email_sent', 'email_opened', 'email_clicked',
                'call', 'meeting', 'task_completed',
                'form_submitted', 'popup_converted',
                'deal_created', 'deal_stage_changed', 'deal_won', 'deal_lost',
                'subscription_started', 'subscription_cancelled',
                'behavior_event', 'page_view', 'friction_detected',
                'attribution_touchpoint',
                'status_changed', 'owner_changed',
                'custom'
            ]);
            $table->string('title')->nullable();
            $table->text('description')->nullable();
            $table->json('metadata')->nullable();
            $table->uuid('created_by_id')->nullable();
            $table->uuid('assigned_to_id')->nullable();
            $table->timestamp('due_date')->nullable();
            $table->timestamp('completed_at')->nullable();
            $table->enum('priority', ['low', 'normal', 'high', 'urgent'])->nullable();
            $table->uuid('email_id')->nullable();
            $table->uuid('form_submission_id')->nullable();
            $table->uuid('popup_id')->nullable();
            $table->uuid('behavior_session_id')->nullable();
            $table->uuid('subscription_id')->nullable();
            $table->timestamp('occurred_at');
            $table->timestamps();
            
            $table->index(['subject_type', 'subject_id']);
            $table->index('type');
            $table->index('occurred_at');
            $table->index('created_by_id');
        });

        // Notes
        Schema::create('crm_notes', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('contact_id')->nullable();
            $table->uuid('deal_id')->nullable();
            $table->uuid('company_id')->nullable();
            $table->text('content');
            $table->boolean('is_pinned')->default(false);
            $table->uuid('created_by_id');
            $table->timestamps();
            
            $table->foreign('contact_id')->references('id')->on('contacts')->onDelete('cascade');
            $table->foreign('deal_id')->references('id')->on('deals')->onDelete('cascade');
            $table->index('contact_id');
            $table->index('deal_id');
            $table->index('created_by_id');
        });

        // Contact Segments
        Schema::create('contact_segments', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->string('name');
            $table->text('description')->nullable();
            $table->json('conditions');
            $table->boolean('is_dynamic')->default(true);
            $table->integer('contacts_count')->default(0);
            $table->timestamp('last_calculated_at')->nullable();
            $table->uuid('created_by_id');
            $table->boolean('is_shared')->default(false);
            $table->timestamps();
            
            $table->index('created_by_id');
            $table->index('is_dynamic');
        });

        // Contact Segment Members (for static segments)
        Schema::create('contact_segment_members', function (Blueprint $table) {
            $table->uuid('segment_id');
            $table->uuid('contact_id');
            $table->timestamp('added_at')->useCurrent();
            
            $table->foreign('segment_id')->references('id')->on('contact_segments')->onDelete('cascade');
            $table->foreign('contact_id')->references('id')->on('contacts')->onDelete('cascade');
            $table->primary(['segment_id', 'contact_id']);
        });

        // Lead Score Logs
        Schema::create('lead_score_logs', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('contact_id');
            $table->integer('previous_score');
            $table->integer('new_score');
            $table->integer('change');
            $table->string('reason');
            $table->json('contributing_factors')->nullable();
            $table->timestamp('created_at')->useCurrent();
            
            $table->foreign('contact_id')->references('id')->on('contacts')->onDelete('cascade');
            $table->index('contact_id');
            $table->index('created_at');
        });

        // Deal Stage History
        Schema::create('deal_stage_history', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->uuid('deal_id');
            $table->uuid('from_stage_id')->nullable();
            $table->uuid('to_stage_id');
            $table->integer('time_in_previous_stage')->nullable();
            $table->uuid('changed_by_id')->nullable();
            $table->string('reason')->nullable();
            $table->timestamp('created_at')->useCurrent();
            
            $table->foreign('deal_id')->references('id')->on('deals')->onDelete('cascade');
            $table->index('deal_id');
            $table->index('created_at');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('deal_stage_history');
        Schema::dropIfExists('lead_score_logs');
        Schema::dropIfExists('contact_segment_members');
        Schema::dropIfExists('contact_segments');
        Schema::dropIfExists('crm_notes');
        Schema::dropIfExists('crm_activities');
        Schema::dropIfExists('deals');
        Schema::dropIfExists('stages');
        Schema::dropIfExists('pipelines');
        Schema::dropIfExists('contacts');
    }
};
