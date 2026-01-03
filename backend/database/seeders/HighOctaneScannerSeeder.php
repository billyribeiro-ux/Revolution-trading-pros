<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Carbon\Carbon;

class HighOctaneScannerSeeder extends Seeder
{
    /**
     * Seed the High Octane Scanner product and membership plans
     * 
     * This seeder creates:
     * 1. Product record for High Octane Scanner
     * 2. Monthly membership plan ($119/month)
     * 3. Yearly membership plan ($1,190/year - 17% savings)
     *
     * @return void
     */
    public function run(): void
    {
        $now = Carbon::now();

        // ═══════════════════════════════════════════════════════════════════════════
        // PRODUCT RECORD
        // ═══════════════════════════════════════════════════════════════════════════
        
        $productId = 'high-octane-scanner';
        
        DB::table('products')->updateOrInsert(
            ['slug' => $productId],
            [
                'id' => $productId,
                'name' => 'High Octane Scanner',
                'slug' => $productId,
                'type' => 'indicator',
                'category' => 'scanner',
                'description' => 'Professional-grade options scanner with real-time alerts and advanced filtering algorithms',
                'long_description' => 'The High Octane Scanner is your ultimate tool for finding high-probability options trades. Built by professional traders for serious market participants, this scanner uses proprietary algorithms to identify opportunities across thousands of stocks in real-time.',
                'price_monthly' => 119.00,
                'price_quarterly' => null,
                'price_yearly' => 1190.00,
                'price_lifetime' => null,
                'status' => 'active',
                'featured' => true,
                'features' => json_encode([
                    'Real-time options scanning',
                    'Advanced filtering algorithms',
                    'Custom alert notifications',
                    'Historical data analysis',
                    'Multi-timeframe analysis',
                    'Mobile app access',
                    'Unlimited scans and alerts',
                    'Priority customer support'
                ]),
                'metadata' => json_encode([
                    'icon' => 'chart-candle',
                    'color' => '#0984ae',
                    'badge' => 'Popular',
                    'trial_days' => 0,
                    'setup_required' => false
                ]),
                'created_at' => $now,
                'updated_at' => $now
            ]
        );

        // ═══════════════════════════════════════════════════════════════════════════
        // MEMBERSHIP PLANS
        // ═══════════════════════════════════════════════════════════════════════════

        // Monthly Plan
        DB::table('membership_plans')->updateOrInsert(
            ['slug' => 'high-octane-scanner-monthly'],
            [
                'id' => Str::uuid(),
                'product_id' => $productId,
                'name' => 'High Octane Scanner - Monthly',
                'slug' => 'high-octane-scanner-monthly',
                'interval' => 'monthly',
                'interval_count' => 1,
                'price' => 119.00,
                'currency' => 'USD',
                'trial_days' => 0,
                'features' => json_encode([
                    'Real-time scanning',
                    'Advanced filters',
                    'Smart alerts',
                    'Historical analysis',
                    'Mobile access',
                    'Email support'
                ]),
                'status' => 'active',
                'stripe_price_id' => null, // Set this after creating in Stripe
                'metadata' => json_encode([
                    'recommended' => false,
                    'savings_percent' => 0
                ]),
                'created_at' => $now,
                'updated_at' => $now
            ]
        );

        // Yearly Plan (17% savings)
        DB::table('membership_plans')->updateOrInsert(
            ['slug' => 'high-octane-scanner-yearly'],
            [
                'id' => Str::uuid(),
                'product_id' => $productId,
                'name' => 'High Octane Scanner - Yearly',
                'slug' => 'high-octane-scanner-yearly',
                'interval' => 'yearly',
                'interval_count' => 1,
                'price' => 1190.00,
                'currency' => 'USD',
                'trial_days' => 0,
                'features' => json_encode([
                    'Real-time scanning',
                    'Advanced filters',
                    'Smart alerts',
                    'Historical analysis',
                    'Mobile access',
                    'Priority support',
                    '17% savings vs monthly'
                ]),
                'status' => 'active',
                'stripe_price_id' => null, // Set this after creating in Stripe
                'metadata' => json_encode([
                    'recommended' => true,
                    'savings_percent' => 17,
                    'monthly_equivalent' => 99.17
                ]),
                'created_at' => $now,
                'updated_at' => $now
            ]
        );

        $this->command->info('✅ High Octane Scanner product and plans seeded successfully!');
        $this->command->info('📊 Product ID: ' . $productId);
        $this->command->info('💰 Monthly: $119/month');
        $this->command->info('💰 Yearly: $1,190/year (17% savings)');
    }
}
