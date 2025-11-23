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
        // Redirects table
        if (!Schema::hasTable('redirects')) {
        Schema::create('redirects', function (Blueprint $table) {
            $table->id();
            $table->string('from_path');
            $table->string('to_path');
            $table->integer('status_code')->default(301);
            $table->enum('type', ['manual', 'automatic'])->default('manual');
            $table->boolean('is_active')->default(true);
            $table->integer('hit_count')->default(0);
            $table->timestamp('last_hit_at')->nullable();
            $table->timestamps();
            
            $table->index('from_path');
            $table->index('is_active');
        });

        // 404 Errors table
        Schema::create('error_404s', function (Blueprint $table) {
            $table->id();
            $table->string('url');
            $table->string('referrer')->nullable();
            $table->integer('hit_count')->default(1);
            $table->boolean('is_resolved')->default(false);
            $table->timestamp('first_seen_at');
            $table->timestamp('last_seen_at');
            $table->timestamps();
            
            $table->index('url');
            $table->index('is_resolved');
            $table->index('last_seen_at');
        });

        // SEO Analyses table
        Schema::create('seo_analyses', function (Blueprint $table) {
            $table->id();
            $table->string('analyzable_type');
            $table->unsignedBigInteger('analyzable_id');
            $table->string('focus_keyword')->nullable();
            $table->integer('seo_score')->default(0);
            $table->json('analysis_results')->nullable();
            $table->json('suggestions')->nullable();
            $table->decimal('keyword_density', 5, 2)->default(0);
            $table->integer('readability_score')->default(0);
            $table->boolean('has_meta_title')->default(false);
            $table->boolean('has_meta_description')->default(false);
            $table->timestamps();
            
            $table->index(['analyzable_type', 'analyzable_id']);
        });

        // Rank Tracking table
        Schema::create('rank_trackings', function (Blueprint $table) {
            $table->id();
            $table->string('keyword');
            $table->string('url');
            $table->integer('current_position')->nullable();
            $table->integer('previous_position')->nullable();
            $table->string('search_engine')->default('google');
            $table->string('location')->default('us');
            $table->string('device')->default('desktop');
            $table->timestamp('last_checked_at')->nullable();
            $table->timestamps();
            
            $table->index('keyword');
            $table->index('last_checked_at');
        });

        // Rank History table
        Schema::create('rank_histories', function (Blueprint $table) {
            $table->id();
            $table->foreignId('rank_tracking_id')->constrained()->onDelete('cascade');
            $table->integer('position');
            $table->date('date');
            $table->timestamps();
            
            $table->index(['rank_tracking_id', 'date']);
        });

        // Backlinks table
        Schema::create('backlinks', function (Blueprint $table) {
            $table->id();
            $table->string('source_url');
            $table->string('target_url');
            $table->string('anchor_text')->nullable();
            $table->integer('domain_authority')->default(0);
            $table->integer('page_authority')->default(0);
            $table->boolean('is_follow')->default(true);
            $table->boolean('is_toxic')->default(false);
            $table->timestamp('first_seen_at');
            $table->timestamp('last_seen_at');
            $table->timestamps();
            
            $table->index('target_url');
            $table->index('is_toxic');
        });

        // SEO Settings table
        Schema::create('seo_settings', function (Blueprint $table) {
            $table->id();
            $table->string('key')->unique();
            $table->text('value')->nullable();
            $table->string('type')->default('string');
            $table->timestamps();
        });

        // SEO Alerts table
        Schema::create('seo_alerts', function (Blueprint $table) {
            $table->id();
            $table->string('type');
            $table->string('severity')->default('info');
            $table->string('title');
            $table->text('message');
            $table->json('metadata')->nullable();
            $table->boolean('is_new')->default(true);
            $table->boolean('is_resolved')->default(false);
            $table->timestamps();
            
            $table->index('type');
            $table->index('is_resolved');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('seo_alerts');
        Schema::dropIfExists('seo_settings');
        Schema::dropIfExists('backlinks');
        Schema::dropIfExists('rank_histories');
        Schema::dropIfExists('rank_trackings');
        Schema::dropIfExists('seo_analyses');
        Schema::dropIfExists('error_404s');
        Schema::dropIfExists('redirects');
    }
};
