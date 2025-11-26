<script lang="ts">
    import { onMount } from 'svelte';
    import { cubicOut } from 'svelte/easing';
    import { browser } from '$app/environment';
    import IconBuildingBank from '@tabler/icons-svelte/icons/building-bank';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconShieldLock from '@tabler/icons-svelte/icons/shield-lock';
    import IconUsersGroup from '@tabler/icons-svelte/icons/users-group';
    import IconChartDots from '@tabler/icons-svelte/icons/chart-dots';
    import IconScale from '@tabler/icons-svelte/icons/scale';
    import IconBriefcase from '@tabler/icons-svelte/icons/briefcase';
    import IconId from '@tabler/icons-svelte/icons/id';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- Animation Logic ---
    let containerRef: HTMLElement;
    let isVisible = false;
    let mouse = { x: 0, y: 0 };

    const handleMouseMove = (e: MouseEvent) => {
        if (!containerRef) return;
        const rect = containerRef.getBoundingClientRect();
        mouse.x = e.clientX - rect.left;
        mouse.y = e.clientY - rect.top;
    };

    onMount(() => {
        if (!browser) return;
        const observer = new IntersectionObserver(
            (entries) => {
                if (entries[0].isIntersecting) {
                    isVisible = true;
                    observer.disconnect();
                }
            },
            { threshold: 0.1 }
        );
        if (containerRef) observer.observe(containerRef);
    });

    function heavySlide(node: Element, { delay = 0, duration = 1000 }) {
        return {
            delay,
            duration,
            css: (t: number) => {
                const eased = cubicOut(t);
                return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px);`;
            }
        };
    }

    // --- Data ---
    const stats = [
        { value: '10,000+', label: 'Active Terminals', icon: IconUsersGroup },
        { value: '85%', label: 'Win Probability', icon: IconTrendingUp },
        { value: '$50M+', label: 'Capital Deployed', icon: IconBuildingBank },
        { value: '24/5', label: 'Desk Coverage', icon: IconShieldLock }
    ];

    const principles = [
        {
            icon: IconScale,
            title: 'Asymmetric Risk',
            id: 'PRIN-01',
            description: 'We do not engage in 1:1 setups. Every position must offer a mathematical expectancy of 3R or greater based on historical variance.'
        },
        {
            icon: IconShieldLock,
            title: 'Variance Control',
            id: 'PRIN-02',
            description: 'Capital preservation is the mandate. We utilize institutional hedging and strict position sizing to flatten equity curve volatility.'
        },
        {
            icon: IconChartDots,
            title: 'Quantitative Edge',
            id: 'PRIN-03',
            description: 'Opinions are irrelevant. We trade order flow, dark pool liquidity, and gamma exposure levels derived from raw market data.'
        }
    ];

    const team = [
        {
            name: 'Michael Rodriguez',
            role: 'Head of Trading',
            id: 'DESK-LEAD',
            specialties: ['Derivatives', 'Macro Flow'],
            bio: '15+ years institutional experience. Former Goldman Sachs derivatives trader specializing in SPX volatility structures.'
        },
        {
            name: 'Sarah Chen',
            role: 'Chief Strategist',
            id: 'QUANT-01',
            specialties: ['Algo Systems', 'HFT Logic'],
            bio: 'PhD in Financial Engineering (MIT). Builds the proprietary algorithms that power our signal detection engine.'
        },
        {
            name: 'David Thompson',
            role: 'Education Director',
            id: 'EDU-LEAD',
            specialties: ['Technical Analysis', 'Market Profile'],
            bio: 'Certified Financial Technician (CFTe). 12 years experience bridging the gap between retail theory and institutional reality.'
        },
        {
            name: 'Jessica Martinez',
            role: 'Risk Manager',
            id: 'RISK-DIR',
            specialties: ['Portfolio Ops', 'Compliance'],
            bio: 'Former JP Morgan Risk Analyst. Ensures all desk strategies adhere to strict drawdown and VaR limits.'
        }
    ];

    // Preserving Schema from original code
    const aboutSchema = [
        {
            '@context': 'https://schema.org',
            '@type': 'AboutPage',
            name: 'Firm Profile | Revolution Trading Pros',
            description: 'Institutional trading education and proprietary data tools.',
            mainEntity: {
                '@type': 'Organization',
                name: 'Revolution Trading Pros',
                foundingDate: '2018'
            }
        }
    ];
</script>

<SEOHead
    title="Firm Profile | Revolution Trading Pros"
    description="Revolution Trading Pros bridges the gap between retail capital and institutional edge. Established 2018."
    canonical="/about"
    schema={aboutSchema}
/>

<div 
    bind:this={containerRef}
    on:mousemove={handleMouseMove}
    role="main"
    aria-label="About Revolution Trading Pros"
    class="relative bg-[#020202] min-h-screen text-slate-400 font-sans selection:bg-amber-900 selection:text-white"
>
    <div class="fixed inset-0 pointer-events-none">
        <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem]"></div>
        <div class="absolute inset-0 opacity-20"
             style="background: radial-gradient(1000px circle at var(--x) var(--y), rgba(255,255,255,0.05), transparent 60%);">
        </div>
    </div>

    <main class="relative z-10 pt-32 pb-24 px-6 lg:px-8">
        
        <section class="max-w-[1600px] mx-auto mb-32">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="flex flex-col lg:flex-row gap-16 lg:items-end border-b border-white/10 pb-16">
                    <div class="lg:w-2/3">
                        <div class="inline-flex items-center gap-3 px-3 py-1 border border-white/10 bg-white/5 text-slate-400 text-[10px] font-bold tracking-[0.3em] uppercase mb-8">
                            <span class="w-2 h-2 bg-amber-600 rounded-sm"></span>
                            Est. 2018
                        </div>
                        
                        <h1 class="text-6xl lg:text-8xl font-serif text-white tracking-tight leading-[0.9] mb-8">
                            Strategic <br />
                            <span class="text-slate-700">Mandate.</span>
                        </h1>
                        
                        <p class="text-xl text-slate-400 font-light leading-relaxed max-w-2xl border-l-2 border-amber-600/50 pl-6">
                            We are not a "community." We are a performance-driven trading environment. 
                            Our mission is to arm retail capital with the infrastructure, data, and discipline of an institutional desk.
                        </p>
                    </div>

                    <div class="lg:w-1/3 grid grid-cols-2 gap-px bg-white/10 border border-white/10">
                        {#each stats as stat}
                            <div class="bg-[#050505] p-6 group hover:bg-[#080808] transition-colors">
                                <div class="text-amber-600 mb-2 opacity-50 group-hover:opacity-100 transition-opacity">
                                    <svelte:component this={stat.icon} size={20} />
                                </div>
                                <div class="text-2xl font-serif text-white mb-1">{stat.value}</div>
                                <div class="text-[10px] font-mono uppercase text-slate-500 tracking-widest">{stat.label}</div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </section>

        <section class="max-w-[1400px] mx-auto mb-32 grid lg:grid-cols-2 gap-24 items-center">
            {#if isVisible}
                <div in:heavySlide={{ delay: 200 }} class="order-2 lg:order-1 relative">
                    <svg class="w-full h-full text-slate-800" viewBox="0 0 400 300" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M0 280 L50 250 L100 260 L150 180 L200 200 L250 100 L300 120 L350 50 L400 20" stroke="currentColor" stroke-width="2" vector-effect="non-scaling-stroke" />
                        <path d="M0 280 L50 250 L100 260 L150 180 L200 200 L250 100 L300 120 L350 50 L400 20 V 300 H 0 Z" fill="url(#grad)" opacity="0.2" />
                        <defs>
                            <linearGradient id="grad" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="0%" stop-color="#d97706" />
                                <stop offset="100%" stop-color="transparent" />
                            </linearGradient>
                        </defs>
                    </svg>
                    <div class="absolute top-10 left-10 bg-[#080808] border border-white/10 p-6 shadow-2xl max-w-xs">
                        <div class="text-[10px] font-mono text-emerald-500 mb-2">MARKET GAP IDENTIFIED</div>
                        <p class="text-sm text-slate-300 font-light">
                            "Retail traders are being sold dreams while institutions are sold data. We built the bridge."
                        </p>
                    </div>
                </div>

                <div in:heavySlide={{ delay: 300 }} class="order-1 lg:order-2">
                    <h2 class="text-4xl font-serif text-white mb-8">The Institutional Gap.</h2>
                    <div class="space-y-6 text-lg font-light leading-relaxed text-slate-400">
                        <p>
                            Revolution Trading Pros was founded in 2018 by former proprietary traders who identified a critical failure in the market: <strong class="text-white font-medium">Information Asymmetry.</strong>
                        </p>
                        <p>
                            While retail traders relied on lagging indicators and "guru" alerts, professional desks operated with Order Flow, Vanna, and Dark Pool data. The playing field wasn't just uneven; it was broken.
                        </p>
                        <p>
                            We built this firm to democratize edge. We do not sell signals. We install a professional operating system into your trading business.
                        </p>
                    </div>
                </div>
            {/if}
        </section>

        <section class="max-w-[1600px] mx-auto mb-32">
            {#if isVisible}
                <div in:heavySlide={{ delay: 400 }} class="border-t border-white/10 pt-16">
                    <div class="flex justify-between items-end mb-12">
                        <h2 class="text-3xl font-serif text-white">Operating Principles</h2>
                        <span class="text-[10px] font-mono text-slate-600 uppercase tracking-widest hidden sm:block">Firm Protocol v4.0</span>
                    </div>

                    <div class="grid md:grid-cols-3 gap-8">
                        {#each principles as prin}
                            <div class="group bg-[#050505] border border-white/10 p-8 hover:border-amber-600/50 transition-colors duration-500">
                                <div class="flex justify-between items-start mb-6">
                                    <div class="p-3 bg-white/5 text-amber-600 group-hover:bg-amber-600 group-hover:text-black transition-colors duration-300">
                                        <svelte:component this={prin.icon} size={24} stroke={1.5} />
                                    </div>
                                    <span class="text-[10px] font-mono text-slate-600 group-hover:text-amber-600 transition-colors">{prin.id}</span>
                                </div>
                                <h3 class="text-xl font-medium text-white mb-4">{prin.title}</h3>
                                <p class="text-sm text-slate-400 leading-relaxed font-light">
                                    {prin.description}
                                </p>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </section>

        <section class="max-w-[1600px] mx-auto">
            {#if isVisible}
                <div in:heavySlide={{ delay: 500 }} class="border-t border-white/10 pt-16">
                    <div class="flex justify-between items-end mb-12">
                        <h2 class="text-3xl font-serif text-white">Desk Personnel</h2>
                        <span class="text-[10px] font-mono text-slate-600 uppercase tracking-widest hidden sm:block">Active Roster</span>
                    </div>

                    <div class="grid gap-px bg-white/10 border border-white/10">
                        {#each team as member}
                            <div class="group bg-[#050505] p-8 md:p-10 grid md:grid-cols-12 gap-8 items-center hover:bg-[#080808] transition-colors duration-300">
                                <div class="md:col-span-2 flex items-center gap-4">
                                    <div class="w-12 h-12 bg-white/5 flex items-center justify-center text-slate-500 border border-white/5 group-hover:border-white/20">
                                        <IconId size={24} />
                                    </div>
                                    <div class="md:hidden text-lg font-serif text-white">{member.name}</div>
                                </div>
                                
                                <div class="hidden md:block md:col-span-3">
                                    <div class="text-xl font-serif text-white">{member.name}</div>
                                    <div class="text-[10px] font-mono text-amber-600 uppercase tracking-widest">{member.role}</div>
                                </div>

                                <div class="md:col-span-5">
                                    <div class="md:hidden text-[10px] font-mono text-amber-600 uppercase tracking-widest mb-2">{member.role}</div>
                                    <p class="text-sm text-slate-400 leading-relaxed font-light">{member.bio}</p>
                                </div>

                                <div class="md:col-span-2 flex flex-col items-end gap-2">
                                    {#each member.specialties as spec}
                                        <span class="px-2 py-1 bg-white/5 border border-white/5 text-[10px] text-slate-400 font-mono uppercase rounded">
                                            {spec}
                                        </span>
                                    {/each}
                                    <div class="flex items-center gap-2 mt-2">
                                        <span class="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse"></span>
                                        <span class="text-[10px] font-mono text-emerald-500 uppercase">Active</span>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </section>

        <section class="mt-32 border-t border-white/10 pt-20 text-center">
            <h2 class="text-3xl md:text-5xl font-serif text-white mb-8">Initialize Protocol.</h2>
            <div class="flex justify-center gap-6">
                <a href="/signup" class="group flex items-center gap-2 px-8 py-4 bg-amber-700 hover:bg-amber-600 text-white font-bold text-sm uppercase tracking-widest transition-all">
                    <span>Access Terminal</span>
                    <IconArrowRight size={16} class="group-hover:translate-x-1 transition-transform" />
                </a>
            </div>
        </section>

    </main>
</div>