<script lang="ts">
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';

    // State for pricing toggle
    type BillingCycle = 'monthly' | 'quarterly' | 'annual';
    let selectedPlan: BillingCycle = 'monthly';

    // Structured Data (JSON-LD) construction
    const schemaOrg = {
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Organization",
                "name": "Revolution Trading Pros",
                "url": "https://revolutiontradingpros.com",
                "logo": "https://revolutiontradingpros.com/logo.png"
            },
            {
                "@type": "Product",
                "name": "Swing Trading Room Membership",
                "description": "Professional swing trading mentorship, SPX alerts, and multi-day trade setups for active traders.",
                "brand": {
                    "@type": "Brand",
                    "name": "Revolution Trading Pros"
                },
                "offers": [
                    {
                        "@type": "Offer",
                        "name": "Monthly Membership",
                        "price": "147",
                        "priceCurrency": "USD",
                        "billingDuration": "P1M"
                    },
                    {
                        "@type": "Offer",
                        "name": "Quarterly Membership",
                        "price": "353",
                        "priceCurrency": "USD",
                        "billingDuration": "P3M"
                    },
                    {
                        "@type": "Offer",
                        "name": "Annual Membership",
                        "price": "1147",
                        "priceCurrency": "USD",
                        "billingDuration": "P1Y"
                    }
                ]
            },
            {
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "What is swing trading?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "Swing trading is a strategy where positions are held for more than a single day, typically 2 to 10 days, to capture larger market moves compared to day trading."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "Do I need to be at my computer all day?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "No. Our swing trading strategy is designed for people with full-time jobs. We provide clear entry and exit alerts that can be executed from your phone."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "What capital is recommended?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "While you can start with less, we generally recommend a starting account size of at least $2,000 to properly manage risk and follow our position sizing guidelines."
                        }
                    }
                ]
            }
        ]
    };

    // Helper to format schema for head injection
    const jsonLdString = JSON.stringify(schemaOrg);
</script>

<svelte:head>
    <title>Live Swing Trading Room & SPX Alerts | Revolution Trading Pros</title>
    <meta name="description" content="Join our professional swing trading room. Get real-time multi-day SPX alerts, risk management guidance, and technical analysis for consistent growth." />
    <meta name="keywords" content="swing trading room, SPX options alerts, swing trade signals, multi-day trading, stock market mentorship" />
    <link rel="canonical" href="https://revolutiontradingpros.com/live-trading-rooms/swing-trading" />

    <meta property="og:type" content="product" />
    <meta property="og:url" content="https://revolutiontradingpros.com/live-trading-rooms/swing-trading" />
    <meta property="og:title" content="Live Swing Trading Room & SPX Alerts" />
    <meta property="og:description" content="Capture larger market moves with professionally analyzed swing trades. 2-10 day hold times. Perfect for busy professionals." />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/swing-trading-og.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Live Swing Trading Room & SPX Alerts" />
    <meta name="twitter:description" content="Professional swing trading setups and alerts. Join the community today." />
    <meta name="twitter:image" content="https://revolutiontradingpros.com/images/swing-trading-og.jpg" />

    {@html `<script type="application/ld+json">${jsonLdString}</script>`}
</svelte:head>

