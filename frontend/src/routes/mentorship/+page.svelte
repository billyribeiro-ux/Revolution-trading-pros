<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { slide } from 'svelte/transition';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// --- ICONS (Inline for Zero-Dependency Safety) ---
	const Icons = {
		Lock: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>`,
		Globe: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>`,
		Shield: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"/></svg>`,
		ChevronDown: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>`,
		Activity: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>`,
		Terminal: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 17 10 11 4 5"/><line x1="12" x2="20" y1="19" y2="19"/></svg>`,
		Brain: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.5 2A2.5 2.5 0 0 1 12 4.5v15a2.5 2.5 0 0 1-4.96.44 2.5 2.5 0 0 1-2.96-3.08 3 3 0 0 1-.34-5.58 2.5 2.5 0 0 1 1.32-4.24 2.5 2.5 0 0 1 1.98-3A2.5 2.5 0 0 1 9.5 2Z"/><path d="M14.5 2A2.5 2.5 0 0 0 12 4.5v15a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 14.5 2Z"/></svg>`,
		Check: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`,
		FileText: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" x2="8" y1="13" y2="13"/><line x1="16" x2="8" y1="17" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>`
	};

	// --- STATE ---
	let openAccordion: number | null = 0;
	const toggleAccordion = (idx: number) => (openAccordion = openAccordion === idx ? null : idx);

	// --- DOM REFS FOR GSAP ---
	let heroContainer: HTMLElement | undefined;
	let heroBadge: HTMLElement | undefined;
	let heroTitle: HTMLElement | undefined;
	let heroDesc: HTMLElement | undefined;
	let heroMetrics: HTMLElement | undefined;
	let heroGraphic: HTMLElement | undefined;

	// --- MOTION ENGINE ---
	onMount(async () => {
		if (!browser) return;
		
		// Dynamic GSAP import for SSR safety
		const { gsap } = await import('gsap');
		
		// 1. Hero Sequence (Timeline)
		const tl = gsap.timeline({ defaults: { ease: 'power4.out' } });

		if (heroBadge) tl.fromTo(heroBadge, { y: 20, opacity: 0 }, { y: 0, opacity: 1, duration: 1, delay: 0.2 });
		if (heroTitle) tl.fromTo(heroTitle, { y: 40, opacity: 0 }, { y: 0, opacity: 1, duration: 1.2, stagger: 0.1 }, '-=0.8');
		if (heroDesc) tl.fromTo(heroDesc, { y: 20, opacity: 0 }, { y: 0, opacity: 1, duration: 1 }, '-=0.8');
		if (heroMetrics) tl.fromTo(heroMetrics, { opacity: 0, scale: 0.98 }, { opacity: 1, scale: 1, duration: 1.2 }, '-=0.6');
		if (heroGraphic) tl.fromTo(heroGraphic, { x: 40, opacity: 0 }, { x: 0, opacity: 1, duration: 1.5 }, '-=1.0');

		// 2. Scroll Reveal Logic
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const target = entry.target as HTMLElement;
						const children = target.querySelectorAll('.gsap-reveal-item');

						if (children.length > 0) {
							gsap.fromTo(children, { y: 30, opacity: 0 }, { y: 0, opacity: 1, duration: 0.8, stagger: 0.1, ease: 'power2.out', overwrite: true });
						} else {
							gsap.fromTo(target, { y: 30, opacity: 0 }, { y: 0, opacity: 1, duration: 0.8, ease: 'power2.out', overwrite: true });
						}
						observer.unobserve(target);
					}
				});
			},
			{ threshold: 0.15 }
		);

		document.querySelectorAll('.gsap-section').forEach((el) => observer.observe(el));
	});

	// --- DATA ---
	const sessionBreakdown = [
		{
			time: '00:00 - 00:30',
			phase: 'Phase I: Microstructure Audit',
			desc: "Forensic analysis of your trade logs (last 1,000 executions). We measure your slippage against ADV, analyze your fill quality across venues, and identify 'Alpha Decay' where execution drag is eroding edge."
		},
		{
			time: '00:30 - 01:00',
			phase: 'Phase II: Risk Parameterization',
			desc: 'Stress-testing your current risk model against 6-sigma events. We reconstruct your position sizing logic using Kelly Criterion modified for fat-tail distribution to optimize Geometric Mean Return.'
		},
		{
			time: '01:00 - 01:45',
			phase: 'Phase III: Edge Calibration',
			desc: "Strategic realignment. We overlay your discretionary edge with institutional data sets (Dark Pool prints, GEX levels, Vanna Flows) to create a 'Confluence Filter' that filters out B-grade setups."
		},
		{
			time: '01:45 - 02:00',
			phase: 'Phase IV: Neural Mapping',
			desc: "Addressing the 'Psychological Ceiling.' We identify the specific cognitive biases (Loss Aversion, Recency Bias) preventing you from scaling size and implement a 'Circuit Breaker' protocol."
		}
	];
