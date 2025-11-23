<?php

namespace Database\Seeders;

use App\Models\Post;
use App\Models\User;
use Illuminate\Database\Seeder;

class OptionsGreeksPostSeeder extends Seeder
{
    public function run(): void
    {
        $author = User::first();
        
        if (!$author) {
            $this->command->error('❌ No users found. Please create a user first.');
            return;
        }

        Post::create([
            // Basic fields
            'title' => 'The Ultimate Guide to Options Greeks',
            'slug' => 'ultimate-guide-options-greeks',
            'excerpt' => 'Master Delta, Gamma, Theta, Vega, and Rho with this comprehensive guide by Billy Ribeiro. Learn how professional traders use the Greeks to consistently profit in all market conditions.',
            'author_id' => $author->id,
            
            // Content
            'content_blocks' => null, // Your table has this
            
            // Categories and tags
            'categories' => ['Options Trading', 'Education', 'Advanced Strategies'],
            'tags' => ['Options Greeks', 'Delta', 'Gamma', 'Theta', 'Vega', 'Rho', 'Risk Management', 'Options Strategy', 'Billy Ribeiro'],
            'keywords' => ['options greeks', 'delta', 'gamma', 'theta', 'vega', 'rho'],
            
            // Status fields
            'status' => 'published',
            'visibility' => 'public',
            'published_at' => now()->subDays(2),
            
            // Flags
            'is_featured' => false,
            'is_pinned' => false,
            'allow_comments' => true,
            'indexable' => true,
            
            // Images
            'featured_image' => 'https://revolutiontradingpros.com/wp-content/uploads/2025/08/ChatGPT-Image-Aug-28-2025-04_19_20-AM-1024x683.png',
            'twitter_card' => 'summary_large_image',
            
            // SEO
            'meta_title' => 'The Ultimate Guide to Options Greeks | Master Delta, Gamma, Theta & Vega',
            'meta_description' => 'Complete guide to Options Greeks by Billy Ribeiro. Learn Delta, Gamma, Theta, Vega and how professional traders use them for consistent profits.',
            
            // Related
            'related_posts' => null,
            'custom_fields' => null,
            'schema_markup' => null,
        ]);

        // Now update with the full HTML content using a separate query
        // This avoids mass assignment issues
        $post = Post::where('slug', 'ultimate-guide-options-greeks')->first();
        $post->content_blocks = [
            [
                'type' => 'html',
                'content' => $this->getFullBlogContent()
            ]
        ];
        $post->save();

        $this->command->info('✅ Options Greeks blog post created successfully!');
        $this->command->info('📊 Post ID: ' . $post->id);
        $this->command->info('🔗 Slug: ' . $post->slug);
    }

