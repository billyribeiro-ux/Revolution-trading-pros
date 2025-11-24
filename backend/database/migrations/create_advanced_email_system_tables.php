<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Email System Complete Migration
 * RevolutionEmailTemplates-L8-System
 * 
 * Creates all tables for enterprise email template system:
 * - Layouts, Blocks, Variables, Automation, Campaigns, Events, Versions, Brand Settings
 */
return new class extends Migration
{
    public function up(): void
    {
        // 1. EMAIL LAYOUTS
        Schema::create('email_layouts', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('html_structure');
            $table->json('default_styles')->nullable();
            $table->json('header_blocks')->nullable();
            $table->json('footer_blocks')->nullable();
            $table->boolean('is_system')->default(false);
            $table->boolean('is_default')->default(false);
            $table->timestamps();
            
            $table->index('slug');
            $table->index('is_default');
        });

        // 2. EMAIL BLOCKS
        Schema::create('email_blocks', function (Blueprint $table) {
            $table->id();
            $table->uuid('uuid')->unique();
            $table->foreignId('template_id')->constrained('email_templates')->onDelete('cascade');
            $table->enum('block_type', ['text', 'image', 'button', 'divider', 'spacer', 'columns', 'header', 'footer', 'product', 'social']);
            $table->json('content');
            $table->json('styles')->nullable();
            $table->json('settings')->nullable();
            $table->integer('order')->default(0);
            $table->foreignId('parent_block_id')->nullable()->constrained('email_blocks')->onDelete('cascade');
            $table->json('conditional_rules')->nullable();
            $table->timestamps();
            
            $table->index(['template_id', 'order']);
            $table->index('block_type');
        });

        // 3. EMAIL VARIABLES
        Schema::create('email_variables', function (Blueprint $table) {
            $table->id();
            $table->string('category'); // user, order, subscription, system, custom
            $table->string('variable_key')->unique(); // user.name
            $table->string('variable_name');
            $table->text('description')->nullable();
            $table->enum('data_type', ['string', 'number', 'date', 'boolean', 'array', 'object']);
            $table->text('default_value')->nullable();
            $table->boolean('is_system')->default(false);
            $table->string('resolver_class')->nullable();
            $table->timestamps();
            
            $table->index('category');
            $table->index('variable_key');
        });

        // 4. EMAIL AUTOMATION WORKFLOWS
        Schema::create('email_automation_workflows', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('trigger_event'); // user.registered, order.placed, etc.
            $table->json('trigger_conditions')->nullable();
            $table->foreignId('template_id')->constrained('email_templates')->onDelete('cascade');
            $table->integer('delay_minutes')->default(0);
            $table->boolean('is_active')->default(true);
            $table->integer('priority')->default(0);
            $table->boolean('send_once_per_user')->default(true);
            $table->integer('max_sends_per_user')->nullable();
            $table->json('statistics')->nullable();
            $table->timestamps();
            
            $table->index('trigger_event');
            $table->index(['is_active', 'priority']);
        });

        // 5. EMAIL EVENTS
        Schema::create('email_events', function (Blueprint $table) {
            $table->id();
            $table->foreignId('email_log_id')->constrained('email_logs')->onDelete('cascade');
            $table->enum('event_type', ['sent', 'delivered', 'opened', 'clicked', 'bounced', 'complained', 'unsubscribed']);
            $table->json('event_data')->nullable();
            $table->string('ip_address')->nullable();
            $table->text('user_agent')->nullable();
            $table->string('device_type')->nullable();
            $table->json('location')->nullable();
            $table->timestamp('event_timestamp');
            $table->timestamps();
            
            $table->index(['email_log_id', 'event_type']);
            $table->index('event_timestamp');
        });

        // 6. EMAIL CAMPAIGNS
        Schema::create('email_campaigns', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->foreignId('template_id')->constrained('email_templates')->onDelete('restrict');
            $table->unsignedBigInteger('segment_id')->nullable();
            $table->string('subject');
            $table->string('from_name');
            $table->string('from_email');
            $table->string('reply_to')->nullable();
            $table->timestamp('scheduled_at')->nullable();
            $table->timestamp('sent_at')->nullable();
            $table->enum('status', ['draft', 'scheduled', 'sending', 'sent', 'cancelled'])->default('draft');
            $table->integer('total_recipients')->default(0);
            $table->integer('sent_count')->default(0);
            $table->integer('opened_count')->default(0);
            $table->integer('clicked_count')->default(0);
            $table->integer('bounced_count')->default(0);
            $table->integer('unsubscribed_count')->default(0);
            $table->json('ab_test_config')->nullable();
            $table->foreignId('created_by')->constrained('users')->onDelete('cascade');
            $table->timestamps();
            
            $table->index('status');
            $table->index('scheduled_at');
        });

        // 7. EMAIL TEMPLATE VERSIONS
        Schema::create('email_template_versions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('template_id')->constrained('email_templates')->onDelete('cascade');
            $table->integer('version_number');
            $table->json('snapshot');
            $table->text('changes_summary')->nullable();
            $table->foreignId('created_by')->constrained('users')->onDelete('cascade');
            $table->timestamps();
            
            $table->unique(['template_id', 'version_number']);
            $table->index('created_at');
        });

        // 8. EMAIL BRAND SETTINGS
        Schema::create('email_brand_settings', function (Blueprint $table) {
            $table->id();
            $table->string('brand_name');
            $table->string('logo_url')->nullable();
            $table->string('primary_color')->default('#007bff');
            $table->string('secondary_color')->default('#6c757d');
            $table->string('accent_color')->default('#28a745');
            $table->string('font_family')->default('Arial, sans-serif');
            $table->integer('font_size_base')->default(16);
            $table->json('button_style')->nullable();
            $table->string('link_color')->default('#007bff');
            $table->string('background_color')->default('#ffffff');
            $table->text('footer_text')->nullable();
            $table->json('social_links')->nullable();
            $table->boolean('is_active')->default(true);
            $table->timestamps();
            
            $table->index('is_active');
        });

        // 9. Enhance existing email_templates table
        Schema::table('email_templates', function (Blueprint $table) {
            if (!Schema::hasColumn('email_templates', 'category')) {
                $table->enum('category', ['transactional', 'marketing', 'automation', 'system'])->default('transactional')->after('slug');
            }
            if (!Schema::hasColumn('email_templates', 'subject_template')) {
                $table->text('subject_template')->nullable()->after('subject');
            }
            if (!Schema::hasColumn('email_templates', 'preheader_template')) {
                $table->text('preheader_template')->nullable()->after('subject_template');
            }
            if (!Schema::hasColumn('email_templates', 'layout_id')) {
                $table->foreignId('layout_id')->nullable()->after('email_type')->constrained('email_layouts')->onDelete('set null');
            }
            if (!Schema::hasColumn('email_templates', 'blocks')) {
                $table->json('blocks')->nullable()->after('body_text');
            }
            if (!Schema::hasColumn('email_templates', 'global_styles')) {
                $table->json('global_styles')->nullable()->after('blocks');
            }
            if (!Schema::hasColumn('email_templates', 'variables_schema')) {
                $table->json('variables_schema')->nullable()->after('variables');
            }
            if (!Schema::hasColumn('email_templates', 'is_system')) {
                $table->boolean('is_system')->default(false)->after('is_active');
            }
            if (!Schema::hasColumn('email_templates', 'version')) {
                $table->integer('version')->default(1)->after('is_system');
            }
            if (!Schema::hasColumn('email_templates', 'parent_template_id')) {
                $table->foreignId('parent_template_id')->nullable()->after('version')->constrained('email_templates')->onDelete('set null');
            }
            if (!Schema::hasColumn('email_templates', 'published_at')) {
                $table->timestamp('published_at')->nullable()->after('parent_template_id');
            }
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('email_template_versions');
        Schema::dropIfExists('email_campaigns');
        Schema::dropIfExists('email_events');
        Schema::dropIfExists('email_automation_workflows');
        Schema::dropIfExists('email_variables');
        Schema::dropIfExists('email_blocks');
        Schema::dropIfExists('email_brand_settings');
        Schema::dropIfExists('email_layouts');
        
        Schema::table('email_templates', function (Blueprint $table) {
            $table->dropColumn([
                'category', 'subject_template', 'preheader_template', 
                'layout_id', 'blocks', 'global_styles', 'variables_schema',
                'is_system', 'version', 'parent_template_id', 'published_at'
            ]);
        });
    }
};
