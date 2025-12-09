<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use Illuminate\Support\Facades\DB;

/**
 * Blog Categories Seeder
 *
 * Seeds predefined blog categories matching the frontend
 * category selection system for trading/education content.
 *
 * @version 1.0.0 - December 2025
 */
class BlogCategoriesSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        $categories = [
            [
                'name' => 'Market Analysis',
                'slug' => 'market-analysis',
                'description' => 'Market analysis, trends, and insights',
                'color' => '#3b82f6',
                'order' => 1,
            ],
            [
                'name' => 'Trading Strategies',
                'slug' => 'trading-strategies',
                'description' => 'Trading strategies and techniques',
                'color' => '#10b981',
                'order' => 2,
            ],
            [
                'name' => 'Risk Management',
                'slug' => 'risk-management',
                'description' => 'Managing risk in trading',
                'color' => '#ef4444',
                'order' => 3,
            ],
            [
                'name' => 'Options Trading',
                'slug' => 'options-trading',
                'description' => 'Options strategies and trading',
                'color' => '#f59e0b',
                'order' => 4,
            ],
            [
                'name' => 'Technical Analysis',
                'slug' => 'technical-analysis',
                'description' => 'Chart patterns and technical indicators',
                'color' => '#6366f1',
                'order' => 5,
            ],
            [
                'name' => 'Fundamental Analysis',
                'slug' => 'fundamental-analysis',
                'description' => 'Company and economic analysis',
                'color' => '#ec4899',
                'order' => 6,
            ],
            [
                'name' => 'Psychology',
                'slug' => 'psychology',
                'description' => 'Trading psychology and mindset',
                'color' => '#8b5cf6',
                'order' => 7,
            ],
            [
                'name' => 'Education',
                'slug' => 'education',
                'description' => 'Educational content for traders',
                'color' => '#14b8a6',
                'order' => 8,
            ],
            [
                'name' => 'News & Updates',
                'slug' => 'news',
                'description' => 'Market news and platform updates',
                'color' => '#06b6d4',
                'order' => 9,
            ],
            [
                'name' => 'Earnings',
                'slug' => 'earnings',
                'description' => 'Earnings reports and analysis',
                'color' => '#f97316',
                'order' => 10,
            ],
            [
                'name' => 'Stocks',
                'slug' => 'stocks',
                'description' => 'Stock market content',
                'color' => '#84cc16',
                'order' => 11,
            ],
            [
                'name' => 'Futures',
                'slug' => 'futures',
                'description' => 'Futures trading content',
                'color' => '#22c55e',
                'order' => 12,
            ],
            [
                'name' => 'Forex',
                'slug' => 'forex',
                'description' => 'Forex trading content',
                'color' => '#0ea5e9',
                'order' => 13,
            ],
            [
                'name' => 'Crypto',
                'slug' => 'crypto',
                'description' => 'Cryptocurrency trading',
                'color' => '#a855f7',
                'order' => 14,
            ],
            [
                'name' => 'Small Accounts',
                'slug' => 'small-accounts',
                'description' => 'Strategies for small trading accounts',
                'color' => '#eab308',
                'order' => 15,
            ],
            [
                'name' => 'Day Trading',
                'slug' => 'day-trading',
                'description' => 'Day trading strategies and tips',
                'color' => '#d946ef',
                'order' => 16,
            ],
            [
                'name' => 'Swing Trading',
                'slug' => 'swing-trading',
                'description' => 'Swing trading strategies',
                'color' => '#64748b',
                'order' => 17,
            ],
            [
                'name' => 'Beginners Guide',
                'slug' => 'beginners',
                'description' => 'Content for beginner traders',
                'color' => '#fb7185',
                'order' => 18,
            ],
        ];

        foreach ($categories as $category) {
            DB::table('categories')->updateOrInsert(
                ['slug' => $category['slug']],
                array_merge($category, [
                    'is_visible' => true,
                    'created_at' => now(),
                    'updated_at' => now(),
                ])
            );
        }

        $this->command->info('Blog categories seeded successfully!');
    }
}
