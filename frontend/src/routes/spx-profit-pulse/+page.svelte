<script lang="ts">
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import { IconChartLine, IconBolt, IconBellRinging, IconShoppingCart } from '@tabler/icons-svelte';
	import { addItemToCart } from '$lib/utils/cart-helpers';

	let headerRef: HTMLDivElement;
	let contentRef: HTMLDivElement;

	onMount(() => {
		initAnimations();
	});

	function initAnimations() {
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
	}

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

<!-- ⭐⭐⭐ FULL ENTERPRISE SEO BLOCK ⭐⭐⭐ -->
<svelte:head>
	<title>SPX Profit Pulse | Premium SPX Options Alerts | Revolution Trading Pros</title>

	<meta
		name="description"
		content="SPX Profit Pulse delivers premium SPX options alerts in real time. Get instant notifications with precise entries, exits, targets, and risk guidance for high-probability SPX trades."
	/>

	<!-- Canonical -->
	<link rel="canonical" href="https://revolutiontradingpros.com/spx-profit-pulse" />

	<!-- Open Graph -->
	<meta property="og:title" content="SPX Profit Pulse | Premium SPX Options Alerts" />
	<meta
		property="og:description"
		content="Get high-probability SPX options trade alerts with entries, exits, targets, and real-time delivery. Built for serious traders."
	/>
	<meta property="og:type" content="product" />
	<meta property="og:url" content="https://revolutiontradingpros.com/spx-profit-pulse" />
	<meta property="og:image" content="https://revolutiontradingpros.com/og-spx-profit-pulse.webp" />

	<!-- Twitter -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="SPX Profit Pulse | Premium SPX Options Alerts" />
	<meta
		name="twitter:description"
		content="Elite SPX options alerts delivered instantly. High-probability setups, precise entries, exits, and risk management."
	/>
	<meta name="twitter:image" content="https://revolutiontradingpros.com/og-spx-profit-pulse.webp" />

	<!-- Robots + Viewport -->
	<meta name="viewport" content="width=device-width, initial-scale=1" />
	<meta name="robots" content="index, follow" />

	<!-- JSON-LD Product / AlertService Schema -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "Product",
			"name": "SPX Profit Pulse",
			"category": "FinancialProduct",
			"description": "Premium SPX options alerts with real-time delivery, high-probability setups, and precise trade guidance.",
			"brand": {
				"@type": "Organization",
				"name": "Revolution Trading Pros",
				"url": "https://revolutiontradingpros.com"
			},
			"url": "https://revolutiontradingpros.com/spx-profit-pulse",
			"image": "https://revolutiontradingpros.com/og-spx-profit-pulse.webp",
			"offers": [
				{
					"@type": "Offer",
					"price": "199",
					"priceCurrency": "USD",
					"availability": "https://schema.org/InStock",
					"category": "monthly"
				},
				{
					"@type": "Offer",
					"price": "549",
					"priceCurrency": "USD",
					"availability": "https://schema.org/InStock",
					"category": "quarterly"
				},
				{
					"@type": "Offer",
					"price": "1999",
					"priceCurrency": "USD",
					"availability": "https://schema.org/InStock",
					"category": "yearly"
				}
			]
		}
	</script>

	<!-- Breadcrumb Schema -->
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
						"@id": "https://revolutiontradingpros.com/spx-profit-pulse",
						"name": "SPX Profit Pulse"
					}
				}
			]
		}
	</script>
</svelte:head>
<!-- ⭐⭐⭐ END SEO ⭐⭐⭐ -->

