<script lang="ts">
    import { onMount } from 'svelte';
    import { gsap } from 'gsap';
    import IconDiamond from '@tabler/icons-svelte/icons/diamond';
    import IconLock from '@tabler/icons-svelte/icons/lock';
    import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
    import IconBuilding from '@tabler/icons-svelte/icons/building';
    import IconBrain from '@tabler/icons-svelte/icons/brain';
    import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
    import IconBriefcase from '@tabler/icons-svelte/icons/briefcase';
    import IconShieldCheck from '@tabler/icons-svelte/icons/shield-check';
    import IconCertificate from '@tabler/icons-svelte/icons/certificate';
    import { addItemToCart } from '$lib/utils/cart-helpers';

    let heroRef: HTMLElement;
    let audienceRef: HTMLElement;
    let detailsRef: HTMLElement;
    let protocolsRef: HTMLElement;

    onMount(() => {
        initAnimations();
    });

    function initAnimations() {
        const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

        tl.from(heroRef.children, {
            y: 50,
            opacity: 0,
            duration: 1,
            stagger: 0.15,
            delay: 0.2
        })
        .from(audienceRef, {
            y: 40,
            opacity: 0,
            duration: 0.8
        }, '-=0.5')
        .from(detailsRef.children, {
            y: 30,
            opacity: 0,
            duration: 0.8,
            stagger: 0.1
        }, '-=0.4')
        .from(protocolsRef, {
            y: 30,
            opacity: 0,
            duration: 0.8
        }, '-=0.4');
    }

    // High-Ticket Consulting Schema + FAQ Schema
    const consultingSchema = {
        '@context': 'https://schema.org',
        '@graph': [
            {
                '@type': 'ProfessionalService',
                name: 'Institutional Trading Consultation',
                description: 'Elite 1-on-1 strategy audit for Portfolio Managers and Institutional Traders. Focus on liquidity, risk modeling, and psychological calibration.',
                priceRange: '$$$$',
                provider: {
                    '@type': 'Organization',
                    name: 'Revolution Trading Pros'
                },
                offers: {
                    '@type': 'Offer',
                    price: '25000',
                    priceCurrency: 'USD',
                    description: '2-Hour Strategic Deep Dive'
                }
            },
            {
                '@type': 'FAQPage',
                mainEntity: [
                    {
                        '@type': 'Question',
                        name: 'What is the confidentiality protocol regarding proprietary strategies?',
                        acceptedAnswer: {
                            '@type': 'Answer',
                            text: 'We execute a strict Mutual Non-Disclosure Agreement (MNDA) prior to data exchange. Revolution Trading Pros adheres to a strict data sovereignty policy; we do not retain, store, or train models on client trade logs post-audit. All session recordings are delivered via encrypted drive and wiped from our servers within 24 hours.'
                        }
                    },
                    {
                        '@type': 'Question',
                        name: 'Does this cover High Frequency (HFT) or Latency Arbitrage?',
                        acceptedAnswer: {
                            '@type': 'Answer',
                            text: 'No. Our expertise lies in discretionary macro, swing positioning, and intraday institutional order flow (gamma exposure/dark pools). We do not audit FPGA code or sub-millisecond execution infrastructure.'
                        }
                    },
                    {
                        '@type': 'Question',
                        name: 'Can this expense be categorized as professional development?',
                        acceptedAnswer: {
                            '@type': 'Answer',
                            text: 'Yes. Most clients expense this under "Professional Strategy Consultation" or "Risk Management Auditing." We provide itemized corporate invoicing suitable for fund administration.'
                        }
                    },
                    {
                        '@type': 'Question',
                        name: 'What is the conflict of interest policy?',
                        acceptedAnswer: {
                            '@type': 'Answer',
                            text: 'We operate under a rigorous Chinese Wall policy. We do not front-run client order flow nor do we trade against client positions discussed during the audit. Our role is strictly advisory.'
                        }
                    }
                ]
            }
        ]
    };
</script>

<svelte:head>
    <title>Institutional Strategy Audit | $25k Consultation | Revolution Trading Pros</title>
    <meta name="description" content="Private 2-hour strategic consultation for Portfolio Managers and Institutional Traders. Optimize execution, scale risk, and calibrate psychology." />
    <link rel="canonical" href="https://revolutiontradingpros.com/mentorship" />
    <meta name="robots" content="index, follow" />
    
    <meta property="og:title" content="Institutional Strategy Audit" />
    <meta property="og:description" content="For those managing 8-figure portfolios. One session to optimize your edge." />
    <meta property="og:image" content="https://revolutiontradingpros.com/images/institutional-consult.jpg" />
    
    {@html `<script type="application/ld+json">${JSON.stringify(consultingSchema)}</script>`}
