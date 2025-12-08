<script lang="ts">
    import { onMount } from 'svelte';
    import { cubicOut, quintOut } from 'svelte/easing';
    import { fade, fly, draw } from 'svelte/transition';
    import { browser } from '$app/environment';
    
    // Icons
    import IconBuildingBank from '@tabler/icons-svelte/icons/building-bank';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconShieldLock from '@tabler/icons-svelte/icons/shield-lock';
    import IconUsersGroup from '@tabler/icons-svelte/icons/users-group';
    import IconChartDots from '@tabler/icons-svelte/icons/chart-dots';
    import IconScale from '@tabler/icons-svelte/icons/scale';
    import IconId from '@tabler/icons-svelte/icons/id';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconBroadcast from '@tabler/icons-svelte/icons/broadcast';
    import IconSchool from '@tabler/icons-svelte/icons/school';
    import IconMessageCircle from '@tabler/icons-svelte/icons/message-circle';
    import IconCheck from '@tabler/icons-svelte/icons/check';
    import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
    import IconActivity from '@tabler/icons-svelte/icons/activity';

    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- Animation Logic (Svelte 5 Runes) ---
    let containerRef: HTMLElement | undefined = $state();
    let isVisible = $state(false);
    let mouse = $state({ x: 0, y: 0 });

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

    // Kept original function exactly as requested
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
        { value: '1,200+', label: 'Active Members', icon: IconUsersGroup },
        { value: 'Daily', label: 'Live Guidance', icon: IconBroadcast },
        { value: '100%', label: 'Transparency', icon: IconShieldLock },
        { value: '24/7', label: 'Support Chat', icon: IconMessageCircle }
    ];

    const features = [
        {
            icon: IconBroadcast,
            title: 'Live Trading Floor',
            subtitle: 'Not just alerts. Real-time education.',
            description: 'We don’t just post a ticker and disappear. We share our screens, talk through the setup via voice chat, and explain the "Why" before the trade happens. You see the wins, the losses, and the management in real-time.'
        },
        {
            icon: IconSchool,
            title: 'Institutional Education',
            subtitle: 'Master the mechanics of the market.',
            description: 'Stop relying on lagging indicators. Our curriculum teaches you how to read Order Flow, Dark Pool data, and Gamma Exposure—the actual data institutions use to move price.'
        },
        {
            icon: IconUsersGroup,
            title: 'Mentorship & Community',
            subtitle: 'You are no longer trading alone.',
            description: 'Trading is the loneliest profession in the world—until now. Join a desk of serious traders who support each other, share alpha, and pick you up when you have a red day.'
        }
    ];

    const team = [
        {
            name: 'Michael Rodriguez',
            role: 'Head Trader & Mentor',
            id: 'DESK-LEAD',
            specialties: ['Live Calling', 'Psychology'],
            bio: 'Michael isn’t just an ex-Goldman trader; he’s a teacher. He leads the morning voice session every day, guiding members through volatility with a calm, professional demeanor focused on risk management.'
        },
        {
            name: 'Sarah Chen',
            role: 'Quant Strategist',
            id: 'QUANT-01',
            specialties: ['Data Analysis', 'Systems'],
            bio: 'Sarah translates complex institutional data into actionable levels for our members. She builds the tools that help you see where the "Smart Money" is hiding.'
        },
        {
            name: 'David Thompson',
            role: 'Education Director',
            id: 'EDU-LEAD',
            specialties: ['Curriculum', 'Market Structure'],
            bio: 'David specializes in fixing bad habits. His "Retail Rehab" sessions help traders unlearn the mistakes taught by fake gurus and replace them with professional process.'
        }
    ];

    const faqs = [
        {
            q: "Is this suitable for beginners?",
            a: "Yes, but be prepared to work. We don't sell 'get rich quick' schemes. We provide a Foundation Course that brings you up to speed on our terminology and tools before you jump into the live room."
        },
        {
            q: "Do you just give buy/sell alerts?",
            a: "No. We are an educational community. While we call out our own live trades, the goal is for you to understand the *rationale* so you can eventually become self-sufficient. Blindly following alerts is a recipe for failure."
        },
        {
            q: "What makes this different from other Discords?",
            a: "Professionalism and care. We don't tolerate hype, rocket emojis, or pumping low-float stocks. This is a serious workspace for people who treat trading as a business, run by mentors who actually care if you succeed."
        },
        {
            q: "Do I need a large account to start?",
            a: "No. We teach risk management based on percentages, not dollar amounts. Many members start with small accounts or funded trader evaluations while they learn the system."
        }
    ];

    // Schema Logic
    const organizationSchema = {
        '@context': 'https://schema.org',
        '@type': 'Organization',
        name: 'Revolution Trading Pros',
        description: 'Professional trading community and mentorship program.',
        foundingDate: '2018',
        url: 'https://revolutiontradingpros.com',
        sameAs: ['https://twitter.com/revolutiontradingpros'] // Example
    };
    
    const faqSchema = {
        '@context': 'https://schema.org',
        '@type': 'FAQPage',
        mainEntity: faqs.map(f => ({
            '@type': 'Question',
            name: f.q,
            acceptedAnswer: {
                '@type': 'Answer',
                text: f.a
            }
        }))
    };

    // --- New Visual Logic ---
    // Fake Ticker Data for Visual Ambience
    const tickerItems = ["ES_F +0.25%", "NQ_F -0.10%", "RTY_F +1.20%", "GC_F +0.05%", "CL_F -0.50%", "DX_F +0.12%", "VIX -4.5%"];
