<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Performance indexes migration
 *
 * Adds missing indexes for frequently queried columns to optimize:
 * - Form submissions filtering and stats
 * - Email logs lookups
 * - Posts filtering by author and date
 * - Email campaigns filtering
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // Form submissions indexes
        if (Schema::hasTable('form_submissions')) {
            Schema::table('form_submissions', function (Blueprint $table) {
                // Individual indexes
                if (!$this->hasIndex('form_submissions', 'form_submissions_form_id_index')) {
                    $table->index('form_id');
                }
                if (!$this->hasIndex('form_submissions', 'form_submissions_status_index')) {
                    $table->index('status');
                }
                if (!$this->hasIndex('form_submissions', 'form_submissions_created_at_index')) {
                    $table->index('created_at');
                }
                // Composite index for common filter combination
                if (!$this->hasIndex('form_submissions', 'form_submissions_form_id_status_index')) {
                    $table->index(['form_id', 'status'], 'form_submissions_form_id_status_index');
                }
            });
        }

        // Email logs indexes
        if (Schema::hasTable('email_logs')) {
            Schema::table('email_logs', function (Blueprint $table) {
                if (!$this->hasIndex('email_logs', 'email_logs_recipient_index')) {
                    $table->index('recipient');
                }
                if (!$this->hasIndex('email_logs', 'email_logs_email_type_sent_at_index')) {
                    $table->index(['email_type', 'sent_at'], 'email_logs_email_type_sent_at_index');
                }
            });
        }

        // Posts indexes
        if (Schema::hasTable('posts')) {
            Schema::table('posts', function (Blueprint $table) {
                if (!$this->hasIndex('posts', 'posts_author_id_index')) {
                    $table->index('author_id');
                }
                if (!$this->hasIndex('posts', 'posts_status_index')) {
                    $table->index('status');
                }
                if (!$this->hasIndex('posts', 'posts_published_at_index')) {
                    $table->index('published_at');
                }
            });
        }

        // Email campaigns indexes
        if (Schema::hasTable('email_campaigns')) {
            Schema::table('email_campaigns', function (Blueprint $table) {
                if (!$this->hasIndex('email_campaigns', 'email_campaigns_created_by_index')) {
                    $table->index('created_by');
                }
                if (!$this->hasIndex('email_campaigns', 'email_campaigns_status_created_at_index')) {
                    $table->index(['status', 'created_at'], 'email_campaigns_status_created_at_index');
                }
            });
        }

        // Contacts indexes
        if (Schema::hasTable('contacts')) {
            Schema::table('contacts', function (Blueprint $table) {
                if (!$this->hasIndex('contacts', 'contacts_status_index')) {
                    $table->index('status');
                }
                if (!$this->hasIndex('contacts', 'contacts_lifecycle_stage_index')) {
                    $table->index('lifecycle_stage');
                }
                if (!$this->hasIndex('contacts', 'contacts_owner_id_index')) {
                    $table->index('owner_id');
                }
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        if (Schema::hasTable('form_submissions')) {
            Schema::table('form_submissions', function (Blueprint $table) {
                $table->dropIndex(['form_id']);
                $table->dropIndex(['status']);
                $table->dropIndex(['created_at']);
                $table->dropIndex('form_submissions_form_id_status_index');
            });
        }

        if (Schema::hasTable('email_logs')) {
            Schema::table('email_logs', function (Blueprint $table) {
                $table->dropIndex(['recipient']);
                $table->dropIndex('email_logs_email_type_sent_at_index');
            });
        }

        if (Schema::hasTable('posts')) {
            Schema::table('posts', function (Blueprint $table) {
                $table->dropIndex(['author_id']);
                $table->dropIndex(['status']);
                $table->dropIndex(['published_at']);
            });
        }

        if (Schema::hasTable('email_campaigns')) {
            Schema::table('email_campaigns', function (Blueprint $table) {
                $table->dropIndex(['created_by']);
                $table->dropIndex('email_campaigns_status_created_at_index');
            });
        }

        if (Schema::hasTable('contacts')) {
            Schema::table('contacts', function (Blueprint $table) {
                $table->dropIndex(['status']);
                $table->dropIndex(['lifecycle_stage']);
                $table->dropIndex(['owner_id']);
            });
        }
    }

    /**
     * Check if an index exists on a table
     */
    private function hasIndex(string $table, string $index): bool
    {
        try {
            $indexes = Schema::getConnection()
                ->getSchemaBuilder()
                ->getIndexes($table);
            
            foreach ($indexes as $idx) {
                if ($idx['name'] === $index) {
                    return true;
                }
            }
            return false;
        } catch (\Exception $e) {
            return false;
        }
    }
};
