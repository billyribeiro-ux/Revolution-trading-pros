<script lang="ts">
    import { onMount } from 'svelte';
    import { slide } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';
    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- Pricing State (Svelte 5 Runes) ---
    let selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');

    // --- FAQ Logic (Svelte 5 Runes) ---
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

    // --- DATA SOURCE: FAQs (Single Source of Truth) ---
    const faqList = [
        {
            question: 'How much capital do I need for swing trading?',
            answer: 'We recommend a minimum of $2,000 to properly manage risk. Unlike day trading, you do NOT need $25,000 since you are not subject to Pattern Day Trader (PDT) rules if using a cash account or limiting frequency. Swing trading is very capital efficient.'
        },
        {
            question: 'Are these day trades?',
            answer: 'No. These are swing trades designed to be held for 3-7 days on average. The goal is to catch larger moves (5-10% on stock, 20-100% on options) without staring at the screen all day.'
        },
        {
            question: 'Do I need to be at my computer all day?',
            answer: 'Absolutely not. This service is specifically built for people with day jobs. We send alerts via SMS, Email, and Push Notification. You can execute the trade on your phone in under 60 seconds.'
        },
        {
            question: 'What instruments do you trade?',
            answer: 'We primarily trade equity options (Calls and Puts) on large-cap tech stocks (NVDA, AMD, TSLA, MSFT) and major indices (SPY, QQQ). We occasionally trade common shares if the setup implies a longer-term hold.'
        },
        {
            question: 'How are the alerts delivered?',
            answer: 'Redundancy is key. You receive alerts through our private Discord server, instant Push Notifications via our mobile app integration, and optional SMS text messages. You will never miss a setup.'
        },
        {
            question: 'Do you trade both long and short?',
            answer: 'Yes. We are market agnostic. We buy Calls when the trend is up, and we buy Puts when the trend breaks down. Making money in bear markets is a core part of our strategy.'
        },
        {
            question: 'What is your risk management strategy?',
            answer: 'We are risk-first. Every alert comes with a predefined Stop Loss level. We generally look for a 1:3 Risk/Reward ratio. If a trade hits our invalidation point, we cut it immediately to preserve capital.'
        },
        {
            question: 'Is this suitable for beginners?',
            answer: 'Yes. Because the pace is slower than day trading, beginners have more time to analyze the entry, check the chart, and execute the order without panic. We also provide a "Swing Trading Bootcamp" video course to get you started.'
        },
        {
            question: 'Can I cancel my subscription?',
            answer: 'Yes, anytime. You can manage your subscription directly from the dashboard. There are no contracts or hidden fees.'
        },
        {
            question: 'What happens if I miss an entry?',
            answer: 'We always provide an "Entry Zone" rather than a specific penny. If the price is still within the zone, you are good to enter. If it has run too far, we advise waiting for a pullback or skipping the trade. Never chase.'
        }
    ];

    // --- SEO: STRUCTURED DATA (JSON-LD) ---
    const productSchema = {
        '@context': 'https://schema.org',
        '@type': 'Product',
        name: 'Explosive Swings Trading Alerts',
        description:
            'Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals. Verified 82% win rate.',
        brand: {
            '@type': 'Organization',
            name: 'Revolution Trading Pros'
        },
        aggregateRating: {
            '@type': 'AggregateRating',
            ratingValue: '4.8',
            reviewCount: '342'
        },
        offers: {
            '@type': 'AggregateOffer',
            priceCurrency: 'USD',
            lowPrice: '97',
            highPrice: '927',
            offerCount: '3',
            offers: [
                 {
                    '@type': 'Offer',
                    name: 'Monthly Swing Access',
                    price: '97',
                    priceCurrency: 'USD',
                    availability: 'https://schema.org/InStock',
                    priceSpecification: {
                        '@type': 'UnitPriceSpecification',
                        price: '97',
                        priceCurrency: 'USD',
                        referenceQuantity: { '@type': 'QuantitativeValue', value: '1', unitCode: 'MON' }
                    }
                },
                {
                    '@type': 'Offer',
                    name: 'Quarterly Swing Access',
                    price: '247',
                    priceCurrency: 'USD'
                },
                {
                    '@type': 'Offer',
                    name: 'Annual Swing Access',
                    price: '927',
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
            name: item.question,
            acceptedAnswer: {
                '@type': 'Answer',
                text: item.answer
            }
        }))
    };

    const combinedSchema = [productSchema, faqSchema];
