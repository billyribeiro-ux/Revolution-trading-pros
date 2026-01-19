<script lang="ts">
	import './explosive-swings.css';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// --- Pricing State ---
	type PlanType = 'monthly' | 'quarterly' | 'annual';
	let selectedPlan = $state<PlanType>('quarterly');

	// Toggle indicator position (0 = monthly, 1 = quarterly, 2 = annual)
	let togglePosition = $derived(
		selectedPlan === 'monthly' ? 0 : selectedPlan === 'quarterly' ? 1 : 2
	);

	// Pricing Data Configuration
	const pricing = {
		monthly: {
			price: '97',
			period: '/mo',
			btnText: 'Select Monthly',
			link: '/checkout/monthly-swings',
			savings: null,
			tagline: 'Flexibility to cancel anytime'
		},
		quarterly: {
			price: '247',
			period: '/qtr',
			btnText: 'Join Quarterly',
			link: '/checkout/quarterly-swings',
			savings: 'Most Popular',
			tagline: 'Save 15% ($8.20 / trading day)'
		},
		annual: {
			price: '927',
			period: '/yr',
			btnText: 'Select Annual',
			link: '/checkout/annual-swings',
			savings: 'Save 20%',
			tagline: 'Like getting 2.5 months FREE'
		}
	};

	// --- FAQ State ---
	let openFaq: number | null = $state(null);
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	// --- Icon SVG ---

	// --- Apple ICT9+ Scroll Animations ---
	// Smooth, performant reveal animations using IntersectionObserver

	function reveal(node: HTMLElement, params: { delay?: number; y?: number } = {}) {
		const delay = params.delay ?? 0;
		const translateY = params.y ?? 30;

		// Check for reduced motion preference
		if (
			typeof window !== 'undefined' &&
			window.matchMedia('(prefers-reduced-motion: reduce)').matches
		) {
			return { destroy() {} };
		}

		// Set initial hidden state with transition already applied
		node.style.opacity = '0';
		node.style.transform = `translateY(${translateY}px)`;
		node.style.transition = `opacity 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94), transform 0.8s cubic-bezier(0.25, 0.46, 0.45, 0.94)`;
		node.style.transitionDelay = `${delay}ms`;
		node.style.willChange = 'opacity, transform';

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						// Trigger animation
						node.style.opacity = '1';
						node.style.transform = 'translateY(0)';

						// Cleanup will-change after animation
						setTimeout(() => {
							node.style.willChange = 'auto';
						}, 800 + delay);

						observer.unobserve(node);
					}
				});
			},
			{ threshold: 0.1, rootMargin: '0px 0px -60px 0px' }
		);

		observer.observe(node);

		return {
			destroy() {
				observer.disconnect();
			}
		};
	}

	// --- EXPANDED FAQ DATA FOR SEO & USER CLARITY ---
	const faqData = [
		{
			q: 'How much capital do I need for swing trading?',
			a: 'We recommend a minimum of $2,000 to properly manage risk. Our position sizing model suggests risking only 1-2% of your account per trade. While the strategies work mathematically on smaller accounts, $2,000 allows you to take multiple positions simultaneously while maintaining a cash buffer.'
		},
		{
			q: 'Does this trigger the PDT (Pattern Day Trader) rule?',
			a: 'Generally, no. The PDT rule applies if you make 4 or more "day trades" (open and close same day) in a 5-day period on a margin account under $25k. Since our average hold time is 3-7 days, these are swing trades, not day trades. However, occasionally we may advise closing a trade same-day if it hits a target immediately or invalidates, but this is rare.'
		},
		{
			q: 'Do I need to sit at my computer all day?',
			a: 'Absolutely not. This service is specifically engineered for people with full-time jobs. We send alerts with clear entry zones. Because we trade multi-day moves, you typically have a generous window (often 30-60 minutes or more) to enter the trade after the alert is sent. No split-second scalping required.'
		},
		{
			q: 'What specific markets and instruments do you trade?',
			a: 'We focus on large-cap US equities (Magnificent 7: NVDA, TSLA, AAPL, etc.) and high-liquidity ETFs (SPY, QQQ, IWM). We primarily use Long Calls and Long Puts (buying options) to leverage these moves, but we also occasionally issue share-equity signals for those who prefer not to trade options.'
		},
		{
			q: 'How do I receive the alerts?',
			a: 'Redundancy is key. You will receive alerts via specific Discord channels (with push notifications), SMS text messages (US/Canada), and Email. We ensure you get the "ding" on your phone wherever you are.'
		},
		{
			q: 'What is your historical win rate and risk profile?',
			a: 'Our historical win rate hovers around 82% on valid setups. However, win rate matters less than Risk:Reward. We target a minimum 2:1 ratio, often seeking 3:1 or 4:1. This means one winner can pay for 2-3 small losers. We use strict "Hard Stops" on every trade to protect capital.'
		},
		{
			q: 'Is this suitable for beginners?',
			a: 'Yes, provided you understand the basics of how to buy and sell an option in your broker. Our alerts are "copy-paste" simple: Ticker, Strike Price, Expiration, and Price. We also provide a library of educational videos explaining our terminology and how to execute the orders.'
		},
		{
			q: 'What happens if I miss an entry price?',
			a: 'We always provide an "Entry Zone" (e.g., Enter NVDA Calls between $4.50 and $4.80). If price has already moved beyond this zone, we advise WAITING. Stocks often retrace. We strictly advise against "chasing" a trade that has left the station. There will always be another setup.'
		},
		{
			q: 'Do you trade through earnings reports?',
			a: 'Rarely. Holding directional options through earnings is considered gambling due to IV crush (Implied Volatility drop). We typically close positions or tighten stops significantly before a company reports earnings to preserve capital.'
		},
		{
			q: 'Can I use Robinhood, Webull, or ThinkOrSwim?',
			a: 'Yes. Our signals are platform-agnostic. As long as your broker allows you to trade Tier 2 Options (buying calls and puts), you can execute these trades. We recommend ThinkOrSwim or Interactive Brokers for the best execution, but mobile brokers work fine for swing trading.'
		},
		{
			q: 'What is the refund policy?',
			a: "We offer a 30-day money-back guarantee for your first month. If you feel the service isn't right for you, simply email support and we will refund your most recent payment. We want you to stay because you are profitable, not because you are locked in."
		},
		{
			q: 'Do you offer short selling signals?',
			a: 'We play both sides of the market. When the market is bearish, we buy PUT options to profit from the downside. We do not "short sell" shares (which requires margin and has infinite risk); we buy Puts (defined risk) instead.'
		}
	];

	const schemaOrg = {
		'@context': 'https://schema.org',
		'@graph': [
			{
				'@type': 'Product',
				name: 'Explosive Swings Trading Alerts',
				description:
					'Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals. Institutional dark pool analysis.',
				brand: {
					'@type': 'Brand',
					name: 'Revolution Trading Pros'
				},
				image: 'https://revolution-trading-pros.pages.dev/images/og-swings.jpg',
				offers: {
					'@type': 'Offer',
					priceCurrency: 'USD',
					price: '97',
					availability: 'https://schema.org/InStock',
					url: 'https://revolution-trading-pros.pages.dev/alerts/explosive-swings'
				},
				aggregateRating: {
					'@type': 'AggregateRating',
					ratingValue: '4.9',
					reviewCount: '128'
				},
				audience: {
					'@type': 'Audience',
					audienceType: 'Swing Traders, Part-time Traders, Retail Investors'
				}
			},
			{
				'@type': 'FAQPage',
				mainEntity: faqData.map((item) => ({
					'@type': 'Question',
					name: item.q,
					acceptedAnswer: { '@type': 'Answer', text: item.a }
				}))
			}
		]
	};

	// Schema for SEOHead
	const combinedSchema = schemaOrg['@graph'];
