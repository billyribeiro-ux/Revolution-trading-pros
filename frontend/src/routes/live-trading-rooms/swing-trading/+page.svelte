<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// --- Pricing State ---
	let selectedPlan: 'monthly' | 'annual' = 'monthly';

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

	// --- SEO: STRUCTURED DATA (JSON-LD) - PRESERVED ---
	const productSchema = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: 'Explosive Swings Trading Alerts',
		description:
			'Premium multi-day swing trading alerts service. Catch 3-7 day moves with precise entry and exit signals.',
		brand: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		},
		offers: {
			'@type': 'Offer',
			priceCurrency: 'USD',
			price: selectedPlan === 'monthly' ? '97' : '927',
			availability: 'https://schema.org/InStock',
			url: 'https://revolutiontradingpros.com/alerts/explosive-swings',
			category: 'FinancialService'
		}
	};

	const faqSchema = {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: [
			{
				'@type': 'Question',
				name: 'How much capital do I need for swing trading?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: 'We recommend a minimum of $2,000 to properly manage risk, though our strategies work on accounts of all sizes.'
				}
			},
			{
				'@type': 'Question',
				name: 'Are these day trades?',
				acceptedAnswer: {
					'@type': 'Answer',
					text: 'No. These are swing trades held typically for 3-7 days, designed for traders who cannot watch the screen all day.'
				}
			}
		]
	};

	// Schema for SEOHead
	const combinedSchema = [productSchema, faqSchema];
</script>

<SEOHead
	title="Swing Trading Room | Multi-Day Trading Opportunities"
	description="Join our swing trading room for multi-day opportunities. Expert analysis, precise entries/exits, and proven strategies. Perfect for traders who can't watch markets all day. 82% win rate."
	canonical="/live-trading-rooms/swing-trading"
	ogType="product"
	ogImage="/images/og-swings.jpg"
	ogImageAlt="Swing Trading Room - Multi-Day Trading Opportunities"
	keywords={[
		'swing trading room',
		'swing trading alerts',
		'multi-day trading',
		'stock options alerts',
		'swing trade signals',
		'options swing trading',
		'trading room'
	]}
	schema={combinedSchema}
	schemaType="Product"
	productPrice={97}
	productCurrency="USD"
	productAvailability="in stock"
/>

<main
	class="w-full overflow-x-hidden bg-rtp-bg text-rtp-text font-sans selection:bg-rtp-emerald selection:text-white"
