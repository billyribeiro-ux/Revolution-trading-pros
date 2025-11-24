<script lang="ts">
    import { fade, slide } from 'svelte/transition';

    // State for pricing interaction
    let selectedPlan: 'monthly' | 'quarterly' | 'annual' = 'monthly';

    // Structured Data (JSON-LD)
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
                "@type": "Service",
                "name": "Live SPX Day Trading Room",
                "provider": {
                    "@type": "Organization",
                    "name": "Revolution Trading Pros"
                },
                "serviceType": "Financial Education",
                "description": "Real-time 0DTE options trading mentorship, live screen sharing, and trade alerts."
            },
            {
                "@type": "Product",
                "name": "Day Trading Room Membership",
                "description": "Access to live trading room, real-time SPX alerts, and daily market analysis.",
                "offers": [
                    {
                        "@type": "Offer",
                        "name": "Monthly Membership",
                        "price": "197",
                        "priceCurrency": "USD",
                        "billingDuration": "P1M"
                    },
                    {
                        "@type": "Offer",
                        "name": "Quarterly Membership",
                        "price": "497",
                        "priceCurrency": "USD",
                        "billingDuration": "P3M"
                    }
                ]
            },
            {
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "Do I need a large account to day trade SPX?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "No. SPX options offer significant leverage. However, we strictly recommend having at least $2,000 to manage risk properly, though cash accounts under $25k can avoid PDT rules."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "What is 0DTE trading?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "0DTE stands for '0 Days To Expiration.' We trade options that expire the same day to capitalize on rapid intraday moves and Theta decay."
                        }
                    }
                ]
            }
        ]
    };

    const jsonLdString = JSON.stringify(schemaOrg);
</script>

<svelte:head>
    <title>Live Day Trading Room | SPX 0DTE Alerts & Mentorship | Revolution Trading Pros</title>
    <meta name="description" content="Trade live with professionals. Our SPX 0DTE Day Trading Room offers real-time screen sharing, precise entry/exit alerts, and institutional order flow analysis." />
    <meta name="keywords" content="live trading room, SPX 0DTE strategy, day trading alerts, options trading community, stock market mentorship" />
    <link rel="canonical" href="https://revolutiontradingpros.com/live-trading-rooms/day-trading" />

    <meta property="og:type" content="website" />
    <meta property="og:title" content="Live SPX Day Trading Room" />
    <meta property="og:description" content="Watch us trade live. Real-time alerts, screen sharing, and professional risk management." />
    <meta property="og:url" content="https://revolutiontradingpros.com/live-trading-rooms/day-trading" />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/day-trading-og.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Live SPX Day Trading Room" />
    <meta name="twitter:description" content="Stop trading alone. Join the Revolution Trading Pros live room today." />
    <meta name="twitter:image" content="https://revolutiontradingpros.com/images/day-trading-og.jpg" />

    {@html `<script type="application/ld+json">${jsonLdString}</script>`}
</svelte:head>

