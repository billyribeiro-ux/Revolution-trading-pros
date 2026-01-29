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

	// --- DATA SOURCE: FAQs (Single Source of Truth) ---
	const faqList = [
		{
			question: 'Do I need $25,000 to join this room?',
			answer:
				'No. This room is specifically designed for accounts under $25k. We teach you how to use Cash Accounts (which settle T+1 for options) to trade daily without triggering the Pattern Day Trader (PDT) rule.'
		},
		{
			question: 'What is the minimum recommended capital?',
			answer:
				'We strictly recommend a minimum of $2,000. While you can technically start with less, $2,000 allows you to trade micro-contracts or single option contracts while adhering to proper risk management rules (risking only 1-2% per trade).'
		},
		{
			question: 'How do you avoid the PDT rule?',
			answer:
				'By using a Cash Account rather than a Margin Account. In a Cash Account, options trades settle the next business day (T+1). This means you can trade your entire account balance every single day without restriction, as long as the cash has settled.'
		},
		{
			question: 'What specific instruments do you trade?',
			answer:
				'We focus on SPX (S&P 500 Index) 0DTE options. SPX offers favorable tax treatment (Section 1256), cash settlement (no assignment risk), and massive liquidity, making it ideal for scalping small accounts.'
		},
		{
			question: 'Is this room beginner friendly?',
			answer:
				'Yes. We assume you are learning. Our "New Member" onboarding covers how to set up your platform, how to read an options chain, and how to execute orders quickly. We focus heavily on risk habits.'
		},
		{
			question: 'Do you trade Futures (ES/NQ)?',
			answer:
				'Yes. For members who prefer Futures, we analyze ES and NQ. Futures accounts also do not have the PDT rule, making them another excellent vehicle for small account growth.'
		},
		{
			question: 'What is the win rate?',
			answer:
				'Win rates fluctuate with market conditions. Our strategy targets a 70%+ win rate with a 1:2 risk/reward ratio. However, in small account trading, risk management (keeping losers small) is more important than win rate.'
		},
		{
			question: 'Can I trade on mobile?',
			answer:
				"Yes, but we recommend a desktop or laptop for the best experience. You can listen to the live voice commentary and see the screen share on the Discord mobile app while executing trades on your broker's app."
		},
		{
			question: 'How many trades do you take per day?',
			answer:
				'Quality over quantity. We typically take 2-4 high-conviction trades per day, usually within the first 2 hours of the market open (9:30 AM - 11:30 AM EST).'
		},
		{
			question: 'What happens if I blow up my account?',
			answer:
				'We aim to prevent this. Our strict risk rules (hard stops) are designed to keep you in the game. If you are struggling, we offer mentorship channels to review your trades and correct bad habits before capital is lost.'
		}
	];

	// --- SEO: STRUCTURED DATA (JSON-LD) ---
	const productSchema = {
		'@context': 'https://schema.org',
		'@type': 'Product',
		name: 'Small Accounts Day Trading Room',
		description:
			'Live trading room designed for growing small accounts (under $25k). Learn professional SPX 0DTE strategies optimized for cash accounts to avoid PDT restrictions.',
		brand: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		},
		aggregateRating: {
			'@type': 'AggregateRating',
			ratingValue: '4.9',
			reviewCount: '184'
		},
		offers: {
			'@type': 'AggregateOffer',
			priceCurrency: 'USD',
			lowPrice: '97',
			highPrice: '1647',
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
	title="Small Accounts Day Trading Room | Grow Accounts Under $25k"
	description="The #1 live trading room for small accounts. Learn to trade SPX 0DTE options without PDT restrictions. Live screen share, real-time alerts, and risk management."
	canonical="/live-trading-rooms/small-accounts"
	ogType="product"
	ogImage="/images/day-trading-og.jpg"
	ogImageAlt="Small Accounts Day Trading Room - PDT-Free Strategies"
	keywords={[
		'small account day trading',
		'how to trade under 25k',
		'PDT workaround strategies',
		'SPX 0DTE for small accounts',
		'cash account options trading',
		'live trading room discord',
		'growing a small trading account'
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
					data-gsap
					class="inline-flex items-center gap-2 bg-rtp-surface border border-rtp-border px-4 py-1.5 rounded-full mb-8 shadow-lg shadow-emerald-500/10 backdrop-blur-md cursor-default hover:border-emerald-500/50 transition-colors"
				>
					<span class="relative flex h-2.5 w-2.5">
						<span
							class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500"></span>
					</span>
					<span class="text-xs font-bold tracking-wide uppercase text-emerald-400"
						>Market Active • Live Commentary On</span
					>
				</div>

				<h1
					data-gsap
					class="text-4xl md:text-6xl font-heading font-extrabold mb-6 leading-tight tracking-tight"
				>
					Master <span
						class="text-transparent bg-clip-text bg-gradient-to-r from-rtp-primary to-emerald-400"
						>0DTE Options</span
					> <br />Without The PDT Rule.
				</h1>

				<p data-gsap class="text-xl text-rtp-muted mb-8 max-w-xl mx-auto lg:mx-0 leading-relaxed">
					Don't just get alerts—learn the execution. Join our live voice & screen-share room where
					we hunt high-probability setups on SPX optimized for <strong
						>accounts under $25,000.</strong
					>
				</p>

				<div data-gsap class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
					<a
						href="#pricing"
						class="bg-rtp-primary text-white px-8 py-4 rounded-xl font-bold text-lg hover:bg-blue-600 transition-all shadow-lg shadow-rtp-primary/25 hover:-translate-y-1 flex items-center justify-center gap-2"
					>
						Join the Live Room
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 7l5 5m0 0l-5 5m5-5H6"
							/></svg
						>
					</a>
					<a
						href="#daily-routine"
						class="bg-rtp-surface border border-rtp-border text-rtp-text px-8 py-4 rounded-xl font-bold text-lg hover:bg-rtp-bg hover:border-rtp-primary/30 transition-all"
					>
						See Daily Routine
					</a>
				</div>

				<div
					data-gsap
					class="mt-8 flex items-center justify-center lg:justify-start gap-4 text-sm text-rtp-muted font-medium"
				>
					<div class="flex -space-x-2">
						<div
							class="w-8 h-8 rounded-full bg-gray-800 border-2 border-rtp-bg bg-cover"
							style="background-image: url('/avatars/1.svg')"
						></div>
						<div
							class="w-8 h-8 rounded-full bg-gray-700 border-2 border-rtp-bg bg-cover"
							style="background-image: url('/avatars/2.svg')"
						></div>
						<div
							class="w-8 h-8 rounded-full bg-rtp-primary border-2 border-rtp-bg flex items-center justify-center text-white text-xs font-bold"
						>
							+500
						</div>
					</div>
					<p>Traders currently active</p>
				</div>
			</div>

			<div class="hidden lg:block relative perspective-1000">
				<div
					class="absolute inset-0 bg-gradient-to-tr from-rtp-primary/20 to-transparent rounded-full blur-3xl transform translate-y-10"
				></div>

				<div
					class="relative transform rotate-y-[-5deg] hover:rotate-0 transition-transform duration-700 ease-[cubic-bezier(0.23,1,0.32,1)]"
				>
					<div class="rounded-2xl overflow-hidden border border-rtp-border shadow-2xl bg-[#1a1b26]">
						[Image of live trading software interface]
					</div>

					<div
						class="absolute -bottom-6 -left-6 bg-rtp-surface/95 backdrop-blur-xl border border-rtp-border p-4 rounded-xl shadow-2xl animate-bounce-slow"
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
								<div class="text-[10px] text-rtp-muted uppercase font-bold tracking-wider">
									Latest Call
								</div>
								<div class="text-white font-bold font-mono">SPX 4450 Calls @ $2.50</div>
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
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-primary mb-2 group-hover:scale-110 transition-transform"
					>
						500+
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Active Members
					</div>
				</div>
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-emerald mb-2 group-hover:scale-110 transition-transform"
					>
						9:30AM
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Live Bell-to-Bell
					</div>
				</div>
				<div class="text-center group cursor-default">
					<div
						class="text-3xl md:text-4xl font-bold text-rtp-indigo mb-2 group-hover:scale-110 transition-transform"
					>
						1000+
					</div>
					<div class="text-rtp-muted font-medium uppercase text-xs tracking-wider">
						Setups Called
					</div>
				</div>
				<div class="text-center group cursor-default">
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

	<section class="py-24 bg-rtp-bg">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="grid lg:grid-cols-2 gap-16 items-center">
				<div>
					<span class="text-emerald-500 font-bold uppercase tracking-wider text-sm mb-2 block"
						>The Methodology</span
					>
					<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
						How We Trade Under $25k
					</h2>
					<p class="text-lg text-rtp-muted mb-6 leading-relaxed">
						The Pattern Day Trader (PDT) rule restricts margin accounts under $25k to only 3 day
						trades per week. <strong>We solve this.</strong>
					</p>
					<ul class="space-y-6">
						<li class="flex gap-4">
							<div
								class="w-12 h-12 rounded-full bg-rtp-surface border border-rtp-border flex items-center justify-center shrink-0"
							>
								<span class="text-emerald-500 font-bold">01</span>
							</div>
							<div>
								<h3 class="text-white font-bold text-lg">Cash Account Mastery</h3>
								<p class="text-rtp-muted text-sm mt-1">
									We teach you how to utilize Cash Accounts. Options settle T+1 (next day), meaning
									you can reuse your cash daily without hitting PDT limits.
								</p>
							</div>
						</li>
						<li class="flex gap-4">
							<div
								class="w-12 h-12 rounded-full bg-rtp-surface border border-rtp-border flex items-center justify-center shrink-0"
							>
								<span class="text-emerald-500 font-bold">02</span>
							</div>
							<div>
								<h3 class="text-white font-bold text-lg">SPX 0DTE Leverage</h3>
								<p class="text-rtp-muted text-sm mt-1">
									The S&P 500 index offers massive liquidity. We scalp quick moves (10-30 minutes)
									using 0DTE options that offer high percentage returns on small capital.
								</p>
							</div>
						</li>
						<li class="flex gap-4">
							<div
								class="w-12 h-12 rounded-full bg-rtp-surface border border-rtp-border flex items-center justify-center shrink-0"
							>
								<span class="text-emerald-500 font-bold">03</span>
							</div>
							<div>
								<h3 class="text-white font-bold text-lg">Strict Position Sizing</h3>
								<p class="text-rtp-muted text-sm mt-1">
									We never bet the farm. We teach you to risk strictly 1-2% of your account per
									trade, ensuring you survive the learning curve.
								</p>
							</div>
						</li>
					</ul>
				</div>
				<div class="relative">
					<div
						class="absolute inset-0 bg-gradient-to-r from-emerald-500/20 to-blue-500/20 rounded-3xl blur-2xl"
					></div>
					<div class="relative bg-rtp-surface border border-rtp-border rounded-2xl p-8 shadow-2xl">
						[Image of trading chart with support and resistance lines]
						<div
							class="mt-4 bg-rtp-bg p-4 rounded-xl border border-rtp-border/50 flex justify-between items-center"
						>
							<div>
								<div class="text-xs text-rtp-muted uppercase">Account Balance</div>
								<div class="text-xl font-mono text-white">$2,450.00</div>
							</div>
							<div class="text-right">
								<div class="text-xs text-rtp-muted uppercase">Today's P&L</div>
								<div class="text-xl font-mono text-emerald-400">+$185.00 (+7.5%)</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section id="inside-the-room" class="py-24 bg-rtp-surface border-y border-rtp-border">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-6">
					Inside The Room
				</h2>
				<p data-gsap class="text-xl text-rtp-muted max-w-3xl mx-auto">
					We don't just give signals. We teach you how to fish with institutional-grade tools.
				</p>
			</div>

			<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
				<div
					data-gsap
					class="bg-rtp-bg p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-primary transition-all duration-300 group hover:-translate-y-1"
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
						zones before the trade happens. No "after the fact" hindsight.
					</p>
				</div>

				<div
					data-gsap
					class="bg-rtp-bg p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-emerald transition-all duration-300 group hover:-translate-y-1"
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
						Trim, and Exit. Trade from anywhere with confidence.
					</p>
				</div>

				<div
					data-gsap
					class="bg-rtp-bg p-8 rounded-2xl shadow-sm border border-rtp-border hover:border-rtp-indigo transition-all duration-300 group hover:-translate-y-1"
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
						Money" is positioning. We don't guess; we follow the volume.
					</p>
				</div>
			</div>
		</div>
	</section>

	<section id="daily-routine" class="py-24 bg-rtp-bg border-y border-rtp-border">
		<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-16">
				<span class="text-rtp-primary font-bold uppercase tracking-wider text-sm mb-2 block"
					>Structure</span
				>
				<h2 data-gsap class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					Your Daily Routine
				</h2>
				<p class="text-xl text-rtp-muted">Consistency is the key to longevity. Here is the plan.</p>
			</div>

			<div
				class="space-y-8 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-rtp-border before:to-transparent"
			>
				<div
					data-gsap
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
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-surface rounded-xl border border-rtp-border shadow-sm"
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
					data-gsap
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
					data-gsap
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
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-surface rounded-xl border border-rtp-border shadow-sm"
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
					data-gsap
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
						class="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-6 bg-rtp-surface rounded-xl border border-rtp-border shadow-sm"
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
				<span class="text-rtp-primary font-bold uppercase tracking-wider text-sm mb-2 block"
					>Investment</span
				>
				<h2 class="text-3xl md:text-5xl font-heading font-bold text-rtp-text mb-4">
					Simple Membership
				</h2>
				<p class="text-xl text-rtp-muted max-w-2xl mx-auto">
					Invest in your education. One good trade covers your monthly access.
				</p>
			</div>

			<div class="flex justify-center mb-16">
				<div
					class="bg-rtp-surface p-1.5 rounded-xl border border-rtp-border inline-flex relative shadow-inner"
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
					class="bg-rtp-surface p-8 rounded-2xl border transition-all duration-300 {selectedPlan ===
					'monthly'
						? 'border-rtp-primary opacity-100 scale-105 shadow-lg shadow-rtp-primary/10'
						: 'border-rtp-border opacity-70 hover:opacity-100'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Monthly</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$197</span>
						<span class="text-rtp-muted">/mo</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-muted bg-rtp-bg p-2 rounded mb-6 text-center border border-rtp-border"
					>
						$9.85 / trading day
					</div>
					<ul class="space-y-4 mb-8 text-sm text-rtp-muted">
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Live Voice & Screen</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Real-time SPX Alerts</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Chat Access</li>
						<li class="flex gap-3"><span class="text-rtp-primary">✓</span> Small Account Guide</li>
					</ul>
					<a
						href="/checkout/monthly-room"
						class="block w-full py-3 bg-rtp-bg border border-rtp-border text-white font-bold rounded-lg text-center hover:bg-white hover:text-black transition-colors"
						>Select Monthly</a
					>
				</div>

				<div
					class="bg-rtp-bg p-10 rounded-3xl border-2 shadow-2xl transform relative z-10 transition-all duration-300 {selectedPlan ===
					'quarterly'
						? 'border-rtp-emerald shadow-rtp-emerald/20 md:scale-110 opacity-100'
						: 'border-rtp-border shadow-rtp-border/10 md:scale-100 opacity-70 hover:opacity-100'}"
				>
					<div
						class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-rtp-emerald text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wide shadow-lg"
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
							<span class="text-rtp-emerald font-bold">✓</span>
							<span class="font-bold">Priority Support</span>
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-emerald font-bold">✓</span> Live Voice & Screen
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-emerald font-bold">✓</span> Real-time SPX Alerts
						</li>
						<li class="flex gap-3">
							<span class="text-rtp-emerald font-bold">✓</span> Small Account Guide
						</li>
					</ul>
					<a
						href="/checkout/quarterly-room"
						class="block w-full py-4 bg-rtp-emerald text-white font-bold rounded-xl text-center hover:bg-emerald-600 transition-colors shadow-lg hover:shadow-emerald-500/50"
						>Join Quarterly</a
					>
				</div>

				<div
					class="bg-rtp-surface p-8 rounded-2xl border transition-all duration-300 {selectedPlan ===
					'annual'
						? 'border-rtp-indigo opacity-100 scale-105 shadow-lg shadow-rtp-indigo/10'
						: 'border-rtp-border opacity-70 hover:opacity-100'}"
				>
					<h3 class="text-xl font-bold text-white mb-4">Annual</h3>
					<div class="flex items-baseline gap-1 mb-6">
						<span class="text-4xl font-bold text-white">$1,647</span>
						<span class="text-rtp-muted">/yr</span>
					</div>
					<div
						class="text-xs font-mono text-rtp-indigo bg-rtp-bg p-2 rounded mb-6 text-center border border-rtp-border"
					>
						Save 30% ($6.50 / trading day)
					</div>
					<ul class="space-y-4 mb-8 text-sm text-rtp-muted">
						<li class="flex gap-3">
							<span class="text-rtp-indigo">✓</span>
							<span class="font-bold">1-on-1 Coaching Call</span>
						</li>
						<li class="flex gap-3"><span class="text-rtp-indigo">✓</span> Live Voice & Screen</li>
						<li class="flex gap-3"><span class="text-rtp-indigo">✓</span> Real-time Alerts</li>
						<li class="flex gap-3"><span class="text-rtp-indigo">✓</span> Chat Access</li>
					</ul>
					<a
						href="/checkout/annual-room"
						class="block w-full py-3 bg-rtp-bg border border-rtp-indigo text-rtp-indigo font-bold rounded-lg text-center hover:bg-rtp-indigo hover:text-white transition-colors"
						>Select Annual</a
					>
				</div>
			</div>

			<div class="mt-12 text-center">
				<p class="text-rtp-muted text-sm mb-4">
					Secure checkout powered by Stripe. Cancel anytime.
				</p>
				<div
					class="flex items-center justify-center gap-2 text-rtp-muted text-sm bg-rtp-surface inline-flex px-4 py-2 rounded-full border border-rtp-border"
				>
					<svg
						class="w-4 h-4 text-emerald-500"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
						></path></svg
					>
					<span>30-Day Money Back Guarantee. Zero risk to try.</span>
				</div>
			</div>
		</div>
	</section>

	<section class="py-24 bg-rtp-surface border-t border-rtp-border">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
			<h2 class="text-3xl font-heading font-bold text-center text-rtp-text mb-12">
				Frequently Asked Questions
			</h2>
			<div class="space-y-4">
				{#each faqList as faq, i}
					<div
						class="border border-rtp-border rounded-xl bg-rtp-bg overflow-hidden hover:border-rtp-primary/30 transition-colors"
					>
						<button
							class="w-full text-left px-6 py-5 font-bold flex justify-between items-center focus:outline-none hover:bg-white/5 transition-colors text-rtp-text"
							onclick={() => toggleFaq(i)}
							aria-expanded={openFaq === i}
						>
							<span class="text-base pr-4">{faq.question}</span>
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
								{faq.answer}
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
				Don't trade alone tomorrow. Join 500+ traders in the live room and start growing your small
				account correctly.
			</p>
			<a
				href="#pricing"
				class="inline-block bg-white text-rtp-primary px-10 py-4 rounded-xl font-bold text-lg hover:bg-blue-50 transition-all shadow-xl hover:-translate-y-1"
			>
				Secure Your Spot
			</a>
			<p class="mt-8 text-sm text-white/60">30-Day Money Back Guarantee • Cancel Anytime</p>
		</div>
	</section>
</div>

<MarketingFooter />
