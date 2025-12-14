<script lang="ts">
    import { onMount } from 'svelte';
    import { cubicOut } from 'svelte/easing';
    import IconBolt from '@tabler/icons-svelte/icons/bolt';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconActivity from '@tabler/icons-svelte/icons/activity';
    import IconTarget from '@tabler/icons-svelte/icons/target';
    import IconArrowUpRight from '@tabler/icons-svelte/icons/arrow-up-right';
    import IconAntenna from '@tabler/icons-svelte/icons/antenna';
    import IconClock from '@tabler/icons-svelte/icons/clock';

    // --- Data Configuration ---
    const signals = [
        {
            id: 'spx',
            type: 'intraday',
            badge: '0DTE STRUCTURE',
            title: 'SPX Profit Pulse',
            price: '$97/mo',
            description: 'Context-rich intraday alerts. We isolate high-probability inflection points on the S&P 500 with defined invalidation levels.',
            metrics: [
                { label: 'Avg Duration', value: '45m' },
                { label: 'Risk/Reward', value: '1:3' },
                { label: 'Frequency', value: 'High' }
            ],
            href: '/alert-services/spx-profit-pulse',
            icon: IconBolt,
            accent: 'amber', // Tailwind color
            chartColor: '#fbbf24' // Hex for SVG
        },
        {
            id: 'swing',
            type: 'swing',
            badge: 'MULTI-DAY FLOW',
            title: 'Explosive Swings',
            price: '$127/mo',
            description: 'Institutional accumulation setups in high-beta equities. We track dark pool positioning to catch breakouts before the crowd.',
            metrics: [
                { label: 'Hold Time', value: '2-5d' },
                { label: 'Risk/Reward', value: '1:5' },
                { label: 'Volatility', value: 'Med' }
            ],
            href: '/alert-services/explosive-swings',
            icon: IconTrendingUp,
            accent: 'orange',
            chartColor: '#fb923c' 
        }
    ];

    // --- Interaction Logic ---
    let containerRef = $state<HTMLElement | null>(null);
    let mouse = $state({ x: 0, y: 0 });
    // ICT11+ Fix: Default to true since LazySection handles lazy loading
    let isVisible = $state(true);

    const handleMouseMove = (e: MouseEvent) => {
        if (!containerRef) return;
        const rect = containerRef.getBoundingClientRect();
        mouse.x = e.clientX - rect.left;
        mouse.y = e.clientY - rect.top;
    };

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
    onmousemove={handleMouseMove}
    role="group"
    aria-label="Alert Services"
    class="relative py-24 lg:py-32 px-4 sm:px-6 lg:px-8 bg-zinc-950 overflow-hidden border-t border-zinc-900"
