<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
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
        }

        // SEO Analytics table
        if (!Schema::hasTable('seo_analytics')) {
            Schema::create('seo_analytics', function (Blueprint $table) {
                $table->id();
                $table->string('url');
                $table->date('date');
                $table->integer('impressions')->default(0);
                $table->integer('clicks')->default(0);
                $table->decimal('ctr', 5, 2)->default(0);
                $table->decimal('average_position', 5, 2)->default(0);
                $table->timestamps();
                
                $table->index(['url', 'date']);
            });
        }

        // Rank Tracking table
        if (!Schema::hasTable('rank_tracking')) {
            Schema::create('rank_tracking', function (Blueprint $table) {
                $table->id();
                $table->string('keyword');
                $table->string('url');
                $table->integer('position')->nullable();
                $table->string('search_engine')->default('google');
                $table->string('location')->default('us');
                $table->date('date');
                $table->timestamps();
                
                $table->index(['keyword', 'date']);
                $table->index('url');
            });
        }

        // Rank Histories table
        if (!Schema::hasTable('rank_histories')) {
            Schema::create('rank_histories', function (Blueprint $table) {
                $table->id();
                $table->foreignId('rank_tracking_id')->constrained()->onDelete('cascade');
                $table->integer('position');
                $table->date('date');
                $table->timestamps();
                
                $table->index(['rank_tracking_id', 'date']);
            });
        }

        // Backlinks table
        if (!Schema::hasTable('backlinks')) {
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
        }

        // SEO Settings table
        if (!Schema::hasTable('seo_settings')) {
            Schema::create('seo_settings', function (Blueprint $table) {
                $table->id();
                $table->string('key')->unique();
                $table->text('value')->nullable();
                $table->string('type')->default('string');
                $table->timestamps();
            });
        }

        // SEO Alerts table
        if (!Schema::hasTable('seo_alerts')) {
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
    }

    public function down(): void
    {
        Schema::dropIfExists('seo_alerts');
        Schema::dropIfExists('seo_settings');
        Schema::dropIfExists('backlinks');
        Schema::dropIfExists('rank_histories');
        Schema::dropIfExists('rank_tracking');
        Schema::dropIfExists('seo_analytics');
        Schema::dropIfExists('redirects');
    }
};
