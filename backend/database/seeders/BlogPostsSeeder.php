<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use App\Models\Post;
use App\Models\User;
use Illuminate\Support\Str;

class BlogPostsSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        // Get or create an admin user for authorship
        $admin = User::where('email', 'admin@test.com')->first();
        
        if (!$admin) {
            $admin = User::create([
                'name' => 'Revolution Trading Team',
                'email' => 'admin@test.com',
                'password' => bcrypt('password123'),
                'email_verified_at' => now(),
            ]);
        }

        $posts = [
            [
                'title' => '5 Essential Risk Management Strategies Every Trader Needs',
                'slug' => '5-essential-risk-management-strategies',
                'excerpt' => 'Master the art of protecting your capital with these proven risk management techniques used by professional traders.',
                'content' => $this->getRiskManagementContent(),
                'featured_image' => 'https://images.unsplash.com/photo-1611974789855-9c2a0a7236a3?w=1200',
                'status' => 'published',
                'published_at' => now()->subDays(5),
                'meta_title' => '5 Essential Risk Management Strategies for Traders | Revolution Trading',
                'meta_description' => 'Learn the top 5 risk management strategies that professional traders use to protect their capital and maximize profits.',
                'reading_time' => 8,
            ],
            [
                'title' => 'Understanding the SPX Profit Pulse Strategy',
                'slug' => 'understanding-spx-profit-pulse-strategy',
                'excerpt' => 'Dive deep into our proprietary SPX trading strategy and learn how to capitalize on market movements with precision.',
                'content' => $this->getSpxStrategyContent(),
                'featured_image' => 'https://images.unsplash.com/photo-1642790106117-e829e14a795f?w=1200',
                'status' => 'published',
                'published_at' => now()->subDays(12),
                'meta_title' => 'SPX Profit Pulse Strategy Explained | Revolution Trading',
                'meta_description' => 'Master the SPX Profit Pulse strategy with our comprehensive guide. Learn entry points, exit strategies, and risk management.',
                'reading_time' => 12,
            ],
            [
                'title' => 'Options Trading 101: A Beginner\'s Guide to Getting Started',
                'slug' => 'options-trading-101-beginners-guide',
                'excerpt' => 'New to options trading? This comprehensive guide covers everything you need to know to start trading options confidently.',
                'content' => $this->getOptionsGuidContent(),
                'featured_image' => 'https://images.unsplash.com/photo-1590283603385-17ffb3a7f29f?w=1200',
                'status' => 'published',
                'published_at' => now()->subDays(18),
                'meta_title' => 'Options Trading 101: Complete Beginner\'s Guide | Revolution Trading',
                'meta_description' => 'Learn options trading from scratch. Understand calls, puts, strategies, and risk management in this comprehensive beginner\'s guide.',
                'reading_time' => 15,
            ],
            [
                'title' => 'How to Read Market Indicators Like a Pro',
                'slug' => 'how-to-read-market-indicators',
                'excerpt' => 'Technical indicators are essential tools for traders. Learn how to interpret RSI, MACD, and other key indicators effectively.',
                'content' => $this->getIndicatorsContent(),
                'featured_image' => 'https://images.unsplash.com/photo-1642790551116-18e150f248e5?w=1200',
                'status' => 'published',
                'published_at' => now()->subDays(25),
                'meta_title' => 'How to Read Market Indicators Like a Pro | Revolution Trading',
                'meta_description' => 'Master technical indicators including RSI, MACD, and Bollinger Bands. Learn professional trading techniques.',
                'reading_time' => 10,
            ],
            [
                'title' => 'Day Trading vs Swing Trading: Which Strategy is Right for You?',
                'slug' => 'day-trading-vs-swing-trading',
                'excerpt' => 'Explore the key differences between day trading and swing trading to determine which approach aligns with your goals and lifestyle.',
                'content' => $this->getDayVsSwingContent(),
                'featured_image' => 'https://images.unsplash.com/photo-1642543492481-44e81e3914a7?w=1200',
                'status' => 'published',
                'published_at' => now()->subDays(30),
                'meta_title' => 'Day Trading vs Swing Trading: Complete Comparison | Revolution Trading',
                'meta_description' => 'Compare day trading and swing trading strategies. Find out which trading style fits your personality and schedule.',
                'reading_time' => 9,
            ],
            [
                'title' => 'The Psychology of Trading: Mastering Your Emotions',
                'slug' => 'psychology-of-trading-mastering-emotions',
                'excerpt' => 'Trading success isn\'t just about strategy—it\'s about mindset. Learn how to control emotions and make rational trading decisions.',
                'content' => $this->getPsychologyContent(),
                'featured_image' => 'https://images.unsplash.com/photo-1559526324-593bc073d938?w=1200',
                'status' => 'published',
                'published_at' => now()->subDays(40),
                'meta_title' => 'Trading Psychology: Master Your Emotions | Revolution Trading',
                'meta_description' => 'Learn how to control fear and greed in trading. Develop the mental discipline needed for consistent trading success.',
                'reading_time' => 11,
            ],
        ];

        foreach ($posts as $postData) {
            Post::create([
                'author_id' => $admin->id,
                'title' => $postData['title'],
                'slug' => $postData['slug'],
                'excerpt' => $postData['excerpt'],
                'content_html' => $postData['content'],
                'featured_image' => $postData['featured_image'],
                'status' => $postData['status'],
                'published_at' => $postData['published_at'],
                'meta_title' => $postData['meta_title'],
                'meta_description' => $postData['meta_description'],
                'reading_time' => $postData['reading_time'],
                'view_count' => rand(150, 2500),
                'word_count' => str_word_count(strip_tags($postData['content'])),
                'is_featured' => false,
                'allow_comments' => true,
                'visibility' => 'public',
            ]);
        }

        $this->command->info('✅ Created ' . count($posts) . ' blog posts successfully!');
    }

    private function getRiskManagementContent(): string
    {
        return <<<'HTML'
<h2>Introduction to Risk Management</h2>
<p>Risk management is the cornerstone of successful trading. Without proper risk controls, even the best trading strategy can lead to devastating losses. In this guide, we'll explore five essential risk management strategies that every trader should implement.</p>

<h2>1. The 1-2% Rule</h2>
<p>Never risk more than 1-2% of your total trading capital on a single trade. This simple rule ensures that even a series of losses won't significantly impact your account.</p>
<p><strong>Example:</strong> With a $10,000 account, risk only $100-$200 per trade. This allows you to survive 50-100 consecutive losses before depleting your account.</p>

<h2>2. Position Sizing</h2>
<p>Calculate your position size based on your stop loss and risk tolerance. Use this formula:</p>
<p><code>Position Size = (Account Risk) / (Trade Risk per Share)</code></p>

<h2>3. Stop Loss Orders</h2>
<p>Always use stop loss orders to limit potential losses. Place them at logical technical levels, not arbitrary prices.</p>

<h2>4. Risk-Reward Ratio</h2>
<p>Aim for a minimum 1:2 risk-reward ratio. If you're risking $100, your potential profit should be at least $200.</p>

<h2>5. Diversification</h2>
<p>Don't put all your eggs in one basket. Spread your risk across different assets, sectors, and strategies.</p>

<h2>Conclusion</h2>
<p>Implementing these five risk management strategies will significantly improve your trading longevity and profitability. Remember: protecting your capital is just as important as growing it.</p>
HTML;
    }

    private function getSpxStrategyContent(): string
    {
        return <<<'HTML'
<h2>What is the SPX Profit Pulse Strategy?</h2>
<p>The SPX Profit Pulse is our proprietary trading strategy designed specifically for S&P 500 index options. This strategy capitalizes on short-term price movements while maintaining strict risk controls.</p>

<h2>Key Components</h2>
<h3>1. Market Analysis</h3>
<p>We analyze multiple timeframes to identify high-probability setups:</p>
<ul>
<li>Daily charts for overall trend direction</li>
<li>4-hour charts for swing points</li>
<li>1-hour charts for precise entries</li>
</ul>

<h3>2. Entry Criteria</h3>
<p>Trades are only taken when all of the following conditions are met:</p>
<ul>
<li>Clear trend direction on daily timeframe</li>
<li>Pullback to key support/resistance level</li>
<li>Momentum indicators showing reversal</li>
<li>Volume confirmation</li>
</ul>

<h3>3. Risk Management</h3>
<p>Every trade includes:</p>
<ul>
<li>Predetermined stop loss (typically 15-20 points)</li>
<li>Target profit levels (2:1 minimum risk-reward)</li>
<li>Position sizing based on account risk</li>
</ul>

<h2>Real-World Example</h2>
<p>On November 15th, SPX pulled back to the 4,500 support level after a strong uptrend. Our indicators showed bullish divergence, and we entered a call option position with:</p>
<ul>
<li>Entry: 4,505</li>
<li>Stop Loss: 4,485 (20 points risk)</li>
<li>Target: 4,545 (40 points profit)</li>
<li>Result: Target hit in 2 days for 100% profit on options</li>
</ul>

<h2>Join Our Live Trading Room</h2>
<p>Want to see the SPX Profit Pulse strategy in action? Join our live trading room where we call out trades in real-time.</p>
HTML;
    }

    private function getOptionsGuidContent(): string
    {
        return <<<'HTML'
<h2>What Are Options?</h2>
<p>Options are financial contracts that give you the right (but not the obligation) to buy or sell an asset at a predetermined price before a specific date.</p>

<h2>Types of Options</h2>
<h3>Call Options</h3>
<p>A call option gives you the right to BUY an asset at a specific price. You buy calls when you expect the price to go UP.</p>

<h3>Put Options</h3>
<p>A put option gives you the right to SELL an asset at a specific price. You buy puts when you expect the price to go DOWN.</p>

<h2>Key Terms You Need to Know</h2>
<ul>
<li><strong>Strike Price:</strong> The price at which you can buy/sell the asset</li>
<li><strong>Expiration Date:</strong> The last day the option is valid</li>
<li><strong>Premium:</strong> The price you pay for the option</li>
<li><strong>In the Money (ITM):</strong> Option has intrinsic value</li>
<li><strong>Out of the Money (OTM):</strong> Option has no intrinsic value</li>
</ul>

<h2>Basic Options Strategies</h2>
<h3>1. Long Call</h3>
<p>Buy a call option when you're bullish. Limited risk (premium paid), unlimited profit potential.</p>

<h3>2. Long Put</h3>
<p>Buy a put option when you're bearish. Limited risk, substantial profit potential.</p>

<h3>3. Covered Call</h3>
<p>Own the stock and sell call options against it. Generate income while holding stocks.</p>

<h2>Getting Started</h2>
<p>Before trading options:</p>
<ol>
<li>Get approved for options trading with your broker</li>
<li>Paper trade to practice without risk</li>
<li>Start small with simple strategies</li>
<li>Never risk more than you can afford to lose</li>
<li>Continue learning and refining your approach</li>
</ol>

<h2>Enroll in Our Options Trading Course</h2>
<p>Ready to master options trading? Our comprehensive course covers everything from basics to advanced strategies.</p>
HTML;
    }

    private function getIndicatorsContent(): string
    {
        return <<<'HTML'
<h2>Understanding Technical Indicators</h2>
<p>Technical indicators are mathematical calculations based on price, volume, or open interest that help traders make informed decisions.</p>

<h2>Top 5 Essential Indicators</h2>

<h3>1. Relative Strength Index (RSI)</h3>
<p>RSI measures the speed and magnitude of price changes on a scale of 0-100.</p>
<ul>
<li>Above 70: Overbought (potential sell signal)</li>
<li>Below 30: Oversold (potential buy signal)</li>
<li>Divergence: Price makes new high/low but RSI doesn't (reversal signal)</li>
</ul>

<h3>2. Moving Average Convergence Divergence (MACD)</h3>
<p>MACD shows the relationship between two moving averages.</p>
<ul>
<li>MACD crosses above signal line: Bullish</li>
<li>MACD crosses below signal line: Bearish</li>
<li>Histogram shows momentum strength</li>
</ul>

<h3>3. Bollinger Bands</h3>
<p>Bollinger Bands measure volatility and identify overbought/oversold conditions.</p>
<ul>
<li>Price touching upper band: Potential resistance</li>
<li>Price touching lower band: Potential support</li>
<li>Squeeze: Low volatility, breakout coming</li>
</ul>

<h3>4. Volume</h3>
<p>Volume confirms the strength of price movements.</p>
<ul>
<li>High volume + price increase: Strong bullish signal</li>
<li>High volume + price decrease: Strong bearish signal</li>
<li>Low volume: Weak signal, be cautious</li>
</ul>

<h3>5. Moving Averages</h3>
<p>Moving averages smooth out price data to identify trends.</p>
<ul>
<li>50-day MA: Medium-term trend</li>
<li>200-day MA: Long-term trend</li>
<li>Golden Cross: 50 MA crosses above 200 MA (bullish)</li>
<li>Death Cross: 50 MA crosses below 200 MA (bearish)</li>
</ul>

<h2>Combining Indicators</h2>
<p>The real power comes from using multiple indicators together:</p>
<ul>
<li>RSI + MACD for confirmation</li>
<li>Moving Averages + Volume for trend strength</li>
<li>Bollinger Bands + RSI for reversal signals</li>
</ul>

<h2>Pro Tips</h2>
<ol>
<li>No indicator is 100% accurate</li>
<li>Use indicators as confirmation, not sole decision-makers</li>
<li>Adjust settings based on your timeframe</li>
<li>Less is more—don't clutter your charts</li>
<li>Practice on historical data before live trading</li>
</ol>
HTML;
    }

    private function getDayVsSwingContent(): string
    {
        return <<<'HTML'
<h2>Day Trading vs Swing Trading: The Ultimate Comparison</h2>
<p>Choosing between day trading and swing trading is one of the most important decisions you'll make as a trader. Let's break down both strategies.</p>

<h2>Day Trading</h2>
<h3>What is Day Trading?</h3>
<p>Day trading involves opening and closing positions within the same trading day. No positions are held overnight.</p>

<h3>Pros:</h3>
<ul>
<li>No overnight risk</li>
<li>Multiple opportunities daily</li>
<li>Quick profits possible</li>
<li>No gap risk</li>
</ul>

<h3>Cons:</h3>
<ul>
<li>Requires full-time commitment</li>
<li>High stress and intensity</li>
<li>More commissions and fees</li>
<li>Requires $25,000 minimum (PDT rule)</li>
</ul>

<h3>Best For:</h3>
<ul>
<li>Full-time traders</li>
<li>Those who thrive under pressure</li>
<li>People with significant capital</li>
<li>Quick decision-makers</li>
</ul>

<h2>Swing Trading</h2>
<h3>What is Swing Trading?</h3>
<p>Swing trading involves holding positions for several days to weeks, capturing larger price movements.</p>

<h3>Pros:</h3>
<ul>
<li>Part-time friendly</li>
<li>Less stressful</li>
<li>Lower commission costs</li>
<li>No PDT rule restrictions</li>
<li>Larger profit potential per trade</li>
</ul>

<h3>Cons:</h3>
<ul>
<li>Overnight and weekend risk</li>
<li>Requires patience</li>
<li>Fewer trading opportunities</li>
<li>Gap risk on earnings/news</li>
</ul>

<h3>Best For:</h3>
<ul>
<li>Part-time traders</li>
<li>Those with full-time jobs</li>
<li>Patient personalities</li>
<li>Smaller account sizes</li>
</ul>

<h2>Side-by-Side Comparison</h2>
<table>
<tr>
<th>Factor</th>
<th>Day Trading</th>
<th>Swing Trading</th>
</tr>
<tr>
<td>Time Commitment</td>
<td>Full-time (6+ hours/day)</td>
<td>Part-time (1-2 hours/day)</td>
</tr>
<tr>
<td>Minimum Capital</td>
<td>$25,000+</td>
<td>$5,000+</td>
</tr>
<tr>
<td>Stress Level</td>
<td>High</td>
<td>Moderate</td>
</tr>
<tr>
<td>Profit per Trade</td>
<td>0.5-2%</td>
<td>5-20%</td>
</tr>
</table>

<h2>Which Should You Choose?</h2>
<p>Consider these questions:</p>
<ol>
<li>How much time can you dedicate to trading?</li>
<li>What's your risk tolerance?</li>
<li>How much capital do you have?</li>
<li>What's your personality type?</li>
<li>What are your financial goals?</li>
</ol>

<h2>Our Recommendation</h2>
<p>Start with swing trading if you're new to trading. It's more forgiving, requires less capital, and allows you to learn without the pressure of day trading. Once you're consistently profitable, you can explore day trading if it aligns with your goals.</p>
HTML;
    }

    private function getPsychologyContent(): string
    {
        return <<<'HTML'
<h2>Why Trading Psychology Matters</h2>
<p>95% of traders fail not because of bad strategies, but because of poor emotional control. Your mindset is your most powerful trading tool.</p>

<h2>The Two Biggest Enemies: Fear and Greed</h2>

<h3>Fear</h3>
<p>Fear causes traders to:</p>
<ul>
<li>Exit winning trades too early</li>
<li>Hesitate on good setups</li>
<li>Overtrade to "make back" losses</li>
<li>Freeze during critical moments</li>
</ul>

<h3>Greed</h3>
<p>Greed causes traders to:</p>
<ul>
<li>Hold losing trades too long</li>
<li>Risk too much on single trades</li>
<li>Ignore stop losses</li>
<li>Chase trades that have already moved</li>
</ul>

<h2>Building Mental Discipline</h2>

<h3>1. Create a Trading Plan</h3>
<p>A detailed plan removes emotion from decisions:</p>
<ul>
<li>Entry criteria</li>
<li>Exit criteria</li>
<li>Position sizing rules</li>
<li>Risk management rules</li>
</ul>

<h3>2. Keep a Trading Journal</h3>
<p>Document every trade:</p>
<ul>
<li>Setup and reasoning</li>
<li>Emotions before/during/after</li>
<li>What went right/wrong</li>
<li>Lessons learned</li>
</ul>

<h3>3. Accept Losses</h3>
<p>Losses are part of trading. Even the best traders have a 50-60% win rate. Focus on:</p>
<ul>
<li>Following your plan</li>
<li>Managing risk properly</li>
<li>Learning from mistakes</li>
<li>Long-term profitability</li>
</ul>

<h3>4. Manage Expectations</h3>
<p>Unrealistic expectations lead to emotional trading:</p>
<ul>
<li>Don't expect to get rich quick</li>
<li>Aim for consistent, modest gains</li>
<li>Focus on process, not profits</li>
<li>Celebrate small wins</li>
</ul>

<h3>5. Take Breaks</h3>
<p>Trading fatigue leads to mistakes:</p>
<ul>
<li>Step away after big wins/losses</li>
<li>Don't trade when emotional</li>
<li>Take regular days off</li>
<li>Maintain work-life balance</li>
</ul>

<h2>The Professional Trader's Mindset</h2>
<p>Professional traders think differently:</p>
<ul>
<li>They view trading as a business</li>
<li>They focus on probabilities, not certainties</li>
<li>They accept that losses are inevitable</li>
<li>They stick to their plan religiously</li>
<li>They continuously learn and adapt</li>
</ul>

<h2>Practical Exercises</h2>

<h3>Exercise 1: Pre-Trade Checklist</h3>
<p>Before every trade, ask yourself:</p>
<ol>
<li>Does this meet my criteria?</li>
<li>What's my risk-reward ratio?</li>
<li>Where's my stop loss?</li>
<li>Am I trading emotionally?</li>
<li>Can I afford to lose this trade?</li>
</ol>

<h3>Exercise 2: Meditation</h3>
<p>5-10 minutes of meditation before trading helps:</p>
<ul>
<li>Clear your mind</li>
<li>Reduce anxiety</li>
<li>Improve focus</li>
<li>Control emotions</li>
</ul>

<h2>Conclusion</h2>
<p>Mastering trading psychology takes time and practice. Be patient with yourself, stay disciplined, and remember: the market will always be there tomorrow.</p>
HTML;
    }
}
