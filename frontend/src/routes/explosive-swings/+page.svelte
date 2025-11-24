<script lang="ts">
    import { onMount } from 'svelte';
    import { gsap } from 'gsap';
    import {
        IconTrendingUp,
        IconFlame,
        IconBellRinging,
        IconShoppingCart,
        IconCheck,
        IconBolt,
        IconChartCandle,
        IconQuestionMark
    } from '@tabler/icons-svelte';
    import { addItemToCart } from '$lib/utils/cart-helpers';

    // --- State & Refs ---
    let headerRef: HTMLElement;
    let contentRef: HTMLElement;
    let pricingRef: HTMLElement;

    // --- Pricing Configuration ---
    // specific types ensuring safety for the cart function
    type BillingInterval = 'monthly' | 'quarterly' | 'yearly';

    interface PricingTier {
        id: BillingInterval;
        price: number;
        displayPrice: string;
        period: string;
        savings?: string;
        isBestValue?: boolean;
    }

    const pricingTiers: PricingTier[] = [
        { id: 'monthly', price: 199, displayPrice: '199', period: '/mo' },
        { id: 'yearly', price: 1999, displayPrice: '1,999', period: '/yr', savings: 'Save $389/yr', isBestValue: true },
        { id: 'quarterly', price: 549, displayPrice: '549', period: '/qtr', savings: 'Save $48/yr' }
    ];

    // --- SEO: Structured Data (JSON-LD) ---
    // Includes Product (Offers) and FAQPage for rich snippets
    const schemaOrg = {
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Product",
                "name": "Explosive Swings Trading Alerts",
                "description": "Professional momentum swing trading alerts. Real-time buy/sell signals with 3-10 day hold times.",
                "image": "https://revolutiontradingpros.com/images/explosive-swings-og.jpg",
                "brand": {
                    "@type": "Brand",
                    "name": "Revolution Trading Pros"
                },
                "offers": [
                    {
                        "@type": "Offer",
                        "name": "Monthly Access",
                        "price": "199.00",
                        "priceCurrency": "USD",
                        "availability": "https://schema.org/InStock",
                        "url": "https://revolutiontradingpros.com/services/explosive-swings"
                    },
                    {
                        "@type": "Offer",
                        "name": "Yearly Access",
                        "price": "1999.00",
                        "priceCurrency": "USD",
                        "availability": "https://schema.org/InStock",
                        "url": "https://revolutiontradingpros.com/services/explosive-swings"
                    }
                ],
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.8",
                    "reviewCount": "124"
                }
            },
            {
                "@type": "FAQPage",
                "mainEntity": [
                    {
                        "@type": "Question",
                        "name": "How much capital do I need to start?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "We recommend a starting account size of at least $2,000 to effectively manage risk and follow our position sizing guidelines."
                        }
                    },
                    {
                        "@type": "Question",
                        "name": "Are these day trades or swing trades?",
                        "acceptedAnswer": {
                            "@type": "Answer",
                            "text": "These are strictly swing trades. We hold positions for 3 to 10 days on average, aiming to capture the bulk of a momentum move."
                        }
                    }
                ]
            }
        ]
    };

    const jsonLdString = JSON.stringify(schemaOrg);

    // --- Lifecycle & Interactions ---
    onMount(() => {
        // Run animations only on client to prevent SSR layout shift issues
        const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

        tl.fromTo(headerRef, 
            { opacity: 0, y: -20 },
            { opacity: 1, y: 0, duration: 0.8, ease: 'back.out(1.2)' }
        )
        .fromTo(contentRef.children, 
            { opacity: 0, y: 20 },
            { opacity: 1, y: 0, duration: 0.6, stagger: 0.1 }, 
            '-=0.4'
        )
        .fromTo(pricingRef.children, 
            { opacity: 0, y: 30 },
            { opacity: 1, y: 0, duration: 0.6, stagger: 0.1 }, 
            '-=0.2'
        );
    });

    async function handleSignUp(tier: PricingTier) {
        try {
            await addItemToCart(
                {
                    id: 'explosive-swings',
                    name: 'Explosive Swings Alerts',
                    description: `High-momentum swing trade alerts (${tier.id} plan)`,
                    price: tier.price,
                    type: 'alert-service',
                    interval: tier.id
                },
                true
            );
        } catch (error) {
            console.error('Cart Error:', error);
            // Ideally dispatch a toast notification here
        }
    }
