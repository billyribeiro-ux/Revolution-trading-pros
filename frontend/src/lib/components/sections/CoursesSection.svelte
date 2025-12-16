<script lang="ts">
    /**
     * CoursesSection - Apple/Netflix Cinematic Design
     * Upgraded with ICT9+ Layout, Motion, and Interaction Physics
     */
    import { onMount, onDestroy, tick } from 'svelte';
    import { browser } from '$app/environment';
    import { cubicOut, backOut } from 'svelte/easing';
    import { spring } from 'svelte/motion';
    
    // Tabler Icons (Preserving sub-path imports for tree-shaking)
    import IconSchool from '@tabler/icons-svelte/icons/school';
    import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
    import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
    import IconBrain from '@tabler/icons-svelte/icons/brain';
    import IconShield from '@tabler/icons-svelte/icons/shield';
    import IconClock from '@tabler/icons-svelte/icons/clock';
    import IconUsers from '@tabler/icons-svelte/icons/users';
    import IconStar from '@tabler/icons-svelte/icons/star-filled';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play-filled';
    import IconCertificate from '@tabler/icons-svelte/icons/certificate';
    import IconFlame from '@tabler/icons-svelte/icons/flame';
    import IconActivity from '@tabler/icons-svelte/icons/activity';

    // ============================================================================
    // COURSE DATA
    // ============================================================================
    const courses = [
        {
            id: 'day-trading',
            title: 'Day Trading Masterclass',
            subtitle: 'From Zero to Profitable',
            description: 'Master institutional-grade execution, order flow analysis, and real-time risk management.',
            level: 'Intermediate',
            duration: '8 Weeks',
            students: '2,847',
            rating: 4.9,
            reviews: 423,
            price: '$497',
            originalPrice: '$997',
            icon: IconChartCandle,
            gradient: 'from-blue-600 via-blue-500 to-cyan-500',
            bgGradient: 'from-blue-950/50 to-cyan-950/30',
            href: '/courses/day-trading-masterclass',
            badge: 'Best Seller',
            badgeColor: 'bg-amber-500',
            features: ['Live Trading Sessions', 'Real-Time Analysis', 'Risk Management'],
            imagePattern: 'radial-gradient(circle at 10% 20%, rgba(59, 130, 246, 0.1) 0%, transparent 20%)'
        },
        {
            id: 'swing-trading',
            title: 'Swing Trading Pro',
            subtitle: 'Capture Multi-Day Moves',
            description: 'Learn to identify high-probability setups and ride trends for maximum profit.',
            level: 'Beginner',
            duration: '6 Weeks',
            students: '3,421',
            rating: 4.8,
            reviews: 567,
            price: '$397',
            originalPrice: '$797',
            icon: IconChartLine,
            gradient: 'from-emerald-600 via-emerald-500 to-teal-500',
            bgGradient: 'from-emerald-950/50 to-teal-950/30',
            href: '/courses/swing-trading-pro',
            badge: 'Popular',
            badgeColor: 'bg-emerald-500',
            features: ['Weekly Setups', 'Trend Analysis', 'Exit Strategies'],
            imagePattern: 'radial-gradient(circle at 90% 80%, rgba(16, 185, 129, 0.1) 0%, transparent 20%)'
        },
        {
            id: 'options',
            title: 'Options Mastery',
            subtitle: 'Unlock Leverage Safely',
            description: 'Comprehensive options training from Greeks to advanced multi-leg strategies.',
            level: 'Advanced',
            duration: '10 Weeks',
            students: '1,892',
            rating: 4.9,
            reviews: 312,
            price: '$597',
            originalPrice: '$1,197',
            icon: IconBrain,
            gradient: 'from-violet-600 via-purple-500 to-fuchsia-500',
            bgGradient: 'from-violet-950/50 to-fuchsia-950/30',
            href: '/courses/options-trading',
            badge: 'Advanced',
            badgeColor: 'bg-violet-500',
            features: ['Greeks Mastery', 'Spreads & Straddles', 'Volatility Trading'],
            imagePattern: 'radial-gradient(circle at 50% 50%, rgba(139, 92, 246, 0.1) 0%, transparent 25%)'
        },
        {
            id: 'risk-management',
            title: 'Risk Management',
            subtitle: 'Protect Your Capital',
            description: 'The foundation of profitable trading. Learn position sizing and drawdown control.',
            level: 'All Levels',
            duration: '4 Weeks',
            students: '4,156',
            rating: 4.9,
            reviews: 689,
            price: '$297',
            originalPrice: '$497',
            icon: IconShield,
            gradient: 'from-amber-600 via-orange-500 to-red-500',
            bgGradient: 'from-amber-950/50 to-red-950/30',
            href: '/courses/risk-management',
            badge: 'Essential',
            badgeColor: 'bg-red-500',
            features: ['Position Sizing', 'Drawdown Control', 'Psychology'],
            imagePattern: 'radial-gradient(circle at 0% 100%, rgba(245, 158, 11, 0.1) 0%, transparent 20%)'
        }
    ];

    // ============================================================================
    // STATE & REFS
    // ============================================================================
    let sectionRef = $state<HTMLElement | null>(null);
    let cardsRef = $state<HTMLElement | null>(null);
    // ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
    let isVisible = $state(false);
    let hoveredCard = $state<string | null>(null);
    let gsapInstance: any = null;
    let scrollTriggerInstance: any = null;
    let prefersReducedMotion = $state(false);

    // Mouse Tracking for Spotlight Effect
    let mouseX = $state(0);
    let mouseY = $state(0);

    // ============================================================================
    // LIFECYCLE
    // ============================================================================
    onMount(() => {
        if (!browser) return;

        // Check for reduced motion preference
        prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
        
        // Trigger entrance animations with rAF for in: transitions
        requestAnimationFrame(() => {
            isVisible = true;
        });

        // Load GSAP asynchronously
        if (!prefersReducedMotion) {
            loadGSAP();
        }

        return () => {
            if (scrollTriggerInstance) scrollTriggerInstance.killAll();
        };
    });

    function handleMouseMove(e: MouseEvent) {
        if (cardsRef) {
            const rect = cardsRef.getBoundingClientRect();
            mouseX = e.clientX - rect.left;
            mouseY = e.clientY - rect.top;
        }
    }

    async function loadGSAP() {
        try {
            const gsapModule = await import('gsap');
            const scrollTriggerModule = await import('gsap/ScrollTrigger');
            const gsap = gsapModule.gsap || gsapModule.default;
            const ScrollTrigger = scrollTriggerModule.ScrollTrigger || scrollTriggerModule.default;
            gsap.registerPlugin(ScrollTrigger);
            gsapInstance = gsap;
            scrollTriggerInstance = ScrollTrigger;

            // Wait for cards to render
            await tick();

            // Animate cards on scroll with staggered parallax
            const cards = cardsRef?.querySelectorAll('.course-card');
            if (cards && cards.length > 0) {
                // Main Entrance
                gsap.fromTo(
                    cards,
                    { y: 100, opacity: 0, scale: 0.95 },
                    {
                        y: 0,
                        opacity: 1,
                        scale: 1,
                        duration: 1,
                        stagger: 0.1,
                        ease: 'power3.out', // More cinematic ease
                        scrollTrigger: {
                            trigger: cardsRef,
                            start: 'top 85%',
                            toggleActions: 'play none none reverse'
                        }
                    }
                );
            }
        } catch (e) {
            console.debug('[CoursesSection] GSAP not available:', e);
        }
    }

    // ============================================================================
    // TRANSITIONS
    // ============================================================================
    function slideUp(node: Element, { delay = 0, duration = 800 }) {
        return {
            delay,
            duration,
            css: (t: number) => {
                const eased = cubicOut(t);
                return `opacity: ${eased}; transform: translateY(${(1 - eased) * 40}px);`;
            }
        };
    }
