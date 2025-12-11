<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Form Versioning and Audit Tables Migration
 *
 * Creates tables for:
 * - Form versions (full history with snapshots)
 * - Audit logs (GDPR compliance)
 * - Performance indexes
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // =====================================================================
        // FORM VERSIONS TABLE
        // =====================================================================
        Schema::create('form_versions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->unsignedInteger('version_number');
            $table->text('description')->nullable();
            $table->json('snapshot'); // Full form + fields snapshot
            $table->string('checksum', 64); // SHA-256 hash for integrity
            $table->boolean('is_published')->default(false);
            $table->timestamp('published_at')->nullable();
            $table->foreignId('created_by')->nullable()->constrained('users')->onDelete('set null');
            $table->timestamps();

            $table->unique(['form_id', 'version_number']);
            $table->index(['form_id', 'is_published']);
            $table->index('checksum');
        });

        // =====================================================================
        // FORM AUDIT LOGS TABLE
        // =====================================================================
        Schema::create('form_audit_logs', function (Blueprint $table) {
            $table->id();
            $table->string('event', 100)->index(); // Event type
            $table->string('severity', 20)->default('info')->index(); // info, warning, error, critical
            $table->foreignId('user_id')->nullable()->index();
            $table->string('user_email')->nullable();
            $table->string('user_role', 50)->nullable();
            $table->foreignId('form_id')->nullable()->index();
            $table->foreignId('submission_id')->nullable()->index();
            $table->string('ip_address', 45)->nullable()->index();
            $table->text('user_agent')->nullable();
            $table->string('request_method', 10)->nullable();
            $table->text('request_url')->nullable();
            $table->string('request_id', 50)->nullable();
            $table->string('session_id', 100)->nullable();
            $table->json('data')->nullable(); // Event-specific data
            $table->json('changes')->nullable(); // Before/after for updates
            $table->timestamp('created_at')->index();

            // Composite indexes for common queries
            $table->index(['form_id', 'event', 'created_at']);
            $table->index(['user_id', 'created_at']);
            $table->index(['severity', 'created_at']);
        });

        // =====================================================================
        // ADD VERSIONING COLUMNS TO FORMS TABLE
        // =====================================================================
        Schema::table('forms', function (Blueprint $table) {
            if (!Schema::hasColumn('forms', 'version')) {
                $table->unsignedInteger('version')->default(1)->after('status');
            }
            if (!Schema::hasColumn('forms', 'parent_id')) {
                $table->foreignId('parent_id')->nullable()->after('version')
                    ->constrained('forms')->onDelete('set null');
            }
            if (!Schema::hasColumn('forms', 'user_id')) {
                $table->foreignId('user_id')->nullable()->after('parent_id')
                    ->constrained('users')->onDelete('set null');
            }
        });

        // =====================================================================
        // PERFORMANCE INDEXES FOR EXISTING TABLES
        // =====================================================================

        // Forms table indexes
        if (!$this->indexExists('forms', 'forms_status_created_at_index')) {
            Schema::table('forms', function (Blueprint $table) {
                $table->index(['status', 'created_at'], 'forms_status_created_at_index');
            });
        }

        if (!$this->indexExists('forms', 'forms_user_id_status_index')) {
            Schema::table('forms', function (Blueprint $table) {
                $table->index(['user_id', 'status'], 'forms_user_id_status_index');
            });
        }

        // Form fields indexes
        if (!$this->indexExists('form_fields', 'form_fields_form_id_order_index')) {
            Schema::table('form_fields', function (Blueprint $table) {
                $table->index(['form_id', 'order'], 'form_fields_form_id_order_index');
            });
        }

        if (!$this->indexExists('form_fields', 'form_fields_field_type_index')) {
            Schema::table('form_fields', function (Blueprint $table) {
                $table->index('field_type', 'form_fields_field_type_index');
            });
        }

        // Form submissions indexes
        if (!$this->indexExists('form_submissions', 'form_submissions_form_id_status_index')) {
            Schema::table('form_submissions', function (Blueprint $table) {
                $table->index(['form_id', 'status'], 'form_submissions_form_id_status_index');
            });
        }

        if (!$this->indexExists('form_submissions', 'form_submissions_form_id_created_at_index')) {
            Schema::table('form_submissions', function (Blueprint $table) {
                $table->index(['form_id', 'created_at'], 'form_submissions_form_id_created_at_index');
            });
        }

        if (!$this->indexExists('form_submissions', 'form_submissions_user_id_created_at_index')) {
            Schema::table('form_submissions', function (Blueprint $table) {
                $table->index(['user_id', 'created_at'], 'form_submissions_user_id_created_at_index');
            });
        }

        // Form submission data indexes
        if (!$this->indexExists('form_submission_data', 'form_submission_data_field_name_index')) {
            Schema::table('form_submission_data', function (Blueprint $table) {
                $table->index('field_name', 'form_submission_data_field_name_index');
            });
        }

        if (!$this->indexExists('form_submission_data', 'form_submission_data_submission_field_index')) {
            Schema::table('form_submission_data', function (Blueprint $table) {
                $table->index(['submission_id', 'field_name'], 'form_submission_data_submission_field_index');
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        // Drop versioning columns from forms
        Schema::table('forms', function (Blueprint $table) {
            $table->dropForeign(['parent_id']);
            $table->dropColumn(['version', 'parent_id']);
        });

        // Drop new tables
        Schema::dropIfExists('form_audit_logs');
        Schema::dropIfExists('form_versions');

        // Drop added indexes (optional, for clean rollback)
        $this->dropIndexIfExists('forms', 'forms_status_created_at_index');
        $this->dropIndexIfExists('forms', 'forms_user_id_status_index');
        $this->dropIndexIfExists('form_fields', 'form_fields_form_id_order_index');
        $this->dropIndexIfExists('form_fields', 'form_fields_field_type_index');
        $this->dropIndexIfExists('form_submissions', 'form_submissions_form_id_status_index');
        $this->dropIndexIfExists('form_submissions', 'form_submissions_form_id_created_at_index');
        $this->dropIndexIfExists('form_submissions', 'form_submissions_user_id_created_at_index');
        $this->dropIndexIfExists('form_submission_data', 'form_submission_data_field_name_index');
        $this->dropIndexIfExists('form_submission_data', 'form_submission_data_submission_field_index');
    }

    /**
     * Check if an index exists
     */
    private function indexExists(string $table, string $indexName): bool
    {
        $indexes = Schema::getIndexListing($table);
        return in_array($indexName, $indexes);
    }

    /**
     * Drop index if it exists
     */
    private function dropIndexIfExists(string $table, string $indexName): void
    {
        if ($this->indexExists($table, $indexName)) {
            Schema::table($table, function (Blueprint $table) use ($indexName) {
                $table->dropIndex($indexName);
            });
        }
    }
};
