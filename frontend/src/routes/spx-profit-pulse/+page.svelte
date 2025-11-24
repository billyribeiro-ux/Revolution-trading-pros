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
        IconChartCandle
    } from '@tabler/icons-svelte';
    import { addItemToCart } from '$lib/utils/cart-helpers';

    let headerRef: HTMLElement;
    let contentRef: HTMLElement;
    let pricingRef: HTMLElement;

    // Structured Data for the Product/Service
    const schemaOrg = {
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Product",
                "name": "Explosive Swings Trading Alerts",
                "description": "High-momentum swing trade alerts focusing on 3-10 day hold times for explosive profit potential.",
                "brand": {
                    "@type": "Brand",
                    "name": "Revolution Trading Pros"
                },
                "image": "https://revolutiontradingpros.com/images/explosive-swings-og.jpg",
                "offers": [
                    {
                        "@type": "Offer",
                        "name": "Monthly Subscription",
                        "price": "199.00",
                        "priceCurrency": "USD",
                        "billingDuration": "P1M"
                    },
                    {
                        "@type": "Offer",
                        "name": "Quarterly Subscription",
                        "price": "549.00",
                        "priceCurrency": "USD",
                        "billingDuration": "P3M"
                    },
                    {
                        "@type": "Offer",
                        "name": "Yearly Subscription",
                        "price": "1999.00",
                        "priceCurrency": "USD",
                        "billingDuration": "P1Y"
                    }
                ]
            }
        ]
    };

    const jsonLdString = JSON.stringify(schemaOrg);

    onMount(() => {
        initAnimations();
    });

    function initAnimations() {
        const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

        // Staggered entrance
        tl.from(headerRef, {
            opacity: 0,
            y: -30,
            duration: 1,
            ease: 'back.out(1.2)'
        })
        .from(contentRef.children, {
            opacity: 0,
            y: 30,
            duration: 0.8,
            stagger: 0.1
        }, '-=0.6')
        .from(pricingRef.children, {
            opacity: 0,
            y: 40,
            duration: 0.8,
            stagger: 0.15
        }, '-=0.4');
    }

    async function handleSignUp(interval: 'monthly' | 'quarterly' | 'yearly') {
        let price: number;
        if (interval === 'monthly') price = 199;
        else if (interval === 'quarterly') price = 549;
        else price = 1999;

        await addItemToCart(
            {
                id: 'explosive-swings',
                name: 'Explosive Swings',
                description: 'High-momentum swing trade alerts',
                price,
                type: 'alert-service',
                interval
            },
            true 
        );
    }
</script>

<svelte:head>
    <title>Explosive Swings | Momentum Stock Alerts | Revolution Trading Pros</title>
    <meta name="description" content="Capture explosive market moves with our high-momentum swing trading alerts. 3-10 day hold times, clear entries, and defined risk management." />
    <meta name="keywords" content="swing trading alerts, momentum stock picks, breakout trading, swing trade signals, stock market alerts" />
    <link rel="canonical" href="https://revolutiontradingpros.com/services/explosive-swings" />

    <meta property="og:type" content="product" />
    <meta property="og:title" content="Explosive Swings Trading Alerts" />
    <meta property="og:description" content="Don't miss the breakout. Get high-momentum swing trade alerts sent directly to your phone." />
    <meta property="og:url" content="https://revolutiontradingpros.com/services/explosive-swings" />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/explosive-swings-og.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Explosive Swings Trading Alerts" />
    <meta name="twitter:description" content="High-momentum swing trading alerts for active traders." />
    <meta name="twitter:image" content="https://revolutiontradingpros.com/images/explosive-swings-og.jpg" />

    {@html `<script type="application/ld+json">${jsonLdString}</script>`}
</svelte:head>