</script>

<SEOHead
    title="Revolution Trading Pros | The #1 Supportive Live Trading Community"
    description="Join a professional trading floor that genuinely cares. Real trades, real-time voice guidance, and institutional data without the hype. Established 2018."
    canonical="/"
    schema={[organizationSchema, faqSchema]}
/>

<svelte:head>
    <style>
        .ticker-wrap {
            width: 100%;
            overflow: hidden;
            white-space: nowrap;
            mask-image: linear-gradient(to right, transparent, black 10%, black 90%, transparent);
        }
        .ticker-move {
            display: inline-block;
            animation: ticker 30s linear infinite;
        }
        @keyframes ticker {
            0% { transform: translate3d(0, 0, 0); }
            100% { transform: translate3d(-50%, 0, 0); }
        }
        .scanner-line {
            animation: scan 4s ease-in-out infinite;
            transform-origin: bottom;
        }
        @keyframes scan {
            0% { transform: translateX(0); opacity: 0; }
            10% { opacity: 1; }
            90% { opacity: 1; }
            100% { transform: translateX(350px); opacity: 0; }
        }
        /* Spotlight Effect Utility */
        .spotlight-card::before {
            content: "";
            position: absolute;
            top: 0; left: 0; right: 0; bottom: 0;
            background: radial-gradient(800px circle at var(--x) var(--y), rgba(255,255,255,0.06), transparent 40%);
            z-index: 0;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.5s;
        }
        .spotlight-card:hover::before { opacity: 1; }
    </style>
</svelte:head>

<div 
    bind:this={containerRef}
    onmousemove={handleMouseMove}
    role="main"
    class="relative bg-[#020202] min-h-screen text-slate-400 font-sans selection:bg-amber-900/40 selection:text-amber-100 overflow-x-hidden"
    style="--x: {mouse.x}px; --y: {mouse.y}px;"
