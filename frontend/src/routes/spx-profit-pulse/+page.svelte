<script lang="ts">
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import {
		IconChartLine,
		IconBolt,
		IconBellRinging,
		IconShoppingCart
	} from '@tabler/icons-svelte';
	import { addItemToCart } from '$lib/utils/cart-helpers';

	let headerRef: HTMLDivElement;
	let contentRef: HTMLDivElement;

	onMount(() => {
		const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

		tl.from(headerRef, {
			opacity: 0,
			y: -50,
			duration: 1,
			ease: 'back.out(1.7)'
		}).from(
			contentRef,
			{
				opacity: 0,
				y: 60,
				duration: 0.8,
				ease: 'power4.out'
			},
			'-=0.5'
		);
	});

	async function handleSignUp(interval: 'monthly' | 'quarterly' | 'yearly') {
		let price: number;

		if (interval === 'monthly') price = 199;
		else if (interval === 'quarterly') price = 549;
		else price = 1999;

		await addItemToCart(
			{
				id: 'spx-profit-pulse',
				name: 'SPX Profit Pulse',
				description: 'Premium SPX options alerts with high-probability setups',
				price,
				type: 'alert-service',
				interval
			},
			true
		);
	}
</script>

<svelte:head>
	<!-- TITLE -->
	<title>SPX Profit Pulse | SPX Options Alerts Service | Revolution Trading Pros</title>

	<!-- META DESCRIPTION -->
	<meta
		name="description"
		content="SPX Profit Pulse provides real-time SPX options alerts with precise entries, exits, targets, and risk management. Built for traders seeking high-probability SPX setups."
	/>

	<!-- CANONICAL -->
	<link rel="canonical" href="https://revolutiontradingpros.com/spx-profit-pulse/" />

	<!-- OG TAGS -->
	<meta property="og:title" content="SPX Profit Pulse | SPX Options Alerts Service" />
	<meta
		property="og:description"
		content="Get elite SPX options alerts delivered instantly with precise entry/exit levels and risk management guidance."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://revolutiontradingpros.com/spx-profit-pulse/" />
	<meta property="og:image" content="https://revolutiontradingpros.com/og-spx-profit-pulse.webp" />

	<!-- TWITTER TAGS -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="SPX Profit Pulse | SPX Options Alerts Service" />
	<meta
		name="twitter:description"
		content="Premium SPX options alerts with high-probability setups and real-time delivery."
	/>
	<meta name="twitter:image" content="https://revolutiontradingpros.com/og-spx-profit-pulse.webp" />

	<!-- INDEXING -->
	<meta name="robots" content="index, follow" />
	<meta name="viewport" content="width=device-width, initial-scale=1" />

	<!-- ⭐ ORGANIZATION SCHEMA (GLOBAL) -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "Organization",
			"name": "Revolution Trading Pros",
			"url": "https://revolutiontradingpros.com",
			"logo": "https://revolutiontradingpros.com/logo.webp",
			"telephone": "+1-203-721-0940",
			"sameAs": [
				"https://twitter.com/revtradingpros",
				"https://www.instagram.com/revolutiontradingpros"
			]
		}
	</script>

	<!-- ⭐ BREADCRUMB SCHEMA -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "BreadcrumbList",
			"itemListElement": [
				{
					"@type": "ListItem",
					"position": 1,
					"item": {
						"@id": "https://revolutiontradingpros.com/",
						"name": "Home"
					}
				},
				{
					"@type": "ListItem",
					"position": 2,
					"item": {
						"@id": "https://revolutiontradingpros.com/spx-profit-pulse/",
						"name": "SPX Profit Pulse"
					}
				}
			]
		}
	</script>

	<!-- ⭐ SERVICE + PRICING SCHEMA -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "Service",
			"name": "SPX Profit Pulse",
			"serviceType": "SPX Options Alerts Service",
			"category": "Alerts Service / Trading Alerts",
			"provider": {
				"@type": "Organization",
				"name": "Revolution Trading Pros",
				"url": "https://revolutiontradingpros.com",
				"telephone": "+1-203-721-0940"
			},
			"url": "https://revolutiontradingpros.com/spx-profit-pulse/",
			"description": "Premium SPX options alerts delivered in real-time with actionable entries, exits, and risk management.",
			"areaServed": "Worldwide",
			"offers": {
				"@type": "OfferCatalog",
				"name": "SPX Profit Pulse Pricing",
				"itemListElement": [
					{
						"@type": "Offer",
						"name": "Monthly Plan",
						"price": "199",
						"priceCurrency": "USD",
						"availability": "https://schema.org/InStock",
						"url": "https://revolutiontradingpros.com/spx-profit-pulse/"
					},
					{
						"@type": "Offer",
						"name": "Quarterly Plan",
						"price": "549",
						"priceCurrency": "USD",
						"availability": "https://schema.org/InStock",
						"url": "https://revolutiontradingpros.com/spx-profit-pulse/"
					},
					{
						"@type": "Offer",
						"name": "Yearly Plan",
						"price": "1999",
						"priceCurrency": "USD",
						"availability": "https://schema.org/InStock",
						"url": "https://revolutiontradingpros.com/spx-profit-pulse/"
					}
				]
			}
		}
	</script>

	<!-- ⭐ FAQ SCHEMA -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "FAQPage",
			"mainEntity": [
				{
					"@type": "Question",
					"name": "How are SPX alerts delivered?",
					"acceptedAnswer": {
						"@type": "Answer",
						"text": "SPX alerts are delivered instantly via SMS, email, and dashboard notifications."
					}
				},
				{
					"@type": "Question",
					"name": "What is included in each SPX alert?",
					"acceptedAnswer": {
						"@type": "Answer",
						"text": "Each alert includes entry level, stop loss, profit targets, risk rating, and real-time updates."
					}
				},
				{
					"@type": "Question",
					"name": "Is this service suitable for beginners?",
					"acceptedAnswer": {
						"@type": "Answer",
						"text": "Yes. Each alert includes clear instructions, risk management, and level-based planning suitable for newer traders."
					}
				}
			]
		}
	</script>