<main class="min-h-screen relative bg-slate-950 overflow-hidden text-slate-200 font-sans selection:bg-yellow-500/30">
    
    <div class="fixed inset-0 z-0 pointer-events-none">
        <div class="gradient-bg absolute inset-0 opacity-40"></div>
        <div class="grid-overlay absolute inset-0 opacity-20"></div>
        <div class="glow-orb glow-orb-1 opacity-20 mix-blend-screen"></div>
        <div class="glow-orb glow-orb-2 opacity-20 mix-blend-screen"></div>
    </div>

    <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16 md:py-24">
        
        <header bind:this={headerRef} class="text-center mb-16 md:mb-24">
            <div class="inline-flex items-center justify-center p-3 mb-6 rounded-2xl bg-gradient-to-b from-slate-800 to-slate-900 border border-slate-700 shadow-2xl shadow-yellow-500/10">
                <div class="relative">
                    <div class="absolute inset-0 bg-yellow-500 blur-lg opacity-40 animate-pulse"></div>
                    <IconFlame size={48} class="text-yellow-400 relative z-10" stroke={1.5} />
                </div>
            </div>
            
            <h1 class="text-5xl md:text-7xl font-heading font-extrabold mb-6 tracking-tight">
                <span class="bg-gradient-to-r from-yellow-200 via-amber-400 to-orange-500 bg-clip-text text-transparent drop-shadow-sm">
                    Explosive Swings
                </span>
            </h1>
            
            <p class="text-xl md:text-2xl text-slate-400 max-w-2xl mx-auto flex flex-col md:flex-row items-center justify-center gap-2 md:gap-4">
                <span class="flex items-center gap-2">
                    <IconTrendingUp size={24} class="text-emerald-400" />
                    Momentum Breakouts
                </span>
                <span class="hidden md:inline text-slate-700">•</span>
                <span class="flex items-center gap-2">
                    <IconBolt size={24} class="text-yellow-400" />
                    3-10 Day Holds
                </span>
            </p>
        </header>

        <section bind:this={contentRef} class="grid lg:grid-cols-2 gap-12 items-center mb-24">
            <div class="space-y-8">
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-yellow-500/10 border border-yellow-500/20 text-yellow-300 font-medium text-sm">
                    <IconBellRinging size={16} />
                    <span>Real-Time Breakout Alerts</span>
                </div>
                
                <h2 class="text-3xl md:text-4xl font-bold text-white leading-tight">
                    Stop Guessing. Start Catching the <span class="text-yellow-400">Big Moves.</span>
                </h2>
                
                <p class="text-lg text-slate-300 leading-relaxed">
                    We scan thousands of stocks daily to identify high-momentum setups. When a stock breaks key resistance with institutional volume, we alert you immediately.
                </p>

                <div class="space-y-4">
                    <div class="flex items-start gap-4 p-4 rounded-xl bg-slate-900/50 border border-slate-800 hover:border-yellow-500/30 transition-colors group">
                        <div class="bg-slate-800 p-2 rounded-lg group-hover:bg-yellow-500/10 transition-colors">
                            <IconChartCandle size={24} class="text-yellow-400" />
                        </div>
                        <div>
                            <h3 class="font-bold text-white mb-1">Technical Breakouts</h3>
                            <p class="text-sm text-slate-400">We target bull flags, cup & handles, and volume breakouts on the daily timeframe.</p>
                        </div>
                    </div>

                    <div class="flex items-start gap-4 p-4 rounded-xl bg-slate-900/50 border border-slate-800 hover:border-yellow-500/30 transition-colors group">
                        <div class="bg-slate-800 p-2 rounded-lg group-hover:bg-yellow-500/10 transition-colors">
                            <IconBellRinging size={24} class="text-yellow-400" />
                        </div>
                        <div>
                            <h3 class="font-bold text-white mb-1">Clear Trade Plans</h3>
                            <p class="text-sm text-slate-400">Every alert includes: Entry Price, Stop Loss, Target 1, Target 2, and Chart Analysis.</p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="relative">
                <div class="absolute -inset-1 bg-gradient-to-r from-yellow-500 to-orange-600 rounded-3xl opacity-20 blur-2xl"></div>
                <div class="relative bg-slate-900 border border-slate-700 rounded-3xl overflow-hidden shadow-2xl">
                    <div class="p-1 bg-slate-800 border-b border-slate-700 flex gap-2">
                        <div class="w-3 h-3 rounded-full bg-red-500/20"></div>
                        <div class="w-3 h-3 rounded-full bg-yellow-500/20"></div>
                        <div class="w-3 h-3 rounded-full bg-green-500/20"></div>
                    </div>
                    <div class="p-8 flex items-center justify-center min-h-[300px] bg-[url('/grid-pattern.svg')]">

                    </div>
                    <div class="p-4 bg-slate-800/80 backdrop-blur border-t border-slate-700">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 rounded-full bg-green-500/20 flex items-center justify-center">
                                    <IconTrendingUp size={20} class="text-green-400" />
                                </div>
                                <div>
                                    <div class="text-sm font-bold text-white">AMD Breakout</div>
                                    <div class="text-xs text-green-400">+12.5% Profit (2 Days)</div>
                                </div>
                            </div>
                            <div class="text-xs text-slate-500">Alert sent 2 days ago</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="max-w-6xl mx-auto">
            <div class="text-center mb-12">
                <h2 class="text-3xl font-heading font-bold text-white mb-4">Select Your Plan</h2>
                <p class="text-slate-400">Full access to all swing alerts, chat room, and updates.</p>
            </div>

            <div bind:this={pricingRef} class="grid md:grid-cols-3 gap-8 items-start">
                
                <div class="relative group">
                    <div class="absolute inset-0 bg-slate-800 rounded-2xl transform transition-transform group-hover:translate-y-1"></div>
                    <div class="relative bg-slate-900 border border-slate-700 rounded-2xl p-8 shadow-xl transition-all group-hover:border-slate-600">
                        <h3 class="text-xl font-bold text-slate-300 mb-2">Monthly</h3>
                        <div class="flex items-baseline gap-1 mb-6">
                            <span class="text-4xl font-bold text-white">$199</span>
                            <span class="text-slate-500">/mo</span>
                        </div>
                        <button 
                            on:click={() => handleSignUp('monthly')}
                            class="w-full py-3 px-4 bg-slate-800 hover:bg-slate-700 text-white font-bold rounded-xl border border-slate-600 transition-all flex items-center justify-center gap-2 mb-8"
                            aria-label="Sign up for monthly plan"
                        >
                            Select Plan
                        </button>
                        <ul class="space-y-4 text-sm text-slate-400">
                            <li class="flex items-start gap-3"><IconCheck size={18} class="text-slate-600 shrink-0" /> Mobile & Email Alerts</li>
                            <li class="flex items-start gap-3"><IconCheck size={18} class="text-slate-600 shrink-0" /> Daily Watchlist</li>
                            <li class="flex items-start gap-3"><IconCheck size={18} class="text-slate-600 shrink-0" /> Entry & Exit signals</li>
                        </ul>
                    </div>
                </div>

                <div class="relative group z-10 md:-mt-4">
                    <div class="absolute -inset-[2px] bg-gradient-to-br from-yellow-400 to-orange-600 rounded-2xl opacity-75 blur-sm group-hover:opacity-100 transition-opacity"></div>
                    <div class="relative bg-slate-900 rounded-2xl p-8 shadow-2xl h-full">
                        <div class="absolute top-0 right-0 -mt-3 mr-3 px-3 py-1 bg-gradient-to-r from-yellow-500 to-orange-500 text-slate-900 text-xs font-bold rounded-full shadow-lg">
                            BEST VALUE
                        </div>
                        <h3 class="text-xl font-bold text-yellow-400 mb-2">Yearly</h3>
                        <div class="flex items-baseline gap-1 mb-2">
                            <span class="text-5xl font-bold text-white">$1,999</span>
                            <span class="text-slate-500">/yr</span>
                        </div>
                        <p class="text-xs text-green-400 font-medium mb-6">Save $389 per year</p>
                        
                        <button 
                            on:click={() => handleSignUp('yearly')}
                            class="w-full py-4 px-6 bg-gradient-to-r from-yellow-500 to-orange-600 hover:from-yellow-400 hover:to-orange-500 text-slate-900 font-bold text-lg rounded-xl shadow-lg shadow-orange-500/25 transition-all flex items-center justify-center gap-2 mb-8 transform hover:-translate-y-1"
                            aria-label="Sign up for yearly plan"
                        >
                            <IconShoppingCart size={20} />
                            Start Trading
                        </button>

                        <div class="space-y-4">
                            <p class="text-xs font-bold text-slate-500 uppercase tracking-wider">Everything in Monthly, plus:</p>
                            <ul class="space-y-4 text-sm text-slate-300">
                                <li class="flex items-start gap-3"><IconCheck size={18} class="text-yellow-500 shrink-0" /> <strong>2 Months Free</strong></li>
                                <li class="flex items-start gap-3"><IconCheck size={18} class="text-yellow-500 shrink-0" /> Priority Support</li>
                                <li class="flex items-start gap-3"><IconCheck size={18} class="text-yellow-500 shrink-0" /> 1-on-1 Portfolio Review</li>
                                <li class="flex items-start gap-3"><IconCheck size={18} class="text-yellow-500 shrink-0" /> Exclusive Video Lessons</li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="relative group">
                    <div class="absolute inset-0 bg-slate-800 rounded-2xl transform transition-transform group-hover:translate-y-1"></div>
                    <div class="relative bg-slate-900 border border-slate-700 rounded-2xl p-8 shadow-xl transition-all group-hover:border-slate-600">
                        <h3 class="text-xl font-bold text-slate-300 mb-2">Quarterly</h3>
                        <div class="flex items-baseline gap-1 mb-2">
                            <span class="text-4xl font-bold text-white">$549</span>
                            <span class="text-slate-500">/qtr</span>
                        </div>
                        <p class="text-xs text-green-400 font-medium mb-6">Save $48 per year</p>
                        <button 
                            on:click={() => handleSignUp('quarterly')}
                            class="w-full py-3 px-4 bg-slate-800 hover:bg-slate-700 text-white font-bold rounded-xl border border-slate-600 transition-all flex items-center justify-center gap-2 mb-8"
                            aria-label="Sign up for quarterly plan"
                        >
                            Select Plan
                        </button>
                        <ul class="space-y-4 text-sm text-slate-400">
                            <li class="flex items-start gap-3"><IconCheck size={18} class="text-slate-600 shrink-0" /> Mobile & Email Alerts</li>
                            <li class="flex items-start gap-3"><IconCheck size={18} class="text-slate-600 shrink-0" /> Daily Watchlist</li>
                            <li class="flex items-start gap-3"><IconCheck size={18} class="text-slate-600 shrink-0" /> Entry & Exit signals</li>
                        </ul>
                    </div>
                </div>

            </div>

            <div class="mt-12 text-center">
                

