<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import {
        IconChartLine,
        IconTrendingUp,
        IconTrendingDown,
        IconActivity,
        IconTarget,
        IconBolt,
        IconStar,
        IconCheck,
        IconArrowRight,
        IconChartCandle,
        IconWaveSine,
        IconChartBar,
        IconAlertTriangle,
        IconUsers,
        IconSchool,
        IconChevronDown
    } from '@tabler/icons-svelte';
    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- Types ---
    interface Indicator {
        id: string;
        name: string;
        slug: string;
        category: string;
        description: string;
        useCase: string;
        difficulty: string;
        icon: any;
        color: string;
        gradient: string;
        features: string[];
    }

    interface FaqItem {
        question: string;
        answer: string;
    }

    // --- Data ---
    const indicators: Indicator[] = [
        {
            id: '1',
            name: 'RSI - Relative Strength Index',
            slug: 'rsi',
            category: 'Momentum',
            description: 'Don’t just spot overbought conditions—identify divergence. The RSI is our primary tool for spotting exhaustion before the price turns.',
            useCase: 'Spotting reversals and confirming trend weakness (Divergence)',
            difficulty: 'Beginner',
            icon: IconActivity,
            color: '#2e8eff',
            gradient: 'linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%)',
            features: ['Bullish/Bearish Divergence', 'Trend Reset Zones', 'Momentum Confirmation']
        },
        {
            id: '2',
            name: 'MACD - Moving Average Convergence Divergence',
            slug: 'macd',
            category: 'Trend Following',
            description: 'The grandfather of momentum indicators. We use it to filter out noise and stay on the right side of the dominant market trend.',
            useCase: 'Filtering false breakouts and confirming trend direction',
            difficulty: 'Intermediate',
            icon: IconWaveSine,
            color: '#34d399',
            gradient: 'linear-gradient(135deg, #34d399 0%, #059669 100%)',
            features: ['Histogram Momentum', 'Zero-Line Rejection', 'Multi-Timeframe Alignment']
        },
        {
            id: '3',
            name: 'Moving Averages (SMA/EMA)',
            slug: 'moving-averages',
            category: 'Trend Following',
            description: 'The dynamic spine of the market. Learn why price respects the 9 EMA and 200 SMA, and how to use them as dynamic support.',
            useCase: 'Trailing stops and identifying dynamic support/resistance zones',
            difficulty: 'Beginner',
            icon: IconChartLine,
            color: '#a78bfa',
            gradient: 'linear-gradient(135deg, #a78bfa 0%, #7c3aed 100%)',
            features: ['Golden Cross / Death Cross', 'The "9 EMA Ride"', 'Mean Reversion']
        },
        {
            id: '4',
            name: 'Bollinger Bands',
            slug: 'bollinger-bands',
            category: 'Volatility',
            description: 'The market breathes in (squeeze) and breathes out (expansion). Bollinger Bands help you catch the explosive moves after the calm.',
            useCase: 'Trading "The Squeeze" and measuring volatility extremes',
            difficulty: 'Intermediate',
            icon: IconChartBar,
            color: '#fb923c',
            gradient: 'linear-gradient(135deg, #fb923c 0%, #ea580c 100%)',
            features: ['Volatility Squeezes', 'Band Walking', 'Mean Reversion Targets']
        },
        {
            id: '5',
            name: 'VWAP - Volume Weighted Average Price',
            slug: 'vwap',
            category: 'Volume',
            description: 'The only indicator institutions care about. If you are day trading without VWAP, you are trading blind against the big banks.',
            useCase: 'Institutional entry points and "Fair Value" assessment',
            difficulty: 'Beginner',
            icon: IconChartCandle,
            color: '#f59e0b',
            gradient: 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)',
            features: ['Institutional Support', 'Intraday Trend Filter', 'Fair Value Gap']
        },
        {
            id: '6',
            name: 'Stochastic Oscillator',
            slug: 'stochastic',
            category: 'Momentum',
            description: 'Faster than RSI, the Stochastic helps you time precision entries in chopping markets or pullbacks within a trend.',
            useCase: 'Timing entries on pullbacks and range trading',
            difficulty: 'Intermediate',
            icon: IconActivity,
            color: '#ec4899',
            gradient: 'linear-gradient(135deg, #ec4899 0%, #be185d 100%)',
            features: ['Fast vs Slow Lines', 'Hidden Divergence', 'Crossover Signals']
        }
    ];

    const faqs: FaqItem[] = [
        {
            question: "Do I need all these indicators to be profitable?",
            answer: "Absolutely not. In fact, using too many causes 'analysis paralysis.' In our trading room, we teach you to master 2-3 core tools (usually VWAP, one momentum indicator, and moving averages) to build a clean, actionable chart."
        },
        {
            question: "Are these indicators lagging?",
            answer: "Most indicators are lagging because they rely on past price data. However, we teach specific techniques—like divergence and multi-timeframe analysis—that turn these lagging tools into leading signals for future price action."
        },
        {
            question: "Which indicator is best for beginners?",
            answer: "We recommend starting with Moving Averages and VWAP. They provide an immediate visual guide to the trend and institutional value without complex calculations. They are the foundation of market structure."
        },
        {
            question: "Do you teach how to set these up?",
            answer: "Yes. When you join Revolution Trading Pros, you get our exact chart templates and settings. We don't just tell you what to use; we show you how to configure your platform to look exactly like ours."
        }
    ];

    const categories = ['All', 'Momentum', 'Trend Following', 'Volatility', 'Volume'];

    // --- State Management (Runes) ---
    let heroVisible = $state(false);
    let cardsVisible = $state<boolean[]>(new Array(indicators.length).fill(false));
    let selectedCategory = $state('All');
    let openFaq = $state<number | null>(null);

    let filteredIndicators = $derived(
        selectedCategory === 'All'
            ? indicators
            : indicators.filter((ind) => ind.category === selectedCategory)
    );

    function toggleFaq(index: number) {
        openFaq = openFaq === index ? null : index;
    }

    // --- Structured Data ---
    const indicatorsSchema = [
        {
            '@context': 'https://schema.org',
            '@type': 'ItemList',
            '@id': 'https://revolutiontradingpros.com/indicators/#indicatorlist',
            name: 'Professional Trading Indicators',
            description: 'Technical analysis indicators used by professional traders',
            numberOfItems: indicators.length,
            itemListElement: indicators.map((indicator, index) => ({
                '@type': 'ListItem',
                position: index + 1,
                item: {
                    '@type': 'SoftwareApplication',
                    name: indicator.name,
                    description: indicator.description,
                    url: `https://revolutiontradingpros.com/indicators/${indicator.slug}`,
                    applicationCategory: 'FinanceApplication',
                    operatingSystem: 'Web Browser',
                    offers: {
                        '@type': 'Offer',
                        price: '0',
                        priceCurrency: 'USD'
                    }
                }
            }))
        },
        {
            '@context': 'https://schema.org',
            '@type': 'FAQPage',
            mainEntity: faqs.map(faq => ({
                '@type': 'Question',
                name: faq.question,
                acceptedAnswer: {
                    '@type': 'Answer',
                    text: faq.answer
                }
            }))
        }
    ];

    onMount(() => {
        if (!browser) return;

        const heroObserver = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        heroVisible = true;
                    }
                });
            },
            { threshold: 0.2 }
        );

        const cardObserver = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        const index = parseInt(entry.target.getAttribute('data-index') || '0');
                        const newCardsVisible = [...cardsVisible];
                        newCardsVisible[index] = true;
                        cardsVisible = newCardsVisible;
                    }
                });
            },
            { threshold: 0.1 }
        );

        const heroElement = document.querySelector('.hero-section');
        if (heroElement) heroObserver.observe(heroElement);

        const cardElements = document.querySelectorAll('.indicator-card');
        cardElements.forEach((card) => cardObserver.observe(card));

        return () => {
            heroObserver.disconnect();
            cardObserver.disconnect();
        };
    });
