<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

/**
 * Add Performance Indexes for Media Tables
 *
 * Google Enterprise Grade database optimization with:
 * - Single column indexes for frequently filtered fields
 * - Composite indexes for common query patterns
 * - Full-text indexes for search functionality
 * - Covering indexes for read-heavy operations
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
return new class extends Migration
{
    /**
     * Run the migrations
     */
    public function up(): void
    {
        // Media table indexes
        Schema::table('media', function (Blueprint $table) {
            // Single column indexes
            $table->index('is_optimized', 'idx_media_is_optimized');
            $table->index('collection', 'idx_media_collection');
            $table->index('uploaded_by', 'idx_media_uploaded_by');
            $table->index('mime_type', 'idx_media_mime_type');
            $table->index('disk', 'idx_media_disk');

            // Composite indexes for common query patterns
            $table->index(
                ['is_optimized', 'created_at'],
                'idx_media_optimization_queue'
            );
            $table->index(
                ['collection', 'created_at'],
                'idx_media_collection_sort'
            );
            $table->index(
                ['uploaded_by', 'created_at'],
                'idx_media_user_sort'
            );
            $table->index(
                ['mime_type', 'is_optimized'],
                'idx_media_type_optimization'
            );
        });

        // Add full-text index for search (MySQL)
        if (DB::getDriverName() === 'mysql') {
            DB::statement('ALTER TABLE media ADD FULLTEXT INDEX idx_media_fulltext (filename, alt_text, title)');
        }

        // Media variants table indexes
        Schema::table('media_variants', function (Blueprint $table) {
            $table->index('media_id', 'idx_variants_media_id');
            $table->index('variant_type', 'idx_variants_type');
            $table->index('format', 'idx_variants_format');
            $table->index(
                ['media_id', 'variant_type'],
                'idx_variants_media_type'
            );
        });

        // Image optimization jobs indexes
        Schema::table('image_optimization_jobs', function (Blueprint $table) {
            $table->index('status', 'idx_jobs_status');
            $table->index('priority', 'idx_jobs_priority');
            $table->index('media_id', 'idx_jobs_media_id');
            $table->index(
                ['status', 'priority', 'created_at'],
                'idx_jobs_queue_order'
            );
            $table->index(
                ['status', 'scheduled_at'],
                'idx_jobs_scheduled'
            );
        });

        // Image optimization presets - usually small table, minimal indexes
        Schema::table('image_optimization_presets', function (Blueprint $table) {
            $table->index('slug', 'idx_presets_slug');
            $table->index('is_active', 'idx_presets_active');
        });
    }

    /**
     * Reverse the migrations
     */
    public function down(): void
    {
        // Remove media table indexes
        Schema::table('media', function (Blueprint $table) {
            $table->dropIndex('idx_media_is_optimized');
            $table->dropIndex('idx_media_collection');
            $table->dropIndex('idx_media_uploaded_by');
            $table->dropIndex('idx_media_mime_type');
            $table->dropIndex('idx_media_disk');
            $table->dropIndex('idx_media_optimization_queue');
            $table->dropIndex('idx_media_collection_sort');
            $table->dropIndex('idx_media_user_sort');
            $table->dropIndex('idx_media_type_optimization');
        });

        // Drop full-text index (MySQL)
        if (DB::getDriverName() === 'mysql') {
            DB::statement('ALTER TABLE media DROP INDEX idx_media_fulltext');
        }

        // Remove media variants indexes
        Schema::table('media_variants', function (Blueprint $table) {
            $table->dropIndex('idx_variants_media_id');
            $table->dropIndex('idx_variants_type');
            $table->dropIndex('idx_variants_format');
            $table->dropIndex('idx_variants_media_type');
        });

        // Remove job indexes
        Schema::table('image_optimization_jobs', function (Blueprint $table) {
            $table->dropIndex('idx_jobs_status');
            $table->dropIndex('idx_jobs_priority');
            $table->dropIndex('idx_jobs_media_id');
            $table->dropIndex('idx_jobs_queue_order');
            $table->dropIndex('idx_jobs_scheduled');
        });

        // Remove preset indexes
        Schema::table('image_optimization_presets', function (Blueprint $table) {
            $table->dropIndex('idx_presets_slug');
            $table->dropIndex('idx_presets_active');
        });
    }
};
