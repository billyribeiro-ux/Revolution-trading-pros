<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    
    // Icons
    import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconTrendingDown from '@tabler/icons-svelte/icons/trending-down';
    import IconActivity from '@tabler/icons-svelte/icons/activity';
    import IconTarget from '@tabler/icons-svelte/icons/target';
    import IconBolt from '@tabler/icons-svelte/icons/bolt';
    import IconStar from '@tabler/icons-svelte/icons/star';
    import IconCheck from '@tabler/icons-svelte/icons/check';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
    import IconWaveSine from '@tabler/icons-svelte/icons/wave-sine';
    import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
    import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
    import IconUsers from '@tabler/icons-svelte/icons/users';
    import IconSchool from '@tabler/icons-svelte/icons/school';
    import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
    import IconChevronUp from '@tabler/icons-svelte/icons/chevron-up';
    import IconClock from '@tabler/icons-svelte/icons/clock';
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

    interface SetupItem {
        title: string;
        value: string;
        detail: string;
        icon: any;
    }

    // --- Expanded Data ---
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
            features: ['Bullish/Bearish Divergence', 'Trend Reset Zones (50 line)', 'Momentum Confirmation']
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

    // New Data for "Golden Setup"
    const goldenSetup: SetupItem[] = [
        { title: 'Primary Trend', value: '1 Hour Chart', detail: 'Above 200 SMA = Bullish', icon: IconClock },
        { title: 'Entry Trigger', value: '5 Min Chart', detail: 'VWAP Reclaim + RSI > 50', icon: IconTarget },
        { title: 'Risk Management', value: '9 EMA Trailing', detail: 'Close below 9 EMA = Exit', icon: IconAlertTriangle },
        { title: 'Momentum', value: 'RSI (14)', detail: 'Look for Divergence', icon: IconActivity }
    ];

    const faqs: FaqItem[] = [
        {
            question: "Do I need all these indicators to be profitable?",
            answer: "Absolutely not. In fact, using too many causes 'analysis paralysis.' In our trading room, we teach you to master 2-3 core tools (usually VWAP, one momentum indicator, and moving averages) to build a clean, actionable chart."
        },
        {
            question: "Are these indicators lagging?",
            answer: "Most indicators are lagging because they rely on past price data. However, we teach specific techniques—like divergence and multi-timeframe analysis—that turn these lagging tools into leading signals for future price action. Divergence often precedes price turns."
        },
        {
            question: "Which indicator is best for beginners?",
            answer: "We recommend starting with Moving Averages and VWAP. They provide an immediate visual guide to the trend and institutional value without complex calculations. They are the foundation of market structure."
        },
        {
            question: "What is the best RSI setting?",
            answer: "Stick to the default 14-period RSI. This is what the algorithms and institutional programs use. Changing it to 7 or 21 often results in curve-fitting. We want to see what the rest of the market sees."
        },
        {
            question: "Do you teach how to set these up?",
            answer: "Yes. When you join Revolution Trading Pros, you get our exact chart templates and settings. We don't just tell you what to use; we show you how to configure your platform (TradingView or TOS) to look exactly like ours."
        },
        {
            question: "Can I use these for Crypto and Forex?",
            answer: "Yes. Technical analysis is universal. RSI divergence works on Bitcoin just as well as it works on Apple. However, VWAP is most effective in markets with centralized volume (Stocks/Futures)."
        }
    ];

    const categories = ['All', 'Momentum', 'Trend Following', 'Volatility', 'Volume'];

    // --- State Management (Svelte 5 Runes) ---
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
    
    // --- Mouse Spotlight Logic ---
    function handleMouseMove(e: MouseEvent) {
        if (!browser) return;
        const target = e.currentTarget as HTMLElement;
        const rect = target.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        target.style.setProperty('--mouse-x', `${x}px`);
        target.style.setProperty('--mouse-y', `${y}px`);
    }

    // --- Structured Data ---
    const indicatorsSchema = {
        '@context': 'https://schema.org',
        '@graph': [
            {
                '@type': 'ItemList',
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
                        applicationCategory: 'FinanceApplication',
                        operatingSystem: 'Web Browser'
                    }
                }))
            },
            {
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
        ]
    };

    onMount(() => {
        if (!browser) return;

        let heroObserver: IntersectionObserver;
        let cardObserver: IntersectionObserver;
        let scrollTriggerCleanup: (() => void) | null = null;

        // Use IIFE for async operations
        (async () => {
            // Dynamically import GSAP to avoid SSR issues
            const { gsap } = await import('gsap');
            const ScrollTrigger = (await import('gsap/ScrollTrigger')).default;
            gsap.registerPlugin(ScrollTrigger);

            // Store cleanup reference
            scrollTriggerCleanup = () => ScrollTrigger.getAll().forEach(st => st.kill());

            // --- Cinematic GSAP Animations ---
            
            // 1. Hero Parallax & Reveal
            const tlHero = gsap.timeline();
            tlHero.to('.hero-title span', {
                backgroundPosition: "200% center",
                duration: 2,
                ease: "none",
                repeat: -1
            });

            // 2. Truth Section - Animated Chart Drawing
            const chartPath = document.querySelector('.mockup-price-path');
            const indicatorPath = document.querySelector('.mockup-indicator-path');
            const annotation = document.querySelector('.mockup-annotation');

            if (chartPath && indicatorPath) {
                const chartLength = (chartPath as SVGPathElement).getTotalLength();
                const indLength = (indicatorPath as SVGPathElement).getTotalLength();

                gsap.set(chartPath, { strokeDasharray: chartLength, strokeDashoffset: chartLength });
                gsap.set(indicatorPath, { strokeDasharray: indLength, strokeDashoffset: indLength });
                gsap.set(annotation, { opacity: 0, scale: 0.8, y: 10 });

                ScrollTrigger.create({
                    trigger: '.truth-section',
                    start: "top 60%",
                    onEnter: () => {
                        gsap.to(chartPath, { strokeDashoffset: 0, duration: 2, ease: "power2.out" });
                        gsap.to(indicatorPath, { strokeDashoffset: 0, duration: 2, delay: 0.2, ease: "power2.out" });
                        gsap.to(annotation, { opacity: 1, scale: 1, y: 0, duration: 0.5, delay: 1.8, ease: "back.out(1.7)" });
                    }
                });
            }

            // 3. Confluence Flow Animation
            const connectors = document.querySelectorAll('.confluence-connector');
            const steps = document.querySelectorAll('.confluence-step');
            
            const tlConfluence = gsap.timeline({
                scrollTrigger: {
                    trigger: '.confluence-section',
                    start: "top 70%"
                }
            });

            if (steps.length > 0) {
                tlConfluence.from(steps[0], { y: 30, opacity: 0, duration: 0.5 })
                    .from(connectors[0], { scale: 0, opacity: 0, duration: 0.3 }, "-=0.1")
                    .from(steps[1], { y: 30, opacity: 0, duration: 0.5 }, "-=0.1")
                    .from(connectors[1], { scale: 0, opacity: 0, duration: 0.3 }, "-=0.1")
                    .from(steps[2], { y: 30, opacity: 0, duration: 0.5 }, "-=0.1");
            }
            
            // 4. Setup Section Animation
            const setupItems = document.querySelectorAll('.setup-item');
            gsap.from(setupItems, {
                scrollTrigger: {
                    trigger: '.setup-grid',
                    start: "top 80%"
                },
                y: 20,
                opacity: 0,
                duration: 0.4,
                stagger: 0.1,
                ease: "power2.out"
            });
        })();

        // --- Original Observer Logic (Preserved for compatibility) ---
        heroObserver = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        heroVisible = true;
                    }
                });
            },
            { threshold: 0.2 }
        );

        cardObserver = new IntersectionObserver(
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
            if (scrollTriggerCleanup) scrollTriggerCleanup();
        };
    });
