<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// --- Pricing State ---
	let selectedPlan: 'monthly' | 'quarterly' | 'annual' = 'quarterly';

	// --- FAQ Logic ---
	let openFaq: number | null = null;
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	// --- Intersection Observer for Scroll Animations ---
	let observer: IntersectionObserver;

	function reveal(node: HTMLElement, params: { delay?: number } = {}) {
		// ENTERPRISE FIX: Show immediately, only hide if observer is ready
		node.classList.add('opacity-100', 'translate-y-0');
		
		// Only add animation classes if observer exists (browser environment)
		if (typeof window !== 'undefined' && observer) {
			node.classList.remove('opacity-100', 'translate-y-0');
			node.classList.add('opacity-0', 'translate-y-8');
			node.dataset.delay = (params.delay || 0).toString();
			observer.observe(node);
		}

		return {
			destroy() {
				if (observer) observer.unobserve(node);
			}
		};
	}

	onMount(() => {
		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const el = entry.target as HTMLElement;
						const delay = parseInt(el.dataset.delay || '0');

						setTimeout(() => {
							el.classList.remove('opacity-0', 'translate-y-8');
							el.classList.add(
								'opacity-100',
								'translate-y-0',
								'transition-all',
								'duration-700',
								'ease-out'
							);
						}, delay);

						observer.unobserve(el);
					}
				});
			},
			{ threshold: 0.15, rootMargin: '0px 0px -50px 0px' }
		);

		document.querySelectorAll('[data-reveal]').forEach((el) => observer.observe(el));
	});

	// --- SEO: STRUCTURED DATA (JSON-LD) ---
	const productSchema = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: 'Live SPX Day Trading Room',
		description:
			'Join our professional day trading room. Live voice commentary, screen sharing, and real-time SPX 0DTE alerts.',
		brand: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
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
					priceCurrency: 'USD'
				},
				{
					'@type': 'Offer',
					name: 'Annual Access',
					price: '1647',
					priceCurrency: 'USD'
				}
			]
		}
	};

	const faqSchema = {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: [
			{
				'@type': 'Question',
				name: 'Do I need to be an expert to join?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: "No. While the trading is fast-paced, we provide a 'New Member' onboarding video series to help you understand our levels, terminology, and platform execution."
				}
			},
			{
				'@type': 'Question',
				name: 'What platform is the room hosted on?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: 'We host our live room on a private, boosted Discord server with low-latency Voice Channels and 1080p Screen Share.'
				}
			}
		]
	};

	// Schema for SEOHead
	const combinedSchema = [productSchema, faqSchema];
</script>

<SEOHead
	title="Live Day Trading Room | Trade SPX 0DTE Live"
	description="Never trade alone again. Join our live SPX day trading room for real-time voice commentary, screen sharing, professional analysis, and community support. Trade with pros daily."
	canonical="/live-trading-rooms/day-trading"
	ogType="product"
	ogImage="/images/og-live-room.jpg"
	ogImageAlt="Live SPX Day Trading Room - Trade with Professional Traders"
	keywords={[
		'day trading room',
		'live trading',
		'spx 0dte',
		'trading community',
		'stock market live stream',
		'live trading room',
		'professional trading room',
		'day trading community'
	]}
	schema={combinedSchema}
	schemaType="Product"
	productPrice="197"
	productCurrency="USD"
	productAvailability="InStock"
/>

<main
	class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text font-sans selection:bg-rtp-primary selection:text-white"