>
    <div class="absolute inset-0 pointer-events-none">
        <div class="absolute inset-0 bg-[linear-gradient(to_right,#18181b_1px,transparent_1px),linear-gradient(to_bottom,#18181b_1px,transparent_1px)] bg-[size:3rem_3rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)] opacity-30"></div>
    </div>

    <div class="relative max-w-7xl mx-auto z-10">
        
        <div class="max-w-3xl mx-auto text-center mb-20">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="inline-flex items-center justify-center gap-2 mb-6">
                    <span class="relative flex h-2 w-2">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"></span>
                    </span>
                    <span class="text-xs font-mono uppercase tracking-widest text-amber-500/80">Signal Feed Active</span>
                </div>
                
                <h2 in:heavySlide={{ delay: 100 }} class="text-3xl md:text-5xl font-medium tracking-tight text-white mb-6">
                    Market Intelligence
                </h2>
                
                <p in:heavySlide={{ delay: 200 }} class="text-base md:text-lg text-zinc-500 leading-relaxed font-light max-w-2xl mx-auto">
                    Algorithmic detection verified by human traders. 
                    Receive fully contextualized trade plans with entry, stop, and target parameters.
                </p>
            {/if}
        </div>

        <div 
            class="group/grid grid md:grid-cols-2 gap-8 max-w-5xl mx-auto"
            style="--x: {mouse.x}px; --y: {mouse.y}px;"
        >
            {#each signals as item, i}
                {@const IconComponent = item.icon}
                {#if isVisible}
                    <div 
                        in:heavySlide={{ delay: 300 + (i * 150) }}
                        class="relative group/card bg-zinc-950 border border-zinc-800 rounded-xl overflow-hidden hover:border-zinc-700 transition-colors duration-500"
                    >
                        <div class="absolute inset-0 z-0 opacity-0 group-hover/grid:opacity-100 transition-opacity duration-500"
                             style="background: radial-gradient(600px circle at var(--x) var(--y), rgba(255,255,255,0.04), transparent 40%); pointer-events: none;">
                        </div>

                        <div class="relative h-48 w-full bg-zinc-900/30 border-b border-zinc-800 overflow-hidden">
                            <div class="absolute inset-0 bg-[size:20px_20px] bg-grid-zinc-800/50"></div>
                            
                            <div class="absolute inset-0 flex items-center justify-center p-8">
                                {#if item.type === 'intraday'}
                                    <svg class="w-full h-full overflow-visible" viewBox="0 0 300 100" preserveAspectRatio="none">
                                        <defs>
                                            <linearGradient id="grad-spx" x1="0%" y1="0%" x2="100%" y2="0%">
                                                <stop offset="0%" stop-color={item.chartColor} stop-opacity="0.1" />
                                                <stop offset="100%" stop-color={item.chartColor} stop-opacity="1" />
                                            </linearGradient>
                                        </defs>
                                        <path 
                                            d="M0,50 L20,45 L40,55 L60,30 L80,60 L100,40 L120,50 L140,20 L160,40 L180,30 L200,80 L220,60 L240,70 L260,20 L300,10" 
                                            fill="none" 
                                            stroke="url(#grad-spx)" 
                                            stroke-width="2" 
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="drop-shadow-[0_0_10px_rgba(251,191,36,0.5)]"
                                        />
                                        <circle cx="300" cy="10" r="3" fill={item.chartColor} class="animate-pulse">
                                            <animate attributeName="r" values="3;6;3" dur="2s" repeatCount="indefinite" />
                                            <animate attributeName="opacity" values="1;0.5;1" dur="2s" repeatCount="indefinite" />
                                        </circle>
                                        <rect x="240" y="15" width="60" height="60" fill={item.chartColor} fill-opacity="0.05" stroke={item.chartColor} stroke-opacity="0.2" stroke-dasharray="4 2" />
                                        <text x="245" y="85" font-family="monospace" font-size="8" fill={item.chartColor} opacity="0.8">ENTRY ZONE</text>
                                    </svg>
                                {:else}
                                    <svg class="w-full h-full overflow-visible" viewBox="0 0 300 100" preserveAspectRatio="none">
                                        <defs>
                                            <linearGradient id="grad-swing" x1="0%" y1="0%" x2="100%" y2="0%">
                                                <stop offset="0%" stop-color={item.chartColor} stop-opacity="0.1" />
                                                <stop offset="100%" stop-color={item.chartColor} stop-opacity="1" />
                                            </linearGradient>
                                        </defs>
                                        <path 
                                            d="M0,20 Q50,20 70,50 Q100,90 150,90 Q200,90 230,50 L250,60 L280,30 L300,5" 
                                            fill="none" 
                                            stroke="url(#grad-swing)" 
                                            stroke-width="2" 
                                            stroke-linecap="round"
                                            class="drop-shadow-[0_0_10px_rgba(251,146,60,0.5)]"
                                        />
                                        <line x1="200" y1="50" x2="300" y2="50" stroke="white" stroke-opacity="0.1" stroke-dasharray="4" />
                                        <circle cx="280" cy="30" r="3" fill={item.chartColor} />
                                        <text x="220" y="40" font-family="monospace" font-size="8" fill="white" opacity="0.5">BREAKOUT LEVEL</text>
                                    </svg>
                                {/if}
                            </div>
                        </div>

                        <div class="relative z-10 p-8">
                            
                            <div class="flex justify-between items-start mb-6">
                                <div class="flex items-center gap-3">
                                    <div class="p-2 rounded bg-zinc-900 border border-zinc-800 {item.accent === 'amber' ? 'text-amber-500' : 'text-orange-500'}">
                                        <IconComponent size={20} />
                                    </div>
                                    <div>
                                        <h3 class="text-xl font-medium text-white">{item.title}</h3>
                                        <div class="flex items-center gap-2 mt-1">
                                            <span class="w-1.5 h-1.5 rounded-full animate-pulse {item.accent === 'amber' ? 'bg-amber-500' : 'bg-orange-500'}"></span>
                                            <span class="text-[10px] font-mono uppercase tracking-wider {item.accent === 'amber' ? 'text-amber-500/80' : 'text-orange-500/80'}">{item.badge}</span>
                                        </div>
                                    </div>
                                </div>
                                <div class="text-right">
                                    <div class="text-lg font-medium text-white">{item.price}</div>
                                    <div class="text-xs text-zinc-500">per month</div>
                                </div>
                            </div>

                            <p class="text-sm text-zinc-400 leading-relaxed mb-8 h-12">
                                {item.description}
                            </p>

                            <div class="grid grid-cols-3 gap-px bg-zinc-800 border border-zinc-800 rounded-lg overflow-hidden mb-8">
                                {#each item.metrics as metric}
                                    <div class="bg-zinc-900/50 p-3 text-center group-hover/card:bg-zinc-900 transition-colors">
                                        <div class="text-[10px] uppercase text-zinc-500 font-mono mb-1">{metric.label}</div>
                                        <div class="text-sm font-medium text-zinc-300">{metric.value}</div>
                                    </div>
                                {/each}
                            </div>

                            <a 
                                href={item.href} 
                                class="group/btn relative w-full flex items-center justify-between px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-md text-sm font-medium text-zinc-300 transition-all duration-300 hover:text-white {item.accent === 'amber' ? 'hover:border-amber-500/50 hover:bg-amber-500/10' : 'hover:border-orange-500/50 hover:bg-orange-500/10'}"
                            >
                                <span class="flex items-center gap-2">
                                    <IconAntenna size={16} class={item.accent === 'amber' ? 'text-amber-500' : 'text-orange-500'} />
                                    <span>Subscribe to Feed</span>
                                </span>
                                <IconArrowUpRight size={16} class="text-zinc-500 transition-transform duration-300 group-hover/btn:translate-x-0.5 group-hover/btn:-translate-y-0.5 group-hover/btn:text-white" />
                            </a>

                        </div>
                    </div>
                {/if}
            {/each}
        </div>

        {#if isVisible}
            <div in:heavySlide={{ delay: 600 }} class="mt-12 text-center border-t border-zinc-900 pt-8">
                <div class="inline-flex items-center gap-6 text-xs text-zinc-500 font-mono">
                    <span class="flex items-center gap-2">
                        <IconTarget size={14} />
                        <span>STRICT INVALIDATION LEVELS</span>
                    </span>
                    <span class="hidden sm:inline w-px h-3 bg-zinc-800"></span>
                    <span class="flex items-center gap-2">
                        <IconClock size={14} />
                        <span>REAL-TIME PUSH NOTIFICATIONS</span>
                    </span>
                    <span class="hidden sm:inline w-px h-3 bg-zinc-800"></span>
                    <span class="flex items-center gap-2">
                        <IconActivity size={14} />
                        <span>FULL TRADE MANAGEMENT</span>
                    </span>
                </div>
            </div>
        {/if}
    </div>
</section>

<style>
    /* Utility for the grid lines inside the charts */
    .bg-grid-zinc-800\/50 {
        background-image: linear-gradient(to right, #27272a 1px, transparent 1px),
                          linear-gradient(to bottom, #27272a 1px, transparent 1px);
    }
</style>