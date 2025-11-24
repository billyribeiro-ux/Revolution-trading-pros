<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { onMount } from 'svelte';

    // State for pricing toggle
    let selectedPlan: 'monthly' | 'annual' = 'monthly';

    // FAQ State (Added for SEO value)
    let openFaq: number | null = null;
    const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

    // --- SEO: STRUCTURED DATA (JSON-LD) ---
    // This tells Google exactly what you are selling and how much it costs.
    const productSchema = {
        "@context": "https://schema.org",
        "@type": "Product",
        "name": "Explosive Swings Trading Alerts",
        "description": "Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals.",
        "brand": {
            "@type": "Organization",
            "name": "Revolution Trading Pros"
        },
        "offers": {
            "@type": "Offer",
            "priceCurrency": "USD",
            "price": selectedPlan === 'monthly' ? "97" : "927",
            "availability": "https://schema.org/InStock",
            "url": "https://revolutiontradingpros.com/explosive-swings",
            "category": "FinancialService"
        }
    };

    const faqSchema = {
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": [
            {
                "@type": "Question",
                "name": "How much capital do I need for swing trading?",
                "acceptedAnswer": { "@type": "Answer", "text": "We recommend a minimum of $2,000 to properly manage risk, though our strategies work on accounts of all sizes." }
            },
            {
                "@type": "Question",
                "name": "Are these day trades?",
                "acceptedAnswer": { "@type": "Answer", "text": "No. These are swing trades held typically for 3-7 days, designed for traders who cannot watch the screen all day." }
            }
        ]
    };

    const jsonLd = JSON.stringify({ "@graph": [productSchema, faqSchema] });
</script>

<svelte:head>
    <title>Explosive Swings Alerts | Multi-Day Swing Trading | Revolution Trading Pros</title>
    <meta name="description" content="Get explosive swing trading alerts for multi-day opportunities. Expert analysis, precise entries/exits, and proven strategies for swing traders." />
    <meta name="keywords" content="swing trading alerts, stock options alerts, swing trade signals, multi-day trading strategies" />
    
    <meta property="og:type" content="product" />
    <meta property="og:title" content="Explosive Swings Alerts | Catch Multi-Day Moves" />
    <meta property="og:description" content="Don't stare at charts all day. Get high-probability swing trade alerts sent to your phone." />
    <meta property="og:url" content="https://revolutiontradingpros.com/explosive-swings" />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/og-swings.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Explosive Swings Trading Alerts" />
    <meta name="twitter:description" content="Precision swing trading alerts. 82% win rate. 3-7 day hold times." />

    <script type="application/ld+json">
        {@html jsonLd}
    </script>
</svelte:head>

