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
        // Form collaborators table
        Schema::create('form_collaborators', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('role')->default('viewer');
            $table->timestamp('invited_at')->nullable();
            $table->timestamp('accepted_at')->nullable();
            $table->timestamp('last_active_at')->nullable();
            $table->timestamps();

            $table->unique(['form_id', 'user_id']);
            $table->index(['form_id', 'role']);
        });

        // Form comments table
        Schema::create('form_comments', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('element_id')->nullable()->index();
            $table->text('content');
            $table->json('position')->nullable();
            $table->uuid('parent_id')->nullable();
            $table->boolean('resolved')->default(false);
            $table->foreignId('resolved_by')->nullable()->constrained('users');
            $table->timestamp('resolved_at')->nullable();
            $table->timestamps();

            $table->index(['form_id', 'element_id']);
            $table->index(['form_id', 'resolved']);
        });

        // Form activity log
        Schema::create('form_activity_log', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('action');
            $table->json('metadata')->nullable();
            $table->timestamp('created_at');

            $table->index(['form_id', 'created_at']);
            $table->index(['user_id', 'created_at']);
        });

        // Form notification logs
        Schema::create('form_notification_logs', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->foreignId('submission_id')->nullable()->constrained('form_submissions')->onDelete('cascade');
            $table->string('channel');
            $table->string('status');
            $table->text('error')->nullable();
            $table->timestamp('created_at');

            $table->index(['form_id', 'channel']);
            $table->index(['form_id', 'status']);
        });

        // Form embed analytics
        Schema::create('form_embed_analytics', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->string('embed_type');
            $table->string('referrer')->nullable();
            $table->string('user_agent')->nullable();
            $table->string('ip_hash', 64)->nullable();
            $table->timestamp('created_at');

            $table->index(['form_id', 'embed_type']);
            $table->index(['form_id', 'created_at']);
        });

        // Form short URLs
        Schema::create('form_short_urls', function (Blueprint $table) {
            $table->id();
            $table->string('code', 16)->unique();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->text('original_url');
            $table->unsignedBigInteger('clicks')->default(0);
            $table->timestamp('created_at');

            $table->index('code');
        });

        // Form search analytics
        Schema::create('form_search_analytics', function (Blueprint $table) {
            $table->id();
            $table->string('query');
            $table->date('date');
            $table->unsignedInteger('results_count')->default(0);
            $table->unsignedInteger('search_count')->default(1);
            $table->timestamps();

            $table->unique(['query', 'date']);
            $table->index(['date', 'search_count']);
        });

        // Related forms
        Schema::create('form_relations', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->foreignId('related_form_id')->constrained('forms')->onDelete('cascade');
            $table->string('relation_type')->default('similar'); // similar, sequel, category
            $table->float('score')->default(0);
            $table->boolean('manual')->default(false);
            $table->timestamps();

            $table->unique(['form_id', 'related_form_id']);
            $table->index(['form_id', 'score']);
        });

        // Add new columns to forms table
        Schema::table('forms', function (Blueprint $table) {
            if (!Schema::hasColumn('forms', 'version')) {
                $table->unsignedInteger('version')->default(1)->after('status');
            }
            if (!Schema::hasColumn('forms', 'seo_score')) {
                $table->unsignedTinyInteger('seo_score')->nullable()->after('version');
            }
            if (!Schema::hasColumn('forms', 'conversion_rate')) {
                $table->decimal('conversion_rate', 5, 2)->nullable()->after('seo_score');
            }
            if (!Schema::hasColumn('forms', 'tags')) {
                $table->json('tags')->nullable()->after('conversion_rate');
            }
            if (!Schema::hasColumn('forms', 'category')) {
                $table->string('category')->nullable()->after('tags');
            }
        });

        // Add indexes for search optimization
        if (Schema::hasColumn('forms', 'title')) {
            Schema::table('forms', function (Blueprint $table) {
                $table->fullText(['title', 'description'], 'forms_fulltext')->change();
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('form_relations');
        Schema::dropIfExists('form_search_analytics');
        Schema::dropIfExists('form_short_urls');
        Schema::dropIfExists('form_embed_analytics');
        Schema::dropIfExists('form_notification_logs');
        Schema::dropIfExists('form_activity_log');
        Schema::dropIfExists('form_comments');
        Schema::dropIfExists('form_collaborators');

        Schema::table('forms', function (Blueprint $table) {
            $columns = ['version', 'seo_score', 'conversion_rate', 'tags', 'category'];
            foreach ($columns as $column) {
                if (Schema::hasColumn('forms', $column)) {
                    $table->dropColumn($column);
                }
            }
        });
    }
};
