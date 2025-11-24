<script lang="ts">
    import { onMount } from 'svelte';
    import { slide, fade } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';

    // --- Pricing State ---
    let billingInterval: 'monthly' | 'annual' = 'monthly';

    // Pricing Data Configuration
    const pricing = {
        monthly: {
            price: '97',
            period: '/mo',
            btnText: 'Subscribe Monthly',
            link: '/checkout/monthly-swings',
            savings: null
        },
        annual: {
            price: '927',
            period: '/yr',
            btnText: 'Join Annual & Save',
            link: '/checkout/annual-swings',
            savings: 'Save $237 / year'
        }
    };

    $: activePlan = pricing[billingInterval];

    // --- FAQ State ---
    let openFaq: number | null = null;
    const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

    // --- Icon SVG ---
    const IconCheckSvg = `<svg class="w-4 h-4 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" /></svg>`;

    // --- Intersection Observer (Scroll Reveal) ---
    // Refined to be robust and respect user motion preferences
    let observer: IntersectionObserver;

    function reveal(node: HTMLElement, params: { delay?: number } = {}) {
        // Check for reduced motion preference
        const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
        if (mediaQuery.matches) return;

        node.classList.add('opacity-0', 'translate-y-8');
        
        if (observer) {
            node.dataset.delay = (params.delay || 0).toString();
            observer.observe(node);
        }

        return {
            destroy() {
                if (observer) observer.unobserve(node);
            }
        };
    }

    onMount(() => {
        const handleIntersect = (entries: IntersectionObserverEntry[]) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const el = entry.target as HTMLElement;
                    const delay = parseInt(el.dataset.delay || '0');
                    
                    setTimeout(() => {
                        el.classList.remove('opacity-0', 'translate-y-8');
                        el.classList.add('opacity-100', 'translate-y-0');
                    }, delay);
                    
                    observer.unobserve(el);
                }
            });
        };

        observer = new IntersectionObserver(handleIntersect, { 
            threshold: 0.15, 
            rootMargin: '0px 0px -50px 0px' 
        });

        // Add transition classes globally to revealed elements
        // (We do this here to keep the HTML clean)
        const revealElements = document.querySelectorAll('[data-reveal]');
        revealElements.forEach(el => {
             el.classList.add('transition-all', 'duration-700', 'ease-out');
             observer.observe(el); // If you use data-reveal attribute instead of use:reveal
        });
    });

    // --- SEO: JSON-LD ---
    const faqData = [
        {
            q: "How much capital do I need for swing trading?",
            a: "We recommend a minimum of $2,000 to properly manage risk (risk 1-2% per trade), though our strategies are mathematically valid on accounts of all sizes."
        },
        {
            q: "Are these day trades?",
            a: "No. These are swing trades held typically for 3-7 days. This service is specifically designed for traders who have day jobs or cannot stare at charts all day."
        },
        {
            q: "What markets do you trade?",
            a: "We primarily trade large-cap US equities and liquid options (SPY, QQQ, Magnificent 7 stocks) to ensure easy entry and exit execution."
        }
    ];

    const schemaOrg = {
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Product",
                "name": "Explosive Swings Trading Alerts",
                "description": "Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals.",
                "brand": {
                    "@type": "Brand",
                    "name": "Revolution Trading Pros"
                },
                "image": "https://revolutiontradingpros.com/images/og-swings.jpg",
                "offers": {
                    "@type": "Offer",
                    "priceCurrency": "USD",
                    "price": billingInterval === 'monthly' ? "97" : "927",
                    "availability": "https://schema.org/InStock",
                    "url": "https://revolutiontradingpros.com/explosive-swings"
                },
                "aggregateRating": {
                    "@type": "AggregateRating",
                    "ratingValue": "4.9",
                    "reviewCount": "84"
                }
            },
            {
                "@type": "FAQPage",
                "mainEntity": faqData.map(item => ({
                    "@type": "Question",
                    "name": item.q,
                    "acceptedAnswer": { "@type": "Answer", "text": item.a }
                }))
            }
        ]
    };

    const jsonLdString = JSON.stringify(schemaOrg);
</script>

