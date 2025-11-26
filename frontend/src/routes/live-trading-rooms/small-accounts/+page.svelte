<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Schema for small accounts trading room
	const productSchema = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: 'Small Accounts Day Trading Room',
		description:
			'Live trading room designed for small account traders. Learn professional strategies optimized for accounts under $25k.',
		brand: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		},
		offers: {
			'@type': 'AggregateOffer',
			priceCurrency: 'USD',
			lowPrice: '97',
			highPrice: '197',
			offerCount: '3'
		}
	};

	// --- Pricing State ---
	let selectedPlan: 'monthly' | 'quarterly' | 'annual' = 'quarterly';

	// --- FAQ Logic ---
	let openFaq: number | null = null;
	const toggleFaq = (index: number) => (openFaq = openFaq === index ? null : index);

	// --- Intersection Observer for Scroll Animations ---
	let observer: IntersectionObserver;

	function reveal(node: HTMLElement, params: { delay?: number } = {}) {
		node.classList.add('opacity-0', 'translate-y-8');

		if (observer) {
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
	const schemaOrg = {
		'@context': 'https://schema.org',
		'@graph': [
			{
				'@type': 'Organization',
				name: 'Revolution Trading Pros',
				url: 'https://revolutiontradingpros.com'
			},
			{
				'@type': 'Service',
				name: 'Live SPX Day Trading Room',
				serviceType: 'Financial Education',
				description:
					'Real-time 0DTE options trading mentorship, live screen sharing, and trade alerts.',
				provider: { '@type': 'Organization', name: 'Revolution Trading Pros' }
			},
			{
				'@type': 'Product',
				name: 'Day Trading Room Membership',
				description:
					'Access to live trading room, real-time SPX alerts, and daily market analysis.',
				offers: {
					'@type': 'AggregateOffer',
					priceCurrency: 'USD',
					lowPrice: '137',
					highPrice: '197',
					offerCount: '3'
				}
			},
			{
				'@type': 'FAQPage',
				mainEntity: [
					{
						'@type': 'Question',
						name: 'Do I need a large account to day trade SPX?',
						acceptedAnswer: {
							'@type': 'Answer',
							text: 'No. SPX options offer significant leverage. However, we strictly recommend having at least $2,000 to manage risk properly.'
						}
					},
					{
						'@type': 'Question',
						name: 'What is 0DTE trading?',
						acceptedAnswer: {
							'@type': 'Answer',
							text: "0DTE stands for '0 Days To Expiration.' We trade options that expire the same day to capitalize on rapid intraday moves."
						}
					}
				]
			}
		]
	};
	// Schema for SEOHead
	const combinedSchema = schemaOrg['@graph'];
</script>

<SEOHead
	title="Small Accounts Day Trading Room | PDT-Free Strategies"
	description="Trade live with professionals. Our Small Accounts Day Trading Room offers real-time screen sharing, precise entry/exit alerts, and strategies optimized for accounts under $25k."
	canonical="/live-trading-rooms/small-accounts"
	ogType="product"
	ogImage="/images/day-trading-og.jpg"
	ogImageAlt="Small Accounts Day Trading Room - PDT-Free Strategies"
	keywords={[
		'small account trading',
		'PDT-free trading',
		'day trading under 25k',
		'small account strategies',
		'live trading room',
		'SPX 0DTE strategy',
		'options trading community'
	]}
	schema={combinedSchema}
	schemaType="Product"
	productPrice={97}
	productCurrency="USD"
	productAvailability="in stock"
/>

<main
	class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text font-sans selection:bg-rtp-primary selection:text-white"
>
	<section class="relative min-h-[90vh] flex items-center overflow-hidden py-24 lg:py-0">
		<div class="absolute inset-0 bg-rtp-bg z-0">
			<div
				class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:40px_40px] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"
			></div>
			<div
				class="absolute top-0 right-0 w-[600px] h-[600px] bg-rtp-primary/10 rounded-full blur-[120px] animate-pulse"
			></div>
			<div
				class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-rtp-indigo/10 rounded-full blur-[100px]"
			></div>
		</div>

		<div
			class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-12 items-center"
		>
			<div class="text-center lg:text-left">
				<div
					use:reveal={{ delay: 0 }}
					class="inline-flex items-center gap-2 bg-rtp-surface border border-rtp-border px-4 py-1.5 rounded-full mb-8 shadow-lg shadow-emerald-500/10 backdrop-blur-md"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
					</span>
					<span class="text-xs font-bold tracking-wide uppercase text-emerald-400"
						>Market Active Now</span
					>
				</div>

				<h1
					use:reveal={{ delay: 100 }}
					class="text-4xl md:text-6xl font-heading font-extrabold mb-6 leading-tight"
				>
					Master <span
						class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-primary to-emerald-400"
						>0DTE Options</span
					> <br />in Real-Time.
				</h1>

				<p
					use:reveal={{ delay: 200 }}
					class="text-xl text-rtp-muted mb-8 max-w-xl mx-auto lg:mx-0 leading-relaxed"
				>
					Don't just get alerts—watch the execution. Join our live voice & screen-share room where
					we hunt high-probability setups on SPX every morning.
				</p>

				<div
					use:reveal={{ delay: 300 }}
					class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start"
				>
					<a
						href="#pricing"
						class="bg-rtp-primary text-white px-8 py-4 rounded-xl font-bold text-lg hover:bg-blue-600 transition-all shadow-lg shadow-rtp-primary/25 hover:-translate-y-1"
					>
						Join the Live Room
					</a>
					<a
						href="#daily-routine"
						class="bg-rtp-surface border border-rtp-border text-rtp-text px-8 py-4 rounded-xl font-bold text-lg hover:bg-rtp-bg transition-all"
					>
						See Daily Routine
					</a>
				</div>

				<div
					use:reveal={{ delay: 400 }}
					class="mt-8 flex items-center justify-center lg:justify-start gap-4 text-sm text-rtp-muted font-medium"
				>
					<div class="flex -space-x-2">
						<div class="w-8 h-8 rounded-full bg-gray-800 border-2 border-rtp-bg"></div>
						<div class="w-8 h-8 rounded-full bg-gray-700 border-2 border-rtp-bg"></div>
						<div
							class="w-8 h-8 rounded-full bg-rtp-primary border-2 border-rtp-bg flex items-center justify-center text-white text-xs font-bold"
						>
							500+
						</div>
					</div>
					<p>Traders currently active</p>
				</div>
			</div>

			<div class="hidden lg:block relative perspective-1000">
				<div
					class="absolute inset-0 bg-gradient-to-tr from-rtp-primary/20 to-transparent rounded-full blur-3xl"
				></div>

				<div
					class="relative transform rotate-y-[-5deg] hover:rotate-0 transition-transform duration-700"
				>
					<div class="rounded-2xl overflow-hidden border border-rtp-border shadow-2xl">
						[Image of live trading software interface]
					</div>

					<div
						class="absolute -bottom-6 -left-6 bg-rtp-surface/90 backdrop-blur border border-rtp-border p-4 rounded-xl shadow-xl animate-bounce-slow"
					>
						<div class="flex items-center gap-3">
							<div class="bg-emerald-500/20 p-2 rounded-lg">
								<svg
									class="w-6 h-6 text-emerald-500"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
									><path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
									/></svg
								>
							</div>
							<div>
								<div class="text-xs text-rtp-muted uppercase font-bold">Latest Call</div>
								<div class="text-white font-bold">SPX 4450 Calls @ $2.50</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="bg-rtp-surface border-y border-rtp-border py-12">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div
				class="grid grid-cols-2 md:grid-cols-4 gap-8 divide-x-0 md:divide-x divide-rtp-border/50"
			>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-primary mb-2 group-hover:scale-110 transition-transform"
					>
						500+
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Active Members
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-2 group-hover:scale-110 transition-transform"
					>
						9:30AM
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Live Bell-to-Bell
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-2 group-hover:scale-110 transition-transform"
					>
						1000+
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Setups Called
					</div>
				</div>
				<div class="text-center group">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-blue mb-2 group-hover:scale-110 transition-transform"
					>
						24/7
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Discord Access
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="inside-the-room" class="py-24 bg-rtp-bg">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					Inside The Room
				</h2>
				<p use:reveal={{ delay: 100 }} class="text-xl text-rtp-muted max-w-3xl mx-auto">
					We don't just give signals. We teach you how to fish with institutional-grade tools.
				</p>
			</div>

			<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
				<div
					use:reveal={{ delay: 0 }}
					class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-primary transition-all duration-300 group"
				>
					<div
						class="w-12 h-12 bg-rtp-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform"
					>
						<svg
							class="w-6 h-6 text-rtp-primary"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Live Screen Sharing</h3>
					<p class="text-rtp-muted leading-relaxed mb-6 text-sm">
						Watch our charts in real-time. See exactly where we draw support, resistance, and supply
						zones before the trade happens.
					</p>
					<div
						class="rounded-lg overflow-hidden border border-rtp-border opacity-80 group-hover:opacity-100 transition-opacity"
					></div>
				</div>

				<div
					use:reveal={{ delay: 100 }}
					class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-emerald transition-all duration-300 group"
				>
					<div
						class="w-12 h-12 bg-rtp-emerald/10 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform"
					>
						<svg
							class="w-6 h-6 text-rtp-emerald"
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
					<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Instant Push Alerts</h3>
					<p class="text-rtp-muted leading-relaxed mb-6 text-sm">
						Can't be at your desk? Get instant notifications via our mobile app for every Entry,
						Trim, and Exit.
					</p>
					<div
						class="rounded-lg overflow-hidden border border-rtp-border opacity-80 group-hover:opacity-100 transition-opacity"
					></div>
				</div>

				<div
					use:reveal={{ delay: 200 }}
					class="bg-rtp-surface p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-indigo transition-all duration-300 group"
				>
					<div
						class="w-12 h-12 bg-rtp-indigo/10 rounded-xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform"
					>
						<svg
							class="w-6 h-6 text-rtp-indigo"
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
					<h3 class="text-xl font-heading font-bold text-rtp-text mb-3">Institutional Flow</h3>
					<p class="text-rtp-muted leading-relaxed mb-6 text-sm">
						We track Dark Pool data and Gamma Exposure (GEX) levels to understand where the "Smart
						Money" is positioning.
					</p>
					<div
						class="rounded-lg overflow-hidden border border-rtp-border opacity-80 group-hover:opacity-100 transition-opacity"
					></div>
				</div>
			</div>
		</div>
	</section>

	<section id="daily-routine" class="py-24 bg-rtp-surface border-y border-rtp-border">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					Your Daily Routine
				</h2>
				<p class="text-xl text-rtp-muted">Consistency is the key to longevity. Here is the plan.</p>
			</div>

			<div
				class="space-y-8 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-rtp-border before:to-transparent"
			>
				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-primary bg-rtp-surface group-hover:bg-rtp-primary group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-primary transition-colors z-10"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
							></path></svg
						>
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm"
					>
						<div class="flex items-center justify-between mb-2">
							<span class="font-bold text-rtp-primary">8:30 AM EST</span>
							<span class="text-xs bg-rtp-primary/10 text-rtp-primary px-2 py-0.5 rounded font-bold"
								>PRE-MARKET</span
							>
						</div>
						<h3 class="font-bold text-rtp-text">Game Plan & Levels</h3>
						<p class="text-sm text-rtp-muted mt-2">
							We review overnight futures, check the economic calendar (CPI, FOMC), and map out key
							Support/Resistance zones on SPX.
						</p>
					</div>
				</div>

				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-emerald-500 bg-emerald-500/10 group-hover:bg-emerald-500 group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-emerald-500 transition-colors z-10"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 10V3L4 14h7v7l9-11h-7z"
							></path></svg
						>
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border-l-4 border-emerald-500 shadow-lg"
					>
						<div class="flex items-center justify-between mb-2">
							<span class="font-bold text-emerald-500">9:30 AM EST</span>
							<span class="text-xs bg-emerald-500/10 text-emerald-500 px-2 py-0.5 rounded font-bold"
								>MARKET OPEN</span
							>
						</div>
						<h3 class="font-bold text-rtp-text">The Opening Bell</h3>
						<p class="text-sm text-rtp-muted mt-2">
							100% Focus. We look for the Opening Range Breakout or Rejection. Voice commentary is
							live and rapid-fire.
						</p>
					</div>
				</div>

				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-indigo bg-rtp-surface group-hover:bg-rtp-indigo group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-indigo transition-colors z-10"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
							></path></svg
						>
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm"
					>
						<div class="flex items-center justify-between mb-2">
							<span class="font-bold text-rtp-indigo">11:00 AM - 2:00 PM</span>
							<span class="text-xs bg-rtp-indigo/10 text-rtp-indigo px-2 py-0.5 rounded font-bold"
								>EDUCATION</span
							>
						</div>
						<h3 class="font-bold text-rtp-text">Review & Analysis</h3>
						<p class="text-sm text-rtp-muted mt-2">
							Volume slows. We review morning trades, answer member Q&A, and teach risk management
							strategies.
						</p>
					</div>
				</div>

				<div
					use:reveal
					class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group"
				>
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full border border-rtp-blue bg-rtp-surface group-hover:bg-rtp-blue group-hover:text-white shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow text-rtp-blue transition-colors z-10"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
							></path></svg
						>
					</div>
					<div
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-bg rounded-xl border border-rtp-border shadow-sm"
					>
						<div class="flex items-center justify-between mb-2">
							<span class="font-bold text-rtp-blue">3:00 PM EST</span>
							<span class="text-xs bg-rtp-blue/10 text-rtp-blue px-2 py-0.5 rounded font-bold"
								>THE CLOSE</span
							>
						</div>
						<h3 class="font-bold text-rtp-text">Power Hour</h3>
						<p class="text-sm text-rtp-muted mt-2">
							Institutional volume returns. We look for End-of-Day squeezes or hedge our positions
							into the close.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="py-24 bg-rtp-bg">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					Simple Membership
				</h2>
				<p class="text-xl text-rtp-muted max-w-2xl mx-auto">
					Invest in your education. Cancel anytime.
				</p>
			</div>

			<div class="flex justify-center mb-12">
				<div
					class="bg-rtp-surface p-1.5 rounded-xl shadow-inner border border-rtp-border inline-flex gap-1 relative"
				>
					{#each ['monthly', 'quarterly', 'annual'] as plan}
						<button
							on:click={() => (selectedPlan = plan as 'monthly' | 'quarterly' | 'annual')}
							class="relative px-6 py-2.5 rounded-lg text-sm font-bold capitalize transition-all duration-200 z-10 focus:outline-none focus:ring-2 focus:ring-rtp-primary
                            {selectedPlan === plan
								? 'text-white'
								: 'text-rtp-muted hover:text-rtp-text'}"
						>
							{plan}
						</button>
					{/each}
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

			<div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 {selectedPlan === 'monthly'
						? 'border-rtp-primary ring-2 ring-rtp-primary/20 opacity-100 scale-100'
						: 'border-rtp-border opacity-60 scale-95'} transition-all duration-300"
				>
					<h3 class="text-xl font-bold text-rtp-text">Monthly</h3>
					<div class="mt-4 flex items-baseline text-rtp-text">
						<span class="text-5xl font-extrabold tracking-tight">$197</span>
						<span class="ml-1 text-xl text-rtp-muted">/mo</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-muted bg-rtp-bg p-2 rounded mt-4 text-center border border-rtp-border"
					>
						Cost: $9.85 / Trading Day
					</div>
					<ul class="mt-8 space-y-4 text-rtp-muted text-sm">
						<li class="flex items-center gap-3">
							<span class="text-rtp-primary">✓</span> Live Voice & Screen
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-primary">✓</span> Real-Time Alerts
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-primary">✓</span> Chat Access
						</li>
					</ul>
					<a
						href="/checkout/monthly-room"
						class="block w-full mt-8 bg-rtp-surface border border-rtp-primary text-rtp-primary py-3 rounded-lg font-bold text-center hover:bg-rtp-primary hover:text-white transition-colors"
						>Select Monthly</a
					>
				</div>

				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-xl border-2 {selectedPlan === 'quarterly'
						? 'border-rtp-emerald ring-4 ring-rtp-emerald/20 opacity-100 scale-105 z-10'
						: 'border-rtp-border opacity-60 scale-95'} transition-all duration-300 relative"
				>
					<div
						class="absolute top-0 right-0 -mt-3 mr-3 bg-rtp-emerald text-white text-xs font-bold px-3 py-1 rounded-full"
					>
						MOST POPULAR
					</div>
					<h3 class="text-xl font-bold text-rtp-text">Quarterly</h3>
					<div class="mt-4 flex items-baseline text-rtp-text">
						<span class="text-5xl font-extrabold tracking-tight">$497</span>
						<span class="ml-1 text-xl text-rtp-muted">/qtr</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-emerald bg-rtp-emerald/10 p-2 rounded mt-4 text-center border border-rtp-emerald/20 font-bold"
					>
						Cost: $8.20 / Trading Day (Save 15%)
					</div>
					<ul class="mt-8 space-y-4 text-rtp-text text-sm font-medium">
						<li class="flex items-center gap-3">
							<span class="text-rtp-emerald">✓</span>
							<span class="font-bold">Priority Support</span>
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-emerald">✓</span> Live Voice & Screen
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-emerald">✓</span> Real-Time Alerts
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-emerald">✓</span> Chat Access
						</li>
					</ul>
					<a
						href="/checkout/quarterly-room"
						class="block w-full mt-8 bg-rtp-emerald text-white py-4 rounded-lg font-bold text-center hover:bg-emerald-600 transition-colors shadow-lg"
						>Join Quarterly</a
					>
				</div>

				<div
					class="bg-rtp-surface p-8 rounded-2xl shadow-lg border-2 {selectedPlan === 'annual'
						? 'border-rtp-indigo ring-2 ring-rtp-indigo/20 opacity-100 scale-100'
						: 'border-rtp-border opacity-60 scale-95'} transition-all duration-300"
				>
					<h3 class="text-xl font-bold text-rtp-text">Annual</h3>
					<div class="mt-4 flex items-baseline text-rtp-text">
						<span class="text-5xl font-extrabold tracking-tight">$1,647</span>
						<span class="ml-1 text-xl text-rtp-muted">/yr</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-indigo bg-rtp-indigo/10 p-2 rounded mt-4 text-center border border-rtp-indigo/20 font-bold"
					>
						Cost: $6.50 / Trading Day (Save 30%)
					</div>
					<ul class="mt-8 space-y-4 text-rtp-muted text-sm">
						<li class="flex items-center gap-3">
							<span class="text-rtp-indigo">✓</span> 1-on-1 Strategy Call
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-indigo">✓</span> Live Voice & Screen
						</li>
						<li class="flex items-center gap-3">
							<span class="text-rtp-indigo">✓</span> Real-Time Alerts
						</li>
					</ul>
					<a
						href="/checkout/annual-room"
						class="block w-full mt-8 bg-rtp-surface border border-rtp-indigo text-rtp-indigo py-3 rounded-lg font-bold text-center hover:bg-rtp-indigo hover:text-white transition-colors"
						>Select Annual</a
					>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-surface border-t border-rtp-border">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">
				Common Questions
			</h2>
			<div class="space-y-4">
				{#each schemaOrg['@graph'][3].mainEntity as faq, i}
					<div class="border border-rtp-border rounded-xl bg-rtp-bg overflow-hidden">
						<button
							class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-rtp-text"
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
		class="py-24 bg-gradient-to-br from-rtp-primary to-indigo-900 text-white text-center relative overflow-hidden"
	>
		<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
			<h2 class="text-3xl md:text-5xl font-heading font-extrabold mb-6">
				Market Opens at 9:30 AM ET.
			</h2>
			<p class="text-xl text-blue-100 mb-10 max-w-2xl mx-auto">
				Don't trade alone tomorrow. Join 500+ traders in the live room.
			</p>
			<a
				href="#pricing"
				class="inline-block bg-white text-rtp-primary px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-xl hover:-translate-y-1"
			>
				Secure Your Spot
			</a>
			<p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee</p>
		</div>
	</section>

	<footer class="bg-rtp-bg py-12 border-t border-rtp-border">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-xs text-rtp-muted leading-relaxed space-y-4 text-center md:text-left">
				<p class="font-bold text-rtp-text">RISK DISCLOSURE:</p>
				<p>
					Day trading options, specifically 0DTE (Zero Days to Expiration), involves a high degree
					of risk and is not suitable for all investors. You can lose more than your initial
					investment. Revolution Trading Pros is an educational platform, not a registered
					investment advisor.
				</p>
				<p class="pt-4 border-t border-rtp-border/50">
					&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
				</p>
			</div>
		</div>
	</footer>
</main>
