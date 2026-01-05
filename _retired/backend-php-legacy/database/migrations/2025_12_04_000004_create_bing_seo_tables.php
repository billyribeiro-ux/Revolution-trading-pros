<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Create Bing SEO tables for URL submission tracking
 */
return new class extends Migration
{
    public function up(): void
    {
        // Bing URL Submissions tracking
        if (!Schema::hasTable('bing_url_submissions')) {
            Schema::create('bing_url_submissions', function (Blueprint $table) {
                $table->id();
                $table->string('url', 2048);
                $table->enum('submission_type', ['indexnow', 'indexnow_batch', 'bing_webmaster', 'sitemap']);
                $table->boolean('success')->default(false);
                $table->unsignedSmallInteger('status_code')->nullable();
                $table->string('response_message', 500)->nullable();
                $table->timestamp('created_at');

                $table->index(['submission_type', 'created_at']);
                $table->index(['success', 'created_at']);
                $table->index('created_at');
            });
        }

        // Bing Search Performance tracking (for Webmaster API data)
        if (!Schema::hasTable('bing_search_performance')) {
            Schema::create('bing_search_performance', function (Blueprint $table) {
                $table->id();
                $table->date('date');
                $table->string('query', 500)->nullable();
                $table->string('page_url', 2048)->nullable();
                $table->unsignedInteger('impressions')->default(0);
                $table->unsignedInteger('clicks')->default(0);
                $table->decimal('ctr', 5, 2)->default(0); // Click-through rate
                $table->decimal('position', 5, 2)->nullable(); // Average position
                $table->timestamp('created_at');

                $table->unique(['date', 'query', 'page_url']);
                $table->index(['date', 'page_url']);
                $table->index('date');
            });
        }

        // SEO Performance Metrics (combined Google + Bing)
        if (!Schema::hasTable('seo_performance_metrics')) {
            Schema::create('seo_performance_metrics', function (Blueprint $table) {
                $table->id();
                $table->date('date');
                $table->enum('source', ['google', 'bing', 'combined']);
                $table->string('metric_type', 50); // impressions, clicks, position, etc.
                $table->string('dimension', 100)->nullable(); // query, page, country, device
                $table->string('dimension_value', 500)->nullable();
                $table->decimal('value', 15, 4);
                $table->json('metadata')->nullable();
                $table->timestamps();

                $table->index(['date', 'source', 'metric_type']);
                $table->index(['dimension', 'dimension_value']);
            });
        }

        // Core Web Vitals tracking
        if (!Schema::hasTable('core_web_vitals')) {
            Schema::create('core_web_vitals', function (Blueprint $table) {
                $table->id();
                $table->string('url', 2048);
                $table->string('device', 20); // mobile, desktop
                $table->decimal('lcp', 8, 2)->nullable(); // Largest Contentful Paint (ms)
                $table->decimal('fid', 8, 2)->nullable(); // First Input Delay (ms)
                $table->decimal('cls', 8, 4)->nullable(); // Cumulative Layout Shift
                $table->decimal('inp', 8, 2)->nullable(); // Interaction to Next Paint (ms)
                $table->decimal('ttfb', 8, 2)->nullable(); // Time to First Byte (ms)
                $table->decimal('fcp', 8, 2)->nullable(); // First Contentful Paint (ms)
                $table->enum('lcp_rating', ['good', 'needs_improvement', 'poor'])->nullable();
                $table->enum('fid_rating', ['good', 'needs_improvement', 'poor'])->nullable();
                $table->enum('cls_rating', ['good', 'needs_improvement', 'poor'])->nullable();
                $table->enum('inp_rating', ['good', 'needs_improvement', 'poor'])->nullable();
                $table->unsignedTinyInteger('overall_score')->nullable(); // 0-100
                $table->json('metadata')->nullable();
                $table->timestamps();

                $table->index(['url', 'device', 'created_at']);
                $table->index('created_at');
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('bing_url_submissions');
        Schema::dropIfExists('bing_search_performance');
        Schema::dropIfExists('seo_performance_metrics');
        Schema::dropIfExists('core_web_vitals');
    }
};