</svelte:head>

<div class="min-h-screen bg-[#050505] text-slate-200 font-sans selection:bg-amber-500/30 overflow-hidden">
    
    <div class="fixed inset-0 z-0 pointer-events-none opacity-20" 
         style="background-image: radial-gradient(#333 1px, transparent 1px); background-size: 40px 40px;">
    </div>
    
    <div class="fixed top-0 left-1/2 -translate-x-1/2 w-[800px] h-[500px] bg-amber-900/10 blur-[120px] rounded-full pointer-events-none z-0"></div>

    <main class="relative z-10">
        
        <section class="pt-32 pb-20 px-4 sm:px-6 lg:px-8 max-w-7xl mx-auto text-center" bind:this={heroRef}>
            <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-amber-500/30 bg-amber-500/5 text-amber-500 text-xs font-bold tracking-[0.2em] uppercase mb-8">
                <IconLock size={14} />
                Private Client Group
            </div>
            
            <h1 class="text-5xl md:text-7xl lg:text-8xl font-heading font-bold text-white mb-8 tracking-tight leading-none">
                Strategic <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-200 via-amber-500 to-amber-700">Alpha</span><br />
                Consultation
            </h1>
            
            <p class="text-xl md:text-2xl text-slate-400 max-w-3xl mx-auto leading-relaxed font-light">
                This is not a course. It is a <strong>120-minute strategic deep-dive</strong> designed solely for Portfolio Managers and Institutional Traders seeking to optimize execution and break psychological ceilings.
            </p>

            <div class="mt-12 flex flex-col items-center justify-center gap-4">
                <div class="text-3xl font-light text-white">
                    $25,000 <span class="text-slate-500 text-lg">/ 2 Hours</span>
                </div>
                <div class="text-xs text-slate-500 uppercase tracking-widest">
                    Application & NDA Required
                </div>
            </div>
        </section>

        <section class="py-20 border-y border-white/5 bg-white/[0.02]" bind:this={audienceRef}>
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid md:grid-cols-2 gap-12 items-center">
                    <div>
                        <h2 class="text-3xl font-heading font-bold text-white mb-6">
                            For The Few, Not The Many.
                        </h2>
                        <p class="text-lg text-slate-400 mb-6 leading-relaxed">
                            At your level, a 1% improvement in execution or risk calibration isn't just a number—it's millions in PnL. We do not teach "setups." We refine professional edges.
                        </p>
                        <p class="text-lg text-slate-400 leading-relaxed">
                            This consultation is strictly for traders managing significant capital who have hit a liquidity ceiling, a psychological plateau, or a strategic bottleneck.
                        </p>
                    </div>
                    <div class="grid grid-cols-1 gap-4">
                        <div class="flex items-center gap-4 p-4 rounded-lg bg-white/5 border border-white/10">
                            <div class="p-2 bg-slate-800 rounded text-amber-500"><IconBuilding size={24} /></div>
                            <div>
                                <h3 class="font-bold text-white">Institutional Traders</h3>
                                <p class="text-sm text-slate-400">Desk traders at hedge funds or prop firms.</p>
                            </div>
                        </div>
                        <div class="flex items-center gap-4 p-4 rounded-lg bg-white/5 border border-white/10">
                            <div class="p-2 bg-slate-800 rounded text-amber-500"><IconBriefcase size={24} /></div>
                            <div>
                                <h3 class="font-bold text-white">Portfolio Managers</h3>
                                <p class="text-sm text-slate-400">Managing $10M+ AUM looking for uncorrelated returns.</p>
                            </div>
                        </div>
                        <div class="flex items-center gap-4 p-4 rounded-lg bg-white/5 border border-white/10">
                            <div class="p-2 bg-slate-800 rounded text-amber-500"><IconDiamond size={24} /></div>
                            <div>
                                <h3 class="font-bold text-white">High-Net-Worth Individuals</h3>
                                <p class="text-sm text-slate-400">Private traders managing 8-figure personal accounts.</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="py-24 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-5xl font-heading font-bold text-white mb-4">The 120-Minute Audit</h2>
                <p class="text-slate-400">No curriculum. No videos. Just surgery on your strategy.</p>
            </div>

            <div class="grid md:grid-cols-3 gap-8" bind:this={detailsRef}>
                <div class="group relative p-8 rounded-2xl bg-[#0A0A0A] border border-white/10 hover:border-amber-500/30 transition-all duration-500">
                    <div class="absolute inset-0 bg-gradient-to-br from-amber-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-2xl"></div>
                    <IconChartLine size={40} class="text-amber-500 mb-6" stroke={1.5} />
                    <h3 class="text-xl font-bold text-white mb-4">Liquidity & Execution</h3>
                    <p class="text-slate-400 leading-relaxed">
                        We analyze your slippage, sizing relative to average daily volume (ADV), and order routing. We optimize how you enter and exit size to minimize market impact and drag.
                    </p>
                </div>

                <div class="group relative p-8 rounded-2xl bg-[#0A0A0A] border border-white/10 hover:border-amber-500/30 transition-all duration-500">
                    <div class="absolute inset-0 bg-gradient-to-br from-amber-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-2xl"></div>
                    <IconBrain size={40} class="text-amber-500 mb-6" stroke={1.5} />
                    <h3 class="text-xl font-bold text-white mb-4">High-Stakes Psychology</h3>
                    <p class="text-slate-400 leading-relaxed">
                        Trading with $10,000 is math. Trading with $10,000,000 is psychology. We identify the subconscious blockers preventing you from scaling your size to the next deviation.
                    </p>
                </div>

                <div class="group relative p-8 rounded-2xl bg-[#0A0A0A] border border-white/10 hover:border-amber-500/30 transition-all duration-500">
                    <div class="absolute inset-0 bg-gradient-to-br from-amber-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-2xl"></div>
                    <IconLock size={40} class="text-amber-500 mb-6" stroke={1.5} />
                    <h3 class="text-xl font-bold text-white mb-4">Risk Modeling</h3>
                    <p class="text-slate-400 leading-relaxed">
                        A deep review of your Sharpe ratio, drawdowns, and tail-risk exposure. We reconstruct your risk parameters to ensure survival during Black Swan events while maximizing upside variance.
                    </p>
                </div>
            </div>
        </section>

        <section class="py-20 border-t border-white/5 bg-[#080808]" bind:this={protocolsRef}>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center gap-3 mb-12 justify-center">
                    <IconShieldCheck size={32} class="text-slate-500" />
                    <h2 class="text-2xl md:text-3xl font-heading font-bold text-white tracking-wide">Engagement Protocols</h2>
                </div>

                <div class="space-y-4">
                    <details class="group bg-[#0A0A0A] border border-white/5 rounded-lg overflow-hidden">
                        <summary class="flex justify-between items-center p-6 cursor-pointer list-none text-slate-200 hover:text-white transition-colors">
                            <span class="font-bold flex items-center gap-3">
                                <IconCertificate size={20} class="text-amber-600" />
                                Confidentiality & NDAs
                            </span>
                            <span class="transition-transform group-open:rotate-180 text-amber-500">▼</span>
                        </summary>
                        <div class="px-6 pb-8 text-slate-400 leading-relaxed border-t border-white/5 pt-4">
                            We execute a strict <strong>Mutual Non-Disclosure Agreement (MNDA)</strong> prior to any data exchange. Revolution Trading Pros adheres to a strict data sovereignty policy; we do not retain, store, or train models on client trade logs post-audit. All session recordings are delivered via encrypted drive and wiped from our servers within 24 hours.
                        </div>
                    </details>

                    <details class="group bg-[#0A0A0A] border border-white/5 rounded-lg overflow-hidden">
                        <summary class="flex justify-between items-center p-6 cursor-pointer list-none text-slate-200 hover:text-white transition-colors">
                            <span class="font-bold flex items-center gap-3">
                                <IconBuilding size={20} class="text-amber-600" />
                                Conflict of Interest Mitigation
                            </span>
                            <span class="transition-transform group-open:rotate-180 text-amber-500">▼</span>
                        </summary>
                        <div class="px-6 pb-8 text-slate-400 leading-relaxed border-t border-white/5 pt-4">
                            We operate under a rigorous "Chinese Wall" policy. We do not front-run client order flow, nor do we trade against client positions discussed during the audit. Our role is strictly advisory. We do not require Power of Attorney (LPOA) or direct access to your brokerage accounts.
                        </div>
                    </details>

                    <details class="group bg-[#0A0A0A] border border-white/5 rounded-lg overflow-hidden">
                        <summary class="flex justify-between items-center p-6 cursor-pointer list-none text-slate-200 hover:text-white transition-colors">
                            <span class="font-bold flex items-center gap-3">
                                <IconChartLine size={20} class="text-amber-600" />
                                Scope of Analysis
                            </span>
                            <span class="transition-transform group-open:rotate-180 text-amber-500">▼</span>
                        </summary>
                        <div class="px-6 pb-8 text-slate-400 leading-relaxed border-t border-white/5 pt-4">
                            Our audit covers discretionary macro, swing positioning, and intraday institutional order flow (gamma exposure/dark pools). 
                            <br/><br/>
                            <span class="text-amber-500 text-xs uppercase tracking-wide font-bold">Exclusions:</span> We do not audit FPGA code, sub-millisecond HFT infrastructure, or tax/legal entity structuring.
                        </div>
                    </details>

                    <details class="group bg-[#0A0A0A] border border-white/5 rounded-lg overflow-hidden">
                        <summary class="flex justify-between items-center p-6 cursor-pointer list-none text-slate-200 hover:text-white transition-colors">
                            <span class="font-bold flex items-center gap-3">
                                <IconBriefcase size={20} class="text-amber-600" />
                                Logistics & Expensing
                            </span>
                            <span class="transition-transform group-open:rotate-180 text-amber-500">▼</span>
                        </summary>
                        <div class="px-6 pb-8 text-slate-400 leading-relaxed border-t border-white/5 pt-4">
                            Sessions can be conducted via encrypted video link or in-person (NYC/London, subject to travel retainers). Most clients expense this engagement under <em>"Professional Strategy Consultation"</em> or <em>"Risk Management Auditing."</em> We provide itemized corporate invoicing suitable for fund administration.
                        </div>
                    </details>
                </div>
            </div>
        </section>

        <section class="py-24 relative overflow-hidden">
            <div class="absolute inset-0 bg-gradient-to-t from-amber-900/10 to-transparent pointer-events-none"></div>
            
            <div class="max-w-4xl mx-auto px-4 relative z-10 text-center">
                <div class="inline-block p-4 rounded-full bg-amber-500/10 mb-8">
                    <IconDiamond size={32} class="text-amber-500" />
                </div>
                
                <h2 class="text-4xl md:text-5xl font-heading font-bold text-white mb-8">
                    Is Your Edge Worth Optimization?
                </h2>
                
                <p class="text-xl text-slate-300 mb-12 max-w-2xl mx-auto">
                    We only accept 4 clients per quarter. Full NDA protection is standard protocol.
                </p>

                <div class="bg-[#0f0f0f] border border-white/10 p-8 md:p-12 rounded-2xl max-w-2xl mx-auto shadow-2xl">
                    <div class="flex flex-col gap-6">
                        <div class="flex justify-between items-center border-b border-white/5 pb-4">
                            <span class="text-slate-400">Duration</span>
                            <span class="text-white font-bold">120 Minutes</span>
                        </div>
                        <div class="flex justify-between items-center border-b border-white/5 pb-4">
                            <span class="text-slate-400">Format</span>
                            <span class="text-white font-bold">Encrypted Video / In-Person (NYC)</span>
                        </div>
                        <div class="flex justify-between items-center border-b border-white/5 pb-4">
                            <span class="text-slate-400">Investment</span>
                            <span class="text-amber-500 font-bold text-xl">$25,000 USD</span>
                        </div>
                        
                        <a href="mailto:institutional@revolutiontradingpros.com?subject=Application%20for%20Strategy%20Audit" 
                           class="group mt-6 w-full py-4 px-6 bg-white text-black hover:bg-amber-400 font-bold text-lg rounded-none uppercase tracking-widest transition-all flex items-center justify-center gap-3">
                           Request Access
                           <IconArrowRight size={20} class="group-hover:translate-x-1 transition-transform" />
                        </a>
                        <p class="text-xs text-slate-600 mt-2">
                            *Payment required 48 hours prior to session via Wire Transfer.
                        </p>
                    </div>
                </div>
            </div>
        </section>

    </main>
</div>