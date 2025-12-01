<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { gsap } from 'gsap';
	import { user } from '$lib/stores/auth';
	import { cartStore, cartTotal } from '$lib/stores/cart';
	import { createCheckoutSession } from '$lib/api/cart';
	import {
		IconCreditCard,
		IconShieldCheck,
		IconArrowLeft,
		IconLock,
		IconCheck
	} from '@tabler/icons-svelte';
	import type { PageData } from './$types';

	export const data: PageData = {} as PageData;

	let isProcessing = false;
	let headerRef: HTMLDivElement;
	let orderSummaryRef: HTMLDivElement;
	let paymentFormRef: HTMLDivElement;

	onMount(() => {
		// Redirect to cart if no items
		if ($cartStore.items.length === 0) {
			goto('/cart');
			return;
		}

		initAnimations();
	});

	function initAnimations() {
		const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

		tl.from(headerRef, {
			opacity: 0,
			y: -50,
			duration: 1,
			ease: 'back.out(1.7)'
		})
			.from(
				orderSummaryRef,
				{
					opacity: 0,
					x: -60,
					duration: 0.8,
					ease: 'power4.out'
				},
				'-=0.5'
			)
			.from(
				paymentFormRef,
				{
					opacity: 0,
					x: 60,
					duration: 0.8,
					ease: 'power4.out'
				},
				'-=0.6'
			);
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(price);
	}

	async function handleCheckout() {
		isProcessing = true;

		try {
			// Create Stripe checkout session
			const session = await createCheckoutSession($cartStore.items);

			// Redirect to Stripe checkout
			window.location.href = session.url;
		} catch (error) {
			console.error('Checkout error:', error);
			const errorMessage =
				error instanceof Error ? error.message : 'Failed to process checkout. Please try again.';
			alert(errorMessage);
			isProcessing = false;
		}
	}
</script>

<svelte:head>
	<title>Secure Checkout | Revolution Trading Pros</title>
</svelte:head>

