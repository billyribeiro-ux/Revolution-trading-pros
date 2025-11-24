<script lang="ts">
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import IconTarget from '@tabler/icons-svelte/icons/target';
    import IconHandshake from '@tabler/icons-svelte/icons/heart-handshake';
    import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
    import IconUsers from '@tabler/icons-svelte/icons/users';
    import IconRocket from '@tabler/icons-svelte/icons/rocket';
    import IconShieldCheck from '@tabler/icons-svelte/icons/shield-check';
    import IconScale from '@tabler/icons-svelte/icons/scale';
    import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
    import IconSchool from '@tabler/icons-svelte/icons/school';

    let isVisible = false;
    let heroRef: HTMLElement;
    let statsRef: HTMLElement;

    // Enhanced Pillars
    const pillars = [
        {
            icon: IconShieldCheck,
            title: 'Radical Transparency',
            description:
                'In an industry of smoke and mirrors, we are the glass house. We publish real track records—wins, losses, and the lessons in between. No "photoshop" returns, only raw data.'
        },
        {
            icon: IconUsers,
            title: 'Collective Alpha',
            description:
                'Trading is often a lonely endeavor. We replace isolation with a hive-mind of disciplined professionals sharing setups, risk assessments, and real-time market sentiment.'
        },
        {
            icon: IconScale,
            title: 'Probabilistic Thinking',
            description:
                'We reject gambling. We teach the mathematics of "Edge"—teaching our members to think in probabilities, expected value (EV), and risk units (R-multiples).'
        }
    ];

    // The "Methodology" Steps
    const steps = [
        {
            icon: IconSchool,
            title: 'Foundational Mastery',
            text: 'Moving beyond basic patterns to understand market auction theory and liquidity.'
        },
        {
            icon: IconChartLine,
            title: 'System Development',
            text: 'Helping traders build back-tested, data-driven playbooks that fit their personality.'
        },
        {
            icon: IconHandshake,
            title: 'Psychological Calibration',
            text: 'Mastering the internal game to execute without fear, hesitation, or greed.'
        }
    ];

    const manifestoItems = [
        'We prioritize capital preservation over aggressive growth.',
        'We never sell "signals" without teaching the logic behind them.',
        'We treat trading as a business, not a casino.',
        'We believe consistency is a result of routine, not luck.',
        'We own our mistakes and view losses as tuition.',
        'We act as fiduciaries to our own education and growth.'
    ];

    onMount(() => {
        if (!browser) return;

        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        entry.target.classList.add('animate-in');
                    }
                });
            },
            { threshold: 0.15 }
        );

        if (heroRef) observer.observe(heroRef);
        if (statsRef) observer.observe(statsRef);
        
        document.querySelectorAll('.animate-on-scroll').forEach((el) => observer.observe(el));

        return () => observer.disconnect();
    });
</script>

<svelte:head>
    <title>Our Mission | Revolution Trading Pros</title>
    <meta
        name="description"
        content="Our mission is to bridge the gap between retail and institutional trading through data, discipline, and radical transparency."
    />
</svelte:head>

<div class="mission-page">
    
    <div class="fixed inset-0 pointer-events-none z-0">
        <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[80vw] h-[80vw] bg-blue-900/10 rounded-full blur-[120px]"></div>
    </div>

    <section class="hero-section" bind:this={heroRef}>
        <div class="hero-container">
            <div class="hero-content">
                <div class="icon-wrapper">
                    <IconTarget size={64} class="hero-icon" stroke={1.5} />
                </div>
                <h1 class="hero-title">We Don't Sell Dreams.<br />We Build <span class="text-gradient">Careers.</span></h1>
                
                <div class="mission-statement">
                    <p class="statement-text">
                        Our mission is to dismantle the "Retail Trader" stereotype and rebuild it with 
                        <span class="highlight">Institutional DNA</span>. We exist to transition you from 
                        a gambler seeking action to a risk manager executing an edge.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <section class="reality-section animate-on-scroll">
        <div class="container">
            <div class="section-badge">
                <IconTrendingUp size={16} />
                <span>THE REALITY GAP</span>
            </div>
            
            <div class="split-layout">
                <div class="text-content">
                    <h2 class="section-title">The "90/90/90" Rule</h2>
                    <p class="lead-text">
                        It is a known statistic in the brokerage industry: <strong>90% of retail traders lose 90% of their money in 90 days.</strong>
                    </p>
                    <p>
                        Why? Because they enter a battlefield against supercomputers and hedge funds armed only with a smartphone and "hope." They lack the mathematical framework to survive variance.
                    </p>
                    <p>
                        We illustrate the difference between gambling and trading using clear data.
                        

