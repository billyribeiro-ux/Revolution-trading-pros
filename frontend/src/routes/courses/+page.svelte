<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import { cubicOut } from 'svelte/easing';
    import { slide } from 'svelte/transition';
    import gsap from 'gsap';

    // --- ICONS ---
    import IconSchool from '@tabler/icons-svelte/icons/school';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
    import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
    import IconBrain from '@tabler/icons-svelte/icons/brain';
    import IconShield from '@tabler/icons-svelte/icons/shield';
    import IconRocket from '@tabler/icons-svelte/icons/rocket';
    import IconCheck from '@tabler/icons-svelte/icons/check';
    import IconX from '@tabler/icons-svelte/icons/x';
    import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconActivity from '@tabler/icons-svelte/icons/activity';
    import IconBolt from '@tabler/icons-svelte/icons/bolt';
    
    // Assumed existing component based on your snippet
    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- TYPES ---

    interface Course {
        id: string;
        title: string;
        slug: string;
        description: string;
        targetAudience: string;
        level: string;
        duration: string;
        students: string;
        rating: number;
        price: string;
        icon: any;
        features: string[];
        // Pre-computed Tailwind classes to avoid JIT interpolation issues
        colorClasses: {
            bg: string;
            border: string;
            text: string;
            icon: string;
        };
    }

    interface FaqItem {
        question: string;
        answer: string;
    }

    // --- DATA ---

    const courses: Course[] = [
        {
            id: '1',
            title: 'Day Trading Masterclass',
            slug: 'day-trading-masterclass',
            description: 'Decode institutional order flow, Level 2 data, and volume analysis to execute sniper-like entries.',
            targetAudience: 'Active Cash Flow',
            level: 'Advanced',
            duration: '8 Weeks',
            students: '2.8k+',
            rating: 4.9,
            price: '$497',
            icon: IconChartCandle,
            features: ['Tape Reading', 'L2 Analysis', 'Gap Strategies'],
            colorClasses: {
                bg: 'bg-blue-500/10',
                border: 'border-blue-500/20',
                text: 'text-blue-400',
                icon: 'text-blue-500'
            }
        },
        {
            id: '2',
            title: 'Swing Trading Pro',
            slug: 'swing-trading-pro',
            description: 'Capture major market moves. Identify macro trends and execute low-stress setups.',
            targetAudience: 'Wealth Compounding',
            level: 'Beginner',
            duration: '6 Weeks',
            students: '3.4k+',
            rating: 4.8,
            price: '$397',
            icon: IconChartLine,
            features: ['Macro Trends', 'Supply/Demand', 'Portfolio Rotation'],
            colorClasses: {
                bg: 'bg-emerald-500/10',
                border: 'border-emerald-500/20',
                text: 'text-emerald-400',
                icon: 'text-emerald-500'
            }
        },
        {
            id: '3',
            title: 'Options Tactics',
            slug: 'options-trading',
            description: 'Defined-risk spreads that profit even if the market goes nowhere. Handle leverage safely.',
            targetAudience: 'Income & Hedging',
            level: 'Intermediate',
            duration: '10 Weeks',
            students: '1.9k+',
            rating: 4.9,
            price: '$597',
            icon: IconBrain,
            features: ['The Greeks', 'Iron Condors', 'Volatility Crush'],
            colorClasses: {
                bg: 'bg-violet-500/10',
                border: 'border-violet-500/20',
                text: 'text-violet-400',
                icon: 'text-violet-500'
            }
        },
        {
            id: '4',
            title: 'Risk Protocol',
            slug: 'risk-management',
            description: 'The mathematical framework used by proprietary desks to ensure you never blow up.',
            targetAudience: 'Mandatory Foundation',
            level: 'All Levels',
            duration: '4 Weeks',
            students: '4.1k+',
            rating: 5.0,
            price: '$297',
            icon: IconShield,
            features: ['Position Sizing', 'R-Multiples', 'Drawdown Protocols'],
            colorClasses: {
                bg: 'bg-orange-500/10',
                border: 'border-orange-500/20',
                text: 'text-orange-400',
                icon: 'text-orange-500'
            }
        }
    ];

    const faqs: FaqItem[] = [
        {
            question: "Where do I start as a beginner?",
            answer: "Start with 'Swing Trading Pro' combined with 'Risk Management Mastery'. This combination builds the foundation of capital preservation and trend identification without the stress of intraday execution."
        },
        {
            question: "Is there live mentorship?",
            answer: "Yes. It's a hybrid ecosystem. Core curriculum is on-demand video, but our Discord provides daily mentor access, trade reviews, and weekly live Q&A sessions."
        },
        {
            question: "Do I need a large account?",
            answer: "No. We enforce starting with a simulator or micro-account. Our Risk Protocol teaches you to scale based on percentage performance, not dollar amount."
        },
        {
            question: "Lifetime access?",
            answer: "Guaranteed. You get all future updates for version 2.0, 3.0, and beyond at no extra cost. Markets evolve, and so does our content."
        }
    ];

    // --- SVELTE 5 RUNES STATE ---

    let scrollY = $state(0);
    let innerHeight = $state(0);
    let innerWidth = $state(0);
    let mounted = $state(false);
    
    // Mouse tracking for spotlight effects
    let mouseX = $state(0);
    let mouseY = $state(0);

    // Accordion State
    let openFaqIndex = $state<number | null>(0);

    // Canvas Ref
    let canvasRef: HTMLCanvasElement;

    // --- PARTICLE CLASS (moved to top level to avoid nested class warning) ---
    
    interface ParticleData {
        x: number;
        y: number;
        size: number;
        speedX: number;
        speedY: number;
        opacity: number;
    }

    function createParticle(width: number, height: number): ParticleData {
        return {
            x: Math.random() * width,
            y: Math.random() * height,
            size: Math.random() * 2,
            speedX: (Math.random() - 0.5) * 0.3,
            speedY: (Math.random() - 0.5) * 0.3,
            opacity: Math.random() * 0.5
        };
    }

    function updateParticle(p: ParticleData, width: number, height: number): void {
        p.x += p.speedX;
        p.y += p.speedY;
        if (p.x > width) p.x = 0;
        if (p.x < 0) p.x = width;
        if (p.y > height) p.y = 0;
        if (p.y < 0) p.y = height;
    }

    function drawParticle(ctx: CanvasRenderingContext2D, p: ParticleData): void {
        ctx.fillStyle = `rgba(100, 116, 139, ${p.opacity})`;
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fill();
    }

    // --- ACTIONS & UTILS ---

    function handleMouseMove(e: MouseEvent) {
        mouseX = e.clientX;
        mouseY = e.clientY;
        
        // Update CSS props for high-performance spotlights on specific containers
        const cards = document.querySelectorAll('.spotlight-card');
        cards.forEach((card) => {
             const rect = card.getBoundingClientRect();
             const x = e.clientX - rect.left;
             const y = e.clientY - rect.top;
             (card as HTMLElement).style.setProperty('--mouse-x', `${x}px`);
             (card as HTMLElement).style.setProperty('--mouse-y', `${y}px`);
        });
    }

    function toggleFaq(index: number) {
        if (openFaqIndex === index) {
            openFaqIndex = null;
        } else {
            openFaqIndex = index;
        }
    }

    // --- ANIMATION EFFECTS ---

    // Animated Numbers Helper
    function createCounter(target: number, duration: number = 2) {
        let current = $state(0);
        $effect(() => {
            if (mounted) {
                gsap.to((val: number) => current = val, {
                    duration,
                    val: target,
                    ease: "power2.out",
                    onUpdate: function() { current = Math.round(this.targets()[0].val * 10) / 10 }
                });
            }
        });
        return {
            get value() { return current; }
        };
    }

    const studentCounter = createCounter(12000);

    // Canvas Particle System (The "Order Flow" Background)
    function initCanvas() {
        if (!canvasRef) return;
        const ctx = canvasRef.getContext('2d');
        if (!ctx) return;

        const particles: ParticleData[] = [];
        const particleCount = innerWidth < 768 ? 30 : 60;

        for (let i = 0; i < particleCount; i++) {
            particles.push(createParticle(innerWidth, innerHeight));
        }

        function animate() {
            if (!ctx) return;
            ctx.clearRect(0, 0, innerWidth, innerHeight);
            
            ctx.strokeStyle = 'rgba(56, 189, 248, 0.05)';
            ctx.lineWidth = 1;

            for (let i = 0; i < particles.length; i++) {
                updateParticle(particles[i], innerWidth, innerHeight);
                drawParticle(ctx, particles[i]);
                
                for (let j = i; j < particles.length; j++) {
                    const dx = particles[i].x - particles[j].x;
                    const dy = particles[i].y - particles[j].y;
                    const distance = Math.sqrt(dx * dx + dy * dy);

                    if (distance < 150) {
                        ctx.beginPath();
                        ctx.moveTo(particles[i].x, particles[i].y);
                        ctx.lineTo(particles[j].x, particles[j].y);
                        ctx.stroke();
                    }
                }
            }
            requestAnimationFrame(animate);
        }

        animate();
    }

    onMount(() => {
        mounted = true;
        initCanvas();

        // GSAP Entrance Timeline
        const tl = gsap.timeline();
        
        tl.from('.hero-badge', { y: 20, opacity: 0, duration: 0.8, ease: 'power3.out' })
          .from('.hero-title-line', { y: 50, opacity: 0, duration: 1, stagger: 0.15, ease: 'power4.out' }, '-=0.4')
          .from('.hero-desc', { y: 20, opacity: 0, duration: 0.8, ease: 'power2.out' }, '-=0.6')
          .from('.hero-stats', { scale: 0.95, opacity: 0, duration: 0.8, ease: 'back.out(1.7)' }, '-=0.6')
          .from('.ticker-bar', { opacity: 0, y: 100, duration: 1 }, '-=0.8');

        return () => {
            // Cleanup handled by Svelte
        };
    });

    // Schema for SEO
    const schema = {
        '@context': 'https://schema.org',
        '@type': 'ItemList',
        '@id': 'https://revolutiontradingpros.com/courses/#courselist',
        name: 'Professional Trading Education Catalog',
        numberOfItems: courses.length,
        itemListElement: courses.map((course, index) => ({
            '@type': 'ListItem',
            position: index + 1,
            item: {
                '@type': 'Course',
                name: course.title,
                description: course.description,
                provider: { '@type': 'Organization', name: 'Revolution Trading Pros' },
                educationalLevel: course.level,
                offers: { '@type': 'Offer', price: course.price.replace('$', ''), priceCurrency: 'USD' }
            }
        }))
    };
