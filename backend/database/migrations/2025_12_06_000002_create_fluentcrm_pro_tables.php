<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * FluentCRM Pro Complete Feature Migration
 *
 * Implements all FluentCRM Pro WordPress plugin features for Laravel 12:
 * - Email Sequences (drip campaigns)
 * - Recurring Campaigns (scheduled newsletters)
 * - Smart Links (trackable action links)
 * - Dynamic Segments (advanced filtering)
 * - Commerce Reports (revenue tracking)
 * - CRM Managers (team permissions)
 * - Funnel Automation (triggers, actions, conditions)
 *
 * @version 2.9.86 compatible
 * @since December 2025
 */
return new class extends Migration
{
    public function up(): void
    {
        // =====================================================
        // EMAIL SEQUENCES (Drip Campaigns)
        // =====================================================
        Schema::create('email_sequences', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('title');
            $table->string('slug')->unique();
            $table->enum('status', ['draft', 'active', 'paused', 'completed'])->default('draft');
            $table->text('description')->nullable();
            $table->string('design_template', 50)->default('simple');
            $table->json('settings')->nullable(); // mailer_settings, timing, etc.
            $table->json('subscriber_settings')->nullable(); // targeting options
            $table->integer('emails_count')->default(0);
            $table->integer('subscribers_count')->default(0);
            $table->integer('total_sent')->default(0);
            $table->integer('total_opened')->default(0);
            $table->integer('total_clicked')->default(0);
            $table->decimal('total_revenue', 12, 2)->default(0);
            $table->string('currency', 3)->default('USD');
            $table->uuid('created_by')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index('status');
            $table->index('created_by');
            $table->index('created_at');
        });

        // Sequence Emails (individual emails in a sequence)
        Schema::create('sequence_mails', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('sequence_id');
            $table->string('title');
            $table->string('email_subject');
            $table->string('email_pre_header')->nullable();
            $table->longText('email_body');
            $table->integer('delay')->default(0); // delay in seconds from sequence start
            $table->enum('delay_unit', ['minutes', 'hours', 'days', 'weeks'])->default('days');
            $table->integer('delay_value')->default(1);
            $table->enum('status', ['draft', 'active', 'paused'])->default('draft');
            $table->json('settings')->nullable(); // mailer_settings, timing restrictions
            $table->integer('position')->default(0);
            $table->integer('sent_count')->default(0);
            $table->integer('open_count')->default(0);
            $table->integer('click_count')->default(0);
            $table->integer('unsubscribe_count')->default(0);
            $table->decimal('revenue', 12, 2)->default(0);
            $table->uuid('created_by')->nullable();
            $table->timestamps();

            $table->foreign('sequence_id')->references('id')->on('email_sequences')->cascadeOnDelete();
            $table->index(['sequence_id', 'position']);
            $table->index('status');
        });

        // Sequence Trackers (subscriber progress through sequences)
        Schema::create('sequence_trackers', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('contact_id');
            $table->uuid('sequence_id');
            $table->uuid('last_sequence_mail_id')->nullable();
            $table->uuid('next_sequence_mail_id')->nullable();
            $table->enum('status', ['active', 'paused', 'completed', 'cancelled', 'failed'])->default('active');
            $table->timestamp('started_at');
            $table->timestamp('last_executed_at')->nullable();
            $table->timestamp('next_execution_at')->nullable();
            $table->timestamp('completed_at')->nullable();
            $table->json('notes')->nullable(); // execution log
            $table->integer('emails_sent')->default(0);
            $table->integer('emails_opened')->default(0);
            $table->integer('emails_clicked')->default(0);
            $table->timestamps();

            $table->foreign('contact_id')->references('id')->on('contacts')->cascadeOnDelete();
            $table->foreign('sequence_id')->references('id')->on('email_sequences')->cascadeOnDelete();
            $table->unique(['contact_id', 'sequence_id']);
            $table->index('status');
            $table->index('next_execution_at');
        });

        // =====================================================
        // RECURRING CAMPAIGNS (Scheduled Newsletters)
        // =====================================================
        Schema::create('recurring_campaigns', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('title');
            $table->string('slug')->unique();
            $table->enum('status', ['draft', 'active', 'paused'])->default('draft');
            $table->string('email_subject')->nullable();
            $table->string('email_pre_header')->nullable();
            $table->longText('email_body')->nullable();
            $table->string('design_template', 50)->default('simple');
            $table->json('settings')->nullable(); // scheduling, mailer, subscriber settings
            $table->json('scheduling_settings')->nullable(); // type, day, time, auto-send
            $table->json('subscriber_settings')->nullable(); // targeting
            $table->json('template_config')->nullable();
            $table->json('labels')->nullable(); // campaign labels
            $table->integer('total_campaigns_sent')->default(0);
            $table->integer('total_emails_sent')->default(0);
            $table->decimal('total_revenue', 12, 2)->default(0);
            $table->timestamp('last_sent_at')->nullable();
            $table->timestamp('next_scheduled_at')->nullable();
            $table->uuid('created_by')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index('status');
            $table->index('next_scheduled_at');
        });

        // Recurring Campaign Emails (individual sends)
        Schema::create('recurring_mails', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('recurring_campaign_id');
            $table->string('email_subject');
            $table->string('email_pre_header')->nullable();
            $table->longText('email_body');
            $table->enum('status', ['draft', 'scheduled', 'sending', 'sent', 'failed'])->default('draft');
            $table->integer('recipients_count')->default(0);
            $table->integer('sent_count')->default(0);
            $table->integer('failed_count')->default(0);
            $table->integer('open_count')->default(0);
            $table->integer('click_count')->default(0);
            $table->integer('unsubscribe_count')->default(0);
            $table->decimal('revenue', 12, 2)->default(0);
            $table->timestamp('scheduled_at')->nullable();
            $table->timestamp('sent_at')->nullable();
            $table->timestamps();

            $table->foreign('recurring_campaign_id')->references('id')->on('recurring_campaigns')->cascadeOnDelete();
            $table->index('status');
            $table->index('scheduled_at');
        });

        // =====================================================
        // SMART LINKS (Trackable Action Links)
        // =====================================================
        Schema::create('smart_links', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('title');
            $table->string('short', 20)->unique(); // short URL slug
            $table->text('target_url')->nullable();
            $table->json('actions')->nullable(); // actions to perform on click
            $table->text('notes')->nullable();
            $table->boolean('is_active')->default(true);
            $table->integer('click_count')->default(0);
            $table->integer('unique_clicks')->default(0);
            $table->json('click_data')->nullable(); // analytics data
            $table->uuid('created_by')->nullable();
            $table->timestamps();

            $table->index('short');
            $table->index('is_active');
        });

        // Smart Link Clicks (tracking)
        Schema::create('smart_link_clicks', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('smart_link_id');
            $table->uuid('contact_id')->nullable();
            $table->string('ip_address', 45)->nullable();
            $table->string('user_agent')->nullable();
            $table->string('referrer')->nullable();
            $table->string('country', 2)->nullable();
            $table->string('city', 100)->nullable();
            $table->string('device', 50)->nullable();
            $table->string('browser', 50)->nullable();
            $table->string('os', 50)->nullable();
            $table->json('utm_params')->nullable();
            $table->timestamp('clicked_at');

            $table->foreign('smart_link_id')->references('id')->on('smart_links')->cascadeOnDelete();
            $table->index(['smart_link_id', 'clicked_at']);
            $table->index('contact_id');
        });

        // =====================================================
        // DYNAMIC SEGMENTS (Advanced Filtering)
        // =====================================================
        Schema::create('dynamic_segment_rules', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('segment_id');
            $table->string('rule_type', 50); // contact_field, behavior, commerce, email, custom
            $table->string('field');
            $table->string('operator', 30);
            $table->json('value')->nullable();
            $table->enum('logic', ['and', 'or'])->default('and');
            $table->integer('group_index')->default(0);
            $table->integer('position')->default(0);
            $table->timestamps();

            $table->foreign('segment_id')->references('id')->on('contact_segments')->cascadeOnDelete();
            $table->index(['segment_id', 'group_index']);
        });

        // =====================================================
        // COMMERCE REPORTS (Revenue Attribution)
        // =====================================================
        Schema::create('contact_commerce_relations', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('contact_id');
            $table->string('provider', 50); // woocommerce, stripe, edd, etc.
            $table->string('provider_id')->nullable(); // external ID
            $table->enum('relation_type', ['customer', 'subscriber', 'member'])->default('customer');
            $table->decimal('total_order_value', 12, 2)->default(0);
            $table->integer('total_orders')->default(0);
            $table->decimal('average_order_value', 12, 2)->default(0);
            $table->timestamp('first_order_at')->nullable();
            $table->timestamp('last_order_at')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamps();

            $table->foreign('contact_id')->references('id')->on('contacts')->cascadeOnDelete();
            $table->unique(['contact_id', 'provider']);
            $table->index('provider');
        });

        // Commerce Relation Items (individual orders/purchases)
        Schema::create('contact_commerce_items', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('relation_id');
            $table->string('item_type', 50); // order, subscription, donation
            $table->string('item_id'); // external order ID
            $table->decimal('amount', 12, 2)->default(0);
            $table->string('currency', 3)->default('USD');
            $table->enum('status', ['pending', 'completed', 'refunded', 'cancelled'])->default('completed');
            $table->json('line_items')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamp('transaction_at');
            $table->timestamps();

            $table->foreign('relation_id')->references('id')->on('contact_commerce_relations')->cascadeOnDelete();
            $table->index(['relation_id', 'transaction_at']);
            $table->index('item_id');
        });

        // Campaign Revenue Attribution
        Schema::create('campaign_revenue', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('campaign_id'); // can be sequence, recurring, or regular campaign
            $table->string('campaign_type', 50); // email_sequence, recurring_campaign, campaign
            $table->uuid('contact_id');
            $table->uuid('commerce_item_id')->nullable();
            $table->decimal('amount', 12, 2);
            $table->string('currency', 3)->default('USD');
            $table->string('attribution_type', 30)->default('direct'); // direct, assisted, last_touch
            $table->integer('days_to_conversion')->default(0);
            $table->timestamp('attributed_at');
            $table->timestamps();

            $table->index(['campaign_id', 'campaign_type']);
            $table->index('contact_id');
            $table->index('attributed_at');
        });

        // =====================================================
        // CRM MANAGERS (Team & Permissions)
        // =====================================================
        Schema::create('crm_managers', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('user_id')->unique();
            $table->json('permissions')->nullable(); // granular permissions
            $table->json('accessible_tags')->nullable(); // tag restrictions
            $table->json('accessible_lists')->nullable(); // list restrictions
            $table->boolean('can_manage_contacts')->default(true);
            $table->boolean('can_manage_campaigns')->default(false);
            $table->boolean('can_manage_sequences')->default(false);
            $table->boolean('can_manage_automations')->default(false);
            $table->boolean('can_view_reports')->default(true);
            $table->boolean('can_export_data')->default(false);
            $table->boolean('is_super_admin')->default(false);
            $table->uuid('created_by')->nullable();
            $table->timestamps();

            $table->foreign('user_id')->references('id')->on('users')->cascadeOnDelete();
        });

        // =====================================================
        // FUNNEL AUTOMATION (Triggers, Actions, Conditions)
        // =====================================================
        Schema::create('automation_funnels', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('title');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->enum('status', ['draft', 'active', 'paused'])->default('draft');
            $table->string('trigger_type', 100); // contact_created, tag_applied, form_submitted, etc.
            $table->json('trigger_settings')->nullable();
            $table->json('conditions')->nullable(); // entry conditions
            $table->integer('subscribers_count')->default(0);
            $table->integer('completed_count')->default(0);
            $table->decimal('total_revenue', 12, 2)->default(0);
            $table->uuid('created_by')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index('status');
            $table->index('trigger_type');
        });

        // Funnel Actions (steps in the funnel)
        Schema::create('funnel_actions', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('funnel_id');
            $table->uuid('parent_id')->nullable(); // for branching
            $table->string('action_type', 100); // send_email, add_tag, wait, condition, etc.
            $table->string('title')->nullable();
            $table->json('settings')->nullable();
            $table->integer('position')->default(0);
            $table->enum('condition_type', ['yes', 'no', 'none'])->default('none'); // for conditional branches
            $table->integer('delay_seconds')->default(0);
            $table->integer('execution_count')->default(0);
            $table->timestamps();

            $table->foreign('funnel_id')->references('id')->on('automation_funnels')->cascadeOnDelete();
            $table->index(['funnel_id', 'position']);
        });

        // Funnel Subscribers (contacts in funnels)
        Schema::create('funnel_subscribers', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->uuid('funnel_id');
            $table->uuid('contact_id');
            $table->uuid('current_action_id')->nullable();
            $table->enum('status', ['active', 'waiting', 'completed', 'cancelled', 'failed'])->default('active');
            $table->timestamp('entered_at');
            $table->timestamp('next_execution_at')->nullable();
            $table->timestamp('completed_at')->nullable();
            $table->json('execution_log')->nullable();
            $table->integer('actions_completed')->default(0);
            $table->timestamps();

            $table->foreign('funnel_id')->references('id')->on('automation_funnels')->cascadeOnDelete();
            $table->foreign('contact_id')->references('id')->on('contacts')->cascadeOnDelete();
            $table->unique(['funnel_id', 'contact_id']);
            $table->index('status');
            $table->index('next_execution_at');
        });

        // =====================================================
        // CONTACT LISTS (for FluentCRM compatibility)
        // =====================================================
        Schema::create('contact_lists', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('title');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->boolean('is_public')->default(false);
            $table->integer('contacts_count')->default(0);
            $table->uuid('created_by')->nullable();
            $table->timestamps();

            $table->index('is_public');
        });

        // Contact List Members
        Schema::create('contact_list_members', function (Blueprint $table): void {
            $table->uuid('list_id');
            $table->uuid('contact_id');
            $table->enum('status', ['subscribed', 'unsubscribed', 'pending'])->default('subscribed');
            $table->timestamp('subscribed_at')->useCurrent();
            $table->timestamp('unsubscribed_at')->nullable();

            $table->primary(['list_id', 'contact_id']);
            $table->foreign('list_id')->references('id')->on('contact_lists')->cascadeOnDelete();
            $table->foreign('contact_id')->references('id')->on('contacts')->cascadeOnDelete();
        });

        // =====================================================
        // CONTACT TAGS (enhanced for FluentCRM)
        // =====================================================
        Schema::create('contact_tags', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('title');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->string('color', 7)->default('#6366F1');
            $table->integer('contacts_count')->default(0);
            $table->uuid('created_by')->nullable();
            $table->timestamps();
        });

        // Contact Tag Pivot
        Schema::create('contact_tag_pivot', function (Blueprint $table): void {
            $table->uuid('tag_id');
            $table->uuid('contact_id');
            $table->timestamp('applied_at')->useCurrent();
            $table->uuid('applied_by')->nullable();

            $table->primary(['tag_id', 'contact_id']);
            $table->foreign('tag_id')->references('id')->on('contact_tags')->cascadeOnDelete();
            $table->foreign('contact_id')->references('id')->on('contacts')->cascadeOnDelete();
        });

        // =====================================================
        // CONTACT COMPANIES (B2B CRM)
        // =====================================================
        Schema::create('crm_companies', function (Blueprint $table): void {
            $table->uuid('id')->primary();
            $table->string('name');
            $table->string('slug')->unique();
            $table->string('website')->nullable();
            $table->string('industry', 100)->nullable();
            $table->string('size', 50)->nullable(); // 1-10, 11-50, etc.
            $table->decimal('annual_revenue', 15, 2)->nullable();
            $table->string('phone', 50)->nullable();
            $table->string('email')->nullable();
            $table->text('description')->nullable();
            $table->string('address_line1')->nullable();
            $table->string('address_line2')->nullable();
            $table->string('city', 100)->nullable();
            $table->string('state', 100)->nullable();
            $table->string('postal_code', 20)->nullable();
            $table->string('country', 100)->nullable();
            $table->string('linkedin_url')->nullable();
            $table->string('twitter_handle', 100)->nullable();
            $table->string('logo_url')->nullable();
            $table->json('custom_fields')->nullable();
            $table->uuid('owner_id')->nullable();
            $table->integer('contacts_count')->default(0);
            $table->integer('deals_count')->default(0);
            $table->decimal('total_deal_value', 15, 2)->default(0);
            $table->timestamps();
            $table->softDeletes();

            $table->index('owner_id');
            $table->index('industry');
        });

        // Update contacts table with company_id foreign key (if not exists)
        if (!Schema::hasColumn('contacts', 'company_id')) {
            Schema::table('contacts', function (Blueprint $table): void {
                $table->uuid('company_id')->nullable()->after('mobile');
                $table->foreign('company_id')->references('id')->on('crm_companies')->nullOnDelete();
            });
        }
    }

    public function down(): void
    {
        // Remove company_id from contacts if we added it
        if (Schema::hasColumn('contacts', 'company_id')) {
            Schema::table('contacts', function (Blueprint $table): void {
                $table->dropForeign(['company_id']);
                $table->dropColumn('company_id');
            });
        }

        Schema::dropIfExists('crm_companies');
        Schema::dropIfExists('contact_tag_pivot');
        Schema::dropIfExists('contact_tags');
        Schema::dropIfExists('contact_list_members');
        Schema::dropIfExists('contact_lists');
        Schema::dropIfExists('funnel_subscribers');
        Schema::dropIfExists('funnel_actions');
        Schema::dropIfExists('automation_funnels');
        Schema::dropIfExists('crm_managers');
        Schema::dropIfExists('campaign_revenue');
        Schema::dropIfExists('contact_commerce_items');
        Schema::dropIfExists('contact_commerce_relations');
        Schema::dropIfExists('dynamic_segment_rules');
        Schema::dropIfExists('smart_link_clicks');
        Schema::dropIfExists('smart_links');
        Schema::dropIfExists('recurring_mails');
        Schema::dropIfExists('recurring_campaigns');
        Schema::dropIfExists('sequence_trackers');
        Schema::dropIfExists('sequence_mails');
        Schema::dropIfExists('email_sequences');
    }
};
