<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Image Optimization Engine - Database Schema
 *
 * Enterprise-grade optimization tables for WebP/AVIF conversion,
 * responsive variants, and comprehensive media analytics.
 */
return new class extends Migration
{
    public function up(): void
    {
        // Update media table with optimization fields
        Schema::table('media', function (Blueprint $table) {
            // Core storage
            $table->string('disk')->default('public')->after('filename');
            $table->string('url')->nullable()->after('path');
            $table->string('cdn_url')->nullable()->after('url');

            // Media classification
            $table->string('type')->default('other')->after('mime_type');
            $table->string('category')->nullable()->after('type');
            $table->string('collection')->nullable()->after('category');

            // Extended metadata
            $table->text('caption')->nullable()->after('title');
            $table->text('description')->nullable()->after('caption');
            $table->json('metadata')->nullable()->after('description');
            $table->json('exif')->nullable()->after('metadata');

            // Dimensions
            $table->unsignedInteger('width')->nullable()->after('exif');
            $table->unsignedInteger('height')->nullable()->after('width');
            $table->decimal('aspect_ratio', 8, 4)->nullable()->after('height');
            $table->unsignedInteger('duration')->nullable()->after('aspect_ratio');

            // File integrity
            $table->string('hash', 64)->nullable()->after('duration');
            $table->index('hash');

            // Optimization status
            $table->boolean('is_optimized')->default(false)->after('hash');
            $table->boolean('is_processed')->default(false)->after('is_optimized');
            $table->string('processing_status')->default('pending')->after('is_processed');
            $table->timestamp('optimized_at')->nullable()->after('processing_status');
            $table->json('processing_log')->nullable()->after('optimized_at');

            // Variants (thumbnails, responsive sizes, formats)
            $table->json('variants')->nullable()->after('processing_log');
            $table->string('thumbnail_path')->nullable()->after('variants');
            $table->string('thumbnail_url')->nullable()->after('thumbnail_path');

            // Organization
            $table->json('tags')->nullable()->after('thumbnail_url');
            $table->boolean('is_public')->default(true)->after('tags');
            $table->boolean('is_featured')->default(false)->after('is_public');

            // Analytics
            $table->unsignedInteger('download_count')->default(0)->after('is_featured');
            $table->unsignedInteger('view_count')->default(0)->after('download_count');
            $table->unsignedInteger('usage_count')->default(0)->after('view_count');
            $table->timestamp('last_accessed_at')->nullable()->after('usage_count');

            // Source tracking
            $table->string('source')->default('upload')->after('last_accessed_at');
            $table->string('source_url')->nullable()->after('source');
            $table->string('license')->nullable()->after('source_url');
            $table->string('copyright')->nullable()->after('license');
            $table->string('credit')->nullable()->after('copyright');

            // SEO
            $table->json('seo')->nullable()->after('credit');

            // Audit
            $table->string('ip_address', 45)->nullable()->after('uploaded_by');
            $table->timestamp('expires_at')->nullable()->after('ip_address');
            $table->softDeletes();

            // Indexes for optimization queries
            $table->index('type');
            $table->index('is_optimized');
            $table->index('processing_status');
            $table->index('collection');
            $table->index(['type', 'is_optimized']);
        });

        // Image optimization settings and presets
        Schema::create('image_optimization_presets', function (Blueprint $table) {
            $table->id();
            $table->string('name')->unique();
            $table->string('slug')->unique();
            $table->text('description')->nullable();

            // Quality settings (0-100)
            $table->unsignedTinyInteger('quality_webp')->default(85);
            $table->unsignedTinyInteger('quality_avif')->default(80);
            $table->unsignedTinyInteger('quality_jpeg')->default(85);
            $table->unsignedTinyInteger('quality_png')->default(90);

            // Compression strategy
            $table->enum('compression_mode', ['lossless', 'lossy', 'auto'])->default('auto');
            $table->boolean('preserve_transparency')->default(true);
            $table->boolean('progressive_jpeg')->default(true);
            $table->boolean('interlaced_png')->default(true);

            // Format preferences
            $table->boolean('convert_to_webp')->default(true);
            $table->boolean('convert_to_avif')->default(false);
            $table->boolean('keep_original_format')->default(true);

            // Responsive sizes (JSON array of widths)
            $table->json('responsive_sizes')->nullable();
            $table->boolean('generate_retina')->default(true);
            $table->unsignedInteger('max_width')->nullable();
            $table->unsignedInteger('max_height')->nullable();

            // Metadata handling
            $table->boolean('strip_exif')->default(false);
            $table->boolean('preserve_icc_profile')->default(true);
            $table->boolean('preserve_copyright')->default(true);

            // Placeholder generation
            $table->boolean('generate_lqip')->default(true);
            $table->boolean('generate_blurhash')->default(true);
            $table->unsignedTinyInteger('lqip_quality')->default(20);
            $table->unsignedSmallInteger('lqip_width')->default(32);

            // Content-based optimization
            $table->enum('content_type', ['auto', 'photo', 'illustration', 'graphic', 'text', 'mixed'])->default('auto');

            $table->boolean('is_default')->default(false);
            $table->boolean('is_active')->default(true);
            $table->timestamps();
        });

        // Media variants (generated versions)
        Schema::create('media_variants', function (Blueprint $table) {
            $table->id();
            $table->foreignId('media_id')->constrained()->cascadeOnDelete();

            // Variant identification
            $table->string('variant_type'); // thumbnail, responsive, webp, avif, retina
            $table->string('size_name')->nullable(); // sm, md, lg, xl, 2x
            $table->unsignedInteger('width');
            $table->unsignedInteger('height');

            // File info
            $table->string('filename');
            $table->string('path');
            $table->string('disk')->default('public');
            $table->string('url')->nullable();
            $table->string('cdn_url')->nullable();
            $table->string('mime_type');
            $table->string('format'); // webp, avif, jpeg, png
            $table->unsignedBigInteger('size');

            // Optimization metrics
            $table->unsignedTinyInteger('quality')->nullable();
            $table->decimal('compression_ratio', 5, 2)->nullable();
            $table->unsignedInteger('original_size')->nullable();
            $table->integer('bytes_saved')->nullable();

            // Placeholder data
            $table->text('lqip_data')->nullable(); // Base64 LQIP
            $table->string('blurhash', 100)->nullable();

            // Retina support
            $table->boolean('is_retina')->default(false);
            $table->unsignedTinyInteger('pixel_density')->default(1);

            $table->timestamps();

            // Indexes
            $table->index(['media_id', 'variant_type']);
            $table->index(['media_id', 'size_name']);
            $table->index(['media_id', 'format']);
        });

        // Optimization queue tracking
        Schema::create('image_optimization_jobs', function (Blueprint $table) {
            $table->id();
            $table->foreignId('media_id')->constrained()->cascadeOnDelete();
            $table->foreignId('preset_id')->nullable()->constrained('image_optimization_presets')->nullOnDelete();

            // Job status
            $table->enum('status', ['pending', 'processing', 'completed', 'failed', 'cancelled'])->default('pending');
            $table->unsignedTinyInteger('priority')->default(5); // 1-10, lower = higher priority
            $table->unsignedSmallInteger('attempts')->default(0);
            $table->unsignedSmallInteger('max_attempts')->default(3);

            // Job configuration
            $table->json('operations')->nullable(); // Array of operations to perform
            $table->json('options')->nullable(); // Operation-specific options

            // Progress tracking
            $table->unsignedTinyInteger('progress')->default(0); // 0-100
            $table->string('current_operation')->nullable();

            // Performance metrics
            $table->unsignedInteger('processing_time_ms')->nullable();
            $table->unsignedBigInteger('memory_peak_bytes')->nullable();

            // Results
            $table->json('results')->nullable(); // Generated variants info
            $table->unsignedBigInteger('original_size')->nullable();
            $table->unsignedBigInteger('optimized_size')->nullable();
            $table->decimal('total_savings_percent', 5, 2)->nullable();

            // Error handling
            $table->text('error_message')->nullable();
            $table->text('error_trace')->nullable();

            // Timestamps
            $table->timestamp('started_at')->nullable();
            $table->timestamp('completed_at')->nullable();
            $table->timestamp('scheduled_at')->nullable();
            $table->timestamps();

            // Indexes
            $table->index('status');
            $table->index('priority');
            $table->index(['status', 'priority']);
        });

        // Media analytics
        Schema::create('media_analytics', function (Blueprint $table) {
            $table->id();
            $table->date('date');

            // Storage metrics
            $table->unsignedBigInteger('total_files')->default(0);
            $table->unsignedBigInteger('total_storage_bytes')->default(0);
            $table->unsignedBigInteger('optimized_storage_bytes')->default(0);
            $table->unsignedBigInteger('savings_bytes')->default(0);
            $table->decimal('savings_percent', 5, 2)->default(0);

            // File counts by type
            $table->unsignedInteger('images_count')->default(0);
            $table->unsignedInteger('videos_count')->default(0);
            $table->unsignedInteger('documents_count')->default(0);

            // Optimization metrics
            $table->unsignedInteger('optimized_count')->default(0);
            $table->unsignedInteger('pending_count')->default(0);
            $table->unsignedInteger('failed_count')->default(0);

            // Format breakdown
            $table->json('format_distribution')->nullable();
            $table->json('size_distribution')->nullable();

            // Bandwidth
            $table->unsignedBigInteger('bandwidth_saved_bytes')->default(0);
            $table->unsignedInteger('cdn_requests')->default(0);

            // Processing performance
            $table->unsignedInteger('jobs_processed')->default(0);
            $table->unsignedInteger('avg_processing_time_ms')->default(0);

            $table->timestamps();

            $table->unique('date');
            $table->index('date');
        });

        // CDN configuration
        Schema::create('cdn_configurations', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('provider'); // cloudflare, cloudinary, bunny, custom
            $table->string('base_url');
            $table->string('api_key')->nullable();
            $table->string('api_secret')->nullable();
            $table->json('settings')->nullable();
            $table->boolean('is_active')->default(true);
            $table->boolean('is_default')->default(false);
            $table->timestamps();
        });

        // Product-media pivot with optimization tracking
        Schema::create('product_media', function (Blueprint $table) {
            $table->foreignId('product_id')->constrained()->cascadeOnDelete();
            $table->foreignId('media_id')->constrained()->cascadeOnDelete();
            $table->unsignedSmallInteger('order')->default(0);
            $table->boolean('is_primary')->default(false);
            $table->string('variant_size')->nullable();
            $table->timestamps();

            $table->primary(['product_id', 'media_id']);
        });

        // Update post_media with additional fields
        Schema::table('post_media', function (Blueprint $table) {
            $table->unsignedSmallInteger('order')->default(0)->after('media_id');
            $table->text('caption')->nullable()->after('order');
            $table->string('alt_text')->nullable()->after('caption');
            $table->string('variant_size')->nullable()->after('alt_text');
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('media_analytics');
        Schema::dropIfExists('image_optimization_jobs');
        Schema::dropIfExists('media_variants');
        Schema::dropIfExists('image_optimization_presets');
        Schema::dropIfExists('cdn_configurations');
        Schema::dropIfExists('product_media');

        Schema::table('post_media', function (Blueprint $table) {
            $table->dropColumn(['order', 'caption', 'alt_text', 'variant_size', 'created_at', 'updated_at']);
        });

        Schema::table('media', function (Blueprint $table) {
            $table->dropIndex(['type']);
            $table->dropIndex(['is_optimized']);
            $table->dropIndex(['processing_status']);
            $table->dropIndex(['collection']);
            $table->dropIndex(['type', 'is_optimized']);
            $table->dropIndex(['hash']);

            $table->dropColumn([
                'disk', 'url', 'cdn_url', 'type', 'category', 'collection',
                'caption', 'description', 'metadata', 'exif', 'width', 'height',
                'aspect_ratio', 'duration', 'hash', 'is_optimized', 'is_processed',
                'processing_status', 'optimized_at', 'processing_log', 'variants',
                'thumbnail_path', 'thumbnail_url', 'tags', 'is_public', 'is_featured',
                'download_count', 'view_count', 'usage_count', 'last_accessed_at',
                'source', 'source_url', 'license', 'copyright', 'credit', 'seo',
                'ip_address', 'expires_at', 'deleted_at'
            ]);
        });
    }
};
