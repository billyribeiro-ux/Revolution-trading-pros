<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        if (!Schema::hasTable('posts')) {
            return;
        }
        
        Schema::table('posts', function (Blueprint $table) {
            $table->index(['status', 'published_at', 'id'], 'idx_published_status');
            $table->index('published_at', 'idx_published_at');
        });
    }

    public function down(): void
    {
        Schema::table('posts', function (Blueprint $table) {
            $table->dropIndex('idx_published_status');
            $table->dropIndex('idx_published_at');
        });
    }
};