[Image of risk reward ratio diagram]

                        Professional trading is not about predicting the future; it is about managing the risk-to-reward ratio over a large sample size of events.
                    </p>
                </div>
                
                <div class="stats-grid" bind:this={statsRef}>
                    <div class="stat-card negative">
                        <span class="stat-val">90%</span>
                        <span class="stat-desc">Retail Failure Rate</span>
                    </div>
                    <div class="stat-card positive">
                        <span class="stat-val">Top 1%</span>
                        <span class="stat-desc">The Standard We Set</span>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="pillars-section">
        <div class="container">
            <div class="section-header">
                <h2 class="section-title">The Foundation</h2>
                <p class="section-subtitle">The three non-negotiables of our ecosystem.</p>
            </div>
            <div class="pillars-grid">
                {#each pillars as pillar}
                    <div class="pillar-card animate-on-scroll">
                        <div class="pillar-icon-wrapper">
                            <svelte:component this={pillar.icon} size={40} class="pillar-icon" stroke={1.5} />
                        </div>
                        <h3 class="pillar-title">{pillar.title}</h3>
                        <p class="pillar-description">{pillar.description}</p>
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="visual-section animate-on-scroll">
        <div class="container">
            <div class="visual-card">
                <div class="visual-text">
                    <h3>Technical Precision</h3>
                    <p>We believe in reading the raw language of the market. No magic indicators, just price, volume, and structure.</p>
                </div>
                <div class="visual-image-placeholder">
                    

[Image of candlestick chart with support and resistance levels]

                    <span class="caption">Visualizing market structure and liquidity zones.</span>
                </div>
            </div>
        </div>
    </section>

    <section class="methodology-section">
        <div class="container">
            <div class="section-header">
                <h2 class="section-title">The Path to Professionalism</h2>
                <p class="section-subtitle">How we actually bridge the gap.</p>
            </div>
            
            <div class="steps-container">
                {#each steps as step, i}
                    <div class="step-card animate-on-scroll" style="--delay: {i * 0.2}s">
                        <div class="step-number">0{i + 1}</div>
                        <div class="step-content">
                            <h3 class="step-title">{step.title}</h3>
                            <p class="step-text">{step.text}</p>
                        </div>
                        <svelte:component this={step.icon} size={64} class="step-icon-bg" stroke={1} />
                    </div>
                {/each}
            </div>
        </div>
    </section>

    <section class="manifesto-section animate-on-scroll">
        <div class="container">
            <div class="manifesto-wrapper">
                <div class="manifesto-header">
                    <h2 class="section-title">Our Manifesto</h2>
                    <p>We commit to these truths.</p>
                </div>
                <div class="manifesto-list">
                    {#each manifestoItems as item}
                        <div class="manifesto-item">
                            <IconShieldCheck size={24} class="check-icon" />
                            <span>{item}</span>
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    </section>

    <section class="cta-section">
        <div class="cta-container animate-on-scroll">
            <div class="cta-content">
                <IconRocket size={48} class="cta-icon-float" />
                <h2 class="cta-title">Stop Gambling. Start Operating.</h2>
                <p class="cta-subtitle">
                    Join the only trading environment dedicated to the professional development of the retail trader.
                </p>
                <div class="cta-buttons">
                    <a href="/pricing" class="cta-primary">
                        <span>Join the Professional Tier</span>
                    </a>
                    <a href="/methodology" class="cta-secondary">
                        <span>Read Our Methodology</span>
                    </a>
                </div>
            </div>
        </div>
    </section>
</div>

<style>
    /* Base & Layout */
    .mission-page {
        background-color: #050505;
        color: #e2e8f0;
        font-family: 'Inter', sans-serif;
        overflow-x: hidden;
        position: relative;
    }

    .container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 1.5rem;
    }

    .section-header {
        text-align: center;
        margin-bottom: 4rem;
    }

    .section-title {
        font-size: clamp(2rem, 4vw, 3rem);
        font-weight: 800;
        color: #ffffff;
        margin: 0;
        letter-spacing: -0.02em;
    }

    .section-subtitle {
        font-size: 1.1rem;
        color: #94a3b8;
        margin-top: 1rem;
    }

    .section-badge {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        background: rgba(46, 142, 255, 0.1);
        border: 1px solid rgba(46, 142, 255, 0.2);
        border-radius: 100px;
        color: #2e8eff;
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 0.1em;
        margin-bottom: 1.5rem;
    }

    .text-gradient {
        background: linear-gradient(135deg, #2e8eff 0%, #60a5fa 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    /* Hero */
    .hero-section {
        padding: 12rem 0 8rem;
        position: relative;
        z-index: 10;
    }

    .hero-container {
        max-width: 1000px;
        margin: 0 auto;
        padding: 0 1.5rem;
        text-align: center;
    }

    .hero-content {
        opacity: 0;
        transform: translateY(30px);
        animation: fadeInUp 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    }

    .icon-wrapper {
        display: inline-flex;
        padding: 1.5rem;
        background: radial-gradient(circle, rgba(46, 142, 255, 0.1) 0%, transparent 70%);
        border: 1px solid rgba(46, 142, 255, 0.2);
        border-radius: 50%;
        margin-bottom: 2rem;
    }

    :global(.hero-icon) { color: #2e8eff; }

    .hero-title {
        font-size: clamp(3rem, 7vw, 5.5rem);
        font-weight: 900;
        line-height: 1.1;
        margin-bottom: 2.5rem;
        color: white;
    }

    .mission-statement {
        max-width: 800px;
        margin: 0 auto;
        padding: 2.5rem;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid rgba(255, 255, 255, 0.05);
        border-radius: 20px;
        backdrop-filter: blur(10px);
    }

    .statement-text {
        font-size: 1.25rem;
        line-height: 1.7;
        color: #cbd5e1;
    }

    .highlight {
        color: white;
        font-weight: 700;
        text-decoration: underline;
        text-decoration-color: #2e8eff;
        text-underline-offset: 4px;
    }

    /* Reality Section */
    .reality-section {
        padding: 6rem 0;
        background: linear-gradient(to bottom, transparent, rgba(15, 23, 42, 0.3));
    }

    .split-layout {
        display: grid;
        grid-template-columns: 1.2fr 0.8fr;
        gap: 4rem;
        align-items: center;
    }

    .lead-text {
        font-size: 1.5rem;
        color: white;
        margin-bottom: 1.5rem;
        line-height: 1.4;
    }

    .text-content p {
        color: #94a3b8;
        margin-bottom: 1.5rem;
        line-height: 1.7;
    }

    .stats-grid {
        display: grid;
        gap: 1.5rem;
    }

    .stat-card {
        padding: 2rem;
        border-radius: 16px;
        text-align: center;
    }

    .stat-card.negative {
        background: rgba(239, 68, 68, 0.05);
        border: 1px solid rgba(239, 68, 68, 0.2);
    }

    .stat-card.positive {
        background: rgba(16, 185, 129, 0.05);
        border: 1px solid rgba(16, 185, 129, 0.2);
    }

    .stat-val {
        display: block;
        font-size: 3rem;
        font-weight: 900;
        margin-bottom: 0.5rem;
    }

    .negative .stat-val { color: #ef4444; }
    .positive .stat-val { color: #10b981; }

    .stat-desc {
        font-size: 0.9rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        font-weight: 600;
        color: #e2e8f0;
    }

    /* Pillars */
    .pillars-section {
        padding: 8rem 0;
    }

    .pillars-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 2rem;
    }

    .pillar-card {
        padding: 2.5rem;
        background: #0a0a0a;
        border: 1px solid #1f1f1f;
        border-radius: 20px;
        transition: transform 0.3s ease, border-color 0.3s ease;
    }

    .pillar-card:hover {
        border-color: #2e8eff;
        transform: translateY(-5px);
    }

    .pillar-icon-wrapper {
        margin-bottom: 1.5rem;
        display: inline-flex;
        padding: 1rem;
        background: rgba(46, 142, 255, 0.1);
        border-radius: 12px;
        color: #2e8eff;
    }

    .pillar-title {
        font-size: 1.5rem;
        font-weight: 700;
        color: white;
        margin-bottom: 1rem;
    }

    .pillar-description {
        color: #94a3b8;
        line-height: 1.6;
    }

    /* Visual Section */
    .visual-section {
        padding: 4rem 0;
    }

    .visual-card {
        background: #0f172a;
        border-radius: 24px;
        border: 1px solid #1e293b;
        padding: 3rem;
        display: grid;
        grid-template-columns: 1fr 1.5fr;
        gap: 3rem;
        align-items: center;
    }

    .visual-text h3 {
        font-size: 2rem;
        color: white;
        margin-bottom: 1rem;
    }
    
    .visual-text p {
        color: #94a3b8;
    }

    .visual-image-placeholder {
        background: #1e293b;
        border-radius: 12px;
        height: 250px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: #475569;
        font-style: italic;
        border: 2px dashed #334155;
    }
    
    .caption {
        font-size: 0.8rem;
        margin-top: 1rem;
        color: #64748b;
    }

    /* Methodology */
    .methodology-section {
        padding: 8rem 0;
    }

    .steps-container {
        display: grid;
        gap: 2rem;
    }

    .step-card {
        display: flex;
        align-items: flex-start;
        gap: 2rem;
        padding: 2.5rem;
        background: rgba(255, 255, 255, 0.02);
        border-left: 4px solid #333;
        border-radius: 0 16px 16px 0;
        position: relative;
        overflow: hidden;
        transition: all 0.3s ease;
    }

    .step-card:hover {
        background: rgba(46, 142, 255, 0.05);
        border-left-color: #2e8eff;
    }

    .step-number {
        font-size: 3rem;
        font-weight: 900;
        color: rgba(255, 255, 255, 0.1);
        line-height: 1;
    }

    .step-title {
        font-size: 1.5rem;
        font-weight: 700;
        color: white;
        margin-bottom: 0.5rem;
    }

    .step-text {
        color: #94a3b8;
    }

    :global(.step-icon-bg) {
        position: absolute;
        right: -10px;
        bottom: -10px;
        opacity: 0.05;
        transform: rotate(-15deg);
    }

    /* Manifesto */
    .manifesto-section {
        padding: 6rem 0;
    }

    .manifesto-wrapper {
        background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
        border-radius: 24px;
        padding: 4rem;
        border: 1px solid #334155;
    }

    .manifesto-list {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1.5rem;
        margin-top: 2rem;
    }

    .manifesto-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        font-size: 1.1rem;
        color: #e2e8f0;
    }

    :global(.check-icon) { color: #10b981; flex-shrink: 0; }

    /* CTA */
    .cta-section {
        padding: 8rem 0 6rem;
    }

    .cta-content {
        text-align: center;
        max-width: 800px;
        margin: 0 auto;
        padding: 4rem 2rem;
        border: 1px solid rgba(46, 142, 255, 0.3);
        background: radial-gradient(circle, rgba(46, 142, 255, 0.1) 0%, transparent 70%);
        border-radius: 32px;
        position: relative;
    }

    :global(.cta-icon-float) {
        color: #2e8eff;
        margin-bottom: 1.5rem;
        animation: float 3s ease-in-out infinite;
    }

    .cta-title {
        font-size: 3rem;
        font-weight: 900;
        color: white;
        margin-bottom: 1.5rem;
    }

    .cta-subtitle {
        font-size: 1.25rem;
        color: #94a3b8;
        margin-bottom: 3rem;
    }

    .cta-buttons {
        display: flex;
        justify-content: center;
        gap: 1.5rem;
    }

    .cta-primary, .cta-secondary {
        padding: 1rem 2rem;
        border-radius: 12px;
        font-weight: 600;
        text-decoration: none;
        transition: all 0.2s ease;
    }

    .cta-primary {
        background: #2e8eff;
        color: white;
    }

    .cta-primary:hover {
        background: #2563eb;
        transform: translateY(-2px);
    }

    .cta-secondary {
        background: transparent;
        border: 1px solid #475569;
        color: white;
    }

    .cta-secondary:hover {
        border-color: white;
    }

    /* Animations */
    @keyframes fadeInUp {
        to { opacity: 1; transform: translateY(0); }
    }

    @keyframes float {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-10px); }
    }

    .animate-on-scroll {
        opacity: 0;
        transform: translateY(30px);
        transition: opacity 0.8s ease, transform 0.8s ease;
    }

    :global(.animate-in) {
        opacity: 1 !important;
        transform: translateY(0) !important;
    }

    /* Mobile */
    @media (max-width: 768px) {
        .split-layout, .pillars-grid, .manifesto-list, .visual-card {
            grid-template-columns: 1fr;
        }
        
        .hero-title { font-size: 3rem; }
        .hero-section { padding-top: 8rem; }
        .manifesto-wrapper { padding: 2rem; }
        .cta-buttons { flex-direction: column; }
    }
</style>