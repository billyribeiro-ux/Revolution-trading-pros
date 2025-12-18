<?php

namespace Database\Seeders;

use App\Models\Post;
use App\Models\User;
use Illuminate\Database\Seeder;

class PostSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        // Get or create an author
        $author = User::first() ?? User::factory()->create([
            'name' => 'Admin',
            'email' => 'admin@revolutiontradingpros.com',
        ]);

        $posts = [
            [
                'title' => '5 Essential Day Trading Strategies for Beginners',
                'slug' => '5-essential-day-trading-strategies-for-beginners',
                'excerpt' => 'Learn the fundamental day trading strategies every beginner needs to start their trading journey with confidence.',
                'content_blocks' => [
                    ['type' => 'paragraph', 'content' => 'Starting your day trading journey can be overwhelming, but with the right strategies, you can build a solid foundation for success.'],
                    ['type' => 'heading', 'content' => '1. Trend Following'],
                    ['type' => 'paragraph', 'content' => 'One of the most reliable strategies is following the trend. The market trend is your friend, and trading in its direction increases your probability of success.'],
                ],
                'featured_image' => '/images/blog/day-trading-strategies.jpg',
                'status' => 'published',
                'published_at' => now()->subDays(2),
            ],
            [
                'title' => 'Understanding SPX Options: A Comprehensive Guide',
                'slug' => 'understanding-spx-options-comprehensive-guide',
                'excerpt' => 'Master SPX options trading with this comprehensive guide covering strategies, risks, and best practices.',
                'content_blocks' => [
                    ['type' => 'paragraph', 'content' => 'SPX options offer unique advantages for traders looking to capitalize on market movements with less capital requirement.'],
                ],
                'featured_image' => '/images/blog/spx-options-guide.jpg',
                'status' => 'published',
                'published_at' => now()->subDays(5),
            ],
            [
                'title' => 'The Psychology of Trading: Mastering Your Emotions',
                'slug' => 'psychology-of-trading-mastering-emotions',
                'excerpt' => 'Discover how to control your emotions and develop the mental discipline required for consistent trading success.',
                'content_blocks' => [
                    ['type' => 'paragraph', 'content' => 'The difference between successful and unsuccessful traders often comes down to psychology and emotional control.'],
                ],
                'featured_image' => '/images/blog/trading-psychology.jpg',
                'status' => 'published',
                'published_at' => now()->subDays(7),
            ],
            [
                'title' => 'Swing Trading vs Day Trading: Which is Right for You?',
                'slug' => 'swing-trading-vs-day-trading',
                'excerpt' => 'Compare swing trading and day trading to determine which approach aligns with your lifestyle and goals.',
                'content_blocks' => [
                    ['type' => 'paragraph', 'content' => 'Choosing between swing trading and day trading depends on your schedule, risk tolerance, and trading capital.'],
                ],
                'featured_image' => '/images/blog/swing-vs-day-trading.jpg',
                'status' => 'published',
                'published_at' => now()->subDays(10),
            ],
            [
                'title' => 'Risk Management: The Key to Long-Term Trading Success',
                'slug' => 'risk-management-key-to-success',
                'excerpt' => 'Learn essential risk management techniques that will protect your capital and ensure longevity in trading.',
                'content_blocks' => [
                    ['type' => 'paragraph', 'content' => 'Proper risk management is what separates professional traders from gamblers. Never risk more than you can afford to lose.'],
                ],
                'featured_image' => '/images/blog/risk-management.jpg',
                'status' => 'published',
                'published_at' => now()->subDays(14),
            ],
            [
                'title' => 'Technical Indicators Every Trader Should Know',
                'slug' => 'technical-indicators-every-trader-should-know',
                'excerpt' => 'Explore the most effective technical indicators and how to use them to improve your trading decisions.',
                'content_blocks' => [
                    ['type' => 'paragraph', 'content' => 'Technical indicators help traders identify patterns, trends, and potential entry and exit points in the market.'],
                ],
                'featured_image' => '/images/blog/technical-indicators.jpg',
                'status' => 'published',
                'published_at' => now()->subDays(18),
            ],
        ];

        foreach ($posts as $postData) {
            Post::create(array_merge($postData, [
                'author_id' => $author->id,
                'meta_title' => $postData['title'],
                'meta_description' => $postData['excerpt'],
                'indexable' => true,
            ]));
        }
    }
}