</script>

<svelte:window onmousemove={handleMouseMove} />

<section
    bind:this={sectionRef}
    class="relative py-20 sm:py-32 overflow-hidden bg-[#050812] selection:bg-violet-500/30 selection:text-violet-200"
>
    <div class="absolute inset-0 pointer-events-none">
        <div class="absolute inset-0 opacity-[0.03] mix-blend-overlay" style="background-image: url('/textures/noise.svg');"></div>
        
        <div
            class="absolute top-[-10%] left-[-10%] w-[800px] h-[800px] bg-violet-600/10 rounded-full blur-[120px] mix-blend-screen animate-pulse-slow"
        ></div>
        <div
            class="absolute bottom-[-10%] right-[-10%] w-[800px] h-[800px] bg-cyan-600/10 rounded-full blur-[120px] mix-blend-screen animate-pulse-slow delay-1000"
        ></div>

        <div
            class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:60px_60px] [mask-image:radial-gradient(ellipse_at_center,black_40%,transparent_80%)]"
        ></div>
    </div>

    <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 z-10">
        {#if isVisible}
            <div class="text-center mb-12 sm:mb-20" in:slideUp={{ delay: 0, duration: 1000 }}>
                <div
                    class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-white/5 border border-white/10 mb-6 backdrop-blur-md shadow-lg shadow-black/20"
                >
                    <IconSchool class="w-4 h-4 text-violet-400" />
                    <span class="text-xs font-semibold text-violet-100 tracking-widest uppercase">Trading Academy</span>
                </div>

                <h2 class="text-4xl sm:text-5xl lg:text-7xl font-bold text-white mb-6 tracking-tight leading-tight">
                    Master the Markets<br />
                    <span
                        class="text-transparent bg-clip-text bg-gradient-to-r from-violet-400 via-fuchsia-400 to-cyan-400"
                    >
                        With Institutional Edge.
                    </span>
                </h2>

                <p class="text-lg sm:text-xl text-zinc-400 max-w-2xl mx-auto leading-relaxed font-light">
                    Structured curriculum designed by proprietary traders. <br class="hidden sm:block"/>Learn at your pace with lifetime access and real-time mentorship.
                </p>

                <div
                    class="grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto mt-12 pt-8 border-t border-white/5"
                    in:slideUp={{ delay: 200, duration: 800 }}
                >
                    <div class="text-center group cursor-default">
                        <div class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300">12k+</div>
                        <div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">Students</div>
                    </div>
                    <div class="text-center group cursor-default">
                        <div class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300">4.9</div>
                        <div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">Rating</div>
                    </div>
                    <div class="text-center group cursor-default">
                        <div class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300">89%</div>
                        <div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">Completion</div>
                    </div>
                    <div class="text-center group cursor-default">
                        <div class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300">24/7</div>
                        <div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">Support</div>
                    </div>
                </div>
            </div>
        {/if}

        <div
            bind:this={cardsRef}
            class="grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8 relative"
            style="--mouse-x: {mouseX}px; --mouse-y: {mouseY}px;"
        >
            <div 
                class="pointer-events-none absolute -inset-px opacity-0 md:opacity-100 transition-opacity duration-300 z-0 rounded-3xl"
                style="background: radial-gradient(800px circle at var(--mouse-x) var(--mouse-y), rgba(139, 92, 246, 0.08), transparent 40%);"
            ></div>

            {#each courses as course, i}
                <a
                    href={course.href}
                    class="course-card group relative rounded-[2rem] overflow-hidden bg-zinc-900/40 border border-white/5 hover:border-white/10 transition-all duration-500 hover:shadow-2xl hover:shadow-violet-900/20 active:scale-[0.99] z-10 isolate"
                    onmouseenter={() => (hoveredCard = course.id)}
                    onmouseleave={() => (hoveredCard = null)}
                    ontouchstart={() => (hoveredCard = course.id)}
                >
                    <div 
                        class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-700 ease-in-out"
                        style="background: {course.imagePattern}; filter: blur(40px);"
                    ></div>
                    
                    <div class="absolute inset-0 bg-gradient-to-br {course.bgGradient} opacity-50 group-hover:opacity-60 transition-opacity duration-500"></div>

                    <div class="relative p-6 sm:p-8 lg:p-10 h-full flex flex-col">
                        
                        <div class="flex items-start justify-between gap-4 mb-6">
                            <div class="flex items-center gap-4">
                                <div
                                    class="w-14 h-14 rounded-2xl bg-white/5 border border-white/10 flex items-center justify-center shadow-lg backdrop-blur-sm group-hover:scale-110 group-hover:rotate-3 transition-all duration-500 ease-out"
                                >
                                    {#if course.icon}
                                        {@const IconComponent = course.icon}
                                        <IconComponent class="w-7 h-7 text-white drop-shadow-[0_0_8px_rgba(255,255,255,0.3)]" />
                                    {/if}
                                </div>
                                <div>
                                    <span class="inline-flex items-center gap-1.5 text-xs font-bold text-violet-300 uppercase tracking-wider mb-1">
                                        {#if course.level === 'Advanced'}
                                            <IconFlame size={12} class="animate-pulse" />
                                        {:else if course.level === 'Intermediate'}
                                            <IconActivity size={12} />
                                        {/if}
                                        {course.level}
                                    </span>
                                    <h3 class="text-xl sm:text-2xl font-bold text-white tracking-tight leading-none group-hover:text-violet-200 transition-colors">
                                        {course.title}
                                    </h3>
                                </div>
                            </div>

                            <span
                                class="px-3 py-1 text-[10px] sm:text-xs font-bold rounded-full bg-white/10 text-white backdrop-blur-md border border-white/10 shadow-lg group-hover:bg-white/20 transition-colors"
                            >
                                {course.badge}
                            </span>
                        </div>

                        <p class="text-base text-zinc-400 mb-8 leading-relaxed line-clamp-2 group-hover:text-zinc-300 transition-colors duration-300">
                            {course.description}
                        </p>

                        <div class="flex gap-2 mb-8 overflow-x-auto pb-2 -mx-2 px-2 scrollbar-hide snap-x">
                            {#each course.features as feature}
                                <span
                                    class="snap-start flex-shrink-0 px-3 py-1.5 text-xs font-medium rounded-lg bg-white/5 text-zinc-300 border border-white/5 whitespace-nowrap group-hover:border-white/20 transition-colors"
                                >
                                    {feature}
                                </span>
                            {/each}
                        </div>

                        <div class="flex-grow"></div>

                        <div class="pt-6 border-t border-white/5 flex flex-col sm:flex-row sm:items-end justify-between gap-4">
                            
                            <div class="flex flex-col gap-2">
                                <div class="flex items-center gap-4 text-xs sm:text-sm text-zinc-500 font-medium">
                                    <div class="flex items-center gap-1.5">
                                        <IconClock class="w-4 h-4 text-zinc-600" />
                                        <span>{course.duration}</span>
                                    </div>
                                    <div class="flex items-center gap-1.5">
                                        <IconUsers class="w-4 h-4 text-zinc-600" />
                                        <span>{course.students}</span>
                                    </div>
                                </div>
                                <div class="flex items-baseline gap-2 mt-1">
                                    <span class="text-2xl font-bold text-white tracking-tight tabular-nums">{course.price}</span>
                                    <span class="text-sm text-zinc-600 line-through decoration-zinc-600/50 tabular-nums">{course.originalPrice}</span>
                                </div>
                            </div>

                            <div
                                class="flex items-center justify-center gap-2 px-6 py-3 rounded-xl bg-white text-black font-bold text-sm shadow-xl shadow-white/5 group-hover:shadow-white/20 transform group-hover:-translate-y-1 transition-all duration-300"
                            >
                                <span>Start Learning</span>
                                <IconArrowRight class="w-4 h-4" />
                            </div>
                        </div>
                    </div>

                    <div
                        class="hidden sm:flex absolute inset-0 items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none z-20"
                    >
                        <div class="absolute inset-0 bg-black/40 backdrop-blur-[2px]"></div>
                        <div
                            class="relative w-20 h-20 rounded-full bg-white/10 backdrop-blur-xl flex items-center justify-center border border-white/20 shadow-2xl scale-50 group-hover:scale-100 transition-transform duration-500 cubic-bezier(0.34, 1.56, 0.64, 1)"
                        >
                            <IconPlayerPlay class="w-8 h-8 text-white ml-1 drop-shadow-lg" />
                        </div>
                    </div>
                </a>
            {/each}
        </div>

        {#if isVisible}
            <div class="text-center mt-20" in:slideUp={{ delay: 400, duration: 800 }}>
                <a
                    href="/courses"
                    class="group inline-flex items-center justify-center gap-3 px-8 py-4 rounded-full bg-gradient-to-r from-violet-600 to-indigo-600 text-white font-semibold shadow-lg shadow-violet-500/25 hover:shadow-violet-500/50 active:scale-[0.98] hover:-translate-y-1 transition-all duration-300 relative overflow-hidden"
                >
                    <span class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-700 ease-in-out"></span>
                    <span class="text-base relative z-10">View Full Curriculum</span>
                    <IconArrowRight class="w-5 h-5 group-hover:translate-x-1 transition-transform relative z-10" />
                </a>

                <p class="mt-6 text-sm text-zinc-500 flex items-center justify-center gap-2 opacity-60 hover:opacity-100 transition-opacity">
                    <IconCertificate class="w-4 h-4 text-violet-500" />
                    Official certification included with all pathways
                </p>
            </div>
        {/if}
    </div>
</section>

<style>
    /* Utility for hiding scrollbars */
    .scrollbar-hide {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }
    .scrollbar-hide::-webkit-scrollbar {
        display: none;
    }

    /* Custom Pulse for background orbs */
    @keyframes pulse-slow {
        0%, 100% { opacity: 0.5; transform: scale(1); }
        50% { opacity: 0.8; transform: scale(1.1); }
    }
    .animate-pulse-slow {
        animation: pulse-slow 8s ease-in-out infinite;
    }
</style>