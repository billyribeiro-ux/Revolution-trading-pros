<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import { cubicOut } from 'svelte/easing';
    // Using the direct path imports as requested in your snippet
    import IconSitemap from '@tabler/icons-svelte/icons/sitemap';
    import IconShieldLock from '@tabler/icons-svelte/icons/shield-lock';
    import IconDatabase from '@tabler/icons-svelte/icons/database';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconCheck from '@tabler/icons-svelte/icons/check';

    const features = [
        {
            id: 'SYS-01',
            title: 'Execution Framework',
            subtitle: 'STRUCTURE',
            description: 'We replace discretionary guessing with a repeatable, probabilistic execution model used by proprietary desks.',
            icon: IconSitemap,
            status: 'Operational',
            type: 'grid'
        },
        {
            id: 'SYS-02',
            title: 'Variance Control',
            subtitle: 'SURVIVAL',
            description: 'Capital preservation is the mandate. Every alert includes hard invalidation points and fat-tail risk sizing logic.',
            icon: IconShieldLock,
            status: 'Active',
            type: 'radar'
        },
        {
            id: 'SYS-03',
            title: 'Data Sovereignty',
            subtitle: 'INTELLIGENCE',
            description: 'Direct access to institutional flows. We track Dark Pool volume, GEX levels, and Vanna flows in real-time.',
            icon: IconDatabase,
            status: 'Live Feed',
            type: 'circuit'
        }
    ];

    // --- Interaction Logic ---
    let containerRef = $state<HTMLElement | null>(null);
    let mouse = $state({ x: 0, y: 0 });
    // ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
    let isVisible = $state(false);

    const handleMouseMove = (e: MouseEvent) => {
        if (!containerRef) return;
        const rect = containerRef.getBoundingClientRect();
        mouse.x = e.clientX - rect.left;
        mouse.y = e.clientY - rect.top;
    };

    function heavySlide(_node: Element, { delay = 0, duration = 1000 }) {
        return {
            delay,
            duration,
            css: (t: number) => {
                const eased = cubicOut(t);
                return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px);`;
            }
        };
    }

    // Trigger entrance animations when section scrolls into viewport
    let observer: IntersectionObserver | null = null;
    
    onMount(() => {
        if (!browser) {
            isVisible = true;
            return;
        }
        
        queueMicrotask(() => {
            if (!containerRef) {
                isVisible = true;
                return;
            }
            
            observer = new IntersectionObserver(
                (entries) => {
                    if (entries[0]?.isIntersecting) {
                        isVisible = true;
                        observer?.disconnect();
                    }
                },
                { threshold: 0.1, rootMargin: '50px' }
            );
            
            observer.observe(containerRef);
        });
        
        return () => observer?.disconnect();
    });
</script>

<section 
    bind:this={containerRef}
    onmousemove={handleMouseMove}
    role="group"
    aria-label="Core Infrastructure Features"
    class="relative py-32 px-6 bg-[#050505] overflow-hidden border-b border-white/5"
>
    <div class="absolute inset-0 pointer-events-none">
        <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"></div>
    </div>

    <div class="relative max-w-[1600px] mx-auto z-10">
        
        <div class="max-w-3xl mb-24">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="inline-flex items-center gap-3 px-3 py-1 border border-amber-900/30 bg-amber-900/10 text-amber-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-8">
                    <span class="relative flex h-2 w-2">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-500 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"></span>
                    </span>
                    System Architecture
                </div>
                
                <h2 in:heavySlide={{ delay: 100 }} class="text-4xl md:text-6xl font-serif text-white mb-8 tracking-tight">
                    Core <span class="text-slate-700">Infrastructure.</span>
                </h2>
                
                <p in:heavySlide={{ delay: 200 }} class="text-lg text-slate-400 font-light leading-relaxed max-w-2xl border-l-2 border-white/10 pl-6">
                    We replaced marketing hype with financial engineering. Our ecosystem combines structured execution frameworks with institutional data tools.
                </p>
            {/if}
        </div>

        <div 
            class="group/grid grid md:grid-cols-3 gap-8"
            style="--x: {mouse.x}px; --y: {mouse.y}px;"
        >
            {#each features as feature, i}
                {@const IconComponent = feature.icon}
                {#if isVisible}
                    <div 
                        in:heavySlide={{ delay: 300 + (i * 150) }}
                        class="relative group/card bg-[#080808] border border-white/10 p-10 overflow-hidden hover:border-amber-900/50 transition-colors duration-500"
                    >
                        <div class="absolute inset-0 opacity-0 group-hover/grid:opacity-100 transition-opacity duration-500 pointer-events-none"
                             style="background: radial-gradient(800px circle at var(--x) var(--y), rgba(255,255,255,0.03), transparent 40%);">
                        </div>

                        <div class="absolute top-0 right-0 w-40 h-40 opacity-[0.02] group-hover/card:opacity-10 transition-opacity duration-500 pointer-events-none text-white">
                            {#if feature.type === 'grid'}
                                <svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="0.5">
                                    <path d="M10 10 H90 M10 30 H90 M10 50 H90 M10 70 H90 M10 90 H90 M10 10 V90 M30 10 V90 M50 10 V90 M70 10 V90 M90 10 V90" />
                                </svg>
                            {:else if feature.type === 'radar'}
                                <svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="0.5">
                                    <circle cx="50" cy="50" r="20" />
                                    <circle cx="50" cy="50" r="35" />
                                    <circle cx="50" cy="50" r="45" opacity="0.5" />
                                    <line x1="50" y1="50" x2="95" y2="50" />
                                </svg>
                            {:else}
                                <svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="0.5">
                                    <rect x="20" y="20" width="60" height="60" rx="4" />
                                    <path d="M50 20 V10 M50 90 V80 M20 50 H10 M90 50 H80" />
                                    <rect x="35" y="35" width="30" height="30" />
                                </svg>
                            {/if}
                        </div>

                        <div class="flex justify-between items-start mb-12 relative z-10">
                            <div class="w-12 h-12 flex items-center justify-center bg-white/5 border border-white/10 text-white group-hover/card:bg-amber-500 group-hover/card:text-black group-hover/card:border-amber-500 transition-all duration-300">
                                <IconComponent size={24} stroke={1.5} />
                            </div>
                            <span class="font-mono text-[10px] text-slate-600 uppercase tracking-widest">
                                {feature.id}
                            </span>
                        </div>

                        <div class="relative z-10">
                            <div class="text-[10px] font-mono uppercase tracking-widest text-amber-600 mb-3">
                                {feature.subtitle}
                            </div>
                            
                            <h3 class="text-2xl font-serif text-white mb-6 group-hover/card:text-white transition-colors">
                                {feature.title}
                            </h3>
                            
                            <p class="text-sm text-slate-400 leading-relaxed font-light mb-12 h-16">
                                {feature.description}
                            </p>

                            <div class="flex items-center justify-between border-t border-white/10 pt-6">
                                <div class="flex items-center gap-2">
                                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                                    <span class="text-[10px] font-mono text-slate-500 uppercase tracking-widest group-hover/card:text-white transition-colors">
                                        {feature.status}
                                    </span>
                                </div>
                                <div class="opacity-0 group-hover/card:opacity-100 transition-opacity duration-300 -translate-x-2 group-hover/card:translate-x-0">
                                    <IconArrowRight size={16} class="text-white" />
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            {/each}
        </div>
    </div>
</section>