<main class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text selection:bg-rtp-emerald selection:text-white">

    <section class="relative bg-gradient-to-br from-rtp-emerald via-rtp-blue to-rtp-primary text-white py-24 md:py-32 overflow-hidden">
        <div class="absolute inset-0 bg-black/20 z-0"></div>
        <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10 z-0"></div>
        
        <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center max-w-4xl mx-auto">
                <div class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-md border border-white/20 px-4 py-2 rounded-full mb-8 shadow-lg">
                    <svg class="w-4 h-4 text-emerald-300" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                        <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
                        <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
                    </svg>
                    <span class="text-sm font-bold tracking-wide uppercase">Premium Swing Trading Alerts</span>
                </div>
                
                <h1 class="text-5xl md:text-7xl font-heading font-extrabold mb-8 leading-tight drop-shadow-sm">
                    Explosive Swings
                </h1>
                
                <p class="text-xl md:text-2xl text-white/90 mb-10 max-w-2xl mx-auto leading-relaxed font-light">
                    Catch multi-day swing trades with our premium alert service. Get precise entry/exit signals for high-probability setups.
                </p>
                
                <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                    <a
                        href="#pricing"
                        class="w-full sm:w-auto bg-white text-rtp-emerald px-8 py-4 rounded-xl font-bold text-lg hover:bg-emerald-50 transition-all hover:-translate-y-1 shadow-xl hover:shadow-2xl"
                    >
                        Subscribe Now
                    </a>
                    <a
                        href="#pricing"
                        class="w-full sm:w-auto bg-white/10 backdrop-blur-sm text-white border border-white/30 px-8 py-4 rounded-xl font-bold text-lg hover:bg-white/20 transition-all"
                    >
                        View Pricing
                    </a>
                </div>
                
                <p class="mt-8 text-sm text-white/60 font-medium">
                    Trusted by 1,000+ Traders • Cancel Anytime
                </p>
            </div>
        </div>
    </section>

    <section class="bg-rtp-surface border-b border-rtp-border py-12 relative z-20 shadow-sm">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <dl class="grid grid-cols-2 md:grid-cols-4 gap-8 divide-x divide-rtp-border/30">
                <div class="text-center px-2">
                    <dt class="text-rtp-muted font-medium text-sm uppercase tracking-wider mb-2">Win Rate</dt>
                    <dd class="text-4xl md:text-5xl font-bold text-rtp-emerald">82%</dd>
                </div>
                <div class="text-center px-2 border-l-0">
                    <dt class="text-rtp-muted font-medium text-sm uppercase tracking-wider mb-2">Days Hold Time</dt>
                    <dd class="text-4xl md:text-5xl font-bold text-rtp-primary">3-7</dd>
                </div>
                <div class="text-center px-2 border-l-0">
                    <dt class="text-rtp-muted font-medium text-sm uppercase tracking-wider mb-2">Avg Risk/Reward</dt>
                    <dd class="text-4xl md:text-5xl font-bold text-rtp-indigo">4:1</dd>
                </div>
                <div class="text-center px-2 border-l-0">
                    <dt class="text-rtp-muted font-medium text-sm uppercase tracking-wider mb-2">Alerts Per Week</dt>
                    <dd class="text-4xl md:text-5xl font-bold text-rtp-blue">2-4</dd>
                </div>
            </dl>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-20">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
                    What's Included
                </h2>
                <p class="text-xl text-rtp-muted max-w-3xl mx-auto">
                    Everything you need to execute profitable swing trades without watching the screen all day.
                </p>
            </div>

            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all border border-rtp-border/50 group">
                    <div class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                        <svg class="w-7 h-7 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Precise Entry Alerts</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Get exact entry prices, position sizes, and stop losses for each swing trade setup. No guesswork.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all border border-rtp-border/50 group">
                    <div class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                        <svg class="w-7 h-7 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Technical Analysis</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Detailed chart analysis explaining the setup, key levels, and expected price action. Learn while you earn.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all border border-rtp-border/50 group">
                    <div class="w-14 h-14 bg-rtp-indigo/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                        <svg class="w-7 h-7 text-rtp-indigo" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Exit Management</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Real-time updates on profit targets, trailing stops, and when to exit positions to maximize gains.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all border border-rtp-border/50 group">
                    <div class="w-14 h-14 bg-rtp-blue/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                        <svg class="w-7 h-7 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Risk Management</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Clear risk parameters with calculated position sizing based on your account size. Protect your capital.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all border border-rtp-border/50 group">
                    <div class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                        <svg class="w-7 h-7 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">SMS & Email Alerts</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Receive instant notifications via SMS, email, and Discord. Never miss a trade setup again.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg hover:shadow-xl transition-all border border-rtp-border/50 group">
                    <div class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                        <svg class="w-7 h-7 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Trading Education</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        We don't just fish for you; we teach you to fish. Learn the strategy behind each trade with detailed market context.
                    </p>
                </article>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface border-y border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid md:grid-cols-2 gap-16 items-center">
                <div>
                    <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-8">
                        Our Swing Trading Approach
                    </h2>
                    <p class="text-lg text-rtp-muted mb-10 leading-relaxed">
                        We focus on high-probability swing setups with clear technical levels and momentum confirmation. Each trade is carefully analyzed for optimal risk/reward ratios.
                    </p>
                    <div class="space-y-8">
                        <div class="flex items-start gap-5">
                            <div class="flex-shrink-0 w-12 h-12 bg-rtp-emerald rounded-full flex items-center justify-center shadow-lg shadow-emerald-500/20">
                                <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                </svg>
                            </div>
                            <div>
                                <h3 class="text-xl font-bold text-rtp-text mb-1">Technical Confirmation</h3>
                                <p class="text-rtp-muted">
                                    Multiple timeframe analysis checks daily, weekly, and hourly support/resistance levels.
                                </p>
                            </div>
                        </div>
                        <div class="flex items-start gap-5">
                            <div class="flex-shrink-0 w-12 h-12 bg-rtp-emerald rounded-full flex items-center justify-center shadow-lg shadow-emerald-500/20">
                                <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                </svg>
                            </div>
                            <div>
                                <h3 class="text-xl font-bold text-rtp-text mb-1">Momentum Analysis</h3>
                                <p class="text-rtp-muted">
                                    We wait for volume confirmation and momentum indicators to align for precise entry timing.
                                </p>
                            </div>
                        </div>
                        <div class="flex items-start gap-5">
                            <div class="flex-shrink-0 w-12 h-12 bg-rtp-emerald rounded-full flex items-center justify-center shadow-lg shadow-emerald-500/20">
                                <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                </svg>
                            </div>
                            <div>
                                <h3 class="text-xl font-bold text-rtp-text mb-1">Risk Management</h3>
                                <p class="text-rtp-muted">
                                    Defined stop losses and profit targets. We never enter a trade without a 3:1 minimum Risk/Reward.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="bg-rtp-bg p-10 rounded-3xl shadow-xl border border-rtp-border">
                    <h3 class="text-2xl font-heading font-bold text-rtp-text mb-8">Typical Trade Profile</h3>
                    <div class="space-y-8">
                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-rtp-muted font-medium">Hold Time</span>
                                <span class="font-bold text-rtp-text">3-7 Days</span>
                            </div>
                            <div class="w-full bg-rtp-border/40 rounded-full h-3">
                                <div class="bg-gradient-to-r from-emerald-400 to-emerald-600 h-3 rounded-full shadow-sm" style="width: 75%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-rtp-muted font-medium">Win Rate</span>
                                <span class="font-bold text-rtp-text">82%</span>
                            </div>
                            <div class="w-full bg-rtp-border/40 rounded-full h-3">
                                <div class="bg-gradient-to-r from-rtp-primary to-blue-500 h-3 rounded-full shadow-sm" style="width: 82%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-rtp-muted font-medium">Avg Profit Target</span>
                                <span class="font-bold text-rtp-text">15-30%</span>
                            </div>
                            <div class="w-full bg-rtp-border/40 rounded-full h-3">
                                <div class="bg-gradient-to-r from-rtp-indigo to-purple-500 h-3 rounded-full shadow-sm" style="width: 65%"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-rtp-muted font-medium">Max Risk Per Trade</span>
                                <span class="font-bold text-rtp-text">2-3%</span>
                            </div>
                            <div class="w-full bg-rtp-border/40 rounded-full h-3">
                                <div class="bg-rtp-blue h-3 rounded-full shadow-sm" style="width: 25%"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Simple Pricing</h2>
                <p class="text-xl text-rtp-muted max-w-3xl mx-auto">
                    Choose your plan and start receiving explosive swing trade alerts today.
                </p>
            </div>

            <div class="flex justify-center mb-16">
                <div class="bg-rtp-surface p-1.5 rounded-xl shadow-inner border border-rtp-border inline-flex gap-1 relative">
                    <button
                        type="button"
                        on:click={() => (selectedPlan = 'monthly')}
                        class="relative z-10 px-8 py-3 rounded-lg font-bold text-sm md:text-base transition-colors duration-200 {selectedPlan === 'monthly' ? 'text-white' : 'text-rtp-muted hover:text-rtp-text'}"
                    >
                        Monthly
                    </button>
                    <button
                        type="button"
                        on:click={() => (selectedPlan = 'annual')}
                        class="relative z-10 px-8 py-3 rounded-lg font-bold text-sm md:text-base transition-colors duration-200 {selectedPlan === 'annual' ? 'text-white' : 'text-rtp-muted hover:text-rtp-text'}"
                    >
                        Annual
                    </button>
                    
                    <div 
                        class="absolute top-1.5 bottom-1.5 bg-rtp-emerald rounded-lg shadow-md transition-all duration-300 ease-out"
                        style="left: {selectedPlan === 'monthly' ? '0.375rem' : '50%'}; width: calc(50% - 0.375rem);"
                    ></div>
                </div>
            </div>

            <div class="max-w-md mx-auto min-h-[500px]">
                {#key selectedPlan}
                    <div in:fade={{ duration: 200, delay: 100 }} class="w-full h-full">
                        {#if selectedPlan === 'monthly'}
                            <div class="bg-rtp-surface p-8 md:p-10 rounded-3xl shadow-xl border-2 border-rtp-emerald hover:border-emerald-400 transition-colors h-full flex flex-col">
                                <div class="text-center mb-8">
                                    <h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Monthly</h3>
                                    <div class="flex items-baseline justify-center gap-1">
                                        <div class="text-5xl font-extrabold text-rtp-emerald">$97</div>
                                        <div class="text-rtp-muted font-medium">/mo</div>
                                    </div>
                                    <div class="text-sm text-rtp-muted mt-2">Billed monthly. Cancel anytime.</div>
                                </div>
                                <ul class="space-y-4 mb-8 flex-grow">
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-emerald flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">2-4 swing alerts per week</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-emerald flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">SMS & Email notifications</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-emerald flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">Full technical analysis</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-emerald flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">Real-time exit updates</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-emerald flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">Discord community access</span>
                                    </li>
                                </ul>
                                <button class="w-full bg-rtp-emerald text-white py-4 rounded-xl font-bold text-lg hover:bg-emerald-600 transition-all hover:-translate-y-1 shadow-lg shadow-emerald-500/20">
                                    Subscribe Now
                                </button>
                            </div>
                        {:else}
                            <div class="bg-rtp-surface p-8 md:p-10 rounded-3xl shadow-xl border-2 border-rtp-primary relative h-full flex flex-col">
                                <div class="absolute top-0 right-0 bg-rtp-primary text-white px-4 py-1 rounded-bl-xl font-bold text-sm">
                                    SAVE 20%
                                </div>
                                <div class="text-center mb-8">
                                    <h3 class="text-2xl font-heading font-bold text-rtp-text mb-2">Annual</h3>
                                    <div class="flex items-baseline justify-center gap-1">
                                        <div class="text-5xl font-extrabold text-rtp-primary">$927</div>
                                        <div class="text-rtp-muted font-medium">/yr</div>
                                    </div>
                                    <div class="text-sm text-rtp-primary font-bold mt-2">Like paying only $77/month</div>
                                </div>
                                <ul class="space-y-4 mb-8 flex-grow">
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-primary flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">Everything in Monthly</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-primary flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-bold text-rtp-primary">Save $237 instantly</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-primary flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">Priority Support</span>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <svg class="w-6 h-6 text-rtp-primary flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                                        <span class="text-rtp-text font-medium">Exclusive Indicators</span>
                                    </li>
                                </ul>
                                <button class="w-full bg-rtp-primary text-white py-4 rounded-xl font-bold text-lg hover:bg-blue-600 transition-all hover:-translate-y-1 shadow-lg shadow-blue-500/20">
                                    Subscribe Annual
                                </button>
                            </div>
                        {/if}
                    </div>
                {/key}
            </div>
        </div>
    </section>
    
    <section class="py-20 bg-rtp-surface border-t border-rtp-border">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">Frequently Asked Questions</h2>
            <div class="space-y-4">
                {#each [
                    { q: "How much capital do I need?", a: "We recommend starting with at least $2,000 to follow our risk management rules effectively, though the strategy scales to any account size." },
                    { q: "Do I need to be at my computer all day?", a: "No! This is exactly why we created the Swing Alerts service. We focus on multi-day moves so you can enter trades calmly without staring at the screen." },
                    { q: "How are alerts delivered?", a: "You get instant notifications via SMS text message, Email, and a private Discord channel. You choose what works for you." }
                ] as item, i}
                    <div class="border border-rtp-border rounded-xl bg-rtp-bg overflow-hidden">
                        <button 
                            class="w-full text-left px-6 py-4 font-bold flex justify-between items-center focus:outline-none"
                            on:click={() => toggleFaq(i)}
                        >
                            {item.q}
                            <svg class="w-5 h-5 transform transition-transform {openFaq === i ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
                        </button>
                        {#if openFaq === i}
                            <div transition:fade={{duration:150}} class="px-6 pb-4 text-rtp-muted">
                                {item.a}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="py-24 bg-gradient-to-br from-rtp-emerald to-rtp-primary text-white relative overflow-hidden">
        <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
            <h2 class="text-3xl md:text-5xl font-heading font-extrabold mb-6">Start Trading Explosive Swings</h2>
            <p class="text-xl text-white/90 mb-10 max-w-2xl mx-auto">
                Join hundreds of profitable swing traders. Backed by our 30-day money-back guarantee.
            </p>
            <a
                href="#pricing"
                class="inline-block bg-white text-rtp-emerald px-10 py-5 rounded-xl font-bold text-lg hover:bg-emerald-50 transition-all hover:-translate-y-1 shadow-2xl"
            >
                Get Started Today
            </a>
            <p class="mt-6 text-sm text-white/70">Secure Checkout • Cancel Anytime</p>
        </div>
    </section>

    <footer class="bg-rtp-bg py-12 border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-rtp-muted leading-relaxed">
            <p class="mb-4 font-bold">RISK DISCLOSURE</p>
            <p class="max-w-4xl mx-auto">
                Trading in financial markets involves a high degree of risk and may not be suitable for all investors. You could lose some or all of your initial investment; do not invest money that you cannot afford to lose. Past performance is not indicative of future results. Revolution Trading Pros is an educational platform and does not provide personalized financial advice.
            </p>
            <p class="mt-8 opacity-50">&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.</p>
        </div>
    </footer>

</main>