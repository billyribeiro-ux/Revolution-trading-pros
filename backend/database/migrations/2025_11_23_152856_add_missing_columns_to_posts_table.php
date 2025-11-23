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
            $table->longText('content_html')->nullable()->after('content_blocks');
            
            // Reading metrics
            $table->integer('reading_time')->default(0)->after('word_count');
            $table->bigInteger('unique_view_count')->unsigned()->default(0)->after('view_count');
            
            // Analytics
            $table->decimal('bounce_rate', 5, 2)->default(0)->after('avg_time_on_page');
            
            // Content flags
            $table->boolean('is_evergreen')->default(false)->after('is_pinned');
            
            // Localization
            $table->string('locale', 10)->default('en')->after('readability_score');
            
            // Revisions
            $table->json('content_warnings')->nullable()->after('custom_fields');
            $table->integer('revision_count')->default(0)->after('version');
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