>
	<section class="relative min-h-[90vh] flex items-center overflow-hidden py-24 lg:py-0">
		<div class="absolute inset-0 bg-rtp-bg z-0">
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
					use:reveal={{ delay: 0 }}
					class="inline-flex items-center gap-3 bg-red-500/10 border border-red-500/20 px-4 py-1.5 rounded-full backdrop-blur-md"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-500 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-red-600"></span>
					</span>
					<span class="text-xs font-bold tracking-widest uppercase text-red-400"
						>Live Trading Active</span
					>
				</div>

				<h1
					use:reveal={{ delay: 100 }}
					class="text-5xl md:text-7xl font-heading font-extrabold leading-tight"
				>
					Never Trade <br />
					<span
						class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-blue via-indigo-400 to-white"
						>Alone Again.</span
					>
				</h1>

				<p
					use:reveal={{ delay: 200 }}
					class="text-xl text-rtp-muted max-w-2xl mx-auto lg:mx-0 leading-relaxed"
				>
					Join the digital trading floor. Watch our screens, hear our voice commentary, and execute
					SPX 0DTE trades alongside 500+ serious traders.
				</p>

				<div
					use:reveal={{ delay: 300 }}
					class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start"
				>
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
					use:reveal={{ delay: 400 }}
					class="flex items-center justify-center lg:justify-start gap-4 text-sm text-rtp-muted pt-4"
				>
					<div class="flex -space-x-3">
						<div class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-gradient-to-br from-blue-500 to-blue-600 flex items-center justify-center text-white text-xs font-bold">
							JD
						</div>
						<div class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center text-white text-xs font-bold">
							MK
						</div>
						<div class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-gradient-to-br from-purple-500 to-purple-600 flex items-center justify-center text-white text-xs font-bold">
							SR
						</div>
						<div
							class="w-10 h-10 rounded-full border-2 border-rtp-bg bg-rtp-surface flex items-center justify-center text-xs font-bold text-white"
						>
							+500
						</div>
					</div>
					<p>Traders online now</p>
				</div>
			</div>

			<div class="hidden lg:block relative perspective-1000">
				<div
					class="absolute inset-0 bg-gradient-to-tr from-rtp-primary/20 to-transparent rounded-full blur-3xl transform translate-x-10 translate-y-10"
				></div>

				<div
					class="relative bg-[#1e1e24] border border-rtp-border rounded-xl shadow-2xl overflow-hidden transform rotate-y-[-5deg] hover:rotate-0 transition-transform duration-500"
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
							<div class="font-bold text-white"># 🔴-live-trading-floor</div>
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
							</svg>
							<div
								class="absolute bottom-4 right-4 bg-black/50 backdrop-blur px-3 py-1 rounded text-emerald-400 text-xs font-mono"
							>
								SPX 4580.50 (+1.2%)
							</div>
						</div>

						<div class="col-span-1 bg-[#2b2b36] p-3 flex flex-col gap-3 overflow-hidden">
							<div class="text-xs text-gray-500 font-bold mb-2">CHAT STREAM</div>

							<div class="flex gap-2 items-start opacity-60">
								<div class="w-6 h-6 rounded-full bg-blue-500"></div>
								<div>
									<div class="h-2 w-16 bg-gray-600 rounded mb-1"></div>
									<div class="h-2 w-24 bg-gray-700 rounded"></div>
								</div>
							</div>
							<div class="flex gap-2 items-start">
								<div class="w-6 h-6 rounded-full bg-emerald-500"></div>
								<div class="text-xs">
									<span class="text-emerald-400 font-bold">Mod:</span>
									<span class="text-gray-300">Approaching VWAP support. Get ready.</span>
								</div>
							</div>
							<div class="flex gap-2 items-start">
								<div class="w-6 h-6 rounded-full bg-indigo-500"></div>
								<div class="text-xs">
									<span class="text-gray-400 font-bold">User88:</span>
									<span class="text-gray-300">Volume looking good!</span>
								</div>
							</div>
							<div class="mt-auto bg-gray-700/50 p-2 rounded text-xs text-gray-400">
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
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-primary transition-colors"
					>
						500+
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">Room Members</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-emerald transition-colors"
					>
						5+ Years
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">Live History</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-indigo transition-colors"
					>
						1,000+
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
						Trades Called
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-extrabold text-white mb-1 group-hover:text-rtp-blue transition-colors"
					>
						Daily
					</div>
					<div class="text-xs font-bold uppercase tracking-widest text-rtp-muted">
						Live Sessions
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-bg relative overflow-hidden">
		<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-5"></div>
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
			<div class="text-center max-w-3xl mx-auto mb-20">
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					A Professional Trading Floor <br /> On Your Screen
				</h2>
				<p use:reveal={{ delay: 100 }} class="text-xl text-rtp-muted">
					Trading is lonely. It doesn't have to be. Surround yourself with winners and professional
					guidance.
				</p>
			</div>

			<div class="grid md:grid-cols-3 gap-8">
				<div
					use:reveal={{ delay: 0 }}
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
						4500." No typing delays. Just execution.
					</p>
				</div>

				<div
					use:reveal={{ delay: 100 }}
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
						Watch our charts live. See exactly what we are seeing—Key Gamma Levels, VWAP, and Order
						Flow—as it happens.
					</p>
				</div>

				<div
					use:reveal={{ delay: 200 }}
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
						No toxicity. No pump and dump. Just a group of serious traders sharing ideas, news, and
						support throughout the session.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section id="schedule" class="py-24 bg-rtp-surface border-y border-rtp-border">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					The Daily Routine
				</h2>
				<p class="text-xl text-rtp-muted">Consistency is key. Here is what your day looks like.</p>
			</div>

			<div
				class="relative space-y-12 before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-rtp-border before:to-transparent"
			>
				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-border bg-rtp-bg shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-rtp-primary font-bold text-xs"
					>
						08:30
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-bg p-6 rounded-xl border border-rtp-border shadow-lg"
					>
						<div class="flex items-center justify-between mb-1">
							<h3 class="font-bold text-white">Pre-Market Prep</h3>
							<span class="text-xs font-mono text-rtp-primary">AM EST</span>
						</div>
						<p class="text-sm text-rtp-muted">
							We review overnight action, key economic data drops (CPI/FOMC), and map out Key Levels
							on the SPX.
						</p>
					</div>
				</div>

				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-emerald-500 bg-emerald-500/20 shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-emerald-500 font-bold text-xs"
					>
						09:30
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-bg p-6 rounded-xl border-l-4 border-emerald-500 shadow-lg"
					>
						<div class="flex items-center justify-between mb-1">
							<h3 class="font-bold text-white">Market Open (The Bell)</h3>
							<span class="text-xs font-mono text-emerald-500">AM EST</span>
						</div>
						<p class="text-sm text-rtp-muted">
							High focus. Voice commentary is active. We look for the opening range breakout or
							rejection. <span class="text-white font-bold">Primary Trading Session.</span>
						</p>
					</div>
				</div>

				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-border bg-rtp-bg shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-rtp-muted font-bold text-xs"
					>
						12:00
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-bg p-6 rounded-xl border border-rtp-border shadow-lg"
					>
						<div class="flex items-center justify-between mb-1">
							<h3 class="font-bold text-white">Lunch / Mid-Day</h3>
							<span class="text-xs font-mono text-rtp-muted">PM EST</span>
						</div>
						<p class="text-sm text-rtp-muted">
							Volume slows. We review morning trades, answer member questions, and scan for setups
							forming for the afternoon session.
						</p>
					</div>
				</div>

				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-border bg-rtp-bg shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 text-rtp-indigo font-bold text-xs"
					>
						15:00
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-rtp-bg p-6 rounded-xl border border-rtp-border shadow-lg"
					>
						<div class="flex items-center justify-between mb-1">
							<h3 class="font-bold text-white">Power Hour</h3>
							<span class="text-xs font-mono text-rtp-indigo">PM EST</span>
						</div>
						<p class="text-sm text-rtp-muted">
							Final hour volatility. We look for end-of-day squeezes or flushes. Positions are
							closed before the bell.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="py-24 bg-rtp-bg relative">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					Membership Access
				</h2>
				<p class="text-xl text-rtp-muted max-w-2xl mx-auto">
					Invest in your education. One good trade can pay for the whole year.
				</p>
			</div>

			<div class="flex justify-center mb-16">
				<div class="bg-rtp-surface p-1.5 rounded-xl border border-rtp-border inline-flex relative">
					<button
						on:click={() => (selectedPlan = 'monthly')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'monthly'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}">Monthly</button
					>
					<button
						on:click={() => (selectedPlan = 'quarterly')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'quarterly'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}">Quarterly</button
					>
					<button
						on:click={() => (selectedPlan = 'annual')}
						class="relative z-10 px-6 py-2 rounded-lg font-bold text-sm transition-colors duration-200 {selectedPlan ===
						'annual'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}">Annual</button
					>

					<div
						class="absolute top-1.5 bottom-1.5 bg-rtp-primary rounded-lg shadow-md transition-all duration-300 ease-out"
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
					class="bg-rtp-surface p-8 rounded-2xl border transition-all {selectedPlan === 'monthly' ? 'border-rtp-primary opacity-100 scale-105' : 'border-rtp-border opacity-70 hover:opacity-90'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Monthly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$197</span>
						<span class="text-rtp-muted">/mo</span>
					</div>
					<div class="text-xs font-mono text-rtp-muted bg-rtp-bg p-2 rounded mb-6 text-center">
						$9.85 / trading day
					</div>
					<ul class="space-y-4 mb-8 text-sm text-rtp-muted">
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Live Voice & Screen</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Real-time Alerts</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Chat Access</li>
					</ul>
					<a
						href="/checkout/monthly-room"
						class="block w-full py-3 bg-rtp-bg border border-rtp-border text-white font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
						>Select Monthly</a
					>
				</div>

				<div
					class="bg-rtp-bg p-10 rounded-3xl border-2 shadow-2xl transform relative z-10 transition-all {selectedPlan === 'quarterly' ? 'border-rtp-primary shadow-rtp-primary/20 md:scale-110 opacity-100' : 'border-rtp-border shadow-rtp-border/10 md:scale-100 opacity-70 hover:opacity-90'}"
				>
					<div
						class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-primary text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide"
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
							<span class="font-bold">Priority Support</span>
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span> Live Voice & Screen
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span> Real-time Alerts
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-primary font-bold">✓</span> Chat Access
						</li>
					</ul>
					<a
						href="/checkout/quarterly-room"
						class="block w-full py-4 bg-rtp-primary text-white font-bold rounded-xl text-center hover:bg-blue-600 transition-colors shadow-lg"
						>Join Quarterly</a
					>
				</div>

				<div
					class="bg-rtp-surface p-8 rounded-2xl border transition-all {selectedPlan === 'annual' ? 'border-emerald-500 opacity-100 scale-105' : 'border-rtp-border opacity-70 hover:opacity-90'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Annual</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$1,647</span>
						<span class="text-rtp-muted">/yr</span>
					</div>
					<div class="text-xs font-mono text-rtp-emerald bg-rtp-bg p-2 rounded mb-6 text-center">
						Save 30% ($6.50 / trading day)
					</div>
					<ul class="space-y-4 mb-8 text-sm text-rtp-muted">
						<li class="flex gap-3">
							<span class="text-rtp-primary">✓</span>
							<span class="font-bold">1-on-1 Coaching Call</span>
						</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Live Voice & Screen</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Real-time Alerts</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Chat Access</li>
					</ul>
					<a
						href="/checkout/annual-room"
						class="block w-full py-3 bg-rtp-bg border border-rtp-emerald text-emerald-500 font-bold rounded-lg text-center hover:bg-emerald-500 hover:text-white transition-colors"
						>Select Annual</a
					>
				</div>
			</div>
			<div class="mt-12 text-center">
				<p class="text-rtp-muted text-sm">Secure checkout powered by Stripe. Cancel anytime.</p>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-surface border-t border-rtp-border">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center text-white mb-12">Common Questions</h2>
			<div class="space-y-4">
				{#each faqSchema.mainEntity as faq, i}
					<div class="border border-rtp-border rounded-xl bg-rtp-bg overflow-hidden">
						<button
							class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-white"
							on:click={() => toggleFaq(i)}
						>
							{faq.name}
							<svg
								class="w-5 h-5 text-rtp-muted transform transition-transform duration-300 {openFaq ===
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
								{faq.acceptedAnswer.text}
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
				Don't miss the next opening bell. Join the room and trade with professionals.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<a
					href="#pricing"
					class="bg-white text-rtp-primary px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-2xl hover:-translate-y-1"
				>
					Get Access Now
				</a>
			</div>
			<p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee</p>
		</div>
	</section>

	<footer class="bg-rtp-bg py-12 border-t border-rtp-border">
		<div
			class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-rtp-muted leading-relaxed"
		>
			<p class="mb-4 font-bold uppercase text-white">Risk Disclosure</p>
			<p class="max-w-4xl mx-auto">
				Trading futures and options involves substantial risk of loss and is not suitable for every
				investor. The highly leveraged nature of futures trading means that small market movements
				will have a great impact on your trading account. Revolution Trading Pros is an educational
				platform and not a financial advisory service.
			</p>
			<p class="mt-8 opacity-50">
				&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
			</p>
		</div>
	</footer>
</main>
