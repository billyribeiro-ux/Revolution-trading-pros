<script lang="ts">
    import { fade, slide } from 'svelte/transition';

    // FAQ Logic
    let openFaq: number | null = null;
    const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

    // --- SEO SCHEMA (JSON-LD) ---
    const productSchema = {
        "@context": "https://schema.org",
        "@type": "Product",
        "name": "SPX Profit Pulse - 0DTE Options Alerts",
        "description": "Professional SPX 0DTE options alerts delivered via SMS and Discord. Real-time entries, exits, and risk management.",
        "brand": {
            "@type": "Organization",
            "name": "Revolution Trading Pros"
        },
        "offers": {
            "@type": "AggregateOffer",
            "priceCurrency": "USD",
            "lowPrice": "65", // Effective monthly price of annual
            "highPrice": "97",
            "offerCount": "3",
            "offers": [
                {
                    "@type": "Offer",
                    "name": "Monthly Plan",
                    "price": "97",
                    "priceCurrency": "USD",
                    "priceSpecification": {
                        "@type": "UnitPriceSpecification",
                        "price": "97",
                        "priceCurrency": "USD",
                        "referenceQuantity": { "@type": "QuantitativeValue", "value": "1", "unitCode": "MON" }
                    }
                },
                {
                    "@type": "Offer",
                    "name": "Annual Plan",
                    "price": "777",
                    "priceCurrency": "USD"
                }
            ]
        }
    };

    const faqSchema = {
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": [
            {
                "@type": "Question",
                "name": "What is SPX 0DTE?",
                "acceptedAnswer": { "@type": "Answer", "text": "SPX 0DTE refers to 'Zero Days to Expiration' options on the S&P 500 index. These contracts expire the same day they are traded, offering high potential returns due to rapid gamma exposure." }
            },
            {
                "@type": "Question",
                "name": "How fast are the alerts?",
                "acceptedAnswer": { "@type": "Answer", "text": "Our alerts are sent instantly via SMS text message and Discord webhooks. The average latency is under 5 seconds from the moment our trader executes the trade." }
            },
            {
                "@type": "Question",
                "name": "What account size do I need?",
                "acceptedAnswer": { "@type": "Answer", "text": "Since we trade SPX options, premiums can range from $2.00 to $10.00 ($200-$1,000 per contract). We recommend a starting account of at least $2,000 to manage risk properly." }
            }
        ]
    };

    const jsonLd = JSON.stringify({ "@graph": [productSchema, faqSchema] });
</script>

<svelte:head>
    <title>SPX Profit Pulse | #1 0DTE Options Alerts | Revolution Trading Pros</title>
    <meta name="description" content="Trade SPX 0DTE options with confidence. Get real-time SMS alerts, precise entries/exits, and professional risk management. Join 1,000+ traders." />
    <meta name="keywords" content="SPX 0DTE alerts, options trading signals, SPX signals, 0DTE trading strategy, same day expiration options" />
    <link rel="canonical" href="https://revolutiontradingpros.com/alert-services/spx-profit-pulse" />

    <meta property="og:type" content="product" />
    <meta property="og:title" content="SPX Profit Pulse | Real-Time 0DTE Alerts" />
    <meta property="og:description" content="Instant SMS alerts for high-probability SPX setups. 78% win rate." />
    <meta property="og:url" content="https://revolutiontradingpros.com/alert-services/spx-profit-pulse" />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/og-spx-pulse.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="SPX Profit Pulse Alerts" />
    <meta name="twitter:description" content="Instant SMS alerts for high-probability SPX setups. 78% win rate." />
    <meta name="twitter:image" content="https://revolutiontradingpros.com/images/og-spx-pulse.jpg" />

    <script type="application/ld+json">
        {@html jsonLd}
    </script>
</svelte:head>

