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
        // Enhance seo_settings table
        Schema::table('seo_settings', function (Blueprint $table) {
            $table->string('category', 50)->nullable()->after('type');
            $table->text('description')->nullable()->after('category');
            $table->json('validation_rules')->nullable()->after('description');
            $table->index(['category', 'key']);
            $table->index('updated_at');
        });

        // Create sitemap_configs table
        Schema::create('sitemap_configs', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100)->unique();
            $table->string('url', 255);
            $table->enum('type', ['posts', 'pages', 'products', 'categories', 'tags', 'custom']);
            $table->enum('frequency', ['always', 'hourly', 'daily', 'weekly', 'monthly', 'yearly', 'never'])->default('daily');
            $table->decimal('priority', 2, 1)->default(0.5);
            $table->json('filters')->nullable();
            $table->boolean('include_images')->default(false);
            $table->boolean('include_videos')->default(false);
            $table->integer('max_entries')->default(10000);
            $table->boolean('is_active')->default(true);
            $table->timestamp('last_generated_at')->nullable();
            $table->timestamps();
            
            $table->index(['is_active', 'type']);
            $table->index('last_generated_at');
        });

        // Enhanced seo_alerts table
        Schema::table('seo_alerts', function (Blueprint $table) {
            $table->string('type', 100)->after('id');
            $table->enum('severity', ['low', 'medium', 'high', 'critical'])->after('type');
            $table->string('category', 50)->after('severity');
            $table->string('title', 255)->after('category');
            $table->text('description')->change();
            $table->string('affected_url', 500)->nullable()->after('description');
            $table->string('affected_content_type', 50)->nullable()->after('affected_url');
            $table->unsignedBigInteger('affected_content_id')->nullable()->after('affected_content_type');
            $table->json('metadata')->nullable()->after('affected_content_id');
            $table->string('source', 50)->default('system')->after('metadata');
            $table->timestamp('resolved_at')->nullable()->after('is_resolved');
            $table->unsignedBigInteger('resolved_by')->nullable()->after('resolved_at');
            $table->text('resolution_notes')->nullable()->after('resolved_by');
            $table->string('action_taken', 500)->nullable()->after('resolution_notes');
            $table->unsignedBigInteger('created_by')->nullable()->after('action_taken');
            
            $table->index(['severity', 'is_resolved']);
            $table->index(['category', 'created_at']);
            $table->index('affected_url');
            $table->index(['affected_content_type', 'affected_content_id']);
            
            $table->foreign('resolved_by')->references('id')->on('users')->onDelete('set null');
            $table->foreign('created_by')->references('id')->on('users')->onDelete('set null');
        });

        // Create local_business_data table
        Schema::create('local_business_data', function (Blueprint $table) {
            $table->id();
            $table->string('name', 255);
            $table->text('description')->nullable();
            $table->json('address');
            $table->string('phone', 50);
            $table->string('email', 255)->nullable();
            $table->string('website', 500)->nullable();
            $table->json('hours');
            $table->json('social_media')->nullable();
            $table->string('logo_url', 500)->nullable();
            $table->json('images')->nullable();
            $table->json('categories')->nullable();
            $table->enum('price_range', ['$', '$$', '$$$', '$$$$'])->nullable();
            $table->boolean('accepts_reservations')->default(false);
            $table->boolean('delivery_available')->default(false);
            $table->boolean('takeout_available')->default(false);
            $table->boolean('wheelchair_accessible')->default(true);
            $table->boolean('parking_available')->default(false);
            $table->boolean('wifi_available')->default(false);
            $table->json('additional_info')->nullable();
            $table->timestamps();
        });

        // Create seo_redirects table for redirect management
        Schema::create('seo_redirects', function (Blueprint $table) {
            $table->id();
            $table->string('from_url', 500);
            $table->string('to_url', 500);
            $table->integer('status_code')->default(301);
            $table->integer('hit_count')->default(0);
            $table->timestamp('last_hit_at')->nullable();
            $table->boolean('is_active')->default(true);
            $table->string('created_reason', 255)->nullable();
            $table->timestamps();
            
            $table->unique('from_url');
            $table->index(['is_active', 'from_url']);
            $table->index('hit_count');
        });

        // Create keyword_trackings table
        Schema::create('keyword_trackings', function (Blueprint $table) {
            $table->id();
            $table->string('keyword', 255);
            $table->string('url', 500)->nullable();
            $table->integer('position')->nullable();
            $table->integer('previous_position')->nullable();
            $table->decimal('search_volume', 10, 2)->nullable();
            $table->decimal('difficulty', 5, 2)->nullable();
            $table->string('location', 100)->default('US');
            $table->string('language', 10)->default('en');
            $table->timestamp('tracked_at');
            $table->timestamps();
            
            $table->index(['keyword', 'location', 'tracked_at']);
            $table->index(['url', 'tracked_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('seo_settings', function (Blueprint $table) {
            $table->dropColumn(['category', 'description', 'validation_rules']);
        });

        Schema::dropIfExists('sitemap_configs');
        
        Schema::table('seo_alerts', function (Blueprint $table) {
            $table->dropForeign(['resolved_by']);
            $table->dropForeign(['created_by']);
            $table->dropColumn([
                'type', 'severity', 'category', 'title',
                'affected_url', 'affected_content_type', 'affected_content_id',
                'metadata', 'source', 'resolved_at', 'resolved_by',
                'resolution_notes', 'action_taken', 'created_by'
            ]);
        });

        Schema::dropIfExists('local_business_data');
        Schema::dropIfExists('seo_redirects');
        Schema::dropIfExists('keyword_trackings');
    }
};