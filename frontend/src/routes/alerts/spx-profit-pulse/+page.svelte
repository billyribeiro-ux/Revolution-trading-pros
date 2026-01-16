<script lang="ts">
    import { onMount } from 'svelte';
    import { fade, slide, fly } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';
    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- Pricing State ---
    let selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');

    // --- FAQ Logic ---
    let openFaq: number | null = $state(null);
    const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

    // --- Apple ICT9+ Scroll Animations ---
    // Smooth, performant reveal animations using IntersectionObserver
    let mounted = $state(false);
    
    function reveal(node: HTMLElement, params: { delay?: number; y?: number } = {}) {
        const delay = params.delay ?? 0;
        const translateY = params.y ?? 30;
        
        // Check for reduced motion preference
        if (typeof window !== 'undefined' && window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
            return;
        }
        
        // Set initial hidden state with transition already applied
        node.style.opacity = '0';
        node.style.transform = `translateY(${translateY}px)`;
        node.style.transition = `opacity 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94), transform 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94)`;
        node.style.transitionDelay = `${delay}ms`;
        node.style.willChange = 'opacity, transform';
        
        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        // Trigger animation
                        node.style.opacity = '1';
                        node.style.transform = 'translateY(0)';
                        
                        // Cleanup will-change after animation
                        setTimeout(() => {
                            node.style.willChange = 'auto';
                        }, 800 + delay);
                        
                        observer.unobserve(node);
                    }
                });
            },
            { threshold: 0.1, rootMargin: '0px 0px -60px 0px' }
        );
        
        observer.observe(node);
        
        return {
            destroy() {
                observer.disconnect();
            }
        };
    }
    
    onMount(() => {
        mounted = true;
    });

    // --- EXPANDED FAQ DATA ---
    const faqList = [
        {
            q: 'What is SPX 0DTE and why do you trade it?',
            a: "SPX 0DTE refers to 'Zero Days to Expiration' options on the S&P 500 index. These contracts expire at 4:00 PM EST on the same day they are traded. We trade them because they offer the fastest potential returns (Gamma risk) and zero overnight risk. You are 100% in cash every single night."
        },
        {
            q: 'How fast are the alerts? Is latency an issue?',
            a: 'Speed is everything in 0DTE. Our alerts are sent instantly via direct SMS text message and Discord webhooks. The average latency is under 5 seconds. We also provide "Warning" alerts (e.g., "Watching 4550 Calls") so you can load the contract before we trigger the official entry.'
        },
        {
            q: 'What is the "SPX Tax Advantage" (Section 1256)?',
            a: 'This is a huge benefit over trading SPY or individual stocks. SPX options fall under Section 1256 of the tax code, meaning gains are taxed at a blended rate: 60% Long Term Capital Gains (lower rate) and 40% Short Term, regardless of how long you hold the trade. Consult your CPA, but for most traders, this results in significant tax savings.'
        },
        {
            q: 'What account size do I need?',
            a: 'SPX options are large contracts (10x the size of SPY). Premiums typically range from $2.00 to $10.00 ($200-$1,000 per contract). We recommend a starting account of at least $2,000 to manage risk properly and allow for position sizing (scaling in/out).'
        },
        {
            q: 'Does this trigger the PDT (Pattern Day Trading) rule?',
            a: 'Yes, if you have a margin account under $25,000. However, many of our members use "Cash Accounts." In a Cash Account, options settle overnight (T+1), meaning you can trade your entire account balance every day without PDT restrictions. SPX is perfect for Cash Accounts due to this T+1 settlement.'
        },
        {
            q: 'Is there "Assignment Risk" at expiration?',
            a: 'No. This is another major advantage of SPX over SPY. SPX is "Cash Settled." This means you can never be forced to buy or sell the underlying shares. If you hold through expiration (which we rarely advise), you simply receive or pay the cash difference. No surprise margin calls for $500,000 worth of stock.'
        },
        {
            q: 'What brokerage do you recommend?',
            a: 'We recommend ThinkOrSwim (Schwab), Interactive Brokers, or TastyTrade for their fast execution and reliable data. Mobile-first brokers like Robinhood and Webull also support SPX, but execution speeds may vary during high volatility.'
        },
        {
            q: 'Do you trade every single day?',
            a: 'The market is open every day, but we only trade when our edge is present. We typically take 1-3 trades per day. If the market is "choppy" or low volume, we sit on our hands. Preservation of capital is our #1 priority.'
        },
        {
            q: 'What is your stop loss strategy?',
            a: 'We use "Hard Stops" based on the premium price (e.g., Stop at 30% loss) or technical invalidate levels on the chart. We do not "hope" trades come back. If a trade hits our stop, we exit immediately. We usually cut losers fast and let winners run.'
        },
        {
            q: 'Can I do this while working a full-time job?',
            a: 'Yes, but you need access to your phone. Our trades are fast—often lasting 15 to 45 minutes. We recommend setting push notifications for our alerts so you can step away for a moment to execute the trade on your mobile app.'
        },
        {
            q: 'What is the win rate?',
            a: 'Our historical win rate is approximately 78%. However, win rate is less important than Risk:Reward. We aim for winners that are 2x or 3x the size of our losers. This mathematical edge ensures profitability even if the win rate dips.'
        },
        {
            q: 'Do you offer a trial?',
            a: 'We offer a highly discounted first month via our Monthly Plan so you can test the service with minimal commitment. We also offer a 30-day money-back guarantee on our Annual plans.'
        },
        {
            q: 'Do you trade Iron Condors or Spreads?',
            a: 'Our primary strategy is "Long Calls" and "Long Puts" (Directional buying). We occasionally signal vertical spreads to cap risk on volatile days, but 90% of our signals are simple directional buys aimed at capturing momentum.'
        },
        {
            q: 'What happens if I miss an entry?',
            a: 'We provide an "Entry Zone" (e.g., Buy between $3.40 and $3.60). If the price has moved significantly past this zone, we advise **not chasing**. There will always be another trade. Chasing entries ruins the risk/reward ratio.'
        }
    ];

    // --- SEO SCHEMA (JSON-LD) ---
    const productSchema = {
        '@context': 'https://schema.org',
        '@type': 'Product',
        name: 'SPX Profit Pulse - 0DTE Options Alerts',
        description:
            'Professional SPX 0DTE options alerts delivered via SMS and Discord. Real-time entries, exits, and risk management. Section 1256 Tax Advantage.',
        brand: {
            '@type': 'Organization',
            name: 'Revolution Trading Pros'
        },
        aggregateRating: {
            '@type': 'AggregateRating',
            ratingValue: '4.8',
            reviewCount: '1042'
        },
        offers: {
            '@type': 'AggregateOffer',
            priceCurrency: 'USD',
            lowPrice: '65',
            highPrice: '97',
            offerCount: '3',
            offers: [
                {
                    '@type': 'Offer',
                    name: 'Monthly Plan',
                    price: '97',
                    priceCurrency: 'USD',
                    priceSpecification: {
                        '@type': 'UnitPriceSpecification',
                        price: '97',
                        priceCurrency: 'USD',
                        referenceQuantity: { '@type': 'QuantitativeValue', value: '1', unitCode: 'MON' }
                    }
                },
                {
                    '@type': 'Offer',
                    name: 'Annual Plan',
                    price: '777',
                    priceCurrency: 'USD'
                }
            ]
        }
    };

    const faqSchema = {
        '@context': 'https://schema.org',
        '@type': 'FAQPage',
        mainEntity: faqList.map(item => ({
            '@type': 'Question',
            name: item.q,
            acceptedAnswer: {
                '@type': 'Answer',
                text: item.a
            }
        }))
    };

    const combinedSchema = [productSchema, faqSchema];