<main class="w-full overflow-x-hidden">
    <section class="relative bg-gradient-to-br from-gray-900 via-rtp-primary to-indigo-900 text-white py-24 md:py-32 overflow-hidden">
        <div class="absolute inset-0 opacity-20" style="background-image: radial-gradient(#ffffff 1px, transparent 1px); background-size: 30px 30px;"></div>
        <div class="absolute inset-0 bg-gradient-to-b from-transparent to-gray-900/90"></div>

        <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid lg:grid-cols-2 gap-12 items-center">
                <div>
                    <div class="inline-flex items-center gap-2 bg-white/5 backdrop-blur-md border border-white/10 px-4 py-2 rounded-full mb-6 shadow-[0_0_15px_rgba(52,211,153,0.3)]">
                        <span class="relative flex h-3 w-3">
                          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                          <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
                        </span>
                        <span class="text-sm font-semibold tracking-wide text-emerald-300">Market Open: LIVE</span>
                    </div>
                    
                    <h1 class="text-4xl md:text-6xl font-heading font-extrabold mb-6 leading-tight">
                        Master <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-emerald-400">0DTE Options</span> in Real-Time
                    </h1>
                    
                    <p class="text-xl text-gray-300 mb-8 max-w-xl leading-relaxed">
                        Don't just get alerts—watch the execution. Join our live voice & screen-share room where we hunt high-probability setups on SPX every single morning.
                    </p>
                    
                    <div class="flex flex-wrap gap-4">
                        <a href="#pricing" class="bg-rtp-primary text-white px-8 py-4 rounded-xl font-bold text-lg hover:bg-blue-600 transition-all shadow-[0_4px_14px_0_rgba(59,130,246,0.39)] hover:-translate-y-1">
                            Join the Live Room
                        </a>
                        <a href="#how-it-works" class="bg-white/5 backdrop-blur-md border border-white/20 text-white px-8 py-4 rounded-xl font-bold text-lg hover:bg-white/10 transition-all">
                            How It Works
                        </a>
                    </div>
                    
                    <div class="mt-8 flex items-center gap-4 text-sm text-gray-400">
                        <div class="flex -space-x-2">
                            <div class="w-8 h-8 rounded-full bg-gray-600 border-2 border-gray-900"></div>
                            <div class="w-8 h-8 rounded-full bg-gray-500 border-2 border-gray-900"></div>
                            <div class="w-8 h-8 rounded-full bg-gray-400 border-2 border-gray-900"></div>
                        </div>
                        <p>500+ Traders currently active</p>
                    </div>
                </div>

                <div class="relative hidden lg:block">
                    <div class="absolute -inset-1 bg-gradient-to-r from-blue-600 to-emerald-600 rounded-2xl blur opacity-30"></div>
                    <div class="relative bg-gray-800 rounded-2xl border border-gray-700 overflow-hidden shadow-2xl">
                        

[Image of multi-monitor trading desk setup with charts]

                        <div class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-gray-900 to-transparent h-24"></div>
                        <div class="absolute bottom-4 left-4 right-4 flex justify-between items-end">
                            <div>
                                <p class="text-xs text-emerald-400 font-mono mb-1">CURRENT STATUS</p>
                                <p class="text-white font-bold">Scanning for Gamma Levels...</p>
                            </div>
                            <div class="bg-gray-900/80 backdrop-blur px-3 py-1 rounded text-xs font-mono text-gray-300 border border-gray-700">
                                SPX: 4450.25
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="bg-rtp-surface border-y border-rtp-border py-12">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-8 divide-x-0 md:divide-x divide-rtp-border/50">
                <div class="text-center group">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-primary mb-2 group-hover:scale-110 transition-transform">500+</div>
                    <div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">Active Members</div>
                </div>
                <div class="text-center group">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-2 group-hover:scale-110 transition-transform">9:30AM</div>
                    <div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">Live Bell-to-Bell</div>
                </div>
                <div class="text-center group">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-2 group-hover:scale-110 transition-transform">1000+</div>
                    <div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">Setups Called</div>
                </div>
                <div class="text-center group">
                    <div class="text-3xl md:text-4xl font-bold text-rtp-blue mb-2 group-hover:scale-110 transition-transform">24/7</div>
                    <div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">Discord Access</div>
                </div>
            </div>
        </div>
    </section>

    <section id="features" class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">Inside The Room</h2>
                <p class="text-xl text-rtp-muted max-w-3xl mx-auto">
                    We don't just give signals. We teach you how to fish.
                </p>
            </div>

            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                <div class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-primary transition-all duration-300">
                    <div class="w-12 h-12 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6">
                        <svg class="w-6 h-6 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Live Screen Sharing</h3>
                    <p class="text-rtp-muted leading-relaxed mb-4">
                        Watch our charts in real-time. See exactly where we draw support, resistance, and supply zones before the trade happens.
                    </p>
                    

[Image of candlestick chart with technical indicators]

                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-emerald transition-all duration-300">
                    <div class="w-12 h-12 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6">
                        <svg class="w-6 h-6 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Instant Push Alerts</h3>
                    <p class="text-rtp-muted leading-relaxed mb-4">
                        Can't be at your desk? Get instant notifications via our mobile app for every Entry, Trim, and Exit.
                    </p>
                    