</script>

<svelte:window bind:scrollY bind:innerHeight bind:innerWidth />

<SEOHead
    title="Trading Courses & Mentorship | Revolution Trading Pros"
    description="Institutional-grade trading education. Learn to read order flow, manage risk, and execute with precision. Join the top 1% of disciplined traders."
    canonical="/courses"
    schema={schema}
    schemaType="Course"
/>

<div 
    class="bg-black text-slate-200 min-h-screen font-sans selection:bg-blue-500/30 selection:text-white overflow-x-hidden"
    onmousemove={handleMouseMove}
    role="main"
>
    
    <section class="relative min-h-[95vh] flex items-center justify-center overflow-hidden pt-20">
        <div class="absolute inset-0 z-0 opacity-40">
            <canvas bind:this={canvasRef} width={innerWidth} height={innerHeight} class="w-full h-full"></canvas>
        </div>

        <div class="absolute inset-0 z-0 bg-gradient-to-b from-black/0 via-black/50 to-black pointer-events-none"></div>
        <div class="absolute inset-0 z-0 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-blue-900/10 via-black/0 to-black pointer-events-none"></div>

        <div class="relative z-10 container mx-auto px-6 flex flex-col items-center text-center">
            
            <div class="hero-badge inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-white/5 border border-white/10 backdrop-blur-md mb-8 shadow-[0_0_15px_rgba(56,189,248,0.15)]">
                <span class="relative flex h-2 w-2">
                  <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                  <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                </span>
                <span class="text-xs font-bold uppercase tracking-widest text-slate-300">Live Market Admissions Open</span>
            </div>

            <h1 class="max-w-5xl mx-auto mb-8 leading-[0.9]">
                <span class="hero-title-line block text-[13vw] md:text-[6rem] lg:text-[7.5rem] font-black tracking-tighter text-white mix-blend-overlay opacity-90">
                    EXECUTION
                </span>
                <span class="hero-title-line block text-4xl md:text-7xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 via-purple-400 to-white tracking-tight -mt-2 md:-mt-6">
                    OVER OPINION
                </span>
            </h1>

            <p class="hero-desc max-w-2xl text-lg md:text-xl text-slate-400 leading-relaxed mb-12 font-light">
                The market transfers money from the <span class="text-red-400 font-medium">impatient</span> to the <span class="text-emerald-400 font-medium">disciplined</span>. 
                Stop gambling on signals. Start trading with institutional edge.
            </p>

            <div class="hero-stats flex flex-col md:flex-row items-center gap-6 md:gap-12 p-2">
                <a href="#curriculum" class="group relative px-8 py-4 bg-white text-black rounded-full font-bold text-lg tracking-tight overflow-hidden transition-transform hover:scale-105 active:scale-95">
                    <div class="absolute inset-0 bg-gradient-to-r from-blue-200 to-white opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <span class="relative flex items-center gap-2">
                        View Curriculum
                        <IconArrowRight size={20} class="transition-transform group-hover:translate-x-1" />
                    </span>
                </a>

                <div class="flex items-center gap-8 text-sm font-medium text-slate-500 uppercase tracking-widest">
                    <div class="flex flex-col items-center">
                        <span class="text-white text-2xl font-bold tracking-tight normal-case">{Math.floor(studentCounter.value).toLocaleString()}+</span>
                        <span>Traders</span>
                    </div>
                    <div class="w-px h-8 bg-white/10"></div>
                    <div class="flex flex-col items-center">
                        <span class="text-white text-2xl font-bold tracking-tight normal-case">4.9/5</span>
                        <span>Rating</span>
                    </div>
                </div>
            </div>

        </div>

        <div class="ticker-bar absolute bottom-0 left-0 w-full border-t border-white/10 bg-black/40 backdrop-blur-md overflow-hidden py-3 z-20">
            <div class="flex animate-marquee whitespace-nowrap">
                {#each Array(8) as _}
                    <div class="flex items-center gap-8 px-4">
                        <span class="flex items-center gap-2 text-xs font-mono text-emerald-400"><IconTrendingUp size={14}/> SPY $542.30 +1.2%</span>
                        <span class="flex items-center gap-2 text-xs font-mono text-red-400"><IconActivity size={14}/> VIX 13.45 -2.1%</span>
                        <span class="flex items-center gap-2 text-xs font-mono text-emerald-400"><IconBolt size={14}/> NVDA $135.20 +3.4%</span>
                        <span class="flex items-center gap-2 text-xs font-mono text-slate-400">BTC $92,430 +0.1%</span>
                        <span class="text-xs font-mono text-slate-600">|</span>
                    </div>
                {/each}
            </div>
        </div>
    </section>


    <section class="py-32 relative z-10">
        <div class="container mx-auto px-6">
            <div class="grid lg:grid-cols-2 gap-16 items-center">
                <div class="space-y-8">
                    <h2 class="text-4xl md:text-5xl font-bold tracking-tight text-white">
                        The <span class="text-blue-500">Retail</span> Trap.
                    </h2>
                    <p class="text-xl text-slate-400 leading-relaxed">
                        90% of traders fail because they treat the market like a casino. They chase alerts, lack a risk model, and trade their P&L instead of the chart.
                    </p>
                    
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <div class="p-6 rounded-2xl bg-red-500/5 border border-red-500/10">
                            <IconX class="text-red-500 mb-4" size={32} />
                            <h3 class="text-white font-bold mb-2">Gambling</h3>
                            <p class="text-sm text-slate-400">Entry based on "feeling" rather than statistical edge.</p>
                        </div>
                        <div class="p-6 rounded-2xl bg-emerald-500/5 border border-emerald-500/10">
                            <IconCheck class="text-emerald-500 mb-4" size={32} />
                            <h3 class="text-white font-bold mb-2">Business</h3>
                            <p class="text-sm text-slate-400">Execution based on a pre-defined, backtested playbook.</p>
                        </div>
                    </div>
                </div>
                
                <div class="relative aspect-square md:aspect-video lg:aspect-square rounded-3xl overflow-hidden border border-white/10 bg-white/5 shadow-2xl">
                    <div class="absolute inset-0 bg-[url('https://assets.codepen.io/907368/noise.svg')] opacity-20"></div>
                    <div class="absolute inset-0 flex items-center justify-center">
                        <div class="relative w-3/4 h-3/4 border border-white/10 bg-black/50 backdrop-blur-xl rounded-xl p-6 flex flex-col">
                            <div class="flex items-center justify-between mb-8 border-b border-white/5 pb-4">
                                <div class="flex gap-2">
                                    <div class="w-3 h-3 rounded-full bg-red-500/50"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500/50"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500/50"></div>
                                </div>
                                <div class="text-[10px] font-mono text-slate-500 uppercase">System: Active</div>
                            </div>
                            <div class="flex-1 flex items-end gap-1">
                                {#each Array(20) as _, i}
                                    <div 
                                        class="flex-1 bg-blue-500/50 rounded-t-sm animate-pulse"
                                        style="height: {30 + Math.random() * 60}%; animation-delay: {i * 0.1}s; opacity: {0.3 + Math.random() * 0.7}"
                                    ></div>
                                {/each}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>


    <section id="curriculum" class="py-32 relative z-10 bg-slate-950/50 border-y border-white/5">
        <div class="container mx-auto px-6">
            
            <div class="text-center max-w-3xl mx-auto mb-20">
                <span class="text-blue-500 font-mono text-xs uppercase tracking-[0.3em] mb-4 block">Classified Intel</span>
                <h2 class="text-4xl md:text-6xl font-bold tracking-tighter text-white mb-6">Master The Setup.</h2>
                <p class="text-slate-400 text-lg">Four specialized pathways designed to take you from unconscious incompetence to unconscious competence.</p>
            </div>

            <div class="grid md:grid-cols-2 gap-6 lg:gap-8 max-w-6xl mx-auto">
                {#each courses as course}
                    {@const Icon = course.icon}
                    {@const colors = course.colorClasses}
                    <div 
                        class="spotlight-card group relative h-full bg-black border border-white/10 rounded-3xl overflow-hidden hover:border-white/20 transition-colors duration-500"
                    >
                        <div 
                            class="absolute pointer-events-none -inset-px opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-0"
                            style="
                                background: radial-gradient(600px circle at var(--mouse-x) var(--mouse-y), rgba(255,255,255,0.06), transparent 40%);
                            "
                        ></div>

                        <div class="relative z-10 p-8 md:p-10 flex flex-col h-full bg-black/40 backdrop-blur-sm">
                            <div class="flex justify-between items-start mb-8">
                                <div class={`p-3 rounded-2xl ${colors.bg} ${colors.border} border ${colors.text}`}>
                                    <Icon size={32} stroke={1.5} />
                                </div>
                                <span class="px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider bg-white/5 border border-white/10 text-slate-300">
                                    {course.level}
                                </span>
                            </div>

                            <h3 class="text-2xl font-bold text-white mb-2 group-hover:text-blue-400 transition-colors">{course.title}</h3>
                            <p class="text-sm font-mono text-slate-500 mb-6">{course.targetAudience}</p>
                            
                            <div class="h-px w-full bg-white/5 mb-6"></div>
                            
                            <p class="text-slate-400 leading-relaxed mb-8 flex-grow">{course.description}</p>

                            <div class="space-y-3 mb-8">
                                {#each course.features as feature}
                                    <div class="flex items-center gap-3 text-sm text-slate-300">
                                        <IconCheck size={16} class={colors.icon} />
                                        {feature}
                                    </div>
                                {/each}
                            </div>

                            <div class="flex items-center justify-between pt-6 border-t border-white/5">
                                <div>
                                    <div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Investment</div>
                                    <div class="text-xl font-bold text-white">{course.price}</div>
                                </div>
                                <a 
                                    href={`/courses/${course.slug}`} 
                                    class="flex items-center gap-2 px-5 py-2.5 rounded-lg bg-white text-black font-semibold text-sm hover:bg-slate-200 transition-colors"
                                >
                                    Start <IconArrowRight size={16} />
                                </a>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>

        </div>
    </section>


    <section class="py-32 relative z-10 bg-black">
        <div class="container mx-auto px-6 max-w-3xl">
            <h2 class="text-3xl font-bold text-white mb-12 text-center">Protocol FAQ</h2>
            
            <div class="space-y-4">
                {#each faqs as faq, i}
                    <button 
                        onclick={() => toggleFaq(i)}
                        class="w-full text-left group bg-white/[0.02] hover:bg-white/[0.04] border border-white/5 rounded-xl transition-all duration-300 overflow-hidden"
                    >
                        <div class="p-6 flex items-center justify-between">
                            <span class="text-lg font-medium text-slate-200 group-hover:text-white transition-colors">{faq.question}</span>
                            <IconChevronDown 
                                size={20} 
                                class={`text-slate-500 transition-transform duration-300 ${openFaqIndex === i ? 'rotate-180 text-blue-400' : ''}`}
                            />
                        </div>
                        
                        {#if openFaqIndex === i}
                            <div 
                                transition:slide={{ duration: 300, easing: cubicOut }}
                                class="px-6 pb-6"
                            >
                                <p class="text-slate-400 leading-relaxed pt-2 border-t border-white/5">
                                    {faq.answer}
                                </p>
                            </div>
                        {/if}
                    </button>
                {/each}
            </div>
        </div>
    </section>

    <section class="relative py-40 overflow-hidden">
        <div class="absolute inset-0 bg-blue-600/5 blur-[100px]"></div>
        
        <div class="container mx-auto px-6 relative z-10 text-center">
            <IconRocket size={48} stroke={1} class="mx-auto text-blue-500 mb-8 animate-bounce" />
            <h2 class="text-5xl md:text-7xl font-black tracking-tighter text-white mb-8">
                Market Opens In... Now.
            </h2>
            <p class="text-xl text-slate-400 mb-12 max-w-xl mx-auto">
                Join 10,000+ disciplined traders building wealth through logic, not luck.
            </p>
            
            <a href="#curriculum" class="inline-flex items-center gap-3 px-10 py-5 bg-blue-600 hover:bg-blue-500 text-white rounded-full font-bold text-lg transition-all shadow-[0_0_40px_rgba(37,99,235,0.3)] hover:shadow-[0_0_60px_rgba(37,99,235,0.5)]">
                <IconSchool size={24} />
                Enroll Now
            </a>
        </div>
    </section>

</div>

<style>
    /* Custom Tailwind-compatible utilities for specific animations */
    
    @keyframes marquee {
        0% { transform: translateX(0); }
        100% { transform: translateX(-50%); }
    }
    
    .animate-marquee {
        animation: marquee 40s linear infinite;
    }

    /* Smooth Scroll Behavior Override */
    :global(html) {
        scroll-behavior: smooth;
    }
    
    /* Font rendering fix for dark mode */
    :global(body) {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        background-color: black;
    }
</style>