</script>

<SEOHead
    title="SPX 0DTE Options Alerts | Daily Income Signals & Gamma Scalping"
    description="Trade SPX 0DTE options with confidence. Real-time SMS alerts, Section 1256 tax benefits, and precise gamma scalping strategies. 78% win rate."
    canonical="/alerts/spx-profit-pulse"
    ogType="product"
    ogImage="/images/og-spx-pulse.jpg"
    ogImageAlt="SPX Profit Pulse - Real-Time 0DTE Options Alerts"
    keywords={[
        'SPX 0DTE alerts',
        'options trading signals',
        'SPX signals',
        '0DTE trading strategy',
        'same day expiration options',
        'SPX options alerts',
        'Section 1256 contracts',
        'options trading service',
        'gamma scalping'
    ]}
    schema={combinedSchema}
    schemaType="Product"
    productPrice={97}
    productCurrency="USD"
    productAvailability="in stock"
/>

<main
    class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text font-sans selection:bg-rtp-primary selection:text-white"
>
    <section class="relative min-h-[90vh] flex items-center overflow-hidden py-24 lg:py-0">
        <div class="absolute inset-0 bg-rtp-bg z-0">
            <div
                class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:64px_64px] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"
            ></div>
            <div
                class="absolute top-[-20%] right-[-10%] w-[800px] h-[800px] bg-rtp-primary/10 rounded-full blur-[120px] animate-pulse"
            ></div>
            <div
                class="absolute bottom-[-10%] left-[-10%] w-[600px] h-[600px] bg-rtp-indigo/10 rounded-full blur-[100px]"
            ></div>
        </div>

        <div
            class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-12 items-center"
        >
            <div class="text-center lg:text-left space-y-8">
                <div
                    use:reveal={{ delay: 0 }}
                    class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-rtp-surface border border-rtp-border/50 shadow-sm backdrop-blur-sm"
                >
                    <span class="relative flex h-2.5 w-2.5">
                        <span
                            class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
                        ></span>
                        <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
                    </span>
                    <span class="text-xs font-bold uppercase tracking-widest text-rtp-muted"
                        >Market Active Now</span
                    >
                </div>

                <h1
                    use:reveal={{ delay: 100 }}
                    class="text-5xl md:text-7xl font-heading font-extrabold tracking-tight leading-[1.1]"
                >
                    Conquer Volatility with
                    <span
                        class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-primary via-rtp-blue to-rtp-emerald"
                        >SPX 0DTE</span
                    >
                </h1>

                <p
                    use:reveal={{ delay: 200 }}
                    class="text-xl text-rtp-muted max-w-2xl mx-auto lg:mx-0 leading-relaxed"
                >
                    Institutional-grade S&P 500 options alerts delivered instantly via SMS & Discord. Capture
                    rapid moves, enjoy Section 1256 tax benefits, and sleep well with zero overnight risk.
                </p>

                <div
                    use:reveal={{ delay: 300 }}
                    class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start pt-4"
                >
                    <a
                        href="#pricing"
                        class="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-rtp-primary rounded-xl hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-rtp-primary offset-rtp-bg shadow-lg hover:shadow-rtp-primary/25 hover:-translate-y-1"
                    >
                        Start Your Trial
                        <svg
                            class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 7l5 5m0 0l-5 5m5-5H6"
                            /></svg
                        >
                    </a>
                    <a
                        href="#performance"
                        class="inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-rtp-text transition-all duration-200 bg-rtp-surface border border-rtp-border rounded-xl hover:bg-rtp-surface/80 hover:border-rtp-primary/30 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-rtp-border offset-rtp-bg"
                    >
                        View Results
                    </a>
                </div>

                <div
                    use:reveal={{ delay: 400 }}
                    class="flex flex-wrap items-center justify-center lg:justify-start gap-6 pt-4 text-sm text-rtp-muted/60 font-medium"
                >
                    <div class="flex items-center gap-2">
                        <svg
                            class="w-5 h-5 text-emerald-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            /></svg
                        >
                        <span>Verified 78% Win Rate</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <svg
                            class="w-5 h-5 text-emerald-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                            /></svg
                        >
                        <span>&lt; 5s Alert Latency</span>
                    </div>
                     <div class="flex items-center gap-2">
                        <svg
                            class="w-5 h-5 text-emerald-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            /></svg
                        >
                        <span>Tax Efficient (1256)</span>
                    </div>
                </div>
            </div>

            <div class="relative hidden lg:block perspective-1000">
                <div
                    class="absolute inset-0 bg-gradient-to-tr from-rtp-primary/20 to-transparent rounded-full blur-3xl transform translate-x-10 translate-y-10"
                ></div>

                <div
                    class="relative bg-rtp-surface/80 backdrop-blur-xl border border-rtp-border/50 rounded-3xl p-6 shadow-2xl transform rotate-y-[-10deg] rotate-x-[5deg] hover:rotate-0 transition-transform duration-700 ease-out"
                >
                    <div class="flex items-center justify-between mb-6 border-b border-rtp-border/30 pb-4">
                        <div class="flex items-center gap-3">
                            <div
                                class="w-10 h-10 rounded-full bg-gradient-to-br from-rtp-primary to-rtp-blue flex items-center justify-center text-white font-bold shadow-inner"
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                                    ><path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M13 10V3L4 14h7v7l9-11h-7z"
                                    /></svg
                                >
                            </div>
                            <div>
                                <div class="font-bold text-rtp-text">SPX Profit Pulse</div>
                                <div class="text-xs text-rtp-emerald">● Live Trading Room</div>
                            </div>
                        </div>
                        <div class="text-xs font-mono text-rtp-muted bg-rtp-bg px-2 py-1 rounded">
                            10:32:45 EST
                        </div>
                    </div>

                    <div class="space-y-4">
                        <div class="bg-rtp-bg/50 p-4 rounded-xl border-l-4 border-emerald-500">
                            <div class="flex justify-between text-xs mb-2">
                                <span class="text-emerald-500 font-bold uppercase">New Signal</span>
                                <span class="text-rtp-muted">Just now</span>
                            </div>
                            <div class="text-sm font-mono text-rtp-text mb-1">
                                BTO <span class="font-bold text-white">SPX 4580 CALL</span> @ $3.50
                            </div>
                            <div class="flex gap-4 text-xs text-rtp-muted">
                                <span>🛑 Stop: $2.10</span>
                                <span>🎯 Target: $5.00+</span>
                            </div>
                        </div>
                        <div class="bg-rtp-bg/50 p-4 rounded-xl border-l-4 border-rtp-blue opacity-60">
                            <div class="flex justify-between text-xs mb-2">
                                <span class="text-rtp-blue font-bold uppercase">Update</span>
                                <span class="text-rtp-muted">15m ago</span>
                            </div>
                            <div class="text-sm text-rtp-text">
                                Approaching VWAP support. Watching for bounce to add to runners.
                            </div>
                        </div>
                    </div>

                    <div
                        class="absolute -bottom-6 -right-6 bg-white text-rtp-bg px-6 py-3 rounded-xl shadow-xl font-bold border-2 border-rtp-bg flex items-center gap-2 animate-bounce"
                    >
                        <span class="text-2xl">🚀</span>
                        <div>
                            <div class="text-xs uppercase tracking-wide opacity-70">Last Trade</div>
                            <div class="text-emerald-600">+85% Profit</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="bg-rtp-surface border-y border-rtp-border relative z-20">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                <div class="text-center group">
                    <div
                        class="text-3xl md:text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-br from-emerald-400 to-emerald-600 mb-2 group-hover:scale-110 transition-transform"
                    >
                        78%
                    </div>
                    <div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
                        Historical Win Rate
                    </div>
                </div>
                <div class="text-center group">
                    <div
                        class="text-3xl md:text-5xl font-extrabold text-rtp-text mb-2 group-hover:scale-110 transition-transform"
                    >
                        &lt;5s
                    </div>
                    <div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
                        Alert Latency
                    </div>
                </div>
                <div class="text-center group">
                    <div
                        class="text-3xl md:text-5xl font-extrabold text-rtp-text mb-2 group-hover:scale-110 transition-transform"
                    >
                        1k+
                    </div>
                    <div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
                        Active Traders
                    </div>
                </div>
                <div class="text-center group">
                    <div
                        class="text-3xl md:text-5xl font-extrabold text-rtp-text mb-2 group-hover:scale-110 transition-transform"
                    >
                        $35M+
                    </div>
                    <div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
                        Volume Traded
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg relative overflow-hidden">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center max-w-3xl mx-auto mb-16">
                <span class="text-rtp-primary font-bold uppercase tracking-wider text-sm">Why SPX?</span>
                <h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mt-2 mb-6">
                    The Unfair Advantage
                </h2>
                <p use:reveal={{ delay: 100 }} class="text-xl text-rtp-muted">
                    Trading SPX isn't just about volatility; it's about structural advantages that put money back in your pocket.
                </p>
                
            </div>

            <div class="grid md:grid-cols-3 gap-8">
                <div use:reveal={{ delay: 0 }} class="bg-rtp-surface p-8 rounded-2xl border border-rtp-border">
                    <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
                         <span class="text-emerald-500">💰</span> Tax Efficient (60/40)
                    </h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Unlike AAPL or TSLA options, SPX options fall under <strong>Section 1256</strong> of the IRS code. This means 60% of your gains are taxed at the lower Long Term Capital Gains rate, even if you day trade them.
                    </p>
                </div>
                <div use:reveal={{ delay: 150 }} class="bg-rtp-surface p-8 rounded-2xl border border-rtp-border">
                    <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
                         <span class="text-blue-500">🛡️</span> No Assignment Risk
                    </h3>
                    <p class="text-rtp-muted leading-relaxed">
                        SPX is <strong>Cash Settled</strong>. You never have to worry about buying 100 shares of a $4,500 index. At expiration, the difference is simply paid in cash. No margin calls for shares you can't afford.
                    </p>
                </div>
                <div use:reveal={{ delay: 300 }} class="bg-rtp-surface p-8 rounded-2xl border border-rtp-border">
                    <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
                         <span class="text-indigo-500">💧</span> Massive Liquidity
                    </h3>
                    <p class="text-rtp-muted leading-relaxed">
                        SPX is one of the most liquid markets in the world. This means tight spreads (difference between bid/ask), allowing us to enter and exit large positions instantly without slippage.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface border-y border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
             <div class="grid lg:grid-cols-2 gap-16 items-center">
                <div>
                    <span class="text-emerald-500 font-bold uppercase tracking-wider text-sm">The Strategy</span>
                    <h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mt-2 mb-6">
                        Gamma Scalping Explained
                    </h2>
                    <p class="text-rtp-muted mb-6 text-lg">
                        0DTE options have the highest "Gamma" in the market. This means the option price accelerates rapidly as the stock moves in your favor.
                    </p>
                    <p class="text-rtp-muted mb-8 leading-relaxed">
                        We identify "Gamma Levels" where Market Makers are forced to hedge. When price hits these triggers, it causes a chain reaction of buying or selling. We ride that wave.
                    </p>
                    <ul class="space-y-4">
                        <li class="flex items-center gap-3 text-white">
                            <span class="w-6 h-6 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-500 font-bold text-xs">✓</span>
                            Entry: Identify key VWAP and Gamma levels.
                        </li>
                        <li class="flex items-center gap-3 text-white">
                            <span class="w-6 h-6 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-500 font-bold text-xs">✓</span>
                            Execution: Sniper entries via Limit Orders.
                        </li>
                        <li class="flex items-center gap-3 text-white">
                            <span class="w-6 h-6 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-500 font-bold text-xs">✓</span>
                            Exit: Scale out into strength to lock in profits.
                        </li>
                    </ul>
                </div>
                <div class="relative bg-rtp-bg p-8 rounded-3xl border border-rtp-border shadow-2xl">
                    
                    <div class="text-center mt-4 text-xs text-rtp-muted">
                        We buy when momentum overcomes Theta decay.
                    </div>
                </div>
             </div>
        </div>
    </section>

    <section class="py-32 bg-rtp-bg relative overflow-hidden">
        <div class="absolute inset-0 opacity-[0.02] bg-[url('/grid-pattern.svg')]"></div>

        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
            <div class="text-center max-w-3xl mx-auto mb-20">
                <h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
                    Institutional Edge, Retail Accessible.
                </h2>
                <p use:reveal={{ delay: 100 }} class="text-xl text-rtp-muted">
                    Most retail traders gamble. We operate like a fund. Data-driven entries, strict sizing,
                    and emotionless execution.
                </p>
            </div>

            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                <div
                    use:reveal={{ delay: 0 }}
                    class="group bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-primary/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-rtp-primary/10"
                >
                    <div
                        class="w-14 h-14 rounded-xl bg-rtp-primary/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
                    >
                        <svg
                            class="w-7 h-7 text-rtp-primary"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Instant SMS & Push</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        Don't miss a move because you stepped away. Alerts hit your phone via SMS and App
                        notification instantly.
                    </p>
                </div>

                <div
                    use:reveal={{ delay: 100 }}
                    class="group bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-indigo/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-rtp-indigo/10"
                >
                    <div
                        class="w-14 h-14 rounded-xl bg-rtp-indigo/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
                    >
                        <svg
                            class="w-7 h-7 text-rtp-indigo"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Detailed Strategy Logic</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        We don't just say "Buy". We tell you *why*. Flow, technicals, and gamma levels explained
                        in every alert.
                    </p>
                </div>

                <div
                    use:reveal={{ delay: 200 }}
                    class="group bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-emerald/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-rtp-emerald/10"
                >
                    <div
                        class="w-14 h-14 rounded-xl bg-rtp-emerald/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
                    >
                        <svg
                            class="w-7 h-7 text-rtp-emerald"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Exact Entry & Exits</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        No guessing games. You get the specific strike, expiration, and limit price. "Buy SPX
                        4600 Call @ $4.20".
                    </p>
                </div>

                <div
                    use:reveal={{ delay: 0 }}
                    class="group bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-blue/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-rtp-blue/10"
                >
                    <div
                        class="w-14 h-14 rounded-xl bg-rtp-blue/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
                    >
                        <svg class="w-7 h-7 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Runner Management</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        We scale out to lock in profits and leave "runners" for the big moves. Maximize upside,
                        minimize stress.
                    </p>
                </div>

                <div
                    use:reveal={{ delay: 100 }}
                    class="group bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-red-400/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-red-400/10"
                >
                    <div
                        class="w-14 h-14 rounded-xl bg-red-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
                    >
                        <svg class="w-7 h-7 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Hard Stops (No Bagley)</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        We never hope. Every trade has a predefined invalidation level. We cut losers fast to
                        protect your capital.
                    </p>
                </div>

                <div
                    use:reveal={{ delay: 200 }}
                    class="group bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-purple-400/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-purple-400/10"
                >
                    <div
                        class="w-14 h-14 rounded-xl bg-purple-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
                    >
                        <svg
                            class="w-7 h-7 text-purple-400"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            ><path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Market Context</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        Receive pre-market plans and mid-day updates. Know when to be aggressive and when to sit
                        on your hands.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface border-y border-rtp-border">
        <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
                    Crystal Clear Execution
                </h2>
                <p use:reveal class="text-xl text-rtp-muted">Follow the lifecycle of a typical trade.</p>
            </div>

            <div class="relative">
                <div
                    class="absolute left-8 md:left-1/2 top-0 bottom-0 w-0.5 bg-gradient-to-b from-emerald-500 via-blue-500 to-rtp-border"
                ></div>

                <div
                    use:reveal
                    class="relative flex flex-col md:flex-row items-center md:justify-between mb-16 group"
                >
                    <div class="md:w-[45%] mb-4 md:mb-0 md:text-right pr-8 order-2 md:order-1">
                        <h3 class="text-2xl font-bold text-white mb-2">1. The Setup & Entry</h3>
                        <p class="text-rtp-muted">
                            We identify a key gamma level holding. You get the alert instantly with strike, price,
                            and risk parameters.
                        </p>
                    </div>
                    <div
                        class="absolute left-8 md:left-1/2 -translate-x-1/2 w-10 h-10 rounded-full bg-rtp-bg border-4 border-emerald-500 z-10 flex items-center justify-center shadow-[0_0_20px_rgba(16,185,129,0.5)]"
                    >
                        <span class="text-emerald-500 font-bold">1</span>
                    </div>
                    <div class="md:w-[45%] pl-16 md:pl-8 order-1 md:order-2 w-full">
                        <div class="bg-rtp-bg p-6 rounded-xl border-l-4 border-emerald-500 shadow-lg">
                            <div class="flex items-center justify-between mb-3">
                                <span
                                    class="bg-emerald-500/20 text-emerald-400 text-[10px] font-bold px-2 py-1 rounded uppercase"
                                    >Signal</span
                                >
                                <span class="font-mono text-xs text-rtp-muted">09:42 AM</span>
                            </div>
                            <div class="font-mono text-sm">
                                <div class="font-bold text-lg text-white">BTO SPX 4580 CALL</div>
                                <div class="grid grid-cols-2 gap-y-2 mt-2 text-xs">
                                    <div class="text-rtp-muted">
                                        Entry: <span class="text-white font-bold">$3.50</span>
                                    </div>
                                    <div class="text-rtp-muted">
                                        Stop: <span class="text-red-400 font-bold">$2.10</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div
                    use:reveal={{ delay: 150 }}
                    class="relative flex flex-col md:flex-row items-center md:justify-between mb-16 group"
                >
                    <div class="md:w-[45%] pl-16 md:pl-0 md:pr-8 order-1 w-full">
                        <div class="bg-rtp-bg p-6 rounded-xl border-l-4 border-blue-500 shadow-lg">
                            <div class="flex items-center justify-between mb-3">
                                <span
                                    class="bg-blue-500/20 text-blue-400 text-[10px] font-bold px-2 py-1 rounded uppercase"
                                    >Update</span
                                >
                                <span class="font-mono text-xs text-rtp-muted">10:05 AM</span>
                            </div>
                            <div class="font-mono text-sm">
                                <div class="font-bold text-lg text-white">TARGET 1 HIT 🎯</div>
                                <p class="text-xs text-rtp-muted mt-1">
                                    Price at $4.50 (+28%). Trim half size. Move stop on runners to Breakeven.
                                </p>
                            </div>
                        </div>
                    </div>
                    <div
                        class="absolute left-8 md:left-1/2 -translate-x-1/2 w-10 h-10 rounded-full bg-rtp-bg border-4 border-blue-500 z-10 flex items-center justify-center shadow-[0_0_20px_rgba(59,130,246,0.5)]"
                    >
                        <span class="text-blue-500 font-bold">2</span>
                    </div>
                    <div class="md:w-[45%] mb-4 md:mb-0 pl-16 md:pl-8 order-2">
                        <h3 class="text-2xl font-bold text-white mb-2">2. Trade Management</h3>
                        <p class="text-rtp-muted">
                            We don't leave you hanging. We send real-time updates to trim profits and protect your
                            downside as the trade moves.
                        </p>
                    </div>
                </div>

                <div
                    use:reveal={{ delay: 300 }}
                    class="relative flex flex-col md:flex-row items-center md:justify-between group"
                >
                    <div class="md:w-[45%] mb-4 md:mb-0 md:text-right pr-8 order-2 md:order-1">
                        <h3 class="text-2xl font-bold text-white mb-2">3. Final Exit</h3>
                        <p class="text-rtp-muted">
                            We squeeze the move for maximum gain, exiting runners into strength before reversal.
                        </p>
                    </div>
                    <div
                        class="absolute left-8 md:left-1/2 -translate-x-1/2 w-10 h-10 rounded-full bg-rtp-bg border-4 border-indigo-500 z-10 flex items-center justify-center shadow-[0_0_20px_rgba(99,102,241,0.5)]"
                    >
                        <span class="text-indigo-500 font-bold">3</span>
                    </div>
                    <div class="md:w-[45%] pl-16 md:pl-8 order-1 md:order-2 w-full">
                        <div class="bg-rtp-bg p-6 rounded-xl border-l-4 border-indigo-500 shadow-lg">
                            <div class="flex items-center justify-between mb-3">
                                <span
                                    class="bg-indigo-500/20 text-indigo-400 text-[10px] font-bold px-2 py-1 rounded uppercase"
                                    >Exit</span
                                >
                                <span class="font-mono text-xs text-rtp-muted">10:45 AM</span>
                            </div>
                            <div class="font-mono text-sm">
                                <div class="font-bold text-lg text-white">ALL OUT</div>
                                <p class="text-xs text-rtp-muted mt-1">Sold runners at $7.00.</p>
                                <p class="text-emerald-400 font-bold mt-2">Total Profit: +100% ✅</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="performance" class="py-24 bg-rtp-bg relative">
        <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex flex-col md:flex-row justify-between items-end mb-12 gap-6">
                <div>
                    <h2 class="text-3xl md:text-4xl font-heading font-bold text-rtp-text mb-2">
                        Recent Performance
                    </h2>
                    <p class="text-rtp-muted">Transparency is our currency. Live log of recent calls.</p>
                </div>
                <a
                    href="/performance"
                    class="text-rtp-primary font-bold hover:text-white transition-colors flex items-center gap-2"
                >
                    View Full Ledger <svg
                        class="w-4 h-4"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        ><path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M17 8l4 4m0 0l-4 4m4-4H3"
                        /></svg
                    >
                </a>
            </div>

            <div class="bg-rtp-surface rounded-2xl border border-rtp-border overflow-hidden shadow-xl">
                <div
                    class="grid grid-cols-12 bg-rtp-bg/50 border-b border-rtp-border p-4 text-xs font-bold uppercase text-rtp-muted tracking-wider"
                >
                    <div class="col-span-3 md:col-span-2">Date</div>
                    <div class="col-span-5 md:col-span-4">Ticker / Strike</div>
                    <div class="col-span-4 md:col-span-2 text-right">Result</div>
                    <div class="hidden md:block md:col-span-4 text-right">Notes</div>
                </div>
                <div class="divide-y divide-rtp-border/50 font-mono text-sm">
                    <div class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors">
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">Nov 15</div>
                        <div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4560 CALL</div>
                        <div class="col-span-4 md:col-span-2 text-right text-emerald-400 font-bold">+50%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Held VWAP perfectly.
                        </div>
                    </div>
                    <div class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors">
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">Nov 14</div>
                        <div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4575 PUT</div>
                        <div class="col-span-4 md:col-span-2 text-right text-emerald-400 font-bold">+50%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Clean breakdown of 4580.
                        </div>
                    </div>
                    <div
                        class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors bg-red-500/5"
                    >
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">Nov 13</div>
                        <div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4590 CALL</div>
                        <div class="col-span-4 md:col-span-2 text-right text-red-400 font-bold">-31%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Stopped out. Choppy open.
                        </div>
                    </div>
                    <div class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors">
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">Nov 12</div>
                        <div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4555 PUT</div>
                        <div class="col-span-4 md:col-span-2 text-right text-emerald-400 font-bold">+60%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Trend day runner.
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-rtp-surface border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
                    Simple, Flat Pricing
                </h2>
                <p class="text-xl text-rtp-muted">Pay for the alerts with one good trade.</p>
            </div>

            <div class="flex justify-center mb-16">
                <div class="bg-rtp-bg p-1.5 rounded-xl border border-rtp-border inline-flex relative">
                    <button
                        onclick={() => (selectedPlan = 'monthly')}
                        class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
                        'monthly'
                            ? 'text-white'
                            : 'text-rtp-muted hover:text-white'}">Monthly</button
                    >
                    <button
                        onclick={() => (selectedPlan = 'quarterly')}
                        class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
                        'quarterly'
                            ? 'text-white'
                            : 'text-rtp-muted hover:text-white'}">Quarterly</button
                    >
                    <button
                        onclick={() => (selectedPlan = 'annual')}
                        class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
                        'annual'
                            ? 'text-white'
                            : 'text-rtp-muted hover:text-white'}">Annual</button
                    >

                    <div
                        class="absolute top-1.5 bottom-1.5 bg-rtp-primary rounded-lg shadow-md transition-all duration-300 ease-out"
                        style="left: {selectedPlan === 'monthly'
                            ? '0.375rem'
                            : selectedPlan === 'quarterly'
                                ? 'calc(33.33% + 0.2rem)'
                                : 'calc(66.66% + 0.1rem)'}; width: calc(33.33% - 0.4rem);"
                    ></div>
                </div>
            </div>

            <div class="grid lg:grid-cols-3 gap-8 max-w-6xl mx-auto items-center">
                <div
                    class="order-2 lg:order-1 bg-rtp-bg p-8 rounded-2xl border transition-all {selectedPlan === 'monthly' ? 'border-rtp-primary opacity-100 scale-105' : 'border-rtp-border opacity-70 hover:opacity-90'}"
                >
                    <h3 class="text-xl font-bold text-rtp-text mb-2">Monthly</h3>
                    <div class="flex items-baseline gap-1 mb-6">
                        <span class="text-4xl font-bold text-white">$97</span>
                        <span class="text-rtp-muted">/mo</span>
                    </div>
                    <p class="text-sm text-rtp-muted mb-8 h-10">Perfect for testing the waters.</p>
                    <a
                        href="/checkout/monthly"
                        class="block w-full py-3 px-4 bg-rtp-surface border border-rtp-border text-rtp-text font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
                    >
                        Select Monthly
                    </a>
                    <div class="mt-8 space-y-4 text-sm text-rtp-muted">
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             Daily Live Alerts
                        </div>
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             Discord Access
                        </div>
                    </div>
                </div>

                <div
                    class="order-1 lg:order-2 bg-rtp-bg p-10 rounded-3xl border-2 shadow-2xl relative transform z-10 transition-all {selectedPlan === 'quarterly' ? 'border-rtp-primary shadow-rtp-primary/20 lg:scale-110 opacity-100' : 'border-rtp-border shadow-rtp-border/10 lg:scale-100 opacity-70 hover:opacity-90'}"
                >
                    <div
                        class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-primary text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide"
                    >
                        Most Popular
                    </div>
                    <h3 class="text-2xl font-bold text-white mb-2">Quarterly</h3>
                    <div class="flex items-baseline gap-1 mb-1">
                        <span class="text-5xl font-extrabold text-white">$247</span>
                        <span class="text-rtp-muted">/qtr</span>
                    </div>
                    <p class="text-emerald-400 text-sm font-bold mb-8">Save $45 vs Monthly</p>

                    <a
                        href="/checkout/quarterly"
                        class="block w-full py-4 px-6 bg-rtp-primary text-white font-bold rounded-xl text-center hover:bg-blue-600 transition-colors shadow-lg mb-8"
                    >
                        Start Quarterly Plan
                    </a>

                    <div class="space-y-4 text-sm text-white/90">
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             <span class="font-bold">Priority Support</span>
                        </div>
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             Instant SMS Alerts
                        </div>
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             Discord Community
                        </div>
                    </div>
                </div>

                <div
                    class="order-3 bg-rtp-bg p-8 rounded-2xl border transition-all {selectedPlan === 'annual' ? 'border-emerald-500 opacity-100 scale-105' : 'border-rtp-border opacity-70 hover:opacity-90'}"
                >
                    <h3 class="text-xl font-bold text-rtp-text mb-2">Annual</h3>
                    <div class="flex items-baseline gap-1 mb-1">
                        <span class="text-4xl font-bold text-white">$777</span>
                        <span class="text-rtp-muted">/yr</span>
                    </div>
                    <p class="text-emerald-500 text-sm font-bold mb-8 h-10">Best Value (Save 33%)</p>

                    <a
                        href="/checkout/annual"
                        class="block w-full py-3 px-4 bg-rtp-surface border border-rtp-emerald text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-white transition-colors"
                    >
                        Select Annual
                    </a>
                    <div class="mt-8 space-y-4 text-sm text-rtp-muted">
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             All Quarterly Features
                        </div>
                        <div class="flex gap-3">
                            <svg class="w-5 h-5 text-emerald-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                             2 Months Free
                        </div>
                    </div>
                </div>
            </div>

            <div class="mt-12 text-center">
                <div class="inline-flex items-center gap-2 text-sm text-rtp-muted">
                    <svg class="w-4 h-4 text-rtp-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                        ><path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                        /></svg
                    >
                    Secure 256-bit Encrypted Checkout. Cancel anytime.
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center mb-4">Frequently Asked Questions</h2>
            <p class="text-center text-rtp-muted mb-12">Common questions about brokers, capital, and risk management.</p>
            <div class="space-y-4">
                {#each faqList as faq, i}
                    <div class="border border-rtp-border rounded-xl bg-rtp-surface overflow-hidden">
                        <button
                            class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors"
                            onclick={() => toggleFaq(i)}
                        >
                            <span class="pr-8">{faq.q}</span>
                            <svg
                                class="w-5 h-5 text-rtp-muted transform transition-transform duration-300 flex-shrink-0 {openFaq ===
                                i
                                    ? 'rotate-180'
                                    : ''}"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                                ><path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M19 9l-7 7-7-7"
                                /></svg
                            >
                        </button>
                        {#if openFaq === i}
                            <div
                                transition:slide={{ duration: 300, easing: cubicOut }}
                                class="px-6 pb-6 text-rtp-muted text-sm leading-relaxed border-t border-rtp-border/50 pt-4"
                            >
                                {faq.a}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="py-24 relative overflow-hidden">
        <div class="absolute inset-0 bg-gradient-to-br from-rtp-primary to-rtp-indigo z-0"></div>
        <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10 z-0"></div>

        <div class="relative z-10 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
            <h2 class="text-4xl md:text-6xl font-heading font-extrabold text-white mb-6 tracking-tight">
                Ready to Level Up?
            </h2>
            <p class="text-xl text-blue-100 mb-10 max-w-2xl mx-auto">
                Join the ranks of professional traders capturing daily alpha in the SPX.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                <a
                    href="#pricing"
                    class="bg-white text-rtp-primary px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-2xl hover:-translate-y-1"
                >
                    Get Access Now
                </a>
            </div>
            <p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee on Annual Plans</p>
        </div>
    </section>

    <footer class="bg-rtp-bg py-16 border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid md:grid-cols-4 gap-8 mb-12">
                <div class="col-span-2">
                    <div class="font-heading font-bold text-xl text-white mb-4">Revolution Trading Pros</div>
                    <p class="text-rtp-muted text-sm max-w-xs">
                        Empowering traders with institutional tools and community-driven education.
                    </p>
                </div>
                <div>
                    <h4 class="font-bold text-white mb-4">Service</h4>
                    <ul class="space-y-2 text-sm text-rtp-muted">
                        <li><a href="/login" class="hover:text-rtp-primary">Member Login</a></li>
                        <li><a href="#pricing" class="hover:text-rtp-primary">Pricing</a></li>
                        <li><a href="/performance" class="hover:text-rtp-primary">Track Record</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="font-bold text-white mb-4">Legal</h4>
                    <ul class="space-y-2 text-sm text-rtp-muted">
                        <li><a href="/terms" class="hover:text-rtp-primary">Terms of Service</a></li>
                        <li><a href="/privacy" class="hover:text-rtp-primary">Privacy Policy</a></li>
                    </ul>
                </div>
            </div>
            <div
                class="border-t border-rtp-border pt-8 text-xs text-rtp-muted/60 leading-relaxed text-justify"
            >
                <p class="font-bold mb-2 uppercase">Risk Disclosure</p>
                <p>
                    Trading futures and options involves substantial risk of loss and is not suitable for
                    every investor. The valuation of futures and options may fluctuate, and as a result,
                    clients may lose more than their original investment. The highly leveraged nature of
                    futures trading means that small market movements will have a great impact on your trading
                    account and this can work against you, leading to large losses or can work for you,
                    leading to large gains. Revolution Trading Pros is an educational platform and not a
                    financial advisory service. Past performance is not indicative of future results.
                </p>
                <p class="mt-6 text-center">
                    &copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
                </p>
            </div>
        </div>
    </footer>
</main>

<style>
    /* SPX Profit Pulse - Hardcoded CSS (not Tailwind) */
    :global(main) {
        --rtp-bg: oklch(0.12 0.02 250);
        --rtp-bg-alt: oklch(0.10 0.02 250);
        --rtp-surface: oklch(0.18 0.02 250);
        --rtp-surface-elevated: oklch(0.22 0.02 250);
        --rtp-border: oklch(0.30 0.02 250);
        --rtp-border-light: oklch(0.25 0.02 250);
        --rtp-text: oklch(0.96 0.01 250);
        --rtp-text-secondary: oklch(0.80 0.02 250);
        --rtp-muted: oklch(0.60 0.02 250);
        --rtp-primary: oklch(0.60 0.20 250);
        --rtp-primary-light: oklch(0.70 0.18 250);
        --rtp-indigo: oklch(0.65 0.20 270);
        --rtp-blue: oklch(0.65 0.20 240);
        --rtp-emerald: oklch(0.65 0.20 160);
    }
    
    :global(.bg-rtp-bg) { background-color: var(--rtp-bg); }
    :global(.bg-rtp-surface) { background-color: var(--rtp-surface); }
    :global(.bg-rtp-primary) { background-color: var(--rtp-primary); }
    :global(.text-rtp-text) { color: var(--rtp-text); }
    :global(.text-rtp-muted) { color: var(--rtp-muted); }
    :global(.text-white) { color: white; }
    :global(.border-rtp-border) { border-color: var(--rtp-border); }
    :global(.text-emerald-500), :global(.text-emerald-400) { color: #10b981; }
    :global(.text-rtp-primary) { color: var(--rtp-primary); }
    :global(.text-rtp-emerald) { color: var(--rtp-emerald); }
    :global(.text-rtp-indigo) { color: var(--rtp-indigo); }
    :global(.text-rtp-blue) { color: var(--rtp-blue); }
    :global(.bg-gradient-to-br) { background-image: linear-gradient(to bottom right, var(--tw-gradient-stops)); }
    :global(.from-rtp-primary) { --tw-gradient-from: var(--rtp-primary); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to, transparent); }
    :global(.to-rtp-indigo) { --tw-gradient-to: var(--rtp-indigo); }
    :global(.hover\\:text-rtp-primary:hover) { color: var(--rtp-primary); }
</style>