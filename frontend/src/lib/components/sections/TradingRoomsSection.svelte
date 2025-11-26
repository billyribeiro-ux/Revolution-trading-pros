<script lang="ts">
    import { onMount } from 'svelte';
    import { cubicOut } from 'svelte/easing';
    import IconActivity from '@tabler/icons-svelte/icons/activity';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconBuilding from '@tabler/icons-svelte/icons/building';
    import IconArrowUpRight from '@tabler/icons-svelte/icons/arrow-up-right';
    import IconTerminal from '@tabler/icons-svelte/icons/terminal';

    // --- Data Configuration ---
    const products = [
        {
            id: 'day',
            type: 'candles', // UPDATED: Triggers the Live Candle animation
            label: 'INTRADAY',
            title: 'Day Trading Desk',
            metric: '9:30 AM EST',
            description: 'High-velocity execution environment. Real-time order flow analysis and level 2 data interpretation.',
            features: ['Live Execution', 'Risk Parameters', 'Gap Strategy'],
            href: '/live-trading-rooms/day-trading',
            icon: IconActivity,
            accent: 'blue', 
            cta: 'Launch Terminal'
        },
        {
            id: 'swing',
            type: 'wave',
            label: 'POSITIONAL',
            title: 'Swing Strategy',
            metric: 'Weekly Cycle',
            description: 'Institutional timeframe alignment. Capture multi-day moves driven by macro-economic flows.',
            features: ['Macro Analysis', 'Flow Tracking', 'Overnight Risk'],
            href: '/live-trading-rooms/swing-trading',
            icon: IconTrendingUp,
            accent: 'emerald',
            cta: 'View Strategy'
        },
        {
            id: 'foundation',
            type: 'step',
            label: 'FOUNDATION',
            title: 'Capital Builder',
            metric: '< $25k Accts',
            description: 'Strict discipline protocol for emerging capital. Focus on risk-adjusted returns and drawdown control.',
            features: ['Risk Protocol', 'Capital Preservation', 'Scaling Logic'],
            href: '/live-trading-rooms/small-accounts',
            icon: IconBuilding,
            accent: 'indigo',
            cta: 'Start Building'
        }
    ];

    let isVisible = false;
    let containerRef: HTMLElement;

    onMount(() => {
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
</script>

<section 
    bind:this={containerRef}
    class="relative py-32 px-4 sm:px-6 lg:px-8 bg-zinc-950 overflow-hidden border-t border-zinc-900"
>
    <div class="absolute inset-0 pointer-events-none">
        <div class="absolute inset-0 bg-[linear-gradient(to_right,#18181b_1px,transparent_1px),linear-gradient(to_bottom,#18181b_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)] opacity-40"></div>
    </div>

    <div class="relative max-w-7xl mx-auto z-10">
        
        <div class="max-w-3xl mx-auto text-center mb-20">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="inline-flex items-center justify-center gap-2 mb-6">
                    <span class="relative flex h-2 w-2">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                    </span>
                    <span class="text-xs font-mono uppercase tracking-widest text-emerald-500/80">Market Access Open</span>
                </div>
                
                <h2 in:heavySlide={{ delay: 100 }} class="text-3xl md:text-5xl font-medium tracking-tight text-white mb-6">
                    Professional Trading Environments
                </h2>
                
                <p in:heavySlide={{ delay: 200 }} class="text-base md:text-lg text-zinc-500 leading-relaxed font-light max-w-2xl mx-auto">
                    Select an environment tailored to your liquidity requirements. 
                    <span class="text-zinc-400">Hover over a desk to preview data feeds.</span>
                </p>
            {/if}
        </div>

        <div class="grid md:grid-cols-3 gap-px bg-zinc-800 border border-zinc-800 rounded-lg overflow-hidden shadow-2xl shadow-black/50">
            {#each products as item, i}
                {#if isVisible}
                    <div 
                        in:heavySlide={{ delay: 300 + (i * 100) }}
                        class="relative group bg-zinc-950 p-8 lg:p-10 flex flex-col h-[420px] overflow-hidden"
                    >
                        {#if item.type === 'candles'}
                            <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none">
                                <div class="absolute inset-0 bg-blue-900/10"></div>
                                
                                <svg class="absolute bottom-10 left-0 w-full h-48" viewBox="0 0 400 200" preserveAspectRatio="none">
                                    <line x1="0" y1="50" x2="400" y2="50" stroke="#1e3a8a" stroke-width="1" stroke-dasharray="4 4" opacity="0.3" />
                                    <line x1="0" y1="100" x2="400" y2="100" stroke="#1e3a8a" stroke-width="1" stroke-dasharray="4 4" opacity="0.3" />
                                    <line x1="0" y1="150" x2="400" y2="150" stroke="#1e3a8a" stroke-width="1" stroke-dasharray="4 4" opacity="0.3" />

                                    <line x1="40" y1="120" x2="40" y2="180" stroke="#3b82f6" stroke-width="1" />
                                    <rect x="30" y="140" width="20" height="30" fill="#3b82f6" opacity="0.8" />
                                    
                                    <line x1="90" y1="130" x2="90" y2="150" stroke="#1e3a8a" stroke-width="1" />
                                    <rect x="80" y="135" width="20" height="10" fill="none" stroke="#3b82f6" stroke-width="2" />

                                    <line x1="140" y1="80" x2="140" y2="140" stroke="#3b82f6" stroke-width="1" />
                                    <rect x="130" y="90" width="20" height="40" fill="#3b82f6" opacity="0.9" />

                                    <line x1="190" y1="85" x2="190" y2="105" stroke="#3b82f6" stroke-width="1" />
                                    <rect x="180" y="90" width="20" height="5" fill="#3b82f6" opacity="0.8" />

                                    <line x1="240" y1="80" x2="240" y2="120" stroke="#1e3a8a" stroke-width="1" />
                                    <rect x="230" y="95" width="20" height="15" fill="none" stroke="#3b82f6" stroke-width="2" />

                                    <line x1="290" y1="60" x2="290" y2="110" stroke="#3b82f6" stroke-width="1" />
                                    <rect x="280" y="70" width="20" height="35" fill="#3b82f6" opacity="0.9" />

                                    <g class="animate-live-candle origin-bottom">
                                        <line x1="340" y1="20" x2="340" y2="80" stroke="#60a5fa" stroke-width="1" />
                                        <rect x="330" y="40" width="20" height="30" fill="#60a5fa" />
                                    </g>
                                </svg>
                            </div>
                        {/if}

                        {#if item.type === 'wave'}
                            <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none">
                                <div class="absolute inset-0 bg-emerald-900/10"></div>
                                <svg class="absolute bottom-0 left-0 w-[200%] h-64 text-emerald-500/20 animate-wave-slide" viewBox="0 0 1000 300" preserveAspectRatio="none">
                                    <path d="M0,150 C150,50 350,250 500,150 C650,50 850,250 1000,150 V300 H0 Z" fill="currentColor" />
                                    <path d="M0,160 C140,60 360,260 500,160 C640,60 860,260 1000,160" fill="none" stroke="currentColor" stroke-width="2" class="text-emerald-400/40" />
                                </svg>
                            </div>
                        {/if}

                        {#if item.type === 'step'}
                            <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none">
                                <div class="absolute inset-0 bg-indigo-900/10"></div>
                                <div class="absolute bottom-0 left-0 right-0 h-64 flex items-end justify-around px-4 pb-0">
                                    {#each Array(8) as _, k}
                                        <div 
                                            class="w-8 bg-indigo-500/20 border-t border-indigo-400/30 transition-all duration-700 ease-out group-hover:h-[calc(20%+var(--h))] h-0"
                                            style="--h: {k * 10}%"
                                        ></div>
                                    {/each}
                                </div>
                            </div>
                        {/if}

                        <div class="absolute inset-0 bg-gradient-to-t from-zinc-950 via-zinc-950/80 to-transparent opacity-90 group-hover:opacity-40 transition-opacity duration-500 pointer-events-none"></div>

                        <div class="relative z-10 flex justify-between items-start mb-8">
                            <div class="p-3 bg-zinc-900/80 backdrop-blur border border-zinc-800 rounded-md text-zinc-400 group-hover:text-white group-hover:border-zinc-600 transition-all duration-300">
                                <svelte:component this={item.icon} size={24} stroke={1.25} />
                            </div>
                            <span class="font-mono text-[10px] tracking-widest text-zinc-600 uppercase border border-zinc-800 bg-zinc-950/50 backdrop-blur px-2 py-1 rounded">
                                {item.label}
                            </span>
                        </div>

                        <div class="relative z-10 flex-grow">
                            <h3 class="text-xl font-medium text-white mb-2 group-hover:scale-105 transition-transform duration-500 origin-left">
                                {item.title}
                            </h3>
                            
                            <div class="flex items-center gap-2 mb-6 text-xs font-mono text-{item.accent}-400/80">
                                <IconTerminal size={12} />
                                <span>{item.metric}</span>
                            </div>

                            <div class="relative">
                                <div class="transition-all duration-500 ease-in-out group-hover:opacity-0 group-hover:-translate-y-2 group-hover:blur-sm">
                                    <p class="text-sm text-zinc-400 leading-relaxed font-light mb-8">
                                        {item.description}
                                    </p>
                                    <div class="space-y-2 border-l border-zinc-800 pl-4">
                                        {#each item.features as feat}
                                            <div class="text-xs text-zinc-500">{feat}</div>
                                        {/each}
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="relative z-20 mt-auto pt-6 border-t border-zinc-900 group-hover:border-{item.accent}-500/30 transition-colors duration-300 bg-zinc-950/20 backdrop-blur-sm">
                            <a 
                                href={item.href} 
                                class="flex items-center justify-between w-full text-sm font-medium text-zinc-300 transition-colors duration-300 group-hover:text-white"
                            >
                                <span class="group-hover:translate-x-1 transition-transform duration-300">{item.cta}</span>
                                <div class="p-1.5 rounded bg-zinc-900 group-hover:bg-{item.accent}-500 group-hover:text-white transition-colors duration-300">
                                    <IconArrowUpRight size={16} />
                                </div>
                            </a>
                        </div>

                    </div>
                {/if}
            {/each}
        </div>
    </div>
</section>

<style>
    /* Live Candle Pulse */
    @keyframes live-candle {
        0% { transform: scaleY(1) translateY(0); }
        50% { transform: scaleY(1.4) translateY(-10px); }
        70% { transform: scaleY(0.9) translateY(5px); }
        100% { transform: scaleY(1) translateY(0); }
    }
    .animate-live-candle {
        animation: live-candle 2s ease-in-out infinite;
        transform-box: fill-box;
        transform-origin: bottom;
    }

    /* Swing Wave Animation */
    @keyframes wave-slide {
        0% { transform: translateX(0); }
        100% { transform: translateX(-50%); }
    }
    .animate-wave-slide {
        animation: wave-slide 10s linear infinite;
    }

    /* General Utilities */
    .gap-px { gap: 1px; }
</style>