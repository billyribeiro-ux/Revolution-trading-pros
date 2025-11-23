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
        Schema::table('coupons', function (Blueprint $table) {
            // Extend type enum to include all coupon types
            $table->string('type', 50)->change();
            
            // Additional coupon fields
            $table->string('display_name')->nullable()->after('code');
            $table->text('description')->nullable()->after('display_name');
            $table->text('internal_notes')->nullable()->after('description');
            $table->timestamp('start_date')->nullable()->after('expiry_date');
            $table->decimal('max_discount_amount', 10, 2)->nullable()->after('value');
            
            // Categories and restrictions
            $table->json('applicable_categories')->nullable()->after('applicable_products');
            $table->json('restrictions')->nullable()->after('applicable_categories');
            
            // Campaign and targeting
            $table->string('campaign_id')->nullable()->index()->after('code');
            $table->json('segments')->nullable()->after('restrictions');
            $table->json('rules')->nullable()->after('segments');
            
            // Advanced features
            $table->boolean('stackable')->default(false)->after('is_active');
            $table->integer('priority')->default(0)->index()->after('stackable');
            $table->string('referral_source')->nullable()->after('priority');
            $table->string('affiliate_id')->nullable()->index()->after('referral_source');
            $table->string('influencer_id')->nullable()->index()->after('affiliate_id');
            
            // Metadata
            $table->json('tags')->nullable()->after('influencer_id');
            $table->json('meta')->nullable()->after('tags');
            $table->string('created_by')->nullable()->after('meta');
            $table->string('updated_by')->nullable()->after('created_by');
            $table->integer('version')->default(1)->after('updated_by');
            
            // Metrics (denormalized for performance)
            $table->integer('unique_customers')->default(0)->after('current_uses');
            $table->decimal('total_revenue', 12, 2)->default(0)->after('unique_customers');
            $table->decimal('total_discount', 12, 2)->default(0)->after('total_revenue');
            
            // A/B Testing
            $table->json('ab_test')->nullable()->after('campaign_id');
            
            // Tiered pricing config
            $table->json('tiers')->nullable()->after('value');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('coupons', function (Blueprint $table) {
            $table->dropColumn([
                'display_name',
                'description',
                'internal_notes',
                'start_date',
                'max_discount_amount',
                'applicable_categories',
                'restrictions',
                'campaign_id',
                'segments',
                'rules',
                'stackable',
                'priority',
                'referral_source',
                'affiliate_id',
                'influencer_id',
                'tags',
                'meta',
                'created_by',
                'updated_by',
                'version',
                'unique_customers',
                'total_revenue',
                'total_discount',
                'ab_test',
                'tiers',
            ]);
        });
    }
};
