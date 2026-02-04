<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';

	// --- Pricing State (Svelte 5 Runes) ---
	let selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');

	// --- FAQ Logic (Svelte 5 Runes) ---
	let openFaq: number | null = $state(null);
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

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

	// --- EXPANDED SEO DATA ---
	// Note: Full expanded FAQ JSON-LD is generated in the dedicated section below
	// but represented here for component passing.

	const productSchema = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: 'Live SPX Day Trading Room',
		description:
			'Professional day trading community specializing in SPX 0DTE options. Features live voice commentary, 1080p screen sharing, and real-time trade alerts.',
		image: 'https://revolution-trading-pros.pages.dev/images/og-live-room.jpg',
		brand: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		},
		aggregateRating: {
			'@type': 'AggregateRating',
			ratingValue: '4.9',
			reviewCount: '127'
		},
		offers: {
			'@type': 'AggregateOffer',
			priceCurrency: 'USD',
			lowPrice: '137',
			highPrice: '197',
			offerCount: '3',
			offers: [
				{
					'@type': 'Offer',
					name: 'Monthly Access',
					price: '197',
					priceCurrency: 'USD',
					availability: 'https://schema.org/InStock',
					priceSpecification: {
						'@type': 'UnitPriceSpecification',
						price: '197',
						priceCurrency: 'USD',
						referenceQuantity: { '@type': 'QuantitativeValue', value: '1', unitCode: 'MON' }
					}
				},
				{
					'@type': 'Offer',
					name: 'Quarterly Access',
					price: '497',
					priceCurrency: 'USD',
					availability: 'https://schema.org/InStock'
				},
				{
					'@type': 'Offer',
					name: 'Annual Access',
					price: '1647',
					priceCurrency: 'USD',
					availability: 'https://schema.org/InStock'
				}
			]
		}
	};

	// Full Expanded FAQ List for UI Rendering
	const faqList = [
		{
			question: 'Do I need to be an expert to join?',
			answer:
				"No. While the trading is fast-paced, we prioritize education. We provide a comprehensive 'New Member' onboarding video series to help you understand our levels, terminology, and platform execution before you take your first trade."
		},
		{
			question: 'What platform is the room hosted on?',
			answer:
				'We host our live room on a private, boosted Discord server. This allows for low-latency Voice Channels (sub-200ms) and crisp 1080p Screen Share, accessible from desktop or mobile.'
		},
		{
			question: 'What specific instruments do you trade?',
			answer:
				'We specialize in SPX (S&P 500 Index) options, specifically 0DTE (Zero Days to Expiration). We occasionally trade SPY, QQQ, and futures (ES/NQ) when setups present high probability, but SPX is our primary focus due to tax benefits and liquidity.'
		},
		{
			question: 'How much capital do I need to start?',
			answer:
				'We recommend a starting account size of at least $2,000 to trade comfortably with proper risk management. However, many members start with less using cash accounts to avoid PDT (Pattern Day Trader) rules.'
		},
		{
			question: 'Do you offer a free trial?',
			answer:
				"We do not offer free trials to maintain the integrity and quality of our community. We focus on dedicated traders who are ready to commit to their education. We do offer a 30-day money-back guarantee if you feel the value wasn't delivered."
		},
		{
			question: 'Is this just copy-trading or signals?',
			answer:
				'This is an educational room first. While we call out every trade we take live (Entry, Stop, Target), the goal is for you to learn the "Why" behind the trade so you can eventually become self-sufficient.'
		},
		{
			question: 'What happens if I miss the live session?',
			answer:
				'All key levels, trade recaps, and educational lessons are posted in text channels. We also record major educational webinars. However, the live voice commentary is exclusive to the live session hours.'
		},
		{
			question: 'Can I trade on my phone?',
			answer:
				"Yes. Discord has an excellent mobile app. You can listen to the voice commentary and see the screen share on your phone while executing trades on your broker's mobile app."
		},
		{
			question: 'What is the win rate of the room?',
			answer:
				'Win rates vary by market conditions. Historically, our strategy targets a 70%+ win rate with a strict 1:2 or 1:3 risk-to-reward ratio. We prioritize risk management over "calling tops and bottoms."'
		},
		{
			question: 'How do I cancel my subscription?',
			answer:
				'You have full control. You can cancel anytime via your user dashboard or the Stripe portal link provided in your welcome email. Access continues until the end of your billing cycle.'
		}
	];

	// Build Schema from the expanded list for SEOHead
	const faqSchema = {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: faqList.map((item) => ({
			'@type': 'Question',
			name: item.question,
			acceptedAnswer: {
				'@type': 'Answer',
				text: item.answer
			}
		}))
	};

	const combinedSchema = [productSchema, faqSchema];