<div class="checkout-page min-h-screen px-4 py-12 overflow-hidden relative">
	<!-- Animated gradient background -->
	<div class="gradient-bg absolute inset-0"></div>

	<!-- Grid overlay -->
	<div class="grid-overlay absolute inset-0"></div>

	<!-- Floating orbs -->
	<div class="glow-orb glow-orb-1"></div>
	<div class="glow-orb glow-orb-2"></div>

	<!-- Checkout content -->
	<div class="relative max-w-7xl mx-auto z-10">
		<!-- Header -->
		<div class="mb-12" bind:this={headerRef}>
			<a
				href="/cart"
				class="inline-flex items-center gap-2 text-emerald-400 hover:text-emerald-300 mb-6 transition-colors duration-300"
			>
				<IconArrowLeft size={20} />
				<span class="font-semibold">Back to Cart</span>
			</a>

			<div class="flex items-center gap-4 mb-4">
				<div
					class="w-16 h-16 rounded-2xl bg-gradient-to-br from-emerald-500 to-teal-500 p-0.5 shadow-lg shadow-emerald-500/50"
				>
					<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
						<IconShieldCheck size={32} class="text-emerald-400" />
					</div>
				</div>
				<div>
					<h1
						class="text-5xl md:text-6xl font-heading font-bold bg-gradient-to-r from-emerald-300 via-teal-300 to-cyan-300 bg-clip-text text-transparent"
					>
						Secure Checkout
					</h1>
					<p class="text-xl text-slate-300 mt-2 flex items-center gap-2">
						<IconLock size={20} class="text-emerald-400" />
						<span>256-bit SSL encrypted payment</span>
					</p>
				</div>
			</div>
		</div>

		<!-- Checkout grid -->
		<div class="grid lg:grid-cols-3 gap-8">
			<!-- Order Summary -->
			<div class="lg:col-span-1 order-2 lg:order-1">
				<div class="sticky top-24" bind:this={orderSummaryRef}>
					<div class="relative">
						<div
							class="absolute -inset-1 bg-gradient-to-r from-indigo-500/20 via-purple-500/20 to-pink-500/20 rounded-3xl opacity-50 blur-2xl"
						></div>
						<div
							class="relative bg-slate-900/95 backdrop-blur-xl border border-indigo-500/30 rounded-3xl p-8 shadow-2xl"
						>
							<h2 class="text-2xl font-heading font-bold text-white mb-6">Order Summary</h2>

							<!-- Cart items -->
							<div class="space-y-4 mb-6 max-h-96 overflow-y-auto">
								{#each $cartStore.items as item}
									<div class="flex gap-3 p-3 bg-slate-800/50 rounded-xl border border-slate-700/50">
										<div class="flex-1 min-w-0">
											<p class="font-semibold text-white text-sm truncate">{item.name}</p>
											<p class="text-xs text-slate-400">Qty: {item.quantity}</p>
											{#if item.interval}
												<span
													class="inline-block mt-1 px-2 py-0.5 bg-purple-500/20 text-purple-300 text-xs rounded-full"
												>
													{item.interval}
												</span>
											{/if}
										</div>
										<div class="text-right">
											<p class="font-bold text-white">{formatPrice(item.price * item.quantity)}</p>
										</div>
									</div>
								{/each}
							</div>

							<div class="space-y-3 mb-6">
								<div class="flex justify-between text-slate-300">
									<span>Subtotal</span>
									<span class="font-semibold">{formatPrice($cartTotal)}</span>
								</div>
								<div class="flex justify-between text-slate-300">
									<span>Tax</span>
									<span class="font-semibold">{formatPrice(0)}</span>
								</div>
								<div
									class="h-px bg-gradient-to-r from-transparent via-indigo-500/50 to-transparent"
								></div>
								<div class="flex justify-between text-white text-xl font-bold">
									<span>Total</span>
									<span>{formatPrice($cartTotal)}</span>
								</div>
							</div>

							<!-- Security badges -->
							<div class="flex flex-wrap gap-3 pt-4 border-t border-slate-700/50">
								<div class="flex items-center gap-2 text-emerald-400 text-xs">
									<IconShieldCheck size={16} />
									<span>Secure Payment</span>
								</div>
								<div class="flex items-center gap-2 text-emerald-400 text-xs">
									<IconLock size={16} />
									<span>SSL Encrypted</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Payment Form -->
			<div class="lg:col-span-2 order-1 lg:order-2">
				<div bind:this={paymentFormRef}>
					<div class="relative">
						<div
							class="absolute -inset-1 bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 rounded-3xl opacity-30 blur-2xl"
						></div>
						<div
							class="relative bg-slate-900/95 backdrop-blur-xl border border-emerald-500/30 rounded-3xl p-8 shadow-2xl"
						>
							<!-- Billing Information -->
							<div class="mb-8">
								<h2 class="text-2xl font-heading font-bold text-white mb-6">Billing Information</h2>

								<div class="p-4 bg-emerald-500/10 border border-emerald-500/30 rounded-xl mb-6">
									<div class="flex items-start gap-3">
										<IconCheck size={20} class="text-emerald-400 flex-shrink-0 mt-0.5" />
										<div>
											<p class="text-emerald-300 font-semibold mb-1">Logged in as:</p>
											<p class="text-white">{$user?.name}</p>
											<p class="text-slate-400 text-sm">{$user?.email}</p>
										</div>
									</div>
								</div>
							</div>

							<!-- Payment Method -->
							<div class="mb-8">
								<h2 class="text-2xl font-heading font-bold text-white mb-6">Payment Method</h2>

								<div
									class="p-6 bg-gradient-to-br from-slate-800/60 to-slate-800/40 rounded-xl border border-slate-700/50 mb-6"
								>
									<div class="flex items-center gap-3 mb-4">
										<IconCreditCard size={24} class="text-emerald-400" />
										<p class="text-white font-semibold">Credit or Debit Card</p>
									</div>
									<p class="text-slate-400 text-sm mb-4">
										You'll be securely redirected to our payment processor (Stripe) to complete your
										purchase.
									</p>
									<div class="flex gap-2">
										<img
											src="https://js.stripe.com/v3/fingerprinted/img/visa-729c05c240c4bdb47b03ac81d9945bfe.svg"
											alt="Visa"
											class="h-8"
										/>
										<img
											src="https://js.stripe.com/v3/fingerprinted/img/mastercard-4d8844094130711885b5e41b28c9848f.svg"
											alt="Mastercard"
											class="h-8"
										/>
										<img
											src="https://js.stripe.com/v3/fingerprinted/img/amex-a49b82f46c5cd6a96a6e418a6ca1717c.svg"
											alt="Amex"
											class="h-8"
										/>
										<img
											src="https://js.stripe.com/v3/fingerprinted/img/discover-ac52cd46f89fa40a29a0bfb954e33173.svg"
											alt="Discover"
											class="h-8"
										/>
									</div>
								</div>
							</div>

							<!-- Complete Purchase Button -->
							<button
								onclick={handleCheckout}
								disabled={isProcessing}
								class="w-full relative px-8 py-5 bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 text-white font-heading font-bold text-lg rounded-xl overflow-hidden group transition-all duration-300 hover:shadow-2xl hover:shadow-emerald-500/50 disabled:opacity-50 disabled:cursor-not-allowed"
							>
								<span class="relative z-10 flex items-center justify-center gap-3">
									{#if isProcessing}
										<div
											class="w-6 h-6 border-3 border-white/30 border-t-white rounded-full animate-spin"
										></div>
										<span>Processing...</span>
									{:else}
										<IconShieldCheck size={24} />
										<span>Complete Secure Purchase</span>
									{/if}
								</span>
								<div
									class="absolute inset-0 bg-white/20 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"
								></div>
							</button>

							<!-- Terms -->
							<p class="text-center text-slate-400 text-sm mt-6">
								By completing this purchase, you agree to our
								<a href="/terms" class="text-emerald-400 hover:text-emerald-300 underline"
									>Terms of Service</a
								>
								and
								<a href="/privacy" class="text-emerald-400 hover:text-emerald-300 underline"
									>Privacy Policy</a
								>
							</p>
						</div>
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
			#064e3b 25%,
			#0f172a 50%,
			#1e1b4b 75%,
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
			linear-gradient(rgba(16, 185, 129, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(16, 185, 129, 0.03) 1px, transparent 1px);
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
		background: radial-gradient(circle, #10b981, transparent 70%);
		animation: float1 25s ease-in-out infinite;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		bottom: -100px;
		right: 10%;
		background: radial-gradient(circle, #14b8a6, transparent 70%);
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
