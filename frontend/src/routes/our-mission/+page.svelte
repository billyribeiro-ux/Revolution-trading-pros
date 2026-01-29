<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// --- GSAP ScrollTrigger Animations (Svelte 5 SSR-safe pattern) ---
	onMount(() => {
		if (!browser) return;

		let ctx: ReturnType<typeof import('gsap').gsap.context> | null = null;

		(async () => {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');
			gsap.registerPlugin(ScrollTrigger);

			const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

			if (prefersReducedMotion) {
				gsap.set('[data-gsap]', { opacity: 1, y: 0 });
				return;
			}

			// Use gsap.context() for scoped cleanup - prevents global ScrollTrigger destruction
			ctx = gsap.context(() => {
				// Only set initial hidden state for elements NOT yet in viewport
				const elements = document.querySelectorAll('[data-gsap]');
				elements.forEach((el) => {
					const rect = el.getBoundingClientRect();
					const isInViewport = rect.top < window.innerHeight * 0.85;
					if (!isInViewport) {
						gsap.set(el, { opacity: 0, y: 30 });
					}
				});

				ScrollTrigger.batch('[data-gsap]', {
					onEnter: (batch) => {
						gsap.to(batch, {
							opacity: 1,
							y: 0,
							duration: 0.8,
							ease: 'power3.out',
							stagger: 0.1,
							overwrite: true
						});
					},
					start: 'top 85%',
					once: true
				});

				ScrollTrigger.refresh();
			});
		})();

		return () => ctx?.revert();
	});

	// --- ICONS (Inline SVG for Zero-Dependency & Performance) ---
	const Icons = {
		Target: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/></svg>`,
		TrendingUp: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/><polyline points="16 7 22 7 22 13"/></svg>`,
		Shield: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>`,
		Users: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>`,
		Scale: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"/><path d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z"/><path d="M7 21h10"/><path d="M12 3v18"/><path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2"/></svg>`,
		Brain: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9.5 2A2.5 2.5 0 0 1 12 4.5v15a2.5 2.5 0 0 1-4.96.44 2.5 2.5 0 0 1-2.96-3.08 3 3 0 0 1-.34-5.58 2.5 2.5 0 0 1 1.32-4.24 2.5 2.5 0 0 1 1.98-3A2.5 2.5 0 0 1 9.5 2Z"/><path d="M14.5 2A2.5 2.5 0 0 0 12 4.5v15a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 14.5 2Z"/></svg>`,
		ChartBar: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18"/><path d="M18 17V9"/><path d="M13 17V5"/><path d="M8 17v-3"/></svg>`,
		ArrowRight: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>`,
		Check: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`,
		X: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>`,
		Quote: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="currentColor" stroke="none"><path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"/><path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"/></svg>`,
		ChevronDown: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>`,
		Server: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="8" x="2" y="2" rx="2" ry="2"/><rect width="20" height="8" x="2" y="14" rx="2" ry="2"/><line x1="6" x2="6.01" y1="6" y2="6"/><line x1="6" x2="6.01" y1="18" y2="18"/></svg>`,
		Activity: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>`,
		Lock: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>`,
		Terminal: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 17 10 11 4 5"/><line x1="12" x2="20" y1="19" y2="19"/></svg>`,
		Book: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg>`,
		Search: `<svg width="100%" height="100%" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>`
	};

	// --- INTERACTIVE RISK CALCULATOR LOGIC ---
	// A comprehensive simulator showing the power of compounding vs ruin
	// Svelte 5 state runes
	let simAccount = $state(25000);
	let simRisk = $state(2);
	let simWinRate = $state(55);
	let simRR = $state(2);

	// Svelte 5 derived runes
	let riskAmount = $derived(Math.round(simAccount * (simRisk / 100)));
	let winAmount = $derived(Math.round(riskAmount * simRR));
	let expectedValue = $derived(
		(simWinRate / 100) * winAmount - (1 - simWinRate / 100) * riskAmount
	);
	let tradesToDouble = $derived(Math.ceil(simAccount / expectedValue));
	let riskOfRuin = $derived(
		simRisk > 5
			? 'HIGH (Casino Mode)'
			: simRisk > 2
				? 'MODERATE (Aggressive)'
				: 'LOW (Institutional)'
	);
	let riskColor = $derived(
		simRisk > 5 ? 'text-red-500' : simRisk > 2 ? 'text-orange-400' : 'text-emerald-400'
	);

	// --- STATES (Svelte 5 runes) ---
	let openFaq = $state<number | null>(null);
	const toggleFaq = (idx: number) => (openFaq = openFaq === idx ? null : idx);

	let openSyllabus = $state<number | null>(0);
	const toggleSyllabus = (idx: number) => (openSyllabus = openSyllabus === idx ? null : idx);

	let glossarySearch = $state('');

	// --- SEO SCHEMA (JSON-LD) ---
	// Comprehensive Schema for Institutional Authority
	const jsonLd = {
		'@context': 'https://schema.org',
		'@graph': [
			{
				'@type': 'Organization',
				name: 'Revolution Trading Pros',
				url: 'https://revolution-trading-pros.pages.dev',
				logo: 'https://revolution-trading-pros.pages.dev/logo.png',
				foundingDate: '2023',
				founder: { '@type': 'Person', name: 'Billy Ribeiro' },
				sameAs: ['https://twitter.com/RevTradingPros'],
				contactPoint: {
					'@type': 'ContactPoint',
					telephone: '+1-555-0123',
					contactType: 'customer service'
				}
			},
			{
				'@type': 'Course',
				name: 'Institutional Trading Mastery',
				description:
					'A comprehensive curriculum covering Auction Market Theory, Volume Profiling, and Institutional Risk Management.',
				provider: { '@type': 'Organization', name: 'Revolution Trading Pros' },
				hasCourseInstance: {
					'@type': 'CourseInstance',
					courseMode: 'online',
					courseWorkload: 'P12W'
				}
			},
			{
				'@type': 'Article',
				headline: 'The Retail Trap: Why 90% of Traders Fail',
				author: { '@type': 'Organization', name: 'Revolution Trading Pros' },
				publisher: { '@type': 'Organization', name: 'Revolution Trading Pros' },
				datePublished: '2024-01-01',
				image: 'https://revolution-trading-pros.pages.dev/images/retail-trap.jpg'
			},
			{
				'@type': 'FAQPage',
				mainEntity: [
					{
						'@type': 'Question',
						name: "What is the 'Retail Trap' in trading?",
						acceptedAnswer: {
							'@type': 'Answer',
							text: 'The Retail Trap refers to the statistical probability of failure for non-professional traders (often cited as the 90/90/90 rule). This failure is driven by a lack of risk management, emotional trading, and reliance on lagging indicators.'
						}
					},
					{
						'@type': 'Question',
						name: 'How is Revolution Trading Pros different from other groups?',
						acceptedAnswer: {
							'@type': 'Answer',
							text: "We focus on 'Auction Market Theory' and 'Volume Profiling' rather than subjective chart patterns. We treat trading as a business of probability (Expectancy) rather than a game of prediction."
						}
					}
				]
			}
		]
	};

	// --- CONTENT DATA ---
	const pillars = [
		{
			icon: Icons.Shield,
			title: 'Radical Transparency',
			desc: 'We operate in a glass house. We publish raw track records, including losses and commissions. In an industry of smoke and mirrors, truth is the only currency.',
			color: 'text-emerald-400',
			bg: 'bg-emerald-400/10',
			border: 'border-emerald-400/20'
		},
		{
			icon: Icons.Users,
			title: 'Collective Intelligence',
			desc: 'Trading is an isolation sport. We replace that with a hive-mind of disciplined professionals sharing real-time order flow, risk assessments, and sentiment. We win together.',
			color: 'text-blue-400',
			bg: 'bg-blue-400/10',
			border: 'border-blue-400/20'
		},
		{
			icon: Icons.Scale,
			title: 'Probabilistic Thinking',
			desc: 'We reject gambling. We teach the mathematics of "Edge"—thinking in Expected Value (EV), Standard Deviation, and R-Multiples. We do not predict; we react to probability.',
			color: 'text-purple-400',
			bg: 'bg-purple-400/10',
			border: 'border-purple-400/20'
		}
	];

	const syllabus = [
		{
			title: 'Module 1: Market Microstructure',
			desc: 'Understanding the Auction Process, Bid/Ask mechanics, and how liquidity moves price. We deconstruct the Limit Order Book (LOB) and aggressive vs passive order flow.'
		},
		{
			title: 'Module 2: Volume Profiling',
			desc: "Identifying value areas, POC (Point of Control), and composite profiles. Learning to spot 'trapped' traders and liquidity voids where price accelerates."
		},
		{
			title: 'Module 3: Institutional Order Flow',
			desc: 'Reading the DOM (Depth of Market), Delta Divergence, and Footprint charts. Identifying iceberg orders and absorption at key levels.'
		},
		{
			title: 'Module 4: Advanced Risk Modeling',
			desc: 'Building a personalized risk management plan based on Kelly Criterion, Sharpe Ratio optimization, and drawdown recovery protocols.'
		},
		{
			title: 'Module 5: The Psychology of Variance',
			desc: "Cognitive Behavioral Therapy (CBT) techniques for traders. Dealing with 'Recency Bias', 'Tilt', and the physiological response to risk."
		},
		{
			title: 'Module 6: System Architecture',
			desc: 'Building your Playbook. Defining A+ setups versus B setups. Creating a trade execution checklist that removes discretionary errors.'
		}
	];

	const glossary = [
		{
			term: 'Expectancy',
			def: 'The average amount you can expect to win (or lose) per dollar at risk over N trades.'
		},
		{
			term: 'R-Multiple',
			def: 'A standardized unit of risk. We measure success in R units, not dollars.'
		},
		{
			term: 'Auction Theory',
			def: 'The market is a two-way auction process seeking to facilitate trade and find fair value.'
		},
		{
			term: 'Alpha Decay',
			def: 'The reduction in edge effectiveness as more participants exploit the same inefficiency.'
		},
		{
			term: 'Liquidity',
			def: 'The ability to enter or exit a position without significant price impact (slippage).'
		},
		{
			term: 'Variance',
			def: "The statistical deviation from expected results. The 'luck' factor in the short term."
		},
		{
			term: 'Absorption',
			def: 'When aggressive buying/selling is met with passive limit orders, halting price movement.'
		},
		{
			term: 'Delta',
			def: 'The difference between buying volume and selling volume at a specific price node.'
		},
		{
			term: 'Gamma',
			def: 'The rate of change of Delta. Crucial for understanding options dealer hedging flows.'
		},
		{ term: 'Vanna', def: 'The sensitivity of Delta to changes in Implied Volatility.' },
		{
			term: 'Dark Pools',
			def: 'Private exchanges for trading securities that are not accessible by the investing public.'
		},
		{
			term: 'Drawdown',
			def: 'The peak-to-trough decline during a specific record period of an investment.'
		}
	];

	// Svelte 5 derived rune for filtered glossary
	let filteredGlossary = $derived(
		glossary.filter((g) => g.term.toLowerCase().includes(glossarySearch.toLowerCase()))
	);

	const axioms = [
		'Risk comes first.',
		"Don't predict, react.",
		'Cash is a position.',
		'Silence the noise.',
		'Trade the chart, not the P&L.',
		'Patience pays.',
		'Routine equals results.'
	];

	const manifesto = [
		'Capital preservation is the primary objective.',
		"We never sell 'signals' without explaining the logic.",
		'Trading is a business of risk, not a casino.',
		'Consistency is a result of routine, not luck.',
		'Losses are tuition fees for the market.',
		'We are fiduciaries to our own education.'
	];
</script>

<SEOHead
	title="Our Mission | The Institutional Bridge"
	description="Dismantling the retail trader stereotype. We bridge the gap between gambling and institutional risk management through data, discipline, and transparency. Join the 1% of traders who treat this as a business."
	canonical="/our-mission"
	ogType="website"
	ogImage="/og-image.webp"
	ogImageAlt="Revolution Trading Pros Mission - Building Trading Careers"
	keywords={[
		'trading mission',
		'institutional trading',
		'retail trading education',
		'auction market theory',
		'risk management',
		'trading psychology',
		'order flow',
		'trading career',
		'professional trading'
	]}
	schema={jsonLd['@graph']}
	author="Billy Ribeiro"
/>

<div class="bg-[#050505] text-slate-300 font-sans selection:bg-rtp-primary/30 selection:text-white">
	<div class="fixed inset-0 z-0 pointer-events-none">
		<div
			class="absolute top-[-10%] left-1/2 -translate-x-1/2 w-[1200px] h-[800px] bg-rtp-primary/5 blur-[120px] rounded-full mix-blend-screen"
		></div>
		<div
			class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHZpZXdCb3g9IjAgMCA2MCA2MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cGF0aCBkPSJNNTkuNSA2MEg2MHYtMC41SDU5LjVWNTBINjB2LTAuNUg1OS41VjQwSDYwdi0wLjVINTkuNVYzMEg2MHYtMC41SDU5LjVWMjBINjB2LTAuNUg1OS41VjEwSDYwdi0wLjVINTkuNVYwSDYwdjYwaC0wLjV6IiBmaWxsPSIjZmZmIiBmaWxsLW9wYWNpdHk9IjAuMDIiLz48L3N2Zz4=')] opacity-20"
		></div>
		<div
			class="absolute bottom-0 right-0 w-[800px] h-[600px] bg-rtp-indigo/5 blur-[100px] rounded-full"
		></div>
	</div>

	<div class="relative z-10">
		<div class="w-full bg-rtp-primary/10 border-b border-rtp-primary/20 overflow-hidden py-2">
			<div class="flex whitespace-nowrap animate-ticker">
				{#each [...axioms, ...axioms, ...axioms] as axiom}
					<div
						class="flex items-center mx-8 text-xs font-mono font-bold text-rtp-primary uppercase tracking-widest"
					>
						<span class="mr-2">●</span>
						{axiom}
					</div>
				{/each}
			</div>
		</div>

		<section class="relative pt-32 pb-32 px-4 sm:px-6 lg:px-8 max-w-7xl mx-auto text-center">
			<div
				data-gsap
				class="inline-flex items-center gap-3 px-4 py-2 rounded-full bg-white/5 border border-white/10 backdrop-blur-md mb-12 shadow-lg shadow-rtp-primary/5 group hover:border-rtp-primary/30 transition-all duration-300 cursor-default"
			>
				<div class="w-2 h-2 rounded-full bg-rtp-primary animate-pulse"></div>
				<span
					class="text-xs font-bold tracking-widest uppercase text-slate-300 group-hover:text-white transition-colors"
					>Mission Control</span
				>
			</div>

			<h1
				data-gsap
				class="text-6xl md:text-8xl lg:text-9xl font-heading font-extrabold text-white tracking-tight mb-10 leading-[0.95]"
			>
				We Don't Sell Dreams.<br />
				We Build
				<span
					class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-primary via-blue-400 to-indigo-400"
					>Careers.</span
				>
			</h1>

			<div data-gsap class="max-w-3xl mx-auto">
				<p class="text-xl md:text-2xl text-slate-400 leading-relaxed font-light">
					Our mission is to dismantle the "Retail Trader" stereotype and rebuild it with <span
						class="text-white font-medium border-b border-rtp-primary/50">Institutional DNA</span
					>. We exist to transition you from a gambler seeking action to a risk manager executing an
					edge.
				</p>
			</div>

			<div data-gsap class="mt-16 grid grid-cols-2 md:grid-cols-4 gap-4 max-w-4xl mx-auto">
				<div class="bg-white/[0.02] border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-white font-mono">90%</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">
						Retail Failure Rate
					</div>
				</div>
				<div class="bg-white/[0.02] border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-rtp-primary font-mono">10+</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">
						Years Experience
					</div>
				</div>
				<div class="bg-white/[0.02] border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-emerald-400 font-mono">100%</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">
						Verified Audits
					</div>
				</div>
				<div class="bg-white/[0.02] border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-white font-mono">$0</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">Hidden Fees</div>
				</div>
			</div>
		</section>

		<section class="py-32 border-y border-white/5 bg-white/[0.01]">
			<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
				<div class="grid lg:grid-cols-12 gap-16 items-center">
					<div class="lg:col-span-5" data-gsap>
						<div class="flex items-center gap-3 mb-8">
							<span class="p-2 bg-red-500/10 rounded-lg text-red-500 border border-red-500/20"
								>{@html Icons.TrendingUp}</span
							>
							<span class="font-bold tracking-widest uppercase text-sm text-red-400"
								>The Reality Check</span
							>
						</div>

						<h2 class="text-4xl md:text-5xl font-heading font-bold text-white mb-8 leading-tight">
							The "90/90/90" Rule is Real.
						</h2>

						<div class="space-y-6 text-lg text-slate-400 leading-relaxed">
							<p>
								It is a known statistic in the brokerage industry: <strong
									>90% of retail traders lose 90% of their money in their first 90 days.</strong
								>
							</p>
							<p>
								Why? Because they enter a battlefield against supercomputers, HFT algorithms, and
								hedge funds armed only with a smartphone and "hope." They lack the mathematical
								framework to survive variance.
							</p>
							<div class="p-6 border border-red-500/20 bg-red-900/5 rounded-xl">
								<h4
									class="text-red-400 font-bold mb-2 text-sm uppercase tracking-wider flex items-center gap-2"
								>
									{@html Icons.Brain} The Cognitive Gap
								</h4>
								<p class="text-sm text-red-200/60">
									Retail traders seek dopamine hits (action). Institutional traders seek boredom
									(execution). The market is designed to transfer wealth from the impatient to the
									patient.
								</p>
							</div>
						</div>
					</div>

					<div class="lg:col-span-7" data-gsap>
						<div
							class="relative bg-[#0a0a0a] border border-white/10 rounded-2xl overflow-hidden shadow-2xl"
						>
							<div
								class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-red-500 to-emerald-500"
							></div>
							<div class="overflow-x-auto">
								<table class="w-full text-left border-collapse">
									<thead>
										<tr
											class="text-xs uppercase tracking-widest border-b border-white/10 text-slate-500"
										>
											<th class="p-6 font-medium bg-white/[0.02]">Metric</th>
											<th class="p-6 font-medium text-red-400 bg-red-900/10 border-l border-white/5"
												>The Amateur</th
											>
											<th
												class="p-6 font-medium text-emerald-400 bg-emerald-900/10 border-l border-white/5"
												>The Professional</th
											>
										</tr>
									</thead>
									<tbody class="text-sm divide-y divide-white/5 font-mono">
										<tr>
											<td class="p-5 font-bold text-white">Focus</td>
											<td class="p-5 text-slate-400 border-l border-white/5">Profit (P&L)</td>
											<td class="p-5 text-white border-l border-white/5">Process & Execution</td>
										</tr>
										<tr>
											<td class="p-5 font-bold text-white">Risk Mgmt</td>
											<td class="p-5 text-slate-400 border-l border-white/5">Arbitrary / Emotion</td
											>
											<td class="p-5 text-white border-l border-white/5"
												>Fixed % (Kelly Criterion)</td
											>
										</tr>
										<tr>
											<td class="p-5 font-bold text-white">Data Source</td>
											<td class="p-5 text-slate-400 border-l border-white/5"
												>Lagging Indicators (RSI)</td
											>
											<td class="p-5 text-white border-l border-white/5">Order Flow & Volume</td>
										</tr>
										<tr>
											<td class="p-5 font-bold text-white">Timeframe</td>
											<td class="p-5 text-slate-400 border-l border-white/5"
												>Immediate Gratification</td
											>
											<td class="p-5 text-white border-l border-white/5">Multi-Quarter Growth</td>
										</tr>
										<tr>
											<td class="p-5 font-bold text-white">Losses</td>
											<td class="p-5 text-slate-400 border-l border-white/5">Source of Anger</td>
											<td class="p-5 text-white border-l border-white/5">Cost of Business</td>
										</tr>
									</tbody>
								</table>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-32 bg-[#080808] border-b border-white/5 relative overflow-hidden">
			<div
				class="absolute -left-40 top-20 w-96 h-96 bg-emerald-500/5 blur-[100px] rounded-full pointer-events-none"
			></div>

			<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
				<div class="text-center mb-16">
					<h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4">
						The Mathematics of Edge
					</h2>
					<p class="text-slate-400 max-w-2xl mx-auto">
						Trading is not magic. It is statistics. Use this institutional risk calculator to see
						how small changes in risk management define your survival.
					</p>
				</div>

				<div
					class="grid md:grid-cols-12 gap-8 bg-[#050505] border border-white/10 rounded-3xl p-8 md:p-12 shadow-2xl"
				>
					<div class="md:col-span-4 space-y-8">
						<div>
							<label
								for="sim-account"
								class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-4 block"
								>Account Size ($)</label
							>
							<div class="relative">
								<span class="absolute left-4 top-3 text-slate-500">$</span>
								<input
									id="sim-account"
									type="number"
									bind:value={simAccount}
									class="w-full bg-white/5 border border-white/10 rounded-lg py-3 pl-8 pr-4 text-white font-mono focus:border-rtp-primary focus:outline-none transition-colors"
								/>
							</div>
						</div>
						<div>
							<label
								for="sim-risk"
								class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-4 block flex justify-between"
							>
								<span>Risk Per Trade (%)</span>
								<span class={riskColor}>{simRisk}%</span>
							</label>
							<input
								id="sim-risk"
								type="range"
								min="0.5"
								max="10"
								step="0.5"
								bind:value={simRisk}
								class="w-full accent-rtp-primary cursor-pointer"
							/>
							<div class="flex justify-between text-[10px] text-slate-600 mt-2 font-mono">
								<span>Conservative (0.5%)</span>
								<span>Gambler (10%)</span>
							</div>
						</div>
						<div class="grid grid-cols-2 gap-4">
							<div>
								<label
									for="sim-winrate"
									class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-2 block"
									>Win Rate (%)</label
								>
								<input
									id="sim-winrate"
									type="number"
									bind:value={simWinRate}
									class="w-full bg-white/5 border border-white/10 rounded-lg py-2 px-3 text-white font-mono focus:border-rtp-primary focus:outline-none"
								/>
							</div>
							<div>
								<label
									for="sim-rr"
									class="text-xs uppercase tracking-widest text-slate-500 font-bold mb-2 block"
									>R:R Ratio</label
								>
								<input
									id="sim-rr"
									type="number"
									step="0.1"
									bind:value={simRR}
									class="w-full bg-white/5 border border-white/10 rounded-lg py-2 px-3 text-white font-mono focus:border-rtp-primary focus:outline-none"
								/>
							</div>
						</div>
					</div>

					<div class="md:col-span-8 flex flex-col justify-center border-l border-white/5 md:pl-12">
						<div class="grid grid-cols-2 gap-6 mb-8">
							<div class="p-4 bg-white/[0.02] rounded-lg border border-white/5">
								<div class="text-xs text-slate-500 uppercase mb-1">Dollar Risk (1R)</div>
								<div class="text-2xl text-white font-mono font-bold">${riskAmount}</div>
							</div>
							<div class="p-4 bg-white/[0.02] rounded-lg border border-white/5">
								<div class="text-xs text-slate-500 uppercase mb-1">Potential Reward</div>
								<div class="text-2xl text-emerald-400 font-mono font-bold">${winAmount}</div>
							</div>
							<div class="p-4 bg-white/[0.02] rounded-lg border border-white/5">
								<div class="text-xs text-slate-500 uppercase mb-1">Trades to Double</div>
								<div class="text-2xl text-rtp-primary font-mono font-bold">
									{tradesToDouble > 0 ? tradesToDouble : '∞'}
								</div>
							</div>
							<div class="p-4 bg-white/[0.02] rounded-lg border border-white/5">
								<div class="text-xs text-slate-500 uppercase mb-1">Expectancy (per trade)</div>
								<div
									class="text-2xl font-mono font-bold {expectedValue > 0
										? 'text-white'
										: 'text-red-500'}"
								>
									${expectedValue.toFixed(2)}
								</div>
							</div>
						</div>

						<div class="bg-black/40 rounded-xl p-6 border border-white/10 relative overflow-hidden">
							<div
								class="absolute top-2 right-4 text-[10px] font-mono text-slate-600 uppercase tracking-widest"
							>
								Analysis Output
							</div>
							<div class="space-y-4">
								<div class="flex justify-between items-center border-b border-white/5 pb-2">
									<span class="text-slate-400 text-sm">Risk Classification</span>
									<span class="font-bold font-mono {riskColor}">{riskOfRuin}</span>
								</div>
								<div class="flex justify-between items-center border-b border-white/5 pb-2">
									<span class="text-slate-400 text-sm">Institutional Viability</span>
									<span
										class="font-bold font-mono {simRisk <= 2 ? 'text-emerald-400' : 'text-red-500'}"
										>{simRisk <= 2 ? 'PASS' : 'FAIL'}</span
									>
								</div>
								<p class="text-xs text-slate-500 leading-relaxed mt-2">
									{#if simRisk > 2}
										<span class="text-red-400 font-bold">WARNING:</span> Your risk per trade is too
										high for institutional standards. A standard drawdown sequence (4-5 losses) will
										cause significant emotional damage (-{simRisk * 5}% equity).
									{:else}
										<span class="text-emerald-400 font-bold">OPTIMAL:</span> Your sizing allows you
										to weather statistical variance. You can survive a 10-trade losing streak with
										only {simRisk * 10}% drawdown.
									{/if}
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-32 relative overflow-hidden">
			<div
				class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[1000px] h-[1000px] bg-indigo-500/5 blur-[150px] rounded-full pointer-events-none"
			></div>

			<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
				<div class="text-center mb-20">
					<div data-gsap class="inline-block mb-4">
						<span class="text-rtp-primary font-bold tracking-widest uppercase text-sm"
							>The Ecosystem</span
						>
					</div>
					<h2 data-gsap class="text-4xl md:text-5xl font-heading font-bold text-white mb-6">
						Institutional Grade.<br />Retail Accessible.
					</h2>
					<p data-gsap class="text-slate-400 max-w-2xl mx-auto text-lg">
						We built the environment we wished existed when we started. A sanctuary of data,
						discipline, and truth.
					</p>
				</div>

				<div class="grid md:grid-cols-3 gap-8">
					{#each pillars as pillar, i}
						<div data-gsap={{ delay: i * 150 }} class="group relative h-full">
							<div
								class="absolute inset-0 bg-[#0f172a] rounded-2xl transform transition-transform duration-300 group-hover:scale-[1.02]"
							></div>
							<div
								class="relative h-full p-8 bg-[#0a0a0a] border border-white/10 rounded-2xl hover:border-white/20 transition-colors duration-300 flex flex-col"
							>
								<div
									class={`w-14 h-14 ${pillar.bg} rounded-xl flex items-center justify-center ${pillar.color} mb-8 border ${pillar.border}`}
								>
									{@html pillar.icon}
								</div>
								<h3 class="text-2xl font-bold text-white mb-4">{pillar.title}</h3>
								<p class="text-slate-400 leading-relaxed flex-grow">
									{pillar.desc}
								</p>
								<div
									class="mt-8 pt-6 border-t border-white/5 flex items-center gap-2 text-sm font-bold text-white/60 group-hover:text-white transition-colors"
								>
									Learn More {@html Icons.ArrowRight}
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="py-32 bg-gradient-to-b from-[#050505] to-[#0a0a0a] border-y border-white/5">
			<div class="max-w-3xl mx-auto px-6">
				<div class="flex justify-center mb-12">
					<div
						class="w-20 h-20 rounded-full bg-white/5 border border-white/10 flex items-center justify-center text-white text-3xl font-heading font-bold text-rtp-primary shadow-[0_0_30px_rgba(59,130,246,0.1)]"
					>
						R
					</div>
				</div>
				<div class="prose prose-invert prose-lg mx-auto font-light">
					<h3 class="text-center font-heading text-3xl text-white mb-10">A Note from the Desk</h3>
					<p>
						I started Revolution Trading Pros because I was tired of seeing good people lose money
						to bad advice. The internet is flooded with "gurus" renting Lamborghinis, selling the
						dream of easy money.
					</p>
					<p>
						<strong>Trading is not easy.</strong> It is the hardest way to make an easy living.
					</p>
					<p>
						When I worked on the institutional side, I saw how the sausage was made. I saw the
						algorithms designed to hunt retail stop losses. I saw the order flow that moves markets
						before the news even hits your feed. I realized that the retail trader is playing a game
						they don't even understand.
					</p>
					<blockquote class="border-l-4 border-rtp-primary pl-6 italic text-white text-xl my-8">
						"The gap between 'Retail' and 'Pro' isn't intelligence. It's information and
						discipline."
					</blockquote>
					<p>
						We built this platform to bridge that gap. To give you the tools (Bookmap, Gamma
						Exposure), the data, and the community you need to survive the learning curve and thrive
						in the volatility.
					</p>
					<p>
						We don't promise you'll get rich quick. We promise we will tell you the truth about what
						it takes. We promise to treat you like a professional from day one.
					</p>
					<div class="mt-12 pt-8 border-t border-white/10 flex items-center gap-4">
						<div
							class="w-12 h-12 rounded-full bg-white/10 flex items-center justify-center font-bold text-white text-lg"
						>
							B
						</div>
						<div>
							<p class="text-white font-bold font-heading">Billy Ribeiro</p>
							<p class="text-sm text-slate-500">Founder & Head Trader</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-32 bg-[#020202] border-t border-white/5">
			<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
				<div class="grid lg:grid-cols-12 gap-20">
					<div data-gsap class="lg:col-span-5 sticky top-32 h-fit">
						<div class="flex items-center gap-3 mb-6">
							<span class="p-2 bg-blue-500/10 rounded-lg text-blue-500 border border-blue-500/20"
								>{@html Icons.Book}</span
							>
							<span class="font-bold tracking-widest uppercase text-sm text-blue-400"
								>The Syllabus</span
							>
						</div>
						<h2 class="text-4xl md:text-5xl font-heading font-bold text-white mb-8">
							From Subjective to <br />
							<span class="text-white border-b-4 border-rtp-primary">Objective.</span>
						</h2>
						<p class="text-lg text-slate-400 mb-8 leading-relaxed">
							Most traders look at a chart and see "patterns." We look at a chart and see <strong
								>Auctions</strong
							>.
						</p>
						<p class="text-lg text-slate-400 mb-12 leading-relaxed">
							We teach you to read the raw data of the market: Volume, Liquidity, and Time. This
							allows you to identify where the "Smart Money" is transacting, not just where price
							has been.
						</p>

						<div class="p-1 bg-white/5 rounded-xl border border-white/10">
							<div
								class="aspect-video bg-[#0f172a] rounded-lg flex items-center justify-center relative overflow-hidden group"
							>
								<div
									class="absolute inset-0 bg-[linear-gradient(0deg,rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:20px_20px]"
								></div>
								<div class="absolute left-0 bottom-20 h-4 w-32 bg-blue-500/20 rounded-r"></div>
								<div class="absolute left-0 bottom-24 h-4 w-48 bg-blue-500/40 rounded-r"></div>
								<div class="absolute left-0 bottom-28 h-4 w-64 bg-blue-500/60 rounded-r"></div>
								<div class="absolute left-0 bottom-32 h-4 w-40 bg-blue-500/40 rounded-r"></div>
								<div class="absolute left-0 bottom-36 h-4 w-24 bg-blue-500/20 rounded-r"></div>

								<div class="absolute right-10 top-10 text-right">
									<div class="text-xs text-slate-500 font-mono">POC: 4450.25</div>
									<div class="text-xs text-slate-500 font-mono">VAH: 4462.00</div>
									<div class="text-xs text-slate-500 font-mono">VAL: 4438.50</div>
								</div>
								<p
									class="text-center text-xs text-slate-500 mt-24 font-mono uppercase tracking-widest z-10"
								>
									Figure 1.1: Auction Market Theory
								</p>
							</div>
						</div>
					</div>

					<div class="lg:col-span-7 space-y-4 pt-8">
						{#each syllabus as module, i}
							<div
								class="border border-white/10 rounded-xl bg-[#0a0a0a] overflow-hidden group hover:border-rtp-primary/50 transition-all duration-300"
							>
								<button
									class="w-full flex justify-between items-center p-6 text-left"
									onclick={() => toggleSyllabus(i)}
								>
									<div class="flex items-center gap-6">
										<span
											class="text-rtp-primary font-mono text-sm bg-rtp-primary/10 px-3 py-1 rounded border border-rtp-primary/20"
											>MOD {i + 1}</span
										>
										<span
											class="font-bold text-white text-lg group-hover:text-rtp-primary transition-colors"
											>{module.title}</span
										>
									</div>
									<div
										class="w-8 h-8 rounded-full bg-white/5 flex items-center justify-center text-slate-500 transform transition-transform duration-300 {openSyllabus ===
										i
											? 'rotate-180 bg-white/10 text-white'
											: ''}"
									>
										{@html Icons.ChevronDown}
									</div>
								</button>
								{#if openSyllabus === i}
									<div
										transition:slide
										class="px-6 pb-8 pl-[5.5rem] text-slate-400 leading-relaxed border-t border-white/5 pt-6 text-sm"
									>
										{module.desc}
										<div class="mt-4 flex gap-4">
											<span class="text-xs font-mono text-emerald-400 flex items-center gap-1"
												>{@html Icons.Check} Video</span
											>
											<span class="text-xs font-mono text-emerald-400 flex items-center gap-1"
												>{@html Icons.Check} PDF</span
											>
											<span class="text-xs font-mono text-emerald-400 flex items-center gap-1"
												>{@html Icons.Check} Quiz</span
											>
										</div>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			</div>
		</section>

		<section class="py-24 bg-white/[0.01] border-y border-white/5">
			<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
				<div class="flex flex-col md:flex-row justify-between items-end mb-12 gap-6">
					<div>
						<h2 class="text-3xl font-heading font-bold text-white mb-2">
							The Institutional Lexicon
						</h2>
						<p class="text-slate-400">We speak the language of the market. You will too.</p>
					</div>
					<div class="relative">
						<div class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500 w-4 h-4">
							{@html Icons.Target}
						</div>
						<input
							id="page-glossarysearch" name="page-glossarysearch" type="text"
							bind:value={glossarySearch}
							placeholder="Search terms..."
							class="bg-[#0a0a0a] border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm text-white focus:border-rtp-primary outline-none w-64 transition-colors"
						/>
					</div>
				</div>

				<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each filteredGlossary as item}
						<div
							class="p-6 bg-[#0a0a0a] border border-white/10 rounded-xl hover:border-white/30 transition-colors group"
						>
							<h4
								class="text-rtp-primary font-bold mb-2 font-mono uppercase tracking-wide text-sm group-hover:text-white transition-colors"
							>
								{item.term}
							</h4>
							<p class="text-sm text-slate-400 leading-relaxed">{item.def}</p>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="py-32 px-4 sm:px-6 lg:px-8">
			<div class="max-w-6xl mx-auto">
				<div
					data-gsap
					class="relative bg-gradient-to-br from-[#0f172a] to-[#020617] border border-white/10 rounded-3xl p-8 md:p-16 overflow-hidden shadow-2xl"
				>
					<div class="absolute top-0 right-0 text-white/5 -mr-8 -mt-8 transform rotate-12">
						<div class="w-64 h-64">{@html Icons.Quote}</div>
					</div>

					<div class="relative z-10">
						<div class="text-center mb-12">
							<h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4">
								The Operator's Manifesto
							</h2>
							<div class="h-1 w-20 bg-rtp-primary mx-auto rounded-full"></div>
						</div>

						<div class="grid md:grid-cols-2 gap-x-16 gap-y-8">
							{#each manifesto as item}
								<div class="flex items-start gap-4 group">
									<div
										class="mt-1 text-emerald-500 bg-emerald-500/10 p-1 rounded-md transition-colors group-hover:bg-emerald-500 group-hover:text-white"
									>
										{@html Icons.Check}
									</div>
									<span
										class="text-slate-300 font-medium text-lg group-hover:text-white transition-colors"
										>{item}</span
									>
								</div>
							{/each}
						</div>

						<div class="mt-16 text-center">
							<p class="text-slate-500 text-sm font-mono uppercase tracking-widest">
								Est. 2023 // New York • London • Tokyo
							</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-24 bg-white/[0.02] border-t border-white/5">
			<div class="max-w-3xl mx-auto px-4">
				<div class="text-center mb-12">
					<h2 class="text-3xl font-bold text-white mb-4">Common Questions</h2>
					<p class="text-slate-400">Understanding our philosophy before you join.</p>
				</div>

				<div class="space-y-4">
					<div class="border border-white/10 rounded-lg bg-[#0a0a0a] overflow-hidden">
						<button
							class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
							onclick={() => toggleFaq(0)}
						>
							<span class="font-bold text-white">Is this a "Get Rich Quick" scheme?</span>
							<span
								class={`text-slate-500 transform transition-transform duration-300 ${openFaq === 0 ? 'rotate-180' : ''}`}
								>{@html Icons.ChevronDown}</span
							>
						</button>
						{#if openFaq === 0}
							<div
								transition:slide
								class="px-6 pb-6 text-slate-400 leading-relaxed border-t border-white/5 pt-4"
							>
								Absolutely not. If you are looking for fast money, please look elsewhere. We teach a
								profession that takes years to master. We promise hard work, not Lamborghinis.
							</div>
						{/if}
					</div>

					<div class="border border-white/10 rounded-lg bg-[#0a0a0a] overflow-hidden">
						<button
							class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
							onclick={() => toggleFaq(1)}
						>
							<span class="font-bold text-white">Do I need a large account to start?</span>
							<span
								class={`text-slate-500 transform transition-transform duration-300 ${openFaq === 1 ? 'rotate-180' : ''}`}
								>{@html Icons.ChevronDown}</span
							>
						</button>
						{#if openFaq === 1}
							<div
								transition:slide
								class="px-6 pb-6 text-slate-400 leading-relaxed border-t border-white/5 pt-4"
							>
								No. In fact, we recommend starting with a small account or a simulator.
								Institutional risk management concepts (Position Sizing, R-Multiples) apply whether
								you are trading $1,000 or $1,000,000.
							</div>
						{/if}
					</div>

					<div class="border border-white/10 rounded-lg bg-[#0a0a0a] overflow-hidden">
						<button
							class="w-full flex justify-between items-center p-6 text-left hover:bg-white/5 transition-colors"
							onclick={() => toggleFaq(2)}
						>
							<span class="font-bold text-white">What trading style do you teach?</span>
							<span
								class={`text-slate-500 transform transition-transform duration-300 ${openFaq === 2 ? 'rotate-180' : ''}`}
								>{@html Icons.ChevronDown}</span
							>
						</button>
						{#if openFaq === 2}
							<div
								transition:slide
								class="px-6 pb-6 text-slate-400 leading-relaxed border-t border-white/5 pt-4"
							>
								We are primarily Day Traders and Swing Traders. We focus on Index Futures (ES, NQ)
								and Large Cap Tech Options. Our methodology is based on Price Action, Volume
								Profile, and Order Flow.
							</div>
						{/if}
					</div>
				</div>
			</div>
		</section>

		<section class="py-32 text-center relative overflow-hidden">
			<div
				class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-full bg-gradient-to-b from-rtp-primary/5 to-transparent pointer-events-none"
			></div>

			<div data-gsap class="max-w-4xl mx-auto px-4 relative z-10">
				<div
					class="inline-block p-4 rounded-full bg-white/5 border border-white/10 mb-8 animate-bounce"
				>
					<div class="w-12 h-12 text-rtp-primary">{@html Icons.Brain}</div>
				</div>

				<h2 class="text-5xl md:text-6xl font-heading font-extrabold text-white mb-8 tracking-tight">
					Stop Gambling. <br />
					Start <span class="text-rtp-primary">Operating.</span>
				</h2>

				<p class="text-xl text-slate-400 mb-12 max-w-2xl mx-auto">
					Join the only trading environment dedicated to the professional development of the retail
					trader. Your career starts here.
				</p>

				<div class="flex flex-col sm:flex-row items-center justify-center gap-6">
					<a
						href="/pricing"
						class="group flex items-center gap-3 bg-rtp-primary text-white px-10 py-5 rounded-xl font-bold text-lg hover:bg-blue-600 transition-all hover:-translate-y-1 shadow-[0_0_30px_rgba(59,130,246,0.3)]"
					>
						Join the Professional Tier
						<span class="group-hover:translate-x-1 transition-transform"
							>{@html Icons.ArrowRight}</span
						>
					</a>
					<a
						href="/methodology"
						class="px-10 py-5 rounded-xl font-bold text-lg text-slate-300 border border-white/10 hover:bg-white/5 transition-all"
					>
						Explore Our Methodology
					</a>
				</div>

				<p class="mt-8 text-sm text-slate-600 font-mono">
					30-Day Money Back Guarantee • Cancel Anytime
				</p>
			</div>
		</section>
	</div>
</div>

<MarketingFooter />

<style>
	/* --- Custom Styles for Specific Effects --- */
	.font-heading {
		font-family:
			'Inter',
			system-ui,
			-apple-system,
			sans-serif;
		letter-spacing: -0.03em;
	}

	/* Gradient Text Utility */
	.text-gradient {
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-image: linear-gradient(to right, #3b82f6, #6366f1);
	}

	/* Animation: Ticker */
	@keyframes ticker {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}

	.animate-ticker {
		animation: ticker 30s linear infinite;
	}

	/* Animation: Pulse Glow */
	@keyframes pulse-glow {
		0%,
		100% {
			box-shadow: 0 0 10px rgba(59, 130, 246, 0.1);
		}
		50% {
			box-shadow: 0 0 20px rgba(59, 130, 246, 0.3);
		}
	}

	/* Custom Scrollbar */
	::-webkit-scrollbar {
		width: 8px;
	}
	::-webkit-scrollbar-track {
		background: #020202;
	}
	::-webkit-scrollbar-thumb {
		background: #1e293b;
		border-radius: 4px;
	}
	::-webkit-scrollbar-thumb:hover {
		background: #334155;
	}
</style>