[Image of mobile phone showing trading alert notification]

                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-indigo transition-all duration-300">
                    <div class="w-12 h-12 bg-rtp-indigo/10 rounded-xl flex items-center justify-center mb-6">
                        <svg class="w-6 h-6 text-rtp-indigo" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Institutional Flow</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        We track Dark Pool data and Gamma Exposure (GEX) levels to understand where the "Smart Money" is positioning.
                    </p>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-blue transition-all duration-300">
                    <div class="w-12 h-12 bg-rtp-blue/10 rounded-xl flex items-center justify-center mb-6">
                        <svg class="w-6 h-6 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Professional Audio</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Voice commentary throughout the session. We talk through the trade psychology so you don't panic sell.
                    </p>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-primary transition-all duration-300">
                    <div class="w-12 h-12 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6">
                        <svg class="w-6 h-6 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Full Session Coverage</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        From the 9:30 AM Open to the 4:00 PM Close. Whether you trade the morning volatility or the afternoon trend, we are live.
                    </p>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-emerald transition-all duration-300">
                    <div class="w-12 h-12 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6">
                        <svg class="w-6 h-6 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Strict Risk Rules</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        We never risk more than 2% per trade. We emphasize capital preservation over gambling.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface relative">
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">A Typical Trading Day</h2>
                <p class="text-xl text-rtp-muted">How we structure the session for maximum efficiency.</p>
            </div>

            <div class="space-y-6 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-rtp-border before:to-transparent">
                
                <div class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active">
                    <div class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-primary bg-rtp-surface group-hover:bg-rtp-primary group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-primary transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    </div>
                    <div class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm">
                        <div class="flex items-center justify-between mb-2">
                            <span class="font-bold text-rtp-primary">8:30 AM EST</span>
                            <span class="text-xs bg-rtp-primary/10 text-rtp-primary px-2 py-0.5 rounded">Pre-Market</span>
                        </div>
                        <h3 class="font-bold text-rtp-text">The Game Plan</h3>
                        <p class="text-sm text-rtp-muted mt-2">We review overnight futures, economic data (CPI/FOMC), and mark key supply/demand zones on SPX/ES.</p>
                    </div>
                </div>

                <div class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group">
                    <div class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-emerald bg-rtp-surface group-hover:bg-rtp-emerald group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-emerald transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                    </div>
                    <div class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm">
                        <div class="flex items-center justify-between mb-2">
                            <span class="font-bold text-rtp-emerald">9:30 AM EST</span>
                            <span class="text-xs bg-rtp-emerald/10 text-rtp-emerald px-2 py-0.5 rounded">Market Open</span>
                        </div>
                        <h3 class="font-bold text-rtp-text">The Opening Bell</h3>
                        <p class="text-sm text-rtp-muted mt-2">High volatility execution. We look for the "Opening Range Breakout" or quick scalp opportunities. 100% focus.</p>
                    </div>
                </div>

                <div class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group">
                    <div class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-indigo bg-rtp-surface group-hover:bg-rtp-indigo group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-indigo transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"></path></svg>
                    </div>
                    <div class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm">
                        <div class="flex items-center justify-between mb-2">
                            <span class="font-bold text-rtp-indigo">11:00 AM - 2:00 PM</span>
                            <span class="text-xs bg-rtp-indigo/10 text-rtp-indigo px-2 py-0.5 rounded">Mid-Day</span>
                        </div>
                        <h3 class="font-bold text-rtp-text">Review & Education</h3>
                        <p class="text-sm text-rtp-muted mt-2">Volume slows. We review morning trades, answer member Q&A, and analyze macro trends. Less trading, more teaching.</p>
                    </div>
                </div>

                <div class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group">
                    <div class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-blue bg-rtp-surface group-hover:bg-rtp-blue group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-blue transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
                    </div>
                    <div class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm">
                        <div class="flex items-center justify-between mb-2">
                            <span class="font-bold text-rtp-blue">3:00 PM EST</span>
                            <span class="text-xs bg-rtp-blue/10 text-rtp-blue px-2 py-0.5 rounded">Power Hour</span>
                        </div>
                        <h3 class="font-bold text-rtp-text">The Close</h3>
                        <p class="text-sm text-rtp-muted mt-2">Institutional volume returns. We look for EOD (End of Day) moves or hedge existing positions before the bell.</p>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Membership Plans</h2>
                <p class="text-xl text-rtp-muted max-w-2xl mx-auto">One winning trade often pays for the entire month.</p>
            </div>

            <div class="flex justify-center mb-12">
                <div class="bg-rtp-surface p-1.5 rounded-xl shadow-inner border border-rtp-border inline-flex gap-1" role="tablist" aria-label="Subscription Duration">
                    {#each ['monthly', 'quarterly', 'annual'] as plan}
                        <button
                            on:click={() => (selectedPlan = plan as 'monthly' | 'quarterly' | 'annual')}
                            class="relative px-6 py-2.5 rounded-lg text-sm font-bold capitalize transition-all duration-200 z-10 focus:outline-none focus:ring-2 focus:ring-rtp-primary
                            {selectedPlan === plan ? 'text-white shadow-md' : 'text-rtp-muted hover:text-rtp-text'}"
                            role="tab"
                            aria-selected={selectedPlan === plan}
                            aria-controls="pricing-panel"
                        >
                            {plan}
                            {#if selectedPlan === plan}
                                <div class="absolute inset-0 bg-rtp-primary rounded-lg -z-10" transition:fade={{duration: 150}}></div>
                            {/if}
                        </button>
                    {/each}
                </div>
            </div>

            <div id="pricing-panel" class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
                <div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 {selectedPlan === 'monthly' ? 'border-rtp-primary scale-100 opacity-100' : 'border-rtp-border opacity-50 scale-95 md:block hidden'} transition-all duration-300">
                    <h3 class="text-xl font-bold text-rtp-text">Monthly</h3>
                    <div class="mt-4 flex items-baseline text-rtp-text">
                        <span class="text-5xl font-extrabold tracking-tight">$197</span>
                        <span class="ml-1 text-xl text-rtp-muted">/mo</span>
                    </div>
                    <ul class="mt-8 space-y-4 text-rtp-muted text-sm">
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Live Voice & Screen</li>
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Real-Time App Alerts</li>
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Discord Community</li>
                    </ul>
                    <button class="w-full mt-8 bg-rtp-surface border-2 border-rtp-primary text-rtp-primary py-3 rounded-lg font-bold hover:bg-rtp-primary hover:text-white transition-colors">Select Monthly</button>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-xl border-2 {selectedPlan === 'quarterly' ? 'border-rtp-primary ring-4 ring-rtp-primary/10 scale-105 z-10 opacity-100' : 'border-rtp-border opacity-50 scale-95 md:block hidden'} transition-all duration-300 relative">
                    <div class="absolute top-0 right-0 -mt-3 mr-3 bg-rtp-emerald text-white text-xs font-bold px-3 py-1 rounded-full">MOST POPULAR</div>
                    <h3 class="text-xl font-bold text-rtp-text">Quarterly</h3>
                    <div class="mt-4 flex items-baseline text-rtp-text">
                        <span class="text-5xl font-extrabold tracking-tight">$497</span>
                        <span class="ml-1 text-xl text-rtp-muted">/qtr</span>
                    </div>
                    <p class="text-xs text-rtp-emerald font-bold mt-2">Save $94 per year</p>
                    <ul class="mt-8 space-y-4 text-rtp-text text-sm font-medium">
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Everything in Monthly</li>
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Priority Support</li>
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Quarterly Strategy Session</li>
                    </ul>
                    <button class="w-full mt-8 bg-rtp-primary text-white py-3 rounded-lg font-bold hover:bg-blue-600 transition-colors shadow-lg">Select Quarterly</button>
                </div>

                <div class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 {selectedPlan === 'annual' ? 'border-rtp-primary scale-100 opacity-100' : 'border-rtp-border opacity-50 scale-95 md:block hidden'} transition-all duration-300">
                    <h3 class="text-xl font-bold text-rtp-text">Annual</h3>
                    <div class="mt-4 flex items-baseline text-rtp-text">
                        <span class="text-5xl font-extrabold tracking-tight">$1,647</span>
                        <span class="ml-1 text-xl text-rtp-muted">/yr</span>
                    </div>
                    <p class="text-xs text-rtp-emerald font-bold mt-2">Save $717 (Best Value)</p>
                    <ul class="mt-8 space-y-4 text-rtp-muted text-sm">
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> All Quarterly Features</li>
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> 1-on-1 Mentorship Call</li>
                        <li class="flex items-center gap-3"><span class="text-rtp-emerald">✓</span> Custom Indicator Suite</li>
                    </ul>
                    <button class="w-full mt-8 bg-rtp-surface border-2 border-rtp-emerald text-rtp-emerald py-3 rounded-lg font-bold hover:bg-rtp-emerald hover:text-white transition-colors">Select Annual</button>
                </div>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">Common Questions</h2>
            <div class="space-y-4">
                <details class="group bg-rtp-bg rounded-lg border border-rtp-border">
                    <summary class="flex justify-between items-center font-medium cursor-pointer list-none p-6 text-rtp-text">
                        <span>Do I need $25,000 to join?</span>
                        <span class="transition group-open:rotate-180 text-rtp-primary"><svg fill="none" height="24" shape-rendering="geometricPrecision" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg></span>
                    </summary>
                    <div class="text-rtp-muted mt-0 px-6 pb-6 leading-relaxed">
                        No. While the PDT rule applies to margin accounts under $25k, many of our members use Cash Accounts (options settle T+1) to trade every day without restriction.
                    </div>
                </details>
                <details class="group bg-rtp-bg rounded-lg border border-rtp-border">
                    <summary class="flex justify-between items-center font-medium cursor-pointer list-none p-6 text-rtp-text">
                        <span>What platform do you use?</span>
                        <span class="transition group-open:rotate-180 text-rtp-primary"><svg fill="none" height="24" shape-rendering="geometricPrecision" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg></span>
                    </summary>
                    <div class="text-rtp-muted mt-0 px-6 pb-6 leading-relaxed">
                        We use Discord for our live stage, voice chat, and text alerts. Screen sharing is high-definition and low latency. You can watch on mobile or desktop.
                    </div>
                </details>
                <details class="group bg-rtp-bg rounded-lg border border-rtp-border">
                    <summary class="flex justify-between items-center font-medium cursor-pointer list-none p-6 text-rtp-text">
                        <span>Is this for beginners?</span>
                        <span class="transition group-open:rotate-180 text-rtp-primary"><svg fill="none" height="24" shape-rendering="geometricPrecision" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg></span>
                    </summary>
                    <div class="text-rtp-muted mt-0 px-6 pb-6 leading-relaxed">
                        While Day Trading SPX is an advanced strategy, our room is educational. We explain every setup. We recommend beginners start by Paper Trading (Sim) our alerts until confident.
                    </div>
                </details>
            </div>
        </div>
    </section>

    <section class="py-24 bg-gradient-to-br from-blue-600 to-indigo-900 text-white text-center">
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl md:text-5xl font-heading font-bold mb-6">Stop Trading Alone</h2>
            <p class="text-xl text-white/90 mb-10 max-w-2xl mx-auto">
                Join 500+ active traders inside the room right now. Your first month is backed by our 30-day money-back guarantee.
            </p>
            <a href="#pricing" class="inline-block bg-white text-blue-900 px-10 py-4 rounded-xl font-bold text-lg hover:bg-gray-100 transition-all shadow-xl hover:-translate-y-1">
                Start Your Trial
            </a>
        </div>
    </section>

    <footer class="bg-rtp-surface py-12 border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-xs text-rtp-muted leading-relaxed space-y-4">
                <p class="font-bold">RISK DISCLOSURE:</p>
                <p>
                    Day trading options, specifically 0DTE (Zero Days to Expiration), involves a high degree of risk and is not suitable for all investors. You can lose more than your initial investment. Revolution Trading Pros is an educational platform, not a registered investment advisor.
                </p>
                <p>
                    Past performance in the trading room is not indicative of future results. Hypothetical or simulated performance results have certain limitations. Unlike an actual performance record, simulated results do not represent actual trading.
                </p>
                <p class="pt-4 border-t border-rtp-border/50">
                    &copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
                </p>
            </div>
        </div>
    </footer>
</main>