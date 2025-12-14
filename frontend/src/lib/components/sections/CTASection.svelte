<script lang="ts">
    import { onMount } from 'svelte';
    import { cubicOut } from 'svelte/easing';
    import IconLockSquare from '@tabler/icons-svelte/icons/lock-square';
    import IconActivity from '@tabler/icons-svelte/icons/activity';
    import IconServer from '@tabler/icons-svelte/icons/server';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconCheck from '@tabler/icons-svelte/icons/check';

    // --- Interaction Logic ---
    let containerRef = $state<HTMLElement | null>(null);
    let mouse = $state({ x: 0, y: 0 });
    // ICT11+ Fix: Default to true since LazySection handles lazy loading
    let isVisible = $state(true);

    // Mouse tracking for subtle lighting effects
    const handleMouseMove = (e: MouseEvent) => {
        if (!containerRef) return;
        const rect = containerRef.getBoundingClientRect();
        mouse.x = e.clientX - rect.left;
        mouse.y = e.clientY - rect.top;
    };

    // Heavy, expensive-feeling transition
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

    // Mock data for the background "Depth of Market" animation
    const marketDepth = Array(20).fill(0).map((_, i) => ({
        price: (4200 + i * 0.25).toFixed(2),
        size: Math.floor(Math.random() * 500)
    }));
</script>

<section 
    bind:this={containerRef}
    onmousemove={handleMouseMove}
    class="relative py-32 px-6 bg-[#020202] overflow-hidden border-t border-white/10"
    aria-label="Account Creation Terminal"