</script>

<SEOHead
	title="Explosive Swings Trading Alerts | High-Probability Multi-Day Signals"
	description="Catch 20%+ moves with 3-7 day swing trading alerts. Institutional dark pool analysis and precise options signals for traders with day jobs. 82% historical win rate."
	canonical="/alerts/explosive-swings"
	ogType="product"
	ogImage="/images/og-swings.jpg"
	ogImageAlt="Explosive Swings Trading Alerts - Multi-Day Opportunities"
	keywords={[
		'swing trading alerts',
		'stock options alerts',
		'swing trade signals',
		'multi-day trading strategies',
		'SPX swing trading',
		'dark pool trading',
		'options swing trading',
		'trading signals for beginners',
		'part time trading strategy'
	]}
	schema={combinedSchema}
	schemaType="Product"
	productPrice={97}
	productCurrency="USD"
	productAvailability="in stock"
/>

<main
	class="explosive-swings-page w-full overflow-x-hidden bg-slate-950 text-slate-200 font-sans selection:bg-emerald-500/30 selection:text-emerald-200"
>
	<section class="relative min-h-[90vh] flex items-center overflow-hidden py-20 lg:py-0">
		<div class="absolute inset-0 z-0 pointer-events-none">
			<div
				class="absolute inset-0 bg-[linear-gradient(rgba(16,185,129,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(16,185,129,0.03)_1px,transparent_1px)] bg-[size:48px_48px] opacity-40"
			></div>
			<div
				class="absolute top-0 right-0 w-[600px] h-[600px] bg-emerald-500/10 rounded-full blur-[100px] animate-pulse"
			></div>
			<div
				class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-blue-600/10 rounded-full blur-[120px]"
			></div>
		</div>

		<div
			class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-16 items-center"
		>
			<div class="text-center lg:text-left">
				<div
					use:reveal={{ delay: 0 }}
					class="inline-flex items-center gap-2 bg-slate-900 border border-emerald-500/30 px-4 py-1.5 rounded-full mb-8 shadow-lg shadow-emerald-500/10 backdrop-blur-md"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
					</span>
					<span class="text-xs font-bold tracking-wider uppercase text-emerald-400"
						>Live Signals Active</span
					>
				</div>

				<h1
					use:reveal={{ delay: 100 }}
					class="text-5xl md:text-7xl font-heading font-extrabold mb-6 leading-tight tracking-tight text-white"
				>
					Catch the <br />
					<span
						class="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 via-emerald-300 to-teal-200"
						>Institutional Moves.</span
					>
				</h1>

				<p
					use:reveal={{ delay: 200 }}
					class="text-xl text-slate-400 mb-10 max-w-2xl mx-auto lg:mx-0 leading-relaxed"
				>
					Stop staring at the 1-minute chart. Get high-precision <strong
						>multi-day swing alerts</strong
					> backed by Dark Pool data. Designed for traders who want financial freedom, not another 9-to-5
					job.
				</p>

				<div
					use:reveal={{ delay: 300 }}
					class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start items-center"
				>
					<a
						href="#pricing"
						class="group relative w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-slate-950 transition-all duration-200 bg-emerald-500 rounded-xl hover:bg-emerald-400 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-emerald-500 focus:ring-offset-slate-900 shadow-lg hover:shadow-emerald-500/25 hover:-translate-y-1"
					>
						Start Trading Swings
						<svg
							class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7l5 5m0 0l-5 5m5-5H6"
							/></svg
						>
					</a>
					<a
						href="#methodology"
						class="w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-slate-200 transition-all duration-200 bg-slate-900 border border-slate-700 rounded-xl hover:bg-slate-800 hover:border-emerald-500/30"
					>
						See The Strategy
					</a>
				</div>

				<div
					use:reveal={{ delay: 400 }}
					class="mt-10 flex flex-wrap items-center justify-center lg:justify-start gap-6 text-sm font-medium text-slate-500"
				>
					<div class="flex items-center gap-2">
						<svg
							class="w-5 h-5 text-emerald-500"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>Precise Entries & Exits</span>
					</div>
					<div class="flex items-center gap-2">
						<svg
							class="w-5 h-5 text-emerald-500"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>3-7 Day Hold Time</span>
					</div>
					<div class="flex items-center gap-2">
						<svg
							class="w-5 h-5 text-emerald-500"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 10V3L4 14h7v7l9-11h-7z"
							/>
						</svg>
						<span>High R:R Ratio</span>
					</div>
				</div>
			</div>

			<div class="hidden lg:block relative perspective-1000">
				<div
					class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-gradient-to-tr from-emerald-500/20 to-transparent rounded-full blur-3xl"
				></div>

				<div
					class="relative bg-slate-900/90 backdrop-blur-xl border border-slate-700 p-8 rounded-3xl shadow-2xl transform rotate-y-[-12deg] rotate-x-[5deg] hover:rotate-0 transition-transform duration-700 ease-out"
				>
					<div class="flex justify-between items-center mb-8">
						<div>
							<h3 class="text-2xl font-bold text-white">Swing Alert 🚀</h3>
							<p class="text-emerald-500 text-sm font-bold">Confirmed Breakout Setup</p>
						</div>
						<div
							class="bg-slate-950 px-3 py-1 rounded-lg border border-slate-800 text-xs font-mono text-slate-400"
						>
							Received: 10:30 AM
						</div>
					</div>

					<div class="space-y-6">
						<div class="bg-slate-950 p-4 rounded-xl border-l-4 border-emerald-500">
							<div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Signal</div>
							<div class="text-lg font-bold text-white flex items-center gap-2">
								<span class="bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded text-sm"
									>BUY</span
								>
								NVDA 480 CALLS
								<span class="text-slate-500 text-sm font-normal ml-auto">Exp: Next Week</span>
							</div>
						</div>

						<div class="grid grid-cols-2 gap-4">
							<div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
								<div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Entry Zone</div>
								<div class="text-xl font-mono font-bold text-white">$5.50 - $6.00</div>
							</div>
							<div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
								<div class="text-xs text-slate-500 uppercase tracking-wider mb-1">Target 1</div>
								<div class="text-xl font-mono font-bold text-emerald-400">$8.50+</div>
							</div>
						</div>

						<div class="bg-slate-950 p-4 rounded-xl border border-red-500/30">
							<div class="flex justify-between items-center">
								<div class="text-xs text-slate-500 uppercase tracking-wider">
									Invalidation (Hard Stop)
								</div>
								<div class="text-red-400 font-mono font-bold">$4.20 (Close Below)</div>
							</div>
						</div>
					</div>

					<div
						class="absolute -right-6 -bottom-6 bg-emerald-500 text-slate-950 p-4 rounded-2xl shadow-xl shadow-emerald-500/20 animate-bounce"
					>
						<div class="text-xs font-bold opacity-80 uppercase">Potential Return</div>
						<div class="text-2xl font-extrabold">+45%</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="bg-slate-900 border-y border-slate-800 relative z-20">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
			<dl class="grid grid-cols-2 md:grid-cols-4 gap-8">
				<div class="text-center">
					<dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">
						Historical Win Rate
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-emerald-500">82%</dd>
				</div>
				<div class="text-center">
					<dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">
						Avg Hold Time
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-white">
						3-7<span class="text-lg text-slate-500 font-normal ml-1">days</span>
					</dd>
				</div>
				<div class="text-center">
					<dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">
						Risk/Reward
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-indigo-400">4:1</dd>
				</div>
				<div class="text-center">
					<dt class="text-slate-500 font-medium text-xs uppercase tracking-wider mb-2">
						Alerts Per Week
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-blue-400">2-4</dd>
				</div>
			</dl>
		</div>
	</section>

	<section class="py-24 bg-slate-950 relative overflow-hidden">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-white mb-6">
					Choose Your Trading Lifestyle
				</h2>
				<p use:reveal={{ delay: 100 }} class="text-xl text-slate-400 max-w-2xl mx-auto">
					Most traders burn out trying to scalp 1-minute candles against high-frequency algorithms.
					We play the bigger timeframe where institutions move money.
				</p>
			</div>

			<div class="grid md:grid-cols-2 gap-8 max-w-5xl mx-auto">
				<div
					use:reveal={{ delay: 0 }}
					class="bg-slate-900/50 border border-slate-800 rounded-3xl p-10 opacity-70 hover:opacity-100 transition-opacity"
				>
					<div class="flex items-center gap-4 mb-6">
						<div
							class="w-12 h-12 rounded-full bg-slate-800 flex items-center justify-center text-2xl"
						>
							😰
						</div>
						<h3 class="text-2xl font-bold text-slate-400">The "Day Scalper"</h3>
					</div>
					<ul class="space-y-4 text-slate-500">
						<li class="flex items-center gap-3">
							<span class="text-red-500 font-bold">✕</span> Glued to screen 6+ hours/day
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-500 font-bold">✕</span> High stress, high cortisol spikes
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-500 font-bold">✕</span> Intense competition with HFT algos
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-500 font-bold">✕</span> "Did I miss the move?" anxiety
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-500 font-bold">✕</span> Often overtrades and triggers PDT
						</li>
					</ul>
				</div>

				<div
					use:reveal={{ delay: 150 }}
					class="bg-slate-900 border-2 border-emerald-500 rounded-3xl p-10 shadow-2xl shadow-emerald-500/10 relative overflow-hidden"
				>
					<div
						class="absolute top-0 right-0 bg-emerald-500 text-slate-900 text-xs font-bold px-3 py-1 rounded-bl-xl"
					>
						RECOMMENDED
					</div>
					<div class="flex items-center gap-4 mb-6">
						<div
							class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center text-2xl"
						>
							🧘‍♂️
						</div>
						<h3 class="text-2xl font-bold text-white">The "Swing Trader"</h3>
					</div>
					<ul class="space-y-4 text-slate-200 font-medium">
						<li class="flex items-center gap-3">
							<svg
								class="w-5 h-5 text-emerald-500"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Check charts once or twice a day
						</li>
						<li class="flex items-center gap-3">
							<svg
								class="w-5 h-5 text-emerald-500"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Calm, calculated decisions based on data
						</li>
						<li class="flex items-center gap-3">
							<svg
								class="w-5 h-5 text-emerald-500"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Catch the meat of the move (20% to 100%+)
						</li>
						<li class="flex items-center gap-3">
							<svg
								class="w-5 h-5 text-emerald-500"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Keep your day job and compounding wealth
						</li>
						<li class="flex items-center gap-3">
							<svg
								class="w-5 h-5 text-emerald-500"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/></svg
							>
							Generous entry windows (no rush)
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<section id="methodology" class="py-24 bg-slate-900 border-t border-slate-800">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="mb-16 md:text-center max-w-3xl mx-auto">
				<span class="text-emerald-500 font-bold uppercase tracking-wider text-sm">The Strategy</span
				>
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-white mt-2 mb-6">
					How We Find the "Whale's Wake"
				</h2>
				<p class="text-slate-400 text-lg">
					We don't guess. We track the flow of money. Institutional investors leave footprints
					called "Dark Pool Prints" and "Unusual Options Activity." We follow them.
				</p>
			</div>

			<div class="grid md:grid-cols-3 gap-10">
				<article
					use:reveal={{ delay: 0 }}
					class="group bg-slate-950 p-8 rounded-2xl border border-slate-800 hover:border-emerald-500/30 transition-colors"
				>
					<div
						class="w-14 h-14 bg-emerald-500/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-emerald-500/20"
					>
						<svg
							class="w-7 h-7 text-emerald-500"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-white mb-3">1. Dark Pool Analysis</h3>
					<p class="text-slate-400 leading-relaxed">
						Institutions trade billions off-exchange to hide their intent. We monitor these "Dark
						Pool" block trades to see where Smart Money is accumulating positions *before* the
						breakout happens.
					</p>
				</article>

				<article
					use:reveal={{ delay: 100 }}
					class="group bg-slate-950 p-8 rounded-2xl border border-slate-800 hover:border-blue-500/30 transition-colors"
				>
					<div
						class="w-14 h-14 bg-blue-500/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-blue-500/20"
					>
						<svg class="w-7 h-7 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
							/><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-white mb-3">2. Technical Confirmation</h3>
					<p class="text-slate-400 leading-relaxed">
						Data isn't enough; timing is everything. We layer technical analysis (Supply/Demand
						Zones, Key Moving Averages, and Volume Profiles) to pinpoint the exact moment momentum
						shifts.
					</p>
				</article>

				<article
					use:reveal={{ delay: 200 }}
					class="group bg-slate-950 p-8 rounded-2xl border border-slate-800 hover:border-indigo-500/30 transition-colors"
				>
					<div
						class="w-14 h-14 bg-indigo-500/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-indigo-500/20"
					>
						<svg
							class="w-7 h-7 text-indigo-500"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-white mb-3">3. Risk-First Execution</h3>
					<p class="text-slate-400 leading-relaxed">
						We hate losing money. Every alert comes with a predefined "Hard Stop" level. We
						calculate position size so that even if a trade fails, it only scratches the paint, but
						when it hits, it pays for the month.
					</p>
				</article>
			</div>

			<div
				use:reveal={{ delay: 300 }}
				class="mt-16 bg-slate-950 p-6 md:p-8 rounded-2xl border border-slate-700/50 flex flex-col md:flex-row items-center gap-6 justify-between"
			>
				<div>
					<h4 class="text-lg font-bold text-white mb-2">How You Receive Alerts</h4>
					<p class="text-slate-400 text-sm">
						We ensure you never miss a signal. Notifications are instant and redundant.
					</p>
				</div>
				<div class="flex gap-4">
					<div
						class="flex items-center gap-2 bg-slate-900 px-4 py-2 rounded-lg border border-slate-800"
					>
						<span class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse"></span>
						<span class="text-sm font-bold text-slate-300">Discord</span>
					</div>
					<div
						class="flex items-center gap-2 bg-slate-900 px-4 py-2 rounded-lg border border-slate-800"
					>
						<span class="w-2 h-2 bg-blue-500 rounded-full animate-pulse"></span>
						<span class="text-sm font-bold text-slate-300">SMS Text</span>
					</div>
					<div
						class="flex items-center gap-2 bg-slate-900 px-4 py-2 rounded-lg border border-slate-800"
					>
						<span class="w-2 h-2 bg-indigo-500 rounded-full animate-pulse"></span>
						<span class="text-sm font-bold text-slate-300">Email</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-slate-950">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="mb-10 flex flex-col md:flex-row justify-between items-end gap-4">
				<div>
					<h2 class="text-3xl font-heading font-bold text-white mb-2">Recent Swing Performance</h2>
					<p class="text-slate-500">Real trades. Real timestamps. Transparent results.</p>
				</div>
				<div class="text-right hidden md:block">
					<span
						class="text-xs font-mono text-emerald-500 bg-emerald-500/10 px-3 py-1 rounded border border-emerald-500/20"
						>Live Database Updated: Today</span
					>
				</div>
			</div>

			<div class="bg-slate-900 rounded-2xl border border-slate-800 overflow-hidden shadow-xl">
				<table class="w-full text-left border-collapse">
					<thead>
						<tr
							class="border-b border-slate-800 text-xs font-bold uppercase text-slate-500 tracking-wider"
						>
							<th class="p-4 md:p-5">Ticker</th>
							<th class="p-4 md:p-5">Direction</th>
							<th class="p-4 md:p-5">Time Held</th>
							<th class="p-4 md:p-5 text-right">Net Return</th>
							<th class="p-4 md:p-5 hidden md:table-cell text-right">Catalyst / Note</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-slate-800 font-mono text-sm">
						<tr class="hover:bg-white/5 transition-colors group">
							<td class="p-4 md:p-5 font-bold text-white flex items-center gap-2">
								NVDA
								<span class="hidden group-hover:inline text-[10px] text-slate-500 font-normal"
									>NVIDIA Corp</span
								>
							</td>
							<td class="p-4 md:p-5 text-emerald-400 font-bold">CALLS (Long)</td>
							<td class="p-4 md:p-5 text-slate-400">5 Days</td>
							<td class="p-4 md:p-5 text-right text-emerald-400 font-bold bg-emerald-500/5"
								>+125%</td
							>
							<td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs"
								>Breakout of $480 resistance level.</td
							>
						</tr>
						<tr class="hover:bg-white/5 transition-colors">
							<td class="p-4 md:p-5 font-bold text-white">AMD</td>
							<td class="p-4 md:p-5 text-emerald-400 font-bold">CALLS (Long)</td>
							<td class="p-4 md:p-5 text-slate-400">3 Days</td>
							<td class="p-4 md:p-5 text-right text-emerald-400 font-bold bg-emerald-500/5">+45%</td
							>
							<td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs"
								>Sector rotation & AI strength.</td
							>
						</tr>
						<tr class="hover:bg-white/5 transition-colors bg-red-500/5">
							<td class="p-4 md:p-5 font-bold text-white">TSLA</td>
							<td class="p-4 md:p-5 text-red-400 font-bold">PUTS (Short)</td>
							<td class="p-4 md:p-5 text-slate-400">1 Day</td>
							<td class="p-4 md:p-5 text-right text-red-400 font-bold">-15%</td>
							<td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs"
								>Reversal against trend. Hard stop hit.</td
							>
						</tr>
						<tr class="hover:bg-white/5 transition-colors">
							<td class="p-4 md:p-5 font-bold text-white">META</td>
							<td class="p-4 md:p-5 text-emerald-400 font-bold">CALLS (Long)</td>
							<td class="p-4 md:p-5 text-slate-400">7 Days</td>
							<td class="p-4 md:p-5 text-right text-emerald-400 font-bold bg-emerald-500/5">+82%</td
							>
							<td class="p-4 md:p-5 hidden md:table-cell text-right text-slate-500 text-xs"
								>Earnings run-up swing strategy.</td
							>
						</tr>
					</tbody>
				</table>
			</div>
			<p class="text-center text-xs text-slate-600 mt-4 italic">
				*Results are illustrative of alert performance. Past performance does not guarantee future
				results.
			</p>
		</div>
	</section>

	<section class="py-20 bg-slate-900 border-t border-slate-800">
		<div class="max-w-4xl mx-auto px-4 text-center">
			<h2 class="text-2xl md:text-3xl font-heading font-bold text-white mb-8">
				Who Needs This Service?
			</h2>
			<div class="grid md:grid-cols-2 gap-8 text-left">
				<div class="flex gap-4">
					<div
						class="w-10 h-10 rounded-full bg-emerald-500/10 flex-shrink-0 flex items-center justify-center text-emerald-500 font-bold"
					>
						1
					</div>
					<div>
						<h4 class="text-white font-bold mb-1">Professionals with Day Jobs</h4>
						<p class="text-slate-400 text-sm leading-relaxed">
							You can't watch the screen from 9:30 to 4:00. You need a strategy that lets you enter
							trades on your lunch break or from your phone without panic.
						</p>
					</div>
				</div>
				<div class="flex gap-4">
					<div
						class="w-10 h-10 rounded-full bg-emerald-500/10 flex-shrink-0 flex items-center justify-center text-emerald-500 font-bold"
					>
						2
					</div>
					<div>
						<h4 class="text-white font-bold mb-1">Small Account Builders</h4>
						<p class="text-slate-400 text-sm leading-relaxed">
							You want to grow a $2k - $10k account aggressively but safely, avoiding the "PDT Rule"
							that restricts day traders.
						</p>
					</div>
				</div>
				<div class="flex gap-4">
					<div
						class="w-10 h-10 rounded-full bg-emerald-500/10 flex-shrink-0 flex items-center justify-center text-emerald-500 font-bold"
					>
						3
					</div>
					<div>
						<h4 class="text-white font-bold mb-1">Failed Day Traders</h4>
						<p class="text-slate-400 text-sm leading-relaxed">
							You've tried scalping and lost money to churn and fees. You're ready for a calmer,
							more statistical approach to the markets.
						</p>
					</div>
				</div>
				<div class="flex gap-4">
					<div
						class="w-10 h-10 rounded-full bg-emerald-500/10 flex-shrink-0 flex items-center justify-center text-emerald-500 font-bold"
					>
						4
					</div>
					<div>
						<h4 class="text-white font-bold mb-1">Options Buyers</h4>
						<p class="text-slate-400 text-sm leading-relaxed">
							You understand the power of leverage. You want to turn a 5% stock move into a 50%
							option gain using safe, defined-risk structures.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="py-24 bg-slate-900 border-t border-slate-800">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-white mb-6">Simple Pricing</h2>
				<p class="text-xl text-slate-400 max-w-3xl mx-auto">
					Pay for the subscription with your first successful swing trade.
				</p>
			</div>

			<div class="flex justify-center mb-16">
				<div
					class="bg-rtp-surface bg-slate-950 p-1.5 rounded-xl border border-slate-700/50 inline-flex relative"
				>
					<button
						type="button"
						onclick={() => (selectedPlan = 'monthly')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'monthly'
							? 'text-white'
							: 'text-slate-400 hover:text-white'}">Monthly</button
					>
					<button
						type="button"
						onclick={() => (selectedPlan = 'quarterly')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'quarterly'
							? 'text-white'
							: 'text-slate-400 hover:text-white'}">Quarterly</button
					>
					<button
						type="button"
						onclick={() => (selectedPlan = 'annual')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'annual'
							? 'text-white'
							: 'text-slate-400 hover:text-white'}">Annual</button
					>

					<div
						class="absolute top-1.5 bottom-1.5 bg-emerald-600 rounded-lg shadow-md transition-all duration-300 ease-out"
						style="left: calc({togglePosition} * 33.33% + 0.375rem); width: calc(33.33% - 0.5rem);"
					></div>
				</div>
			</div>

			<div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto items-center">
				<div
					class="bg-slate-900 p-8 rounded-2xl border transition-all {selectedPlan === 'monthly'
						? 'border-emerald-500 opacity-100 scale-105 shadow-xl shadow-emerald-500/10'
						: 'border-slate-800 opacity-70 hover:opacity-90'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Monthly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">${pricing.monthly.price}</span>
						<span class="text-slate-500">{pricing.monthly.period}</span>
					</div>
					<div class="text-xs font-mono text-slate-500 bg-slate-950 p-2 rounded mb-6 text-center">
						{pricing.monthly.tagline}
					</div>
					<ul class="space-y-4 mb-8 text-sm text-slate-400">
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span> 2-4 Premium Swings / Week
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span> Instant SMS & Email Alerts
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span> Private Discord Community
						</li>
						<li class="flex gap-3"><span class="text-emerald-500">✓</span> Entry & Exit Zones</li>
					</ul>
					<a
						href={pricing.monthly.link}
						class="block w-full py-3 bg-slate-950 border border-slate-800 text-white font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
						>{pricing.monthly.btnText}</a
					>
				</div>

				<div
					class="bg-slate-950 p-10 rounded-3xl border-2 shadow-2xl transform relative z-10 transition-all {selectedPlan ===
					'quarterly'
						? 'border-emerald-500 shadow-emerald-500/20 md:scale-110 opacity-100'
						: 'border-slate-800 shadow-slate-800/10 md:scale-100 opacity-70 hover:opacity-90'}"
				>
					<div
						class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-emerald-500 text-slate-900 px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide"
					>
						Most Popular
					</div>
					<h3 class="text-2xl font-bold text-white mb-4">Quarterly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-5xl font-extrabold text-white">${pricing.quarterly.price}</span>
						<span class="text-slate-500">{pricing.quarterly.period}</span>
					</div>
					<div
						class="text-xs font-mono text-emerald-400 bg-emerald-500/10 p-2 rounded mb-6 text-center border border-emerald-500/30"
					>
						{pricing.quarterly.tagline}
					</div>
					<ul class="space-y-4 mb-8 text-sm text-white">
						<li class="flex gap-3">
							<span class="text-emerald-500 font-bold">✓</span>
							<span class="font-bold">Priority Support</span>
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500 font-bold">✓</span> 2-4 Premium Swings / Week
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500 font-bold">✓</span> Instant SMS & Email Alerts
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500 font-bold">✓</span> Private Discord Community
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500 font-bold">✓</span> Full Trade Management
						</li>
					</ul>
					<a
						href={pricing.quarterly.link}
						class="block w-full py-4 bg-emerald-500 text-slate-900 font-bold rounded-xl text-center hover:bg-emerald-400 transition-colors shadow-lg"
						>{pricing.quarterly.btnText}</a
					>
				</div>

				<div
					class="bg-slate-900 p-8 rounded-2xl border transition-all {selectedPlan === 'annual'
						? 'border-emerald-500 opacity-100 scale-105'
						: 'border-slate-800 opacity-70 hover:opacity-90'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Annual</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">${pricing.annual.price}</span>
						<span class="text-slate-500">{pricing.annual.period}</span>
					</div>
					<div class="text-xs font-mono text-emerald-400 bg-slate-950 p-2 rounded mb-6 text-center">
						{pricing.annual.tagline}
					</div>
					<ul class="space-y-4 mb-8 text-sm text-slate-400">
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span>
							<span class="font-bold">Bonus: Strategy Video Library</span>
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span> 2-4 Premium Swings / Week
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span> Instant SMS & Email Alerts
						</li>
						<li class="flex gap-3">
							<span class="text-emerald-500">✓</span> Private Discord Community
						</li>
					</ul>
					<a
						href={pricing.annual.link}
						class="block w-full py-3 bg-slate-950 border border-emerald-500 text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-slate-900 transition-colors"
						>{pricing.annual.btnText}</a
					>
				</div>
			</div>
			<div class="mt-12 text-center">
				<p class="text-slate-500 text-sm">Secure checkout powered by Stripe. Cancel anytime.</p>
			</div>

			<p class="text-center text-slate-500 text-sm mt-6 flex items-center justify-center gap-2">
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
					/></svg
				>
				30-Day Money Back Guarantee.
			</p>
		</div>
	</section>

	<section class="py-20 bg-slate-950">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center text-white mb-4">
				Frequently Asked Questions
			</h2>
			<p class="text-slate-500 text-center mb-12">
				Everything you need to know about the service, capital requirements, and strategy.
			</p>

			<div class="space-y-4">
				{#each faqData as item, i}
					<div class="border border-slate-800 rounded-xl bg-slate-900 overflow-hidden">
						<button
							type="button"
							class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-slate-200"
							onclick={() => toggleFaq(i)}
							aria-expanded={openFaq === i}
						>
							<span class="pr-8">{item.q}</span>
							<svg
								class="w-5 h-5 text-slate-500 transform transition-transform duration-300 flex-shrink-0 {openFaq ===
								i
									? 'rotate-180'
									: ''}"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								><path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								/></svg
							>
						</button>
						{#if openFaq === i}
							<div
								transition:slide={{ duration: 300, easing: cubicOut }}
								class="px-6 pb-6 text-slate-400 text-sm leading-relaxed border-t border-slate-800/50 pt-4"
							>
								{item.a}
							</div>
						{/if}
					</div>
				{/each}
			</div>

			<div class="text-center mt-12">
				<p class="text-slate-500">Still have questions?</p>
				<a
					href="/contact"
					class="text-emerald-500 hover:text-emerald-400 font-bold mt-2 inline-block border-b border-emerald-500/30 pb-0.5"
					>Contact Our Support Team</a
				>
			</div>
		</div>
	</section>

	<section
		class="py-24 bg-gradient-to-br from-emerald-600 to-teal-800 text-white relative overflow-hidden"
	>
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
			<h2 class="text-4xl md:text-6xl font-heading font-extrabold mb-6 tracking-tight">
				Stop Overtrading. Start Swinging.
			</h2>
			<p class="text-xl text-emerald-100 mb-10 max-w-2xl mx-auto">
				Join the trading room that values your time as much as your capital. Instant alerts,
				verified results, and a community of winners.
			</p>
			<a
				href="#pricing"
				class="inline-block bg-white text-emerald-800 px-10 py-5 rounded-xl font-bold text-lg hover:bg-emerald-50 transition-all shadow-2xl hover:-translate-y-1"
			>
				Get Instant Access
			</a>
			<p class="mt-6 text-sm text-emerald-100/70 flex justify-center items-center gap-4">
				<span>Secure Checkout</span>
				<span class="w-1 h-1 bg-emerald-300 rounded-full"></span>
				<span>Cancel Anytime</span>
				<span class="w-1 h-1 bg-emerald-300 rounded-full"></span>
				<span>30-Day Guarantee</span>
			</p>
		</div>
	</section>

	<footer class="bg-slate-950 py-12 border-t border-slate-900">
		<div
			class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-slate-600 leading-relaxed"
		>
			<p class="mb-4 font-bold uppercase text-slate-500">Risk Disclosure</p>
			<p class="max-w-4xl mx-auto mb-6">
				Trading in financial markets involves a high degree of risk and may not be suitable for all
				investors. You could lose some or all of your initial investment; do not invest money that
				you cannot afford to lose. Past performance is not indicative of future results. Revolution
				Trading Pros is an educational platform and does not provide personalized financial advice.
			</p>
			<nav class="flex justify-center gap-6 mb-8 text-slate-500">
				<a href="/terms" class="hover:text-slate-300">Terms of Service</a>
				<a href="/privacy" class="hover:text-slate-300">Privacy Policy</a>
				<a href="/disclaimer" class="hover:text-slate-300">Full Disclaimer</a>
			</nav>
			<p class="opacity-50">
				&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
			</p>
		</div>
	</footer>
</main>