    private function getFullBlogContent(): string
    {
        return <<<'HTML'
<div style="font-family: Montserrat, sans-serif; color: #333333;">

<h1 style="text-align: center; font-family: 'Montserrat', sans-serif; color: #333333;">
    <span style="color: #333333;">The Ultimate Guide to Options Greeks</span>
</h1>

<img class="wp-image-44962 aligncenter" src="https://revolutiontradingpros.com/wp-content/uploads/2025/08/ChatGPT-Image-Aug-28-2025-04_19_20-AM-1024x683.png" alt="The Ultimate Guide to Greeks Graph" width="1388" height="926" style="max-width: 100%; height: auto; display: block; margin: 20px auto;" />

<div style="display: flex; align-items: center; margin-top: 25px; margin-bottom: 25px;">
    <img style="width: 80px; height: 80px; border-radius: 50%; margin-right: 15px;" src="https://revolutiontradingpros.com/wp-content/uploads/2025/05/Billy-no-grey-768x768.png.webp" alt="Photo of Billy Ribeiro" />
    <span style="font-style: italic; color: #333333;"><strong>By Billy Ribeiro | Globally Recognized Trader & Market Strategist</strong></span>
</div>

<p style="color: #333333; font-family: Montserrat;"><strong>Billy Ribeiro</strong> is a globally recognized trader renowned for his mastery of price action analysis and innovative trading strategies. He was personally mentored by Mark McGoldrick, famously known as "Goldfinger," Goldman Sachs's most successful investor in history.</p>

<div style="background-color: #f7faff; border: 1px solid #cce0ff; padding: 20px 25px; margin: 30px 0; border-radius: 12px;">
    <h3 style="margin-top: 0;"><span style="color: #333333; font-family: Montserrat;">Billy's Key Takeaways</span></h3>
    <p style="font-style: italic; margin-bottom: 15px;"><span style="color: #333333; font-family: Montserrat;">"After 15+ years of trading and mentoring thousands of traders, here's what I wish every options trader understood from day one:"</span></p>
    <ul>
        <li><span style="color: #333333; font-family: Montserrat;"><strong>The Greeks aren't predictions—they're risk measurements.</strong></span></li>
        <li><span style="color: #333333; font-family: Montserrat;"><strong>Delta is your directional exposure speedometer.</strong></span></li>
        <li><span style="color: #333333; font-family: Montserrat;"><strong>Gamma is the hidden killer for premium sellers.</strong></span></li>
        <li><span style="color: #333333; font-family: Montserrat;"><strong>Theta is your best friend—if you're selling premium.</strong></span></li>
        <li><span style="color: #333333; font-family: Montserrat;"><strong>Vega separates the pros from the amateurs.</strong></span></li>
        <li><span style="color: #333333; font-family: Montserrat;"><strong>The Greeks work together, never in isolation.</strong></span></li>
    </ul>
</div>

<h2 style="margin-top: 40px;"><span style="color: #333333; font-family: Montserrat;">What Are the Option Greeks?</span></h2>

<p style="color: #333333; font-family: Montserrat;">The "Greeks" are mathematical measurements that quantify how an option's price responds to changes in underlying market conditions.</p>

<h2 style="margin-top: 40px;"><span style="color: #333333; font-family: Montserrat;">1. Delta (Δ): Your Directional Exposure</span></h2>

<p style="color: #333333; font-family: Montserrat;"><strong>Delta measures the expected change in an option's price for every $1 move in the underlying stock.</strong></p>

<h2 style="margin-top: 40px;"><span style="color: #333333; font-family: Montserrat;">2. Gamma (Γ): The Accelerator</span></h2>

<p style="color: #333333; font-family: Montserrat;"><strong>Gamma measures the rate of change of Delta for every $1 move in the underlying stock.</strong></p>

<h2 style="margin-top: 40px;"><span style="color: #333333; font-family: Montserrat;">3. Theta (Θ): Time Decay Machine</span></h2>

<p style="color: #333333; font-family: Montserrat;"><strong>Theta measures the rate of an option's price decay as time passes.</strong></p>

<h2 style="margin-top: 40px;"><span style="color: #333333; font-family: Montserrat;">4. Vega (ν): Volatility Master Key</span></h2>

<p style="color: #333333; font-family: Montserrat;"><strong>Vega measures an option's sensitivity to changes in implied volatility.</strong></p>

<div style="margin-top: 40px; padding: 30px 20px; background-color: #f8f9fa; border-radius: 20px;">
    <h2 style="text-align: center;"><span style="color: #333333;">Start Trading Like a Professional</span></h2>
    <p style="text-align: center; color: #333333;">Join our community for in-depth analysis and expert strategy.</p>
</div>

<div style="border: 1px solid #ccc; padding: 15px; margin-top: 40px; background-color: #fafafa; border-radius: 8px;">
    <p style="font-size: 0.9em; color: #555; text-align: center; margin: 0;"><strong>CRITICAL:</strong> Trading involves substantial risk of loss and is not suitable for all investors.</p>
</div>

</div>
HTML;
    }
}