</script>

<SEOHead
	title="Live SPX 0DTE Trading Room | Real-Time Voice & Screen Share"
	description="Join the #1 live options trading room. Watch professional traders execute SPX 0DTE strategies with live voice commentary, 1080p screen sharing, and real-time mentorship."
	canonical="/live-trading-rooms/day-trading"
	ogType="product"
	ogImage="/images/og-live-room.jpg"
	ogImageAlt="Live SPX Day Trading Room - Trade with Professional Traders"
	keywords={[
		'day trading room',
		'live trading discord',
		'spx 0dte strategy',
		'options trading community',
		'live stock market commentary',
		'professional trading signals',
		'learn to trade options',
		'spx gamma levels'
	]}
	schema={combinedSchema}
	schemaType="Product"
	productPrice={197}
	productCurrency="USD"
	productAvailability="in stock"
/>

<div class="w-full bg-rtp-bg text-rtp-text font-sans selection:bg-rtp-primary selection:text-white">
	<section class="relative min-h-[90vh] flex items-center overflow-hidden py-24 lg:py-0">
		<div class="absolute inset-0 bg-rtp-bg z-0 pointer-events-none">
			<div
				class="absolute inset-0 bg-[linear-gradient(rgba(59,130,246,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(59,130,246,0.03)_1px,transparent_1px)] bg-[size:40px_40px] opacity-30"
			></div>
			<div
				class="absolute top-[-10%] right-[-5%] w-[800px] h-[800px] bg-rtp-blue/10 rounded-full blur-[120px] animate-pulse"
			></div>
			<div
				class="absolute bottom-[-10%] left-[-10%] w-[600px] h-[600px] bg-rtp-indigo/10 rounded-full blur-[100px]"
			></div>
		</div>

		<div
			class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-16 items-center"
		>
			<div class="text-center lg:text-left space-y-8">
				<div
					data-gsap
					class="inline-flex items-center gap-3 bg-red-500/10 border border-red-500/20 px-4 py-1.5 rounded-full backdrop-blur-md"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-500 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-red-600"></span>
					</span>
					<span class="text-xs font-bold tracking-widest uppercase text-red-400"
						>Live Trading Active • Pre-Market Prep 08:30 EST</span
					>
				</div>

				<h1
					data-gsap
					class="text-5xl md:text-7xl font-heading font-extrabold leading-tight tracking-tight"
				>
					Never Trade <br />
					<span
						class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-blue via-indigo-400 to-white"
						>Alone Again.</span
					>
				</h1>

				<p data-gsap class="text-xl text-rtp-muted max-w-2xl mx-auto lg:mx-0 leading-relaxed">
					Join the digital trading floor. Watch our screens, hear our professional voice commentary,
					and execute
					<strong>SPX 0DTE trades</strong> alongside 500+ serious traders every single morning.
				</p>

				<div data-gsap class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
					<a
						href="#pricing"
						class="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-rtp-primary rounded-xl hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-rtp-primary offset-rtp-bg shadow-lg hover:shadow-rtp-primary/25 hover:-translate-y-1"
					>
						Join the Room
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
						href="#schedule"
						class="inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-rtp-text transition-all duration-200 bg-rtp-surface border border-rtp-border rounded-xl hover:bg-rtp-surface/80 hover:border-rtp-primary/30"
					>
						View Schedule
					</a>
				</div>

				<div
					data-gsap
					class="flex items-center justify-center lg:justify-start gap-4 text-sm text-rtp-muted pt-4"
				>
					<div class="flex -space-x-3">
						<div
							class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-gradient-to-br from-blue-500 to-blue-600 flex items-center justify-center text-white text-xs font-bold shadow-sm"
						>
							JD
						</div>
						<div
							class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center text-white text-xs font-bold shadow-sm"
						>
							MK
						</div>
						<div
							class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-gradient-to-br from-purple-500 to-purple-600 flex items-center justify-center text-white text-xs font-bold shadow-sm"
						>
							SR
						</div>
						<div
							class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-rtp-surface flex items-center justify-center text-xs font-bold text-white shadow-sm"
						>
							+500
						</div>
					</div>
					<div class="flex flex-col text-xs">
						<span class="text-white font-bold">500+ Traders</span>
						<span>Active in Discord now</span>
					</div>
				</div>
			</div>

			<div class="hidden lg:block relative perspective-1000">
				<div
					class="absolute inset-0 bg-gradient-to-tr from-rtp-primary/20 to-transparent rounded-full blur-3xl transform translate-x-10 translate-y-10"
				></div>

				<div
					class="relative bg-[#1e1e24] border border-rtp-border rounded-xl shadow-2xl overflow-hidden transform rotate-y-[-5deg] hover:rotate-0 transition-transform duration-500 ease-out"
				>
					<div class="bg-[#2b2b36] p-4 border-b border-gray-700 flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="text-gray-400">
								<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M4 6h16M4 12h16M4 18h7"
									/></svg
								>
							</div>
							<div class="font-bold text-white tracking-wide text-sm"># 🔴-live-trading-floor</div>
						</div>
						<div class="flex items-center gap-3">
							<div
								class="flex items-center gap-2 bg-red-500/20 px-3 py-1 rounded text-red-400 text-xs font-bold"
							>
								<span class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span> LIVE VOICE
							</div>
						</div>
					</div>

					<div class="grid grid-cols-3 h-[400px]">
						<div
							class="col-span-2 bg-gray-900 p-4 flex flex-col justify-center items-center border-r border-gray-700 relative"
						>
							<div class="absolute top-4 left-4 text-xs text-gray-500 font-mono">
								SCREEN SHARE: HEAD TRADER
							</div>
							<svg
								class="w-full h-64 text-rtp-primary opacity-80"
								viewBox="0 0 400 200"
								fill="none"
								stroke="currentColor"
							>
								<path
									stroke-width="2"
									d="M0 150 C 50 150, 50 100, 100 100 C 150 100, 150 180, 200 180 C 250 180, 250 40, 300 40 C 350 40, 350 90, 400 90"
								/>
								<path
									stroke-width="1"
									stroke-dasharray="4 4"
									class="text-gray-600"
									d="M0 100 H400"
								/>
								<circle cx="400" cy="90" r="4" class="fill-current animate-pulse" />
							</svg>
							<div
								class="absolute bottom-4 right-4 bg-black/60 backdrop-blur px-3 py-1.5 rounded border border-white/10 text-emerald-400 text-xs font-mono"
							>
								SPX 4580.50 (+1.2%)
							</div>
						</div>

						<div class="col-span-1 bg-[#2b2b36] p-3 flex flex-col gap-3 overflow-hidden">
							<div class="text-xs text-gray-500 font-bold mb-2 uppercase tracking-wider">
								Chat Stream
							</div>

							<div class="flex gap-2 items-start opacity-60">
								<div class="w-6 h-6 rounded-full bg-blue-500 shrink-0"></div>
								<div>
									<div class="h-2 w-16 bg-gray-600 rounded mb-1"></div>
									<div class="h-2 w-24 bg-gray-700 rounded"></div>
								</div>
							</div>
							<div class="flex gap-2 items-start">
								<div
									class="w-6 h-6 rounded-full bg-emerald-500 shrink-0 flex items-center justify-center text-[10px] text-white font-bold"
								>
									M
								</div>
								<div class="text-xs">
									<span class="text-emerald-400 font-bold">Mod:</span>
									<span class="text-gray-300"
										>Approaching VWAP support at 4500. Watch for the bounce.</span
									>
								</div>
							</div>
							<div class="flex gap-2 items-start">
								<div class="w-6 h-6 rounded-full bg-indigo-500 shrink-0"></div>
								<div class="text-xs">
									<span class="text-gray-400 font-bold">Trader88:</span>
									<span class="text-gray-300">Calls loaded. Volume looking massive!</span>
								</div>
							</div>
							<div class="flex gap-2 items-start">
								<div class="w-6 h-6 rounded-full bg-orange-500 shrink-0"></div>
								<div class="text-xs">
									<span class="text-gray-400 font-bold">SarahK:</span>
									<span class="text-gray-300">Took 20% on that leg. Thanks!</span>
								</div>
							</div>
							<div
								class="mt-auto bg-gray-700/50 p-2 rounded text-xs text-gray-400 border border-gray-600/30"
							>
								Message #live-trading...
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="bg-rtp-surface border-y border-rtp-border relative z-20">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
			<div class="grid grid-cols-2 md:grid-cols-4 gap-8">
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-primary transition-colors"
					>
						500+
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
						Active Members
					</div>
				</div>
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-emerald transition-colors"
					>
						5+ Years
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">Live History</div>
				</div>
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-indigo transition-colors"
					>
						1,000+
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
						Trades Verified
					</div>
				</div>
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-blue transition-colors"
					>
						Daily
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
						Live Voice Sessions
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-bg relative overflow-hidden">
		<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-5 pointer-events-none"></div>
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
			<div class="text-center max-w-3xl mx-auto mb-20">
				<span class="text-rtp-primary font-bold uppercase tracking-wider text-sm mb-2 block"
					>The Experience</span
				>
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					A Professional Trading Floor <br /> On Your Screen
				</h2>
				<p data-gsap class="text-xl text-rtp-muted">
					Trading is lonely. It doesn't have to be. Surround yourself with winners and professional
					guidance every single day.
				</p>
			</div>

			<div class="grid md:grid-cols-3 gap-8">
				<div
					data-gsap
					class="bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-primary/50 transition-all hover:-translate-y-2 group"
				>
					<div
						class="w-14 h-14 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform"
					>
						<svg
							class="w-7 h-7 text-rtp-primary"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-white mb-3">Low-Latency Voice</h3>
					<p class="text-rtp-muted text-sm leading-relaxed">
						Hear the moderator's thought process in real-time. "I'm looking for a bounce here at
						4500." No typing delays. Just pure execution speed when seconds count.
					</p>
				</div>

				<div
					data-gsap
					class="bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-emerald/50 transition-all hover:-translate-y-2 group"
				>
					<div
						class="w-14 h-14 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform"
					>
						<svg
							class="w-7 h-7 text-rtp-emerald"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-white mb-3">1080p Screen Share</h3>
					<p class="text-rtp-muted text-sm leading-relaxed">
						Watch our charts live. See exactly what we are seeing—Key Gamma Levels, VWAP, Order
						Flow, and Fibonacci levels—as the market prints them.
					</p>
				</div>

				<div
					data-gsap
					class="bg-rtp-surface p-8 rounded-2xl border border-rtp-border hover:border-rtp-indigo/50 transition-all hover:-translate-y-2 group"
				>
					<div
						class="w-14 h-14 bg-rtp-indigo/10 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform"
					>
						<svg
							class="w-7 h-7 text-rtp-indigo"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-white mb-3">Pro Community</h3>
					<p class="text-rtp-muted text-sm leading-relaxed">
						No toxicity. No pump and dump. Just a group of serious, funded traders sharing
						high-quality ideas, news, and psychological support throughout the session.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section class="py-20 bg-rtp-surface border-y border-rtp-border">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="grid lg:grid-cols-2 gap-12 items-center">
				<div>
					<h2 data-gsap class="text-3xl md:text-4xl font-heading font-bold text-white mb-6">
						Why We Trade SPX 0DTE
					</h2>
					<p data-gsap class="text-lg text-rtp-muted mb-6">
						The S&P 500 Index (SPX) is the premier instrument for professional day traders. Unlike
						individual stocks like TSLA or NVDA, the SPX offers unique advantages that align with
						our institutional trading approach.
					</p>
					<ul class="space-y-4">
						<li class="flex gap-4 items-start" data-gsap>
							<div
								class="w-6 h-6 rounded bg-emerald-500/10 flex items-center justify-center text-emerald-500 mt-1"
							>
								✓
							</div>
							<div>
								<h4 class="text-white font-bold">Favorable Tax Treatment</h4>
								<p class="text-sm text-rtp-muted">
									SPX options fall under Section 1256, meaning 60% of gains are taxed at the lower
									long-term capital gains rate, regardless of holding period.
								</p>
							</div>
						</li>
						<li class="flex gap-4 items-start" data-gsap>
							<div
								class="w-6 h-6 rounded bg-emerald-500/10 flex items-center justify-center text-emerald-500 mt-1"
							>
								✓
							</div>
							<div>
								<h4 class="text-white font-bold">Cash Settled & No Early Assignment</h4>
								<p class="text-sm text-rtp-muted">
									You can never be assigned shares. The index settles to cash, eliminating the risk
									of overnight gap risk in physical shares.
								</p>
							</div>
						</li>
						<li class="flex gap-4 items-start" data-gsap>
							<div
								class="w-6 h-6 rounded bg-emerald-500/10 flex items-center justify-center text-emerald-500 mt-1"
							>
								✓
							</div>
							<div>
								<h4 class="text-white font-bold">Institutional Liquidity</h4>
								<p class="text-sm text-rtp-muted">
									Massive volume ensures tight bid/ask spreads, allowing for rapid entries and exits
									even with large position sizes.
								</p>
							</div>
						</li>
					</ul>
				</div>
				<div class="relative rounded-2xl overflow-hidden border border-rtp-border shadow-2xl">
					<div
						class="absolute inset-0 bg-gradient-to-t from-rtp-bg to-transparent opacity-60 z-10"
					></div>
					<div class="bg-gray-900 aspect-video flex items-center justify-center relative">
						<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-20"></div>
						<div class="z-20 text-center">
							<span class="text-6xl font-black text-white/10 tracking-widest">SPX</span>
						</div>
						<svg
							class="absolute inset-0 w-full h-full text-rtp-primary opacity-30"
							viewBox="0 0 100 100"
							preserveAspectRatio="none"
						>
							<path
								d="M0,50 Q25,30 50,50 T100,50"
								fill="none"
								stroke="currentColor"
								stroke-width="0.5"
							/>
							<path
								d="M0,50 Q25,20 50,50 T100,50"
								fill="none"
								stroke="currentColor"
								stroke-width="0.5"
							/>
							<path
								d="M0,50 Q25,80 50,50 T100,50"
								fill="none"
								stroke="currentColor"
								stroke-width="0.5"
							/>
						</svg>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="schedule" class="py-24 bg-rtp-bg relative">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<span class="text-rtp-primary font-bold uppercase tracking-wider text-sm mb-2 block"
					>Routine</span
				>
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					The Daily Trading Routine
				</h2>
				<p class="text-xl text-rtp-muted">
					Consistency is the hallmark of a professional. This is exactly how we approach every
					trading day.
				</p>
			</div>

			<div
				class="relative space-y-12 before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-rtp-border before:to-transparent"
			>
				<div
					data-gsap
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-border bg-rtp-bg shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-rtp-primary font-bold text-xs z-10"
					>
						08:30
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-surface p-6 rounded-xl border border-rtp-border shadow-lg"
					>
						<div class="flex items-center justify-between mb-2">
							<h3 class="font-bold text-white text-lg">Pre-Market Prep</h3>
							<span class="text-xs font-mono text-rtp-primary bg-rtp-primary/10 px-2 py-1 rounded"
								>AM EST</span
							>
						</div>
						<p class="text-sm text-rtp-muted leading-relaxed">
							We review overnight action, analyze key economic data drops (CPI/PPI/FOMC), and map
							out Critical Gamma Levels on the SPX. This prepares us mentally for the scenarios
							ahead.
						</p>
					</div>
				</div>

				<div
					data-gsap
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-emerald-500 bg-emerald-500/20 shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-emerald-500 font-bold text-xs z-10"
					>
						09:30
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-bg p-6 rounded-xl border-l-4 border-emerald-500 shadow-xl ring-1 ring-white/5"
					>
						<div class="flex items-center justify-between mb-2">
							<h3 class="font-bold text-white text-lg">Market Open (The Bell)</h3>
							<span class="text-xs font-mono text-emerald-500 bg-emerald-500/10 px-2 py-1 rounded"
								>AM EST</span
							>
						</div>
						<p class="text-sm text-rtp-muted leading-relaxed">
							High focus. Voice commentary is active. We execute our primary strategies: Opening
							Range Breakout or Rejection. <span class="text-white font-bold"
								>This is the most profitable window of the day.</span
							>
						</p>
					</div>
				</div>

				<div
					data-gsap
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-border bg-rtp-bg shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-rtp-muted font-bold text-xs z-10"
					>
						12:00
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-surface p-6 rounded-xl border border-rtp-border shadow-lg"
					>
						<div class="flex items-center justify-between mb-2">
							<h3 class="font-bold text-white text-lg">Lunch / Mid-Day</h3>
							<span class="text-xs font-mono text-rtp-muted bg-rtp-border/30 px-2 py-1 rounded"
								>PM EST</span
							>
						</div>
						<p class="text-sm text-rtp-muted leading-relaxed">
							Institutional volume slows. We review morning trades, calculate P&L, answer member
							questions in voice, and scan for afternoon setups. Preservation of morning capital is
							the goal.
						</p>
					</div>
				</div>

				<div
					data-gsap
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-border bg-rtp-bg shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-rtp-indigo font-bold text-xs z-10"
					>
						15:00
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-surface p-6 rounded-xl border border-rtp-border shadow-lg"
					>
						<div class="flex items-center justify-between mb-2">
							<h3 class="font-bold text-white text-lg">Power Hour</h3>
							<span class="text-xs font-mono text-rtp-indigo bg-rtp-indigo/10 px-2 py-1 rounded"
								>PM EST</span
							>
						</div>
						<p class="text-sm text-rtp-muted leading-relaxed">
							Final hour volatility injection. We look for end-of-day squeezes or flushes (MOC
							imbalances). All 0DTE positions are closed flat before the closing bell.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="py-24 bg-rtp-surface relative">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<span class="text-rtp-primary font-bold uppercase tracking-wider text-sm mb-2 block"
					>Investment</span
				>
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					Membership Access
				</h2>
				<p class="text-xl text-rtp-muted max-w-2xl mx-auto">
					Invest in your education. One disciplined trade can pay for the whole year. Cancel
					anytime.
				</p>
			</div>

			<div class="flex justify-center mb-16">
				<div
					class="bg-rtp-bg p-1.5 rounded-xl border border-rtp-border inline-flex relative shadow-inner"
				>
					<button
						onclick={() => (selectedPlan = 'monthly')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'monthly'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}">Monthly</button
					>
					<button
						onclick={() => (selectedPlan = 'quarterly')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'quarterly'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}">Quarterly</button
					>
					<button
						onclick={() => (selectedPlan = 'annual')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'annual'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}">Annual</button
					>

					<div
						class="absolute top-1.5 bottom-1.5 bg-rtp-primary rounded-lg shadow-md transition-all duration-300 ease-[cubic-bezier(0.25,0.1,0.25,1)]"
						style="left: {selectedPlan === 'monthly'
							? '0.375rem'
							: selectedPlan === 'quarterly'
								? 'calc(33.33% + 0.2rem)'
								: 'calc(66.66% + 0.1rem)'}; width: calc(33.33% - 0.4rem);"
					></div>
				</div>
			</div>

			<div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto items-center">
				<div
					class="bg-rtp-bg p-8 rounded-2xl border transition-all duration-300 {selectedPlan ===
					'monthly'
						? 'border-rtp-primary opacity-100 scale-105 shadow-xl shadow-rtp-primary/10'
						: 'border-rtp-border opacity-70 hover:opacity-100'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Monthly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$197</span>
						<span class="text-rtp-muted">/mo</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-muted bg-rtp-surface p-2 rounded mb-6 text-center border border-rtp-border"
					>
						$9.85 / trading day
					</div>
					<ul class="space-y-4 mb-8 text-sm text-rtp-muted">
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Live Voice & Screen</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Real-time SPX Alerts</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Full Chat Access</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Onboarding Course</li>
					</ul>
					<a
						href="/checkout/monthly-room"
						class="block w-full py-3 bg-rtp-surface border border-rtp-border text-white font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
						>Select Monthly</a
					>
				</div>

				<div
					class="bg-rtp-bg p-10 rounded-3xl border-2 shadow-2xl transform relative z-10 transition-all duration-300 {selectedPlan ===
					'quarterly'
						? 'border-rtp-primary shadow-rtp-primary/20 md:scale-110 opacity-100'
						: 'border-rtp-border shadow-rtp-border/10 md:scale-100 opacity-70 hover:opacity-100'}"
				>
					<div
						class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-primary text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide shadow-lg"
					>
						Most Popular
					</div>
					<h3 class="text-2xl font-bold text-white mb-4">Quarterly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-5xl font-extrabold text-white">$497</span>
						<span class="text-rtp-muted">/qtr</span>
					</div>
					<div
						class="text-xs font-mono text-emerald-400 bg-emerald-500/10 p-2 rounded mb-6 text-center border border-emerald-500/30"
					>
						Save 15% ($8.20 / trading day)
					</div>
					<ul class="space-y-4 mb-8 text-sm text-white">
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span>
							<span class="font-bold">Priority Discord Support</span>
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span> Live Voice & Screen Share
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span> Real-time SPX Alerts
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span> Onboarding Course
						</li>
					</ul>
					<a
						href="/checkout/quarterly-room"
						class="block w-full py-4 bg-rtp-primary text-white font-bold rounded-xl text-center hover:bg-blue-600 transition-colors shadow-lg hover:shadow-rtp-primary/50"
						>Join Quarterly</a
					>
				</div>

				<div
					class="bg-rtp-bg p-8 rounded-2xl border transition-all duration-300 relative {selectedPlan ===
					'annual'
						? 'border-emerald-500 opacity-100 scale-105 shadow-xl shadow-emerald-500/10'
						: 'border-rtp-border opacity-70 hover:opacity-100'}"
				>
					{#if selectedPlan === 'annual'}
						<div
							class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-emerald text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide shadow-lg"
						>
							Best Deal
						</div>
					{/if}
					<h3 class="text-xl font-bold text-white mb-4">Annual</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$1,647</span>
						<span class="text-rtp-muted">/yr</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-emerald bg-rtp-surface p-2 rounded mb-6 text-center border border-rtp-border"
					>
						Save 30% ($6.50 / trading day)
					</div>
					<ul class="space-y-4 mb-8 text-sm text-rtp-muted">
						<li class="flex gap-3">
							<span class="text-rtp-primary">✓</span>
							<span class="font-bold">1-on-1 Coaching Call (1hr)</span>
						</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Live Voice & Screen</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Real-time Alerts</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Onboarding Course</li>
					</ul>
					<a
						href="/checkout/annual-room"
						class="block w-full py-3 bg-rtp-surface border border-rtp-emerald text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-white transition-colors"
						>Select Annual</a
					>
				</div>
			</div>
			<div class="mt-12 text-center">
				<p class="text-rtp-muted text-sm flex items-center justify-center gap-2">
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
						></path></svg
					>
					Secure checkout powered by Stripe. Cancel anytime.
				</p>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-bg border-t border-rtp-border">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center text-white mb-12">
				Frequently Asked Questions
			</h2>
			<div class="space-y-4">
				{#each faqList as faq, i}
					<div
						class="border border-rtp-border rounded-xl bg-rtp-surface overflow-hidden hover:border-rtp-primary/30 transition-colors"
					>
						<button
							class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-white"
							onclick={() => toggleFaq(i)}
							aria-expanded={openFaq === i}
						>
							<span class="text-base pr-4">{faq.question}</span>
							<svg
								class="w-5 h-5 text-rtp-primary flex-shrink-0 transform transition-transform duration-300 {openFaq ===
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
								class="px-6 pb-6 text-rtp-muted text-sm leading-relaxed border-t border-rtp-border/50 pt-4"
							>
								{faq.answer}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section
		class="py-24 bg-gradient-to-br from-rtp-primary to-rtp-indigo text-white relative overflow-hidden"
	>
		<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
			<h2 class="text-4xl md:text-6xl font-heading font-extrabold mb-6">
				Markets Open at 9:30 AM.
			</h2>
			<p class="text-xl text-blue-100 mb-10 max-w-2xl mx-auto">
				Don't miss the next opening bell. Join the room today and be ready for tomorrow's session.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<a
					href="#pricing"
					class="bg-white text-rtp-primary px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-2xl hover:-translate-y-1"
				>
					Get Access Now
				</a>
			</div>
			<p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee • Cancel Anytime</p>
		</div>
	</section>
</div>

<MarketingFooter />
