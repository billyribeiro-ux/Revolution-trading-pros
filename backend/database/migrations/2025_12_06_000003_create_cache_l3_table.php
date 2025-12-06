<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

/**
 * Creates L3 (database) cache table for multi-tier caching system
 *
 * @version 1.0.0
 */
return new class extends Migration
{
    public function up(): void
    {
        Schema::create('cache_l3', function (Blueprint $table) {
            $table->string('key', 500)->primary();
            $table->longText('value');
            $table->json('tags')->nullable();
            $table->timestamp('expires_at')->index();
            $table->timestamp('last_accessed_at')->nullable()->index();
            $table->timestamps();

            // Compound index for LRU-based cleanup
            $table->index(['expires_at', 'last_accessed_at'], 'cache_l3_expiry_lru_index');
        });

        // Create fulltext index for tag-based queries (MySQL 5.7+)
        if (config('database.default') === 'mysql') {
            try {
                DB::statement('CREATE FULLTEXT INDEX cache_l3_tags_fulltext ON cache_l3(tags)');
            } catch (\Throwable $e) {
                // Fulltext on JSON not supported in older MySQL versions, ignore
            }
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('cache_l3');
    }
};