>
    <div class="absolute inset-0 pointer-events-none opacity-[0.03] overflow-hidden flex justify-between px-10">
        <div class="flex flex-col text-[10px] font-mono text-emerald-500 animate-scroll-up">
            {#each [...marketDepth, ...marketDepth] as tick}
                <div class="flex gap-8 my-1">
                    <span>{tick.price}</span>
                    <span class="opacity-50">{tick.size}</span>
                </div>
            {/each}
        </div>
        <div class="flex flex-col text-[10px] font-mono text-amber-600 animate-scroll-down text-right">
            {#each [...marketDepth, ...marketDepth] as tick}
                <div class="flex gap-8 my-1 justify-end">
                    <span class="opacity-50">{tick.size}</span>
                    <span>{tick.price}</span>
                </div>
            {/each}
        </div>
    </div>

    <div class="absolute inset-0 pointer-events-none opacity-30"
         style="background: radial-gradient(800px circle at var(--x) var(--y), rgba(255,255,255,0.03), transparent 60%);">
    </div>

    <div class="relative max-w-4xl mx-auto z-10">
        
        <div class="text-center">
            {#if isVisible}
                <div in:heavySlide={{ delay: 0, duration: 1000 }} class="inline-flex items-center justify-center gap-3 mb-10">
                    <span class="relative flex h-2 w-2">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-500 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                    </span>
                    <span class="text-[10px] font-mono uppercase tracking-[0.2em] text-slate-500">
                        Market Status: <span class="text-emerald-500">Open</span>
                    </span>
                </div>

                <h2 in:heavySlide={{ delay: 100 }} class="text-5xl md:text-7xl font-serif text-white mb-8 tracking-tight leading-[0.95]">
                    Professional <br />
                    <span class="text-slate-700">Execution</span> Only.
                </h2>

                <p in:heavySlide={{ delay: 200 }} class="text-lg text-slate-400 font-light leading-relaxed max-w-2xl mx-auto mb-16">
                    This is not a game. It is a business. Authenticate now to access institutional-grade alerts, live mentorship, and proprietary indicators.
                </p>

                <div in:heavySlide={{ delay: 300 }} class="relative max-w-xl mx-auto group perspective-1000">
                    
                    <div class="absolute -inset-1 bg-gradient-to-r from-amber-600/20 to-emerald-600/20 rounded opacity-0 group-hover:opacity-100 blur-xl transition-opacity duration-700"></div>

                    <div class="relative bg-[#050505] border border-white/10 p-1 shadow-2xl">
                        
                        <div class="flex items-center justify-between px-4 py-2 bg-[#0a0a0a] border-b border-white/5">
                            <div class="flex items-center gap-2">
                                <IconLockSquare size={14} class="text-amber-600" />
                                <span class="text-[10px] font-mono uppercase text-slate-500 tracking-widest">Secure_Enclave_v4.2</span>
                            </div>
                            <div class="text-[10px] font-mono text-emerald-500">TLS 1.3 ENCRYPTED</div>
                        </div>

                        <div class="p-6 md:p-8 text-left">
                            <div class="mb-6">
                                <label for="email-access" class="block text-[10px] font-mono uppercase text-slate-500 tracking-widest mb-2">
                                    Identity / Email
                                </label>
                                <div class="relative">
                                    <input 
                                        type="email" 
                                        id="email-access" 
                                        placeholder="trader@fund.com" 
                                        class="w-full bg-[#020202] border border-white/10 text-white font-mono text-sm px-4 py-3 focus:outline-none focus:border-amber-600 focus:ring-1 focus:ring-amber-600 transition-all placeholder:text-slate-700"
                                    />
                                    <div class="absolute right-3 top-1/2 -translate-y-1/2 w-1.5 h-4 bg-amber-600 animate-pulse pointer-events-none"></div>
                                </div>
                            </div>

                            <a 
                                href="/signup" 
                                class="flex items-center justify-between w-full bg-amber-700 hover:bg-amber-600 text-white px-6 py-4 transition-all duration-200 group/btn shadow-[0_4px_20px_rgba(180,83,9,0.2)]"
                            >
                                <div class="flex flex-col">
                                    <span class="text-xs font-mono uppercase tracking-widest text-amber-200/80">Action</span>
                                    <span class="text-lg font-bold tracking-wide">EXECUTE ORDER</span>
                                </div>
                                <IconArrowRight size={24} class="transform group-hover/btn:translate-x-1 transition-transform" />
                            </a>
                        </div>
                        
                        <div class="px-4 py-3 bg-[#0a0a0a] border-t border-white/5 flex justify-between items-center text-[10px] font-mono text-slate-600">
                            <span>Cost Basis: $0.00</span>
                            <span class="flex items-center gap-1">
                                <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                                Route: DIRECT
                            </span>
                        </div>
                    </div>
                </div>

                <div in:heavySlide={{ delay: 400 }} class="mt-16 pt-8 border-t border-white/5 grid grid-cols-2 md:grid-cols-4 gap-8">
                    <div class="text-center md:text-left">
                        <div class="flex items-center justify-center md:justify-start gap-2 text-slate-500 mb-1">
                            <IconServer size={16} />
                            <span class="text-[10px] font-mono uppercase tracking-widest">Network</span>
                        </div>
                        <div class="text-white font-serif text-lg">Global Edge</div>
                    </div>

                    <div class="text-center md:text-left">
                        <div class="flex items-center justify-center md:justify-start gap-2 text-slate-500 mb-1">
                            <IconActivity size={16} />
                            <span class="text-[10px] font-mono uppercase tracking-widest">Latency</span>
                        </div>
                        <div class="text-white font-serif text-lg">&lt; 20ms</div>
                    </div>

                    <div class="text-center md:text-left">
                        <div class="flex items-center justify-center md:justify-start gap-2 text-slate-500 mb-1">
                            <IconLockSquare size={16} />
                            <span class="text-[10px] font-mono uppercase tracking-widest">Security</span>
                        </div>
                        <div class="text-white font-serif text-lg">AES-256</div>
                    </div>

                    <div class="text-center md:text-left">
                        <div class="flex items-center justify-center md:justify-start gap-2 text-slate-500 mb-1">
                            <IconCheck size={16} />
                            <span class="text-[10px] font-mono uppercase tracking-widest">Commitment</span>
                        </div>
                        <div class="text-white font-serif text-lg">Cancel Anytime</div>
                    </div>
                </div>

            {/if}
        </div>
    </div>
</section>

<style>
    /* 3D Perspective for the Ticket */
    .perspective-1000 {
        perspective: 1000px;
    }

    /* Infinite Scroll Animation for the DOM data */
    @keyframes scroll-up {
        0% { transform: translateY(0); }
        100% { transform: translateY(-50%); }
    }
    @keyframes scroll-down {
        0% { transform: translateY(-50%); }
        100% { transform: translateY(0); }
    }

    .animate-scroll-up {
        animation: scroll-up 20s linear infinite;
    }
    .animate-scroll-down {
        animation: scroll-down 20s linear infinite;
    }
</style>