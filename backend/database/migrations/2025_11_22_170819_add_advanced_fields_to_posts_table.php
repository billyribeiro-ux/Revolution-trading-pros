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
        Schema::table('posts', function (Blueprint $table) {
            // Extend status enum to include 'scheduled'
            $table->string('status', 50)->change();
            
            // Categories and tags
            $table->json('categories')->nullable()->after('content_blocks');
            $table->json('tags')->nullable()->after('categories');
            
            // Analytics and engagement
            $table->unsignedBigInteger('view_count')->default(0)->after('published_at');
            $table->unsignedInteger('comment_count')->default(0)->after('view_count');
            $table->unsignedInteger('share_count')->default(0)->after('comment_count');
            $table->unsignedInteger('like_count')->default(0)->after('share_count');
            $table->decimal('engagement_rate', 5, 2)->default(0)->after('like_count');
            $table->decimal('ctr', 5, 2)->default(0)->after('engagement_rate');
            $table->decimal('avg_time_on_page', 8, 2)->default(0)->after('ctr');
            
            // SEO metrics
            $table->decimal('seo_score', 5, 2)->default(0)->after('schema_markup');
            $table->json('keywords')->nullable()->after('seo_score');
            $table->integer('word_count')->default(0)->after('keywords');
            $table->decimal('readability_score', 5, 2)->default(0)->after('word_count');
            
            // Scheduling
            $table->timestamp('scheduled_at')->nullable()->after('published_at');
            $table->boolean('auto_publish')->default(false)->after('scheduled_at');
            
            // Visibility and access
            $table->boolean('is_featured')->default(false)->index()->after('status');
            $table->boolean('is_pinned')->default(false)->index()->after('is_featured');
            $table->boolean('allow_comments')->default(true)->after('is_pinned');
            $table->string('visibility', 20)->default('public')->after('allow_comments'); // public, private, password
            $table->string('password')->nullable()->after('visibility');
            
            // Content metadata
            $table->string('reading_time')->nullable()->after('excerpt');
            $table->json('related_posts')->nullable()->after('tags');
            $table->json('custom_fields')->nullable()->after('related_posts');
            
            // Versioning
            $table->integer('version')->default(1)->after('custom_fields');
            $table->timestamp('last_edited_at')->nullable()->after('version');
            $table->foreignId('last_edited_by')->nullable()->constrained('users')->after('last_edited_at');
            
            // Social media
            $table->string('og_image')->nullable()->after('featured_image');
            $table->string('twitter_card')->default('summary_large_image')->after('og_image');
            
            // Add indexes for performance
            $table->index('view_count');
            $table->index('engagement_rate');
            $table->index('seo_score');
            $table->index('scheduled_at');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('posts', function (Blueprint $table) {
            $table->dropColumn([
                'categories',
                'tags',
                'view_count',
                'comment_count',
                'share_count',
                'like_count',
                'engagement_rate',
                'ctr',
                'avg_time_on_page',
                'seo_score',
                'keywords',
                'word_count',
                'readability_score',
                'scheduled_at',
                'auto_publish',
                'is_featured',
                'is_pinned',
                'allow_comments',
                'visibility',
                'password',
                'reading_time',
                'related_posts',
                'custom_fields',
                'version',
                'last_edited_at',
                'last_edited_by',
                'og_image',
                'twitter_card',
            ]);
        });
    }
};
