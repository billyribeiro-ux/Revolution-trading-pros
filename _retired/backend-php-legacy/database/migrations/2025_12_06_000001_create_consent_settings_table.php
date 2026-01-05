<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     *
     * Consent Settings table for complete consent management configuration.
     * Mirrors all functionality from Consent Magic Pro WordPress plugin.
     */
    public function up(): void
    {
        Schema::create('consent_settings', function (Blueprint $table) {
            $table->id();
            $table->string('key', 100)->unique();
            $table->text('value')->nullable();
            $table->string('type', 50)->default('string');
            $table->string('group', 50)->default('general');
            $table->text('description')->nullable();
            $table->boolean('is_public')->default(false);
            $table->timestamps();

            $table->index(['group', 'key']);
            $table->index('is_public');
        });

        // Consent Banner Templates
        Schema::create('consent_banner_templates', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100);
            $table->string('slug', 100)->unique();
            $table->string('category', 50)->default('custom');
            $table->text('description')->nullable();
            $table->boolean('is_active')->default(false);
            $table->boolean('is_default')->default(false);
            $table->boolean('is_system')->default(false);

            // Layout Settings
            $table->string('layout_type', 50)->default('bar'); // bar, popup, floating, drawer
            $table->string('position', 50)->default('bottom'); // top, bottom, center
            $table->string('position_horizontal', 50)->default('center'); // left, center, right

            // Design - Colors
            $table->string('background_color', 20)->default('#1a1f2e');
            $table->string('text_color', 20)->default('#ffffff');
            $table->string('link_color', 20)->default('#3b82f6');
            $table->string('title_color', 20)->default('#ffffff');
            $table->string('border_color', 20)->default('#333333');
            $table->string('border_style', 20)->default('solid');
            $table->integer('border_width')->default(0);

            // Buttons - Accept All
            $table->string('accept_btn_bg', 20)->default('#3b82f6');
            $table->string('accept_btn_text', 20)->default('#ffffff');
            $table->string('accept_btn_hover_bg', 20)->default('#2563eb');

            // Buttons - Reject All
            $table->string('reject_btn_bg', 20)->default('#374151');
            $table->string('reject_btn_text', 20)->default('#ffffff');
            $table->string('reject_btn_hover_bg', 20)->default('#4b5563');

            // Buttons - Settings/Customize
            $table->string('settings_btn_bg', 20)->default('transparent');
            $table->string('settings_btn_text', 20)->default('#3b82f6');
            $table->string('settings_btn_border', 20)->default('#3b82f6');

            // Toggle Colors
            $table->string('toggle_active_color', 20)->default('#3b82f6');
            $table->string('toggle_inactive_color', 20)->default('#6b7280');

            // Typography
            $table->string('font_family', 100)->default('system-ui, -apple-system, sans-serif');
            $table->integer('title_font_size')->default(18);
            $table->integer('title_font_weight')->default(600);
            $table->integer('body_font_size')->default(14);
            $table->integer('body_font_weight')->default(400);
            $table->integer('btn_font_size')->default(14);
            $table->integer('btn_font_weight')->default(500);

            // Spacing
            $table->integer('padding_top')->default(20);
            $table->integer('padding_bottom')->default(20);
            $table->integer('padding_left')->default(24);
            $table->integer('padding_right')->default(24);
            $table->integer('btn_padding_x')->default(20);
            $table->integer('btn_padding_y')->default(12);
            $table->integer('btn_margin')->default(8);
            $table->integer('btn_border_radius')->default(8);
            $table->integer('container_border_radius')->default(0);
            $table->integer('container_max_width')->default(1200);

            // Animation
            $table->string('animation_type', 50)->default('slide'); // slide, fade, none
            $table->integer('animation_duration')->default(300);

            // Content
            $table->string('title')->default('We value your privacy');
            $table->text('message_text')->nullable(); // Banner message/body text
            $table->string('accept_btn_label', 50)->default('Accept All');
            $table->string('reject_btn_label', 50)->default('Reject All');
            $table->string('settings_btn_label', 50)->default('Manage Preferences');
            $table->string('privacy_link_text', 100)->default('Privacy Policy');
            $table->string('privacy_link_url', 255)->nullable();
            $table->string('cookie_link_text', 100)->default('Cookie Policy');
            $table->string('cookie_link_url', 255)->nullable();

            // Behavior
            $table->boolean('show_reject_btn')->default(true);
            $table->boolean('show_settings_btn')->default(true);
            $table->boolean('show_privacy_link')->default(true);
            $table->boolean('show_cookie_link')->default(false);
            $table->boolean('close_on_scroll')->default(false);
            $table->integer('close_on_scroll_distance')->default(60);
            $table->boolean('show_close_btn')->default(false);
            $table->boolean('block_page_scroll')->default(false);
            $table->boolean('show_powered_by')->default(false);

            // Logo
            $table->string('logo_url', 255)->nullable();
            $table->integer('logo_size')->default(40);
            $table->string('logo_position', 20)->default('left');

            // Full JSON config for advanced customization
            $table->json('full_config')->nullable();

            $table->timestamps();

            $table->index('is_active');
            $table->index('category');
        });

        // Consent Script Blocking Rules
        Schema::create('consent_script_rules', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100);
            $table->string('slug', 100)->unique();
            $table->string('category', 50); // necessary, analytics, marketing, preferences
            $table->text('description')->nullable();
            $table->boolean('is_enabled')->default(true);
            $table->boolean('is_system')->default(false);

            // Pattern matching
            $table->text('script_patterns')->nullable(); // JSON array of regex patterns
            $table->text('cookie_patterns')->nullable(); // JSON array of cookie patterns
            $table->text('domain_patterns')->nullable(); // JSON array of domain patterns

            // Vendor info
            $table->string('vendor_name', 100)->nullable();
            $table->string('vendor_privacy_url', 255)->nullable();

            $table->timestamps();

            $table->index('category');
            $table->index('is_enabled');
        });

        // Consent Geolocation Rules
        Schema::create('consent_geo_rules', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100);
            $table->string('region_code', 10); // US, EU, CA, GB, etc.
            $table->string('region_type', 20); // country, continent, state
            $table->boolean('is_enabled')->default(true);

            // Rule configuration
            $table->boolean('require_explicit_consent')->default(true);
            $table->boolean('require_opt_in')->default(true);
            $table->boolean('show_reject_button')->default(true);
            $table->boolean('pre_check_analytics')->default(false);
            $table->boolean('pre_check_marketing')->default(false);
            $table->boolean('pre_check_preferences')->default(false);

            // Compliance framework
            $table->string('framework', 50)->nullable(); // GDPR, CCPA, LGPD, PIPEDA, etc.
            $table->json('framework_config')->nullable();

            $table->timestamps();

            $table->unique(['region_code', 'region_type']);
            $table->index('is_enabled');
        });

        // Consent Cookie Definitions
        Schema::create('consent_cookie_definitions', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100);
            $table->string('pattern', 255);
            $table->string('category', 50);
            $table->string('type', 20)->default('first-party'); // first-party, third-party
            $table->text('purpose')->nullable();
            $table->string('duration', 50)->nullable();
            $table->string('vendor', 100)->nullable();
            $table->boolean('is_http_only')->default(false);
            $table->boolean('is_secure')->default(false);
            $table->boolean('is_system')->default(false);

            $table->timestamps();

            $table->index('category');
            $table->index('vendor');
        });

        // Consent Proof (for compliance auditing)
        Schema::create('consent_proofs', function (Blueprint $table) {
            $table->id();
            $table->string('consent_id', 50)->unique();
            $table->string('identifier', 255);
            $table->string('identifier_type', 50);

            // Consent state at time of proof
            $table->json('consent_state');
            $table->json('categories_consented');

            // Context
            $table->string('ip_address', 45)->nullable();
            $table->text('user_agent')->nullable();
            $table->string('page_url', 500)->nullable();
            $table->string('region', 10)->nullable();

            // Compliance info
            $table->string('method', 50); // banner, modal, api
            $table->string('banner_version', 50)->nullable();
            $table->string('policy_version', 50)->nullable();

            // Timestamps
            $table->timestamp('consented_at');
            $table->timestamp('expires_at')->nullable();
            $table->timestamps();

            $table->index('identifier');
            $table->index('consented_at');
            $table->index('expires_at');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('consent_proofs');
        Schema::dropIfExists('consent_cookie_definitions');
        Schema::dropIfExists('consent_geo_rules');
        Schema::dropIfExists('consent_script_rules');
        Schema::dropIfExists('consent_banner_templates');
        Schema::dropIfExists('consent_settings');
    }
};