</svelte:head>
<!-- PAGE WRAPPER -->
<div class="spx-page min-h-screen px-4 py-12 overflow-hidden relative">
	<!-- Animated Background Layers -->
	<div class="gradient-bg absolute inset-0"></div>
	<div class="grid-overlay absolute inset-0"></div>

	<!-- Ambient Glow Orbs -->
	<div class="glow-orb glow-orb-1"></div>
	<div class="glow-orb glow-orb-2"></div>

	<!-- MAIN CONTENT WRAPPER -->
	<div class="relative max-w-7xl mx-auto z-10">

		<!-- ========================= -->
		<!--        HERO HEADER        -->
		<!-- ========================= -->
		<header class="mb-12" bind:this={headerRef}>
			<div class="flex items-center gap-4 mb-4">
				<div class="w-16 h-16 rounded-2xl bg-gradient-to-br from-orange-500 to-red-500 p-0.5 shadow-lg shadow-orange-500/50">
					<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
						<IconChartLine size={32} class="text-orange-400" />
					</div>
				</div>

				<div>
					<h1 class="text-5xl md:text-6xl font-heading font-bold bg-gradient-to-r from-orange-300 via-red-300 to-pink-300 bg-clip-text text-transparent">
						SPX Profit Pulse
					</h1>

					<p class="text-xl text-slate-300 mt-2 flex items-center gap-2">
						<IconBolt size={20} class="text-orange-400" />
						<span>Premium SPX options alerts delivered instantly</span>
					</p>
				</div>
			</div>
		</header>

		<!-- ========================= -->
		<!--     MAIN CONTENT CARD     -->
		<!-- ========================= -->
		<section bind:this={contentRef}>
			<div class="relative">
				<!-- Outer Glow -->
				<div class="absolute -inset-1 bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 rounded-3xl opacity-30 blur-2xl"></div>

				<!-- Content Body -->
				<div class="relative bg-slate-900/95 backdrop-blur-xl border border-orange-500/30 rounded-3xl p-8 md:p-12 shadow-2xl">

					<!-- ========================= -->
					<!--     SECTION: OVERVIEW     -->
					<!-- ========================= -->
					<section class="text-center mb-12">
						<div class="inline-flex items-center gap-2 px-4 py-2 bg-orange-500/20 border border-orange-500/30 rounded-full mb-6">
							<IconBellRinging size={20} class="text-orange-400" />
							<span class="text-orange-300 font-semibold">Real-Time SPX Alerts</span>
						</div>

						<h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4">
							Elite SPX Options Alerts
						</h2>

						<p class="text-lg text-slate-300 max-w-3xl mx-auto">
							Get instant notifications for high-probability SPX options trades. Our system identifies premium entry and exit points for precision execution.
						</p>
					</section>

					<!-- ========================= -->
					<!--     SECTION: FEATURES     -->
					<!-- ========================= -->
					<section class="mb-12">
						<h2 class="sr-only">SPX Profit Pulse Features</h2>

						<div class="grid md:grid-cols-2 gap-6">
							<!-- Feature 1 -->
							<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
								<h3 class="text-xl font-bold text-white mb-2">⚡ Instant Alerts</h3>
								<p class="text-slate-300">Real-time SMS and email notifications the moment trades trigger.</p>
							</div>

							<!-- Feature 2 -->
							<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
								<h3 class="text-xl font-bold text-white mb-2">📊 Complete Trade Details</h3>
								<p class="text-slate-300">Entries, exits, risk, targets, and management plans included.</p>
							</div>

							<!-- Feature 3 -->
							<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
								<h3 class="text-xl font-bold text-white mb-2">🎯 High Win Rate</h3>
								<p class="text-slate-300">Carefully selected setups with a proven performance edge.</p>
							</div>

							<!-- Feature 4 -->
							<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
								<h3 class="text-xl font-bold text-white mb-2">📱 Mobile Friendly</h3>
								<p class="text-slate-300">Optimized for traders on the go. Trade from anywhere.</p>
							</div>
						</div>
					</section>

					<!-- ========================= -->
					<!--       SECTION: PRICING    -->
					<!-- ========================= -->
					<section class="mb-16">
						<h2 class="text-center text-3xl font-heading font-bold text-white mb-10">
							Choose Your Plan
						</h2>

						<div class="grid md:grid-cols-3 gap-6">

							<!-- Monthly Plan -->
							<div class="relative">
								<div class="absolute -inset-1 bg-gradient-to-r from-orange-500 to-red-500 rounded-2xl opacity-20 blur"></div>
								<div class="relative bg-slate-800/80 backdrop-blur-xl border border-orange-500/30 rounded-2xl p-6">
									<div class="mb-6">
										<h3 class="text-xl font-bold text-white mb-2">Monthly</h3>
										<div class="flex items-baseline gap-2">
											<span class="text-4xl font-bold bg-gradient-to-r from-orange-300 to-red-300 bg-clip-text text-transparent">$199</span>
											<span class="text-slate-400 text-sm">/month</span>
										</div>
										<p class="text-xs text-slate-400 mt-2">Flexible monthly billing</p>
									</div>

									<button
										on:click={() => handleSignUp('monthly')}
										class="w-full px-6 py-3 bg-gradient-to-r from-orange-500 to-red-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-orange-500/50 transition-all duration-300 flex items-center justify-center gap-2"
									>
										<IconShoppingCart size={18} />
										<span>Sign Up</span>
									</button>
								</div>
							</div>

							<!-- Quarterly Plan -->
							<div class="relative">
								<div class="absolute -inset-1 bg-gradient-to-r from-red-500 to-pink-500 rounded-2xl opacity-25 blur"></div>
								<div class="relative bg-slate-800/80 backdrop-blur-xl border border-red-500/30 rounded-2xl p-6">
									<div class="absolute -top-3 -right-3 px-3 py-1 bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs font-bold rounded-full">
										Save 8%
									</div>

									<div class="mb-6">
										<h3 class="text-xl font-bold text-white mb-2">Quarterly</h3>
										<div class="flex items-baseline gap-2">
											<span class="text-4xl font-bold bg-gradient-to-r from-red-300 to-pink-300 bg-clip-text text-transparent">$549</span>
											<span class="text-slate-400 text-sm">/3 months</span>
										</div>
										<p class="text-xs text-slate-400 mt-2">$183/month • Save $48</p>
									</div>

									<button
										on:click={() => handleSignUp('quarterly')}
										class="w-full px-6 py-3 bg-gradient-to-r from-red-500 to-pink-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-red-500/50 transition-all duration-300 flex items-center justify-center gap-2"
									>
										<IconShoppingCart size={18} />
										<span>Sign Up</span>
									</button>
								</div>
							</div>

							<!-- Yearly Plan -->
							<div class="relative">
								<div class="absolute -inset-1 bg-gradient-to-r from-pink-500 to-rose-500 rounded-2xl opacity-30 blur"></div>
								<div class="relative bg-slate-800/80 backdrop-blur-xl border border-pink-500/30 rounded-2xl p-6">
									<div class="absolute -top-3 -right-3 px-3 py-1 bg-gradient-to-r from-orange-500 to-yellow-500 text-white text-xs font-bold rounded-full">
										Best Value
									</div>

									<div class="mb-6">
										<h3 class="text-xl font-bold text-white mb-2">Yearly</h3>
										<div class="flex items-baseline gap-2">
											<span class="text-4xl font-bold bg-gradient-to-r from-pink-300 to-rose-300 bg-clip-text text-transparent">$1,999</span>
											<span class="text-slate-400 text-sm">/year</span>
										</div>
										<p class="text-xs text-slate-400 mt-2">$166/month • Save $389</p>
									</div>

									<button
										on:click={() => handleSignUp('yearly')}
										class="w-full px-6 py-3 bg-gradient-to-r from-pink-500 to-rose-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-pink-500/50 transition-all duration-300 flex items-center justify-center gap-2"
									>
										<IconShoppingCart size={18} />
										<span>Sign Up</span>
									</button>
								</div>
							</div>

						</div>
					</section>

					<!-- ========================= -->
					<!--      SECTION: DETAILS     -->
					<!-- ========================= -->
					<section class="text-center mb-20">
						<p class="text-slate-400">
							Cancel anytime • No long-term commitments • Immediate access
						</p>
					</section>

					<!-- ========================= -->
					<!--      SECTION: FAQ         -->
					<!-- ========================= -->
					<section class="mt-20">
						<h2 class="text-3xl font-heading font-bold text-white text-center mb-10">
							Frequently Asked Questions
						</h2>

						<div class="max-w-3xl mx-auto space-y-6">

							<!-- FAQ 1 -->
							<div class="p-6 bg-slate-800/60 border border-slate-700/50 rounded-xl">
								<h3 class="text-xl font-bold text-white mb-2">How are SPX alerts delivered?</h3>
								<p class="text-slate-300">
									Alerts are delivered instantly through SMS, email, and your member dashboard.
								</p>
							</div>

							<!-- FAQ 2 -->
							<div class="p-6 bg-slate-800/60 border border-slate-700/50 rounded-xl">
								<h3 class="text-xl font-bold text-white mb-2">What is included in each alert?</h3>
								<p class="text-slate-300">
									Each alert includes exact entry level, stop loss, profit targets, and risk classification.
								</p>
							</div>

							<!-- FAQ 3 -->
							<div class="p-6 bg-slate-800/60 border border-slate-700/50 rounded-xl">
								<h3 class="text-xl font-bold text-white mb-2">Is this service beginner-friendly?</h3>
								<p class="text-slate-300">
									Yes — every alert includes clear instructions and risk guidance suitable for new traders.
								</p>
							</div>

						</div>
					</section>

				</div> <!-- END INNER CARD -->
			</div>
		</section>

	</div> <!-- END MAIN WRAPPER -->
</div>