[Image of mobile phone showing trading alert notification]

                <p class="text-sm text-slate-500 mt-4">
                    Secure 256-bit SSL Encrypted Payment • Cancel Anytime via Dashboard
                </p>
            </div>
        </section>
    </div>

    <footer class="relative z-10 border-t border-slate-900 bg-slate-950 py-12 px-4">
        <div class="max-w-4xl mx-auto text-center">
            <h4 class="text-slate-500 text-xs font-bold uppercase tracking-widest mb-4">Risk Disclosure</h4>
            <p class="text-xs text-slate-600 leading-relaxed">
                Trading securities involves a high degree of risk and is not suitable for all investors. You could lose some or all of your initial investment. Revolution Trading Pros is an educational platform and data provider; we are not registered investment advisors. Past performance of any trading system or methodology is not necessarily indicative of future results. Please trade responsibly.
            </p>
        </div>
    </footer>
</main>

<style>
    /* Optimized Animations:
       Moved to CSS to keep the main thread free for interaction and parsing.
    */
    
    .gradient-bg {
        background: radial-gradient(circle at 50% 50%, #1e1b4b 0%, #020617 100%);
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
        filter: blur(100px);
        animation-timing-function: ease-in-out;
        animation-iteration-count: infinite;
    }

    .glow-orb-1 {
        width: 600px;
        height: 600px;
        top: -150px;
        left: 20%;
        background: radial-gradient(circle, rgba(234, 179, 8, 0.15), transparent 70%);
        animation: float1 20s infinite;
    }

    .glow-orb-2 {
        width: 500px;
        height: 500px;
        bottom: -150px;
        right: 10%;
        background: radial-gradient(circle, rgba(249, 115, 22, 0.15), transparent 70%);
        animation: float2 25s infinite;
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
        .glow-orb { opacity: 0.15; filter: blur(60px); }
        .glow-orb-1, .glow-orb-2 { width: 300px; height: 300px; }
    }
</style>