</script>

<SEOHead
    title="Best Technical Indicators for Day Trading (2025 Guide)"
    description="Stop guessing. Master the RSI, VWAP, MACD, and Bollinger Bands. Get the exact 'Golden Setup' configurations used by professional traders."
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
        'chart reading',
        'golden setup'
    ]}
    schema={indicatorsSchema['@graph']}
/>

<div class="indicators-page antialiased">
    <div class="fixed inset-0 pointer-events-none opacity-[0.03] z-[1] mix-blend-overlay" style="background-image: url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0IiBoZWlnaHQ9IjQiPgo8cmVjdCB3aWR0aD0iNCIgaGVpZ2h0PSI0IiBmaWxsPSIjZmZmIi8+CjxyZWN0IHdpZHRoPSIxIiBoZWlnaHQ9IjEiIGZpbGw9IiMwMDAiLz4KPC9zdmc+');"></div>

    <section class="hero-section relative" class:visible={heroVisible}>
        <div class="hero-background">
            <div class="glow-orb glow-orb-1 mix-blend-screen"></div>
            <div class="glow-orb glow-orb-2 mix-blend-screen"></div>
            <div class="glow-orb glow-orb-3 mix-blend-screen"></div>
            
            <div class="absolute inset-0 perspective-grid opacity-20"></div>

            <div class="chart-lines">
                <svg class="absolute inset-0 w-full h-full" preserveAspectRatio="none">
                    <path class="chart-line-svg line-1" d="M0,300 Q400,250 800,350 T1600,200" fill="none" stroke="#2e8eff" stroke-width="2" />
                    <path class="chart-line-svg line-2" d="M0,350 Q400,300 800,400 T1600,250" fill="none" stroke="#34d399" stroke-width="2" />
                </svg>
            </div>
        </div>

        <div class="hero-content relative z-10 px-4">
            <div class="hero-badge animate-float">
                <IconChartLine size={18} stroke={2} />
                <span class="tracking-wide text-xs uppercase font-bold">The Professional Toolkit</span>
            </div>

            <h1 class="hero-title tracking-tight">
                Master the Tools<br />
                <span class="gradient-text bg-clip-text text-transparent bg-gradient-to-r from-blue-400 via-emerald-400 to-blue-400 bg-[length:200%_auto]">Pro Traders Use</span>
            </h1>

            <p class="hero-description text-lg md:text-xl text-slate-300/90 leading-relaxed">
                Stop looking for a "magic bullet." Successful trading isn't about finding the perfect indicator—it's about interpreting the data correctly. We teach you how to use institutional-grade tools like VWAP and RSI to read the market's narrative, not just its noise.
            </p>

            <div class="hero-cta-group pt-4">
                <button class="cta-button primary group relative overflow-hidden">
                    <span class="relative z-10 flex items-center gap-2">
                        Join the Live Room
                        <IconArrowRight size={20} class="transition-transform group-hover:translate-x-1" />
                    </span>
                    <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300 ease-out"></div>
                </button>
                <button class="cta-button secondary backdrop-blur-md">
                    Explore Indicators
                </button>
            </div>

            <div class="hero-stats border-t border-white/10 mt-12 pt-8">
                <div class="stat-item group">
                    <div class="p-3 rounded-full bg-white/5 group-hover:bg-blue-500/10 transition-colors">
                        <IconTarget size={28} stroke={1.5} class="stat-icon text-blue-400" />
                    </div>
                    <div class="stat-content">
                        <div class="stat-value text-white font-mono">Precision</div>
                        <div class="stat-label text-slate-400">Entries & Exits</div>
                    </div>
                </div>
                <div class="stat-item group">
                    <div class="p-3 rounded-full bg-white/5 group-hover:bg-emerald-500/10 transition-colors">
                        <IconBolt size={28} stroke={1.5} class="stat-icon text-emerald-400" />
                    </div>
                    <div class="stat-content">
                        <div class="stat-value text-white font-mono">Real-Time</div>
                        <div class="stat-label text-slate-400">Live Application</div>
                    </div>
                </div>
                <div class="stat-item group">
                    <div class="p-3 rounded-full bg-white/5 group-hover:bg-purple-500/10 transition-colors">
                        <IconUsers size={28} stroke={1.5} class="stat-icon text-purple-400" />
                    </div>
                    <div class="stat-content">
                        <div class="stat-value text-white font-mono">Community</div>
                        <div class="stat-label text-slate-400">Mentorship</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="setup-section relative">
         <div class="section-container">
            <div class="setup-header text-center mb-12">
                <h2 class="section-title tracking-tight mb-4">The "Golden Setup"</h2>
                <p class="section-subtitle mb-8">
                    You don't need 20 indicators. You need 3 that work. Here is the exact configuration we use every day.
                </p>
            </div>
            
            <div class="setup-grid grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {#each goldenSetup as item}
                    {@const Icon = item.icon}
                    <div class="setup-item bg-white/5 backdrop-blur-md border border-white/10 rounded-2xl p-6 hover:border-blue-500/30 transition-all duration-300 group">
                        <div class="setup-icon-wrapper mb-4 w-12 h-12 rounded-xl bg-blue-500/10 flex items-center justify-center group-hover:scale-110 transition-transform">
                             <Icon size={24} class="text-blue-400" />
                        </div>
                        <h3 class="text-slate-400 text-sm font-medium uppercase tracking-wide mb-2">{item.title}</h3>
                        <div class="text-xl font-bold text-white mb-1">{item.value}</div>
                        <div class="text-xs text-emerald-400 font-mono bg-emerald-500/10 inline-block px-2 py-1 rounded border border-emerald-500/20">{item.detail}</div>
                    </div>
                {/each}
            </div>
         </div>
    </section>

    <section class="truth-section relative overflow-hidden">
        <div class="absolute right-0 top-0 w-1/3 h-full bg-gradient-to-l from-blue-900/10 to-transparent pointer-events-none"></div>

        <div class="section-container relative z-10">
            <div class="truth-grid">
                <div class="truth-content">
                    <h2 class="section-title left-align tracking-tighter">The Truth About <span class="text-highlight">Technical Indicators</span></h2>
                    <p class="truth-text text-slate-300">
                        Most new traders fail because they treat indicators as "Go/Stop" signals. They buy when the RSI crosses 30, or sell when lines cross, without understanding context.
                    </p>
                    <p class="truth-text text-slate-300">
                        <strong>Here is the reality:</strong> Indicators are just derivatives of price. They are tools, not crystal balls. In our community, we teach you to use these tools to build a case—a "confluence" of evidence—that tilts the probabilities in your favor.
                    </p>
                    <ul class="truth-list space-y-4">
                        <li class="bg-white/5 p-4 rounded-lg border border-white/5">
                            <IconAlertTriangle size={24} class="warning-icon shrink-0" />
                            <span>Stop chasing "lagging" signals blindly.</span>
                        </li>
                        <li class="bg-white/5 p-4 rounded-lg border border-white/5">
                            <IconCheck size={24} class="check-icon shrink-0" />
                            <span>Start using indicators to confirm price action.</span>
                        </li>
                        <li class="bg-white/5 p-4 rounded-lg border border-white/5">
                            <IconCheck size={24} class="check-icon shrink-0" />
                            <span>Learn to spot what the institutions are doing.</span>
                        </li>
                    </ul>
                </div>
                <div class="truth-visual">
                    <div class="chart-card glass-panel shadow-2xl shadow-blue-900/20 backdrop-blur-xl">
                        <div class="chart-header flex items-center justify-between border-b border-white/10 pb-4 mb-6">
                            <div class="flex gap-2">
                                <span class="dot red"></span>
                                <span class="dot yellow"></span>
                                <span class="dot green"></span>
                            </div>
                            <div class="text-xs font-mono text-slate-500">M15 • ES_F</div>
                        </div>
                        <div class="chart-mockup h-[220px] relative w-full">
                            <svg class="w-full h-full overflow-visible" viewBox="0 0 400 200">
                                <defs>
                                    <linearGradient id="chartGrad" x1="0%" y1="0%" x2="0%" y2="100%">
                                        <stop offset="0%" stop-color="#2e8eff" stop-opacity="0.3"/>
                                        <stop offset="100%" stop-color="#2e8eff" stop-opacity="0"/>
                                    </linearGradient>
                                </defs>
                                <path class="mockup-price-path" d="M0,150 L50,140 L80,160 L120,100 L160,120 L220,50 L260,80 L350,20" 
                                      fill="none" stroke="#2e8eff" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
                                
                                <path class="mockup-indicator-path" d="M0,180 L50,175 L80,185 L120,170 L160,175 L220,172 L260,178 L350,180" 
                                      fill="none" stroke="#f59e0b" stroke-width="2" stroke-dasharray="4 4" opacity="0.8" />
                            </svg>
                            
                            <div class="mockup-annotation absolute top-4 right-8 bg-emerald-500 text-black font-bold px-3 py-1 rounded shadow-lg transform origin-bottom-left">
                                <div class="flex items-center gap-1 text-xs">
                                    <IconAlertTriangle size={14} />
                                    <span>Bearish Divergence</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="filter-section" id="indicators-list">
        <div class="filter-container">
            <h3 class="tracking-tight">Explore Our Core Indicators</h3>
            <p class="filter-subtitle text-lg">These are the exact tools active in our trading room charts right now.</p>
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
                        class="indicator-card group relative"
                        class:visible={cardsVisible[index]}
                        data-index={index}
                        onmousemove={handleMouseMove}
                        style="--delay: {index * 0.1}s; --card-color: {indicator.color};"
                    >
                        <div class="spotlight-overlay opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>

                        <div class="card-header relative overflow-hidden" style="background: {indicator.gradient}">
                            <div class="absolute inset-0 bg-noise opacity-20"></div>
                            
                            <div class="card-icon relative z-10 transform transition-transform duration-500 group-hover:scale-110 group-hover:rotate-3">
                                <Icon size={48} stroke={1.5} />
                            </div>
                            <div class="card-category shadow-lg">{indicator.category}</div>
                        </div>

                        <div class="card-content bg-[#0B101E]">
                            <h3 class="card-title text-white group-hover:text-[var(--card-color)] transition-colors duration-300">{indicator.name}</h3>
                            <p class="card-description">{indicator.description}</p>

                            <div class="card-use-case bg-white/5 border-l-2 border-[var(--card-color)]">
                                <IconTarget size={18} stroke={2} />
                                <span>{indicator.useCase}</span>
                            </div>

                            <div class="card-meta">
                                <div class="meta-badge difficulty border border-white/10">
                                    <IconStar size={16} stroke={2} />
                                    <span>{indicator.difficulty}</span>
                                </div>
                            </div>

                            <ul class="card-features">
                                {#each indicator.features as feature}
                                    <li class="group-hover:pl-1 transition-all duration-300">
                                        <IconCheck size={16} stroke={2} />
                                        <span>{feature}</span>
                                    </li>
                                {/each}
                            </ul>

                            <a href="/indicators/{indicator.slug}" class="card-button group/btn">
                                View Strategy Guide
                                <IconArrowRight size={18} stroke={2} class="transition-transform group-hover/btn:translate-x-1" />
                            </a>
                        </div>
                    </article>
                {/each}
            </div>
        </div>
    </section>

    <section class="confluence-section relative">
        <div class="absolute inset-0 bg-gradient-to-b from-transparent via-blue-900/5 to-transparent pointer-events-none"></div>
        <div class="section-container relative z-10">
            <h2 class="section-title tracking-tight">The Power of <span class="gradient-text">Confluence</span></h2>
            <p class="section-subtitle">A single indicator can be wrong. Three indicators telling the same story are rarely wrong. This is how we find high-probability trades.</p>

            <div class="confluence-grid">
                <div class="confluence-step hover:border-blue-500/50 transition-colors duration-300">
                    <div class="step-number shadow-lg shadow-blue-500/30">1</div>
                    <h3 class="text-xl font-bold mb-2">Trend</h3>
                    <p class="text-slate-400 text-sm">We use <strong>Moving Averages</strong> to determine if the market is bullish or bearish. We never fight the trend.</p>
                </div>
                
                <div class="confluence-connector hidden md:block">
                     <IconArrowRight size={32} class="text-slate-600 animate-pulse" />
                </div>
                <div class="confluence-connector md:hidden rotate-90">
                     <IconArrowRight size={32} class="text-slate-600 animate-pulse" />
                </div>

                <div class="confluence-step hover:border-emerald-500/50 transition-colors duration-300">
                    <div class="step-number bg-emerald-500 shadow-lg shadow-emerald-500/30">2</div>
                    <h3 class="text-xl font-bold mb-2">Location</h3>
                    <p class="text-slate-400 text-sm">We wait for price to return to value areas like <strong>VWAP</strong> or Support zones. We don't chase extended moves.</p>
                </div>

                <div class="confluence-connector hidden md:block">
                     <IconArrowRight size={32} class="text-slate-600 animate-pulse" />
                </div>
                <div class="confluence-connector md:hidden rotate-90">
                     <IconArrowRight size={32} class="text-slate-600 animate-pulse" />
                </div>

                <div class="confluence-step hover:border-purple-500/50 transition-colors duration-300">
                    <div class="step-number bg-purple-500 shadow-lg shadow-purple-500/30">3</div>
                    <h3 class="text-xl font-bold mb-2">Momentum</h3>
                    <p class="text-slate-400 text-sm">We execute when <strong>RSI or MACD</strong> confirms the buyers are stepping back in. This creates the "sniper" entry.</p>
                </div>
            </div>
            
            <div class="confluence-cta mt-12">
                <p class="text-slate-400 mb-2">Want to see this confluence strategy in action?</p>
                <a href="/trading-room" class="text-link inline-flex items-center gap-2 group">
                    Watch us trade this setup live tomorrow morning 
                    <IconArrowRight size={18} class="transition-transform group-hover:translate-x-1" />
                </a>
            </div>
        </div>
    </section>

    <section class="faq-section">
        <div class="section-container">
            <h2 class="section-title tracking-tight">Common Questions</h2>
            <div class="faq-list">
                {#each faqs as faq, i}
                    <div class="faq-item transition-all duration-300" class:open={openFaq === i}>
                        <button class="faq-question hover:text-blue-400 transition-colors" onclick={() => toggleFaq(i)}>
                            {faq.question}
                            <div class="faq-icon transform transition-transform duration-300" class:rotate-180={openFaq === i}>
                                <IconChevronDown size={20} />
                            </div>
                        </button>
                        <div class="faq-answer">
                            <p class="text-slate-300">{faq.answer}</p>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="final-cta relative overflow-hidden">
        <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[400px] bg-blue-600/10 blur-[120px] rounded-full pointer-events-none"></div>
        
        <div class="cta-content relative z-10">
            <div class="inline-block p-4 rounded-full bg-blue-500/10 mb-6">
                <IconSchool size={48} class="cta-icon text-blue-400 !mb-0" />
            </div>
            <h2 class="tracking-tight">Stop Learning Alone. Start Trading Together.</h2>
            <p>You have the tools. Now get the guidance. Join a community of traders who genuinely care about your success.</p>
            <button class="cta-button primary large transform hover:scale-105 transition-transform duration-300 shadow-xl shadow-blue-500/20">
                Join Revolution Trading Pros
            </button>
            <p class="cta-subtext">No spam. No fake alerts. Just real trading.</p>
        </div>
    </section>
</div>

<style>
    /* * RESTORED CUSTOM CSS ARCHITECTURE 
     * (Preserving the original animations, layout logic, and visual fidelity)
     */

    .indicators-page {
        min-height: 100vh;
        background: #050812;
        color: white;
        font-family: 'Inter', system-ui, sans-serif;
        overflow-x: hidden;
    }

    /* Cinematic Spotlight Effect */
    .spotlight-overlay {
        pointer-events: none;
        position: absolute;
        inset: 0;
        background: radial-gradient(
            600px circle at var(--mouse-x) var(--mouse-y),
            rgba(255, 255, 255, 0.06),
            transparent 40%
        );
        z-index: 2;
    }

    .bg-noise {
        background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)' opacity='1'/%3E%3C/svg%3E");
    }

    /* Common Utilities */
    .section-container {
        max-width: 1400px;
        margin: 0 auto;
        padding: 0 2rem;
    }

    .section-title {
        font-size: clamp(2rem, 4vw, 3.5rem);
        font-weight: 800;
        text-align: center;
        margin-bottom: 1.5rem;
        line-height: 1.1;
    }
    
    .section-subtitle {
        text-align: center;
        max-width: 800px;
        margin: 0 auto 4rem;
        color: #94a3b8;
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
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 8rem 2rem 6rem;
        overflow: hidden;
        opacity: 0;
        transform: translateY(30px);
        transition: opacity 1s cubic-bezier(0.16, 1, 0.3, 1), transform 1s cubic-bezier(0.16, 1, 0.3, 1);
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

    .perspective-grid {
        background-image: linear-gradient(rgba(46, 142, 255, 0.1) 1px, transparent 1px),
        linear-gradient(90deg, rgba(46, 142, 255, 0.1) 1px, transparent 1px);
        background-size: 40px 40px;
        transform: perspective(500px) rotateX(60deg) translateY(-100px) scale(2);
        animation: gridMove 20s linear infinite;
        mask-image: linear-gradient(to bottom, transparent, black 40%, transparent);
    }

    @keyframes gridMove {
        0% { transform: perspective(500px) rotateX(60deg) translateY(0) scale(2); }
        100% { transform: perspective(500px) rotateX(60deg) translateY(40px) scale(2); }
    }

    .glow-orb {
        position: absolute;
        border-radius: 50%;
        filter: blur(100px);
        opacity: 0.4;
        animation: pulse 8s ease-in-out infinite;
    }

    .glow-orb-1 {
        width: 800px;
        height: 800px;
        background: radial-gradient(circle, #2e8eff 0%, transparent 70%);
        top: -300px;
        right: -200px;
    }

    .glow-orb-2 {
        width: 600px;
        height: 600px;
        background: radial-gradient(circle, #34d399 0%, transparent 70%);
        bottom: -250px;
        left: -150px;
        animation-delay: 2s;
    }

    .glow-orb-3 {
        width: 400px;
        height: 400px;
        background: radial-gradient(circle, #a78bfa 0%, transparent 70%);
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        animation-delay: 4s;
        opacity: 0.2;
    }

    .chart-lines {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
    }
    
    .chart-line-svg {
        stroke-dasharray: 2000;
        stroke-dashoffset: 2000;
        animation: drawLine 4s ease-out forwards infinite;
        opacity: 0.3;
    }
    
    .line-2 { animation-delay: 2s; }

    @keyframes pulse {
        0%, 100% { transform: scale(1); opacity: 0.3; }
        50% { transform: scale(1.1); opacity: 0.5; }
    }

    @keyframes drawLine {
        0% { stroke-dashoffset: 2000; opacity: 0; }
        20% { opacity: 0.4; }
        100% { stroke-dashoffset: 0; opacity: 0; }
    }

    .animate-float {
        animation: float 6s ease-in-out infinite;
    }

    @keyframes float {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-10px); }
    }

    .hero-content {
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
        background: rgba(46, 142, 255, 0.1);
        border: 1px solid rgba(46, 142, 255, 0.2);
        border-radius: 50px;
        color: #60a5fa;
        margin-bottom: 2rem;
        backdrop-filter: blur(10px);
    }

    .hero-title {
        font-size: clamp(3rem, 6vw, 5.5rem);
        font-weight: 800;
        line-height: 1;
        margin-bottom: 1.5rem;
        letter-spacing: -0.03em;
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
        padding: 1rem 2.5rem;
        border-radius: 50px;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        text-decoration: none;
        border: none;
        letter-spacing: -0.01em;
    }

    .cta-button.primary {
        background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
        color: white;
        box-shadow: 0 4px 15px rgba(37, 99, 235, 0.3);
        border: 1px solid rgba(255,255,255,0.1);
    }

    .cta-button.primary:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(37, 99, 235, 0.4);
    }

    .cta-button.secondary {
        background: rgba(255,255,255,0.03);
        border: 1px solid rgba(255,255,255,0.08);
        color: white;
    }

    .cta-button.secondary:hover {
        background: rgba(255,255,255,0.08);
        border-color: rgba(255,255,255,0.2);
    }

    .hero-stats {
        display: flex;
        justify-content: center;
        gap: 4rem;
        flex-wrap: wrap;
        width: 100%;
    }

    .stat-item {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    /* Setup Section (New) */
    .setup-section {
        padding: 6rem 0;
        background: linear-gradient(to bottom, #050812, #080c18);
        border-top: 1px solid rgba(255,255,255,0.03);
    }
    
    .setup-item {
        transition: transform 0.3s ease, border-color 0.3s ease;
    }
    
    .setup-item:hover {
        transform: translateY(-5px);
    }

    /* Truth Section */
    .truth-section {
        padding: 8rem 0;
        background: linear-gradient(to bottom, #050812, #0a0f1e);
    }

    .truth-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 6rem;
        align-items: center;
    }

    .left-align { text-align: left; }

    .truth-text {
        font-size: 1.125rem;
        line-height: 1.8;
        margin-bottom: 1.5rem;
    }

    .truth-list li {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    :global(.warning-icon) { color: #f59e0b; }
    :global(.check-icon) { color: #34d399; }

    .chart-card {
        background: rgba(13, 18, 30, 0.6);
        border: 1px solid rgba(255,255,255,0.05);
        border-radius: 24px;
        padding: 2rem;
        height: auto;
        position: relative;
    }
    
    .dot { width: 10px; height: 10px; border-radius: 50%; display: block; }
    .red { background: #ef4444; } .yellow { background: #f59e0b; } .green { background: #22c55e; }

    /* Filter Section */
    .filter-section {
        padding: 6rem 2rem 2rem;
    }

    .filter-container {
        max-width: 1400px;
        margin: 0 auto;
        text-align: center;
    }

    .filter-buttons {
        display: flex;
        gap: 0.75rem;
        justify-content: center;
        flex-wrap: wrap;
    }

    .filter-button {
        padding: 0.6rem 1.5rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 50px;
        color: #94a3b8;
        font-weight: 500;
        font-size: 0.9375rem;
        cursor: pointer;
        transition: all 0.3s ease;
    }

    .filter-button:hover {
        background: rgba(255, 255, 255, 0.08);
        color: white;
    }

    .filter-button.active {
        background: #2e8eff;
        border-color: #2e8eff;
        color: white;
        box-shadow: 0 0 20px rgba(46, 142, 255, 0.3);
    }

    /* Indicators Grid */
    .indicators-section {
        padding: 2rem 2rem 8rem;
    }

    .indicators-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 320px), 1fr));
        gap: 1.5rem;
    }

    .indicator-card {
        background: #0B101E;
        border: 1px solid rgba(255, 255, 255, 0.05);
        border-radius: 24px;
        overflow: hidden;
        transition: transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1), opacity 0.6s ease;
        opacity: 0;
        transform: translateY(30px);
        display: flex;
        flex-direction: column;
        box-shadow: 0 0 0 1px rgba(255,255,255,0.02);
    }

    .indicator-card.visible {
        opacity: 1;
        transform: translateY(0);
        transition-delay: var(--delay);
    }

    .indicator-card:hover {
        transform: translateY(-8px) scale(1.01);
        border-color: rgba(255,255,255,0.1);
        box-shadow: 0 20px 50px -10px rgba(0, 0, 0, 0.5);
    }

    .card-header {
        padding: 2.5rem 2rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .card-content {
        padding: 2rem;
        flex-grow: 1;
        display: flex;
        flex-direction: column;
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
        padding: 0.25rem 0.75rem;
        background: rgba(255, 255, 255, 0.03);
        border-radius: 6px;
        font-size: 0.8125rem;
        font-weight: 500;
        color: #94a3b8;
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
        font-size: 0.9rem;
        color: #cbd5e1;
        border-bottom: 1px solid rgba(255, 255, 255, 0.03);
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
        background: rgba(255, 255, 255, 0.03);
        color: white;
        text-decoration: none;
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 12px;
        font-weight: 600;
        font-size: 0.9375rem;
        transition: all 0.3s ease;
        margin-top: auto;
    }

    .card-button:hover {
        background: var(--card-color);
        border-color: var(--card-color);
        color: #000;
    }

    /* Confluence Section */
    .confluence-section {
        padding: 8rem 2rem;
        background: #050812;
        border-top: 1px solid rgba(255,255,255,0.05);
        border-bottom: 1px solid rgba(255,255,255,0.05);
    }

    .confluence-grid {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 3rem;
        margin: 5rem 0;
        flex-wrap: wrap;
    }

    .confluence-step {
        background: linear-gradient(145deg, rgba(255,255,255,0.03), rgba(255,255,255,0.01));
        border: 1px solid rgba(255,255,255,0.05);
        padding: 2.5rem 2rem;
        border-radius: 20px;
        flex: 1;
        min-width: 250px;
        max-width: 350px;
        text-align: center;
        position: relative;
        backdrop-filter: blur(10px);
    }

    .step-number {
        background: #2e8eff;
        width: 48px;
        height: 48px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: 800;
        font-size: 1.25rem;
        margin: -3.5rem auto 1.5rem;
        border: 4px solid #050812;
        color: white;
    }

    .confluence-cta {
        text-align: center;
        margin-top: 4rem;
    }

    .text-link {
        color: #34d399;
        font-weight: 600;
        text-decoration: none;
        font-size: 1.125rem;
        border-bottom: 1px solid transparent;
        transition: border-color 0.2s;
    }
    .text-link:hover { border-bottom-color: #34d399; }

    /* FAQ Section */
    .faq-section {
        padding: 8rem 2rem;
        max-width: 900px;
        margin: 0 auto;
    }

    .faq-list {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .faq-item {
        background: rgba(255,255,255,0.02);
        border-radius: 16px;
        overflow: hidden;
        border: 1px solid rgba(255,255,255,0.05);
    }

    .faq-item.open {
        background: rgba(255,255,255,0.04);
        border-color: rgba(46,142,255,0.3);
        box-shadow: 0 0 30px rgba(0,0,0,0.2);
    }

    .faq-question {
        width: 100%;
        text-align: left;
        padding: 1.5rem 2rem;
        background: none;
        border: none;
        color: white;
        font-weight: 600;
        font-size: 1.125rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        cursor: pointer;
    }

    .faq-answer {
        max-height: 0;
        overflow: hidden;
        transition: max-height 0.4s cubic-bezier(0, 1, 0, 1);
        padding: 0 2rem;
        color: #94a3b8;
        line-height: 1.7;
    }

    .faq-item.open .faq-answer {
        max-height: 300px;
        padding-bottom: 2rem;
    }

    /* Final CTA */
    .final-cta {
        padding: 10rem 2rem;
        text-align: center;
        background: radial-gradient(circle at center, rgba(10, 15, 30, 0) 0%, #050812 100%);
    }

    .cta-content {
        max-width: 700px;
        margin: 0 auto;
    }

    .final-cta h2 {
        font-size: clamp(2.5rem, 5vw, 3.5rem);
        font-weight: 800;
        margin-bottom: 1.5rem;
        line-height: 1.1;
    }

    .final-cta p {
        color: #cbd5e1;
        font-size: 1.25rem;
        margin-bottom: 2.5rem;
    }

    .cta-button.large {
        padding: 1.25rem 3.5rem;
        font-size: 1.125rem;
    }

    /* Responsive */
    @media (max-width: 1200px) {
        .glow-orb-1 { width: 500px; height: 500px; }
        .glow-orb-2 { width: 450px; height: 450px; }
    }

    @media (max-width: 1024px) {
        .truth-grid { grid-template-columns: 1fr; gap: 4rem; }
        .confluence-grid { flex-direction: column; gap: 2rem; }
    }

    @media (max-width: 768px) {
        .hero-section { padding: 6rem 1.5rem 4rem; min-height: auto; }
        .indicators-grid { grid-template-columns: 1fr; }
        .hero-stats { flex-direction: column; gap: 1.5rem; }
        .stat-item { justify-content: flex-start; }
        .filter-section { padding: 4rem 1rem 1rem; }
        .setup-grid { grid-template-columns: 1fr; }
    }
</style>