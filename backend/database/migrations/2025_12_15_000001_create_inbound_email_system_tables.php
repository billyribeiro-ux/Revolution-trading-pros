<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

/**
 * Create Inbound Email System Tables
 *
 * Enterprise-grade inbound email processing system for CRM Pro.
 * Supports conversation threading, attachment storage, and contact linking.
 *
 * Tables created:
 * - email_conversations: Thread/conversation grouping
 * - email_messages: Individual inbound/outbound messages
 * - email_attachments: Secure attachment storage metadata
 * - email_thread_mappings: Email header threading correlation
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // Email Conversations Table - Thread/conversation grouping
        if (!Schema::hasTable('email_conversations')) {
            Schema::create('email_conversations', function (Blueprint $table): void {
                $table->uuid('id')->primary();
                $table->foreignId('contact_id')
                    ->constrained('contacts')
                    ->cascadeOnDelete();
                $table->foreignId('assigned_to')
                    ->nullable()
                    ->constrained('users')
                    ->nullOnDelete();
                $table->string('subject', 998); // RFC 2822 max subject length
                $table->string('thread_id', 255)->unique()->index();
                $table->string('mailbox_hash', 255)->nullable()->index();
                $table->enum('status', [
                    'open',
                    'pending',
                    'resolved',
                    'archived',
                    'spam',
                ])->default('open');
                $table->enum('priority', [
                    'low',
                    'normal',
                    'high',
                    'urgent',
                ])->default('normal');
                $table->string('channel', 50)->default('email');
                $table->unsignedInteger('message_count')->default(0);
                $table->unsignedInteger('unread_count')->default(0);
                $table->timestamp('first_message_at')->nullable();
                $table->timestamp('latest_message_at')->nullable()->index();
                $table->timestamp('first_response_at')->nullable();
                $table->timestamp('resolved_at')->nullable();
                $table->unsignedInteger('response_time_seconds')->nullable();
                $table->json('participants')->nullable();
                $table->json('tags')->nullable();
                $table->json('metadata')->nullable();
                $table->boolean('is_starred')->default(false);
                $table->boolean('is_muted')->default(false);
                $table->foreignId('created_by')
                    ->nullable()
                    ->constrained('users')
                    ->nullOnDelete();
                $table->timestamps();
                $table->softDeletes();

                // Performance indexes
                $table->index(['contact_id', 'status']);
                $table->index(['assigned_to', 'status']);
                $table->index(['status', 'priority', 'latest_message_at']);
                $table->index('created_at');
            });
        }

        // Email Messages Table - Individual messages (inbound and outbound)
        if (!Schema::hasTable('email_messages')) {
            Schema::create('email_messages', function (Blueprint $table): void {
                $table->uuid('id')->primary();
                $table->uuid('conversation_id');
                $table->foreign('conversation_id')
                    ->references('id')
                    ->on('email_conversations')
                    ->cascadeOnDelete();
                $table->string('message_id', 995)->unique(); // RFC 2822 message-id
                $table->string('provider_message_id', 255)->nullable()->index();
                $table->string('in_reply_to', 995)->nullable();
                $table->json('references')->nullable();
                $table->string('from_email', 254); // RFC 5321 max email length
                $table->string('from_name', 255)->nullable();
                $table->string('to_email', 254);
                $table->string('to_name', 255)->nullable();
                $table->json('cc')->nullable();
                $table->json('bcc')->nullable();
                $table->string('reply_to_email', 254)->nullable();
                $table->string('reply_to_name', 255)->nullable();
                $table->string('subject', 998);
                $table->longText('body_text')->nullable();
                $table->longText('body_html')->nullable();
                $table->text('body_preview')->nullable(); // First 500 chars
                $table->enum('direction', ['inbound', 'outbound'])->index();
                $table->enum('status', [
                    'pending',
                    'processing',
                    'processed',
                    'failed',
                    'spam',
                    'quarantined',
                ])->default('pending');
                $table->boolean('is_read')->default(false);
                $table->boolean('is_starred')->default(false);
                $table->boolean('is_draft')->default(false);
                $table->boolean('is_automated')->default(false);

                // Spam and security analysis
                $table->decimal('spam_score', 5, 2)->nullable();
                $table->enum('spam_verdict', ['pass', 'fail', 'neutral'])->nullable();
                $table->enum('virus_verdict', ['pass', 'fail', 'neutral'])->nullable();
                $table->enum('spf_verdict', ['pass', 'fail', 'neutral', 'none'])->nullable();
                $table->enum('dkim_verdict', ['pass', 'fail', 'neutral', 'none'])->nullable();
                $table->enum('dmarc_verdict', ['pass', 'fail', 'neutral', 'none'])->nullable();
                $table->json('authentication_results')->nullable();
                $table->json('spam_report')->nullable();

                // Email metadata
                $table->json('headers')->nullable();
                $table->unsignedInteger('size_bytes')->nullable();
                $table->unsignedSmallInteger('attachment_count')->default(0);
                $table->string('content_type', 100)->nullable();
                $table->string('charset', 50)->nullable();

                // Tracking
                $table->string('ip_address', 45)->nullable();
                $table->string('user_agent', 500)->nullable();
                $table->json('geo_location')->nullable();

                // Provider info
                $table->string('provider', 50)->default('postmark');
                $table->json('provider_data')->nullable();

                // Processing metadata
                $table->unsignedInteger('processing_time_ms')->nullable();
                $table->string('error_code', 50)->nullable();
                $table->text('error_message')->nullable();
                $table->unsignedTinyInteger('retry_count')->default(0);
                $table->timestamp('processed_at')->nullable();

                // Timestamps
                $table->timestamp('sent_at')->nullable();
                $table->timestamp('received_at')->nullable();
                $table->timestamp('read_at')->nullable();
                $table->timestamps();
                $table->softDeletes();

                // Performance indexes
                $table->index(['conversation_id', 'created_at']);
                $table->index(['from_email', 'created_at']);
                $table->index(['to_email', 'created_at']);
                $table->index(['status', 'direction']);
                $table->index('received_at');
                $table->index('in_reply_to');
            });
        }

        // Email Attachments Table - Secure attachment storage metadata
        if (!Schema::hasTable('email_attachments')) {
            Schema::create('email_attachments', function (Blueprint $table): void {
                $table->uuid('id')->primary();
                $table->uuid('message_id');
                $table->foreign('message_id')
                    ->references('id')
                    ->on('email_messages')
                    ->cascadeOnDelete();
                $table->string('filename', 255);
                $table->string('original_filename', 255);
                $table->string('content_type', 127); // RFC 4288 max MIME type length
                $table->string('content_id', 255)->nullable(); // For inline images
                $table->enum('disposition', ['attachment', 'inline'])->default('attachment');
                $table->unsignedBigInteger('size_bytes');
                $table->string('checksum_md5', 32)->nullable();
                $table->string('checksum_sha256', 64)->nullable();
                $table->string('storage_disk', 50)->default('r2');
                $table->string('storage_path', 500);
                $table->string('storage_key', 255)->unique();
                $table->boolean('is_inline')->default(false);
                $table->boolean('is_encrypted')->default(false);
                $table->enum('scan_status', [
                    'pending',
                    'clean',
                    'infected',
                    'error',
                    'skipped',
                ])->default('pending');
                $table->timestamp('scanned_at')->nullable();
                $table->string('scan_result', 255)->nullable();
                $table->json('metadata')->nullable();
                $table->timestamp('expires_at')->nullable();
                $table->timestamps();
                $table->softDeletes();

                // Performance indexes
                $table->index(['message_id', 'disposition']);
                $table->index('content_type');
                $table->index('scan_status');
                $table->index('checksum_sha256');
            });
        }

        // Email Thread Mappings Table - Email header threading correlation
        if (!Schema::hasTable('email_thread_mappings')) {
            Schema::create('email_thread_mappings', function (Blueprint $table): void {
                $table->id();
                $table->uuid('conversation_id');
                $table->foreign('conversation_id')
                    ->references('id')
                    ->on('email_conversations')
                    ->cascadeOnDelete();
                $table->string('message_id_header', 995)->index();
                $table->string('in_reply_to_header', 995)->nullable()->index();
                $table->string('references_first', 995)->nullable();
                $table->string('subject_normalized', 255)->index();
                $table->string('mailbox_hash', 255)->nullable()->index();
                $table->timestamps();

                // Composite indexes for thread matching
                $table->index(['message_id_header', 'conversation_id']);
                $table->index(['subject_normalized', 'conversation_id']);
            });
        }

        // Inbound Email Configuration Table
        if (!Schema::hasTable('inbound_email_settings')) {
            Schema::create('inbound_email_settings', function (Blueprint $table): void {
                $table->id();
                $table->string('key', 100)->unique();
                $table->text('value')->nullable();
                $table->string('type', 20)->default('string');
                $table->text('description')->nullable();
                $table->boolean('is_encrypted')->default(false);
                $table->timestamps();
            });

            // Insert default settings
            DB::table('inbound_email_settings')->insert([
                [
                    'key' => 'provider',
                    'value' => 'postmark',
                    'type' => 'string',
                    'description' => 'Inbound email provider (postmark, ses, sendgrid)',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'signature_key',
                    'value' => null,
                    'type' => 'string',
                    'description' => 'Webhook signature verification key',
                    'is_encrypted' => true,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'inbound_address_pattern',
                    'value' => 'reply+{hash}@mail.yourdomain.com',
                    'type' => 'string',
                    'description' => 'Pattern for inbound email addresses',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'spam_threshold',
                    'value' => '5.0',
                    'type' => 'float',
                    'description' => 'Spam score threshold for auto-marking as spam',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'max_attachment_size_mb',
                    'value' => '25',
                    'type' => 'integer',
                    'description' => 'Maximum attachment size in megabytes',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'allowed_attachment_types',
                    'value' => json_encode([
                        'application/pdf',
                        'application/msword',
                        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
                        'application/vnd.ms-excel',
                        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
                        'image/jpeg',
                        'image/png',
                        'image/gif',
                        'image/webp',
                        'text/plain',
                        'text/csv',
                    ]),
                    'type' => 'json',
                    'description' => 'Allowed MIME types for attachments',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'blocked_attachment_extensions',
                    'value' => json_encode([
                        'exe', 'bat', 'cmd', 'com', 'pif', 'scr', 'vbs', 'vbe',
                        'js', 'jse', 'ws', 'wsf', 'wsc', 'wsh', 'ps1', 'ps1xml',
                        'ps2', 'ps2xml', 'psc1', 'psc2', 'msc', 'msi', 'msp', 'mst',
                        'reg', 'dll', 'cpl', 'jar', 'hta', 'msh', 'msh1', 'msh2',
                    ]),
                    'type' => 'json',
                    'description' => 'Blocked file extensions for security',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'auto_create_contacts',
                    'value' => 'true',
                    'type' => 'boolean',
                    'description' => 'Automatically create contacts for unknown senders',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'default_contact_lifecycle_stage',
                    'value' => 'lead',
                    'type' => 'string',
                    'description' => 'Default lifecycle stage for auto-created contacts',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
                [
                    'key' => 'enable_virus_scanning',
                    'value' => 'true',
                    'type' => 'boolean',
                    'description' => 'Enable virus scanning for attachments',
                    'is_encrypted' => false,
                    'created_at' => now(),
                    'updated_at' => now(),
                ],
            ]);
        }

        // Add conversation relationship to contacts if not exists
        if (Schema::hasTable('contacts') && !Schema::hasColumn('contacts', 'conversations_count')) {
            Schema::table('contacts', function (Blueprint $table): void {
                $table->unsignedInteger('conversations_count')->default(0)->after('notes_count');
                $table->timestamp('last_email_received_at')->nullable()->after('last_contacted_at');
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        // Remove contact columns first
        if (Schema::hasTable('contacts')) {
            Schema::table('contacts', function (Blueprint $table): void {
                if (Schema::hasColumn('contacts', 'conversations_count')) {
                    $table->dropColumn('conversations_count');
                }
                if (Schema::hasColumn('contacts', 'last_email_received_at')) {
                    $table->dropColumn('last_email_received_at');
                }
            });
        }

        // Drop tables in reverse order due to foreign key constraints
        Schema::dropIfExists('email_thread_mappings');
        Schema::dropIfExists('email_attachments');
        Schema::dropIfExists('email_messages');
        Schema::dropIfExists('email_conversations');
        Schema::dropIfExists('inbound_email_settings');
    }
};
