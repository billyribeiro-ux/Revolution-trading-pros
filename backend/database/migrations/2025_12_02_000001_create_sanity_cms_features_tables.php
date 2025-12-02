<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Sanity-Equivalent CMS Features Migration
 *
 * Implements all Sanity.io CMS features that don't exist in the current system:
 * - Portable Text with annotations
 * - Document perspectives (draft/published)
 * - Document history with diffs
 * - Release bundles
 * - Webhooks system
 * - Strong references
 * - Schema definitions
 * - Real-time collaboration
 * - Image hotspots/crops
 * - Live preview tokens
 */
return new class extends Migration
{
    public function up(): void
    {
        // ═══════════════════════════════════════════════════════════════════
        // 1. DOCUMENT PERSPECTIVES (Draft/Published Pairs)
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('document_perspectives', function (Blueprint $table) {
            $table->id();
            $table->uuid('document_id')->unique();
            $table->string('document_type', 50); // post, page, product, etc.
            $table->unsignedBigInteger('source_id'); // Original model ID
            $table->enum('perspective', ['draft', 'published'])->default('draft');
            $table->json('draft_content')->nullable(); // Full draft document
            $table->json('published_content')->nullable(); // Full published document
            $table->timestamp('draft_updated_at')->nullable();
            $table->timestamp('published_at')->nullable();
            $table->unsignedBigInteger('draft_by')->nullable();
            $table->unsignedBigInteger('published_by')->nullable();
            $table->boolean('has_unpublished_changes')->default(false);
            $table->string('preview_token', 64)->nullable()->unique();
            $table->timestamp('preview_token_expires_at')->nullable();
            $table->timestamps();

            $table->index(['document_type', 'source_id']);
            $table->index('preview_token');
            $table->foreign('draft_by')->references('id')->on('users')->nullOnDelete();
            $table->foreign('published_by')->references('id')->on('users')->nullOnDelete();
        });

        // ═══════════════════════════════════════════════════════════════════
        // 2. DOCUMENT HISTORY (Full Revision History with Diffs)
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('document_history', function (Blueprint $table) {
            $table->id();
            $table->uuid('history_id')->unique();
            $table->uuid('document_id');
            $table->string('document_type', 50);
            $table->unsignedBigInteger('revision_number');
            $table->json('snapshot'); // Full document at this point
            $table->json('patches')->nullable(); // JSON Patch operations from previous
            $table->json('diff')->nullable(); // Human-readable diff
            $table->string('change_summary', 500)->nullable();
            $table->enum('action', ['create', 'update', 'publish', 'unpublish', 'restore', 'delete']);
            $table->unsignedBigInteger('user_id')->nullable();
            $table->string('user_name')->nullable(); // Denormalized for history
            $table->ipAddress('ip_address')->nullable();
            $table->string('user_agent', 500)->nullable();
            $table->timestamp('created_at');

            $table->index(['document_id', 'revision_number']);
            $table->index(['document_type', 'created_at']);
            $table->foreign('user_id')->references('id')->on('users')->nullOnDelete();
        });

        // ═══════════════════════════════════════════════════════════════════
        // 3. RELEASE BUNDLES (Grouped Document Publishing)
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('release_bundles', function (Blueprint $table) {
            $table->id();
            $table->uuid('bundle_id')->unique();
            $table->string('name');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->enum('status', ['draft', 'scheduled', 'publishing', 'published', 'failed', 'cancelled'])->default('draft');
            $table->timestamp('scheduled_at')->nullable();
            $table->timestamp('published_at')->nullable();
            $table->unsignedBigInteger('created_by');
            $table->unsignedBigInteger('published_by')->nullable();
            $table->json('metadata')->nullable();
            $table->json('publish_log')->nullable(); // Log of publish operations
            $table->timestamps();
            $table->softDeletes();

            $table->index(['status', 'scheduled_at']);
            $table->foreign('created_by')->references('id')->on('users');
            $table->foreign('published_by')->references('id')->on('users')->nullOnDelete();
        });

        Schema::create('release_bundle_documents', function (Blueprint $table) {
            $table->id();
            $table->uuid('bundle_id');
            $table->uuid('document_id');
            $table->string('document_type', 50);
            $table->enum('action', ['create', 'update', 'delete'])->default('update');
            $table->json('snapshot')->nullable(); // Document state to publish
            $table->unsignedInteger('order')->default(0);
            $table->enum('status', ['pending', 'published', 'failed'])->default('pending');
            $table->text('error_message')->nullable();
            $table->timestamps();

            $table->unique(['bundle_id', 'document_id']);
            $table->index('document_id');
        });

        // ═══════════════════════════════════════════════════════════════════
        // 4. WEBHOOKS SYSTEM
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('webhooks', function (Blueprint $table) {
            $table->id();
            $table->uuid('webhook_id')->unique();
            $table->string('name');
            $table->string('url', 2048);
            $table->string('secret', 64)->nullable();
            $table->json('events'); // Array of event types to trigger on
            $table->json('filter')->nullable(); // GROQ-like filter for documents
            $table->json('projection')->nullable(); // Fields to include in payload
            $table->enum('http_method', ['POST', 'PUT', 'PATCH'])->default('POST');
            $table->json('headers')->nullable(); // Custom headers
            $table->boolean('is_active')->default(true);
            $table->unsignedInteger('timeout_ms')->default(30000);
            $table->unsignedInteger('max_retries')->default(3);
            $table->unsignedInteger('retry_delay_ms')->default(1000);
            $table->unsignedBigInteger('created_by');
            $table->timestamps();
            $table->softDeletes();

            $table->index(['is_active', 'events']);
            $table->foreign('created_by')->references('id')->on('users');
        });

        Schema::create('webhook_deliveries', function (Blueprint $table) {
            $table->id();
            $table->uuid('delivery_id')->unique();
            $table->foreignId('webhook_id')->constrained()->cascadeOnDelete();
            $table->string('event_type', 100);
            $table->uuid('document_id')->nullable();
            $table->string('document_type', 50)->nullable();
            $table->json('payload');
            $table->json('response_headers')->nullable();
            $table->text('response_body')->nullable();
            $table->unsignedSmallInteger('response_status')->nullable();
            $table->enum('status', ['pending', 'success', 'failed', 'retrying'])->default('pending');
            $table->unsignedTinyInteger('attempt_count')->default(0);
            $table->timestamp('next_retry_at')->nullable();
            $table->unsignedInteger('duration_ms')->nullable();
            $table->text('error_message')->nullable();
            $table->timestamp('delivered_at')->nullable();
            $table->timestamps();

            $table->index(['webhook_id', 'status']);
            $table->index(['status', 'next_retry_at']);
            $table->index('created_at');
        });

        // ═══════════════════════════════════════════════════════════════════
        // 5. STRONG REFERENCES (Cross-Document Links)
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('document_references', function (Blueprint $table) {
            $table->id();
            $table->uuid('source_document_id');
            $table->string('source_document_type', 50);
            $table->string('source_field_path', 255); // JSON path to field containing ref
            $table->uuid('target_document_id');
            $table->string('target_document_type', 50);
            $table->enum('reference_type', ['strong', 'weak'])->default('strong');
            $table->boolean('is_array_item')->default(false);
            $table->unsignedInteger('array_index')->nullable();
            $table->timestamps();

            $table->unique(['source_document_id', 'source_field_path', 'target_document_id', 'array_index'], 'unique_reference');
            $table->index('source_document_id');
            $table->index('target_document_id');
            $table->index(['target_document_id', 'reference_type']);
        });

        // ═══════════════════════════════════════════════════════════════════
        // 6. SCHEMA DEFINITIONS (Schema as Code)
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('content_schemas', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100)->unique(); // e.g., 'post', 'page', 'author'
            $table->string('title');
            $table->string('description', 500)->nullable();
            $table->string('icon', 50)->nullable();
            $table->json('fields'); // Field definitions
            $table->json('fieldsets')->nullable(); // Field groupings
            $table->json('preview')->nullable(); // Preview configuration
            $table->json('orderings')->nullable(); // Sort options
            $table->json('validation')->nullable(); // Schema-level validation
            $table->json('initial_value')->nullable(); // Default values
            $table->boolean('is_document')->default(true); // true = document, false = object
            $table->boolean('is_singleton')->default(false);
            $table->boolean('is_system')->default(false);
            $table->unsignedInteger('version')->default(1);
            $table->timestamps();

            $table->index(['is_document', 'name']);
        });

        // ═══════════════════════════════════════════════════════════════════
        // 7. PORTABLE TEXT ANNOTATIONS
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('portable_text_annotations', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100)->unique();
            $table->string('title');
            $table->string('description', 500)->nullable();
            $table->string('icon', 50)->nullable();
            $table->json('fields'); // Annotation field definitions
            $table->json('component')->nullable(); // Render component config
            $table->boolean('is_active')->default(true);
            $table->timestamps();
        });

        Schema::create('portable_text_block_types', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100)->unique();
            $table->string('title');
            $table->string('description', 500)->nullable();
            $table->string('icon', 50)->nullable();
            $table->json('fields'); // Block type fields
            $table->json('preview')->nullable();
            $table->json('component')->nullable();
            $table->boolean('is_active')->default(true);
            $table->unsignedInteger('order')->default(0);
            $table->timestamps();
        });

        // ═══════════════════════════════════════════════════════════════════
        // 8. REAL-TIME COLLABORATION
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('document_presence', function (Blueprint $table) {
            $table->id();
            $table->uuid('document_id');
            $table->string('document_type', 50);
            $table->unsignedBigInteger('user_id');
            $table->string('session_id', 64);
            $table->json('cursor_position')->nullable(); // {path: [...], offset: n}
            $table->json('selection')->nullable(); // {anchor: {...}, focus: {...}}
            $table->string('color', 7)->nullable(); // User color for presence
            $table->timestamp('last_heartbeat_at');
            $table->timestamps();

            $table->unique(['document_id', 'user_id', 'session_id']);
            $table->index(['document_id', 'last_heartbeat_at']);
            $table->foreign('user_id')->references('id')->on('users')->cascadeOnDelete();
        });

        Schema::create('document_mutations', function (Blueprint $table) {
            $table->id();
            $table->uuid('mutation_id')->unique();
            $table->uuid('document_id');
            $table->string('document_type', 50);
            $table->unsignedBigInteger('user_id');
            $table->string('session_id', 64);
            $table->json('patches'); // JSON Patch operations
            $table->unsignedBigInteger('base_revision');
            $table->unsignedBigInteger('result_revision')->nullable();
            $table->enum('status', ['pending', 'applied', 'rejected', 'conflict'])->default('pending');
            $table->json('conflict_info')->nullable();
            $table->timestamp('applied_at')->nullable();
            $table->timestamps();

            $table->index(['document_id', 'base_revision']);
            $table->index(['document_id', 'status']);
            $table->foreign('user_id')->references('id')->on('users');
        });

        Schema::create('document_locks', function (Blueprint $table) {
            $table->id();
            $table->uuid('document_id');
            $table->string('document_type', 50);
            $table->string('field_path', 255)->nullable(); // null = whole document
            $table->unsignedBigInteger('user_id');
            $table->string('session_id', 64);
            $table->enum('lock_type', ['exclusive', 'shared'])->default('exclusive');
            $table->timestamp('expires_at');
            $table->timestamps();

            $table->unique(['document_id', 'field_path']);
            $table->index('expires_at');
            $table->foreign('user_id')->references('id')->on('users')->cascadeOnDelete();
        });

        // ═══════════════════════════════════════════════════════════════════
        // 9. IMAGE HOTSPOTS & CROPS
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('image_hotspots', function (Blueprint $table) {
            $table->id();
            $table->foreignId('media_id')->constrained()->cascadeOnDelete();
            $table->decimal('x', 5, 4); // 0.0 - 1.0 (percentage)
            $table->decimal('y', 5, 4); // 0.0 - 1.0 (percentage)
            $table->decimal('width', 5, 4)->nullable(); // For ellipse hotspot
            $table->decimal('height', 5, 4)->nullable();
            $table->boolean('is_default')->default(true);
            $table->string('name', 100)->nullable();
            $table->timestamps();

            $table->index(['media_id', 'is_default']);
        });

        Schema::create('image_crops', function (Blueprint $table) {
            $table->id();
            $table->foreignId('media_id')->constrained()->cascadeOnDelete();
            $table->string('name', 100); // e.g., 'square', '16:9', 'portrait'
            $table->string('aspect_ratio', 20)->nullable(); // e.g., '16:9', '1:1'
            $table->decimal('top', 5, 4); // 0.0 - 1.0
            $table->decimal('left', 5, 4);
            $table->decimal('bottom', 5, 4);
            $table->decimal('right', 5, 4);
            $table->boolean('is_default')->default(false);
            $table->timestamps();

            $table->unique(['media_id', 'name']);
            $table->index(['media_id', 'aspect_ratio']);
        });

        // ═══════════════════════════════════════════════════════════════════
        // 10. LIVE PREVIEW SESSIONS
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('preview_sessions', function (Blueprint $table) {
            $table->id();
            $table->string('token', 64)->unique();
            $table->uuid('document_id')->nullable(); // null = all drafts
            $table->string('document_type', 50)->nullable();
            $table->unsignedBigInteger('user_id');
            $table->enum('perspective', ['draft', 'published', 'previewDrafts'])->default('previewDrafts');
            $table->json('allowed_documents')->nullable(); // Array of document IDs
            $table->json('config')->nullable(); // Preview configuration
            $table->timestamp('expires_at');
            $table->timestamps();

            $table->index(['token', 'expires_at']);
            $table->foreign('user_id')->references('id')->on('users')->cascadeOnDelete();
        });

        // ═══════════════════════════════════════════════════════════════════
        // 11. GROQ QUERY CACHE
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('groq_query_cache', function (Blueprint $table) {
            $table->id();
            $table->string('query_hash', 64)->unique();
            $table->text('query');
            $table->json('params')->nullable();
            $table->json('result');
            $table->unsignedInteger('execution_time_ms');
            $table->timestamp('cached_at');
            $table->timestamp('expires_at');
            $table->unsignedInteger('hit_count')->default(0);

            $table->index('expires_at');
            $table->index(['query_hash', 'expires_at']);
        });

        // ═══════════════════════════════════════════════════════════════════
        // 12. DOCUMENT ACTIONS
        // ═══════════════════════════════════════════════════════════════════
        Schema::create('document_actions', function (Blueprint $table) {
            $table->id();
            $table->string('name', 100)->unique();
            $table->string('title');
            $table->string('description', 500)->nullable();
            $table->string('icon', 50)->nullable();
            $table->json('schema_types'); // Array of schema types this applies to
            $table->json('conditions')->nullable(); // When to show the action
            $table->json('handler'); // Action handler configuration
            $table->json('confirmation')->nullable(); // Confirmation dialog config
            $table->boolean('is_destructive')->default(false);
            $table->boolean('requires_permission')->default(false);
            $table->string('permission_name')->nullable();
            $table->unsignedInteger('order')->default(0);
            $table->boolean('is_active')->default(true);
            $table->timestamps();
        });

        Schema::create('document_action_logs', function (Blueprint $table) {
            $table->id();
            $table->uuid('log_id')->unique();
            $table->string('action_name', 100);
            $table->uuid('document_id');
            $table->string('document_type', 50);
            $table->unsignedBigInteger('user_id');
            $table->json('input')->nullable();
            $table->json('result')->nullable();
            $table->enum('status', ['success', 'failed', 'cancelled'])->default('success');
            $table->text('error_message')->nullable();
            $table->unsignedInteger('duration_ms')->nullable();
            $table->timestamps();

            $table->index(['document_id', 'created_at']);
            $table->index(['action_name', 'created_at']);
            $table->foreign('user_id')->references('id')->on('users');
        });

        // Add hotspot columns to existing media table
        if (Schema::hasTable('media')) {
            Schema::table('media', function (Blueprint $table) {
                if (!Schema::hasColumn('media', 'hotspot_x')) {
                    $table->decimal('hotspot_x', 5, 4)->nullable()->after('height');
                }
                if (!Schema::hasColumn('media', 'hotspot_y')) {
                    $table->decimal('hotspot_y', 5, 4)->nullable()->after('hotspot_x');
                }
                if (!Schema::hasColumn('media', 'crop_data')) {
                    $table->json('crop_data')->nullable()->after('hotspot_y');
                }
                if (!Schema::hasColumn('media', 'lqip')) {
                    $table->text('lqip')->nullable()->after('crop_data'); // Low-quality image placeholder
                }
                if (!Schema::hasColumn('media', 'blurhash')) {
                    $table->string('blurhash', 100)->nullable()->after('lqip');
                }
            });
        }

        // Add document_id to posts for perspective linking
        if (Schema::hasTable('posts')) {
            Schema::table('posts', function (Blueprint $table) {
                if (!Schema::hasColumn('posts', 'document_id')) {
                    $table->uuid('document_id')->nullable()->unique()->after('id');
                }
                if (!Schema::hasColumn('posts', 'portable_text')) {
                    $table->json('portable_text')->nullable()->after('content_blocks');
                }
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('document_action_logs');
        Schema::dropIfExists('document_actions');
        Schema::dropIfExists('groq_query_cache');
        Schema::dropIfExists('preview_sessions');
        Schema::dropIfExists('image_crops');
        Schema::dropIfExists('image_hotspots');
        Schema::dropIfExists('document_locks');
        Schema::dropIfExists('document_mutations');
        Schema::dropIfExists('document_presence');
        Schema::dropIfExists('portable_text_block_types');
        Schema::dropIfExists('portable_text_annotations');
        Schema::dropIfExists('content_schemas');
        Schema::dropIfExists('document_references');
        Schema::dropIfExists('webhook_deliveries');
        Schema::dropIfExists('webhooks');
        Schema::dropIfExists('release_bundle_documents');
        Schema::dropIfExists('release_bundles');
        Schema::dropIfExists('document_history');
        Schema::dropIfExists('document_perspectives');

        if (Schema::hasTable('media')) {
            Schema::table('media', function (Blueprint $table) {
                $table->dropColumn(['hotspot_x', 'hotspot_y', 'crop_data', 'lqip', 'blurhash']);
            });
        }

        if (Schema::hasTable('posts')) {
            Schema::table('posts', function (Blueprint $table) {
                $table->dropColumn(['document_id', 'portable_text']);
            });
        }
    }
};
