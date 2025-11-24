<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        if (!Schema::hasTable('popups')) {
            return;
        }
        
        Schema::table('popups', function (Blueprint $table) {
            // Status and activity flags for filtering
            $table->string('status', 50)->default('draft')->index()->after('name');
            $table->boolean('is_active')->default(true)->index()->after('status');
            
            // Popup type for categorization
            $table->string('type', 50)->default('modal')->index()->after('is_active');
            
            // A/B testing support
            $table->string('ab_test_id')->nullable()->index()->after('type');
            $table->string('variant_title')->nullable()->after('ab_test_id');
            
            // Priority for display order
            $table->integer('priority')->default(0)->index()->after('variant_title');
            
            // JSON columns for structured data
            $table->json('attention_animation')->nullable()->after('config');
            $table->json('countdown_timer')->nullable()->after('attention_animation');
            $table->json('video_embed')->nullable()->after('countdown_timer');
            $table->json('display_rules')->nullable()->after('video_embed');
            $table->json('form_fields')->nullable()->after('display_rules');
            $table->json('design')->nullable()->after('form_fields');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('popups', function (Blueprint $table) {
            $table->dropColumn([
                'status',
                'is_active',
                'type',
                'ab_test_id',
                'variant_title',
                'priority',
                'attention_animation',
                'countdown_timer',
                'video_embed',
                'display_rules',
                'form_fields',
                'design',
            ]);
        });
    }
};