<main class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text">

    <section class="relative bg-gradient-to-br from-rtp-primary via-rtp-blue to-rtp-indigo text-white py-24 md:py-32 overflow-hidden">
        <div class="absolute inset-0 bg-black/30 z-0"></div>
        <div class="absolute top-0 right-0 w-[500px] h-[500px] bg-blue-500/20 rounded-full blur-[100px] animate-pulse"></div>
        <div class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-indigo-500/20 rounded-full blur-[100px]"></div>
        <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10 z-0 mix-blend-overlay"></div>

        <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center max-w-4xl mx-auto">
                <div class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-md border border-white/20 px-4 py-2 rounded-full mb-8 shadow-lg">
                    <span class="relative flex h-3 w-3">
                      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                      <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
                    </span>
                    <span class="text-sm font-bold tracking-wide">Live SMS & Discord Alerts</span>
                </div>
                
                <h1 class="text-5xl md:text-7xl font-heading font-extrabold mb-6 leading-tight drop-shadow-xl">
                    Master the <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-300 to-teal-200">SPX 0DTE</span>
                </h1>
                
                <p class="text-xl md:text-2xl text-blue-50 mb-10 max-w-2xl mx-auto leading-relaxed">
                    Professional S&P 500 options alerts delivered instantly to your phone. Capture daily volatility with defined risk.
                </p>
                
                <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                    <a
                        href="#pricing"
                        class="w-full sm:w-auto bg-white text-rtp-primary px-8 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all hover:-translate-y-1 shadow-xl hover:shadow-2xl"
                    >
                        Start Your Trial
                    </a>
                    <a
                        href="#performance"
                        class="w-full sm:w-auto bg-white/10 backdrop-blur-sm text-white border border-white/30 px-8 py-4 rounded-xl font-bold text-lg hover:bg-white/20 transition-all"
                    >
                        View Track Record
                    </a>
                </div>
            </div>
        </div>
    </section>

    <section class="bg-rtp-surface border-b border-rtp-border py-12 relative z-20 shadow-sm">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <dl class="grid grid-cols-2 md:grid-cols-4 gap-8 divide-x divide-rtp-border/30">
                <div class="text-center px-4">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">Win Rate</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-emerald">78%</dd>
                </div>
                <div class="text-center px-4 border-l-0">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">Alert Speed</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-primary">&lt;30s</dd>
                </div>
                <div class="text-center px-4 border-l-0">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">Daily Alerts</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-indigo">3-5</dd>
                </div>
                <div class="text-center px-4 border-l-0">
                    <dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">Active Traders</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-rtp-blue">1k+</dd>
                </div>
            </dl>
        </div>
    </section>

    <section class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-20">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
                    Institutional-Grade Signals
                </h2>
                <p class="text-xl text-rtp-muted max-w-3xl mx-auto">
                    We don't just guess. We use flow, volume, and technicals to snipe high-probability setups.
                </p>
            </div>

            <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border/50 hover:border-rtp-primary/50 transition-all hover:-translate-y-1 group">
                    <div class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:rotate-6 transition-transform">
                        <svg class="w-7 h-7 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Instant SMS Alerts</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Don't be glued to Discord. Get alerts sent straight to your phone so you can trade on the go from anywhere.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border/50 hover:border-rtp-indigo/50 transition-all hover:-translate-y-1 group">
                    <div class="w-14 h-14 bg-rtp-indigo/10 rounded-2xl flex items-center justify-center mb-6 group-hover:rotate-6 transition-transform">
                        <svg class="w-7 h-7 text-rtp-indigo" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Private Discord</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Join the war room. Get detailed charts, live market commentary, and reasoning behind every trade in our premium channels.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border/50 hover:border-rtp-emerald/50 transition-all hover:-translate-y-1 group">
                    <div class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:rotate-6 transition-transform">
                        <svg class="w-7 h-7 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Precise Entries</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        No vague signals. You get the exact Strike, Expiration, and Limit Price. "Buy SPX 4580 Call at $3.50."
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border/50 hover:border-rtp-blue/50 transition-all hover:-translate-y-1 group">
                    <div class="w-14 h-14 bg-rtp-blue/10 rounded-2xl flex items-center justify-center mb-6 group-hover:rotate-6 transition-transform">
                        <svg class="w-7 h-7 text-rtp-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Scaling Out</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        We lock in gains along the way. Alerts include profit targets at 25%, 50%, 75% and runners to maximize ROI.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border/50 hover:border-rtp-primary/50 transition-all hover:-translate-y-1 group">
                    <div class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:rotate-6 transition-transform">
                        <svg class="w-7 h-7 text-rtp-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Hard Stops</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        We hate big losses. Every trade has a defined invalidation level to protect your capital. Live to trade another day.
                    </p>
                </article>

                <article class="bg-rtp-surface p-8 rounded-2xl shadow-lg border border-rtp-border/50 hover:border-rtp-emerald/50 transition-all hover:-translate-y-1 group">
                    <div class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:rotate-6 transition-transform">
                        <svg class="w-7 h-7 text-rtp-emerald" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
                    </div>
                    <h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Live Updates</h3>
                    <p class="text-rtp-muted leading-relaxed">
                        Markets change fast. We send real-time updates to roll strikes, cut early, or hold longer based on price action.
                    </p>
                </article>
            </div>
        </div>
    </section>

    <section class="py-24 bg-rtp-surface border-y border-rtp-border">
        <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Inside The Alerts</h2>
                <p class="text-xl text-rtp-muted">Crystal clear communication. No confusion.</p>
            </div>

            <div class="space-y-6">
                <div class="bg-rtp-bg p-8 rounded-2xl border-l-4 border-rtp-emerald shadow-lg relative overflow-hidden">
                    <div class="absolute top-0 right-0 p-4 opacity-10">
                        <svg class="w-24 h-24 text-rtp-emerald" fill="currentColor" viewBox="0 0 20 20"><path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z"/></svg>
                    </div>
                    <div class="flex items-center gap-4 mb-4 relative z-10">
                        <span class="bg-emerald-100 text-emerald-800 text-xs font-bold px-3 py-1 rounded-full uppercase">Entry Signal</span>
                        <span class="text-rtp-muted text-xs font-mono">09:45 AM EST</span>
                    </div>
                    <div class="font-mono text-sm md:text-base relative z-10">
                        <div class="text-rtp-text font-bold text-lg mb-2">🚀 SPX 0DTE CALL ENTRY</div>
                        <div class="grid md:grid-cols-2 gap-4">
                            <div>
                                <span class="text-rtp-muted block">Strike</span>
                                <span class="font-bold text-rtp-text">4,580</span>
                            </div>
                            <div>
                                <span class="text-rtp-muted block">Premium Limit</span>
                                <span class="font-bold text-rtp-text">$3.50</span>
                            </div>
                            <div>
                                <span class="text-rtp-muted block">Stop Loss</span>
                                <span class="font-bold text-red-500">$2.10 (Hard Stop)</span>
                            </div>
                            <div>
                                <span class="text-rtp-muted block">Targets</span>
                                <span class="font-bold text-emerald-600">$4.50, $5.25, $7.00+</span>
                            </div>
                        </div>
                        <div class="mt-4 pt-4 border-t border-rtp-border text-rtp-muted italic text-sm">
                            "Reasoning: SPX reclaiming VWAP with increasing bullish volume. 4575 gamma wall held as support."
                        </div>
                    </div>
                </div>

                <div class="bg-rtp-bg p-8 rounded-2xl border-l-4 border-rtp-blue shadow-lg ml-0 md:ml-12">
                     <div class="flex items-center gap-4 mb-4">
                        <span class="bg-blue-100 text-blue-800 text-xs font-bold px-3 py-1 rounded-full uppercase">Update</span>
                        <span class="text-rtp-muted text-xs font-mono">10:15 AM EST</span>
                    </div>
                    <div class="font-mono text-sm md:text-base">
                        <div class="text-rtp-text font-bold mb-2">⚡ TARGET 2 HIT</div>
                        <p class="text-rtp-muted mb-2">Taking partials at $5.25 (+50%).</p>
                        <p class="text-rtp-text">Move stop loss on runners to Breakeven ($3.50). Risk free trade now.</p>
                    </div>
                </div>

                <div class="bg-rtp-bg p-8 rounded-2xl border-l-4 border-rtp-primary shadow-lg">
                     <div class="flex items-center gap-4 mb-4">
                        <span class="bg-indigo-100 text-indigo-800 text-xs font-bold px-3 py-1 rounded-full uppercase">Exit</span>
                        <span class="text-rtp-muted text-xs font-mono">11:30 AM EST</span>
                    </div>
                    <div class="font-mono text-sm md:text-base">
                        <div class="text-rtp-text font-bold mb-2">💰 FINAL EXIT</div>
                        <p class="text-rtp-muted mb-2">Closed remaining runners at $7.10.</p>
                        <p class="text-emerald-600 font-bold text-lg">Total Profit: +103% 🎯</p>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="performance" class="py-24 bg-rtp-bg">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
                    Verified Performance
                </h2>
                <p class="text-xl text-rtp-muted">Last 30 days of trading activity.</p>
            </div>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-6 mb-12">
                {#each [
                    { label: 'Win Rate', val: '78%', color: 'text-rtp-emerald' },
                    { label: 'Avg Winner', val: '+42%', color: 'text-rtp-primary' },
                    { label: 'Avg Loser', val: '-18%', color: 'text-rtp-indigo' },
                    { label: 'Risk/Reward', val: '2.3:1', color: 'text-rtp-blue' }
                ] as metric}
                    <div class="bg-rtp-surface p-6 rounded-xl text-center border border-rtp-border shadow-sm">
                        <div class="text-3xl md:text-4xl font-bold {metric.color} mb-2">{metric.val}</div>
                        <div class="text-rtp-muted font-medium text-sm uppercase">{metric.label}</div>
                    </div>
                {/each}
            </div>

            <div class="bg-rtp-surface rounded-2xl border border-rtp-border overflow-hidden shadow-lg">
                <div class="p-6 border-b border-rtp-border bg-rtp-surface/50">
                    <h3 class="font-bold text-rtp-text">Recent Trade Log</h3>
                </div>
                <div class="divide-y divide-rtp-border">
                    <div class="flex items-center justify-between p-5 hover:bg-rtp-bg transition-colors">
                        <div>
                            <div class="font-bold text-rtp-text">SPX 4,560 Call</div>
                            <div class="text-xs text-rtp-muted font-mono mt-1">Entry: $4.20 → Exit: $6.30</div>
                        </div>
                        <div class="text-right">
                            <span class="block text-emerald-500 font-bold">+50%</span>
                            <span class="text-xs text-rtp-muted">Nov 15</span>
                        </div>
                    </div>
                    <div class="flex items-center justify-between p-5 hover:bg-rtp-bg transition-colors">
                        <div>
                            <div class="font-bold text-rtp-text">SPX 4,575 Put</div>
                            <div class="text-xs text-rtp-muted font-mono mt-1">Entry: $3.80 → Exit: $5.70</div>
                        </div>
                        <div class="text-right">
                            <span class="block text-emerald-500 font-bold">+50%</span>
                            <span class="text-xs text-rtp-muted">Nov 14</span>
                        </div>
                    </div>
                    <div class="flex items-center justify-between p-5 hover:bg-rtp-bg transition-colors bg-red-50/5">
                        <div>
                            <div class="font-bold text-rtp-text">SPX 4,590 Call</div>
                            <div class="text-xs text-rtp-muted font-mono mt-1">Entry: $2.90 → Exit: $2.00</div>
                        </div>
                        <div class="text-right">
                            <span class="block text-red-500 font-bold">-31%</span>
                            <span class="text-xs text-rtp-muted">Nov 13</span>
                        </div>
                    </div>
                     <div class="flex items-center justify-between p-5 hover:bg-rtp-bg transition-colors">
                        <div>
                            <div class="font-bold text-rtp-text">SPX 4,555 Put</div>
                            <div class="text-xs text-rtp-muted font-mono mt-1">Entry: $5.10 → Exit: $8.15</div>
                        </div>
                        <div class="text-right">
                            <span class="block text-emerald-500 font-bold">+60%</span>
                            <span class="text-xs text-rtp-muted">Nov 12</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-rtp-surface border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">Simple, Transparent Pricing</h2>
                <p class="text-xl text-rtp-muted">No hidden fees. Cancel anytime.</p>
            </div>

            <div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto items-center">
                
                <div class="bg-rtp-bg p-8 rounded-2xl shadow-lg border border-rtp-border hover:border-rtp-primary/50 transition-all h-full flex flex-col">
                    <div class="mb-8 text-center">
                        <h3 class="text-xl font-bold text-rtp-text mb-2">Monthly</h3>
                        <div class="text-4xl font-extrabold text-rtp-primary">$97<span class="text-lg font-medium text-rtp-muted">/mo</span></div>
                    </div>
                    <ul class="space-y-4 mb-8 flex-grow text-sm">
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Instant SMS Alerts</li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Discord Access</li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> 3-5 Signals / Day</li>
                    </ul>
                    <a href="/checkout/monthly" class="block w-full py-3 px-4 bg-rtp-surface border border-rtp-primary text-rtp-primary font-bold rounded-lg text-center hover:bg-rtp-primary hover:text-white transition-colors">Choose Monthly</a>
                </div>

                <div class="bg-rtp-bg p-8 rounded-2xl shadow-2xl border-2 border-rtp-primary transform md:scale-105 relative z-10 h-full flex flex-col">
                    <div class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-primary text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide shadow-md">Most Popular</div>
                    <div class="mb-8 text-center">
                        <h3 class="text-xl font-bold text-rtp-text mb-2">Quarterly</h3>
                        <div class="text-5xl font-extrabold text-rtp-primary">$247<span class="text-lg font-medium text-rtp-muted">/qtr</span></div>
                        <p class="text-xs text-emerald-600 font-bold mt-2">Save $45</p>
                    </div>
                    <ul class="space-y-4 mb-8 flex-grow text-sm">
                        <li class="flex gap-3"><span class="text-emerald-500 font-bold">✓</span> <span class="font-bold">Priority Support</span></li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Instant SMS Alerts</li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Discord Access</li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> All Signal Types</li>
                    </ul>
                    <a href="/checkout/quarterly" class="block w-full py-4 px-6 bg-rtp-primary text-white font-bold rounded-lg text-center hover:bg-blue-600 transition-colors shadow-lg">Choose Quarterly</a>
                </div>

                <div class="bg-rtp-bg p-8 rounded-2xl shadow-lg border border-rtp-emerald hover:border-emerald-500 transition-all h-full flex flex-col">
                    <div class="mb-8 text-center">
                        <h3 class="text-xl font-bold text-rtp-text mb-2">Annual</h3>
                        <div class="text-4xl font-extrabold text-rtp-emerald">$777<span class="text-lg font-medium text-rtp-muted">/yr</span></div>
                        <p class="text-xs text-emerald-600 font-bold mt-2">Save $387 (33%)</p>
                    </div>
                    <ul class="space-y-4 mb-8 flex-grow text-sm">
                         <li class="flex gap-3"><span class="text-emerald-500 font-bold">✓</span> <span class="font-bold">Exclusive Indicators</span></li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Priority Support</li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Instant SMS Alerts</li>
                        <li class="flex gap-3"><span class="text-emerald-500">✓</span> Discord Access</li>
                    </ul>
                    <a href="/checkout/annual" class="block w-full py-3 px-4 bg-rtp-emerald text-white font-bold rounded-lg text-center hover:bg-emerald-600 transition-colors shadow-md">Choose Annual</a>
                </div>

            </div>
        </div>
    </section>

    <section class="py-20 bg-rtp-bg">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center mb-12">Frequently Asked Questions</h2>
            <div class="space-y-4">
                {#each faqSchema.mainEntity as faq, i}
                    <div class="border border-rtp-border rounded-xl bg-rtp-surface overflow-hidden">
                        <button 
                            class="w-full text-left px-6 py-4 font-bold flex justify-between items-center focus:outline-none hover:bg-rtp-bg/50"
                            on:click={() => toggleFaq(i)}
                        >
                            {faq.name}
                            <svg class="w-5 h-5 transform transition-transform {openFaq === i ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
                        </button>
                        {#if openFaq === i}
                            <div transition:slide class="px-6 pb-4 text-rtp-muted text-sm leading-relaxed">
                                {faq.acceptedAnswer.text}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="py-24 bg-gradient-to-br from-rtp-primary to-rtp-indigo text-white relative overflow-hidden">
        <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
            <h2 class="text-4xl md:text-5xl font-heading font-extrabold mb-6">Stop Guessing. Start Profiting.</h2>
            <p class="text-xl text-white/90 mb-10 max-w-2xl mx-auto">
                Join 1,000+ traders receiving professional SPX 0DTE alerts today.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                <a
                    href="#pricing"
                    class="bg-white text-rtp-primary px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-xl hover:-translate-y-1"
                >
                    Get Access Now
                </a>
                <a
                    href="/resources"
                    class="bg-white/10 backdrop-blur-sm text-white border border-white/30 px-10 py-4 rounded-xl font-bold text-lg hover:bg-white/20 transition-all"
                >
                    Learn More
                </a>
            </div>
            <p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee • Secure Checkout</p>
        </div>
    </section>

    <footer class="bg-rtp-bg py-12 border-t border-rtp-border">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-rtp-muted/80 leading-relaxed">
            <p class="font-bold mb-2">RISK DISCLOSURE</p>
            <p class="max-w-4xl mx-auto">
                Trading futures and options involves substantial risk of loss and is not suitable for every investor. The valuation of futures and options may fluctuate, and as a result, clients may lose more than their original investment. The highly leveraged nature of futures trading means that small market movements will have a great impact on your trading account and this can work against you, leading to large losses or can work for you, leading to large gains. Revolution Trading Pros is an educational platform and not a financial advisory service.
            </p>
            <p class="mt-6">&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.</p>
        </div>
    </footer>
</main>