</script>

<svelte:head>
    <title>Explosive Swings | Momentum Stock Alerts | Revolution Trading Pros</title>
    <meta name="description" content="Capture explosive market moves with high-momentum swing trading alerts. 3-10 day hold times, precise entries, and strict risk management for active traders." />
    <meta name="keywords" content="swing trading alerts, momentum stock picks, breakout trading, swing trade signals, stock market alerts, SPX swing trading" />
    <link rel="canonical" href="https://revolutiontradingpros.com/services/explosive-swings" />

    <meta property="og:type" content="product" />
    <meta property="og:title" content="Explosive Swings Trading Alerts" />
    <meta property="og:description" content="Don't miss the breakout. Get professional high-momentum swing trade alerts sent directly to your phone." />
    <meta property="og:url" content="https://revolutiontradingpros.com/services/explosive-swings" />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/explosive-swings-og.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Explosive Swings Trading Alerts" />
    <meta name="twitter:description" content="Professional swing trading alerts for active traders. Clear entries, exits, and risk plans." />
    <meta name="twitter:image" content="https://revolutiontradingpros.com/images/explosive-swings-og.jpg" />

    {@html `<script type="application/ld+json">${jsonLdString}</script>`}
</svelte:head>

<main class="min-h-screen relative bg-slate-950 text-slate-200 font-sans selection:bg-yellow-500/30 overflow-x-hidden">
    
    <div class="fixed inset-0 z-0 pointer-events-none" aria-hidden="true">
        <div class="gradient-bg absolute inset-0 opacity-40"></div>
        <div class="grid-overlay absolute inset-0 opacity-20"></div>
        <div class="glow-orb glow-orb-1 opacity-20 mix-blend-screen"></div>
        <div class="glow-orb glow-orb-2 opacity-20 mix-blend-screen"></div>
    </div>

    <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16 md:py-24">
        
        <header bind:this={headerRef} class="text-center mb-16 md:mb-24 opacity-0"> <div class="inline-flex items-center justify-center p-3 mb-6 rounded-2xl bg-gradient-to-b from-slate-800 to-slate-900 border border-slate-700 shadow-2xl shadow-yellow-500/10">
                <div class="relative">
                    <div class="absolute inset-0 bg-yellow-500 blur-lg opacity-40 animate-pulse"></div>
                    <IconFlame size={48} class="text-yellow-400 relative z-10" stroke={1.5} />
                </div>
            </div>
            
            <h1 class="text-5xl md:text-7xl font-heading font-extrabold mb-6 tracking-tight text-white">
                <span class="bg-gradient-to-r from-yellow-200 via-amber-400 to-orange-500 bg-clip-text text-transparent drop-shadow-sm">
                    Explosive Swings
                </span>
            </h1>
            
            <div class="text-xl md:text-2xl text-slate-400 max-w-2xl mx-auto flex flex-col md:flex-row items-center justify-center gap-2 md:gap-4 mb-8">
                <span class="flex items-center gap-2">
                    <IconTrendingUp size={24} class="text-emerald-400" />
                    Momentum Breakouts
                </span>
                <span class="hidden md:inline text-slate-700" aria-hidden="true">•</span>
                <span class="flex items-center gap-2">
                    <IconBolt size={24} class="text-yellow-400" />
                    3-10 Day Holds
                </span>
            </div>

            <a href="#pricing" class="inline-flex items-center gap-2 px-8 py-4 bg-yellow-500 hover:bg-yellow-400 text-slate-950 font-bold rounded-full transition-transform hover:scale-105 shadow-lg shadow-yellow-500/20">
                Start Receiving Alerts
                <IconBolt size={20} />
            </a>
        </header>

        <section bind:this={contentRef} class="grid lg:grid-cols-2 gap-12 items-center mb-24 opacity-0">
            <div class="space-y-8">
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-yellow-500/10 border border-yellow-500/20 text-yellow-300 font-medium text-sm">
                    <IconBellRinging size={16} />
                    <span>Real-Time Breakout Alerts</span>
                </div>
                
                <h2 class="text-3xl md:text-4xl font-bold text-white leading-tight">
                    Stop Guessing. Start Catching the <span class="text-yellow-400">Big Moves.</span>
                </h2>
                
                <p class="text-lg text-slate-300 leading-relaxed">
                    We scan thousands of stocks daily to identify high-momentum setups. When a stock breaks key resistance with institutional volume, we alert you immediately with a complete trade plan.
                </p>

                <div class="space-y-4">
                    <article class="flex items-start gap-4 p-4 rounded-xl bg-slate-900/50 border border-slate-800 hover:border-yellow-500/30 transition-colors group">
                        <div class="bg-slate-800 p-2 rounded-lg group-hover:bg-yellow-500/10 transition-colors">
                            <IconChartCandle size={24} class="text-yellow-400" />
                        </div>
                        <div>
                            <h3 class="font-bold text-white mb-1">Technical Breakouts</h3>
                            <p class="text-sm text-slate-400">We target bull flags, cup & handles, and volume breakouts on the daily timeframe.</p>
                        </div>
                    </article>

                    <article class="flex items-start gap-4 p-4 rounded-xl bg-slate-900/50 border border-slate-800 hover:border-yellow-500/30 transition-colors group">
                        <div class="bg-slate-800 p-2 rounded-lg group-hover:bg-yellow-500/10 transition-colors">
                            <IconBellRinging size={24} class="text-yellow-400" />
                        </div>
                        <div>
                            <h3 class="font-bold text-white mb-1">Clear Trade Plans</h3>
                            <p class="text-sm text-slate-400">No ambiguity. Every alert includes Entry Price, Stop Loss, Target 1, Target 2, and Chart Analysis.</p>
                        </div>
                    </article>
                </div>
            </div>

            <div class="relative group" role="img" aria-label="Example of a trading alert showing AMD breakout">
                <div class="absolute -inset-1 bg-gradient-to-r from-yellow-500 to-orange-600 rounded-3xl opacity-20 blur-2xl group-hover:opacity-30 transition-opacity duration-500"></div>
                <div class="relative bg-slate-900 border border-slate-700 rounded-3xl overflow-hidden shadow-2xl">
                    <div class="p-3 bg-slate-800 border-b border-slate-700 flex gap-2">
                        <div class="w-3 h-3 rounded-full bg-red-500/20"></div>
                        <div class="w-3 h-3 rounded-full bg-yellow-500/20"></div>
                        <div class="w-3 h-3 rounded-full bg-green-500/20"></div>
                    </div>
                    
                    <div class="p-8 flex items-center justify-center min-h-[300px] bg-slate-900 relative">
                        <svg viewBox="0 0 400 200" class="w-full h-full opacity-50" preserveAspectRatio="none">
                            <path d="M0,150 C50,150 100,100 150,120 C200,140 250,50 300,40 C350,30 400,10 400,10" fill="none" stroke="#fbbf24" stroke-width="2" />
                            <circle cx="300" cy="40" r="4" fill="#fbbf24" class="animate-pulse" />
                        </svg>
                        
                        <div class="absolute bottom-8 left-1/2 -translate-x-1/2 w-64 bg-slate-800/90 backdrop-blur-md p-4 rounded-xl border border-slate-600 shadow-xl">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 rounded-full bg-green-500/20 flex items-center justify-center shrink-0">
                                    <IconTrendingUp size={20} class="text-green-400" />
                                </div>
                                <div>
                                    <div class="text-sm font-bold text-white">AMD Alert Triggered</div>
                                    <div class="text-xs text-green-400">Entry: $142.50 | Target: $155</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section id="pricing" class="max-w-6xl mx-auto mb-24">
            <div class="text-center mb-12">
                <h2 class="text-3xl font-heading font-bold text-white mb-4">Select Your Plan</h2>
                <p class="text-slate-400">Full access to all swing alerts, community chat, and daily updates.</p>
            </div>

            <div bind:this={pricingRef} class="grid md:grid-cols-3 gap-8 items-start opacity-0">
                {#each pricingTiers as tier (tier.id)}
                    <div class="relative group {tier.isBestValue ? 'z-10 md:-mt-4' : ''}">
                        {#if tier.isBestValue}
                            <div class="absolute -inset-[2px] bg-gradient-to-br from-yellow-400 to-orange-600 rounded-2xl opacity-75 blur-sm group-hover:opacity-100 transition-opacity"></div>
                        {:else}
                            <div class="absolute inset-0 bg-slate-800 rounded-2xl transform transition-transform group-hover:translate-y-1"></div>
                        {/if}

                        <div class="relative bg-slate-900 border {tier.isBestValue ? 'border-transparent' : 'border-slate-700'} rounded-2xl p-8 shadow-2xl h-full transition-all group-hover:border-slate-600">
                            
                            {#if tier.isBestValue}
                                <div class="absolute top-0 right-0 -mt-3 mr-3 px-3 py-1 bg-gradient-to-r from-yellow-500 to-orange-500 text-slate-900 text-xs font-bold rounded-full shadow-lg">
                                    BEST VALUE
                                </div>
                            {/if}

                            <h3 class="text-xl font-bold {tier.isBestValue ? 'text-yellow-400' : 'text-slate-300'} mb-2 capitalize">{tier.id}</h3>
                            
                            <div class="flex items-baseline gap-1 mb-2">
                                <span class="text-4xl md:text-5xl font-bold text-white">${tier.displayPrice}</span>
                                <span class="text-slate-500">{tier.period}</span>
                            </div>

                            {#if tier.savings}
                                <p class="text-xs text-green-400 font-medium mb-6">{tier.savings}</p>
                            {:else}
                                <div class="h-4 mb-6"></div> {/if}
                            
                            <button 
                                on:click={() => handleSignUp(tier)}
                                class="w-full py-3 px-4 rounded-xl font-bold transition-all flex items-center justify-center gap-2 mb-8 
                                {tier.isBestValue 
                                    ? 'bg-gradient-to-r from-yellow-500 to-orange-600 hover:from-yellow-400 hover:to-orange-500 text-slate-900 shadow-lg shadow-orange-500/25 transform hover:-translate-y-1' 
                                    : 'bg-slate-800 hover:bg-slate-700 text-white border border-slate-600'}"
                                aria-label={`Sign up for ${tier.id} plan at $${tier.displayPrice}`}
                            >
                                {#if tier.isBestValue}<IconShoppingCart size={20} />{/if}
                                {tier.isBestValue ? 'Start Trading' : 'Select Plan'}
                            </button>

                            <ul class="space-y-4 text-sm text-slate-300">
                                <li class="flex items-start gap-3">
                                    <IconCheck size={18} class="{tier.isBestValue ? 'text-yellow-500' : 'text-slate-600'} shrink-0" /> 
                                    <span>Mobile & Email Alerts</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <IconCheck size={18} class="{tier.isBestValue ? 'text-yellow-500' : 'text-slate-600'} shrink-0" /> 
                                    <span>Daily Watchlist</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <IconCheck size={18} class="{tier.isBestValue ? 'text-yellow-500' : 'text-slate-600'} shrink-0" /> 
                                    <span>Entry & Exit signals</span>
                                </li>
                                {#if tier.isBestValue}
                                    <li class="flex items-start gap-3 pt-2 border-t border-slate-800">
                                        <IconCheck size={18} class="text-yellow-500 shrink-0" /> 
                                        <strong>Priority Support</strong>
                                    </li>
                                    <li class="flex items-start gap-3">
                                        <IconCheck size={18} class="text-yellow-500 shrink-0" /> 
                                        <strong>Exclusive Video Lessons</strong>
                                    </li>
                                {/if}
                            </ul>
                        </div>
                    </div>
                {/each}
            </div>

            <div class="mt-12 text-center">
                <figure class="inline-block relative mx-auto">
                    <div class="w-64 h-12 bg-slate-800 rounded-2xl flex items-center justify-center gap-4 border border-slate-700 shadow-lg">
                        <div class="flex gap-1">
                            <span class="block w-2 h-2 rounded-full bg-slate-600 animate-bounce"></span>
                            <span class="block w-2 h-2 rounded-full bg-slate-600 animate-bounce delay-75"></span>
                            <span class="block w-2 h-2 rounded-full bg-slate-600 animate-bounce delay-150"></span>
                        </div>
                        <span class="text-xs font-mono text-slate-400">Incoming Alert...</span>
                    </div>
                    <figcaption class="sr-only">Visual representation of receiving a mobile notification</figcaption>
                </figure>
                
                <p class="text-sm text-slate-500 mt-6 flex items-center justify-center gap-2">
                     <span class="w-2 h-2 bg-green-500 rounded-full"></span> 
                     Secure 256-bit SSL Encrypted Payment • Cancel Anytime
                </p>
            </div>
        </section>

        <section class="max-w-4xl mx-auto mb-24 border-t border-slate-800 pt-16">
            <h2 class="text-2xl font-bold text-white mb-8 flex items-center gap-2">
                <IconQuestionMark size={28} class="text-slate-500" />
                Frequently Asked Questions
            </h2>
            <div class="grid md:grid-cols-2 gap-8">
                <div>
                    <h3 class="font-bold text-white mb-2">How much capital do I need?</h3>
                    <p class="text-slate-400 text-sm leading-relaxed">We recommend starting with at least $2,000. This allows you to take trades while adhering to proper risk management rules (never risking more than 2-3% of your account per trade).</p>
                </div>
                <div>
                    <h3 class="font-bold text-white mb-2">Are these day trades?</h3>
                    <p class="text-slate-400 text-sm leading-relaxed">No, "Explosive Swings" focuses on positions held for 3 to 10 days. If you are looking for intraday scalps, check out our Day Trading Room service.</p>
                </div>
                <div>
                    <h3 class="font-bold text-white mb-2">How are alerts delivered?</h3>
                    <p class="text-slate-400 text-sm leading-relaxed">You receive alerts instantly via our mobile app push notifications, email, and inside the members-only dashboard.</p>
                </div>
                <div>
                    <h3 class="font-bold text-white mb-2">Can I cancel?</h3>
                    <p class="text-slate-400 text-sm leading-relaxed">Yes. You can cancel your subscription at any time directly from your dashboard. There are no long-term contracts.</p>
                </div>
            </div>
        </section>

    </div>

    <footer class="relative z-10 border-t border-slate-900 bg-slate-950 py-12 px-4">
        <div class="max-w-4xl mx-auto text-center">
            <aside class="mb-6 p-4 bg-slate-900/50 rounded-lg border border-slate-800/50">
                <h4 class="text-slate-500 text-xs font-bold uppercase tracking-widest mb-2">Risk Disclosure</h4>
                <p class="text-xs text-slate-600 leading-relaxed text-justify md:text-center">
                    Trading securities involves a high degree of risk and is not suitable for all investors. You could lose some or all of your initial investment. Revolution Trading Pros is an educational platform and data provider; we are not registered investment advisors. Past performance of any trading system or methodology is not necessarily indicative of future results. Please trade responsibly.
                </p>
            </aside>
            <p class="text-xs text-slate-700">&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.</p>
        </div>
    </footer>
</main>

<style>
    /* * Performance Optimization:
     * Complex gradients and filters handled in CSS for GPU acceleration 
     */
    
    .gradient-bg {
        background: radial-gradient(circle at 50% 50%, #1e1b4b 0%, #020617 100%);
        /* Hint browser to promote to layer */
        will-change: transform; 
    }

    .grid-overlay {
        background-image: 
            linear-gradient(rgba(234, 179, 8, 0.05) 1px, transparent 1px), 
            linear-gradient(90deg, rgba(234, 179, 8, 0.05) 1px, transparent 1px);
        background-size: 40px 40px;
        mask-image: radial-gradient(circle at center, black 40%, transparent 100%);
    }

    .glow-orb {
        position: absolute;
        border-radius: 50%;
        filter: blur(80px); /* Reduced blur radius slightly for performance */
    }

    .glow-orb-1 {
        width: 600px;
        height: 600px;
        top: -150px;
        left: 20%;
        background: radial-gradient(circle, rgba(234, 179, 8, 0.12), transparent 70%);
        animation: float1 20s infinite ease-in-out;
    }

    .glow-orb-2 {
        width: 500px;
        height: 500px;
        bottom: -150px;
        right: 10%;
        background: radial-gradient(circle, rgba(249, 115, 22, 0.12), transparent 70%);
        animation: float2 25s infinite ease-in-out;
    }

    @keyframes float1 {
        0%, 100% { transform: translate(0, 0) scale(1); }
        50% { transform: translate(-30px, 30px) scale(1.1); }
    }

    @keyframes float2 {
        0%, 100% { transform: translate(0, 0) scale(1); }
        50% { transform: translate(30px, -30px) scale(0.9); }
    }

    @media (max-width: 640px) {
        .glow-orb { opacity: 0.1; filter: blur(50px); }
        .glow-orb-1, .glow-orb-2 { width: 300px; height: 300px; }
    }
</style>