</script>

<SEOHead
	title="Institutional Strategy Audit | $25,000 Consultation"
	description="A high-velocity, forensic deconstruction of your trading business. Designed strictly for Portfolio Managers and Proprietary Traders deploying 7-8 figure capital."
	canonical="/mentorship"
	ogType="website"
	noindex={true}
	nofollow={true}
	keywords={[
		'institutional trading audit',
		'trading consultation',
		'portfolio manager consulting',
		'proprietary trading',
		'trading mentorship',
		'high-net-worth trading'
	]}
/>

<div
	class="min-h-screen bg-[#020202] text-slate-400 font-sans selection:bg-white selection:text-black overflow-x-hidden"
>
	<div
		class="fixed inset-0 z-0 pointer-events-none"
		style="background-image: linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px); background-size: 60px 60px;"
	></div>

	<main class="relative z-10">
		<nav class="w-full border-b border-white/10 bg-[#020202]/80 backdrop-blur-sm fixed top-0 z-50">
			<div class="max-w-[1600px] mx-auto px-6 h-16 flex justify-between items-center">
				<div
					class="text-xs font-mono uppercase tracking-[0.2em] text-white hover:text-amber-500 transition-colors cursor-default"
				>
					Revolution <span class="text-slate-600">//</span> Institutional
				</div>
				<div class="flex items-center gap-6">
					<div class="flex items-center gap-2">
						<div class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></div>
						<span class="text-[10px] font-mono text-slate-400 uppercase tracking-widest"
							>Secure Uplink</span
						>
					</div>
					<div class="h-4 w-px bg-white/10"></div>
					<span class="text-[10px] font-mono text-slate-500 uppercase tracking-widest"
						>Client ID: Guest</span
					>
				</div>
			</div>
		</nav>

		<section class="pt-48 pb-32 px-6 border-b border-white/10" bind:this={heroContainer}>
			<div class="max-w-[1600px] mx-auto grid lg:grid-cols-12 gap-16">
				<div class="lg:col-span-8">
					<div
						bind:this={heroBadge}
						class="inline-flex items-center gap-3 px-3 py-1 border border-amber-900/30 bg-amber-900/10 text-amber-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-12"
					>
						<span class="w-3 h-3">{@html Icons.Lock}</span>
						Restricted Access
					</div>

					<h1
						bind:this={heroTitle}
						class="text-6xl md:text-8xl lg:text-9xl font-serif text-white mb-12 tracking-tight leading-[0.9] origin-left"
					>
						Strategic<br />
						<span class="text-slate-700">Alpha</span> Audit.
					</h1>

					<div bind:this={heroDesc} class="max-w-2xl border-l-2 border-amber-700 pl-8 py-2">
						<p class="text-xl md:text-2xl text-slate-300 font-light leading-relaxed">
							A high-velocity, forensic deconstruction of your trading business. Designed strictly
							for <span class="text-white font-medium">Portfolio Managers</span> and
							<span class="text-white font-medium">Proprietary Traders</span> deploying 7-8 figure capital.
						</p>
					</div>

					<div bind:this={heroMetrics} class="mt-16 flex flex-wrap gap-12">
						<div>
							<div class="text-[10px] font-mono uppercase tracking-widest text-slate-600 mb-2">
								Consultation Fee
							</div>
							<div class="text-3xl font-serif text-white">
								$25,000 <span class="text-sm text-slate-600 font-sans align-middle">USD</span>
							</div>
						</div>
						<div>
							<div class="text-[10px] font-mono uppercase tracking-widest text-slate-600 mb-2">
								Duration
							</div>
							<div class="text-3xl font-serif text-white">
								120 <span class="text-sm text-slate-600 font-sans align-middle">MINUTES</span>
							</div>
						</div>
						<div>
							<div class="text-[10px] font-mono uppercase tracking-widest text-slate-600 mb-2">
								Availability
							</div>
							<div class="text-3xl font-serif text-white">
								Q4: <span class="text-amber-500">2 SLOTS</span>
							</div>
						</div>
					</div>
				</div>

				<div bind:this={heroGraphic} class="lg:col-span-4 flex flex-col justify-end">
					<div class="bg-[#080808] border border-white/10 p-8 relative overflow-hidden">
						<div class="absolute top-0 right-0 p-4 opacity-20 w-12 h-12 text-slate-500">
							{@html Icons.Globe}
						</div>
						<h3
							class="font-mono text-xs uppercase tracking-widest text-white mb-6 border-b border-white/10 pb-4"
						>
							Scope of Engagement
						</h3>
						<ul class="space-y-4 text-sm font-mono text-slate-400">
							<li class="flex justify-between items-center">
								<span>> Execution Audit</span>
								<div class="flex items-center gap-2">
									<div class="w-1.5 h-1.5 bg-emerald-500 rounded-full"></div>
									<span class="text-emerald-500 text-[10px] uppercase">Active</span>
								</div>
							</li>
							<li class="flex justify-between items-center">
								<span>> Risk Parameterization</span>
								<div class="flex items-center gap-2">
									<div class="w-1.5 h-1.5 bg-emerald-500 rounded-full"></div>
									<span class="text-emerald-500 text-[10px] uppercase">Active</span>
								</div>
							</li>
							<li class="flex justify-between items-center">
								<span>> Psychological Mapping</span>
								<div class="flex items-center gap-2">
									<div class="w-1.5 h-1.5 bg-emerald-500 rounded-full"></div>
									<span class="text-emerald-500 text-[10px] uppercase">Active</span>
								</div>
							</li>
							<li class="flex justify-between items-center">
								<span>> Infrastructure Review</span>
								<div class="flex items-center gap-2">
									<div class="w-1.5 h-1.5 bg-emerald-500 rounded-full"></div>
									<span class="text-emerald-500 text-[10px] uppercase">Active</span>
								</div>
							</li>
						</ul>
						<div class="mt-8 pt-6 border-t border-white/10">
							<p class="text-xs text-slate-600 leading-relaxed">
								<span class="text-amber-600 font-bold">WARNING:</span> This service is not educational.
								It is advisory. We assume the client possesses sophisticated knowledge of derivatives,
								margin mechanics, and market microstructure.
							</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-32 px-6 bg-[#050505] border-b border-white/5 gsap-section">
			<div class="max-w-[1600px] mx-auto">
				<div class="grid lg:grid-cols-2 gap-24">
					<div class="gsap-reveal-item">
						<h2 class="text-4xl font-serif text-white mb-8">The Liquidity Ceiling.</h2>
						<div class="space-y-6 text-lg font-light leading-relaxed text-slate-400">
							<p>
								Scaling a portfolio from $100,000 to $1,000,000 is a math problem. Scaling from
								$10,000,000 to $100,000,000 is a liquidity problem.
							</p>
							<p>
								At institutional size, your entry <em>is</em> the market. Alpha decays the moment you
								execute. Without optimizing your participation rate, dark pool routing, and variance
								drag, you are simply paying a "Size Tax" to HFT firms.
							</p>
							<p class="text-white border-l-2 border-white/20 pl-6 py-2">
								Revolution Trading Pros intervenes at this inflection point. We do not teach you <em
									>how</em
								> to trade. We engineer your business to scale without breaking.
							</p>
						</div>
					</div>

					<div class="grid grid-cols-2 gap-px bg-white/10 border border-white/10">
						<div class="bg-[#080808] p-10 flex flex-col justify-between h-64 gsap-reveal-item">
							<div class="text-amber-600 w-8 h-8">{@html Icons.Activity}</div>
							<div>
								<div class="text-3xl font-serif text-white mb-2">
									-18<span class="text-lg">%</span>
								</div>
								<div class="text-[10px] font-mono uppercase tracking-widest text-slate-500">
									Avg. Execution Drag
								</div>
							</div>
						</div>
						<div class="bg-[#080808] p-10 flex flex-col justify-between h-64 gsap-reveal-item">
							<div class="text-amber-600 w-8 h-8">{@html Icons.Brain}</div>
							<div>
								<div class="text-3xl font-serif text-white mb-2">Bias</div>
								<div class="text-[10px] font-mono uppercase tracking-widest text-slate-500">
									Primary Bottleneck
								</div>
							</div>
						</div>
						<div class="bg-[#080808] p-10 flex flex-col justify-between h-64 gsap-reveal-item">
							<div class="text-amber-600 w-8 h-8">{@html Icons.Terminal}</div>
							<div>
								<div class="text-3xl font-serif text-white mb-2">Zero</div>
								<div class="text-[10px] font-mono uppercase tracking-widest text-slate-500">
									Latency Tolerance
								</div>
							</div>
						</div>
						<div class="bg-[#080808] p-10 flex flex-col justify-between h-64 gsap-reveal-item">
							<div class="text-amber-600 w-8 h-8">{@html Icons.Shield}</div>
							<div>
								<div class="text-3xl font-serif text-white mb-2">MNDA</div>
								<div class="text-[10px] font-mono uppercase tracking-widest text-slate-500">
									Legal Protection
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-32 px-6 border-b border-white/5 gsap-section">
			<div class="max-w-5xl mx-auto">
				<div
					class="flex items-end justify-between mb-20 border-b border-white/10 pb-8 gsap-reveal-item"
				>
					<div>
						<h2 class="text-4xl font-serif text-white mb-2">The 120-Minute Protocol</h2>
						<p class="text-sm font-mono text-slate-500 uppercase tracking-widest">
							Session Agenda // Confidential
						</p>
					</div>
					<div class="hidden md:block text-right">
						<div class="text-amber-600 font-mono text-sm">NYC / LON / SIN</div>
						<div class="text-xs text-slate-600 font-mono uppercase">Global Availability</div>
					</div>
				</div>

				<div class="space-y-8">
					{#each sessionBreakdown as item}
						<div class="group gsap-reveal-item">
							<div
								class="flex flex-col md:flex-row gap-8 md:gap-16 p-8 hover:bg-white/[0.02] transition-colors border-l-2 border-white/10 hover:border-amber-600"
							>
								<div class="w-32 shrink-0 pt-1">
									<span class="font-mono text-amber-600 text-sm">{item.time}</span>
								</div>
								<div>
									<h3 class="text-xl font-serif text-white mb-3">{item.phase}</h3>
									<p class="text-slate-400 font-light leading-relaxed text-sm">{item.desc}</p>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="py-32 px-6 bg-[#080808] border-b border-white/5 gsap-section">
			<div class="max-w-[1600px] mx-auto grid lg:grid-cols-2 gap-24">
				<div class="gsap-reveal-item">
					<div class="flex items-center gap-4 mb-10">
						<div class="w-10 h-10 flex items-center justify-center rounded bg-white/5 text-white">
							{@html Icons.FileText}
						</div>
						<h3 class="text-2xl font-serif text-white">Technical Prerequisites</h3>
					</div>
					<div class="bg-[#020202] border border-white/10 p-10">
						<p class="text-sm text-slate-500 mb-8 leading-relaxed">
							To ensure maximum utility of the 120-minute window, we require the following data
							points to be uploaded to our encrypted portal 48 hours prior to the session.
						</p>
						<ul class="space-y-4">
							<li class="flex items-start gap-3 text-sm font-mono text-slate-300">
								<span class="text-amber-600 shrink-0">>></span>
								Past 12 Months of Brokerage Statements (Redacted PII)
							</li>
							<li class="flex items-start gap-3 text-sm font-mono text-slate-300">
								<span class="text-amber-600 shrink-0">>></span>
								Current Risk Management Policy Document
							</li>
							<li class="flex items-start gap-3 text-sm font-mono text-slate-300">
								<span class="text-amber-600 shrink-0">>></span>
								List of Execution Venues / Prime Brokerage Relationships
							</li>
							<li class="flex items-start gap-3 text-sm font-mono text-slate-300">
								<span class="text-amber-600 shrink-0">>></span>
								Signed Mutual Non-Disclosure Agreement (MNDA)
							</li>
						</ul>
					</div>
				</div>

				<div class="gsap-reveal-item">
					<div class="flex items-center gap-4 mb-10">
						<div class="w-10 h-10 flex items-center justify-center rounded bg-white/5 text-white">
							{@html Icons.Shield}
						</div>
						<h3 class="text-2xl font-serif text-white">Engagement Protocols</h3>
					</div>

					<div class="space-y-2">
						<div class="border border-white/10 bg-[#020202]">
							<button
								class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
								onclick={() => toggleAccordion(0)}
							>
								<span class="font-bold text-white text-sm tracking-wide"
									>CONFIDENTIALITY & DATA SOVEREIGNTY</span
								>
								<span
									class="w-4 h-4 text-slate-500 transform transition-transform {openAccordion === 0
										? 'rotate-180'
										: ''}">{@html Icons.ChevronDown}</span
								>
							</button>
							{#if openAccordion === 0}
								<div
									transition:slide
									class="px-6 pb-6 text-slate-400 text-sm font-light leading-relaxed border-t border-white/5 pt-4"
								>
									Revolution Trading Pros operates under a strict code of silence. We execute a
									Mutual Non-Disclosure Agreement (MNDA) before receiving any trade logs. We do not
									store client data on cloud servers; all analysis is performed on air-gapped local
									machines and wiped post-session.
								</div>
							{/if}
						</div>

						<div class="border border-white/10 bg-[#020202]">
							<button
								class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
								onclick={() => toggleAccordion(1)}
							>
								<span class="font-bold text-white text-sm tracking-wide">CONFLICT OF INTEREST</span>
								<span
									class="w-4 h-4 text-slate-500 transform transition-transform {openAccordion === 1
										? 'rotate-180'
										: ''}">{@html Icons.ChevronDown}</span
								>
							</button>
							{#if openAccordion === 1}
								<div
									transition:slide
									class="px-6 pb-6 text-slate-400 text-sm font-light leading-relaxed border-t border-white/5 pt-4"
								>
									We maintain a rigorous "Chinese Wall." We do not front-run client order flow, nor
									do we take opposing positions in instruments discussed during the consultation.
									Our role is strictly advisory; we do not take custody of funds or execute trades
									on your behalf.
								</div>
							{/if}
						</div>

						<div class="border border-white/10 bg-[#020202]">
							<button
								class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
								onclick={() => toggleAccordion(2)}
							>
								<span class="font-bold text-white text-sm tracking-wide">LOGISTICS & EXPENSING</span
								>
								<span
									class="w-4 h-4 text-slate-500 transform transition-transform {openAccordion === 2
										? 'rotate-180'
										: ''}">{@html Icons.ChevronDown}</span
								>
							</button>
							{#if openAccordion === 2}
								<div
									transition:slide
									class="px-6 pb-6 text-slate-400 text-sm font-light leading-relaxed border-t border-white/5 pt-4"
								>
									Sessions are conducted via encrypted video link (Zoom Enterprise/Teams) or
									in-person in NYC (subject to travel retainer). We provide itemized corporate
									invoicing suitable for fund administration expenses under "Professional
									Consultation."
								</div>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-40 px-6 relative overflow-hidden bg-[#020202] gsap-section">
			<div class="absolute top-0 left-1/2 -translate-x-1/2 w-px h-full bg-white/5"></div>
			<div class="absolute top-1/2 left-0 w-full h-px bg-white/5"></div>

			<div
				class="max-w-4xl mx-auto relative z-10 bg-[#020202] border border-white/10 p-1 gsap-reveal-item"
			>
				<div class="border border-white/5 p-12 md:p-20 text-center relative overflow-hidden group">
					<div
						class="absolute inset-0 bg-amber-900/10 opacity-0 group-hover:opacity-100 transition-opacity duration-1000 pointer-events-none"
					></div>

					<div class="mb-10">
						<div class="text-amber-600 mx-auto w-12 h-12 mb-6">{@html Icons.Check}</div>
						<h2 class="text-4xl md:text-5xl font-serif text-white mb-4">Initiate Application</h2>
						<p class="text-slate-500 font-mono text-xs uppercase tracking-[0.2em]">
							Reference: Q4-INST-AUDIT
						</p>
					</div>

					<div class="grid grid-cols-2 gap-8 max-w-lg mx-auto mb-12 text-left">
						<div class="border-l border-amber-800/50 pl-4">
							<div class="text-[10px] font-mono uppercase text-slate-500 mb-1">Wire Amount</div>
							<div class="text-xl text-white font-serif">$25,000.00</div>
						</div>
						<div class="border-l border-amber-800/50 pl-4">
							<div class="text-[10px] font-mono uppercase text-slate-500 mb-1">Payment Terms</div>
							<div class="text-xl text-white font-serif">Net 0</div>
						</div>
					</div>

					<a
						href="mailto:institutional@revolutiontradingpros.com?subject=Institutional%20Audit%20Application"
						class="inline-flex items-center justify-center w-full md:w-auto px-12 py-5 bg-white text-black font-bold text-sm uppercase tracking-[0.25em] hover:bg-amber-500 transition-all duration-300"
					>
						Submit Request
					</a>

					<p class="mt-8 text-[10px] text-slate-600 font-mono uppercase">
						Due to high demand, applications are reviewed weekly. <br class="hidden md:block" /> We reserve
						the right to decline engagements based on fit.
					</p>
				</div>
			</div>
		</section>
	</main>
</div>
