<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import { spring } from 'svelte/motion';
    import SEOHead from '$lib/components/SEOHead.svelte';

    /**
     * Data: Trading Rooms (Upgraded with Technical IDs for Custom Icons)
     */
    const rooms = [
        {
            id: 'day-trading',
            iconType: 'volatility', // Triggers Candlestick Animation
            name: 'Day Trading Command',
            tagline: 'Trade the Open with Professionals',
            description: 'High-velocity execution. Join expert traders for the opening bell, real-time scanners, and rapid-fire trade calls.',
            liveCount: 842,
            features: [
                'Daily Live Session (9:30 AM - 12:00 PM ET)',
                'Institutional Order Flow Analysis',
                'Screen Share & Voice Comms',
                'Gap Scanner & Pre-Market Prep',
                'Private Discord Community'
            ],
            price: { monthly: 247, quarterly: 597, annual: 1897 },
            accent: 'cyan',
            badge: 'MOST POPULAR'
        },
        {
            id: 'swing-trading',
            iconType: 'trend', // Triggers Sine Wave Animation
            name: 'Swing Alpha Room',
            tagline: 'Catch Multi-Day Moves',
            description: 'Strategic positioning for 3-7 day holds. Perfect for those who cannot watch the screen all day but want institutional returns.',
            liveCount: 1250,
            features: [
                'Weekly Deep-Dive Strategy Session',
                'Sunday Night Watchlist Blueprint',
                'Risk/Reward Position Sizing',
                'Private Analyst Chat',
                'Weekend Market Prep'
            ],
            price: { monthly: 197, quarterly: 497, annual: 1497 },
            accent: 'emerald',
            badge: ''
        },
        {
            id: 'small-accounts',
            iconType: 'growth', // Triggers Step-Chart Animation
            name: 'Growth Accelerator',
            tagline: 'Small Account → Big Future',
            description: 'The disciplined path to $25K and beyond. Learn risk management and compounding strategies specifically for smaller capital bases.',
            liveCount: 430,
            features: [
                'Under $25K Specific Strategies',
                'Strict Risk Management Rules',
                'Account Builder Roadmap',
                'Live Mentorship Q&A',
                'Community Support'
            ],
            price: { monthly: 147, quarterly: 397, annual: 1197 },
            accent: 'amber',
            badge: 'BEGINNER FRIENDLY'
        }
    ];

    /**
     * Data: Why Join Us (Upgraded with Technical IDs)
     */
    const benefits = [
        {
            iconType: 'analysis',
            title: 'Institutional Mentorship',
            desc: 'Learn from professionals with decades of combined experience at major firms and prop desks.'
        },
        {
            iconType: 'radar',
            title: 'Sub-Second Alerts',
            desc: 'Get trade alerts as they happen via SMS, email, and Discord. Never miss a breakout.'
        },
        {
            iconType: 'strategy',
            title: 'Continuous Education',
            desc: 'Access a library of strategy breakdowns, market analysis, and psychological training.'
        },
        {
            iconType: 'network',
            title: 'Elite Community',
            desc: 'Join thousands of focused traders who help each other succeed, share alpha, and grow.'
        }
    ];

    /**
     * Data: Ticker Symbols
     */
    const symbols = [
        { sym: 'SPY', price: '478.22', change: '+0.45%', up: true },
        { sym: 'QQQ', price: '408.12', change: '+0.82%', up: true },
        { sym: 'IWM', price: '198.40', change: '-0.12%', up: false },
        { sym: 'NVDA', price: '492.11', change: '+2.30%', up: true },
        { sym: 'TSLA', price: '245.50', change: '-1.20%', up: false },
        { sym: 'AMD', price: '138.00', change: '+1.05%', up: true },
        { sym: 'BTC', price: '42,100', change: '+1.2%', up: true },
    ];
    const tickerItems = [...symbols, ...symbols, ...symbols, ...symbols];

    /**
     * Action: 3D Tilt Effect
     */
    function tilt(node: HTMLElement) {
        const x = spring(0, { stiffness: 0.05, damping: 0.25 });
        const y = spring(0, { stiffness: 0.05, damping: 0.25 });

        const unsubX = x.subscribe(v => node.style.setProperty('--rotX', `${v}deg`));
        const unsubY = y.subscribe(v => node.style.setProperty('--rotY', `${v}deg`));

        function handleMove(e: MouseEvent) {
            const rect = node.getBoundingClientRect();
            x.set(((e.clientY - rect.top - rect.height / 2) / rect.height) * -3);
            y.set(((e.clientX - rect.left - rect.width / 2) / rect.width) * 3);
        }

        function handleLeave() {
            x.set(0);
            y.set(0);
        }

        node.addEventListener('mousemove', handleMove);
        node.addEventListener('mouseleave', handleLeave);

        return {
            destroy() {
                node.removeEventListener('mousemove', handleMove);
                node.removeEventListener('mouseleave', handleLeave);
                unsubX();
                unsubY();
            }
        };
    }

    /**
     * Animation Controller
     */
    let heroRef: HTMLElement;
    let gridRef: HTMLElement;
    let benefitsRef: HTMLElement;
    let ctaRef: HTMLElement;

    onMount(() => {
        if (!browser) return;
        
        let ScrollTrigger: typeof import('gsap/dist/ScrollTrigger').ScrollTrigger;
        
        // IIFE pattern for async operations with cleanup
        (async () => {
            // Dynamic GSAP import for SSR safety
            const gsapModule = await import('gsap');
            const scrollTriggerModule = await import('gsap/dist/ScrollTrigger');
            const gsap = gsapModule.gsap;
            ScrollTrigger = scrollTriggerModule.ScrollTrigger;
            gsap.registerPlugin(ScrollTrigger);
            
            const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

            // Hero Sequence
            tl.fromTo('.hero-badge', 
                { y: -20, opacity: 0, scale: 0.9 }, 
                { y: 0, opacity: 1, scale: 1, duration: 0.8 }
            )
            .fromTo('.hero-title span', 
                { y: 100, opacity: 0, rotateX: 20 }, 
                { y: 0, opacity: 1, rotateX: 0, stagger: 0.1, duration: 1 }, 
                '-=0.4'
            )
            .fromTo('.hero-desc', 
                { y: 20, opacity: 0 }, 
                { y: 0, opacity: 1, duration: 0.8 }, 
                '-=0.6'
            )
            .fromTo(gridRef?.children || [], 
                { y: 60, opacity: 0, filter: 'blur(10px)' }, 
                { y: 0, opacity: 1, filter: 'blur(0px)', stagger: 0.15, duration: 0.8 }, 
                '-=0.4'
            );

            // Benefits Section Scroll Trigger
            if (benefitsRef?.children) {
                gsap.fromTo(benefitsRef.children, 
                    { y: 40, opacity: 0 },
                    {
                        y: 0, opacity: 1, stagger: 0.1, duration: 0.8,
                        scrollTrigger: {
                            trigger: benefitsRef,
                            start: 'top 80%',
                            toggleActions: 'play none none reverse'
                        }
                    }
                );
            }

            // Final CTA Scroll Trigger
            if (ctaRef) {
                gsap.fromTo(ctaRef,
                    { scale: 0.95, opacity: 0 },
                    {
                        scale: 1, opacity: 1, duration: 0.8,
                        scrollTrigger: {
                            trigger: ctaRef,
                            start: 'top 85%'
                        }
                    }
                );
            }
        })();
        
        // Cleanup on unmount
        return () => {
            if (ScrollTrigger) {
                ScrollTrigger.getAll().forEach(t => t.kill());
            }
        };
    });