<div class="spx-page min-h-screen px-4 py-12 overflow-hidden relative">
	<div class="gradient-bg absolute inset-0"></div>
	<div class="grid-overlay absolute inset-0"></div>

	<div class="glow-orb glow-orb-1"></div>
	<div class="glow-orb glow-orb-2"></div>

	<div class="relative max-w-7xl mx-auto z-10">
		<div class="mb-12" bind:this={headerRef}>
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
		</div>

		<div bind:this={contentRef}>
			<div class="relative">
				<div class="absolute -inset-1 bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 rounded-3xl opacity-30 blur-2xl"></div>

				<div class="relative bg-slate-900/95 backdrop-blur-xl border border-orange-500/30 rounded-3xl p-8 md:p-12 shadow-2xl">

					<!-- Hero Section -->
					<div class="text-center mb-12">
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
					</div>

					<!-- Features Grid -->
					<div class="grid md:grid-cols-2 gap-6 mb-12">
						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">⚡ Instant Alerts</h3>
							<p class="text-slate-300">Real-time SMS and email notifications the moment trades trigger.</p>
						</div>

						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">📊 Complete Trade Details</h3>
							<p class="text-slate-300">Entries, exits, risk, targets, and management plans included.</p>
						</div>

						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">🎯 High Win Rate</h3>
							<p class="text-slate-300">Carefully selected setups with a proven performance edge.</p>
						</div>

						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">📱 Mobile Friendly</h3>
							<p class="text-slate-300">Optimized for traders on the go. Trade from anywhere.</p>
						</div>
					</div>

					<!-- Pricing Plans -->
					<div class="grid md:grid-cols-3 gap-6">

						<!-- Monthly -->
						<div class="relative">
							<div class="absolute -inset-1 bg-gradient-to-r from-orange-500 to-red-500 rounded-2xl opacity-20 blur"></div>
							<div class="relative bg-slate-800/80 backdrop-blur-xl border border-orange-500/30 rounded-2xl p-6">
								<div class="mb-6">
									<h3 class="text-xl font-bold text-white mb-2">Monthly</h3>
									<div class="flex items-baseline gap-2">
										<span class="text-4xl font-bold bg-gradient-to-r from-orange-300 to-red-300 bg-clip-text text-transparent">
											$199
										</span>
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

						<!-- Quarterly -->
						<div class="relative">
							<div class="absolute -inset-1 bg-gradient-to-r from-red-500 to-pink-500 rounded-2xl opacity-25 blur"></div>
							<div class="relative bg-slate-800/80 backdrop-blur-xl border border-red-500/30 rounded-2xl p-6">

								<div class="absolute -top-3 -right-3 px-3 py-1 bg-gradient-to-r from-orange-500 to-red-500 text-white text-xs font-bold rounded-full">
									Save 8%
								</div>

								<div class="mb-6">
									<h3 class="text-xl font-bold text-white mb-2">Quarterly</h3>
									<div class="flex items-baseline gap-2">
										<span class="text-4xl font-bold bg-gradient-to-r from-red-300 to-pink-300 bg-clip-text text-transparent">
											$549
										</span>
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

						<!-- Yearly -->
						<div class="relative">
							<div class="absolute -inset-1 bg-gradient-to-r from-pink-500 to-rose-500 rounded-2xl opacity-30 blur"></div>
							<div class="relative bg-slate-800/80 backdrop-blur-xl border border-pink-500/30 rounded-2xl p-6">

								<div class="absolute -top-3 -right-3 px-3 py-1 bg-gradient-to-r from-orange-500 to-yellow-500 text-white text-xs font-bold rounded-full">
									Best Value
								</div>

								<div class="mb-6">
									<h3 class="text-xl font-bold text-white mb-2">Yearly</h3>
									<div class="flex items-baseline gap-2">
										<span class="text-4xl font-bold bg-gradient-to-r from-pink-300 to-rose-300 bg-clip-text text-transparent">
											$1,999
										</span>
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

					<div class="mt-8 text-center">
						<p class="text-slate-400">
							Cancel anytime • No long-term commitments • Immediate access
						</p>
					</div>

				</div>
			</div>
		</div>

	</div>
</div>

<style>
	.gradient-bg {
		background: linear-gradient(
			135deg,
			#0f172a 0%,
			#7c2d12 25%,
			#0f172a 50%,
			#991b1b 75%,
			#0f172a 100%
		);
		background-size: 400% 400%;
		animation: gradientShift 25s ease infinite;
	}

	@keyframes gradientShift {
		0%, 100% { background-position: 0% 50%; }
		50% { background-position: 100% 50%; }
	}

	.grid-overlay {
		background-image:
			linear-gradient(rgba(249, 115, 22, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(249, 115, 22, 0.03) 1px, transparent 1px);
		background-size: 50px 50px;
		animation: gridMove 30s linear infinite;
		pointer-events: none;
	}

	@keyframes gridMove {
		0% { transform: translate(0,0); }
		100% { transform: translate(50px,50px); }
	}

	.glow-orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.4;
		pointer-events: none;
	}

	.glow-orb-1 {
		width: 600px;
		height: 600px;
		top: -100px;
		left: 50%;
		background: radial-gradient(circle, #f97316, transparent 70%);
		animation: float1 25s ease-in-out infinite;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		bottom: -100px;
		right: 10%;
		background: radial-gradient(circle, #dc2626, transparent 70%);
		animation: float2 30s ease-in-out infinite;
	}

	@keyframes float1 {
		0%, 100% { transform: translateX(-50%) translateY(0) scale(1); }
		50% { transform: translateX(-50%) translateY(-50px) scale(1.1); }
	}

	@keyframes float2 {
		0%, 100% { transform: translateY(0) scale(1); }
		50% { transform: traAnslateY(50px) scale(0.9); }
	}

	@media (max-width: 640px) {
		.glow-orb { filter: blur(60px); opacity: 0.3; }
		.glow-orb-1, .glow-orb-2 { width: 300px; height: 300px; }
	}
</style>