<svelte:head>
    <title>Explosive Swings Alerts | Multi-Day Swing Trading | Revolution Trading Pros</title>
    <meta name="description" content="Get explosive swing trading alerts for multi-day opportunities. Expert analysis, precise entries/exits, and proven strategies for active swing traders." />
    <meta name="keywords" content="swing trading alerts, stock options alerts, swing trade signals, multi-day trading strategies, SPX swing trading" />
    <link rel="canonical" href="https://revolutiontradingpros.com/explosive-swings" />
    
    <meta property="og:type" content="product" />
    <meta property="og:title" content="Explosive Swings Alerts | Catch Multi-Day Moves" />
    <meta property="og:description" content="Don't stare at charts all day. Get high-probability swing trade alerts sent to your phone." />
    <meta property="og:url" content="https://revolutiontradingpros.com/explosive-swings" />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/og-swings.jpg" />

    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="Explosive Swings Trading Alerts" />
    <meta name="twitter:description" content="Precision swing trading alerts. 82% win rate. 3-7 day hold times." />
    <meta name="twitter:image" content="https://revolutiontradingpros.com/images/og-swings.jpg" />

    {@html `<script type="application/ld+json">${jsonLdString}</script>`}
</svelte:head>

<main class="w-full overflow-x-hidden bg-slate-950 text-slate-200 font-sans selection:bg-emerald-500/30 selection:text-emerald-200">

    <section class="relative min-h-[90vh] flex items-center overflow-hidden py-20 lg:py-0">
        <div class="absolute inset-0 z-0 pointer-events-none">
            <div class="absolute inset-0 bg-[linear-gradient(rgba(16,185,129,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(16,185,129,0.03)_1px,transparent_1px)] bg-[size:48px_48px] opacity-40"></div>
            <div class="absolute top-0 right-0 w-[600px] h-[600px] bg-emerald-500/10 rounded-full blur-[100px] animate-pulse"></div>
            <div class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-blue-600/10 rounded-full blur-[120px]"></div>
        </div>

        <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-16 items-center">
            
            <div class="text-center lg:text-left">
                <div use:reveal={{ delay: 0 }} class="inline-flex items-center gap-2 bg-slate-900 border border-emerald-500/30 px-4 py-1.5 rounded-full mb-8 shadow-lg shadow-emerald-500/10 backdrop-blur-md">
                    <span class="relative flex h-2.5 w-2.5">
                      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                      <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
                    </span>
                    <span class="text-xs font-bold tracking-wider uppercase text-emerald-400">Swing Signals Active</span>
                </div>
                
                <h1 use:reveal={{ delay: 100 }} class="text-5xl md:text-7xl font-heading font-extrabold mb-6 leading-tight tracking-tight text-white">
                    Catch the <br />
                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 via-emerald-300 to-teal-200">Big Moves.</span>
                </h1>
                
                <p use:reveal={{ delay: 200 }} class="text-xl text-slate-400 mb-10 max-w-2xl mx-auto lg:mx-0 leading-relaxed">
                    Stop staring at the 1-minute chart. Get high-precision multi-day swing alerts designed for traders who want freedom, not a job.
                </p>
                
                <div use:reveal={{ delay: 300 }} class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start items-center">
                    <a href="#pricing" class="group relative w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-slate-950 transition-all duration-200 bg-emerald-500 rounded-xl hover:bg-emerald-400 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-emerald-500 focus:ring-offset-slate-900 shadow-lg hover:shadow-emerald-500/25 hover:-translate-y-1">
                        Start Trading Swings
                        <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" /></svg>
                    </a>
                    <a href="#process" class="w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-slate-200 transition-all duration-200 bg-slate-900 border border-slate-700 rounded-xl hover:bg-slate-800 hover:border-emerald-500/30">
                        See How It Works
                    </a>
                </div>
                
                <div use:reveal={{ delay: 400 }} class="mt-10 flex items-center justify-center lg:justify-start gap-6 text-sm font-medium text-slate-500">
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        <span>Precise Entries</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        <span>3-7 Day Holds</span>
                    </div>
                </div>
            </div>

            <div class="hidden lg:block relative perspective-1000">
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-gradient-to-tr from-emerald-500/20 to-transparent rounded-full blur-3xl"></div>
                
                <div class="relative bg-slate-900/90 backdrop-blur-xl border border-slate-700 p-8 rounded-3xl shadow-2xl transform rotate-y-[-12deg] rotate-x-[5deg] hover:rotate-0 transition-transform duration-700 ease-out">
                    <div class="flex justify-between items-center mb-8">
                        <div>
                            <h3 class="text-2xl font-bold text-white">Swing Alert 🚀</h3>
                            <p class="text-emerald-500 text-sm font-bold">High Probability Setup</p>
                        </div>
                        <div class="bg-slate-950 px-3 py-1 rounded-lg border border-slate-800 text-xs font-mono text-slate-400">
                            Sent: 10:30 AM
                        </div>
                    </div>

                    <div class="space-y-6">
                        <div class="bg-slate-950 p-4 rounded-xl border-l-4 border-emerald-500">
                            <div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Action</div>
                            <div class="text-lg font-bold text-white flex items-center gap-2">
                                <span class="bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded text-sm">BUY</span>
                                NVDA 480 CALLS
                            </div>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
                                <div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Entry Zone</div>
                                <div class="text-xl font-mono font-bold text-white">$5.50 - $6.00</div>
                            </div>
                            <div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
                                <div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Target</div>
                                <div class="text-xl font-mono font-bold text-emerald-400">$8.50+</div>
                            </div>
                        </div>

                        <div class="bg-slate-950 p-4 rounded-xl border border-red-500/30">
                             <div class="flex justify-between items-center">
                                <div class="text-xs text-slate-500 uppercase tracking-wider">Invalidation (Stop)</div>
                                <div class="text-red-400 font-mono font-bold">$4.20 (Hard Stop)</div>
                             </div>
                        </div>
                    </div>

                    <div class="absolute -right-6 -bottom-6 bg-emerald-500 text-slate-950 p-4 rounded-2xl shadow-xl shadow-emerald-500/20 animate-bounce">
                        <div class="text-xs font-bold opacity-80 uppercase">Potential Return</div>
                        <div class="text-2xl font-extrabold">+45%</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="bg-slate-900 border-y border-slate-800 relative z-20">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <dl class="grid grid-cols-2 md:grid-cols-4 gap-8">
                <div class="text-center">
                    <dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">Historical Win Rate</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-emerald-500">82%</dd>
                </div>
                <div class="text-center">
                    <dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">Avg Hold Time</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-white">3-7<span class="text-lg text-slate-500 font-normal ml-1">days</span></dd>
                </div>
                <div class="text-center">
                    <dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">Risk/Reward</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-indigo-400">4:1</dd>
                </div>
                <div class="text-center">
                    <dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">Alerts Per Week</dt>
                    <dd class="text-4xl md:text-5xl font-extrabold text-blue-400">2-4</dd>
                </div>
            </dl>
        </div>
    </section>

    <section class="py-24 bg-slate-950 relative overflow-hidden">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-white mb-6">
                    Choose Your Lifestyle
                </h2>
                <p use:reveal={{ delay: 100 }} class="text-xl text-slate-400 max-w-2xl mx-auto">
                    Most traders burn out scalping 1-minute candles. We play the bigger timeframe.
                </p>
            </div>

            <div class="grid md:grid-cols-2 gap-8 max-w-5xl mx-auto">
                <div use:reveal={{ delay: 0 }} class="bg-slate-900/50 border border-slate-800 rounded-3xl p-10 opacity-70 hover:opacity-100 transition-opacity">
                    <div class="flex items-center gap-4 mb-6">
                        <div class="w-12 h-12 rounded-full bg-slate-800 flex items-center justify-center text-2xl">😰</div>
                        <h3 class="text-2xl font-bold text-slate-400">Day Scalper</h3>
                    </div>
                    <ul class="space-y-4 text-slate-500">
                        <li class="flex items-center gap-3"><span class="text-red-500">✕</span> Glued to screen 6 hours/day</li>
                        <li class="flex items-center gap-3"><span class="text-red-500">✕</span> High stress, high cortisol</li>
                        <li class="flex items-center gap-3"><span class="text-red-500">✕</span> Expensive commissions</li>
                        <li class="flex items-center gap-3"><span class="text-red-500">✕</span> "Did I miss the move?" anxiety</li>
                    </ul>
                </div>

                <div use:reveal={{ delay: 150 }} class="bg-slate-900 border-2 border-emerald-500 rounded-3xl p-10 shadow-2xl shadow-emerald-500/10 relative overflow-hidden">
                    <div class="absolute top-0 right-0 bg-emerald-500 text-slate-900 text-xs font-bold px-3 py-1 rounded-bl-xl">RECOMMENDED</div>
                    <div class="flex items-center gap-4 mb-6">
                        <div class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center text-2xl">🧘‍♂️</div>
                        <h3 class="text-2xl font-bold text-white">Swing Trader</h3>
                    </div>
                    <ul class="space-y-4 text-slate-200 font-medium">
                        <li class="flex items-center gap-3"><svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg> Check charts once a day</li>
                        <li class="flex items-center gap-3"><svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg> Calm, calculated decisions</li>
                        <li class="flex items-center gap-3"><svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg> Catch the meat of the move (20%+)</li>
                        <li class="flex items-center gap-3"><svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg> Keep your day job</li>
                    </ul>
                </div>
            </div>
        </div>
    </section>

    <section id="process" class="py-24 bg-slate-900 border-t border-slate-800">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid md:grid-cols-3 gap-10">
                <article use:reveal={{ delay: 0 }} class="group">
                    <div class="w-14 h-14 bg-emerald-500/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-emerald-500/20">
                        <svg class="w-7 h-7 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-3">Institutional Analysis</h3>
                    <p class="text-slate-400 leading-relaxed">
                        We track dark pool prints and institutional flow to identify stocks about to break out. We ride the whale's wake.
                    </p>
                </article>

                <article use:reveal={{ delay: 100 }} class="group">
                    <div class="w-14 h-14 bg-blue-500/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-blue-500/20">
                        <svg class="w-7 h-7 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-3">SMS & Push Alerts</h3>
                    <p class="text-slate-400 leading-relaxed">
                        You can't miss the entry. We send alerts via SMS, Email, and Discord immediately when our criteria are met.
                    </p>
                </article>

                <article use:reveal={{ delay: 200 }} class="group">
                    <div class="w-14 h-14 bg-indigo-500/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-indigo-500/20">
                        <svg class="w-7 h-7 text-indigo-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" /></svg>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-3">Risk-First Approach</h3>
                    <p class="text-slate-400 leading-relaxed">
                        We hate losing money. Every trade comes with a predefined "Hard Stop" level. We cut losers fast and let winners run.
                    </p>
                </article>
            </div>
        </div>
    </section>

    <section class="py-24 bg-slate-950">
        <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="mb-10">
                <h2 class="text-3xl font-heading font-bold text-white mb-2">Recent Swings</h2>
                <p class="text-slate-500">Real trades. Real timestamps. Verified results.</p>
            </div>

            <div class="bg-slate-900 rounded-2xl border border-slate-800 overflow-hidden shadow-xl">
                <table class="w-full text-left border-collapse">
                    <thead>
                        <tr class="border-b border-slate-800 text-xs font-bold uppercase text-slate-500 tracking-wider">
                            <th class="p-4 md:p-5">Ticker</th>
                            <th class="p-4 md:p-5">Type</th>
                            <th class="p-4 md:p-5">Days Held</th>
                            <th class="p-4 md:p-5 text-right">Return</th>
                            <th class="p-4 md:p-5 hidden md:table-cell text-right">Notes</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800 font-mono text-sm">
                        <tr class="hover:bg-white/5 transition-colors">
                            <td class="p-4 md:p-5 font-bold text-white">NVDA</td>
                            <td class="p-4 md:p-5 text-emerald-400">CALLS</td>
                            <td class="p-4 md:p-5 text-slate-400">5 Days</td>
                            <td class="p-4 md:p-5 text-right text-emerald-400 font-bold">+125%</td>
                            <td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs">Breakout over $480 level.</td>
                        </tr>
                        <tr class="hover:bg-white/5 transition-colors">
                            <td class="p-4 md:p-5 font-bold text-white">AMD</td>
                            <td class="p-4 md:p-5 text-emerald-400">CALLS</td>
                            <td class="p-4 md:p-5 text-slate-400">3 Days</td>
                            <td class="p-4 md:p-5 text-right text-emerald-400 font-bold">+45%</td>
                            <td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs">Sector rotation play.</td>
                        </tr>
                        <tr class="hover:bg-white/5 transition-colors bg-red-500/5">
                            <td class="p-4 md:p-5 font-bold text-white">TSLA</td>
                            <td class="p-4 md:p-5 text-red-400">PUTS</td>
                            <td class="p-4 md:p-5 text-slate-400">1 Day</td>
                            <td class="p-4 md:p-5 text-right text-red-400 font-bold">-15%</td>
                            <td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs">Hit stop loss on reversal.</td>
                        </tr>
                        <tr class="hover:bg-white/5 transition-colors">
                            <td class="p-4 md:p-5 font-bold text-white">META</td>
                            <td class="p-4 md:p-5 text-emerald-400">CALLS</td>
                            <td class="p-4 md:p-5 text-slate-400">7 Days</td>
                            <td class="p-4 md:p-5 text-right text-emerald-400 font-bold">+82%</td>
                            <td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs">Earnings run-up swing.</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </section>

    <section id="pricing" class="py-24 bg-slate-900 border-t border-slate-800">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-white mb-6">Simple Pricing</h2>
                <p class="text-xl text-slate-400 max-w-3xl mx-auto">
                    Pay for the subscription with your first successful swing trade.
                </p>
            </div>

            <div class="flex justify-center mb-16">
                <div class="bg-slate-950 p-1.5 rounded-xl border border-slate-800 inline-flex relative">
                    <button
                        type="button"
                        on:click={() => (billingInterval = 'monthly')}
                        class="relative z-10 px-8 py-3 rounded-lg font-bold text-sm md:text-base transition-colors duration-200 {billingInterval === 'monthly' ? 'text-white' : 'text-slate-500 hover:text-white'}"
                    >
                        Monthly
                    </button>
                    <button
                        type="button"
                        on:click={() => (billingInterval = 'annual')}
                        class="relative z-10 px-8 py-3 rounded-lg font-bold text-sm md:text-base transition-colors duration-200 {billingInterval === 'annual' ? 'text-white' : 'text-slate-500 hover:text-white'}"
                    >
                        Annual
                    </button>
                    <div 
                        class="absolute top-1.5 bottom-1.5 bg-emerald-500 rounded-lg shadow-lg shadow-emerald-500/20 transition-all duration-300 ease-out"
                        style="left: {billingInterval === 'monthly' ? '0.375rem' : '50%'}; width: calc(50% - 0.375rem);"
                    ></div>
                </div>
            </div>

            <div class="max-w-lg mx-auto">
                <div class="bg-slate-950 p-8 md:p-10 rounded-3xl shadow-2xl border-2 border-emerald-500 relative overflow-hidden transition-all duration-300">
                    
                    {#if activePlan.savings}
                        <div transition:fade class="absolute top-0 right-0 bg-emerald-500 text-slate-900 px-4 py-1 rounded-bl-xl font-bold text-xs uppercase tracking-wide">
                            {activePlan.savings}
                        </div>
                    {/if}

                    <div class="text-center mb-8">
                        <h3 class="text-2xl font-bold text-white mb-2 capitalize">{billingInterval} Pass</h3>
                        
                        <div class="flex items-baseline justify-center gap-1 h-20 items-end pb-2">
                            <span class="text-6xl font-extrabold text-white tracking-tight">
                                ${activePlan.price}
                            </span>
                            <span class="text-slate-500 font-medium text-lg">{activePlan.period}</span>
                        </div>
                        
                        <p class="text-emerald-500 font-bold text-sm mt-2 min-h-[1.25rem]">
                            {billingInterval === 'annual' ? 'Basically 2.5 months FREE' : 'Flexibility to cancel anytime'}
                        </p>
                    </div>

                    <ul class="space-y-5 mb-10 text-sm md:text-base">
                        <li class="flex items-center gap-3"><div class="bg-emerald-500/20 p-1 rounded-full">{@html IconCheckSvg}</div> <span class="text-slate-200">2-4 Premium Swings / Week</span></li>
                        <li class="flex items-center gap-3"><div class="bg-emerald-500/20 p-1 rounded-full">{@html IconCheckSvg}</div> <span class="text-slate-200">Instant SMS & Email Alerts</span></li>
                        <li class="flex items-center gap-3"><div class="bg-emerald-500/20 p-1 rounded-full">{@html IconCheckSvg}</div> <span class="text-slate-200">Private Discord Community</span></li>
                        <li class="flex items-center gap-3"><div class="bg-emerald-500/20 p-1 rounded-full">{@html IconCheckSvg}</div> <span class="text-slate-200">Detailed Technical Analysis</span></li>
                        {#if billingInterval === 'annual'}
                            <li transition:slide class="flex items-center gap-3"><div class="bg-emerald-500/20 p-1 rounded-full">{@html IconCheckSvg}</div> <span class="text-emerald-400 font-bold">Strategy Video Library</span></li>
                        {/if}
                    </ul>

                    <a href={activePlan.link} class="w-full bg-emerald-500 text-slate-900 py-4 rounded-xl font-bold text-lg hover:bg-emerald-400 transition-all shadow-lg shadow-emerald-500/25 text-center block transform hover:-translate-y-1">
                        {activePlan.btnText}
                    </a>
                </div>
            </div>

             <p class="text-center text-slate-500 text-sm mt-12 flex items-center justify-center gap-2">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
                30-Day Money Back Guarantee. Cancel anytime.
            </p>
        </div>
    </section>

    <section class="py-20 bg-slate-950">
        <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 class="text-3xl font-heading font-bold text-center text-white mb-12">Frequently Asked Questions</h2>
            <div class="space-y-4">
                {#each faqData as item, i}
                    <div class="border border-slate-800 rounded-xl bg-slate-900 overflow-hidden">
                        <button 
                            type="button"
                            class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-slate-200"
                            on:click={() => toggleFaq(i)}
                            aria-expanded={openFaq === i}
                        >
                            {item.q}
                            <svg class="w-5 h-5 text-slate-500 transform transition-transform duration-300 {openFaq === i ? 'rotate-180' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
                        </button>
                        {#if openFaq === i}
                            <div transition:slide={{ duration: 300, easing: cubicOut }} class="px-6 pb-6 text-slate-400 text-sm leading-relaxed border-t border-slate-800/50 pt-4">
                                {item.a}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="py-24 bg-gradient-to-br from-emerald-600 to-teal-800 text-white relative overflow-hidden">
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
            <h2 class="text-4xl md:text-6xl font-heading font-extrabold mb-6 tracking-tight">Stop Overtrading. Start Swinging.</h2>
            <p class="text-xl text-emerald-100 mb-10 max-w-2xl mx-auto">
                Join the trading room that values your time as much as your capital.
            </p>
            <a
                href="#pricing"
                class="inline-block bg-white text-emerald-800 px-10 py-5 rounded-xl font-bold text-lg hover:bg-emerald-50 transition-all shadow-2xl hover:-translate-y-1"
            >
                Get Instant Access
            </a>
            <p class="mt-6 text-sm text-emerald-100/70">Secure Checkout • Cancel Anytime</p>
        </div>
    </section>

    <footer class="bg-slate-950 py-12 border-t border-slate-900">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-slate-600 leading-relaxed">
            <p class="mb-4 font-bold uppercase text-slate-500">Risk Disclosure</p>
            <p class="max-w-4xl mx-auto">
                Trading in financial markets involves a high degree of risk and may not be suitable for all investors. You could lose some or all of your initial investment; do not invest money that you cannot afford to lose. Past performance is not indicative of future results. Revolution Trading Pros is an educational platform and does not provide personalized financial advice.
            </p>
            <p class="mt-8 opacity-50">&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.</p>
        </div>
    </footer>

</main>