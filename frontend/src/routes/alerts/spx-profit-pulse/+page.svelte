<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * SPX Profit Pulse - Marketing Landing Page
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Premium 0DTE SPX options alert service landing page with:
	 *   - Hero section with value proposition
	 *   - Two-column layout with performance sidebar
	 *   - Interactive pricing plans
	 *   - FAQ accordion
	 *   - SEO-optimized schema markup
	 *
	 * @version 3.0.0
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 * @author Revolution Trading Pros
	 * @license Proprietary
	 *
	 * @standards Apple Principal Engineer ICT Level 11
	 * @syntax Svelte 5 Runes (January 2026)
	 */

	// ═══════════════════════════════════════════════════════════════════════════════
	// IMPORTS - Organized by category
	// ═══════════════════════════════════════════════════════════════════════════════
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// ═══════════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS - Strict TypeScript interfaces
	// ═══════════════════════════════════════════════════════════════════════════════

	/** Available pricing plan options */
	type PricingPlan = 'monthly' | 'quarterly' | 'annual';

	/** FAQ item structure */
	interface FAQItem {
		readonly q: string;
		readonly a: string;
	}

	/** Sidebar performance statistics */
	interface PerformanceStats {
		readonly total: string;
		readonly winRate: number;
		readonly active: number;
		readonly closed: number;
	}

	/** Latest update video item */
	interface UpdateItem {
		readonly title: string;
		readonly duration: string;
		readonly date: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE - Svelte 5 $state runes
	// ═══════════════════════════════════════════════════════════════════════════════

	/** Currently selected pricing plan */
	let selectedPlan = $state<PricingPlan>('quarterly');

	/** Currently expanded FAQ index (null = all collapsed) */
	let openFaq = $state<number | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Computed values using $derived
	// ═══════════════════════════════════════════════════════════════════════════════

	/** Check if any FAQ is currently open */
	const hasFaqOpen = $derived(openFaq !== null);

	/** Get pricing slider position based on selected plan */
	const pricingSliderPosition = $derived(
		selectedPlan === 'monthly'
			? '0.375rem'
			: selectedPlan === 'quarterly'
				? 'calc(33.33% + 0.2rem)'
				: 'calc(66.66% + 0.1rem)'
	);

	// ═══════════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS - Pure functions for user interactions
	// ═══════════════════════════════════════════════════════════════════════════════

	/**
	 * Toggle FAQ accordion item
	 * @param index - FAQ item index to toggle
	 */
	const toggleFaq = (index: number): void => {
		openFaq = openFaq === index ? null : index;
	};

	/**
	 * Handle pricing plan selection
	 * @param plan - Selected plan type
	 */
	const selectPlan = (plan: PricingPlan): void => {
		selectedPlan = plan;
	};

	// ═══════════════════════════════════════════════════════════════════════════════
	// LIFECYCLE - GSAP ScrollTrigger Animations (Svelte 5 SSR-safe)
	// ═══════════════════════════════════════════════════════════════════════════════

	/**
	 * Initialize scroll-triggered animations using GSAP
	 * - Respects prefers-reduced-motion for accessibility
	 * - Uses gsap.context() for proper cleanup on unmount
	 * - Lazy-loads GSAP only on client-side
	 */
	onMount(() => {
		if (!browser) return;

		let ctx: ReturnType<typeof import('gsap').gsap.context> | null = null;

		const initAnimations = async (): Promise<void> => {
			const { gsap } = await import('gsap');
			const { ScrollTrigger } = await import('gsap/ScrollTrigger');
			gsap.registerPlugin(ScrollTrigger);

			// Respect user's motion preferences (WCAG 2.1 compliance)
			const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

			if (prefersReducedMotion) {
				gsap.set('[data-gsap]', { opacity: 1, y: 0 });
				return;
			}

			// Use gsap.context() for scoped cleanup - prevents global ScrollTrigger destruction
			ctx = gsap.context(() => {
				// Only set initial hidden state for elements NOT yet in viewport
				const elements = document.querySelectorAll<HTMLElement>('[data-gsap]');
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
		};

		initAnimations();

		// Cleanup function - revert all GSAP animations
		return () => ctx?.revert();
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// STATIC DATA - Immutable constants (as const for type safety)
	// ═══════════════════════════════════════════════════════════════════════════════

	/** Performance statistics for sidebar display */
	const PERFORMANCE_STATS: PerformanceStats = {
		total: '+$12,450',
		winRate: 87,
		active: 24,
		closed: 18
	} as const;

	/** Latest updates for sidebar */
	const LATEST_UPDATES: readonly UpdateItem[] = [
		{ title: 'SPX Entry Alert', duration: '2:15', date: 'Today' },
		{ title: 'Market Update', duration: '5:30', date: 'Yesterday' },
		{ title: 'Exit Strategy', duration: '3:45', date: '2 days ago' }
	] as const;

	/** Comprehensive FAQ data for SEO and user education */
	const faqList: readonly FAQItem[] = [
		{
			q: 'What is SPX 0DTE and why do you trade it?',
			a: "SPX 0DTE refers to 'Zero Days to Expiration' options on the S&P 500 index. These contracts expire at 4:00 PM EST on the same day they are traded. We trade them because they offer the fastest potential returns (Gamma risk) and zero overnight risk. You are 100% in cash every single night."
		},
		{
			q: 'How fast are the alerts? Is latency an issue?',
			a: 'Speed is everything in 0DTE. Our alerts are sent instantly via direct SMS text message and Discord webhooks. The average latency is under 5 seconds. We also provide "Warning" alerts (e.g., "Watching 4550 Calls") so you can load the contract before we trigger the official entry.'
		},
		{
			q: 'What is the "SPX Tax Advantage" (Section 1256)?',
			a: 'This is a huge benefit over trading SPY or individual stocks. SPX options fall under Section 1256 of the tax code, meaning gains are taxed at a blended rate: 60% Long Term Capital Gains (lower rate) and 40% Short Term, regardless of how long you hold the trade. Consult your CPA, but for most traders, this results in significant tax savings.'
		},
		{
			q: 'What account size do I need?',
			a: 'SPX options are large contracts (10x the size of SPY). Premiums typically range from $2.00 to $10.00 ($200-$1,000 per contract). We recommend a starting account of at least $2,000 to manage risk properly and allow for position sizing (scaling in/out).'
		},
		{
			q: 'Does this trigger the PDT (Pattern Day Trading) rule?',
			a: 'Yes, if you have a margin account under $25,000. However, many of our members use "Cash Accounts." In a Cash Account, options settle overnight (T+1), meaning you can trade your entire account balance every day without PDT restrictions. SPX is perfect for Cash Accounts due to this T+1 settlement.'
		},
		{
			q: 'Is there "Assignment Risk" at expiration?',
			a: 'No. This is another major advantage of SPX over SPY. SPX is "Cash Settled." This means you can never be forced to buy or sell the underlying shares. If you hold through expiration (which we rarely advise), you simply receive or pay the cash difference. No surprise margin calls for $500,000 worth of stock.'
		},
		{
			q: 'What brokerage do you recommend?',
			a: 'We recommend ThinkOrSwim (Schwab), Interactive Brokers, or TastyTrade for their fast execution and reliable data. Mobile-first brokers like Robinhood and Webull also support SPX, but execution speeds may vary during high volatility.'
		},
		{
			q: 'Do you trade every single day?',
			a: 'The market is open every day, but we only trade when our edge is present. We typically take 1-3 trades per day. If the market is "choppy" or low volume, we sit on our hands. Preservation of capital is our #1 priority.'
		},
		{
			q: 'What is your stop loss strategy?',
			a: 'We use "Hard Stops" based on the premium price (e.g., Stop at 30% loss) or technical invalidate levels on the chart. We do not "hope" trades come back. If a trade hits our stop, we exit immediately. We usually cut losers fast and let winners run.'
		},
		{
			q: 'Can I do this while working a full-time job?',
			a: 'Yes, but you need access to your phone. Our trades are fast—often lasting 15 to 45 minutes. We recommend setting push notifications for our alerts so you can step away for a moment to execute the trade on your mobile app.'
		},
		{
			q: 'What is the win rate?',
			a: 'Our historical win rate is approximately 78%. However, win rate is less important than Risk:Reward. We aim for winners that are 2x or 3x the size of our losers. This mathematical edge ensures profitability even if the win rate dips.'
		},
		{
			q: 'Do you offer a trial?',
			a: 'We offer a highly discounted first month via our Monthly Plan so you can test the service with minimal commitment. We also offer a 30-day money-back guarantee on our Annual plans.'
		},
		{
			q: 'Do you trade Iron Condors or Spreads?',
			a: 'Our primary strategy is "Long Calls" and "Long Puts" (Directional buying). We occasionally signal vertical spreads to cap risk on volatile days, but 90% of our signals are simple directional buys aimed at capturing momentum.'
		},
		{
			q: 'What happens if I miss an entry?',
			a: 'We provide an "Entry Zone" (e.g., Buy between $3.40 and $3.60). If the price has moved significantly past this zone, we advise **not chasing**. There will always be another trade. Chasing entries ruins the risk/reward ratio.'
		}
	];

	// --- SEO SCHEMA (JSON-LD) ---
	const productSchema = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: 'SPX Profit Pulse - 0DTE Options Alerts',
		description:
			'Professional SPX 0DTE options alerts delivered via SMS and Discord. Real-time entries, exits, and risk management. Section 1256 Tax Advantage.',
		brand: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		},
		aggregateRating: {
			'@type': 'AggregateRating',
			ratingValue: '4.8',
			reviewCount: '1042'
		},
		offers: {
			'@type': 'AggregateOffer',
			priceCurrency: 'USD',
			lowPrice: '65',
			highPrice: '97',
			offerCount: '3',
			offers: [
				{
					'@type': 'Offer',
					name: 'Monthly Plan',
					price: '97',
					priceCurrency: 'USD',
					priceSpecification: {
						'@type': 'UnitPriceSpecification',
						price: '97',
						priceCurrency: 'USD',
						referenceQuantity: { '@type': 'QuantitativeValue', value: '1', unitCode: 'MON' }
					}
				},
				{
					'@type': 'Offer',
					name: 'Annual Plan',
					price: '777',
					priceCurrency: 'USD'
				}
			]
		}
	};

	const faqSchema = {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: faqList.map((item) => ({
			'@type': 'Question',
			name: item.q,
			acceptedAnswer: {
				'@type': 'Answer',
				text: item.a
			}
		}))
	};

	const combinedSchema = [productSchema, faqSchema];
</script>

<SEOHead
	title="SPX 0DTE Options Alerts | Daily Income Signals & Gamma Scalping"
	description="Trade SPX 0DTE options with confidence. Real-time SMS alerts, Section 1256 tax benefits, and precise gamma scalping strategies. 78% win rate."
	canonical="/alerts/spx-profit-pulse"
	ogType="product"
	ogImage="/images/og-spx-pulse.jpg"
	ogImageAlt="SPX Profit Pulse - Real-Time 0DTE Options Alerts"
	keywords={[
		'SPX 0DTE alerts',
		'options trading signals',
		'SPX signals',
		'0DTE trading strategy',
		'same day expiration options',
		'SPX options alerts',
		'Section 1256 contracts',
		'options trading service',
		'gamma scalping'
	]}
	schema={combinedSchema}
	schemaType="Product"
	productPrice={97}
	productCurrency="USD"
	productAvailability="in stock"
/>

<div
	class="w-full bg-slate-950 text-slate-200 font-sans selection:bg-indigo-600 selection:text-white"
>
	<!-- Two-Column Layout: Main Content + Sidebar -->
	<div class="main-grid-wrapper bg-slate-950">
		<div class="main-grid max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
			<!-- MAIN CONTENT -->
			<div class="main-content">
				<section class="relative min-h-[90vh] flex items-center overflow-hidden py-24 lg:py-0">
		<div class="absolute inset-0 bg-slate-950 z-0">
			<div
				class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:64px_64px] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"
			></div>
			<div
				class="absolute top-[-20%] right-[-10%] w-[800px] h-[800px] bg-indigo-600/10 rounded-full blur-[120px] animate-pulse"
			></div>
			<div
				class="absolute bottom-[-10%] left-[-10%] w-[600px] h-[600px] bg-indigo-500/10 rounded-full blur-[100px]"
			></div>
		</div>

		<div
			class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-12 items-center overflow-hidden"
		>
			<div class="text-center lg:text-left space-y-8">
				<div
					data-gsap
					class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-slate-900 border border-slate-800/50 shadow-sm backdrop-blur-sm"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
					</span>
					<span class="text-xs font-bold uppercase tracking-widest text-slate-400"
						>Market Active Now</span
					>
				</div>

				<h1
					data-gsap
					class="text-5xl md:text-7xl font-heading font-extrabold tracking-tight leading-[1.1]"
				>
					Conquer Volatility with
					<span
						class="text-transparent bg-clip-text bg-gradient-to-r from-indigo-600 via-blue-500 to-emerald-500"
						>SPX 0DTE</span
					>
				</h1>

				<p
					data-gsap
					class="text-xl text-slate-400 max-w-2xl mx-auto lg:mx-0 leading-relaxed"
				>
					Institutional-grade S&P 500 options alerts delivered instantly via SMS & Discord. Capture
					rapid moves, enjoy Section 1256 tax benefits, and sleep well with zero overnight risk.
				</p>

				<div
					data-gsap
					class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start pt-4"
				>
					<a
						href="#pricing"
						class="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-indigo-600 rounded-xl hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-600 offset-slate-950 shadow-lg hover:shadow-indigo-500/25 hover:-translate-y-1"
					>
						Start Your Trial
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
						href="#performance"
						class="inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-slate-200 transition-all duration-200 bg-slate-900 border border-slate-800 rounded-xl hover:bg-slate-900/80 hover:border-indigo-500/30 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-slate-800 offset-slate-950"
					>
						View Results
					</a>
				</div>

				<div
					data-gsap
					class="flex flex-wrap items-center justify-center lg:justify-start gap-6 pt-4 text-sm text-slate-400/60 font-medium"
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
						<span>Verified 78% Win Rate</span>
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
						<span>&lt; 5s Alert Latency</span>
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
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
						<span>Tax Efficient (1256)</span>
					</div>
				</div>
			</div>

			<div class="relative hidden lg:block perspective-1000 overflow-hidden">
				<div
					class="absolute inset-0 bg-gradient-to-tr from-indigo-600/20 to-transparent rounded-full blur-3xl transform translate-x-10 translate-y-10"
				></div>

				<div
					class="relative bg-slate-900/80 backdrop-blur-xl border border-slate-800/50 rounded-3xl p-6 shadow-2xl transform rotate-y-[-10deg] rotate-x-[5deg] hover:rotate-0 transition-transform duration-700 ease-out"
				>
					<div class="flex items-center justify-between mb-6 border-b border-slate-800/30 pb-4">
						<div class="flex items-center gap-3">
							<div
								class="w-10 h-10 rounded-full bg-gradient-to-br from-indigo-600 to-blue-500 flex items-center justify-center text-white font-bold shadow-inner"
							>
								<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M13 10V3L4 14h7v7l9-11h-7z"
									/></svg
								>
							</div>
							<div>
								<div class="font-bold text-slate-200">SPX Profit Pulse</div>
								<div class="text-xs text-emerald-500">● Live Trading Room</div>
							</div>
						</div>
						<div class="text-xs font-mono text-slate-400 bg-slate-950 px-2 py-1 rounded">
							10:32:45 EST
						</div>
					</div>

					<div class="space-y-4">
						<div class="bg-slate-950/50 p-4 rounded-xl border-l-4 border-emerald-500">
							<div class="flex justify-between text-xs mb-2">
								<span class="text-emerald-500 font-bold uppercase">New Signal</span>
								<span class="text-slate-400">Just now</span>
							</div>
							<div class="text-sm font-mono text-slate-200 mb-1">
								BTO <span class="font-bold text-white">SPX 4580 CALL</span> @ $3.50
							</div>
							<div class="flex gap-4 text-xs text-slate-400">
								<span>🛑 Stop: $2.10</span>
								<span>🎯 Target: $5.00+</span>
							</div>
						</div>
						<div class="bg-slate-950/50 p-4 rounded-xl border-l-4 border-blue-500 opacity-60">
							<div class="flex justify-between text-xs mb-2">
								<span class="text-blue-500 font-bold uppercase">Update</span>
								<span class="text-slate-400">15m ago</span>
							</div>
							<div class="text-sm text-slate-200">
								Approaching VWAP support. Watching for bounce to add to runners.
							</div>
						</div>
					</div>

					<div
						class="absolute -bottom-6 -right-6 bg-white text-slate-950 px-6 py-3 rounded-xl shadow-xl font-bold border-2 border-slate-950 flex items-center gap-2 animate-bounce"
					>
						<span class="text-2xl">🚀</span>
						<div>
							<div class="text-xs uppercase tracking-wide opacity-70">Last Trade</div>
							<div class="text-emerald-600">+85% Profit</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="bg-slate-900 border-y border-slate-800 relative z-20">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
			<div class="grid grid-cols-2 md:grid-cols-4 gap-8">
				<div class="text-center group">
					<div
						class="text-3xl md:text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-br from-emerald-400 to-emerald-600 mb-2 group-hover:scale-110 transition-transform"
					>
						78%
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-slate-400">
						Historical Win Rate
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-5xl font-extrabold text-slate-200 mb-2 group-hover:scale-110 transition-transform"
					>
						&lt;5s
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-slate-400">
						Alert Latency
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-5xl font-extrabold text-slate-200 mb-2 group-hover:scale-110 transition-transform"
					>
						1k+
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-slate-400">
						Active Traders
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-5xl font-extrabold text-slate-200 mb-2 group-hover:scale-110 transition-transform"
					>
						$35M+
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-slate-400">
						Volume Traded
					</div>
				</div>
			</div>
		</div>
	</section>
			</div>

			<!-- ═══════════════════════════════════════════════════════════════════════════
			     SIDEBAR - Performance metrics and resources
			     ═══════════════════════════════════════════════════════════════════════════ -->
			<aside class="sidebar" aria-label="Performance sidebar">
				<!-- 30-Day Performance Card -->
				<div class="sidebar-card" role="region" aria-labelledby="perf-heading">
					<h3 id="perf-heading">30-Day Performance</h3>
					<div class="perf-chart">
						<svg viewBox="0 0 200 100" class="mini-chart" aria-hidden="true">
							<defs>
								<linearGradient id="chartGrad" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" style="stop-color:#22c55e;stop-opacity:0.3" />
									<stop offset="100%" style="stop-color:#22c55e;stop-opacity:0" />
								</linearGradient>
							</defs>
							<path
								d="M0,80 L30,70 L60,55 L90,45 L120,50 L150,30 L180,20 L200,15 L200,100 L0,100 Z"
								fill="url(#chartGrad)"
							/>
							<polyline
								points="0,80 30,70 60,55 90,45 120,50 150,30 180,20 200,15"
								fill="none"
								stroke="#22c55e"
								stroke-width="2.5"
							/>
						</svg>
						<div class="perf-total" aria-label="Total profit">{PERFORMANCE_STATS.total}</div>
					</div>
					<div class="perf-stats" role="list">
						<div role="listitem"><span>{PERFORMANCE_STATS.winRate}%</span> Win Rate</div>
						<div role="listitem"><span>{PERFORMANCE_STATS.active}</span> Active</div>
						<div role="listitem"><span>{PERFORMANCE_STATS.closed}</span> Closed</div>
					</div>
				</div>

				<!-- Resources Card -->
				<nav class="sidebar-card" aria-labelledby="resources-heading">
					<h3 id="resources-heading">Resources</h3>
					<div class="quick-links">
						<a href="/dashboard/spx-profit-pulse/video-library">🎬 Video Library</a>
						<a href="/dashboard/spx-profit-pulse/trade-tracker">📊 Trade Tracker</a>
						<a href="/dashboard/spx-profit-pulse/favorites">⭐ My Favorites</a>
						<a href="/api/export/watchlist?room_slug=spx-profit-pulse&format=csv" download>📥 Export CSV</a>
						<a href="/dashboard/account">⚙️ Alert Settings</a>
					</div>
				</nav>

				<!-- Support Card -->
				<div class="sidebar-card support-card" role="region" aria-labelledby="support-heading">
					<h3 id="support-heading">Need Help?</h3>
					<p>Questions about SPX 0DTE trading?</p>
					<a 
						href="https://intercom.help/simpler-trading/en/" 
						target="_blank" 
						rel="noopener noreferrer"
						class="support-btn"
					>
						Contact Support
					</a>
				</div>

				<!-- Latest Updates Card -->
				<div class="sidebar-card" role="region" aria-labelledby="updates-heading">
					<h3 id="updates-heading">Latest Updates</h3>
					<div class="updates-list" role="list">
						{#each LATEST_UPDATES as update (update.title)}
							<button class="update-item">
								<div class="update-thumbnail">
									<svg viewBox="0 0 24 24" fill="currentColor" class="play-icon">
										<path d="M8 5v14l11-7z" />
									</svg>
									<span class="duration">{update.duration}</span>
								</div>
								<div class="update-info">
									<div class="update-title">{update.title}</div>
									<div class="update-date">{update.date}</div>
								</div>
							</button>
						{/each}
					</div>
				</div>
			</aside>
		</div>
	</div>

	<section class="py-24 bg-slate-950 relative overflow-hidden">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center max-w-3xl mx-auto mb-16">
				<span class="text-indigo-500 font-bold uppercase tracking-wider text-sm">Why SPX?</span>
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-slate-200 mt-2 mb-6">
					The Unfair Advantage
				</h2>
				<p data-gsap class="text-xl text-slate-400">
					Trading SPX isn't just about volatility; it's about structural advantages that put money
					back in your pocket.
				</p>
			</div>

			<div class="grid md:grid-cols-3 gap-8">
				<div
					data-gsap
					class="bg-slate-900 p-8 rounded-2xl border border-slate-800"
				>
					<h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
						<span class="text-emerald-500">💰</span> Tax Efficient (60/40)
					</h3>
					<p class="text-slate-400 leading-relaxed">
						Unlike AAPL or TSLA options, SPX options fall under <strong>Section 1256</strong> of the IRS
						code. This means 60% of your gains are taxed at the lower Long Term Capital Gains rate, even
						if you day trade them.
					</p>
				</div>
				<div
					data-gsap
					class="bg-slate-900 p-8 rounded-2xl border border-slate-800"
				>
					<h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
						<span class="text-blue-500">🛡️</span> No Assignment Risk
					</h3>
					<p class="text-slate-400 leading-relaxed">
						SPX is <strong>Cash Settled</strong>. You never have to worry about buying 100 shares of
						a $4,500 index. At expiration, the difference is simply paid in cash. No margin calls
						for shares you can't afford.
					</p>
				</div>
				<div
					data-gsap
					class="bg-slate-900 p-8 rounded-2xl border border-slate-800"
				>
					<h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
						<span class="text-indigo-500">💧</span> Massive Liquidity
					</h3>
					<p class="text-slate-400 leading-relaxed">
						SPX is one of the most liquid markets in the world. This means tight spreads (difference
						between bid/ask), allowing us to enter and exit large positions instantly without
						slippage.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-slate-900 border-y border-slate-800">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="grid lg:grid-cols-2 gap-16 items-center">
				<div>
					<span class="text-emerald-500 font-bold uppercase tracking-wider text-sm"
						>The Strategy</span
					>
					<h2
						data-gsap
						class="text-3xl md:text-5xl font-heading font-bold text-slate-200 mt-2 mb-6"
					>
						Gamma Scalping Explained
					</h2>
					<p class="text-slate-400 mb-6 text-lg">
						0DTE options have the highest "Gamma" in the market. This means the option price
						accelerates rapidly as the stock moves in your favor.
					</p>
					<p class="text-slate-400 mb-8 leading-relaxed">
						We identify "Gamma Levels" where Market Makers are forced to hedge. When price hits
						these triggers, it causes a chain reaction of buying or selling. We ride that wave.
					</p>
					<ul class="space-y-4">
						<li class="flex items-center gap-3 text-white">
							<span
								class="w-6 h-6 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-500 font-bold text-xs"
								>✓</span
							>
							Entry: Identify key VWAP and Gamma levels.
						</li>
						<li class="flex items-center gap-3 text-white">
							<span
								class="w-6 h-6 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-500 font-bold text-xs"
								>✓</span
							>
							Execution: Sniper entries via Limit Orders.
						</li>
						<li class="flex items-center gap-3 text-white">
							<span
								class="w-6 h-6 rounded-full bg-emerald-500/20 flex items-center justify-center text-emerald-500 font-bold text-xs"
								>✓</span
							>
							Exit: Scale out into strength to lock in profits.
						</li>
					</ul>
				</div>
				<div class="relative bg-slate-950 p-8 rounded-3xl border border-slate-800 shadow-2xl">
					<div class="text-center mt-4 text-xs text-slate-400">
						We buy when momentum overcomes Theta decay.
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="py-32 bg-slate-950 relative overflow-hidden">
		<div class="absolute inset-0 opacity-[0.02] bg-[url('/grid-pattern.svg')]"></div>

		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
			<div class="text-center max-w-3xl mx-auto mb-20">
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-slate-200 mb-6">
					Institutional Edge, Retail Accessible.
				</h2>
				<p data-gsap class="text-xl text-slate-400">
					Most retail traders gamble. We operate like a fund. Data-driven entries, strict sizing,
					and emotionless execution.
				</p>
			</div>

			<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
				<div
					data-gsap
					class="group bg-slate-900 p-8 rounded-2xl border border-slate-800 hover:border-indigo-500/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-indigo-500/10"
				>
					<div
						class="w-14 h-14 rounded-xl bg-indigo-600/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
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
								d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-slate-200 mb-3">Instant SMS & Push</h3>
					<p class="text-slate-400 leading-relaxed text-sm">
						Don't miss a move because you stepped away. Alerts hit your phone via SMS and App
						notification instantly.
					</p>
				</div>

				<div
					data-gsap
					class="group bg-slate-900 p-8 rounded-2xl border border-slate-800 hover:border-indigo-500/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-indigo-500/10"
				>
					<div
						class="w-14 h-14 rounded-xl bg-indigo-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
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
								d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-slate-200 mb-3">Detailed Strategy Logic</h3>
					<p class="text-slate-400 leading-relaxed text-sm">
						We don't just say "Buy". We tell you *why*. Flow, technicals, and gamma levels explained
						in every alert.
					</p>
				</div>

				<div
					data-gsap
					class="group bg-slate-900 p-8 rounded-2xl border border-slate-800 hover:border-emerald-500/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-emerald-500/10"
				>
					<div
						class="w-14 h-14 rounded-xl bg-emerald-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
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
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-slate-200 mb-3">Exact Entry & Exits</h3>
					<p class="text-slate-400 leading-relaxed text-sm">
						No guessing games. You get the specific strike, expiration, and limit price. "Buy SPX
						4600 Call @ $4.20".
					</p>
				</div>

				<div
					data-gsap
					class="group bg-slate-900 p-8 rounded-2xl border border-slate-800 hover:border-blue-500/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-blue-500/10"
				>
					<div
						class="w-14 h-14 rounded-xl bg-blue-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
					>
						<svg class="w-7 h-7 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-slate-200 mb-3">Runner Management</h3>
					<p class="text-slate-400 leading-relaxed text-sm">
						We scale out to lock in profits and leave "runners" for the big moves. Maximize upside,
						minimize stress.
					</p>
				</div>

				<div
					data-gsap
					class="group bg-slate-900 p-8 rounded-2xl border border-slate-800 hover:border-red-400/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-red-400/10"
				>
					<div
						class="w-14 h-14 rounded-xl bg-red-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
					>
						<svg class="w-7 h-7 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-slate-200 mb-3">Hard Stops (No Bagley)</h3>
					<p class="text-slate-400 leading-relaxed text-sm">
						We never hope. Every trade has a predefined invalidation level. We cut losers fast to
						protect your capital.
					</p>
				</div>

				<div
					data-gsap
					class="group bg-slate-900 p-8 rounded-2xl border border-slate-800 hover:border-purple-400/50 transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-purple-400/10"
				>
					<div
						class="w-14 h-14 rounded-xl bg-purple-500/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
					>
						<svg
							class="w-7 h-7 text-purple-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-slate-200 mb-3">Market Context</h3>
					<p class="text-slate-400 leading-relaxed text-sm">
						Receive pre-market plans and mid-day updates. Know when to be aggressive and when to sit
						on your hands.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-slate-900 border-y border-slate-800">
		<div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-slate-200 mb-4">
					Crystal Clear Execution
				</h2>
				<p data-gsap class="text-xl text-slate-400">Follow the lifecycle of a typical trade.</p>
			</div>

			<div class="relative">
				<div
					class="absolute left-8 md:left-1/2 top-0 bottom-0 w-0.5 bg-gradient-to-b from-emerald-500 via-blue-500 to-slate-800"
				></div>

				<div
					data-gsap
					class="relative flex flex-col md:flex-row items-center md:justify-between mb-16 group"
				>
					<div class="md:w-[45%] mb-4 md:mb-0 md:text-right pr-8 order-2 md:order-1">
						<h3 class="text-2xl font-bold text-white mb-2">1. The Setup & Entry</h3>
						<p class="text-slate-400">
							We identify a key gamma level holding. You get the alert instantly with strike, price,
							and risk parameters.
						</p>
					</div>
					<div
						class="absolute left-8 md:left-1/2 -translate-x-1/2 w-10 h-10 rounded-full bg-slate-950 border-4 border-emerald-500 z-10 flex items-center justify-center shadow-[0_0_20px_rgba(16,185,129,0.5)]"
					>
						<span class="text-emerald-500 font-bold">1</span>
					</div>
					<div class="md:w-[45%] pl-16 md:pl-8 order-1 md:order-2 w-full">
						<div class="bg-slate-950 p-6 rounded-xl border-l-4 border-emerald-500 shadow-lg">
							<div class="flex items-center justify-between mb-3">
								<span
									class="bg-emerald-500/20 text-emerald-400 text-[10px] font-bold px-2 py-1 rounded uppercase"
									>Signal</span
								>
								<span class="font-mono text-xs text-slate-400">09:42 AM</span>
							</div>
							<div class="font-mono text-sm">
								<div class="font-bold text-lg text-white">BTO SPX 4580 CALL</div>
								<div class="grid grid-cols-2 gap-y-2 mt-2 text-xs">
									<div class="text-slate-400">
										Entry: <span class="text-white font-bold">$3.50</span>
									</div>
									<div class="text-slate-400">
										Stop: <span class="text-red-400 font-bold">$2.10</span>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>

				<div
					data-gsap
					class="relative flex flex-col md:flex-row items-center md:justify-between mb-16 group"
				>
					<div class="md:w-[45%] pl-16 md:pl-0 md:pr-8 order-1 w-full">
						<div class="bg-slate-950 p-6 rounded-xl border-l-4 border-blue-500 shadow-lg">
							<div class="flex items-center justify-between mb-3">
								<span
									class="bg-blue-500/20 text-blue-400 text-[10px] font-bold px-2 py-1 rounded uppercase"
									>Update</span
								>
								<span class="font-mono text-xs text-slate-400">10:05 AM</span>
							</div>
							<div class="font-mono text-sm">
								<div class="font-bold text-lg text-white">TARGET 1 HIT 🎯</div>
								<p class="text-xs text-slate-400 mt-1">
									Price at $4.50 (+28%). Trim half size. Move stop on runners to Breakeven.
								</p>
							</div>
						</div>
					</div>
					<div
						class="absolute left-8 md:left-1/2 -translate-x-1/2 w-10 h-10 rounded-full bg-slate-950 border-4 border-blue-500 z-10 flex items-center justify-center shadow-[0_0_20px_rgba(59,130,246,0.5)]"
					>
						<span class="text-blue-500 font-bold">2</span>
					</div>
					<div class="md:w-[45%] mb-4 md:mb-0 pl-16 md:pl-8 order-2">
						<h3 class="text-2xl font-bold text-white mb-2">2. Trade Management</h3>
						<p class="text-slate-400">
							We don't leave you hanging. We send real-time updates to trim profits and protect your
							downside as the trade moves.
						</p>
					</div>
				</div>

				<div
					data-gsap
					class="relative flex flex-col md:flex-row items-center md:justify-between group"
				>
					<div class="md:w-[45%] mb-4 md:mb-0 md:text-right pr-8 order-2 md:order-1">
						<h3 class="text-2xl font-bold text-white mb-2">3. Final Exit</h3>
						<p class="text-slate-400">
							We squeeze the move for maximum gain, exiting runners into strength before reversal.
						</p>
					</div>
					<div
						class="absolute left-8 md:left-1/2 -translate-x-1/2 w-10 h-10 rounded-full bg-slate-950 border-4 border-indigo-500 z-10 flex items-center justify-center shadow-[0_0_20px_rgba(99,102,241,0.5)]"
					>
						<span class="text-indigo-500 font-bold">3</span>
					</div>
					<div class="md:w-[45%] pl-16 md:pl-8 order-1 md:order-2 w-full">
						<div class="bg-slate-950 p-6 rounded-xl border-l-4 border-indigo-500 shadow-lg">
							<div class="flex items-center justify-between mb-3">
								<span
									class="bg-indigo-500/20 text-indigo-400 text-[10px] font-bold px-2 py-1 rounded uppercase"
									>Exit</span
								>
								<span class="font-mono text-xs text-slate-400">10:45 AM</span>
							</div>
							<div class="font-mono text-sm">
								<div class="font-bold text-lg text-white">ALL OUT</div>
								<p class="text-xs text-slate-400 mt-1">Sold runners at $7.00.</p>
								<p class="text-emerald-400 font-bold mt-2">Total Profit: +100% ✅</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="performance" class="py-24 bg-slate-950 relative">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="flex flex-col md:flex-row justify-between items-end mb-12 gap-6">
				<div>
					<h2 class="text-3xl md:text-4xl font-heading font-bold text-slate-200 mb-2">
						Recent Performance
					</h2>
					<p class="text-slate-400">Transparency is our currency. Live log of recent calls.</p>
				</div>
				<a
					href="/performance"
					class="text-indigo-500 font-bold hover:text-white transition-colors flex items-center gap-2"
				>
					View Full Ledger <svg
						class="w-4 h-4"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M17 8l4 4m0 0l-4 4m4-4H3"
						/></svg
					>
				</a>
			</div>

			<div class="bg-slate-900 rounded-2xl border border-slate-800 overflow-hidden shadow-xl">
				<div
					class="grid grid-cols-12 bg-slate-950/50 border-b border-slate-800 p-4 text-xs font-bold uppercase text-slate-400 tracking-wider"
				>
					<div class="col-span-3 md:col-span-2">Date</div>
					<div class="col-span-5 md:col-span-4">Ticker / Strike</div>
					<div class="col-span-4 md:col-span-2 text-right">Result</div>
					<div class="hidden md:block md:col-span-4 text-right">Notes</div>
				</div>
				<div class="divide-y divide-slate-800/50 font-mono text-sm">
					<div class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors">
						<div class="col-span-3 md:col-span-2 text-slate-400">Nov 15</div>
						<div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4560 CALL</div>
						<div class="col-span-4 md:col-span-2 text-right text-emerald-400 font-bold">+50%</div>
						<div class="hidden md:block md:col-span-4 text-right text-slate-400 text-xs">
							Held VWAP perfectly.
						</div>
					</div>
					<div class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors">
						<div class="col-span-3 md:col-span-2 text-slate-400">Nov 14</div>
						<div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4575 PUT</div>
						<div class="col-span-4 md:col-span-2 text-right text-emerald-400 font-bold">+50%</div>
						<div class="hidden md:block md:col-span-4 text-right text-slate-400 text-xs">
							Clean breakdown of 4580.
						</div>
					</div>
					<div
						class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors bg-red-500/5"
					>
						<div class="col-span-3 md:col-span-2 text-slate-400">Nov 13</div>
						<div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4590 CALL</div>
						<div class="col-span-4 md:col-span-2 text-right text-red-400 font-bold">-31%</div>
						<div class="hidden md:block md:col-span-4 text-right text-slate-400 text-xs">
							Stopped out. Choppy open.
						</div>
					</div>
					<div class="grid grid-cols-12 p-4 items-center hover:bg-white/5 transition-colors">
						<div class="col-span-3 md:col-span-2 text-slate-400">Nov 12</div>
						<div class="col-span-5 md:col-span-4 font-bold text-white">SPX 4555 PUT</div>
						<div class="col-span-4 md:col-span-2 text-right text-emerald-400 font-bold">+60%</div>
						<div class="hidden md:block md:col-span-4 text-right text-slate-400 text-xs">
							Trend day runner.
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="py-24 bg-slate-900 border-t border-slate-800 overflow-hidden">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-slate-200 mb-4">
					Simple, Flat Pricing
				</h2>
				<p class="text-xl text-slate-400">Pay for the alerts with one good trade.</p>
			</div>

			<div class="flex justify-center mb-16">
				<div 
					class="bg-slate-950 p-1.5 rounded-xl border border-slate-800 inline-flex relative"
					role="tablist"
					aria-label="Pricing plans"
				>
					<button
						onclick={() => selectPlan('monthly')}
						role="tab"
						aria-selected={selectedPlan === 'monthly'}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan === 'monthly' ? 'text-white' : 'text-slate-400 hover:text-white'}"
					>
						Monthly
					</button>
					<button
						onclick={() => selectPlan('quarterly')}
						role="tab"
						aria-selected={selectedPlan === 'quarterly'}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan === 'quarterly' ? 'text-white' : 'text-slate-400 hover:text-white'}"
					>
						Quarterly
					</button>
					<button
						onclick={() => selectPlan('annual')}
						role="tab"
						aria-selected={selectedPlan === 'annual'}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan === 'annual' ? 'text-white' : 'text-slate-400 hover:text-white'}"
					>
						Annual
					</button>

					<!-- Animated slider indicator -->
					<div
						class="absolute top-1.5 bottom-1.5 bg-indigo-600 rounded-lg shadow-md transition-all duration-300 ease-out"
						style="left: {pricingSliderPosition}; width: calc(33.33% - 0.4rem);"
						aria-hidden="true"
					></div>
				</div>
			</div>

			<div class="grid lg:grid-cols-3 gap-8 max-w-6xl mx-auto items-center overflow-visible">
				<div
					class="order-2 lg:order-1 bg-slate-950 p-8 rounded-2xl border transition-all {selectedPlan ===
					'monthly'
						? 'border-indigo-600 opacity-100 scale-105'
						: 'border-slate-800 opacity-70 hover:opacity-90'}"
				>
					<h3 class="text-xl font-bold text-slate-200 mb-2">Monthly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$97</span>
						<span class="text-slate-400">/mo</span>
					</div>
					<p class="text-sm text-slate-400 mb-8 h-10">Perfect for testing the waters.</p>
					<a
						href="/checkout/monthly"
						class="block w-full py-3 px-4 bg-slate-900 border border-slate-800 text-slate-200 font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
					>
						Select Monthly
					</a>
					<div class="mt-8 space-y-4 text-sm text-slate-400">
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-500 flex-shrink-0"
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
							Daily Live Alerts
						</div>
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-500 flex-shrink-0"
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
							Discord Access
						</div>
					</div>
				</div>

				<div
					class="order-1 lg:order-2 bg-slate-950 p-10 rounded-3xl border-2 shadow-2xl relative transform z-10 transition-all {selectedPlan ===
					'quarterly'
						? 'border-indigo-600 shadow-indigo-600/20 lg:scale-110 opacity-100'
						: 'border-slate-800 shadow-slate-800/10 lg:scale-100 opacity-70 hover:opacity-90'}"
				>
					<div
						class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-indigo-600 text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide"
					>
						Most Popular
					</div>
					<h3 class="text-2xl font-bold text-white mb-2">Quarterly</h3>
					<div class="flex items-baseline gap-1 mb-1">
						<span class="text-5xl font-extrabold text-white">$247</span>
						<span class="text-slate-400">/qtr</span>
					</div>
					<p class="text-emerald-400 text-sm font-bold mb-8">Save $45 vs Monthly</p>

					<a
						href="/checkout/quarterly"
						class="block w-full py-4 px-6 bg-indigo-600 text-white font-bold rounded-xl text-center hover:bg-blue-600 transition-colors shadow-lg mb-8"
					>
						Start Quarterly Plan
					</a>

					<div class="space-y-4 text-sm text-white/90">
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-400 flex-shrink-0"
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
							<span class="font-bold">Priority Support</span>
						</div>
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-400 flex-shrink-0"
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
							Instant SMS Alerts
						</div>
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-400 flex-shrink-0"
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
							Discord Community
						</div>
					</div>
				</div>

				<div
					class="order-3 bg-slate-950 p-8 rounded-2xl border transition-all {selectedPlan === 'annual'
						? 'border-emerald-500 opacity-100 scale-105'
						: 'border-slate-800 opacity-70 hover:opacity-90'}"
				>
					<h3 class="text-xl font-bold text-slate-200 mb-2">Annual</h3>
					<div class="flex items-baseline gap-1 mb-1">
						<span class="text-4xl font-bold text-white">$777</span>
						<span class="text-slate-400">/yr</span>
					</div>
					<p class="text-emerald-500 text-sm font-bold mb-8 h-10">Best Value (Save 33%)</p>

					<a
						href="/checkout/annual"
						class="block w-full py-3 px-4 bg-slate-900 border border-emerald-500 text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-white transition-colors"
					>
						Select Annual
					</a>
					<div class="mt-8 space-y-4 text-sm text-slate-400">
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-500 flex-shrink-0"
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
							All Quarterly Features
						</div>
						<div class="flex gap-3">
							<svg
								class="w-5 h-5 text-emerald-500 flex-shrink-0"
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
							2 Months Free
						</div>
					</div>
				</div>
			</div>

			<div class="mt-12 text-center">
				<div class="inline-flex items-center gap-2 text-sm text-slate-400">
					<svg class="w-4 h-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
						/></svg
					>
					Secure 256-bit Encrypted Checkout. Cancel anytime.
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-slate-950">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center mb-4">Frequently Asked Questions</h2>
			<p class="text-center text-slate-400 mb-12">
				Common questions about brokers, capital, and risk management.
			</p>
			<div class="space-y-4">
				{#each faqList as faq, i}
					<div class="border border-slate-800 rounded-xl bg-slate-900 overflow-hidden">
						<button
							class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors"
							onclick={() => toggleFaq(i)}
						>
							<span class="pr-8">{faq.q}</span>
							<svg
								class="w-5 h-5 text-slate-400 transform transition-transform duration-300 flex-shrink-0 {openFaq ===
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
								{faq.a}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="py-24 relative overflow-hidden">
		<div class="absolute inset-0 bg-gradient-to-br from-indigo-600 to-indigo-500 z-0"></div>
		<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10 z-0"></div>

		<div class="relative z-10 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
			<h2 class="text-4xl md:text-6xl font-heading font-extrabold text-white mb-6 tracking-tight">
				Ready to Level Up?
			</h2>
			<p class="text-xl text-blue-100 mb-10 max-w-2xl mx-auto">
				Join the ranks of professional traders capturing daily alpha in the SPX.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<a
					href="#pricing"
					class="bg-white text-indigo-500 px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-2xl hover:-translate-y-1"
				>
					Get Access Now
				</a>
			</div>
			<p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee on Annual Plans</p>
		</div>
	</section>
</div>

<MarketingFooter />

<style>
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * SPX Profit Pulse - Component Styles
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Scoped styles for the SPX Profit Pulse landing page
	 * @version 3.0.0
	 * @standards Apple Principal Engineer ICT Level 11
	 *
	 * CSS Architecture:
	 * - CSS Custom Properties for theming
	 * - Logical properties for RTL support
	 * - Mobile-first responsive design
	 * - BEM-inspired naming convention
	 */

	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES - Design tokens
	   ═══════════════════════════════════════════════════════════════════════════ */
	:root {
		--sidebar-width: 340px;
		--sidebar-gap: 30px;
		--card-radius: 16px;
		--card-padding: 25px;
		--color-primary: #143e59;
		--color-primary-light: #1e5175;
		--color-accent: #0984ae;
		--color-success: #22c55e;
		--color-surface: #fff;
		--color-text: #333;
		--color-text-muted: #666;
		--shadow-card: 0 4px 20px rgba(0, 0, 0, 0.06);
		--shadow-hover: 0 4px 12px rgba(0, 0, 0, 0.15);
		--transition-fast: 0.2s ease;
		--transition-smooth: 0.3s ease-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN GRID LAYOUT - Two Column with Sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */
	.main-grid {
		display: grid;
		grid-template-columns: 1fr var(--sidebar-width);
		gap: var(--sidebar-gap);
		align-items: start;
		max-inline-size: 1400px;
		margin-inline: auto;
	}

	/* Responsive: Stack on tablet and below */
	@media (max-width: 1024px) {
		.main-grid {
			grid-template-columns: 1fr;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Sticky performance panel
	   ═══════════════════════════════════════════════════════════════════════════ */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 20px;
		position: sticky;
		inset-block-start: 20px;
	}

	.sidebar-card {
		background: var(--color-surface);
		border-radius: var(--card-radius);
		padding: var(--card-padding);
		box-shadow: var(--shadow-card);
		text-align: center;
	}

	.sidebar-card h3 {
		font-size: 1rem;
		font-weight: 700;
		margin-block: 0 20px;
		color: var(--color-text);
		font-family: 'Montserrat', system-ui, sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PERFORMANCE CHART - Mini chart visualization
	   ═══════════════════════════════════════════════════════════════════════════ */
	.mini-chart {
		inline-size: 100%;
		block-size: 80px;
	}

	.perf-total {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--color-success);
		margin-block: 15px;
		font-family: 'Montserrat', system-ui, sans-serif;
	}

	.perf-stats {
		display: flex;
		justify-content: center;
		gap: 20px;
		font-size: 0.8125rem;
		color: var(--color-text-muted);
	}

	.perf-stats span {
		font-weight: 700;
		color: var(--color-primary);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   QUICK LINKS - Resource navigation
	   ═══════════════════════════════════════════════════════════════════════════ */
	.quick-links {
		display: flex;
		flex-direction: column;
		gap: 12px;
		text-align: start;
	}

	.quick-links a {
		color: var(--color-primary);
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 600;
		transition: all var(--transition-fast);
		padding-block: 8px;
		padding-inline: 12px;
		border-radius: 8px;
		display: block;
	}

	.quick-links a:hover,
	.quick-links a:focus-visible {
		background: #f8fafc;
		color: var(--color-accent);
		transform: translateX(4px);
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUPPORT CARD - Help section with gradient
	   ═══════════════════════════════════════════════════════════════════════════ */
	.support-card {
		background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-light) 100%);
		color: white;
	}

	.support-card h3 {
		color: white;
	}

	.support-card p {
		color: rgba(255, 255, 255, 0.9);
		font-size: 0.875rem;
		margin-block-end: 15px;
	}

	.support-btn {
		display: inline-block;
		background: white;
		color: var(--color-primary);
		padding-block: 10px;
		padding-inline: 20px;
		border-radius: 8px;
		font-weight: 700;
		font-size: 0.875rem;
		text-decoration: none;
		transition: all var(--transition-fast);
	}

	.support-btn:hover,
	.support-btn:focus-visible {
		background: #f8fafc;
		transform: translateY(-2px);
		box-shadow: var(--shadow-hover);
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   UPDATES LIST - Latest video updates
	   ═══════════════════════════════════════════════════════════════════════════ */
	.updates-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
		text-align: start;
	}

	.update-item {
		display: flex;
		gap: 12px;
		padding: 10px;
		border-radius: 8px;
		text-decoration: none;
		transition: all var(--transition-fast);
		background: transparent;
		border: none;
		inline-size: 100%;
		cursor: pointer;
		text-align: start;
	}

	.update-item:hover,
	.update-item:focus-visible {
		background: #f8fafc;
		outline: none;
	}

	.update-thumbnail {
		position: relative;
		inline-size: 60px;
		block-size: 45px;
		background: var(--color-primary);
		border-radius: 6px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.play-icon {
		inline-size: 20px;
		block-size: 20px;
		color: white;
	}

	.duration {
		position: absolute;
		inset-block-end: 3px;
		inset-inline-end: 3px;
		background: rgba(0, 0, 0, 0.8);
		color: white;
		font-size: 0.625rem;
		padding-block: 2px;
		padding-inline: 4px;
		border-radius: 3px;
		font-family: ui-monospace, monospace;
	}

	.update-info {
		flex: 1;
		min-inline-size: 0;
	}

	.update-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--color-text);
		margin-block-end: 2px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.update-date {
		font-size: 0.6875rem;
		color: var(--color-text-muted);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile-first breakpoints
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.sidebar {
			position: static;
		}
	}

	/* Reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.update-item,
		.quick-links a,
		.support-btn {
			transition: none;
		}
	}
</style>

