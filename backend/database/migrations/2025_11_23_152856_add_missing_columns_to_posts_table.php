<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::table('posts', function (Blueprint $table) {
            // Content rendering
            if (!Schema::hasColumn('posts', 'content_html')) {
                $table->longText('content_html')->nullable()->after('content_blocks');
            }
            
            // Reading metrics
            if (!Schema::hasColumn('posts', 'reading_time')) {
                $table->integer('reading_time')->default(0);
            }
            if (!Schema::hasColumn('posts', 'unique_view_count')) {
                $table->bigInteger('unique_view_count')->unsigned()->default(0);
            }
            
            // Analytics
            if (!Schema::hasColumn('posts', 'bounce_rate')) {
                $table->decimal('bounce_rate', 5, 2)->default(0);
            }
            
            // Content flags
            if (!Schema::hasColumn('posts', 'is_evergreen')) {
                $table->boolean('is_evergreen')->default(false);
            }
            
            // Localization
            if (!Schema::hasColumn('posts', 'locale')) {
                $table->string('locale', 10)->default('en');
            }
            
            // Revisions
            if (!Schema::hasColumn('posts', 'content_warnings')) {
                $table->json('content_warnings')->nullable();
            }
            if (!Schema::hasColumn('posts', 'revision_count')) {
                $table->integer('revision_count')->default(0);
            }
        });
    }

    public function down(): void
    {
        Schema::table('posts', function (Blueprint $table) {
            $table->dropColumn([
                'content_html',
                'reading_time',
                'unique_view_count',
                'bounce_rate',
                'is_evergreen',
                'locale',
                'content_warnings',
                'revision_count',
            ]);
        });
    }
};