</script>

<SEOHead
    title="Swing Trading Alerts | Multi-Day Stock & Options Signals"
    description="The #1 swing trading room for part-time traders. Get high-precision stock and options alerts (3-7 day holds) sent via SMS & Push. 82% historical win rate."
    canonical="/live-trading-rooms/swing-trading"
    ogType="product"
    ogImage="/images/og-swings.jpg"
    ogImageAlt="Swing Trading Room - Multi-Day Trading Opportunities"
    keywords={[
        'swing trading room',
        'swing trading alerts',
        'multi-day trading strategies',
        'stock options alerts sms',
        'swing trade signals',
        'options swing trading',
        'part time trading',
        'swing trading for beginners'
    ]}
    schema={combinedSchema}
    schemaType="Product"
    productPrice={97}
    productCurrency="USD"
    productAvailability="in stock"
/>

<main
    class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text font-sans selection:bg-rtp-emerald selection:text-white"
>
    <section class="relative min-h-[90vh] flex items-center overflow-hidden py-20 lg:py-0">
        <div class="absolute inset-0 bg-rtp-bg z-0 pointer-events-none">
            <div
                class="absolute inset-0 bg-[linear-gradient(rgba(16,185,129,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(16,185,129,0.03)_1px,transparent_1px)] bg-[size:48px_48px] opacity-50"
            ></div>
            <div
                class="absolute top-0 right-0 w-[600px] h-[600px] bg-rtp-emerald/10 rounded-full blur-[100px] animate-pulse"
            ></div>
            <div
                class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-rtp-blue/10 rounded-full blur-[120px]"
            ></div>
        </div>

        <div
            class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-16 items-center"
        >
            <div class="text-center lg:text-left">
                <div
                    use:reveal={{ delay: 0 }}
                    class="inline-flex items-center gap-2 bg-rtp-surface border border-rtp-emerald/30 px-4 py-1.5 rounded-full mb-8 shadow-lg shadow-emerald-500/10 backdrop-blur-md hover:border-rtp-emerald/50 transition-colors cursor-default"
                >
                    <span class="relative flex h-2.5 w-2.5">
                        <span
                            class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
                        ></span>
                        <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
                    </span>
                    <span class="text-xs font-bold tracking-wider uppercase text-emerald-400"
                        >Signals Active • Last Alert: +45% (AMD)</span
                    >
                </div>

                <h1
                    use:reveal={{ delay: 100 }}
                    class="text-5xl md:text-7xl font-heading font-extrabold mb-6 leading-tight tracking-tight"
                >
                    Catch the <br />
                    <span
                        class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-emerald via-emerald-300 to-teal-200"
                        >Big Moves.</span
                    >
                </h1>

                <p
                    use:reveal={{ delay: 200 }}
                    class="text-xl text-rtp-muted mb-10 max-w-2xl mx-auto lg:mx-0 leading-relaxed"
                >
                    Stop staring at the 1-minute chart. Get high-precision <strong>multi-day swing alerts</strong> designed for
                    traders who want freedom, not another 9-to-5 job.
                </p>

                <div
                    use:reveal={{ delay: 300 }}
                    class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start items-center"
                >
                    <a
                        href="#pricing"
                        class="group relative w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-rtp-emerald rounded-xl hover:bg-emerald-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-rtp-emerald offset-rtp-bg shadow-lg hover:shadow-emerald-500/25 hover:-translate-y-1"
                    >
                        Start Trading Swings
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
                        href="#process"
                        class="w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-rtp-text transition-all duration-200 bg-rtp-surface border border-rtp-border rounded-xl hover:bg-rtp-surface/80 hover:border-rtp-emerald/30"
                    >
                        See How It Works
                    </a>
                </div>

                <div
                    use:reveal={{ delay: 400 }}
                    class="mt-10 flex flex-wrap items-center justify-center lg:justify-start gap-6 text-sm font-medium text-rtp-muted/60"
                >
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        <span>Precise Entries</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        <span>3-7 Day Holds</span>
                    </div>
                     <div class="flex items-center gap-2">
                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
                        <span>SMS Alerts</span>
                    </div>
                </div>
            </div>

            <div class="hidden lg:block relative perspective-1000">
                <div
                    class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-gradient-to-tr from-rtp-emerald/20 to-transparent rounded-full blur-3xl"
                ></div>

                <div
                    class="relative bg-rtp-surface/90 backdrop-blur-xl border border-rtp-border/50 p-8 rounded-3xl shadow-2xl transform rotate-y-[-12deg] rotate-x-[5deg] hover:rotate-0 transition-transform duration-700 ease-[cubic-bezier(0.25,0.1,0.25,1)]"
                >
                    <div class="flex justify-between items-center mb-8">
                        <div>
                            <h3 class="text-2xl font-bold text-white">Swing Alert 🚀</h3>
                            <p class="text-rtp-emerald text-sm font-bold">High Probability Setup</p>
                        </div>
                        <div
                            class="bg-rtp-bg px-3 py-1 rounded-lg border border-rtp-border text-xs font-mono text-rtp-muted"
                        >
                            Sent: 10:30 AM
                        </div>
                    </div>

                    <div class="space-y-6">
                        <div class="bg-rtp-bg p-4 rounded-xl border-l-4 border-emerald-500">
                            <div class="text-xs text-rtp-muted uppercase tracking-wider mb-1">Action</div>
                            <div class="text-lg font-bold text-white flex items-center gap-2">
                                <span class="bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded text-sm"
                                    >BUY</span
                                >
                                NVDA 480 CALLS
                            </div>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div class="bg-rtp-bg p-4 rounded-xl border border-rtp-border/50">
                                <div class="text-xs text-rtp-muted uppercase tracking-wider mb-1">Entry Zone</div>
                                <div class="text-xl font-mono font-bold text-white">$5.50 - $6.00</div>
                            </div>
                            <div class="bg-rtp-bg p-4 rounded-xl border border-rtp-border/50">
                                <div class="text-xs text-rtp-muted uppercase tracking-wider mb-1">Target</div>
                                <div class="text-xl font-mono font-bold text-emerald-400">$8.50+</div>
                            </div>
                        </div>

                        <div class="bg-rtp-bg p-4 rounded-xl border border-red-500/30">
                            <div class="flex justify-between items-center">
                                <div class="text-xs text-rtp-muted uppercase tracking-wider">
                                    Invalidation (Stop)
                                </div>
                                <div class="text-red-400 font-mono font-bold">$4.20 (Hard Stop)</div>
                            </div>
                        </div>
                    </div>

                    <div
                        class="absolute -right-6 -bottom-6 bg-emerald-500 text-white p-4 rounded-2xl shadow-xl shadow-emerald-500/20 animate-bounce-slow"
                    >
                        <div class="text-xs font-bold opacity-80 uppercase">Potential Return</div>
                        <div class="text-2xl font-extrabold">+45%</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="bg-rtp-surface border-y border-rtp-border relative z-20">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <dl class="grid grid-cols-2 md:grid-cols-4 gap-8">
                <div class="text-center group cursor-default">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2 group-hover:text-emerald-400 transition-colors">
                        Historical Win Rate
                    </dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-emerald">82%</dd>
                </div>
                <div class="text-center group cursor-default">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2 group-hover:text-emerald-400 transition-colors">
                        Avg Hold Time
                    </dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-primary">
                        3-7<span class="text-lg text-rtp-muted font-normal ml-1">days</span>
                    </dd>
                </div>
                <div class="text-center group cursor-default">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2 group-hover:text-emerald-400 transition-colors">
                        Risk/Reward
                    </dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-indigo">4:1</dd>
                </div>
                <div class="text-center group cursor-default">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2 group-hover:text-emerald-400 transition-colors">
                        Alerts Per Week
                    </dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-blue">2-4</dd>
                </div>
            </dl>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg relative overflow-hidden">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                 <span class="text-rtp-emerald font-bold uppercase tracking-wider text-sm mb-2 block">Lifestyle First</span>
                <h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
                    Choose Your Battle
                </h2>
                <p use:reveal={{ delay: 100 }} class="text-xl text-rtp-muted max-w-2xl mx-auto">
                    Most traders burn out trying to scalp 1-minute candles. We play the bigger timeframe for bigger peace of mind.
                </p>
            </div>

            <div class="grid md:grid-cols-2 gap-8 max-w-5xl mx-auto">
                <div
                    use:reveal={{ delay: 0 }}
                    class="bg-rtp-surface/50 border border-rtp-border rounded-3xl p-10 opacity-70 hover:opacity-100 transition-all duration-300"
                >
                    <div class="flex items-center gap-4 mb-6">
                        <div
                            class="w-12 h-12 rounded-full bg-rtp-border flex items-center justify-center text-2xl shadow-inner"
                        >
                            😰
                        </div>
                        <h3 class="text-2xl font-bold text-rtp-muted">Day Scalper</h3>
                    </div>
                    <ul class="space-y-4 text-rtp-muted">
                        <li class="flex items-center gap-3">
                            <span class="text-red-400 font-bold text-lg">✕</span> Glued to screen 6 hours/day
                        </li>
                        <li class="flex items-center gap-3">
                            <span class="text-red-400 font-bold text-lg">✕</span> High stress, cortisol spikes
                        </li>
                        <li class="flex items-center gap-3">
                            <span class="text-red-400 font-bold text-lg">✕</span> Expensive commissions eat profits
                        </li>
                        <li class="flex items-center gap-3">
                            <span class="text-red-400 font-bold text-lg">✕</span> "Did I miss the move?" anxiety
                        </li>
                    </ul>
                </div>

                <div
                    use:reveal={{ delay: 150 }}
                    class="bg-rtp-surface border-2 border-rtp-emerald rounded-3xl p-10 shadow-2xl shadow-emerald-500/10 relative overflow-hidden transform md:scale-105"
                >
                    <div
                        class="absolute top-0 right-0 bg-rtp-emerald text-white text-xs font-bold px-3 py-1 rounded-bl-xl shadow-md"
                    >
                        RECOMMENDED
                    </div>
                    <div class="flex items-center gap-4 mb-6">
                        <div
                            class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center text-2xl shadow-inner border border-emerald-500/20"
                        >
                            🧘‍♂️
                        </div>
                        <h3 class="text-2xl font-bold text-rtp-text">Swing Trader</h3>
                    </div>
                    <ul class="space-y-4 text-rtp-text font-medium">
                        <li class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                             Check charts once a day (evening)
                        </li>
                        <li class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                             Calm, calculated decisions
                        </li>
                        <li class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                             Catch the meat of the move (20%+)
                        </li>
                        <li class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                             Keep your day job & freedom
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    </section>

    <section id="process" class="py-24 bg-rtp-surface border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                 <span class="text-rtp-emerald font-bold uppercase tracking-wider text-sm mb-2 block">How It Works</span>
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text">
                    The Anatomy of a Swing Trade
                </h2>
            </div>
            
            <div class="grid md:grid-cols-3 gap-10">
                <article use:reveal={{ delay: 0 }} class="group">
                    <div
                        class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-rtp-emerald/20"
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
                                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Institutional Flow Analysis</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        Retail traders guess. We track <strong>Dark Pool prints</strong> and unusual options flow to identify where institutions are positioning billions of dollars before the price moves. We ride the whale's wake.
                    </p>
                </article>

                <article use:reveal={{ delay: 100 }} class="group">
                    <div
                        class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-rtp-primary/20"
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
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Multi-Channel Alerts</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        You can't miss the entry. We send alerts via <strong>SMS Text, Email, Push Notification, and Discord</strong> immediately when our criteria are met. You have a full window to enter the trade.
                    </p>
                </article>

                <article use:reveal={{ delay: 200 }} class="group">
                    <div
                        class="w-14 h-14 bg-rtp-indigo/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-rtp-indigo/20"
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
                                d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                            /></svg
                        >
                    </div>
                    <h3 class="text-xl font-bold text-rtp-text mb-3">Risk-First Approach</h3>
                    <p class="text-rtp-muted leading-relaxed text-sm">
                        Capital preservation is our #1 job. Every trade comes with a predefined "Hard Stop" level. We cut losers fast and let winners run to target. No bag holding. No hope.
                    </p>
                </article>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg">
        <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex flex-col md:flex-row justify-between items-end mb-10 gap-4">
                <div>
                    <h2 class="text-3xl font-heading font-bold text-rtp-text mb-2">Recent Verified Swings</h2>
                    <p class="text-rtp-muted">Real trades. Real timestamps. Verified community results.</p>
                </div>
                <div>
                    <a href="/performance" class="text-emerald-500 font-bold hover:text-emerald-400 flex items-center gap-1 text-sm">
                        View Full Performance Log
                         <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"></path></svg>
                    </a>
                </div>
            </div>

            <div class="bg-rtp-surface rounded-2xl border border-rtp-border overflow-hidden shadow-xl">
                <div
                    class="grid grid-cols-12 bg-rtp-surface border-b border-rtp-border p-4 text-xs font-bold uppercase text-rtp-muted tracking-wider"
                >
                    <div class="col-span-3 md:col-span-2">Ticker</div>
                    <div class="col-span-3 md:col-span-2">Type</div>
                    <div class="col-span-3 md:col-span-2">Days Held</div>
                    <div class="col-span-3 md:col-span-2 text-right">Return</div>
                    <div class="hidden md:block md:col-span-4 text-right">Notes</div>
                </div>
                <div class="divide-y divide-rtp-border/50 font-mono text-sm">
                    <div class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors">
                        <div class="col-span-3 md:col-span-2 font-bold text-white flex items-center gap-2">
                             <img src="/logos/nvda.svg" alt="" class="w-5 h-5 opacity-70 hidden sm:block">
                             NVDA
                        </div>
                        <div class="col-span-3 md:col-span-2 text-emerald-400 font-bold">CALLS</div>
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">5 Days</div>
                        <div class="col-span-3 md:col-span-2 text-right text-emerald-400 font-bold">+125%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Breakout over $480 psych level.
                        </div>
                    </div>
                    <div class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors">
                        <div class="col-span-3 md:col-span-2 font-bold text-white flex items-center gap-2">
                            <img src="/logos/amd.svg" alt="" class="w-5 h-5 opacity-70 hidden sm:block">
                            AMD
                        </div>
                        <div class="col-span-3 md:col-span-2 text-emerald-400 font-bold">CALLS</div>
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">3 Days</div>
                        <div class="col-span-3 md:col-span-2 text-right text-emerald-400 font-bold">+45%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Sector rotation play / AI Lag.
                        </div>
                    </div>
                    <div
                        class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors bg-red-500/5"
                    >
                        <div class="col-span-3 md:col-span-2 font-bold text-white flex items-center gap-2">
                             <img src="/logos/tsla.svg" alt="" class="w-5 h-5 opacity-70 hidden sm:block">
                            TSLA
                        </div>
                        <div class="col-span-3 md:col-span-2 text-red-400 font-bold">PUTS</div>
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">1 Day</div>
                        <div class="col-span-3 md:col-span-2 text-right text-red-400 font-bold">-15%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Hit stop loss on reversal. Rule disciplined.
                        </div>
                    </div>
                    <div class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors">
                        <div class="col-span-3 md:col-span-2 font-bold text-white flex items-center gap-2">
                             <img src="/logos/meta.svg" alt="" class="w-5 h-5 opacity-70 hidden sm:block">
                            META
                        </div>
                        <div class="col-span-3 md:col-span-2 text-emerald-400 font-bold">CALLS</div>
                        <div class="col-span-3 md:col-span-2 text-rtp-muted">7 Days</div>
                        <div class="col-span-3 md:col-span-2 text-right text-emerald-400 font-bold">+82%</div>
                        <div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
                            Earnings run-up swing strategy.
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-rtp-surface border-t border-rtp-border relative">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                 <span class="text-rtp-emerald font-bold uppercase tracking-wider text-sm mb-2 block">Membership</span>
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
                    Simple Pricing
                </h2>
                <p class="text-xl text-rtp-muted max-w-3xl mx-auto">
                    Pay for the subscription with your first successful swing trade.
                </p>
            </div>

            <div class="flex justify-center mb-16">
                <div class="bg-rtp-bg p-1.5 rounded-xl border border-rtp-border inline-flex relative shadow-inner">
                    <button
                        type="button"
                        onclick={() => (selectedPlan = 'monthly')}
                        class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
                        'monthly'
                            ? 'text-white'
                            : 'text-rtp-muted hover:text-white'}">Monthly</button
                    >
                    <button
                        type="button"
                        onclick={() => (selectedPlan = 'quarterly')}
                        class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
                        'quarterly'
                            ? 'text-white'
                            : 'text-rtp-muted hover:text-white'}">Quarterly</button
                    >
                    <button
                        type="button"
                        onclick={() => (selectedPlan = 'annual')}
                        class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
                        'annual'
                            ? 'text-white'
                            : 'text-rtp-muted hover:text-white'}">Annual</button
                    >

                    <div
                        class="absolute top-1.5 bottom-1.5 bg-rtp-emerald rounded-lg shadow-md transition-all duration-300 ease-[cubic-bezier(0.25,0.1,0.25,1)]"
                        style="left: {selectedPlan === 'monthly'
                            ? '0.375rem'
                            : selectedPlan === 'quarterly'
                                ? 'calc(33.33% + 0.2rem)'
                                : 'calc(66.66% + 0.1rem)'}; width: calc(33.33% - 0.4rem);"
                    ></div>
                </div>
            </div>

            <div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto items-center">
                <div
                    class="bg-rtp-bg p-8 rounded-2xl border transition-all duration-300 {selectedPlan === 'monthly' ? 'border-rtp-emerald opacity-100 scale-105 shadow-xl shadow-emerald-500/10' : 'border-rtp-border opacity-70 hover:opacity-100'}"
                >
                    <h3 class="text-xl font-bold text-white mb-4">Monthly</h3>
                    <div class="flex items-baseline gap-1 mb-6">
                        <span class="text-4xl font-bold text-white">$97</span>
                        <span class="text-rtp-muted">/mo</span>
                    </div>
                    <div class="text-xs font-mono text-rtp-muted bg-rtp-surface p-2 rounded mb-6 text-center border border-rtp-border">
                        $3.20 per day
                    </div>
                    <ul class="space-y-4 mb-8 text-sm text-rtp-muted">
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> 2-4 Premium Swings / Week</li>
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> Instant SMS & Email Alerts</li>
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> Private Discord Community</li>
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> Swing Bootcamp Video</li>
                    </ul>
                    <a
                        href="/checkout/monthly-swings"
                        class="block w-full py-3 bg-rtp-surface border border-rtp-border text-white font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
                        >Select Monthly</a
                    >
                </div>

                <div
                    class="bg-rtp-bg p-10 rounded-3xl border-2 shadow-2xl transform relative z-10 transition-all duration-300 {selectedPlan === 'quarterly' ? 'border-rtp-emerald shadow-rtp-emerald/20 md:scale-110 opacity-100' : 'border-rtp-border shadow-rtp-border/10 md:scale-100 opacity-70 hover:opacity-100'}"
                >
                    <div
                        class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-emerald text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide shadow-lg"
                    >
                        Most Popular
                    </div>
                    <h3 class="text-2xl font-bold text-white mb-4">Quarterly</h3>
                    <div class="flex items-baseline gap-1 mb-6">
                        <span class="text-5xl font-extrabold text-white">$247</span>
                        <span class="text-rtp-muted">/qtr</span>
                    </div>
                    <div
                        class="text-xs font-mono text-emerald-400 bg-emerald-500/10 p-2 rounded mb-6 text-center border border-emerald-500/30"
                    >
                        Save 15% ($2.75 / day)
                    </div>
                    <ul class="space-y-4 mb-8 text-sm text-white">
                        <li class="flex gap-3">
                            <span class="text-rtp-emerald font-bold">✓</span>
                            <span class="font-bold">Priority Support</span>
                        </li>
                        <li class="flex gap-3">
                            <span class="text-rtp-emerald font-bold">✓</span> 2-4 Premium Swings / Week
                        </li>
                        <li class="flex gap-3">
                            <span class="text-rtp-emerald font-bold">✓</span> Instant SMS & Email Alerts
                        </li>
                        <li class="flex gap-3">
                            <span class="text-rtp-emerald font-bold">✓</span> Private Discord Community
                        </li>
                    </ul>
                    <a
                        href="/checkout/quarterly-swings"
                        class="block w-full py-4 bg-rtp-emerald text-white font-bold rounded-xl text-center hover:bg-emerald-600 transition-colors shadow-lg hover:shadow-emerald-500/50"
                        >Join Quarterly</a
                    >
                </div>

                <div
                    class="bg-rtp-surface p-8 rounded-2xl border transition-all duration-300 {selectedPlan === 'annual' ? 'border-emerald-500 opacity-100 scale-105 shadow-xl shadow-emerald-500/10' : 'border-rtp-border opacity-70 hover:opacity-100'}"
                >
                    <h3 class="text-xl font-bold text-white mb-4">Annual</h3>
                    <div class="flex items-baseline gap-1 mb-6">
                        <span class="text-4xl font-bold text-white">$927</span>
                        <span class="text-rtp-muted">/yr</span>
                    </div>
                    <div class="text-xs font-mono text-rtp-emerald bg-rtp-bg p-2 rounded mb-6 text-center border border-rtp-border">
                        Save 20% ($2.54 / day)
                    </div>
                    <ul class="space-y-4 mb-8 text-sm text-rtp-muted">
                        <li class="flex gap-3">
                            <span class="text-rtp-emerald">✓</span>
                            <span class="font-bold">Strategy Video Library</span>
                        </li>
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> 2-4 Premium Swings / Week</li>
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> Instant SMS & Email Alerts</li>
                        <li class="flex gap-3"><span class="text-rtp-emerald">✓</span> Private Discord Community</li>
                    </ul>
                    <a
                        href="/checkout/annual-swings"
                        class="block w-full py-3 bg-rtp-bg border border-rtp-emerald text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-white transition-colors"
                        >Select Annual</a
                    >
                </div>
            </div>
            
            <div class="mt-12 text-center">
                <p class="text-rtp-muted text-sm mb-4">Secure checkout powered by Stripe. Cancel anytime.</p>
                <div class="flex items-center justify-center gap-2 text-rtp-muted text-sm bg-rtp-bg inline-flex px-4 py-2 rounded-full border border-rtp-border">
                    <svg class="w-4 h-4 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    <span>30-Day Money Back Guarantee. Zero risk to try.</span>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg border-t border-rtp-border">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">
                Frequently Asked Questions
            </h2>
            <div class="space-y-4">
                {#each faqList as faq, i}
                    <div class="border border-rtp-border rounded-xl bg-rtp-surface overflow-hidden hover:border-rtp-emerald/30 transition-colors">
                        <button
                            class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-rtp-text"
                            onclick={() => toggleFaq(i)}
                            aria-expanded={openFaq === i}
                        >
                            <span class="text-base pr-4">{faq.question}</span>
                            <svg
                                class="w-5 h-5 text-rtp-emerald flex-shrink-0 transform transition-transform duration-300 {openFaq ===
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
                                {faq.answer}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section
        class="py-24 bg-gradient-to-br from-rtp-emerald to-teal-900 text-white relative overflow-hidden"
    >
        <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
            <h2 class="text-4xl md:text-6xl font-heading font-extrabold mb-6 tracking-tight">
                Stop Overtrading. <br> Start Swinging.
            </h2>
            <p class="text-xl text-emerald-100 mb-10 max-w-2xl mx-auto">
                Join the trading room that values your time as much as your capital.
            </p>
            <a
                href="#pricing"
                class="inline-block bg-white text-emerald-700 px-10 py-5 rounded-xl font-bold text-lg hover:bg-emerald-50 transition-all shadow-2xl hover:-translate-y-1"
            >
                Get Instant Access
            </a>
            <p class="mt-6 text-sm text-emerald-100/70">Secure Checkout • Cancel Anytime</p>
        </div>
    </section>

    <footer class="bg-rtp-bg py-12 border-t border-rtp-border">
        <div
            class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-rtp-muted leading-relaxed"
        >
            <p class="mb-4 font-bold uppercase text-rtp-text">Risk Disclosure</p>
            <p class="max-w-4xl mx-auto">
                Trading in financial markets involves a high degree of risk and may not be suitable for all
                investors. Options trading involves significant risk and is not suitable for every investor. 
                You could lose some or all of your initial investment; do not invest money that
                you cannot afford to lose. Past performance is not indicative of future results. Revolution
                Trading Pros is an educational platform and does not provide personalized financial advice.
            </p>
            <div class="flex justify-center gap-6 mt-8 mb-4">
                 <a href="/terms" class="hover:text-emerald-400 transition-colors">Terms of Service</a>
                 <a href="/privacy" class="hover:text-emerald-400 transition-colors">Privacy Policy</a>
                 <a href="/contact" class="hover:text-emerald-400 transition-colors">Contact Support</a>
            </div>
            <p class="opacity-50">
                &copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
            </p>
        </div>
    </footer>
</main>