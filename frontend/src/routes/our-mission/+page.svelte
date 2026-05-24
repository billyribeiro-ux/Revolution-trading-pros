<script lang="ts">
	import type { RawSchemaConfig } from '$lib/utils/structured-data';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/seo/SeoHead.svelte';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconShield from '@tabler/icons-svelte-runes/icons/shield';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconScale from '@tabler/icons-svelte-runes/icons/scale';
	import IconBrain from '@tabler/icons-svelte-runes/icons/brain';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconQuote from '@tabler/icons-svelte-runes/icons/quote';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import MathOfEdgeCalculator from './_sections/MathOfEdgeCalculator.svelte';
	import SyllabusAccordion from './_sections/SyllabusAccordion.svelte';
	import MissionFaqAccordion from './_sections/MissionFaqAccordion.svelte';
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
				// Hide ALL data-gsap elements initially. Doing this synchronously
				// inside the gsap context ensures the entrance tweens have a
				// from-state to animate from. The brief flash before this runs
				// is acceptable and identical to the existing pattern across the
				// site; CSS-level pre-hiding is intentionally avoided to keep
				// no-JS / SSR fallbacks visible.
				const all = document.querySelectorAll('[data-gsap]');
				gsap.set(all, { opacity: 0, y: 30 });

				// Split: above-fold animate immediately on mount with a stagger
				// (the hero entrance); below-fold animate on scroll via batch.
				const aboveFold: Element[] = [];
				const belowFold: Element[] = [];
				all.forEach((el) => {
					const top = el.getBoundingClientRect().top;
					if (top < window.innerHeight * 0.85) aboveFold.push(el);
					else belowFold.push(el);
				});

				if (aboveFold.length > 0) {
					gsap.to(aboveFold, {
						opacity: 1,
						y: 0,
						duration: 0.9,
						ease: 'power3.out',
						stagger: 0.1
					});
				}

				if (belowFold.length > 0) {
					ScrollTrigger.batch(belowFold, {
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
				}

				ScrollTrigger.refresh();
			});
		})();

		return () => ctx?.revert();
	});


	// --- STATES (Svelte 5 runes) ---
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
			icon: IconShield,
			title: 'Radical Transparency',
			desc: 'We operate in a glass house. We publish raw track records, including losses and commissions. In an industry of smoke and mirrors, truth is the only currency.',
			color: 'text-emerald-400',
			bg: 'bg-emerald-400/10',
			border: 'border-emerald-400/20'
		},
		{
			icon: IconUsers,
			title: 'Collective Intelligence',
			desc: 'Trading is an isolation sport. We replace that with a hive-mind of disciplined professionals sharing real-time order flow, risk assessments, and sentiment. We win together.',
			color: 'text-blue-400',
			bg: 'bg-blue-400/10',
			border: 'border-blue-400/20'
		},
		{
			icon: IconScale,
			title: 'Probabilistic Thinking',
			desc: 'We reject gambling. We teach the mathematics of "Edge"—thinking in Expected Value (EV), Standard Deviation, and R-Multiples. We do not predict; we react to probability.',
			color: 'text-purple-400',
			bg: 'bg-purple-400/10',
			border: 'border-purple-400/20'
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
	canonicalUrl="/our-mission"
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
	structuredData={(jsonLd['@graph'] as Record<string, unknown>[]).map((d) => ({ type: 'Raw' as const, data: d })) satisfies RawSchemaConfig[]}
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
				{#each [...axioms, ...axioms] as axiom, _ai (_ai)}
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
					class="text-transparent bg-clip-text bg-linear-to-r from-rtp-primary via-blue-400 to-indigo-400"
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
				<div class="bg-white/2 border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-white font-mono">90%</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">
						Retail Failure Rate
					</div>
				</div>
				<div class="bg-white/2 border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-rtp-primary font-mono">10+</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">
						Years Experience
					</div>
				</div>
				<div class="bg-white/2 border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-emerald-400 font-mono">100%</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">
						Verified Audits
					</div>
				</div>
				<div class="bg-white/2 border border-white/10 p-6 rounded-xl backdrop-blur-sm">
					<div class="text-3xl font-bold text-white font-mono">$0</div>
					<div class="text-[10px] uppercase tracking-widest text-slate-500 mt-2">Hidden Fees</div>
				</div>
			</div>
		</section>

		<section class="py-32 border-y border-white/5 bg-white/1">
			<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
				<div class="grid lg:grid-cols-12 gap-16 items-center">
					<div class="lg:col-span-5" data-gsap>
						<div class="flex items-center gap-3 mb-8">
							<span class="p-2 bg-red-500/10 rounded-lg text-red-500 border border-red-500/20"><IconTrendingUp size={20} stroke={1.5} /></span>
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
									<IconBrain size={16} stroke={1.5} /> The Cognitive Gap
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
								class="absolute top-0 left-0 w-full h-1 bg-linear-to-r from-red-500 to-emerald-500"
							></div>
							<div class="overflow-x-auto">
								<table class="w-full text-left border-collapse">
									<thead>
										<tr
											class="text-xs uppercase tracking-widest border-b border-white/10 text-slate-500"
										>
											<th class="p-6 font-medium bg-white/2">Metric</th>
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

		<MathOfEdgeCalculator />

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
					{#each pillars as pillar, _i (pillar.title)}
						{@const PillarIcon = pillar.icon}
						<div data-gsap class="group relative h-full">
							<div
								class="absolute inset-0 bg-[#0f172a] rounded-2xl transform transition-transform duration-300 group-hover:scale-[1.02]"
							></div>
							<div
								class="relative h-full p-8 bg-[#0a0a0a] border border-white/10 rounded-2xl hover:border-white/20 transition-colors duration-300 flex flex-col"
							>
								<div
									class={`w-14 h-14 ${pillar.bg} rounded-xl flex items-center justify-center ${pillar.color} mb-8 border ${pillar.border}`}
								>
									<PillarIcon size={28} stroke={1.5} />
								</div>
								<h3 class="text-2xl font-bold text-white mb-4">{pillar.title}</h3>
								<p class="text-slate-400 leading-relaxed grow">
									{pillar.desc}
								</p>
								<div
									class="mt-8 pt-6 border-t border-white/5 flex items-center gap-2 text-sm font-bold text-white/60 group-hover:text-white transition-colors"
								>
									Learn More <IconArrowRight size={16} stroke={2} />
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="py-32 bg-linear-to-b from-[#050505] to-[#0a0a0a] border-y border-white/5">
			<div class="max-w-3xl mx-auto px-6">
				<div class="flex justify-center mb-12">
					<div
						class="w-20 h-20 rounded-full bg-white/5 border border-white/10 flex items-center justify-center text-3xl font-heading font-bold text-rtp-primary shadow-[0_0_30px_rgba(59,130,246,0.1)]"
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

		<SyllabusAccordion />

		<section class="py-24 bg-white/1 border-y border-white/5">
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
							<IconSearch size={16} stroke={1.5} />
						</div>
						<input
							id="page-glossarysearch"
							name="page-glossarysearch"
							type="text"
							bind:value={glossarySearch}
							placeholder="Search terms..."
							class="bg-[#0a0a0a] border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm text-white focus:border-rtp-primary outline-none w-64 transition-colors"
						/>
					</div>
				</div>

				<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each filteredGlossary as item (item.term)}
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
					class="relative bg-linear-to-br from-[#0f172a] to-[#020617] border border-white/10 rounded-3xl p-8 md:p-16 overflow-hidden shadow-2xl"
				>
					<div class="absolute top-0 right-0 text-white/5 -mr-8 -mt-8 transform rotate-12">
						<IconQuote size={256} class="text-white/5" />
					</div>

					<div class="relative z-10">
						<div class="text-center mb-12">
							<h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4">
								The Operator's Manifesto
							</h2>
							<div class="h-1 w-20 bg-rtp-primary mx-auto rounded-full"></div>
						</div>

						<div class="grid md:grid-cols-2 gap-x-16 gap-y-8">
							{#each manifesto as item (item)}
								<div class="flex items-start gap-4 group">
									<div
										class="mt-1 text-emerald-500 bg-emerald-500/10 p-1 rounded-md transition-colors group-hover:bg-emerald-500 group-hover:text-white"
									>
										<IconCheck size={16} stroke={2} />
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

		<MissionFaqAccordion />

		<section class="py-32 text-center relative overflow-hidden">
			<div
				class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-full bg-linear-to-b from-rtp-primary/5 to-transparent pointer-events-none"
			></div>

			<div data-gsap class="max-w-4xl mx-auto px-4 relative z-10">
				<div
					class="inline-block p-4 rounded-full bg-white/5 border border-white/10 mb-8 animate-bounce"
				>
					<IconBrain size={48} stroke={1.5} class="text-rtp-primary" />
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
						<span class="group-hover:translate-x-1 transition-transform"><IconArrowRight size={20} stroke={2} /></span>
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


<style>
	/* Pre-hide data-gsap elements so the GSAP entrance animation has a
	   from-state without a flash-of-visible-content during the dynamic
	   import. The `transition: none` rule prevents Tailwind's
	   `transition-all` (used on the hero badge for hover effects) from
	   interpolating against GSAP's per-frame opacity ticks, which would
	   otherwise make GSAP's animation appear ~6× slower. Reduced-motion
	   users (and no-JS) bypass these rules. */
	:global([data-gsap]) {
		opacity: 0;
		transform: translateY(30px);
		transition: none;
	}
	@media (prefers-reduced-motion: reduce) {
		:global([data-gsap]) {
			opacity: 1;
			transform: none;
			transition: revert;
		}
	}

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
	:global(::-webkit-scrollbar) {
		width: 8px;
	}
	:global(::-webkit-scrollbar-track) {
		background: #020202;
	}
	:global(::-webkit-scrollbar-thumb) {
		background: #1e293b;
		border-radius: 4px;
	}
	:global(::-webkit-scrollbar-thumb:hover) {
		background: #334155;
	}
</style>