>
	<section class="relative min-h-[90vh] flex items-center overflow-hidden py-20 lg:py-0">
		<div class="absolute inset-0 bg-rtp-bg z-0">
			<div
				class="absolute inset-0 bg-[linear-gradient(rgba(16,185,129,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(16,185,129,0.03)_1px,transparent_1px)] bg-[size:48px_48px] opacity-50"
			></div>
			<div
				class="absolute top-0 right-0 w-[600px] h-[600px] bg-rtp-emerald/10 rounded-full blur-[100px] animate-pulse"
			></div>
			<div
				class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-rtp-blue/10 rounded-full blur-[120px]"
			></div>
		</div>

		<div
			class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid lg:grid-cols-2 gap-16 items-center"
		>
			<div class="text-center lg:text-left">
				<div
					use:reveal={{ delay: 0 }}
					class="inline-flex items-center gap-2 bg-rtp-surface border border-rtp-emerald/30 px-4 py-1.5 rounded-full mb-8 shadow-lg shadow-emerald-500/10 backdrop-blur-md"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
					</span>
					<span class="text-xs font-bold tracking-wider uppercase text-emerald-400"
						>Swing Signals Active</span
					>
				</div>

				<h1
					use:reveal={{ delay: 100 }}
					class="text-5xl md:text-7xl font-heading font-extrabold mb-6 leading-tight tracking-tight"
				>
					Catch the <br />
					<span
						class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-emerald via-emerald-300 to-teal-200"
						>Big Moves.</span
					>
				</h1>

				<p
					use:reveal={{ delay: 200 }}
					class="text-xl text-rtp-muted mb-10 max-w-2xl mx-auto lg:mx-0 leading-relaxed"
				>
					Stop staring at the 1-minute chart. Get high-precision multi-day swing alerts designed for
					traders who want freedom, not a job.
				</p>

				<div
					use:reveal={{ delay: 300 }}
					class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start items-center"
				>
					<a
						href="#pricing"
						class="group relative w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-white transition-all duration-200 bg-rtp-emerald rounded-xl hover:bg-emerald-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-rtp-emerald offset-rtp-bg shadow-lg hover:shadow-emerald-500/25 hover:-translate-y-1"
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
						href="#process"
						class="w-full sm:w-auto inline-flex items-center justify-center px-8 py-4 text-lg font-bold text-rtp-text transition-all duration-200 bg-rtp-surface border border-rtp-border rounded-xl hover:bg-rtp-surface/80 hover:border-rtp-emerald/30"
					>
						See How It Works
					</a>
				</div>

				<div
					use:reveal={{ delay: 400 }}
					class="mt-10 flex items-center justify-center lg:justify-start gap-6 text-sm font-medium text-rtp-muted/60"
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
						<span>Precise Entries</span>
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
						<span>3-7 Day Holds</span>
					</div>
				</div>
			</div>

			<div class="hidden lg:block relative perspective-1000">
				<div
					class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-gradient-to-tr from-rtp-emerald/20 to-transparent rounded-full blur-3xl"
				></div>

				<div
					class="relative bg-rtp-surface/90 backdrop-blur-xl border border-rtp-border/50 p-8 rounded-3xl shadow-2xl transform rotate-y-[-12deg] rotate-x-[5deg] hover:rotate-0 transition-transform duration-700 ease-out"
				>
					<div class="flex justify-between items-center mb-8">
						<div>
							<h3 class="text-2xl font-bold text-white">Swing Alert 🚀</h3>
							<p class="text-rtp-emerald text-sm font-bold">High Probability Setup</p>
						</div>
						<div
							class="bg-rtp-bg px-3 py-1 rounded-lg border border-rtp-border text-xs font-mono text-rtp-muted"
						>
							Sent: 10:30 AM
						</div>
					</div>

					<div class="space-y-6">
						<div class="bg-rtp-bg p-4 rounded-xl border-l-4 border-emerald-500">
							<div class="text-xs text-rtp-muted uppercase tracking-wider mb-1">Action</div>
							<div class="text-lg font-bold text-white flex items-center gap-2">
								<span class="bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded text-sm"
									>BUY</span
								>
								NVDA 480 CALLS
							</div>
						</div>

						<div class="grid grid-cols-2 gap-4">
							<div class="bg-rtp-bg p-4 rounded-xl border border-rtp-border/50">
								<div class="text-xs text-rtp-muted uppercase tracking-wider mb-1">Entry Zone</div>
								<div class="text-xl font-mono font-bold text-white">$5.50 - $6.00</div>
							</div>
							<div class="bg-rtp-bg p-4 rounded-xl border border-rtp-border/50">
								<div class="text-xs text-rtp-muted uppercase tracking-wider mb-1">Target</div>
								<div class="text-xl font-mono font-bold text-emerald-400">$8.50+</div>
							</div>
						</div>

						<div class="bg-rtp-bg p-4 rounded-xl border border-red-500/30">
							<div class="flex justify-between items-center">
								<div class="text-xs text-rtp-muted uppercase tracking-wider">
									Invalidation (Stop)
								</div>
								<div class="text-red-400 font-mono font-bold">$4.20 (Hard Stop)</div>
							</div>
						</div>
					</div>

					<div
						class="absolute -right-6 -bottom-6 bg-emerald-500 text-white p-4 rounded-2xl shadow-xl shadow-emerald-500/20 animate-bounce-slow"
					>
						<div class="text-xs font-bold opacity-80 uppercase">Potential Return</div>
						<div class="text-2xl font-extrabold">+45%</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="bg-rtp-surface border-y border-rtp-border relative z-20">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
			<dl class="grid grid-cols-2 md:grid-cols-4 gap-8">
				<div class="text-center">
					<dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">
						Historical Win Rate
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-rtp-emerald">82%</dd>
				</div>
				<div class="text-center">
					<dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">
						Avg Hold Time
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-rtp-primary">
						3-7<span class="text-lg text-rtp-muted font-normal ml-1">days</span>
					</dd>
				</div>
				<div class="text-center">
					<dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">
						Risk/Reward
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-rtp-indigo">4:1</dd>
				</div>
				<div class="text-center">
					<dt class="text-rtp-muted font-medium text-xs uppercase tracking-wider mb-2">
						Alerts Per Week
					</dt>
					<dd class="text-4xl md:text-5xl font-extrabold text-rtp-blue">2-4</dd>
				</div>
			</dl>
		</div>
	</section>

	<section class="py-24 bg-rtp-bg relative overflow-hidden">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 use:reveal class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					Choose Your Lifestyle
				</h2>
				<p use:reveal={{ delay: 100 }} class="text-xl text-rtp-muted max-w-2xl mx-auto">
					Most traders burn out scalping 1-minute candles. We play the bigger timeframe.
				</p>
			</div>

			<div class="grid md:grid-cols-2 gap-8 max-w-5xl mx-auto">
				<div
					use:reveal={{ delay: 0 }}
					class="bg-rtp-surface/50 border border-rtp-border rounded-3xl p-10 opacity-70 hover:opacity-100 transition-opacity"
				>
					<div class="flex items-center gap-4 mb-6">
						<div
							class="w-12 h-12 rounded-full bg-rtp-border flex items-center justify-center text-2xl"
						>
							😰
						</div>
						<h3 class="text-2xl font-bold text-rtp-muted">Day Scalper</h3>
					</div>
					<ul class="space-y-4 text-rtp-muted">
						<li class="flex items-center gap-3">
							<span class="text-red-400">✕</span> Glued to screen 6 hours/day
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-400">✕</span> High stress, high cortisol
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-400">✕</span> Expensive commissions
						</li>
						<li class="flex items-center gap-3">
							<span class="text-red-400">✕</span> "Did I miss the move?" anxiety
						</li>
					</ul>
				</div>

				<div
					use:reveal={{ delay: 150 }}
					class="bg-rtp-surface border-2 border-rtp-emerald rounded-3xl p-10 shadow-2xl shadow-emerald-500/10 relative overflow-hidden"
				>
					<div
						class="absolute top-0 right-0 bg-rtp-emerald text-white text-xs font-bold px-3 py-1 rounded-bl-xl"
					>
						RECOMMENDED
					</div>
					<div class="flex items-center gap-4 mb-6">
						<div
							class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center text-2xl"
						>
							🧘‍♂️
						</div>
						<h3 class="text-2xl font-bold text-rtp-text">Swing Trader</h3>
					</div>
					<ul class="space-y-4 text-rtp-text font-medium">
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
							> Check charts once a day
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
							> Calm, calculated decisions
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
							> Catch the meat of the move (20%+)
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
							> Keep your day job
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<section id="process" class="py-24 bg-rtp-surface border-t border-rtp-border">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="grid md:grid-cols-3 gap-10">
				<article use:reveal={{ delay: 0 }} class="group">
					<div
						class="w-14 h-14 bg-rtp-emerald/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-rtp-emerald/20"
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
								d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-rtp-text mb-3">Instituional Analysis</h3>
					<p class="text-rtp-muted leading-relaxed">
						We track dark pool prints and institutional flow to identify stocks about to break out.
						We ride the whale's wake.
					</p>
				</article>

				<article use:reveal={{ delay: 100 }} class="group">
					<div
						class="w-14 h-14 bg-rtp-primary/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-rtp-primary/20"
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
								d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-rtp-text mb-3">SMS & Push Alerts</h3>
					<p class="text-rtp-muted leading-relaxed">
						You can't miss the entry. We send alerts via SMS, Email, and Discord immediately when
						our criteria are met.
					</p>
				</article>

				<article use:reveal={{ delay: 200 }} class="group">
					<div
						class="w-14 h-14 bg-rtp-indigo/10 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300 border border-rtp-indigo/20"
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
								d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
							/></svg
						>
					</div>
					<h3 class="text-xl font-bold text-rtp-text mb-3">Risk-First Approach</h3>
					<p class="text-rtp-muted leading-relaxed">
						We hate losing money. Every trade comes with a predefined "Hard Stop" level. We cut
						losers fast and let winners run.
					</p>
				</article>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-bg">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="flex justify-between items-end mb-10">
				<div>
					<h2 class="text-3xl font-heading font-bold text-rtp-text mb-2">Recent Swings</h2>
					<p class="text-rtp-muted">Real trades. Real timestamps. Verified results.</p>
				</div>
			</div>

			<div class="bg-rtp-surface rounded-2xl border border-rtp-border overflow-hidden shadow-xl">
				<div
					class="grid grid-cols-12 bg-rtp-surface border-b border-rtp-border p-4 text-xs font-bold uppercase text-rtp-muted tracking-wider"
				>
					<div class="col-span-3 md:col-span-2">Ticker</div>
					<div class="col-span-3 md:col-span-2">Type</div>
					<div class="col-span-3 md:col-span-2">Days Held</div>
					<div class="col-span-3 md:col-span-2 text-right">Return</div>
					<div class="hidden md:block md:col-span-4 text-right">Notes</div>
				</div>
				<div class="divide-y divide-rtp-border/50 font-mono text-sm">
					<div class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors">
						<div class="col-span-3 md:col-span-2 font-bold text-white">NVDA</div>
						<div class="col-span-3 md:col-span-2 text-emerald-400">CALLS</div>
						<div class="col-span-3 md:col-span-2 text-rtp-muted">5 Days</div>
						<div class="col-span-3 md:col-span-2 text-right text-emerald-400 font-bold">+125%</div>
						<div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
							Breakout over $480 level.
						</div>
					</div>
					<div class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors">
						<div class="col-span-3 md:col-span-2 font-bold text-white">AMD</div>
						<div class="col-span-3 md:col-span-2 text-emerald-400">CALLS</div>
						<div class="col-span-3 md:col-span-2 text-rtp-muted">3 Days</div>
						<div class="col-span-3 md:col-span-2 text-right text-emerald-400 font-bold">+45%</div>
						<div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
							Sector rotation play.
						</div>
					</div>
					<div
						class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors bg-red-500/5"
					>
						<div class="col-span-3 md:col-span-2 font-bold text-white">TSLA</div>
						<div class="col-span-3 md:col-span-2 text-red-400">PUTS</div>
						<div class="col-span-3 md:col-span-2 text-rtp-muted">1 Day</div>
						<div class="col-span-3 md:col-span-2 text-right text-red-400 font-bold">-15%</div>
						<div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
							Hit stop loss on reversal.
						</div>
					</div>
					<div class="grid grid-cols-12 p-5 items-center hover:bg-white/5 transition-colors">
						<div class="col-span-3 md:col-span-2 font-bold text-white">META</div>
						<div class="col-span-3 md:col-span-2 text-emerald-400">CALLS</div>
						<div class="col-span-3 md:col-span-2 text-rtp-muted">7 Days</div>
						<div class="col-span-3 md:col-span-2 text-right text-emerald-400 font-bold">+82%</div>
						<div class="hidden md:block md:col-span-4 text-right text-rtp-muted text-xs">
							Earnings run-up swing.
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="pricing" class="py-24 bg-rtp-surface border-t border-rtp-border">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					Simple Pricing
				</h2>
				<p class="text-xl text-rtp-muted max-w-3xl mx-auto">
					Pay for the subscription with your first successful swing trade.
				</p>
			</div>

			<div class="flex justify-center mb-16">
				<div class="bg-rtp-bg p-1.5 rounded-xl border border-rtp-border inline-flex relative">
					<button
						type="button"
						on:click={() => (selectedPlan = 'monthly')}
						class="relative z-10 px-8 py-3 rounded-lg font-bold text-sm md:text-base transition-colors duration-200 {selectedPlan ===
						'monthly'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}"
					>
						Monthly
					</button>
					<button
						type="button"
						on:click={() => (selectedPlan = 'annual')}
						class="relative z-10 px-8 py-3 rounded-lg font-bold text-sm md:text-base transition-colors duration-200 {selectedPlan ===
						'annual'
							? 'text-white'
							: 'text-rtp-muted hover:text-white'}"
					>
						Annual
					</button>

					<div
						class="absolute top-1.5 bottom-1.5 bg-rtp-emerald rounded-lg shadow-lg shadow-emerald-500/20 transition-all duration-300 ease-out"
						style="left: {selectedPlan === 'monthly'
							? '0.375rem'
							: '50%'}; width: calc(50% - 0.375rem);"
					></div>
				</div>
			</div>

			<div class="max-w-lg mx-auto">
				<div class="relative min-h-[550px]">
					{#key selectedPlan}
						<div in:fade={{ duration: 300 }} class="absolute inset-0">
							{#if selectedPlan === 'monthly'}
								<div
									class="bg-rtp-bg p-8 md:p-10 rounded-3xl shadow-2xl border border-rtp-border h-full flex flex-col"
								>
									<div class="text-center mb-8">
										<h3 class="text-2xl font-bold text-rtp-text mb-2">Monthly Pass</h3>
										<div class="flex items-baseline justify-center gap-1">
											<div class="text-6xl font-extrabold text-rtp-text">$97</div>
											<div class="text-rtp-muted font-medium">/mo</div>
										</div>
										<p class="text-rtp-muted text-sm mt-4">Flexibility to cancel anytime.</p>
									</div>
									<ul class="space-y-5 mb-10 flex-grow text-sm md:text-base">
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">2-4 Premium Swings / Week</span>
										</li>
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">Instant SMS & Email Alerts</span>
										</li>
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">Private Discord Community</span>
										</li>
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">Detailed Technical Analysis</span>
										</li>
									</ul>
									<a
										href="/checkout/monthly-swings"
										class="w-full bg-rtp-surface border border-rtp-emerald text-emerald-500 py-4 rounded-xl font-bold text-lg hover:bg-emerald-500 hover:text-white transition-all text-center block"
									>
										Subscribe Monthly
									</a>
								</div>
							{:else}
								<div
									class="bg-rtp-bg p-8 md:p-10 rounded-3xl shadow-2xl border-2 border-rtp-emerald h-full flex flex-col relative overflow-hidden"
								>
									<div
										class="absolute top-0 right-0 bg-rtp-emerald text-white px-4 py-1 rounded-bl-xl font-bold text-xs uppercase tracking-wide"
									>
										Save 20%
									</div>
									<div class="text-center mb-8">
										<h3 class="text-2xl font-bold text-rtp-text mb-2">Annual Access</h3>
										<div class="flex items-baseline justify-center gap-1">
											<div class="text-6xl font-extrabold text-rtp-emerald">$927</div>
											<div class="text-rtp-muted font-medium">/yr</div>
										</div>
										<p class="text-emerald-500 font-bold text-sm mt-4">
											Like getting 2.5 months FREE
										</p>
									</div>
									<ul class="space-y-5 mb-10 flex-grow text-sm md:text-base">
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">Everything in Monthly</span>
										</li>
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text font-bold text-emerald-500"
												>Save $237 Instantly</span
											>
										</li>
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">Priority Support</span>
										</li>
										<li class="flex items-center gap-3">
											<div class="bg-emerald-500/20 p-1 rounded-full">
												<svg
													class="w-4 h-4 text-emerald-500"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
													><path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="3"
														d="M5 13l4 4L19 7"
													/></svg
												>
											</div>
											<span class="text-rtp-text">Exclusive Strategy Video Library</span>
										</li>
									</ul>
									<a
										href="/checkout/annual-swings"
										class="w-full bg-rtp-emerald text-white py-4 rounded-xl font-bold text-lg hover:bg-emerald-600 transition-all shadow-lg shadow-emerald-500/25 text-center block"
									>
										Join Annual
									</a>
								</div>
							{/if}
						</div>
					{/key}
				</div>
			</div>
			<p class="text-center text-rtp-muted text-sm mt-12 flex items-center justify-center gap-2">
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
					/></svg
				>
				30-Day Money Back Guarantee. Cancel anytime in 1-click.
			</p>
		</div>
	</section>

	<section class="py-20 bg-rtp-bg">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">
				Frequently Asked Questions
			</h2>
			<div class="space-y-4">
				{#each faqSchema.mainEntity as faq, i}
					<div class="border border-rtp-border rounded-xl bg-rtp-surface overflow-hidden">
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
		class="py-24 bg-gradient-to-br from-rtp-emerald to-teal-800 text-white relative overflow-hidden"
	>
		<div class="absolute inset-0 bg-[url('/grid-pattern.svg')] opacity-10"></div>
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
			<h2 class="text-4xl md:text-6xl font-heading font-extrabold mb-6 tracking-tight">
				Stop Overtrading. Start Swinging.
			</h2>
			<p class="text-xl text-emerald-100 mb-10 max-w-2xl mx-auto">
				Join the trading room that values your time as much as your capital.
			</p>
			<a
				href="#pricing"
				class="inline-block bg-white text-emerald-700 px-10 py-5 rounded-xl font-bold text-lg hover:bg-emerald-50 transition-all shadow-2xl hover:-translate-y-1"
			>
				Get Instant Access
			</a>
			<p class="mt-6 text-sm text-emerald-100/70">Secure Checkout • Cancel Anytime</p>
		</div>
	</section>

	<footer class="bg-rtp-bg py-12 border-t border-rtp-border">
		<div
			class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-xs text-rtp-muted leading-relaxed"
		>
			<p class="mb-4 font-bold uppercase text-rtp-text">Risk Disclosure</p>
			<p class="max-w-4xl mx-auto">
				Trading in financial markets involves a high degree of risk and may not be suitable for all
				investors. You could lose some or all of your initial investment; do not invest money that
				you cannot afford to lose. Past performance is not indicative of future results. Revolution
				Trading Pros is an educational platform and does not provide personalized financial advice.
			</p>
			<p class="mt-8 opacity-50">
				&copy; {new Date().getFullYear()} Revolution Trading Pros. All rights reserved.
			</p>
		</div>
	</footer>
</main>