</script>

<SEOHead
    title="Best Technical Indicators for Day Trading (2025 Guide)"
    description="Stop guessing. Master the RSI, VWAP, MACD, and Bollinger Bands with guidance from professional traders. Learn how to combine indicators for high-probability setups."
    canonical="/indicators"
    ogType="article"
    ogImage="/og-indicators-guide.webp"
    ogImageAlt="Technical Analysis Indicators Guide"
    keywords={[
        'trading indicators',
        'technical analysis',
        'RSI strategy',
        'VWAP trading',
        'MACD divergence',
        'day trading tools',
        'best indicators for day trading',
        'chart reading'
    ]}
    schema={indicatorsSchema}
/>

<div class="indicators-page">
    <section class="hero-section" class:visible={heroVisible}>
        <div class="hero-background">
            <div class="glow-orb glow-orb-1"></div>
            <div class="glow-orb glow-orb-2"></div>
            <div class="chart-lines">
                <div class="chart-line line-1"></div>
                <div class="chart-line line-2"></div>
                <div class="chart-line line-3"></div>
            </div>
        </div>

        <div class="hero-content">
            <div class="hero-badge">
                <IconChartLine size={20} stroke={2} />
                <span>The Professional Toolkit</span>
            </div>

            <h1 class="hero-title">
                Master the Tools<br />
                <span class="gradient-text">Pro Traders Use</span>
            </h1>

            <p class="hero-description">
                Stop looking for a "magic bullet." Successful trading isn't about finding the perfect indicator—it's about interpreting the data correctly. We teach you how to use institutional-grade tools like VWAP and RSI to read the market's narrative, not just its noise.
            </p>

            <div class="hero-cta-group">
                <button class="cta-button primary">
                    Join the Live Room
                    <IconArrowRight size={20} />
                </button>
                <button class="cta-button secondary">
                    Explore Indicators
                </button>
            </div>

            <div class="hero-stats">
                <div class="stat-item">
                    <IconTarget size={32} stroke={1.5} class="stat-icon" />
                    <div class="stat-content">
                        <div class="stat-value">Precision</div>
                        <div class="stat-label">Entries & Exits</div>
                    </div>
                </div>
                <div class="stat-item">
                    <IconBolt size={32} stroke={1.5} class="stat-icon" />
                    <div class="stat-content">
                        <div class="stat-value">Real-Time</div>
                        <div class="stat-label">Live Application</div>
                    </div>
                </div>
                <div class="stat-item">
                    <IconUsers size={32} stroke={1.5} class="stat-icon" />
                    <div class="stat-content">
                        <div class="stat-value">Community</div>
                        <div class="stat-label">Mentorship</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="truth-section">
        <div class="section-container">
            <div class="truth-grid">
                <div class="truth-content">
                    <h2 class="section-title left-align">The Truth About <span class="text-highlight">Technical Indicators</span></h2>
                    <p class="truth-text">
                        Most new traders fail because they treat indicators as "Go/Stop" signals. They buy when the RSI crosses 30, or sell when lines cross, without understanding context.
                    </p>
                    <p class="truth-text">
                        <strong>Here is the reality:</strong> Indicators are just derivatives of price. They are tools, not crystal balls. In our community, we teach you to use these tools to build a case—a "confluence" of evidence—that tilts the probabilities in your favor.
                    </p>
                    <ul class="truth-list">
                        <li>
                            <IconAlertTriangle size={24} class="warning-icon" />
                            <span>Stop chasing "lagging" signals blindly.</span>
                        </li>
                        <li>
                            <IconCheck size={24} class="check-icon" />
                            <span>Start using indicators to confirm price action.</span>
                        </li>
                        <li>
                            <IconCheck size={24} class="check-icon" />
                            <span>Learn to spot what the institutions are doing.</span>
                        </li>
                    </ul>
                </div>
                <div class="truth-visual">
                    <div class="chart-card glass-panel">
                        <div class="chart-header">
                            <span class="dot red"></span>
                            <span class="dot yellow"></span>
                            <span class="dot green"></span>
                        </div>
                        <div class="chart-mockup">
                            <div class="mockup-price"></div>
                            <div class="mockup-indicator"></div>
                            <div class="mockup-annotation">Divergence Detected</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="filter-section" id="indicators-list">
        <div class="filter-container">
            <h3>Explore Our Core Indicators</h3>
            <p class="filter-subtitle">These are the exact tools active in our trading room charts right now.</p>
            <div class="filter-buttons">
                {#each categories as category}
                    <button
                        class="filter-button"
                        class:active={selectedCategory === category}
                        onclick={() => (selectedCategory = category)}
                    >
                        {category}
                    </button>
                {/each}
            </div>
        </div>
    </section>

    <section class="indicators-section">
        <div class="section-container">
            <div class="indicators-grid">
                {#each filteredIndicators as indicator, index}
                    {@const Icon = indicator.icon}
                    <article
                        class="indicator-card"
                        class:visible={cardsVisible[index]}
                        data-index={index}
                        style="--delay: {index * 0.1}s; --card-color: {indicator.color}"
                    >
                        <div class="card-header" style="background: {indicator.gradient}">
                            <div class="card-icon">
                                <Icon size={48} stroke={1.5} />
                            </div>
                            <div class="card-category">{indicator.category}</div>
                        </div>

                        <div class="card-content">
                            <h3 class="card-title">{indicator.name}</h3>
                            <p class="card-description">{indicator.description}</p>

                            <div class="card-use-case">
                                <IconTarget size={18} stroke={2} />
                                <span>{indicator.useCase}</span>
                            </div>

                            <div class="card-meta">
                                <div class="meta-badge difficulty">
                                    <IconStar size={16} stroke={2} />
                                    <span>{indicator.difficulty}</span>
                                </div>
                            </div>

                            <ul class="card-features">
                                {#each indicator.features as feature}
                                    <li>
                                        <IconCheck size={16} stroke={2} />
                                        <span>{feature}</span>
                                    </li>
                                {/each}
                            </ul>

                            <a href="/indicators/{indicator.slug}" class="card-button">
                                View Strategy Guide
                                <IconArrowRight size={18} stroke={2} />
                            </a>
                        </div>
                    </article>
                {/each}
            </div>
        </div>
    </section>

    <section class="confluence-section">
        <div class="section-container">
            <h2 class="section-title">The Power of <span class="gradient-text">Confluence</span></h2>
            <p class="section-subtitle">A single indicator can be wrong. Three indicators telling the same story are rarely wrong. This is how we find high-probability trades.</p>

            <div class="confluence-grid">
                <div class="confluence-step">
                    <div class="step-number">1</div>
                    <h3>Trend</h3>
                    <p>We use <strong>Moving Averages</strong> to determine if the market is bullish or bearish. We never fight the trend.</p>
                </div>
                <div class="confluence-connector">+</div>
                <div class="confluence-step">
                    <div class="step-number">2</div>
                    <h3>Location</h3>
                    <p>We wait for price to return to value areas like <strong>VWAP</strong> or Support zones. We don't chase extended moves.</p>
                </div>
                <div class="confluence-connector">+</div>
                <div class="confluence-step">
                    <div class="step-number">3</div>
                    <h3>Momentum</h3>
                    <p>We execute when <strong>RSI or MACD</strong> confirms the buyers are stepping back in. This creates the "sniper" entry.</p>
                </div>
            </div>
            
            <div class="confluence-cta">
                <p>Want to see this confluence strategy in action?</p>
                <a href="/trading-room" class="text-link">Watch us trade this setup live tomorrow morning &rarr;</a>
            </div>
        </div>
    </section>

    <section class="faq-section">
        <div class="section-container">
            <h2 class="section-title">Common Questions</h2>
            <div class="faq-list">
                {#each faqs as faq, i}
                    <div class="faq-item" class:open={openFaq === i}>
                        <button class="faq-question" onclick={() => toggleFaq(i)}>
                            {faq.question}
                            <div class="faq-icon" class:open={openFaq === i}>
                                <IconChevronDown size={20} />
                            </div>
                        </button>
                        <div class="faq-answer">
                            <p>{faq.answer}</p>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="final-cta">
        <div class="cta-content">
            <IconSchool size={48} class="cta-icon" />
            <h2>Stop Learning Alone. Start Trading Together.</h2>
            <p>You have the tools. Now get the guidance. Join a community of traders who genuinely care about your success.</p>
            <button class="cta-button primary large">
                Join Revolution Trading Pros
            </button>
            <p class="cta-subtext">No spam. No fake alerts. Just real trading.</p>
        </div>
    </section>
</div>

<style>
    .indicators-page {
        min-height: 100vh;
        background: linear-gradient(to bottom, #0a0f1e 0%, #050812 100%);
        color: white;
        font-family: 'Inter', sans-serif; /* Assumed font */
    }

    /* Common Utilities */
    .section-container {
        max-width: 1400px;
        margin: 0 auto;
        padding: 0 2rem;
    }

    .section-title {
        font-size: clamp(2rem, 4vw, 3rem);
        font-weight: 800;
        text-align: center;
        margin-bottom: 1.5rem;
        line-height: 1.2;
    }
    
    .section-subtitle {
        text-align: center;
        max-width: 800px;
        margin: 0 auto 4rem;
        color: rgba(255,255,255,0.7);
        font-size: 1.125rem;
        line-height: 1.6;
    }

    .gradient-text {
        background: linear-gradient(135deg, #2e8eff 0%, #34d399 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    .text-highlight {
        color: #2e8eff;
    }

    /* Hero Section */
    .hero-section {
        position: relative;
        min-height: 90vh; /* Increased height */
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 8rem 2rem 6rem;
        overflow: hidden;
        opacity: 0;
        transform: translateY(30px);
        transition: opacity 0.8s ease, transform 0.8s ease;
    }

    .hero-section.visible {
        opacity: 1;
        transform: translateY(0);
    }

    .hero-background {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        overflow: hidden;
        z-index: 0;
    }

    .glow-orb {
        position: absolute;
        border-radius: 50%;
        filter: blur(80px);
        opacity: 0.3;
        animation: pulse 4s ease-in-out infinite;
    }

    .glow-orb-1 {
        width: 600px;
        height: 600px;
        background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
        top: -200px;
        right: -200px;
    }

    .glow-orb-2 {
        width: 500px;
        height: 500px;
        background: radial-gradient(circle, #34d399 0%, transparent 70%);
        bottom: -150px;
        left: -150px;
        animation-delay: 2s;
    }

    .chart-lines {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 100%;
        max-width: 1000px;
        height: 400px;
    }

    .chart-line {
        position: absolute;
        width: 100%;
        height: 2px;
        background: linear-gradient(90deg, transparent 0%, var(--line-color) 50%, transparent 100%);
        opacity: 0.2;
        animation: slideLine 3s ease-in-out infinite;
    }

    .line-1 { --line-color: #2e8eff; top: 30%; animation-delay: 0s; }
    .line-2 { --line-color: #34d399; top: 50%; animation-delay: 1s; }
    .line-3 { --line-color: #a78bfa; top: 70%; animation-delay: 2s; }

    @keyframes pulse {
        0%, 100% { transform: scale(1); opacity: 0.3; }
        50% { transform: scale(1.1); opacity: 0.5; }
    }

    @keyframes slideLine {
        0%, 100% { transform: translateX(-10%); opacity: 0.1; }
        50% { transform: translateX(10%); opacity: 0.3; }
    }

    .hero-content {
        position: relative;
        z-index: 1;
        max-width: 1000px;
        text-align: center;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .hero-badge {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1.25rem;
        background: rgba(46, 142, 255, 0.15);
        border: 1px solid rgba(46, 142, 255, 0.3);
        border-radius: 50px;
        font-size: 0.875rem;
        font-weight: 500;
        color: #2e8eff;
        margin-bottom: 2rem;
        backdrop-filter: blur(10px);
    }

    .hero-title {
        font-size: clamp(2.5rem, 5vw, 4.5rem);
        font-weight: 800;
        line-height: 1.1;
        margin-bottom: 1.5rem;
        letter-spacing: -0.02em;
    }

    .hero-description {
        font-size: clamp(1.1rem, 2vw, 1.25rem);
        line-height: 1.7;
        color: rgba(255, 255, 255, 0.8);
        max-width: 850px;
        margin: 0 auto 2.5rem;
    }

    /* CTA Buttons */
    .hero-cta-group {
        display: flex;
        gap: 1rem;
        margin-bottom: 4rem;
        flex-wrap: wrap;
        justify-content: center;
    }

    .cta-button {
        padding: 1rem 2rem;
        border-radius: 50px;
        font-weight: 700;
        font-size: 1rem;
        cursor: pointer;
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        text-decoration: none;
        border: none;
    }

    .cta-button.primary {
        background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
        color: white;
        box-shadow: 0 4px 15px rgba(46, 142, 255, 0.3);
    }

    .cta-button.primary:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(46, 142, 255, 0.4);
    }

    .cta-button.secondary {
        background: rgba(255,255,255,0.05);
        border: 1px solid rgba(255,255,255,0.1);
        color: white;
    }

    .cta-button.secondary:hover {
        background: rgba(255,255,255,0.1);
    }

    .hero-stats {
        display: flex;
        justify-content: center;
        gap: 3rem;
        flex-wrap: wrap;
        margin-top: 1rem;
        padding-top: 2rem;
        border-top: 1px solid rgba(255,255,255,0.1);
        width: 100%;
    }

    .stat-item {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .stat-content {
        text-align: left;
    }

    .stat-value {
        font-size: 1.25rem;
        font-weight: 700;
        color: white;
    }

    .stat-label {
        font-size: 0.875rem;
        color: rgba(255, 255, 255, 0.6);
    }

    /* Truth Section (New) */
    .truth-section {
        padding: 6rem 0;
        background: rgba(255,255,255,0.02);
    }

    .truth-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 4rem;
        align-items: center;
    }

    .left-align { text-align: left; }

    .truth-text {
        font-size: 1.1rem;
        line-height: 1.7;
        color: rgba(255,255,255,0.8);
        margin-bottom: 1.5rem;
    }

    .truth-list {
        list-style: none;
        padding: 0;
        margin-top: 2rem;
    }

    .truth-list li {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1rem;
        font-size: 1.1rem;
    }

    :global(.warning-icon) { color: #f59e0b; }
    :global(.check-icon) { color: #34d399; }

    .chart-card {
        background: rgba(0,0,0,0.3);
        border: 1px solid rgba(255,255,255,0.1);
        border-radius: 12px;
        padding: 1.5rem;
        height: 300px;
        position: relative;
    }
    
    .chart-header { display: flex; gap: 8px; margin-bottom: 2rem; }
    .dot { width: 10px; height: 10px; border-radius: 50%; display: block; }
    .red { background: #ef4444; } .yellow { background: #f59e0b; } .green { background: #22c55e; }

    .mockup-price {
        height: 2px;
        width: 100%;
        background: linear-gradient(90deg, transparent, #2e8eff, transparent);
        margin-bottom: 2rem;
    }

    .mockup-annotation {
        position: absolute;
        bottom: 2rem;
        right: 2rem;
        background: #34d399;
        color: #000;
        padding: 0.5rem 1rem;
        border-radius: 4px;
        font-weight: bold;
        font-size: 0.8rem;
    }

    /* Filter Section */
    .filter-section {
        padding: 6rem 2rem 2rem;
    }

    .filter-container {
        max-width: 1400px;
        margin: 0 auto;
        text-align: center;
    }

    .filter-container h3 {
        font-size: 2rem;
        font-weight: 700;
        margin-bottom: 0.5rem;
    }
    
    .filter-subtitle {
        color: rgba(255,255,255,0.6);
        margin-bottom: 2rem;
    }

    .filter-buttons {
        display: flex;
        gap: 1rem;
        justify-content: center;
        flex-wrap: wrap;
    }

    .filter-button {
        padding: 0.75rem 1.5rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 50px;
        color: rgba(255, 255, 255, 0.7);
        font-weight: 600;
        font-size: 0.9375rem;
        cursor: pointer;
        transition: all 0.3s ease;
    }

    .filter-button:hover {
        background: rgba(255, 255, 255, 0.1);
        border-color: rgba(46, 142, 255, 0.3);
        color: white;
    }

    .filter-button.active {
        background: linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%);
        border-color: #2e8eff;
        color: white;
    }

    /* Indicators Grid */
    .indicators-section {
        padding: 2rem 2rem 6rem;
    }

    .indicators-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(380px, 1fr));
        gap: 2rem;
    }

    .indicator-card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 20px;
        overflow: hidden;
        transition: all 0.4s ease;
        opacity: 0;
        transform: translateY(30px);
        display: flex;
        flex-direction: column;
    }

    .indicator-card.visible {
        opacity: 1;
        transform: translateY(0);
        transition-delay: var(--delay);
    }

    .indicator-card:hover {
        transform: translateY(-8px);
        border-color: var(--card-color);
        box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
    }

    .card-header {
        position: relative;
        padding: 2.5rem 2rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .card-icon {
        color: white;
        filter: drop-shadow(0 0 10px rgba(255, 255, 255, 0.3));
    }

    .card-category {
        position: absolute;
        top: 1rem;
        right: 1rem;
        padding: 0.4rem 1rem;
        background: rgba(0, 0, 0, 0.3);
        backdrop-filter: blur(10px);
        border-radius: 50px;
        font-size: 0.75rem;
        font-weight: 600;
        color: white;
    }

    .card-content {
        padding: 2rem;
        flex-grow: 1;
        display: flex;
        flex-direction: column;
    }

    .card-title {
        font-size: 1.5rem;
        font-weight: 700;
        margin-bottom: 1rem;
    }

    .card-description {
        font-size: 0.9375rem;
        line-height: 1.6;
        color: rgba(255, 255, 255, 0.7);
        margin-bottom: 1.5rem;
    }

    .card-use-case {
        display: flex;
        align-items: flex-start;
        gap: 0.75rem;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-left: 3px solid var(--card-color);
        border-radius: 8px;
        margin-bottom: 1.5rem;
        font-size: 0.875rem;
        color: rgba(255, 255, 255, 0.8);
    }

    .card-use-case :global(svg) {
        color: var(--card-color);
        flex-shrink: 0;
        margin-top: 2px;
    }

    .card-meta {
        display: flex;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .meta-badge {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 50px;
        font-size: 0.8125rem;
        font-weight: 600;
    }

    .meta-badge.difficulty :global(svg) {
        color: #f59e0b;
    }

    .card-features {
        list-style: none;
        padding: 0;
        margin: 0 0 2rem 0;
        flex-grow: 1;
    }

    .card-features li {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 0;
        font-size: 0.875rem;
        color: rgba(255, 255, 255, 0.8);
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    }

    .card-features li:last-child {
        border-bottom: none;
    }

    .card-features li :global(svg) {
        color: var(--card-color);
        flex-shrink: 0;
    }

    .card-button {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        width: 100%;
        justify-content: center;
        padding: 1rem 1.5rem;
        background: rgba(255, 255, 255, 0.05);
        color: white;
        text-decoration: none;
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 10px;
        font-weight: 600;
        font-size: 0.9375rem;
        transition: all 0.3s ease;
        margin-top: auto;
    }

    .card-button:hover {
        background: var(--card-color);
        border-color: var(--card-color);
        transform: translateX(4px);
        box-shadow: 0 5px 20px rgba(0, 0, 0, 0.3);
    }

    /* Confluence Section */
    .confluence-section {
        padding: 6rem 2rem;
        background: rgba(255,255,255,0.02);
        border-top: 1px solid rgba(255,255,255,0.05);
        border-bottom: 1px solid rgba(255,255,255,0.05);
    }

    .confluence-grid {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 2rem;
        margin: 4rem 0;
        flex-wrap: wrap;
    }

    .confluence-step {
        background: rgba(255,255,255,0.03);
        border: 1px solid rgba(255,255,255,0.1);
        padding: 2rem;
        border-radius: 16px;
        flex: 1;
        min-width: 250px;
        max-width: 350px;
        text-align: center;
        position: relative;
    }

    .step-number {
        background: #2e8eff;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        margin: -3rem auto 1.5rem;
        border: 4px solid #050812;
    }

    .confluence-connector {
        font-size: 3rem;
        color: rgba(255,255,255,0.2);
        font-weight: 100;
    }

    .confluence-cta {
        text-align: center;
        margin-top: 3rem;
    }

    .text-link {
        color: #34d399;
        font-weight: 600;
        text-decoration: none;
        font-size: 1.1rem;
    }
    .text-link:hover { text-decoration: underline; }

    /* FAQ Section */
    .faq-section {
        padding: 6rem 2rem;
        max-width: 900px;
        margin: 0 auto;
    }

    .faq-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .faq-item {
        background: rgba(255,255,255,0.03);
        border-radius: 12px;
        overflow: hidden;
        border: 1px solid rgba(255,255,255,0.05);
    }

    .faq-item.open {
        background: rgba(255,255,255,0.05);
        border-color: rgba(46,142,255,0.3);
    }

    .faq-question {
        width: 100%;
        text-align: left;
        padding: 1.5rem;
        background: none;
        border: none;
        color: white;
        font-weight: 600;
        font-size: 1.1rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        cursor: pointer;
    }

    .faq-icon {
        transition: transform 0.3s ease;
    }

    .faq-icon.open {
        transform: rotate(180deg);
    }

    .faq-answer {
        max-height: 0;
        overflow: hidden;
        transition: max-height 0.3s ease;
        padding: 0 1.5rem;
        color: rgba(255,255,255,0.7);
        line-height: 1.6;
    }

    .faq-item.open .faq-answer {
        max-height: 200px; /* Adjust as needed */
        padding-bottom: 1.5rem;
    }

    /* Final CTA */
    .final-cta {
        padding: 6rem 2rem;
        text-align: center;
        background: radial-gradient(circle at center, rgba(46,142,255,0.1) 0%, transparent 70%);
    }

    .cta-content {
        max-width: 700px;
        margin: 0 auto;
    }

    :global(.cta-icon) {
        color: #2e8eff;
        margin-bottom: 1.5rem;
    }

    .final-cta h2 {
        font-size: clamp(2rem, 3vw, 2.5rem);
        font-weight: 800;
        margin-bottom: 1rem;
    }

    .final-cta p {
        color: rgba(255,255,255,0.7);
        font-size: 1.1rem;
        margin-bottom: 2rem;
    }

    .cta-button.large {
        padding: 1.25rem 3rem;
        font-size: 1.125rem;
    }

    .cta-subtext {
        font-size: 0.875rem;
        color: rgba(255,255,255,0.5);
        margin-top: 1rem;
    }

    /* Responsive */
    @media (max-width: 1200px) {
        .glow-orb-1 { width: 500px; height: 500px; }
        .glow-orb-2 { width: 450px; height: 450px; }
    }

    @media (max-width: 1024px) {
        .truth-grid { grid-template-columns: 1fr; gap: 2rem; }
        .confluence-grid { flex-direction: column; gap: 1rem; }
        .confluence-connector { transform: rotate(90deg); margin: 0; font-size: 2rem; }
    }

    @media (max-width: 768px) {
        .hero-section { padding: 6rem 1.5rem 4rem; min-height: auto; }
        .indicators-grid { grid-template-columns: 1fr; }
        .hero-stats { flex-direction: column; gap: 1.5rem; }
        .stat-item { justify-content: center; }
        .filter-section { padding: 4rem 1rem 1rem; }
    }
</style>