>
    <div class="fixed inset-0 pointer-events-none z-0">
        <div class="absolute inset-0 opacity-[0.03] mix-blend-overlay" style="background-image: url('data:image/svg+xml,%3Csvg viewBox=%220 0 200 200%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cfilter id=%22noiseFilter%22%3E%3CfeTurbulence type=%22fractalNoise%22 baseFrequency=%220.65%22 numOctaves=%223%22 stitchTiles=%22stitch%22/%3E%3C/filter%3E%3Crect width=%22100%25%22 height=%22100%25%22 filter=%22url(%23noiseFilter)%22/%3E%3C/svg%3E');"></div>
        
        <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_60%_at_50%_50%,#000_10%,transparent_100%)]"></div>
        
        <div class="absolute inset-0 opacity-40"
             style="background: radial-gradient(600px circle at var(--x) var(--y), rgba(217,119,6,0.05), transparent 60%);">
        </div>
    </div>

    <div class="relative z-20 border-b border-white/5 bg-[#020202]/80 backdrop-blur-md h-10 flex items-center">
        <div class="ticker-wrap text-[10px] font-mono uppercase tracking-widest text-slate-500">
            <div class="ticker-move">
                {#each [...tickerItems, ...tickerItems, ...tickerItems, ...tickerItems] as item}
                    <span class="inline-block px-8">
                        <span class={item.includes('+') ? 'text-emerald-500' : 'text-rose-500'}>●</span> {item}
                    </span>
                {/each}
            </div>
        </div>
    </div>

    <main id="main-content" class="relative z-10 pt-24 pb-24 px-6 lg:px-8">
        
        <section class="max-w-[1600px] mx-auto mb-32 lg:mb-48">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="flex flex-col lg:flex-row gap-16 lg:items-end border-b border-white/10 pb-16 relative">
                    <div class="absolute -top-4 -left-4 w-8 h-8 border-t border-l border-amber-800/50"></div>
                    <div class="absolute -bottom-4 -right-4 w-8 h-8 border-b border-r border-amber-800/50"></div>

                    <div class="lg:w-2/3 relative z-10">
                        <div class="inline-flex items-center gap-3 px-4 py-1.5 border border-emerald-900/30 bg-emerald-950/10 text-emerald-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-8 rounded-full shadow-[0_0_15px_rgba(16,185,129,0.1)]">
                            <span class="relative flex h-2 w-2">
                              <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                              <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                            </span>
                            Live Trading Floor Open
                        </div>
                        
                        <h1 class="text-6xl sm:text-7xl lg:text-9xl font-serif text-white tracking-tighter leading-[0.9] mb-10 mix-blend-screen">
                            Trade Real <span class="text-transparent bg-clip-text bg-gradient-to-r from-slate-200 to-slate-500">Capital.</span><br />
                            <span class="text-amber-600 drop-shadow-[0_0_15px_rgba(217,119,6,0.4)]">Real Mentors.</span><br />
                            <span class="text-slate-700">Real Time.</span>
                        </h1>
                        
                        <p class="text-xl text-slate-400 font-light leading-relaxed max-w-2xl border-l-2 border-amber-600/50 pl-6 mb-12 bg-gradient-to-r from-white/5 to-transparent py-2">
                            Stop guessing. Join a professional trading floor where we share screens, explain the 'why' behind every move, and fight for profitability together. No hindsight. No hiding.
                        </p>

                        <div class="flex flex-col sm:flex-row gap-4">
                            <a href="/join" class="group relative inline-flex justify-center items-center gap-3 px-10 py-5 bg-gradient-to-r from-amber-700 to-amber-800 text-white font-bold text-xs uppercase tracking-[0.2em] transition-all overflow-hidden border border-amber-600/50 hover:border-amber-500">
                                <span class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300"></span>
                                <span class="relative z-10">Join The Community</span>
                                <IconArrowRight size={16} class="relative z-10 group-hover:translate-x-1 transition-transform" />
                            </a>
                            <a href="#how-it-works" class="group inline-flex justify-center items-center gap-2 px-10 py-5 border border-white/10 hover:bg-white/5 text-slate-300 font-bold text-xs uppercase tracking-[0.2em] transition-all backdrop-blur-sm">
                                <span class="group-hover:text-white transition-colors">See How It Works</span>
                            </a>
                        </div>
                    </div>

                    <div class="lg:w-1/3 grid grid-cols-2 gap-px bg-white/5 border border-white/10 backdrop-blur-sm shadow-2xl">
                        {#each stats as stat, i}
                            {@const Icon = stat.icon}
                            <div class="bg-[#050505]/80 p-6 group hover:bg-[#0A0A0A] transition-colors relative overflow-hidden">
                                <div class="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-amber-600/50 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-1000"></div>
                                <div class="text-amber-600 mb-3 opacity-60 group-hover:opacity-100 transition-opacity group-hover:scale-110 duration-300 origin-left">
                                    <Icon size={24} stroke={1.5} />
                                </div>
                                <div class="text-3xl font-serif text-white mb-1 tabular-nums tracking-tight">{stat.value}</div>
                                <div class="text-[9px] font-mono uppercase text-slate-500 tracking-[0.2em]">{stat.label}</div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </section>

        <section class="max-w-[1400px] mx-auto mb-32 lg:mb-48 grid lg:grid-cols-2 gap-24 items-center">
            {#if isVisible}
                <div in:heavySlide={{ delay: 200 }} class="order-2 lg:order-1 relative group perspective-1000">
                    <div class="relative w-full aspect-[4/3] bg-[#050505] border border-white/10 rounded-sm overflow-hidden shadow-2xl transform transition-transform duration-700 group-hover:rotate-y-2">
                        <div class="absolute inset-0 bg-[linear-gradient(0deg,rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:2rem_2rem]"></div>
                        
                        <svg class="absolute inset-0 w-full h-full text-amber-500" viewBox="0 0 400 300" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path 
                                d="M0 280 L50 250 L100 260 L150 180 L200 200 L250 100 L300 120 L350 50 L400 20" 
                                stroke="currentColor" 
                                stroke-width="2" 
                                stroke-linecap="round"
                                vector-effect="non-scaling-stroke"
                                in:draw={{ duration: 3000, easing: cubicOut }} 
                            />
                            <path d="M0 280 L50 250 L100 260 L150 180 L200 200 L250 100 L300 120 L350 50 L400 20 V 300 H 0 Z" fill="url(#grad)" opacity="0.2" in:fade={{ delay: 1000, duration: 2000 }} />
                            <defs>
                                <linearGradient id="grad" x1="0" y1="0" x2="0" y2="1">
                                    <stop offset="0%" stop-color="#d97706" />
                                    <stop offset="100%" stop-color="transparent" />
                                </linearGradient>
                            </defs>
                        </svg>

                        <div class="absolute top-0 bottom-0 w-[1px] bg-white/50 scanner-line shadow-[0_0_10px_rgba(255,255,255,0.5)]"></div>
                        
                        <div class="absolute top-10 left-10 bg-[#080808]/90 border border-red-500/30 p-6 shadow-2xl max-w-xs backdrop-blur-md">
                            <div class="flex items-center gap-2 mb-3">
                                <IconActivity size={14} class="text-red-500 animate-pulse" />
                                <div class="text-[9px] font-mono text-red-500 uppercase tracking-widest">The Retail Trap</div>
                            </div>
                            <p class="text-sm text-slate-300 font-light italic leading-relaxed">
                                "Most traders fail not because they lack intelligence, but because they lack support, structure, and emotional control."
                            </p>
                        </div>
                    </div>
                </div>

                <div in:heavySlide={{ delay: 300 }} class="order-1 lg:order-2">
                    <h2 class="text-4xl lg:text-5xl font-serif text-white mb-8 tracking-tight">You Shouldn't Have to <br /><span class="text-amber-600 italic">Trade Alone.</span></h2>
                    <div class="space-y-6 text-lg font-light leading-relaxed text-slate-400">
                        <p>
                            The hardest part of trading isn't reading a chart. It's the psychological battle of sitting alone in a room, making high-stakes decisions with no one to guide you.
                        </p>
                        <p class="pl-4 border-l-2 border-white/20">
                            <strong class="text-white font-medium">Revolution Trading Pros changed the model.</strong> We replaced the "Guru Alert" system with a true "Virtual Trading Floor."
                        </p>
                        <p>
                            When you join us, you aren't just buying data. You are plugging into a collective intelligence. You get mentors who check your risk, a community that keeps you accountable, and a system that prioritizes long-term survival over gambling.
                        </p>
                        <ul class="space-y-4 mt-8">
                            {#each ['No pump and dumps', 'No fake P&L screenshots', 'Real-time voice commentary'] as item}
                                <li class="flex items-center gap-4 text-sm text-amber-500 font-mono uppercase tracking-wider group cursor-default">
                                    <div class="p-1 rounded bg-amber-500/10 border border-amber-500/20 group-hover:bg-amber-500 group-hover:text-black transition-colors">
                                        <IconCheck size={14} stroke={3} />
                                    </div>
                                    {item}
                                </li>
                            {/each}
                        </ul>
                    </div>
                </div>
            {/if}
        </section>

        <section id="how-it-works" class="max-w-[1600px] mx-auto mb-32 lg:mb-48">
            {#if isVisible}
                <div in:heavySlide={{ delay: 400 }} class="border-t border-white/10 pt-20">
                    <div class="flex flex-col md:flex-row justify-between items-end mb-20 gap-6">
                        <div>
                            <span class="text-amber-600 font-mono text-[10px] tracking-[0.3em] uppercase mb-4 block flex items-center gap-2">
                                <span class="w-8 h-[1px] bg-amber-600"></span> The Ecosystem
                            </span>
                            <h2 class="text-4xl md:text-6xl font-serif text-white tracking-tight">Everything You Need<br />To Succeed.</h2>
                        </div>
                        <p class="text-slate-500 text-sm max-w-md md:text-right hidden md:block leading-relaxed border-r-2 border-amber-600 pr-6">
                            A complete operating system for your trading business, <br /> from data to psychology.
                        </p>
                    </div>

                    <div class="grid md:grid-cols-3 gap-6">
                        {#each features as feat, i}
                            {@const Icon = feat.icon}
                            <div class="group spotlight-card bg-[#050505] border border-white/10 p-10 hover:border-amber-600/30 transition-all duration-500 relative overflow-hidden flex flex-col h-full rounded-sm">
                                <div class="absolute -top-6 -right-6 text-white opacity-[0.02] group-hover:opacity-[0.05] transition-opacity duration-700 pointer-events-none transform group-hover:scale-110 group-hover:-rotate-12">
                                    <Icon size={200} />
                                </div>
                                
                                <div class="flex justify-between items-start mb-8 relative z-10">
                                    <div class="p-4 bg-white/5 border border-white/5 text-amber-600 group-hover:bg-amber-600 group-hover:text-black transition-all duration-300 rounded-sm shadow-lg">
                                        <Icon size={28} stroke={1.5} />
                                    </div>
                                    <span class="font-mono text-[10px] text-slate-700">0{i + 1}</span>
                                </div>
                                
                                <h3 class="text-2xl font-serif text-white mb-3 relative z-10 group-hover:text-amber-500 transition-colors">{feat.title}</h3>
                                <div class="w-12 h-[1px] bg-amber-600/50 mb-4 group-hover:w-full transition-all duration-700"></div>
                                <p class="text-[10px] font-mono text-amber-600/80 uppercase tracking-widest mb-6 relative z-10 min-h-[1.5em]">{feat.subtitle}</p>
                                <p class="text-sm text-slate-400 leading-relaxed font-light relative z-10 mt-auto">
                                    {feat.description}
                                </p>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </section>

        <section class="max-w-[1600px] mx-auto mb-32 lg:mb-48">
            {#if isVisible}
                <div in:heavySlide={{ delay: 500 }} class="border-t border-white/10 pt-20">
                    <div class="mb-16 flex flex-col md:flex-row md:items-end justify-between gap-8">
                        <div>
                            <h2 class="text-4xl font-serif text-white mb-4">Meet Your Mentors.</h2>
                            <p class="text-slate-400 max-w-2xl font-light text-lg">
                                We aren't anonymous admins. We are real people who are in the chat every day.
                            </p>
                        </div>
                        <div class="hidden md:flex gap-2">
                             {#each team as _}<div class="w-2 h-2 rounded-full bg-white/20"></div>{/each}
                        </div>
                    </div>

                    <div class="grid gap-px bg-white/5 border border-white/10 overflow-hidden rounded-sm">
                        {#each team as member}
                            <div class="group bg-[#050505] p-8 md:p-12 grid md:grid-cols-12 gap-8 items-center hover:bg-[#080808] transition-colors duration-300 relative">
                                <div class="absolute left-0 top-0 bottom-0 w-1 bg-amber-600 transform -translate-x-full group-hover:translate-x-0 transition-transform duration-300"></div>

                                <div class="md:col-span-2 flex items-center gap-6">
                                    <div class="w-16 h-16 rounded-full bg-gradient-to-br from-white/10 to-transparent flex items-center justify-center text-slate-400 border border-white/5 group-hover:border-amber-600/50 group-hover:text-amber-500 overflow-hidden shadow-2xl transition-all">
                                        <IconId size={32} stroke={1.5} />
                                    </div>
                                    <div class="md:hidden">
                                        <div class="text-xl font-serif text-white">{member.name}</div>
                                        <div class="text-[10px] font-mono text-slate-500 uppercase">{member.role}</div>
                                    </div>
                                </div>
                                
                                <div class="hidden md:block md:col-span-3">
                                    <div class="text-2xl font-serif text-white group-hover:text-amber-500 transition-colors tracking-tight">{member.name}</div>
                                    <div class="text-[10px] font-mono text-slate-500 uppercase tracking-widest mt-1 flex items-center gap-2">
                                        {member.role} <span class="text-white/10">|</span> <span class="text-white/20">{member.id}</span>
                                    </div>
                                </div>

                                <div class="md:col-span-5">
                                    <p class="text-sm text-slate-400 leading-relaxed font-light border-l border-white/10 pl-4">{member.bio}</p>
                                </div>

                                <div class="md:col-span-2 flex flex-col items-end gap-3">
                                    {#each member.specialties as spec}
                                        <span class="px-3 py-1.5 bg-white/5 border border-white/5 text-[9px] text-slate-300 font-mono uppercase tracking-wider rounded-sm hover:border-amber-600/50 hover:text-amber-500 transition-colors cursor-default whitespace-nowrap">
                                            {spec}
                                        </span>
                                    {/each}
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </section>

        <section class="max-w-[1200px] mx-auto mb-32 lg:mb-48 px-4">
             {#if isVisible}
                <div in:heavySlide={{ delay: 600 }} class="text-center mb-20 relative">
                    <div class="absolute left-1/2 -translate-x-1/2 top-1/2 -translate-y-1/2 w-64 h-64 bg-amber-600/20 blur-[100px] rounded-full pointer-events-none"></div>
                    <h2 class="text-3xl font-serif text-white mb-4 relative z-10">Why Traders Stay.</h2>
                    <p class="text-slate-500 text-[10px] font-mono uppercase tracking-[0.3em] relative z-10">Community Feedback</p>
                </div>

                <div in:heavySlide={{ delay: 700 }} class="grid md:grid-cols-2 gap-8">
                    <div class="bg-[#080808]/80 backdrop-blur p-10 border border-white/5 rounded-sm relative hover:border-amber-600/30 transition-colors duration-500">
                        <div class="absolute -top-4 -left-4 text-amber-900/20"><IconMessageCircle size={80} /></div>
                        
                        <div class="relative z-10">
                             <div class="flex gap-1 mb-6 text-amber-600">
                                 {#each Array(5) as _}<IconScale size={12} class="fill-current" />{/each}
                             </div>
                            <p class="text-lg text-slate-300 font-light italic mb-8 leading-relaxed">
                                "I spent years jumping from one alert service to another, losing money. Revolution is different. They actually taught me how to fish. I finally feel in control of my risk."
                            </p>
                            <div class="flex items-center gap-4 border-t border-white/5 pt-6">
                                <div class="w-10 h-10 bg-gradient-to-tr from-slate-800 to-slate-700 rounded-full flex items-center justify-center text-xs font-bold text-white">MK</div>
                                <div>
                                    <div class="text-sm text-white font-medium">Verified Member</div>
                                    <div class="text-[10px] text-slate-500 font-mono uppercase">Member since 2021</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-[#080808]/80 backdrop-blur p-10 border border-white/5 rounded-sm relative hover:border-amber-600/30 transition-colors duration-500">
                         <div class="absolute -top-4 -left-4 text-amber-900/20"><IconMessageCircle size={80} /></div>
                        
                        <div class="relative z-10">
                            <div class="flex gap-1 mb-6 text-amber-600">
                                 {#each Array(5) as _}<IconScale size={12} class="fill-current" />{/each}
                             </div>
                            <p class="text-lg text-slate-300 font-light italic mb-8 leading-relaxed">
                                "The morning voice chat is a game changer. Hearing Michael explain his thought process in real-time kept me out of so many bad trades. It's like having a risk manager over your shoulder."
                            </p>
                            <div class="flex items-center gap-4 border-t border-white/5 pt-6">
                                <div class="w-10 h-10 bg-gradient-to-tr from-slate-800 to-slate-700 rounded-full flex items-center justify-center text-xs font-bold text-white">JT</div>
                                <div>
                                    <div class="text-sm text-white font-medium">Verified Member</div>
                                    <div class="text-[10px] text-slate-500 font-mono uppercase">Member since 2023</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
             {/if}
        </section>

        <section class="max-w-[900px] mx-auto mb-32">
             {#if isVisible}
                <div in:heavySlide={{ delay: 800 }} class="border-t border-white/10 pt-20">
                    <div class="text-center mb-16">
                         <h2 class="text-4xl font-serif text-white mb-2">Frequently Asked Questions</h2>
                         <div class="h-1 w-20 bg-amber-600 mx-auto rounded-full"></div>
                    </div>
                    
                    <div class="space-y-4">
                        {#each faqs as faq}
                            <details class="group bg-[#050505] border border-white/10 open:border-amber-600/30 transition-all duration-300 rounded-sm overflow-hidden">
                                <summary class="flex justify-between items-center p-6 cursor-pointer list-none bg-white/[0.02] hover:bg-white/[0.04] transition-colors">
                                    <span class="text-lg text-slate-200 font-light pr-8">{faq.q}</span>
                                    <span class="text-amber-600 transform group-open:rotate-180 transition-transform duration-300">
                                        <IconChevronDown size={20} />
                                    </span>
                                </summary>
                                <div class="px-6 pb-8 text-slate-400 font-light leading-relaxed border-t border-white/5 pt-6 bg-[#020202]">
                                    {faq.a}
                                </div>
                            </details>
                        {/each}
                    </div>
                </div>
             {/if}
        </section>

        <section class="mt-32 border-t border-white/10 pt-32 pb-32 text-center relative overflow-hidden">
            <div class="absolute inset-0 bg-amber-600/5 blur-[120px] pointer-events-none"></div>
            <div class="absolute top-0 left-0 w-full h-px bg-gradient-to-r from-transparent via-amber-600/50 to-transparent"></div>

            <div class="relative z-10 px-4">
                <IconBuildingBank size={48} class="mx-auto text-amber-600 mb-8 opacity-80" stroke={1} />
                <h2 class="text-5xl md:text-7xl font-serif text-white mb-8 tracking-tighter">Ready to <span class="italic text-amber-600">Turn Pro?</span></h2>
                <p class="text-slate-400 text-xl mb-12 max-w-xl mx-auto font-light leading-relaxed">
                    The market is open. The team is ready. <br />The only thing missing is you.
                </p>
                <div class="flex flex-col sm:flex-row justify-center gap-6">
                    <a href="/join" class="group relative flex justify-center items-center gap-4 px-12 py-6 bg-amber-700 text-white font-bold text-sm uppercase tracking-[0.2em] transition-all shadow-[0_0_40px_rgba(180,83,9,0.2)] hover:shadow-[0_0_60px_rgba(180,83,9,0.4)] overflow-hidden">
                        <span class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-700 ease-in-out"></span>
                        <span class="relative z-10">Start Your Membership</span>
                        <IconArrowRight size={18} class="relative z-10 group-hover:translate-x-1 transition-transform" />
                    </a>
                </div>
                <div class="mt-12 flex flex-wrap justify-center gap-6 text-[10px] text-slate-600 uppercase tracking-widest font-mono">
                    <span class="flex items-center gap-2"><IconShieldLock size={12} /> Secure Payment</span>
                    <span class="w-px h-3 bg-slate-800"></span>
                    <span>Cancel Anytime</span>
                    <span class="w-px h-3 bg-slate-800"></span>
                    <span>24/7 Support</span>
                </div>
            </div>
        </section>

    </main>
</div>