<main class="w-full overflow-x-hidden">
    <section class="relative bg-gradient-to-br from-rtp-indigo via-rtp-primary to-rtp-blue text-white py-20 md:py-32 overflow-hidden">
        <div class="absolute inset-0 bg-black/20 z-0" aria-hidden="true"></div>
        <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center max-w-4xl mx-auto">
                <div class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-sm px-4 py-2 rounded-full mb-8 border border-white/10">
                    <div class="w-2 h-2 bg-rtp-emerald rounded-full animate-pulse shadow-[0_0_10px_#10B981]"></div>
                    <span class="text-sm font-medium tracking-wide">Live Multi-Day Positions</span>
                </div>
                
                <h1 class="text-4xl md:text-6xl lg:text-7xl font-heading font-bold mb-6 leading-tight">
                    Professional <span class="text-transparent bg-clip-text bg-gradient-to-r from-white to-gray-300">Swing Trading</span> Room
                </h1>
                
                <p class="text-xl md:text-2xl text-white/90 mb-10 max-w-2xl mx-auto leading-relaxed">
                    Stop watching screens all day. Capture major market moves with high-probability setups, clear risk parameters, and active mentorship.
                </p>
                
                <div class="flex flex-wrap gap-4 justify-center items-center">
                    <a
                        href="#pricing"
                        class="bg-white text-rtp-indigo px-8 py-4 rounded-lg font-bold text-lg hover:bg-gray-100 transition-transform transform hover:-translate-y-1 shadow-xl focus:outline-none focus:ring-4 focus:ring-white/50"
                        aria-label="View pricing plans for swing trading"
                    >
                        Start Trading Now
                    </a>
                    <a
                        href="#methodology"
                        class="group bg-white/10 backdrop-blur-sm text-white border border-white/30 px-8 py-4 rounded-lg font-semibold hover:bg-white/20 transition-all focus:outline-none focus:ring-4 focus:ring-white/30"
                    >
                        How It Works
                        <span class="inline-block ml-2 transition-transform group-hover:translate-x-1">→</span>
                    </a>
                </div>
                <p class="mt-6 text-sm text-white/60">No long-term contracts. Cancel anytime.</p>
            </div>
        </div>
    </section>

    <section class="bg-rtp-surface border-b border-rtp-border py-12 relative z-20 -mt-4 mx-4 md:mx-auto md:max-w-6xl rounded-xl shadow-2xl">
        <div class="px-6 lg:px-8">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-8 divide-x-0 md:divide-x divide-rtp-border/50">
                <div class="text-center">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-1">65%*</div>
                    <div class="text-rtp-muted text-sm font-medium uppercase tracking-wider">Historical Win Rate</div>
                </div>
                <div class="text-center">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-1">3.2:1</div>
                    <div class="text-rtp-muted text-sm font-medium uppercase tracking-wider">Avg Risk/Reward</div>
                </div>
                <div class="text-center">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-primary mb-1">2-10</div>
                    <div class="text-rtp-muted text-sm font-medium uppercase tracking-wider">Days Per Trade</div>
                </div>
                <div class="text-center">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-blue mb-1">300+</div>
                    <div class="text-rtp-muted text-sm font-medium uppercase tracking-wider">Active Traders</div>
                </div>
            </div>
            <div class="mt-4 text-center">
                <p class="text-xs text-rtp-muted">*Past performance does not guarantee future results. See risk disclosure.</p>
            </div>
        </div>
    </section>

    <section id="features" class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
                    Why Traders Choose Our Room
                </h2>
                <p class="text-xl text-rtp-muted max-w-3xl mx-auto leading-relaxed">
                    We designed this service for traders who want professional alpha without the stress of staring at 1-minute charts all day.
                </p>
            </div>

            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                <div class="group bg-rtp-surface p-8 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-rtp-border hover:border-rtp-indigo/30">
                    <div class="w-14 h-14 bg-rtp-indigo/10 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-rtp-indigo/20 transition-colors">
                        <svg class="w-7 h-7 text-rtp-indigo" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">High-Probability Setups</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        We don't guess. We trade specific technical setups on the SPX and major indices with 2-10 day holding periods targeting 50%+ ROI per trade.
                    </p>
                </div>

                <div class="group bg-rtp-surface p-8 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-rtp-border hover:border-rtp-primary/30">
                    <div class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-rtp-primary/20 transition-colors">
                        <svg class="w-7 h-7 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Institutional Analysis</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Go beyond basic patterns. We analyze Dark Pool data, gamma exposure (GEX), and macro flow to align with institutional money.
                    </p>
                </div>

                <div class="group bg-rtp-surface p-8 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-rtp-border hover:border-rtp-emerald/30">
                    <div class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-rtp-emerald/20 transition-colors">
                        <svg class="w-7 h-7 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Real-Time Alerts</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Get instant notifications via Discord and mobile app for every entry, adjustment, and exit. Never miss a trade because of work.
                    </p>
                </div>

                <div class="group bg-rtp-surface p-8 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-rtp-border hover:border-rtp-blue/30">
                    <div class="w-14 h-14 bg-rtp-blue/10 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-rtp-blue/20 transition-colors">
                        <svg class="w-7 h-7 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Strict Risk Management</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Capital preservation is #1. Every alert comes with a predefined stop loss, position size, and max risk calculation.
                    </p>
                </div>

                <div class="group bg-rtp-surface p-8 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-rtp-border hover:border-rtp-indigo/30">
                    <div class="w-14 h-14 bg-rtp-indigo/10 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-rtp-indigo/20 transition-colors">
                        <svg class="w-7 h-7 text-rtp-indigo" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Community & Mentorship</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Join hundreds of serious traders. Ask questions, share charts, and get feedback from our lead analysts during market hours.
                    </p>
                </div>

                <div class="group bg-rtp-surface p-8 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-rtp-border hover:border-rtp-primary/30">
                    <div class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:bg-rtp-primary/20 transition-colors">
                        <svg class="w-7 h-7 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Job-Friendly Strategy</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Ideal for those with 9-5 commitments. Check alerts on your break, enter the trade, and let the system work for you.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <section id="methodology" class="py-24 bg-rtp-surface relative overflow-hidden">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
            <div class="grid lg:grid-cols-2 gap-16 items-center">
                <div class="space-y-10">
                    <div>
                        <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
                            Systematic Execution
                        </h2>
                        <p class="text-xl text-rtp-muted">How we find and execute winning trades consistently.</p>
                    </div>

                    <div class="space-y-8">
                        <div class="flex gap-6">
                            <div class="flex-shrink-0 w-12 h-12 bg-rtp-primary text-white rounded-full flex items-center justify-center font-bold text-lg shadow-lg shadow-rtp-primary/30">1</div>
                            <div>
                                <h3 class="text-xl font-heading font-bold text-rtp-text mb-2">Macro & Market Analysis</h3>
                                <p class="text-rtp-muted leading-relaxed">
                                    Every Sunday, we issue a "Week Ahead" report identifying key support/resistance zones, economic catalysts, and trend direction.
                                </p>
                            </div>
                        </div>

                        <div class="flex gap-6">
                            <div class="flex-shrink-0 w-12 h-12 bg-rtp-indigo text-white rounded-full flex items-center justify-center font-bold text-lg shadow-lg shadow-rtp-indigo/30">2</div>
                            <div>
                                <h3 class="text-xl font-heading font-bold text-rtp-text mb-2">The Setup & Entry</h3>
                                <p class="text-rtp-muted leading-relaxed">
                                    When our proprietary indicators trigger (oversold RSI + Key Level + Volume Spike), we issue a precise alert with Entry Price, Stop Loss, and Take Profit targets.
                                </p>
                            </div>
                        </div>

                        <div class="flex gap-6">
                            <div class="flex-shrink-0 w-12 h-12 bg-rtp-emerald text-white rounded-full flex items-center justify-center font-bold text-lg shadow-lg shadow-rtp-emerald/30">3</div>
                            <div>
                                <h3 class="text-xl font-heading font-bold text-rtp-text mb-2">Active Trade Management</h3>
                                <p class="text-rtp-muted leading-relaxed">
                                    We don't "set and forget." We issue updates to roll positions, trim profits at 25%/50%, or tighten stops to protect capital.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="relative">
                    <div class="absolute -inset-4 bg-gradient-to-r from-rtp-indigo to-rtp-primary rounded-2xl opacity-20 blur-xl"></div>
                    <div class="bg-rtp-bg border border-rtp-border p-8 rounded-2xl shadow-2xl relative">
                        <div class="mb-6 pb-6 border-b border-rtp-border flex justify-between items-center">
                            <span class="font-bold text-rtp-text">Recent Closed Trades</span>
                            <span class="text-xs font-mono text-rtp-muted bg-rtp-surface px-2 py-1 rounded">Verifiable Performance</span>
                        </div>
                        
                        <div class="space-y-4">
                            <div class="bg-rtp-surface p-4 rounded-xl border-l-4 border-rtp-emerald shadow-sm">
                                <div class="flex justify-between items-center mb-1">
                                    <span class="font-bold text-rtp-text">SPX Call Vertical</span>
                                    <span class="text-rtp-emerald font-bold">+45% Gain</span>
                                </div>
                                <div class="text-xs text-rtp-muted flex justify-between">
                                    <span>Held: 3 Days</span>
                                    <span>Outcome: Target Hit</span>
                                </div>
                            </div>

                            <div class="bg-rtp-surface p-4 rounded-xl border-l-4 border-rtp-emerald shadow-sm">
                                <div class="flex justify-between items-center mb-1">
                                    <span class="font-bold text-rtp-text">TSLA Put Sweep</span>
                                    <span class="text-rtp-emerald font-bold">+82% Gain</span>
                                </div>
                                <div class="text-xs text-rtp-muted flex justify-between">
                                    <span>Held: 5 Days</span>
                                    <span>Outcome: Runners Closed</span>
                                </div>
                            </div>

                            <div class="bg-rtp-surface p-4 rounded-xl border-l-4 border-rtp-blue shadow-sm">
                                <div class="flex justify-between items-center mb-1">
                                    <span class="font-bold text-rtp-text">AMZN Iron Condor</span>
                                    <span class="text-rtp-blue font-bold">+15% Gain</span>
                                </div>
                                <div class="text-xs text-rtp-muted flex justify-between">
                                    <span>Held: 7 Days</span>
                                    <span>Outcome: Expired Worthless (Full Profit)</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-12">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Simple, Transparent Pricing</h2>
                <p class="text-xl text-rtp-muted">Invest in your education. One winning trade often covers the membership.</p>
            </div>

            <div class="flex justify-center mb-12">
                <div class="bg-rtp-surface p-1.5 rounded-xl shadow-inner border border-rtp-border inline-flex relative" role="group" aria-label="Billing cycle selection">
                    {#each ['monthly', 'quarterly', 'annual'] as plan}
                        <button
                            on:click={() => (selectedPlan = plan as BillingCycle)}
                            class="relative px-6 py-2.5 rounded-lg text-sm font-bold capitalize transition-all duration-200 z-10 focus:outline-none focus:ring-2 focus:ring-rtp-indigo
                            {selectedPlan === plan ? 'text-white shadow-md' : 'text-rtp-muted hover:text-rtp-text'}"
                            aria-pressed={selectedPlan === plan}
                        >
                            {plan}
                            {#if selectedPlan === plan}
                                <div class="absolute inset-0 bg-rtp-indigo rounded-lg -z-10" transition:fade={{duration: 150}}></div>
                            {/if}
                        </button>
                    {/each}
                </div>
            </div>

            <div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto items-start">
                
                <div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border relative opacity-100 {selectedPlan !== 'monthly' ? 'hidden md:block md:opacity-50 md:scale-95' : 'scale-100 border-rtp-indigo/50'} transition-all duration-300">
                    <h3 class="text-xl font-bold text-rtp-text">Monthly</h3>
                    <div class="mt-4 flex items-baseline text-rtp-text">
                        <span class="text-5xl font-extrabold tracking-tight">$147</span>
                        <span class="ml-1 text-xl text-rtp-muted">/mo</span>
                    </div>
                    <p class="mt-4 text-rtp-muted text-sm">Flexible access for new members.</p>
                    <a href="/checkout/swing-monthly" class="mt-8 block w-full bg-rtp-surface border-2 border-rtp-indigo text-rtp-indigo hover:bg-rtp-indigo hover:text-white py-3 rounded-lg font-bold text-center transition-colors">Select Monthly</a>
                    <ul class="mt-8 space-y-4">
                        <li class="flex items-start text-rtp-muted"><span class="text-rtp-emerald mr-2">✓</span> Full Trading Room Access</li>
                        <li class="flex items-start text-rtp-muted"><span class="text-rtp-emerald mr-2">✓</span> Real-time App Alerts</li>
                        <li class="flex items-start text-rtp-muted"><span class="text-rtp-emerald mr-2">✓</span> Daily Market Briefing</li>
                    </ul>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border relative opacity-100 {selectedPlan !== 'quarterly' ? 'hidden md:block md:opacity-50 md:scale-95' : 'scale-100 border-rtp-indigo ring-2 ring-rtp-indigo/20'} transition-all duration-300">
                    <div class="absolute top-0 right-0 -mt-4 mr-4 bg-rtp-indigo text-white text-xs font-bold px-3 py-1 rounded-full shadow-lg">POPULAR</div>
                    <h3 class="text-xl font-bold text-rtp-text">Quarterly</h3>
                    <div class="mt-4 flex items-baseline text-rtp-text">
                        <span class="text-5xl font-extrabold tracking-tight">$353</span>
                        <span class="ml-1 text-xl text-rtp-muted">/qtr</span>
                    </div>
                    <p class="mt-4 text-rtp-emerald font-semibold text-sm">Save $88 per year</p>
                    <a href="/checkout/swing-quarterly" class="mt-8 block w-full bg-rtp-indigo text-white py-3 rounded-lg font-bold text-center hover:bg-indigo-700 transition-colors shadow-lg hover:shadow-xl">Select Quarterly</a>
                    <ul class="mt-8 space-y-4">
                        <li class="flex items-start text-rtp-text font-medium"><span class="text-rtp-emerald mr-2">✓</span> Everything in Monthly</li>
                        <li class="flex items-start text-rtp-text font-medium"><span class="text-rtp-emerald mr-2">✓</span> Priority Discord Support</li>
                        <li class="flex items-start text-rtp-text font-medium"><span class="text-rtp-emerald mr-2">✓</span> Bonus: Options 101 Course</li>
                    </ul>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 border-rtp-emerald relative opacity-100 {selectedPlan !== 'annual' ? 'hidden md:block md:opacity-50 md:scale-95' : 'scale-100 ring-4 ring-rtp-emerald/10'} transition-all duration-300">
                    <div class="absolute top-0 right-0 -mt-4 mr-4 bg-rtp-emerald text-white text-xs font-bold px-3 py-1 rounded-full shadow-lg">BEST VALUE</div>
                    <h3 class="text-xl font-bold text-rtp-text">Annual</h3>
                    <div class="mt-4 flex items-baseline text-rtp-text">
                        <span class="text-5xl font-extrabold tracking-tight">$1,147</span>
                        <span class="ml-1 text-xl text-rtp-muted">/yr</span>
                    </div>
                    <p class="mt-4 text-rtp-emerald font-semibold text-sm">Save $617 (That's 4 months free!)</p>
                    <a href="/checkout/swing-annual" class="mt-8 block w-full bg-rtp-emerald text-white py-3 rounded-lg font-bold text-center hover:bg-emerald-700 transition-colors shadow-lg hover:shadow-xl">Select Annual</a>
                    <ul class="mt-8 space-y-4">
                        <li class="flex items-start text-rtp-text font-medium"><span class="text-rtp-emerald mr-2">✓</span> VIP All-Access Pass</li>
                        <li class="flex items-start text-rtp-text font-medium"><span class="text-rtp-emerald mr-2">✓</span> 1-on-1 Strategy Session</li>
                        <li class="flex items-start text-rtp-text font-medium"><span class="text-rtp-emerald mr-2">✓</span> Exclusive Indicator Suite</li>
                    </ul>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl md:text-5xl font-heading font-bold text-center text-rtp-text mb-16">
                Real Members. Real Results.
            </h2>
            <div class="grid md:grid-cols-3 gap-8">
                <article class="bg-rtp-bg p-8 rounded-2xl border border-rtp-border shadow-sm">
                    <div class="flex items-center gap-1 mb-4" aria-label="5 out of 5 stars">
                        {#each Array(5) as _}
                            <svg class="w-5 h-5 text-yellow-400 fill-current" viewBox="0 0 20 20" aria-hidden="true"><path d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"/></svg>
                        {/each}
                    </div>
                    <blockquote class="text-rtp-text mb-6 italic leading-relaxed">
                        "I work in healthcare and can't look at charts. The mobile alerts are precise. I enter the trade on my phone, set the stop, and go back to work. Up 24% this month."
                    </blockquote>
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-rtp-indigo rounded-full flex items-center justify-center text-white font-bold">TC</div>
                        <div>
                            <cite class="font-semibold text-rtp-text not-italic">Tom C.</cite>
                            <div class="text-sm text-rtp-muted">Full-time Professional</div>
                        </div>
                    </div>
                </article>

                <article class="bg-rtp-bg p-8 rounded-2xl border border-rtp-border shadow-sm">
                    <div class="flex items-center gap-1 mb-4" aria-label="5 out of 5 stars">
                        {#each Array(5) as _}
                            <svg class="w-5 h-5 text-yellow-400 fill-current" viewBox="0 0 20 20" aria-hidden="true"><path d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"/></svg>
                        {/each}
                    </div>
                    <blockquote class="text-rtp-text mb-6 italic leading-relaxed">
                        "The risk management education is worth the price alone. I stopped blowing up accounts and started seeing steady growth. The community is super supportive."
                    </blockquote>
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-rtp-primary rounded-full flex items-center justify-center text-white font-bold">LH</div>
                        <div>
                            <cite class="font-semibold text-rtp-text not-italic">Lisa H.</cite>
                            <div class="text-sm text-rtp-muted">Swing Trader</div>
                        </div>
                    </div>
                </article>

                 <article class="bg-rtp-bg p-8 rounded-2xl border border-rtp-border shadow-sm">
                    <div class="flex items-center gap-1 mb-4" aria-label="5 out of 5 stars">
                        {#each Array(5) as _}
                            <svg class="w-5 h-5 text-yellow-400 fill-current" viewBox="0 0 20 20" aria-hidden="true"><path d="M10 15l-5.878 3.09 1.123-6.545L.489 6.91l6.572-.955L10 0l2.939 5.955 6.572.955-4.756 4.635 1.123 6.545z"/></svg>
                        {/each}
                    </div>
                    <blockquote class="text-rtp-text mb-6 italic leading-relaxed">
                        "Finally, a service that understands I can't day trade. The multi-day SPX spreads are brilliant. Low stress, high reward."
                    </blockquote>
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-rtp-emerald rounded-full flex items-center justify-center text-white font-bold">RK</div>
                        <div>
                            <cite class="font-semibold text-rtp-text not-italic">Robert K.</cite>
                            <div class="text-sm text-rtp-muted">Part-time Trader</div>
                        </div>
                    </div>
                </article>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">Frequently Asked Questions</h2>
            <div class="space-y-6">
                <details class="group bg-rtp-surface rounded-lg border border-rtp-border">
                    <summary class="flex justify-between items-center font-medium cursor-pointer list-none p-6">
                        <span class="text-rtp-text font-bold">What is the difference between this and day trading?</span>
                        <span class="transition group-open:rotate-180 text-rtp-indigo">
                            <svg fill="none" height="24" shape-rendering="geometricPrecision" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg>
                        </span>
                    </summary>
                    <div class="text-rtp-muted mt-0 px-6 pb-6 leading-relaxed">
                        Day trading involves opening and closing positions within the same day. Our swing trading room focuses on multi-day positions (usually 2 to 10 days) to capture larger moves without the need for constant monitoring.
                    </div>
                </details>
                
                <details class="group bg-rtp-surface rounded-lg border border-rtp-border">
                    <summary class="flex justify-between items-center font-medium cursor-pointer list-none p-6">
                        <span class="text-rtp-text font-bold">How much capital do I need to start?</span>
                        <span class="transition group-open:rotate-180 text-rtp-indigo">
                            <svg fill="none" height="24" shape-rendering="geometricPrecision" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg>
                        </span>
                    </summary>
                    <div class="text-rtp-muted mt-0 px-6 pb-6 leading-relaxed">
                        While we have members with large accounts, we recommend a starting balance of at least $2,000 to $5,000. This allows you to properly size your positions and manage risk according to our guidelines.
                    </div>
                </details>

                <details class="group bg-rtp-surface rounded-lg border border-rtp-border">
                    <summary class="flex justify-between items-center font-medium cursor-pointer list-none p-6">
                        <span class="text-rtp-text font-bold">Do you trade options or stocks?</span>
                        <span class="transition group-open:rotate-180 text-rtp-indigo">
                            <svg fill="none" height="24" shape-rendering="geometricPrecision" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg>
                        </span>
                    </summary>
                    <div class="text-rtp-muted mt-0 px-6 pb-6 leading-relaxed">
                        We primarily focus on SPX options (vertical spreads, iron condors) and large-cap tech stocks (TSLA, NVDA, AAPL). We focus on liquid assets that respect technical levels.
                    </div>
                </details>
            </div>
        </div>
    </section>

    <section class="py-24 bg-gradient-to-br from-rtp-indigo to-rtp-primary text-white text-center">
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl md:text-5xl font-heading font-bold mb-6">Ready to Master Swing Trading?</h2>
            <p class="text-xl text-white/90 mb-10 max-w-2xl mx-auto">
                Join the community today and get instant access to our next trade setup. 30-day money-back guarantee.
            </p>
            <a
                href="#pricing"
                class="inline-block bg-white text-rtp-indigo px-10 py-4 rounded-lg font-bold text-lg hover:bg-gray-100 transition-all shadow-xl hover:-translate-y-1"
            >
                Choose Your Plan
            </a>
        </div>
    </section>

    <footer class="bg-rtp-surface py-12 border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-xs text-rtp-muted leading-relaxed space-y-4">
                <p class="font-bold">RISK DISCLOSURE:</p>
                <p>
                    Trading futures, options, and stocks involves a substantial risk of loss and is not suitable for every investor. The valuation of futures, stocks, and options may fluctuate, and as a result, clients may lose more than their original investment. The impact of seasonal and geopolitical events is already factored into market prices.
                </p>
                <p>
                    Revolution Trading Pros is an educational platform and does not provide financial advice. All trade alerts and setups are for educational purposes only. Past performance (including the 65% win rate referenced on this page) is not necessarily indicative of future results. You are solely responsible for your trading decisions.
                </p>
                <p class="pt-4 border-t border-rtp-border/50">
                    &copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
                </p>
            </div>
        </div>
    </footer>
</main>