<script lang="ts">
    import { onMount } from 'svelte';
    import { cubicOut } from 'svelte/easing';
    import IconSitemap from '@tabler/icons-svelte/icons/sitemap';
    import IconShield from '@tabler/icons-svelte/icons/shield';
    import IconCpu from '@tabler/icons-svelte/icons/cpu';
    import IconCheck from '@tabler/icons-svelte/icons/check';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';

    const features = [
        {
            title: 'Structured Curriculum',
            subtitle: 'FRAMEWORK',
            description: 'Move beyond random setups. We teach a repeatable, probabilistic execution model used by proprietary desks.',
            icon: IconSitemap,
            accent: 'cyan',
            type: 'grid' // Triggers grid background
        },
        {
            title: 'Risk Protocols',
            subtitle: 'SURVIVAL',
            description: 'Capital preservation is the primary objective. Every trade alert includes hard invalidation points and sizing logic.',
            icon: IconShield,
            accent: 'emerald',
            type: 'radar' // Triggers radar background
        },
        {
            title: 'Proprietary Analytics',
            subtitle: 'INFRASTRUCTURE',
            description: 'Access institutional-grade indicators that track dark pool volume, gamma exposure, and volatility flows.',
            icon: IconCpu,
            accent: 'indigo',
            type: 'circuit' // Triggers circuit background
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
    aria-label="Core Infrastructure Features"
    class="relative py-24 lg:py-32 px-4 sm:px-6 lg:px-8 bg-zinc-950 overflow-hidden border-t border-zinc-900"
>
    <!-- Technical Background -->
    <div class="absolute inset-0 pointer-events-none">
        <div class="absolute inset-0 bg-[linear-gradient(to_right,#18181b_1px,transparent_1px),linear-gradient(to_bottom,#18181b_1px,transparent_1px)] bg-[size:2rem_2rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)] opacity-20"></div>
    </div>

    <div class="relative max-w-7xl mx-auto z-10">
        
        <!-- Header -->
        <div class="max-w-3xl mx-auto text-center mb-20">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="inline-flex items-center justify-center gap-2 mb-6">
                    <span class="relative flex h-2 w-2">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-zinc-400 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-zinc-500"></span>
                    </span>
                    <span class="text-xs font-mono uppercase tracking-widest text-zinc-500">System Architecture</span>
                </div>
                
                <h2 in:heavySlide={{ delay: 100 }} class="text-3xl md:text-5xl font-medium tracking-tight text-white mb-6">
                    Core Infrastructure
                </h2>
                
                <p in:heavySlide={{ delay: 200 }} class="text-base md:text-lg text-zinc-500 leading-relaxed font-light max-w-2xl mx-auto">
                    We replaced hype with engineering. Our ecosystem combines structured education, strict risk controls, and professional data tools.
                </p>
            {/if}
        </div>

        <!-- 3-Column Grid -->
        <div 
            class="group/grid grid md:grid-cols-3 gap-8"
            style="--x: {mouse.x}px; --y: {mouse.y}px;"
        >
            {#each features as feature, i}
                {@const IconComponent = feature.icon}
                {#if isVisible}
                    <div 
                        in:heavySlide={{ delay: 300 + (i * 150) }}
                        class="relative group/card bg-zinc-950/50 border border-zinc-800 rounded-xl p-8 hover:bg-zinc-900/30 transition-all duration-500 overflow-hidden"
                    >
                        <!-- Spotlight Overlay -->
                        <div class="absolute inset-0 opacity-0 group-hover/grid:opacity-100 transition-opacity duration-500 pointer-events-none"
                             style="background: radial-gradient(600px circle at var(--x) var(--y), rgba(255,255,255,0.03), transparent 40%);">
                        </div>

                        <!-- Technical SVG Backgrounds -->
                        <div class="absolute top-0 right-0 w-32 h-32 opacity-[0.03] group-hover/card:opacity-10 transition-opacity duration-500 pointer-events-none {feature.accent === 'cyan' ? 'text-cyan-500' : feature.accent === 'emerald' ? 'text-emerald-500' : 'text-indigo-500'}">
                            {#if feature.type === 'grid'}
                                <svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="1">
                                    <path d="M10 10 H90 M10 30 H90 M10 50 H90 M10 70 H90 M10 90 H90 M10 10 V90 M30 10 V90 M50 10 V90 M70 10 V90 M90 10 V90" />
                                </svg>
                            {:else if feature.type === 'radar'}
                                <svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="1">
                                    <circle cx="50" cy="50" r="20" />
                                    <circle cx="50" cy="50" r="35" />
                                    <circle cx="50" cy="50" r="45" opacity="0.5" />
                                    <line x1="50" y1="50" x2="95" y2="50" />
                                </svg>
                            {:else}
                                <svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="1">
                                    <rect x="20" y="20" width="60" height="60" rx="4" />
                                    <path d="M50 20 V10 M50 90 V80 M20 50 H10 M90 50 H80" />
                                    <rect x="35" y="35" width="30" height="30" />
                                </svg>
                            {/if}
                        </div>

                        <!-- Icon Container -->
                        <div class="relative z-10 mb-8 inline-block">
                            <div class="p-3 bg-zinc-900 border border-zinc-800 rounded-lg group-hover/card:shadow-[0_0_20px_rgba(0,0,0,0.3)] transition-all duration-300 group-hover/card:-translate-y-1 {feature.accent === 'cyan' ? 'text-cyan-500 group-hover/card:border-cyan-500/30' : feature.accent === 'emerald' ? 'text-emerald-500 group-hover/card:border-emerald-500/30' : 'text-indigo-500 group-hover/card:border-indigo-500/30'}">
                                <IconComponent size={28} stroke={1.5} />
                            </div>
                            <!-- Connecting Line -->
                            <div class="absolute left-1/2 bottom-0 w-px h-8 bg-zinc-800 translate-y-full -translate-x-1/2 -z-10 transition-colors {feature.accent === 'cyan' ? 'group-hover/card:bg-cyan-500/50' : feature.accent === 'emerald' ? 'group-hover/card:bg-emerald-500/50' : 'group-hover/card:bg-indigo-500/50'}"></div>
                        </div>

                        <!-- Content -->
                        <div class="relative z-10 mt-4">
                            <div class="flex items-center gap-3 mb-3">
                                <span class="text-[10px] font-mono uppercase tracking-widest text-zinc-500 border border-zinc-800 px-2 py-0.5 rounded">
                                    {feature.subtitle}
                                </span>
                            </div>
                            
                            <h3 class="text-xl font-medium text-white mb-4 transition-colors {feature.accent === 'cyan' ? 'group-hover/card:text-cyan-400' : feature.accent === 'emerald' ? 'group-hover/card:text-emerald-400' : 'group-hover/card:text-indigo-400'}">
                                {feature.title}
                            </h3>
                            
                            <p class="text-sm text-zinc-400 leading-relaxed font-light mb-8">
                                {feature.description}
                            </p>

                            <!-- Micro Status Indicator -->
                            <div class="flex items-center gap-2 text-[10px] font-mono text-zinc-600 border-t border-zinc-900 pt-4 group-hover/card:text-zinc-500 transition-colors">
                                <IconCheck size={12} class={feature.accent === 'cyan' ? 'text-cyan-500' : feature.accent === 'emerald' ? 'text-emerald-500' : 'text-indigo-500'} />
                                <span>MODULE ACTIVE</span>
                            </div>
                        </div>
                    </div>
                {/if}
            {/each}
        </div>
    </div>
</section>