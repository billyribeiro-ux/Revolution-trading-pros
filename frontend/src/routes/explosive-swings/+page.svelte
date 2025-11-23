<script lang="ts">
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import {
		IconTrendingUp,
		IconFlame,
		IconBellRinging,
		IconShoppingCart
	} from '@tabler/icons-svelte';
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
				id: 'explosive-swings',
				name: 'Explosive Swings',
				description: 'High-momentum swing trade alerts with explosive profit potential',
				price,
				type: 'alert-service',
				interval
			},
			true // Navigate to cart after adding
		);
	}
</script>

<svelte:head>
	<title>Explosive Swings | Revolution Trading Pros</title>
</svelte:head>

<div class="explosive-page min-h-screen px-4 py-12 overflow-hidden relative">
	<!-- Animated gradient background -->
	<div class="gradient-bg absolute inset-0"></div>

	<!-- Grid overlay -->
	<div class="grid-overlay absolute inset-0"></div>

	<!-- Floating orbs -->
	<div class="glow-orb glow-orb-1"></div>
	<div class="glow-orb glow-orb-2"></div>

	<!-- Content -->
	<div class="relative max-w-7xl mx-auto z-10">
		<!-- Header -->
		<div class="mb-12" bind:this={headerRef}>
			<div class="flex items-center gap-4 mb-4">
				<div
					class="w-16 h-16 rounded-2xl bg-gradient-to-br from-yellow-500 to-amber-500 p-0.5 shadow-lg shadow-yellow-500/50"
				>
					<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
						<IconFlame size={32} class="text-yellow-400" />
					</div>
				</div>
				<div>
					<h1
						class="text-5xl md:text-6xl font-heading font-bold bg-gradient-to-r from-yellow-300 via-amber-300 to-orange-300 bg-clip-text text-transparent"
					>
						Explosive Swings
					</h1>
					<p class="text-xl text-slate-300 mt-2 flex items-center gap-2">
						<IconTrendingUp size={20} class="text-yellow-400" />
						<span>High-momentum swing alerts for maximum gains</span>
					</p>
				</div>
			</div>
		</div>

		<!-- Main Content Card -->
		<div bind:this={contentRef}>
			<div class="relative">
				<div
					class="absolute -inset-1 bg-gradient-to-r from-yellow-500 via-amber-500 to-orange-500 rounded-3xl opacity-30 blur-2xl"
				></div>
				<div
					class="relative bg-slate-900/95 backdrop-blur-xl border border-yellow-500/30 rounded-3xl p-8 md:p-12 shadow-2xl"
				>
					<!-- Hero Section -->
					<div class="text-center mb-12">
						<div
							class="inline-flex items-center gap-2 px-4 py-2 bg-yellow-500/20 border border-yellow-500/30 rounded-full mb-6"
						>
							<IconBellRinging size={20} class="text-yellow-400" />
							<span class="text-yellow-300 font-semibold">Momentum Swing Alerts</span>
						</div>

						<h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4">
							Capture Explosive Market Moves
						</h2>
						<p class="text-lg text-slate-300 max-w-3xl mx-auto">
							Get alerts for high-momentum swing trades with explosive profit potential. Our system
							identifies stocks breaking out with strong momentum for multi-day gains that can
							transform your portfolio.
						</p>
					</div>

					<!-- Features Grid -->
					<div class="grid md:grid-cols-2 gap-6 mb-12">
						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">🚀 Momentum Breakouts</h3>
							<p class="text-slate-300">
								Stocks with strong momentum and volume breaking key resistance levels
							</p>
						</div>

						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">📈 Multi-Day Holds</h3>
							<p class="text-slate-300">
								Swing positions designed to capture 3-10 day explosive moves
							</p>
						</div>

						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">🎯 Clear Exit Strategy</h3>
							<p class="text-slate-300">Defined profit targets and stop losses for every alert</p>
						</div>

						<div class="p-6 bg-slate-800/50 rounded-xl border border-slate-700/50">
							<h3 class="text-xl font-bold text-white mb-2">⚡ Real-Time Updates</h3>
							<p class="text-slate-300">Position updates and exit alerts sent in real-time</p>
						</div>
					</div>

					<!-- Pricing Options -->
					<div class="grid md:grid-cols-3 gap-6">
						<!-- Monthly Plan -->
						<div class="relative">
							<div
								class="absolute -inset-1 bg-gradient-to-r from-yellow-500 to-amber-500 rounded-2xl opacity-20 blur"
							></div>
							<div
								class="relative bg-slate-800/80 backdrop-blur-xl border border-yellow-500/30 rounded-2xl p-6"
							>
								<div class="mb-6">
									<h3 class="text-xl font-bold text-white mb-2">Monthly</h3>
									<div class="flex items-baseline gap-2">
										<span
											class="text-4xl font-bold bg-gradient-to-r from-yellow-300 to-amber-300 bg-clip-text text-transparent"
											>$199</span
										>
										<span class="text-slate-400 text-sm">/month</span>
									</div>
									<p class="text-xs text-slate-400 mt-2">Flexible monthly billing</p>
								</div>

								<button
									on:click={() => handleSignUp('monthly')}
									class="w-full px-6 py-3 bg-gradient-to-r from-yellow-500 to-amber-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-yellow-500/50 transition-all duration-300 flex items-center justify-center gap-2"
								>
									<IconShoppingCart size={18} />
									<span>Sign Up</span>
								</button>
							</div>
						</div>

						<!-- Quarterly Plan -->
						<div class="relative">
							<div
								class="absolute -inset-1 bg-gradient-to-r from-amber-500 to-orange-500 rounded-2xl opacity-25 blur"
							></div>
							<div
								class="relative bg-slate-800/80 backdrop-blur-xl border border-amber-500/30 rounded-2xl p-6"
							>
								<div
									class="absolute -top-3 -right-3 px-3 py-1 bg-gradient-to-r from-yellow-500 to-amber-500 text-white text-xs font-bold rounded-full"
								>
									Save 8%
								</div>

								<div class="mb-6">
									<h3 class="text-xl font-bold text-white mb-2">Quarterly</h3>
									<div class="flex items-baseline gap-2">
										<span
											class="text-4xl font-bold bg-gradient-to-r from-amber-300 to-orange-300 bg-clip-text text-transparent"
											>$549</span
										>
										<span class="text-slate-400 text-sm">/3 months</span>
									</div>
									<p class="text-xs text-slate-400 mt-2">$183/month - Save $48</p>
								</div>

								<button
									on:click={() => handleSignUp('quarterly')}
									class="w-full px-6 py-3 bg-gradient-to-r from-amber-500 to-orange-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-amber-500/50 transition-all duration-300 flex items-center justify-center gap-2"
								>
									<IconShoppingCart size={18} />
									<span>Sign Up</span>
								</button>
							</div>
						</div>

						<!-- Yearly Plan -->
						<div class="relative">
							<div
								class="absolute -inset-1 bg-gradient-to-r from-orange-500 to-red-500 rounded-2xl opacity-30 blur"
							></div>
							<div
								class="relative bg-slate-800/80 backdrop-blur-xl border border-orange-500/30 rounded-2xl p-6"
							>
								<div
									class="absolute -top-3 -right-3 px-3 py-1 bg-gradient-to-r from-orange-500 to-yellow-500 text-white text-xs font-bold rounded-full"
								>
									Best Value
								</div>

								<div class="mb-6">
									<h3 class="text-xl font-bold text-white mb-2">Yearly</h3>
									<div class="flex items-baseline gap-2">
										<span
											class="text-4xl font-bold bg-gradient-to-r from-orange-300 to-red-300 bg-clip-text text-transparent"
											>$1,999</span
										>
										<span class="text-slate-400 text-sm">/year</span>
									</div>
									<p class="text-xs text-slate-400 mt-2">$166/month - Save $389</p>
								</div>

								<button
									on:click={() => handleSignUp('yearly')}
									class="w-full px-6 py-3 bg-gradient-to-r from-orange-500 to-red-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-orange-500/50 transition-all duration-300 flex items-center justify-center gap-2"
								>
									<IconShoppingCart size={18} />
									<span>Sign Up</span>
								</button>
							</div>
						</div>
					</div>

					<!-- Additional Info -->
					<div class="mt-8 text-center">
						<p class="text-slate-400">
							Cancel anytime • No long-term commitment • Full access from day one
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* Animated gradient background */
	.gradient-bg {
		background: linear-gradient(
			135deg,
			#0f172a 0%,
			#713f12 25%,
			#0f172a 50%,
			#78350f 75%,
			#0f172a 100%
		);
		background-size: 400% 400%;
		animation: gradientShift 25s ease infinite;
	}

	@keyframes gradientShift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	/* Grid overlay */
	.grid-overlay {
		background-image:
			linear-gradient(rgba(234, 179, 8, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(234, 179, 8, 0.03) 1px, transparent 1px);
		background-size: 50px 50px;
		animation: gridMove 30s linear infinite;
		pointer-events: none;
	}

	@keyframes gridMove {
		0% {
			transform: translate(0, 0);
		}
		100% {
			transform: translate(50px, 50px);
		}
	}

	/* Glowing orbs */
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
		background: radial-gradient(circle, #eab308, transparent 70%);
		animation: float1 25s ease-in-out infinite;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		bottom: -100px;
		right: 10%;
		background: radial-gradient(circle, #f59e0b, transparent 70%);
		animation: float2 30s ease-in-out infinite;
	}

	@keyframes float1 {
		0%,
		100% {
			transform: translateX(-50%) translateY(0) scale(1);
		}
		50% {
			transform: translateX(-50%) translateY(-50px) scale(1.1);
		}
	}

	@keyframes float2 {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(50px) scale(0.9);
		}
	}

	/* Responsive */
	@media (max-width: 640px) {
		.glow-orb {
			filter: blur(60px);
			opacity: 0.3;
		}

		.glow-orb-1,
		.glow-orb-2 {
			width: 300px;
			height: 300px;
		}
	}
</style>
