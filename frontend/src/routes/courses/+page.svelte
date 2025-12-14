<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { browser } from '$app/environment';
    import { slide, fade, fly } from 'svelte/transition';
    import { spring, tweened } from 'svelte/motion';
    import { cubicOut, elasticOut } from 'svelte/easing';
    
    // Icon Imports
    import IconSchool from '@tabler/icons-svelte/icons/school';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
    import IconClock from '@tabler/icons-svelte/icons/clock';
    import IconUsers from '@tabler/icons-svelte/icons/users';
    import IconCertificate from '@tabler/icons-svelte/icons/certificate';
    import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
    import IconBrain from '@tabler/icons-svelte/icons/brain';
    import IconShield from '@tabler/icons-svelte/icons/shield';
    import IconRocket from '@tabler/icons-svelte/icons/rocket';
    import IconStar from '@tabler/icons-svelte/icons/star';
    import IconCheck from '@tabler/icons-svelte/icons/check';
    import IconX from '@tabler/icons-svelte/icons/x';
    import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
    import IconHelp from '@tabler/icons-svelte/icons/help';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconActivity from '@tabler/icons-svelte/icons/activity';
    import IconLayoutGrid from '@tabler/icons-svelte/icons/layout-grid';
    import SEOHead from '$lib/components/SEOHead.svelte';

    // --- DATA MODELS ---

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
        gradient: string;
    }

    interface FaqItem {
        question: string;
        answer: string;
    }

    // --- CONTENT DATA ---

    const courses: Course[] = [
        {
            id: '1',
            title: 'Day Trading Masterclass',
            slug: 'day-trading-masterclass',
            description:
                'Stop chasing alerts. Learn to read the raw price action. This masterclass decodes institutional order flow, Level 2 data, and volume analysis to help you execute sniper-like entries in real-time.',
            targetAudience: 'For active traders wanting daily cash flow',
            level: 'Intermediate to Advanced',
            duration: '8 Weeks',
            students: '2,847',
            rating: 4.9,
            price: '$497',
            icon: IconChartCandle,
            features: [
                'Live tape reading & order flow analysis',
                'Pre-market planning routines',
                'Gap-and-go & reversal strategies',
                'Scaling in/out mechanics',
                'Handling emotional tilt in real-time'
            ],
            gradient: 'linear-gradient(135deg, #2e8eff 0%, #1e5cb8 100%)'
        },
        {
            id: '2',
            title: 'Swing Trading Pro',
            slug: 'swing-trading-pro',
            description:
                'Capture major market moves without being glued to your screen. We teach you how to identify macro trends and execute low-stress setups that compound wealth over days and weeks.',
            targetAudience: 'For part-time traders with full-time jobs',
            level: 'Beginner to Intermediate',
            duration: '6 Weeks',
            students: '3,421',
            rating: 4.8,
            price: '$397',
            icon: IconChartLine,
            features: [
                'Weekly timeframe context mastery',
                'Trendline & supply/demand zones',
                'Setting "Set & Forget" orders',
                'Portfolio rotation strategies',
                'Swing trading psychology'
            ],
            gradient: 'linear-gradient(135deg, #34d399 0%, #059669 100%)'
        },
        {
            id: '3',
            title: 'Options Trading Fundamentals',
            slug: 'options-trading',
            description:
                'Leverage is a double-edged sword. We teach you how to handle it safely. Move beyond simple calls and puts into defined-risk spreads that profit even if the market goes nowhere.',
            targetAudience: 'For traders seeking income & hedging',
            level: 'Intermediate',
            duration: '10 Weeks',
            students: '1,892',
            rating: 4.9,
            price: '$597',
            icon: IconBrain,
            features: [
                'The Greeks decoded (Delta, Theta, Vega)',
                'Income generation (Credit Spreads/Iron Condors)',
                'Hedging your long-term portfolio',
                'Implied Volatility crush setups',
                'Zero-DTE risk management'
            ],
            gradient: 'linear-gradient(135deg, #a78bfa 0%, #7c3aed 100%)'
        },
        {
            id: '4',
            title: 'Risk Management Mastery',
            slug: 'risk-management',
            description:
                'The only difference between a gambler and a professional is risk management. Learn the mathematical framework used by proprietary desks to ensure you never blow up your account.',
            targetAudience: 'Essential for EVERY trader',
            level: 'All Levels',
            duration: '4 Weeks',
            students: '4,156',
            rating: 5.0,
            price: '$297',
            icon: IconShield,
            features: [
                'The 1% Rule & Position Sizing math',
                'R-Multiples and Expectancy',
                'Drawdown recovery protocols',
                'Building a mechanical trading plan',
                'Account growth modeling'
            ],
            gradient: 'linear-gradient(135deg, #fb923c 0%, #ea580c 100%)'
        }
    ];

    const faqs: FaqItem[] = [
        {
            question: "I'm a complete beginner. Which course should I start with?",
            answer: "We recommend starting with 'Swing Trading Pro' combined with 'Risk Management Mastery'. Swing trading allows you more time to think and analyze without the high-pressure environment of day trading, while the risk management course ensures you protect your capital while you learn."
        },
        {
            question: "Is this just pre-recorded videos or is there live help?",
            answer: "It is a hybrid ecosystem. While the core curriculum is high-definition on-demand video (so you can learn at your pace), our value comes from the community. You get access to our Discord where mentors clarify concepts, and we host weekly live Q&A sessions for students."
        },
        {
            question: "Do I need a large account to start?",
            answer: "Absolutely not. In fact, we strongly encourage starting with a paper trading account (simulated money) or a very small micro-account. Our Risk Management course specifically teaches you how to grow a small account using percentage-based risk, rather than dollar-based risk."
        },
        {
            question: "How is this different from other 'gurus'?",
            answer: "Most 'gurus' sell signals so you become dependent on them. We teach you to fish. We focus heavily on risk management and psychology—the two things that actually determine longevity. We don't show off Lamborghinis; we show off consistent equity curves and disciplined execution."
        },
        {
            question: "Do I get lifetime access?",
            answer: "Yes. Once you purchase a course, you have lifetime access to the materials, including all future updates. Markets evolve, and so does our curriculum—you won't be charged extra for version 2.0 updates."
        }
    ];

    // --- STATE MANAGEMENT ---

    let heroVisible = false;
    let cardsVisible: boolean[] = new Array(courses.length).fill(false);
    let openFaqIndex: number | null = null;
    
    // NEW: Motion State
    let scrollY = 0;
    let innerHeight = 0;
    let mouseX = 0;
    let mouseY = 0;

    // Animated Counters
    const studentCount = tweened(0, { duration: 2000, easing: cubicOut });
    const ratingCount = tweened(0, { duration: 2000, easing: cubicOut });

    function toggleFaq(index: number) {
        openFaqIndex = openFaqIndex === index ? null : index;
    }

    function handleMouseMove(e: MouseEvent) {
        mouseX = e.clientX;
        mouseY = e.clientY;
        
        // Update CSS variables for spotlight effects globally or per container
        const cards = document.querySelectorAll('.course-card');
        cards.forEach((card) => {
            const rect = card.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            (card as HTMLElement).style.setProperty('--mouse-x', `${x}px`);
            (card as HTMLElement).style.setProperty('--mouse-y', `${y}px`);
        });
    }

    // --- SCHEMA MARKUP ---

    const coursesSchema = {
        '@context': 'https://schema.org',
        '@type': 'ItemList',
        '@id': 'https://revolutiontradingpros.com/courses/#courselist',
        name: 'Professional Trading Education Catalog',
        description: 'Institutional-grade trading education from Revolution Trading Pros.',
        numberOfItems: courses.length,
        itemListElement: courses.map((course, index) => ({
            '@type': 'ListItem',
            position: index + 1,
            item: {
                '@type': 'Course',
                name: course.title,
                description: course.description,
                provider: {
                    '@type': 'Organization',
                    name: 'Revolution Trading Pros',
                    sameAs: 'https://revolutiontradingpros.com'
                },
                educationalLevel: course.level,
                offers: {
                    '@type': 'Offer',
                    price: course.price.replace('$', ''),
                    priceCurrency: 'USD',
                    availability: 'https://schema.org/InStock'
                }
            }
        }))
    };

    // --- LIFECYCLE ---

    onMount(() => {
        if (!browser) return;

        // Trigger Counter Animation
        setTimeout(() => {
            studentCount.set(10000);
            ratingCount.set(4.9);
        }, 500);

        // Hero animation
        const heroObserver = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        heroVisible = true;
                    }
                });
            },
            { threshold: 0.2 }
        );

        const heroElement = document.querySelector('.hero-section');
        if (heroElement) heroObserver.observe(heroElement);

        // Card animations
        const cardObserver = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        const index = parseInt(entry.target.getAttribute('data-index') || '0');
                        cardsVisible[index] = true;
                    }
                });
            },
            { threshold: 0.1 }
        );

        const cardElements = document.querySelectorAll('.course-card');
        cardElements.forEach((card) => cardObserver.observe(card));

        return () => {
            heroObserver.disconnect();
            cardObserver.disconnect();
        };
    });
