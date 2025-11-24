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
        if (!Schema::hasTable('posts')) {
            return;
        }
        
        Schema::table('posts', function (Blueprint $table) {
            // Categories and tags
            $table->json('categories')->nullable();
            $table->json('tags')->nullable();
            
            // Analytics and engagement
            $table->unsignedBigInteger('view_count')->default(0);
            $table->unsignedInteger('comment_count')->default(0);
            $table->unsignedInteger('share_count')->default(0);
            $table->unsignedInteger('like_count')->default(0);
            $table->decimal('engagement_rate', 5, 2)->default(0);
            $table->decimal('ctr', 5, 2)->default(0);
            $table->decimal('avg_time_on_page', 8, 2)->default(0);
            
            // SEO metrics
            $table->decimal('seo_score', 5, 2)->default(0);
            $table->json('keywords')->nullable();
            $table->integer('word_count')->default(0);
            $table->decimal('readability_score', 5, 2)->default(0);
            
            // Scheduling
            $table->timestamp('scheduled_at')->nullable();
            $table->boolean('auto_publish')->default(false);
            
            // Visibility and access
            $table->boolean('is_featured')->default(false)->index();
            $table->boolean('is_pinned')->default(false)->index();
            $table->boolean('allow_comments')->default(true);
            $table->string('visibility', 20)->default('public'); // public, private, password
            $table->string('password')->nullable();
            
            // Content metadata
            $table->string('reading_time')->nullable();
            $table->json('related_posts')->nullable();
            $table->json('custom_fields')->nullable();
            
            // Versioning
            $table->integer('version')->default(1);
            $table->timestamp('last_edited_at')->nullable();
            $table->foreignId('last_edited_by')->nullable()->constrained('users');
            
            // Social media
            $table->string('og_image')->nullable();
            $table->string('twitter_card')->default('summary_large_image');
            
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