</script>

<SEOHead
    title="Live Trading Rooms | Revolution Trading Pros"
    description="Join our live trading rooms. Day trading, swing trading, and small account strategies with real-time alerts."
/>

<div class="min-h-screen bg-[#050505] text-white selection:bg-blue-500/30 overflow-x-hidden font-sans relative">
    
    <div class="fixed inset-0 pointer-events-none z-0">
        <div class="absolute top-[-20%] left-[-10%] w-[60%] h-[60%] bg-blue-900/10 rounded-full blur-[120px] mix-blend-screen animate-pulse-slow"></div>
        <div class="absolute bottom-[-20%] right-[-10%] w-[60%] h-[60%] bg-emerald-900/10 rounded-full blur-[120px] mix-blend-screen animate-pulse-slow" style="animation-delay: 2s"></div>
        <div class="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]"></div>
    </div>

    <div class="relative z-20 border-b border-white/5 bg-[#050505]/80 backdrop-blur-md h-10 flex items-center overflow-hidden">
        <div class="ticker-track flex items-center gap-12 whitespace-nowrap px-4">
            {#each tickerItems as item}
                <div class="flex items-center gap-3 text-xs font-mono select-none">
                    <span class="font-bold text-zinc-300">{item.sym}</span>
                    <span class="text-zinc-500">{item.price}</span>
                    <span class={item.up ? 'text-emerald-400' : 'text-rose-400'}>
                        {item.change}
                    </span>
                </div>
            {/each}
        </div>
    </div>

    <main class="relative z-10 pt-20 pb-0 container mx-auto px-4 sm:px-6 lg:px-8">
        
        <section bind:this={heroRef} class="text-center max-w-5xl mx-auto mb-24">
            <div class="hero-badge inline-flex items-center gap-2.5 px-4 py-1.5 rounded-full bg-white/5 border border-white/10 backdrop-blur-sm mb-8 shadow-[0_0_20px_rgba(16,185,129,0.15)]">
                <span class="relative flex h-2.5 w-2.5">
                    <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                    <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
                </span>
                <span class="text-xs font-mono uppercase tracking-widest text-emerald-400 font-bold">Markets are Open</span>
            </div>

            <h1 class="hero-title text-5xl md:text-7xl lg:text-8xl font-bold tracking-tight leading-[1.1] mb-8 text-transparent bg-clip-text bg-gradient-to-b from-white via-white to-white/40">
                <span class="block">Trade Together.</span>
                <span class="block bg-gradient-to-r from-blue-400 to-emerald-400 bg-clip-text text-transparent">Win Together.</span>
            </h1>

            <p class="hero-desc text-lg md:text-xl text-zinc-400 max-w-2xl mx-auto leading-relaxed">
                Step inside our professional trading floors. Real-time data, rapid execution, and a community that wins together.
            </p>
        </section>

        <div bind:this={gridRef} id="rooms-section" class="grid grid-cols-1 md:grid-cols-3 gap-6 lg:gap-8 perspective-container mb-32">
            {#each rooms as room}
                <article use:tilt class="group relative h-full card-3d" role="region" aria-label={room.name}>
                    <div class={`absolute -inset-[1px] rounded-3xl bg-gradient-to-b opacity-0 group-hover:opacity-100 transition-opacity duration-500 blur-sm 
                        ${room.accent === 'cyan' ? 'from-cyan-500/50' : ''}
                        ${room.accent === 'emerald' ? 'from-emerald-500/50' : ''}
                        ${room.accent === 'amber' ? 'from-amber-500/50' : ''}
                        to-transparent`}>
                    </div>

                    <div class="relative h-full flex flex-col bg-[#0A0A0A] border border-white/5 hover:border-white/10 rounded-3xl p-1 shadow-2xl overflow-hidden transition-colors duration-300">
                        <div class="flex-1 flex flex-col p-6 lg:p-8 rounded-[20px] bg-gradient-to-b from-white/[0.02] to-transparent">
                            
                            {#if room.badge}
                                <div class="absolute top-6 right-6">
                                    <span class={`inline-block px-2 py-1 rounded text-[10px] font-bold uppercase tracking-wider border
                                        ${room.accent === 'cyan' ? 'text-cyan-300 bg-cyan-500/10 border-cyan-500/20' : ''}
                                        ${room.accent === 'emerald' ? 'text-emerald-300 bg-emerald-500/10 border-emerald-500/20' : ''}
                                        ${room.accent === 'amber' ? 'text-amber-300 bg-amber-500/10 border-amber-500/20' : ''}
                                    `}>
                                        {room.badge}
                                    </span>
                                </div>
                            {/if}

                            <div class={`w-14 h-14 rounded-xl flex items-center justify-center mb-6 transition-all duration-300 ease-out border border-white/5
                                ${room.accent === 'cyan' ? 'bg-cyan-500/5 text-cyan-400' : ''}
                                ${room.accent === 'emerald' ? 'bg-emerald-500/5 text-emerald-400' : ''}
                                ${room.accent === 'amber' ? 'bg-amber-500/5 text-amber-400' : ''}
                            `}>
                                {#if room.iconType === 'volatility'}
                                    <svg class="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                        <path class="group-hover:animate-candle-1" d="M6 6v12" stroke-linecap="round"/>
                                        <rect class="group-hover:animate-candle-body-1" x="4" y="9" width="4" height="6" fill="currentColor" fill-opacity="0.2"/>
                                        <path class="group-hover:animate-candle-2" d="M12 4v16" stroke-linecap="round"/>
                                        <rect class="group-hover:animate-candle-body-2" x="10" y="7" width="4" height="10" fill="currentColor" fill-opacity="0.2"/>
                                        <path class="group-hover:animate-candle-3" d="M18 8v8" stroke-linecap="round"/>
                                        <rect class="group-hover:animate-candle-body-3" x="16" y="10" width="4" height="4" fill="currentColor" fill-opacity="0.2"/>
                                    </svg>
                                {:else if room.iconType === 'trend'}
                                    <svg class="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                        <path class="group-hover:animate-draw-line" stroke-dasharray="30" stroke-dashoffset="30" d="M3 12c0-3 3-6 6-6s6 3 6 6 3 6 6 6" stroke-linecap="round"/>
                                        <circle cx="21" cy="18" r="2" fill="currentColor" class="opacity-0 group-hover:opacity-100 transition-opacity delay-300"/>
                                    </svg>
                                {:else if room.iconType === 'growth'}
                                    <svg class="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                        <path d="M3 20h18" stroke-linecap="round"/>
                                        <path class="group-hover:animate-grow-1" d="M5 20v-4" stroke-linecap="round"/>
                                        <path class="group-hover:animate-grow-2" d="M9 20v-8" stroke-linecap="round"/>
                                        <path class="group-hover:animate-grow-3" d="M13 20v-12" stroke-linecap="round"/>
                                        <path class="group-hover:animate-grow-4" d="M17 20v-16" stroke-linecap="round"/>
                                        <path d="M17 4l2 2" stroke-linecap="round"/>
                                    </svg>
                                {/if}
                            </div>

                            <h2 class="text-2xl font-bold text-white mb-2">{room.name}</h2>
                            <p class="text-sm font-medium text-zinc-400 mb-4">{room.tagline}</p>
                            
                            <div class="flex items-center gap-2 mb-6 text-xs font-mono text-zinc-500">
                                <span class="relative flex h-2 w-2">
                                    <span class={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75
                                        ${room.accent === 'cyan' ? 'bg-cyan-400' : ''}
                                        ${room.accent === 'emerald' ? 'bg-emerald-400' : ''}
                                        ${room.accent === 'amber' ? 'bg-amber-400' : ''}
                                    `}></span>
                                    <span class={`relative inline-flex rounded-full h-2 w-2 
                                        ${room.accent === 'cyan' ? 'bg-cyan-500' : ''}
                                        ${room.accent === 'emerald' ? 'bg-emerald-500' : ''}
                                        ${room.accent === 'amber' ? 'bg-amber-500' : ''}
                                    `}></span>
                                </span>
                                {room.liveCount} Traders Online
                            </div>

                            <p class="text-sm leading-relaxed text-zinc-400 mb-8 border-t border-white/5 pt-6">{room.description}</p>

                            <ul class="space-y-3 mb-8 flex-1">
                                {#each room.features as feature}
                                    <li class="flex items-start gap-3 text-sm text-zinc-300">
                                        <svg class={`w-4 h-4 shrink-0 mt-[3px] 
                                            ${room.accent === 'cyan' ? 'text-cyan-500/80' : ''}
                                            ${room.accent === 'emerald' ? 'text-emerald-500/80' : ''}
                                            ${room.accent === 'amber' ? 'text-amber-500/80' : ''}
                                        `} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                        {feature}
                                    </li>
                                {/each}
                            </ul>

                            <div class="mt-auto">
                                <div class="flex flex-col gap-1 mb-6">
                                    <div class="flex items-end gap-1">
                                        <span class="text-3xl font-bold text-white">${room.price.monthly}</span>
                                        <span class="text-zinc-500 text-sm mb-1">/ month</span>
                                    </div>
                                    <div class="flex items-center justify-between text-xs text-zinc-500 mt-2">
                                        <span>Annual: ${room.price.annual}/yr</span>
                                        <span class="text-emerald-400 font-medium">Save 36%</span>
                                    </div>
                                </div>
                                
                                <a href="/live-trading-rooms/{room.id}" class={`group/btn relative w-full flex items-center justify-center gap-2 py-4 rounded-xl text-black font-bold text-sm transition-all duration-300 overflow-hidden
                                        ${room.accent === 'cyan' ? 'bg-white hover:bg-cyan-400' : ''}
                                        ${room.accent === 'emerald' ? 'bg-white hover:bg-emerald-400' : ''}
                                        ${room.accent === 'amber' ? 'bg-white hover:bg-amber-400' : ''}
                                        hover:shadow-[0_0_20px_rgba(255,255,255,0.3)]
                                    `}>
                                    <span class="relative z-10">Access Room</span>
                                    <svg class="w-4 h-4 relative z-10 transition-transform group-hover/btn:translate-x-1" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14m-7-7l7 7-7 7"/></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </article>
            {/each}
        </div>

        <section class="py-24 border-t border-white/5 relative">
            <div class="absolute inset-0 bg-blue-500/5 blur-[100px] pointer-events-none"></div>
            
            <div class="text-center mb-16 relative z-10">
                <h2 class="text-3xl md:text-5xl font-bold mb-6 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/60">
                    Why the Pros Choose Us
                </h2>
                <p class="text-zinc-400 max-w-2xl mx-auto">
                    We don't just sell courses. We build institutional-grade traders through immersion, technology, and community.
                </p>
            </div>

            <div bind:this={benefitsRef} class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                {#each benefits as item}
                    <div class="p-6 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 hover:border-white/10 transition-colors duration-300 text-center group cursor-default">
                        <div class="mx-auto w-12 h-12 mb-6 text-zinc-400 group-hover:text-blue-400 transition-colors duration-300">
                            {#if item.iconType === 'analysis'}
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                    <circle cx="12" cy="12" r="10" stroke-dasharray="4 4" class="group-hover:animate-spin-slow"/>
                                    <path d="M12 2v20M2 12h20" class="opacity-30"/>
                                    <circle cx="12" cy="12" r="3" class="group-hover:scale-125 transition-transform"/>
                                </svg>
                            {:else if item.iconType === 'radar'}
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                                    <path class="group-hover:animate-pulse" d="M12 8v4l3 3" stroke-linecap="round"/>
                                </svg>
                            {:else if item.iconType === 'strategy'}
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
                                    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
                                    <path class="group-hover:animate-draw-line" stroke-dasharray="10" stroke-dashoffset="10" d="M9 8h6M9 12h4" stroke-linecap="round"/>
                                </svg>
                            {:else if item.iconType === 'network'}
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                    <circle cx="12" cy="5" r="2" class="group-hover:fill-current"/>
                                    <circle cx="5" cy="19" r="2" class="group-hover:fill-current transition-colors delay-100"/>
                                    <circle cx="19" cy="19" r="2" class="group-hover:fill-current transition-colors delay-200"/>
                                    <path d="M12 7l-7 12M12 7l7 12M5 19h14" class="opacity-50"/>
                                </svg>
                            {/if}
                        </div>

                        <h3 class="text-lg font-bold text-white mb-3">{item.title}</h3>
                        <p class="text-sm text-zinc-400 leading-relaxed">{item.desc}</p>
                    </div>
                {/each}
            </div>
        </section>

        <section bind:this={ctaRef} class="py-24 pb-32 text-center relative overflow-hidden rounded-3xl my-12 bg-gradient-to-b from-blue-900/20 to-black border border-white/10">
            <div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
            
            <div class="relative z-10 max-w-3xl mx-auto px-4">
                <h2 class="text-4xl md:text-5xl font-bold text-white mb-6">Ready to Level Up?</h2>
                <p class="text-xl text-zinc-400 mb-10">
                    Join thousands of traders who have transformed their results. <br class="hidden md:block"/>
                    The market is waiting. Your desk is ready.
                </p>
                
                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                    <a href="#rooms-section" class="px-8 py-4 bg-white text-black font-bold rounded-xl hover:bg-blue-50 hover:scale-105 transition-all duration-300 shadow-[0_0_40px_rgba(255,255,255,0.3)]">
                        Choose Your Room
                    </a>
                    <a href="/about" class="px-8 py-4 bg-transparent border border-white/20 text-white font-bold rounded-xl hover:bg-white/5 hover:border-white/40 transition-all duration-300">
                        Talk to an Advisor
                    </a>
                </div>
            </div>
        </section>

        <div class="text-center border-t border-white/5 pt-16 pb-8">
            <h3 class="text-zinc-600 text-xs font-mono uppercase tracking-[0.2em] mb-8">Trusted by 10,000+ Traders Worldwide</h3>
            <div class="flex flex-wrap justify-center gap-12 opacity-30 grayscale hover:grayscale-0 transition-all duration-500">
                <div class="h-6 w-20 bg-white/20 rounded-sm"></div>
                <div class="h-6 w-20 bg-white/20 rounded-sm"></div>
                <div class="h-6 w-20 bg-white/20 rounded-sm"></div>
                <div class="h-6 w-20 bg-white/20 rounded-sm"></div>
            </div>
        </div>

    </main>
</div>

<style>
    /* 3D Physics */
    .card-3d {
        transform-style: preserve-3d;
        transform: perspective(1000px) rotateX(var(--rotX, 0deg)) rotateY(var(--rotY, 0deg));
        will-change: transform;
    }

    /* Ambient Animation */
    .animate-pulse-slow {
        animation: pulse 8s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }
    @keyframes pulse {
        0%, 100% { opacity: 0.1; }
        50% { opacity: 0.2; }
    }

    /* Ticker Animation */
    .ticker-track {
        animation: scroll 60s linear infinite;
    }
    .ticker-track:hover {
        animation-play-state: paused;
    }
    @keyframes scroll {
        0% { transform: translateX(0); }
        100% { transform: translateX(-50%); }
    }

    /* Custom SVG Animations */
    @keyframes draw-line {
        to { stroke-dashoffset: 0; }
    }
    .animate-draw-line {
        animation: draw-line 1s ease-out forwards;
    }

    @keyframes spin-slow {
        to { transform: rotate(360deg); transform-origin: center; }
    }
    .animate-spin-slow {
        animation: spin-slow 8s linear infinite;
    }

    /* Candle Animations */
    @keyframes candle-up {
        0%, 100% { transform: scaleY(1); }
        50% { transform: scaleY(1.2); }
    }
    .animate-candle-body-1 { transform-origin: bottom; animation: candle-up 2s infinite ease-in-out; }
    .animate-candle-body-2 { transform-origin: bottom; animation: candle-up 3s infinite ease-in-out 0.5s; }
    .animate-candle-body-3 { transform-origin: bottom; animation: candle-up 2.5s infinite ease-in-out 0.2s; }

    /* Growth Bar Animations */
    @keyframes grow-up {
        from { transform: scaleY(0); }
        to { transform: scaleY(1); }
    }
    .animate-grow-1 { transform-origin: bottom; animation: grow-up 0.4s ease-out forwards; }
    .animate-grow-2 { transform-origin: bottom; animation: grow-up 0.4s ease-out 0.1s forwards; }
    .animate-grow-3 { transform-origin: bottom; animation: grow-up 0.4s ease-out 0.2s forwards; }
    .animate-grow-4 { transform-origin: bottom; animation: grow-up 0.4s ease-out 0.3s forwards; }

    .perspective-container {
        perspective: 2000px;
    }
</style>