</script>

<svelte:window bind:scrollY bind:innerHeight onmousemove={handleMouseMove} />

<SEOHead
    title="Professional Trading Courses | Day, Swing & Options Mentorship"
    description="Stop trading alone. Join 10,000+ students in a supportive ecosystem. Learn real risk management, institutional strategies, and disciplined execution."
    canonical="/courses"
    ogType="website"
    ogImage="/og-image-courses.webp"
    schema={coursesSchema}
    schemaType="Course"
/>

<div class="courses-page antialiased selection:bg-blue-500/30 selection:text-blue-200">
    
    <section class="hero-section relative w-full overflow-hidden" class:visible={heroVisible}>
        <div class="hero-background absolute inset-0 z-0">
            <div class="perspective-grid" style="transform: translateY({scrollY * 0.5}px) rotateX(60deg);"></div>
            
            <div class="glow-orb glow-orb-1"></div>
            <div class="glow-orb glow-orb-2"></div>
            
            <div class="noise-overlay absolute inset-0 opacity-[0.03] pointer-events-none mix-blend-overlay"></div>
            <div class="grid-overlay"></div>
        </div>

        <div class="hero-content relative z-10 flex flex-col items-center justify-center min-h-[90vh] px-4 sm:px-6 lg:px-8 max-w-7xl mx-auto pt-20">
            
            <div class="flex flex-col items-center text-center space-y-8" 
                 style="transform: translateY({heroVisible ? 0 : 30}px); opacity: {heroVisible ? 1 : 0}; transition: all 1s cubic-bezier(0.16, 1, 0.3, 1);">
                
                <div class="hero-badge animate-pulse-soft">
                    <IconCertificate size={16} stroke={2} />
                    <span class="tracking-wide text-xs uppercase font-bold">Certified Trading Education</span>
                </div>

                <h1 class="hero-title max-w-4xl mx-auto">
                    Stop Guessing. Start Trading With<br />
                    <span class="gradient-text relative inline-block">
                        Institutional Precision.
                        <svg class="absolute -bottom-2 left-0 w-full h-2 text-blue-500/50" viewBox="0 0 100 10" preserveAspectRatio="none">
                            <path d="M0 5 Q 50 10 100 5" stroke="currentColor" stroke-width="2" fill="none" />
                        </svg>
                    </span>
                </h1>

                <p class="hero-description text-lg md:text-xl md:leading-relaxed max-w-2xl mx-auto text-slate-400">
                    The market is an efficient machine designed to transfer money from the undisciplined to the disciplined. 
                    Our curriculum bridges the gap between "retail gambling" and professional consistency through 
                    <strong class="text-white">real mentorship</strong>, <strong class="text-white">proven edge</strong>, and <strong class="text-white">uncompromising risk management</strong>.
                </p>

                <div class="hero-stats grid grid-cols-1 md:grid-cols-3 gap-8 py-8 w-full max-w-3xl border-y border-white/5 bg-white/[0.01] backdrop-blur-sm rounded-2xl">
                    <div class="stat-item flex flex-col items-center justify-center p-4">
                        <div class="flex items-center gap-3 mb-2 text-blue-400">
                            <IconUsers size={28} stroke={1.5} />
                        </div>
                        <div class="stat-content text-center">
                            <div class="stat-value text-3xl font-bold tabular-nums tracking-tight">
                                {Math.floor($studentCount).toLocaleString()}+
                            </div>
                            <div class="stat-label text-sm text-slate-500 font-medium uppercase tracking-wider">Community Members</div>
                        </div>
                    </div>
                    <div class="stat-item flex flex-col items-center justify-center p-4 border-t md:border-t-0 md:border-l border-white/5">
                        <div class="flex items-center gap-3 mb-2 text-emerald-400">
                            <IconStar size={28} stroke={1.5} />
                        </div>
                        <div class="stat-content text-center">
                            <div class="stat-value text-3xl font-bold tabular-nums tracking-tight">
                                {$ratingCount.toFixed(1)}/5.0
                            </div>
                            <div class="stat-label text-sm text-slate-500 font-medium uppercase tracking-wider">Student Rating</div>
                        </div>
                    </div>
                    <div class="stat-item flex flex-col items-center justify-center p-4 border-t md:border-t-0 md:border-l border-white/5">
                        <div class="flex items-center gap-3 mb-2 text-purple-400">
                            <IconRocket size={28} stroke={1.5} />
                        </div>
                        <div class="stat-content text-center">
                            <div class="stat-value text-3xl font-bold tabular-nums tracking-tight">24/7</div>
                            <div class="stat-label text-sm text-slate-500 font-medium uppercase tracking-wider">Mentor Support</div>
                        </div>
                    </div>
                </div>

                <div class="hero-cta flex flex-col sm:flex-row gap-4 w-full justify-center pt-4">
                    <a href="#courses" class="cta-button primary group relative overflow-hidden">
                        <span class="relative z-10 flex items-center gap-2">
                            View The Curriculum
                            <IconSchool size={20} stroke={2} class="group-hover:translate-x-1 transition-transform" />
                        </span>
                        <div class="absolute inset-0 bg-gradient-to-r from-blue-600 to-blue-400 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                    </a>
                    <a href="/mentorship" class="cta-button secondary group">
                        <span class="flex items-center gap-2">
                            How Mentorship Works
                            <IconArrowRight size={18} class="opacity-0 -ml-4 group-hover:opacity-100 group-hover:ml-0 transition-all" />
                        </span>
                    </a>
                </div>
            </div>
        </div>
        
        <div class="absolute bottom-0 w-full overflow-hidden py-3 bg-black/20 border-t border-white/5 backdrop-blur-md">
            <div class="ticker-wrap flex whitespace-nowrap">
                {#each Array(10) as _, i}
                    <div class="ticker-item flex gap-8 px-4 text-xs font-mono text-slate-500 opacity-70">
                        <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-emerald-500"></span> SPY $542.30</span>
                        <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-red-500"></span> QQQ $460.12</span>
                        <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-emerald-500"></span> NVDA $135.40</span>
                        <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-slate-500"></span> VIX 12.45</span>
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="reality-section relative z-10">
        <div class="reality-content container mx-auto px-4 md:px-8">
            <h2 class="section-title small">The Reality of Trading</h2>
            <div class="reality-grid gap-8">
                <div class="reality-card negative group" style="transform: translateY({scrollY * -0.05}px);">
                    <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                        <IconActivity size={120} />
                    </div>
                    <div class="icon-wrap error relative z-10"><IconX size={28} /></div>
                    <h3 class="relative z-10">Why 90% Fail</h3>
                    <p class="relative z-10">Most traders treat the market like a casino. They chase alerts, ignore risk, trade with emotion, and eventually blow up accounts due to a lack of structure.</p>
                </div>
                <div class="reality-card positive group" style="transform: translateY({scrollY * 0.02}px);"> <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                        <IconLayoutGrid size={120} />
                    </div>
                    <div class="icon-wrap success relative z-10"><IconCheck size={28} /></div>
                    <h3 class="relative z-10">How Our Students Succeed</h3>
                    <p class="relative z-10">We treat trading as a business. You will learn to protect your capital first, master one setup at a time, and rely on data—not feelings—to make decisions.</p>
                </div>
            </div>
        </div>
    </section>

    <section class="courses-section relative z-10" id="courses">
        <div class="absolute top-1/4 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-blue-500/20 to-transparent"></div>
        
        <div class="section-header relative">
            <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-blue-500/10 text-blue-400 text-xs font-bold uppercase tracking-widest mb-4 border border-blue-500/20">
                Pathways to Profitability
            </div>
            <h2 class="section-title">Institutional-Grade Curriculum</h2>
            <p class="section-description">
                Comprehensive training pathways designed to take you from "unconscious incompetence" to "unconscious competence."
            </p>
        </div>

        <div class="courses-grid px-4">
            {#each courses as course, index}
                {@const IconComponent = course.icon}
                <article
                    class="course-card group relative"
                    class:visible={cardsVisible[index]}
                    data-index={index}
                    style="--delay: {index * 0.1}s;"
                >
                    <div class="spotlight-glow" aria-hidden="true"></div>
                    
                    <div class="card-inner relative z-10 h-full flex flex-col bg-[#0a0f1e] rounded-[20px] overflow-hidden">
                        <div class="card-header relative overflow-hidden" style="background: {course.gradient}">
                            <div class="absolute inset-0 opacity-20 mix-blend-overlay" style="background-image: url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MCIgaGVpZ2h0PSI0MCIgdmlld0JveD0iMCAwIDQwIDQwIj48ZyBmaWxsLXJ1bGU9ImV2ZW5vZGQiPjxwYXRoIGQ9Ik0wIDQwTDQwIDBIOjB2NDB6bTQwIDBWMHjwIDQweiIgZmlsbD0iI2ZmZiIgZmlsbC1vcGFjaXR5PSIwLjEiLz48L2c+PC9zdmc+');"></div>
                            
                            <div class="relative z-10 flex justify-between items-start">
                                <div class="card-icon p-3 bg-white/10 backdrop-blur-md rounded-xl border border-white/20 shadow-lg group-hover:scale-110 transition-transform duration-500">
                                    <IconComponent size={32} stroke={1.5} class="text-white" />
                                </div>
                                <div class="card-badge shadow-lg">{course.level}</div>
                            </div>
                        </div>

                        <div class="card-content flex-grow">
                            <h3 class="card-title group-hover:text-blue-400 transition-colors">{course.title}</h3>
                            <p class="card-audience">
                                <IconUsers size={14} class="inline mr-1" />
                                {course.targetAudience}
                            </p>
                            <div class="h-[1px] w-12 bg-white/10 my-4"></div>
                            <p class="card-description">{course.description}</p>

                            <div class="card-meta bg-white/[0.03] rounded-lg p-3 border border-white/5">
                                <div class="meta-item">
                                    <IconClock size={16} stroke={2} class="text-blue-400" />
                                    <span>{course.duration}</span>
                                </div>
                                <div class="w-[1px] h-4 bg-white/10"></div>
                                <div class="meta-item">
                                    <IconUsers size={16} stroke={2} class="text-emerald-400" />
                                    <span>{course.students}</span>
                                </div>
                            </div>

                            <div class="mt-6">
                                <div class="features-label flex items-center gap-2">
                                    <span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span>
                                    What You'll Master:
                                </div>
                                <ul class="card-features">
                                    {#each course.features as feature}
                                        <li>
                                            <IconTrendingUp size={16} stroke={2} />
                                            <span>{feature}</span>
                                        </li>
                                    {/each}
                                </ul>
                            </div>
                        </div>

                        <div class="card-footer">
                            <div class="card-price">
                                <span class="price-label">Investment</span>
                                <span class="price-value">{course.price}</span>
                            </div>
                            <a href="/courses/{course.slug}" class="card-button group/btn">
                                <span>Start Learning</span>
                                <IconArrowRight size={18} stroke={2} class="group-hover/btn:translate-x-1 transition-transform" />
                            </a>
                        </div>
                    </div>
                </article>
            {/each}
        </div>
    </section>

    <div class="marquee-background py-4 opacity-5 bg-white/5 border-y border-white/5 overflow-hidden rotate-1 scale-110 pointer-events-none">
        <div class="flex whitespace-nowrap text-6xl font-black text-transparent stroke-text">
            RISK MANAGEMENT • DISCIPLINE • EDGE • EXECUTION • PATIENCE • RISK MANAGEMENT • DISCIPLINE • EDGE • EXECUTION • PATIENCE • 
        </div>
    </div>

    <section class="why-choose-section relative">
        <div class="why-choose-content">
            <h2 class="section-title">More Than Just Videos</h2>
            <p class="section-subtitle">We don't just sell you a course and disappear. We built the ecosystem we wished we had when we started.</p>

            <div class="benefits-grid">
                <div class="benefit-card group hover:bg-white/[0.06]">
                    <div class="benefit-icon group-hover:scale-110 group-hover:rotate-3 transition-transform duration-300">
                        <IconChartCandle size={32} stroke={1.5} />
                    </div>
                    <h3>Real, Verified Strategies</h3>
                    <p>
                        No hindsight trading. We teach strategies that we actually trade in our own accounts. You see the wins, the losses, and the logic behind every move.
                    </p>
                </div>

                <div class="benefit-card group hover:bg-white/[0.06]">
                    <div class="benefit-icon group-hover:scale-110 group-hover:rotate-3 transition-transform duration-300">
                        <IconUsers size={32} stroke={1.5} />
                    </div>
                    <h3>Supportive Community</h3>
                    <p>
                        Trading is lonely. Join a community of serious individuals who share charts, review trades, and lift each other up. No toxicity allowed.
                    </p>
                </div>

                <div class="benefit-card group hover:bg-white/[0.06]">
                    <div class="benefit-icon group-hover:scale-110 group-hover:rotate-3 transition-transform duration-300">
                        <IconBrain size={32} stroke={1.5} />
                    </div>
                    <h3>Psychology First</h3>
                    <p>
                        Strategy is 20%; psychology is 80%. We provide mental frameworks to help you stop self-sabotaging and stick to your plan.
                    </p>
                </div>

                <div class="benefit-card group hover:bg-white/[0.06]">
                    <div class="benefit-icon group-hover:scale-110 group-hover:rotate-3 transition-transform duration-300">
                        <IconCertificate size={32} stroke={1.5} />
                    </div>
                    <h3>Lifetime Updates</h3>
                    <p>
                        Markets change. Our courses are living documents. You get free updates whenever we refine our strategies or when market conditions shift.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <section class="faq-section relative">
        <div class="faq-container">
            <h2 class="section-title text-center">Common Questions</h2>
            <div class="faq-grid">
                {#each faqs as faq, i}
                    <div class="faq-item group" class:active={openFaqIndex === i}>
                        <button class="faq-question focus:outline-none" onclick={() => toggleFaq(i)} aria-expanded={openFaqIndex === i}>
                            <span class="question-text group-hover:text-blue-300 transition-colors">{faq.question}</span>
                            <span class="chevron w-8 h-8 flex items-center justify-center rounded-full bg-white/5" class:rotated={openFaqIndex === i}>
                                <IconChevronDown size={20} />
                            </span>
                        </button>
                        {#if openFaqIndex === i}
                            <div class="faq-answer" transition:slide={{ duration: 300, easing: cubicOut }}>
                                <div class="answer-content">
                                    {faq.answer}
                                </div>
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="final-cta">
        <div class="cta-content group">
            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/5 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-1000 ease-in-out"></div>
            
            <h2 class="cta-title">Ready to Treat Trading Like a Business?</h2>
            <p class="cta-text">Join the waitlist for our next intake or start a self-paced course today.</p>
            <a href="#courses" class="cta-button primary large shadow-2xl shadow-blue-500/20 group/btn relative overflow-hidden">
                <span class="relative z-10 flex items-center gap-2">
                    Find Your Course
                    <IconArrowRight size={24} class="group-hover/btn:translate-x-1 transition-transform duration-300" />
                </span>
                <div class="absolute inset-0 bg-gradient-to-r from-blue-600 to-blue-400 opacity-0 group-hover/btn:opacity-100 transition-opacity duration-300"></div>
            </a>
        </div>
    </section>
</div>

<style>
    /* While we use Tailwind for structure, these complex visual effects 
       ensure the "Apple/Netflix" quality regardless of the Tailwind config 
    */

    /* Base Styles */
    :global(html) {
        scroll-behavior: smooth;
    }

    .courses-page {
        min-height: 100vh;
        background: #050812; /* Deepest Navy/Black */
        color: white;
        font-family: 'Inter', system-ui, -apple-system, sans-serif;
        overflow-x: hidden;
    }

    /* Cinematic Hero */
    .hero-section {
        perspective: 1000px;
    }

    .perspective-grid {
        position: absolute;
        top: -50%;
        left: -50%;
        width: 200%;
        height: 200%;
        background-image: 
            linear-gradient(rgba(46, 142, 255, 0.1) 1px, transparent 1px),
            linear-gradient(90deg, rgba(46, 142, 255, 0.1) 1px, transparent 1px);
        background-size: 80px 80px;
        opacity: 0.15;
        mask-image: radial-gradient(circle at center, black 0%, transparent 70%);
    }

    .hero-background {
        background: radial-gradient(circle at 50% 0%, #111827 0%, #050812 100%);
    }

    .glow-orb {
        position: absolute;
        border-radius: 50%;
        filter: blur(100px);
        opacity: 0.4;
        animation: pulse 6s ease-in-out infinite alternate;
        will-change: transform, opacity;
    }

    .glow-orb-1 {
        width: 800px;
        height: 800px;
        background: radial-gradient(circle, rgba(46, 142, 255, 0.4) 0%, transparent 70%);
        top: -300px;
        left: 50%;
        transform: translateX(-50%);
    }

    .glow-orb-2 {
        width: 600px;
        height: 600px;
        background: radial-gradient(circle, rgba(52, 211, 153, 0.3) 0%, transparent 70%);
        bottom: -200px;
        left: 10%;
        animation-delay: -3s;
    }

    @keyframes pulse {
        0% { transform: translateX(-50%) scale(1); opacity: 0.3; }
        100% { transform: translateX(-50%) scale(1.1); opacity: 0.5; }
    }

    .hero-title {
        font-size: clamp(2.5rem, 5vw, 5rem);
        font-weight: 800;
        line-height: 1.05;
        letter-spacing: -0.03em;
        text-shadow: 0 10px 30px rgba(0,0,0,0.5);
    }

    .gradient-text {
        background: linear-gradient(135deg, #60a5fa 0%, #34d399 50%, #fff 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    .hero-cta .cta-button {
        padding: 1rem 2rem;
        border-radius: 99px; /* Pill shape */
        font-weight: 600;
        transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    }

    .cta-button.primary {
        background: #2e8eff;
        color: white;
        box-shadow: 0 0 0 0 rgba(46, 142, 255, 0.7);
    }

    .cta-button.primary:hover {
        transform: translateY(-2px);
        box-shadow: 0 10px 25px -5px rgba(46, 142, 255, 0.5);
    }

    .cta-button.secondary {
        background: rgba(255, 255, 255, 0.05);
        color: white;
        border: 1px solid rgba(255, 255, 255, 0.1);
        backdrop-filter: blur(10px);
    }
    
    .cta-button.secondary:hover {
        background: rgba(255, 255, 255, 0.1);
        border-color: rgba(255, 255, 255, 0.3);
    }

    /* Reality Section */
    .reality-section {
        padding: 8rem 0;
        background: linear-gradient(to bottom, transparent, rgba(0,0,0,0.5), transparent);
    }

    .section-title.small {
        font-size: 1rem;
        font-weight: 700;
        text-align: center;
        color: #94a3b8;
        text-transform: uppercase;
        letter-spacing: 0.2em;
        margin-bottom: 3rem;
    }

    .reality-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    }

    .reality-card {
        padding: 3rem;
        border-radius: 24px;
        background: rgba(255,255,255,0.02);
        border: 1px solid rgba(255,255,255,0.05);
        position: relative;
        overflow: hidden;
        transition: transform 0.1s linear; /* Smooth parallax */
    }

    .reality-card.negative { 
        border-color: rgba(239, 68, 68, 0.2);
        background: radial-gradient(circle at top right, rgba(239, 68, 68, 0.05), transparent 70%);
    }
    .reality-card.positive { 
        border-color: rgba(34, 197, 94, 0.2);
        background: radial-gradient(circle at top right, rgba(34, 197, 94, 0.05), transparent 70%);
    }

    .icon-wrap {
        display: inline-flex;
        padding: 1rem;
        border-radius: 16px;
        margin-bottom: 1.5rem;
    }
    .icon-wrap.error { color: #f87171; background: rgba(239, 68, 68, 0.1); }
    .icon-wrap.success { color: #4ade80; background: rgba(34, 197, 94, 0.1); }

    .reality-card h3 { font-size: 1.5rem; font-weight: 700; margin-bottom: 1rem; color: white; }
    .reality-card p { font-size: 1.05rem; color: #cbd5e1; line-height: 1.7; }

    /* Courses Section with Spotlight */
    .courses-section {
        padding: 6rem 0;
        max-width: 1400px;
        margin: 0 auto;
    }

    .section-title {
        font-size: clamp(2.5rem, 4vw, 3.5rem);
        font-weight: 800;
        margin-bottom: 1rem;
        letter-spacing: -0.02em;
        text-align: center;
    }
    
    .section-description {
        text-align: center;
        font-size: 1.125rem;
        color: #94a3b8;
        max-width: 600px;
        margin: 0 auto 4rem;
        line-height: 1.6;
    }

    .courses-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(360px, 1fr));
        gap: 2.5rem;
    }

    .course-card {
        transition: all 0.6s cubic-bezier(0.22, 1, 0.36, 1);
        opacity: 0;
        transform: translateY(40px);
        background: transparent; /* Wrapper is transparent, inner has color */
    }

    .course-card.visible {
        opacity: 1;
        transform: translateY(0);
        transition-delay: var(--delay);
    }
    
    .course-card:hover {
        transform: translateY(-5px);
    }

    /* SPOTLIGHT EFFECT */
    .spotlight-glow {
        position: absolute;
        inset: -1px;
        border-radius: 21px; /* Outer radius */
        background: radial-gradient(
            600px circle at var(--mouse-x, 0px) var(--mouse-y, 0px),
            rgba(255, 255, 255, 0.15),
            transparent 40%
        );
        opacity: 0;
        transition: opacity 0.3s;
        pointer-events: none;
        z-index: 0;
    }
    
    .course-card:hover .spotlight-glow {
        opacity: 1;
    }

    .card-header {
        padding: 2.5rem 2rem;
    }

    .card-badge {
        padding: 0.4rem 1rem;
        background: rgba(0, 0, 0, 0.4);
        backdrop-filter: blur(8px);
        border: 1px solid rgba(255,255,255,0.1);
        border-radius: 50px;
        font-size: 0.75rem;
        font-weight: 700;
        color: white;
        letter-spacing: 0.05em;
        text-transform: uppercase;
    }

    .card-content {
        padding: 2rem;
    }

    .card-title {
        font-size: 1.75rem;
        font-weight: 700;
        margin-bottom: 0.5rem;
        line-height: 1.2;
    }
    
    .card-audience {
        font-size: 0.9rem;
        color: #60a5fa; /* Blue-400 */
        font-weight: 500;
    }

    .card-description {
        font-size: 1rem;
        line-height: 1.6;
        color: #94a3b8;
    }

    .card-meta {
        display: flex;
        align-items: center;
        gap: 1.5rem;
    }

    .meta-item {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.9rem;
        color: #cbd5e1;
        font-weight: 500;
    }

    .features-label {
        font-size: 0.75rem;
        text-transform: uppercase;
        letter-spacing: 1.5px;
        color: #64748b;
        font-weight: 700;
        margin-bottom: 1rem;
    }

    .card-features {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .card-features li {
        display: flex;
        align-items: flex-start;
        gap: 0.75rem;
        padding: 0.6rem 0;
        font-size: 0.95rem;
        color: #e2e8f0;
    }

    .card-features li :global(svg) {
        color: #34d399; /* Emerald */
        flex-shrink: 0;
        margin-top: 3px;
    }

    .card-footer {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 1.5rem 2rem;
        background: rgba(0,0,0,0.2);
        border-top: 1px solid rgba(255, 255, 255, 0.05);
    }

    .price-label {
        display: block;
        font-size: 0.7rem;
        text-transform: uppercase;
        letter-spacing: 1px;
        color: #64748b;
        margin-bottom: 2px;
    }

    .price-value {
        font-size: 1.5rem;
        font-weight: 700;
        color: white;
    }

    .card-button {
        display: inline-flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1.5rem;
        background: white;
        color: #0f172a;
        text-decoration: none;
        border-radius: 12px;
        font-weight: 700;
        font-size: 0.95rem;
        transition: all 0.3s ease;
    }

    .card-button:hover {
        background: #f1f5f9;
        transform: translateY(-1px);
    }

    /* Benefits */
    .why-choose-section {
        padding: 8rem 2rem;
    }

    .why-choose-content { max-width: 1200px; margin: 0 auto; }

    .benefit-card {
        padding: 2.5rem;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 20px;
    }

    .benefit-icon {
        width: 64px;
        height: 64px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
        border: 1px solid rgba(255,255,255,0.1);
        border-radius: 16px;
        margin-bottom: 1.5rem;
        color: #38bdf8;
        box-shadow: 0 10px 30px -10px rgba(0,0,0,0.5);
    }

    .benefit-card h3 { font-size: 1.25rem; font-weight: 700; margin-bottom: 0.75rem; color: white; }
    .benefit-card p { font-size: 1rem; color: #94a3b8; line-height: 1.6; }

    /* FAQ */
    .faq-section { padding: 6rem 2rem; }
    .faq-container { max-width: 800px; margin: 0 auto; }
    
    .faq-grid { display: flex; flex-direction: column; gap: 1rem; margin-top: 3rem; }

    .faq-item {
        border: 1px solid rgba(255,255,255,0.08);
        border-radius: 16px;
        background: rgba(255,255,255,0.02);
        overflow: hidden;
    }

    .faq-item.active { 
        border-color: #2e8eff; 
        background: rgba(46, 142, 255, 0.03); 
    }

    .faq-question {
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 2rem;
        color: white;
        font-size: 1.1rem;
        font-weight: 600;
        text-align: left;
    }
    
    .chevron.rotated { transform: rotate(180deg); color: #2e8eff; background: rgba(46,142,255,0.1); }

    .answer-content {
        padding: 0 2rem 2rem;
        color: #cbd5e1;
        line-height: 1.7;
    }

    /* Final CTA */
    .final-cta { padding: 8rem 2rem; text-align: center; }
    
    .cta-content {
        max-width: 900px;
        margin: 0 auto;
        padding: 5rem 2rem;
        background: linear-gradient(180deg, rgba(46,142,255,0.08) 0%, rgba(5,8,18,0) 100%);
        border: 1px solid rgba(46,142,255,0.15);
        border-radius: 32px;
        position: relative;
        overflow: hidden;
    }
    
    .cta-title {
        font-size: clamp(2rem, 4vw, 3rem);
        font-weight: 800;
        margin-bottom: 1.5rem;
        line-height: 1.1;
    }
    
    .cta-text { font-size: 1.25rem; color: #94a3b8; margin-bottom: 3rem; }

    /* Stroke Text */
    .stroke-text {
        -webkit-text-stroke: 1px rgba(255,255,255,0.1);
        color: transparent;
    }

    /* Ticker Animation */
    @keyframes marquee {
        0% { transform: translateX(0); }
        100% { transform: translateX(-50%); }
    }
    
    .ticker-wrap { animation: marquee 30s linear infinite; }
    .marquee-background .flex { animation: marquee 120s linear infinite; }

    /* Mobile Adjustments */
    @media (max-width: 768px) {
        .hero-stats { grid-template-columns: 1fr; border: 1px solid rgba(255,255,255,0.1); border-radius: 16px; }
        .stat-item { border: none; border-bottom: 1px solid rgba(255,255,255,0.1); padding: 1.5rem; }
        .stat-item:last-child { border-bottom: none; }
        .card-footer { flex-direction: column; gap: 1rem; align-items: stretch; text-align: center; }
        .card-button { justify-content: center; }
        .reality-grid { grid-template-columns: 1fr; }
    }
</style>