<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { gsap } from 'gsap';
	import { cartStore, cartItemCount, cartTotal } from '$lib/stores/cart';
	import { validateCoupon, type CouponType } from '$lib/api/coupons';
	import { createOrder, createCheckoutSession } from '$lib/api/checkout';
	import { isAuthenticated } from '$lib/stores/auth';
	import {
		IconTrash,
		IconPlus,
		IconMinus,
		IconShoppingCart,
		IconCreditCard,
		IconShieldCheck,
		IconArrowLeft,
		IconTicket,
		IconX,
		IconLoader
	} from '@tabler/icons-svelte';

	// Svelte 5 state runes
	let isVisible = $state(false);
	let headerRef: HTMLDivElement | undefined = $state();
	let cartItemsRef: HTMLDivElement | undefined = $state();
	let summaryRef: HTMLDivElement | undefined = $state();

	// Coupon state
	let couponCode = $state('');
	let appliedCoupon = $state<{ code: string; discount: number; type: CouponType } | null>(null);
	let couponError = $state('');
	let applyingCoupon = $state(false);

	// Checkout state
	let isCheckingOut = $state(false);
	let checkoutError = $state('');

	onMount(() => {
		isVisible = true;
		initAnimations();
	});

	function initAnimations() {
		const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

		// Animate header
		tl.from(headerRef, {
			opacity: 0,
			y: -50,
			duration: 1,
			ease: 'back.out(1.7)'
		})
			// Animate cart items
			.from(
				'.cart-item-card',
				{
					opacity: 0,
					x: -60,
					duration: 0.8,
					stagger: 0.15,
					ease: 'power4.out'
				},
				'-=0.5'
			)
			// Animate summary
			.from(
				summaryRef,
				{
					opacity: 0,
					x: 60,
					duration: 0.8,
					ease: 'power4.out'
				},
				'-=0.6'
			);
	}

	function removeItem(itemId: string, interval?: 'monthly' | 'quarterly' | 'yearly') {
		cartStore.removeItem(itemId, interval);
	}

	function updateQuantity(
		itemId: string,
		quantity: number,
		interval?: 'monthly' | 'quarterly' | 'yearly'
	) {
		cartStore.updateQuantity(itemId, quantity, interval);
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(price);
	}

	async function handleCheckout() {
		// Check if user is authenticated
		if (!$isAuthenticated) {
			// Redirect to login with return URL
			await goto('/login?redirect=/cart');
			return;
		}

		if ($cartStore.items.length === 0) {
			checkoutError = 'Your cart is empty';
			return;
		}

		isCheckingOut = true;
		checkoutError = '';

		try {
			// Step 1: Create order from cart items
			const order = await createOrder({
				items: $cartStore.items.map((item) => ({
					id: item.id,
					name: item.name,
					price: item.price,
					quantity: item.quantity,
					interval: item.interval
				})),
				couponCode: appliedCoupon?.code,
				discountAmount: discountAmount
			});

			// Step 2: Create Stripe Checkout session
			const session = await createCheckoutSession(
				order.id,
				`${window.location.origin}/checkout/success?order_id=${order.id}`,
				`${window.location.origin}/cart`
			);

			// Step 3: Redirect to Stripe Checkout
			if (session.url) {
				// Clear cart before redirecting
				cartStore.clearCart();
				window.location.href = session.url;
			} else {
				throw new Error('No checkout URL received');
			}
		} catch (error) {
			console.error('Checkout error:', error);
			checkoutError =
				error instanceof Error ? error.message : 'Failed to process checkout. Please try again.';
		} finally {
			isCheckingOut = false;
		}
	}

	async function applyCouponCode() {
		if (!couponCode.trim()) {
			couponError = 'Please enter a coupon code';
			return;
		}

		applyingCoupon = true;
		couponError = '';

		try {
			const result = await validateCoupon(couponCode.trim().toUpperCase(), $cartTotal);

			if (result.valid) {
				appliedCoupon = {
					code: couponCode.trim().toUpperCase(),
					discount: result.discountAmount || 0,
					type: result.type || 'percentage'
				};

				// Apply to cart store
				cartStore.applyCoupon(appliedCoupon.code, appliedCoupon.discount);

				couponCode = '';
				couponError = '';
			} else {
				couponError = result.message || 'Invalid coupon code';
			}
		} catch (error) {
			console.error('Error applying coupon:', error);
			couponError = 'Failed to apply coupon. Please try again.';
		} finally {
			applyingCoupon = false;
		}
	}

	function removeCouponCode() {
		appliedCoupon = null;
		couponCode = '';
		couponError = '';
		cartStore.removeCoupon();
	}

	// Svelte 5 derived runes - Calculate discount amount
	let discountAmount = $derived(
		appliedCoupon
			? appliedCoupon.type === 'percentage'
				? ($cartTotal * appliedCoupon.discount) / 100
				: appliedCoupon.discount
			: 0
	);

	// Calculate final total after discount
	let finalTotal = $derived(Math.max(0, $cartTotal - discountAmount));
</script>

<svelte:head>
	<title>Shopping Cart | Revolution Trading Pros</title>
</svelte:head>

<div class="cart-page min-h-screen px-4 py-12 overflow-hidden relative">
	<!-- Animated gradient background -->
	<div class="gradient-bg absolute inset-0"></div>

	<!-- Grid overlay -->
	<div class="grid-overlay absolute inset-0"></div>

	<!-- Floating orbs -->
	<div class="glow-orb glow-orb-1"></div>
	<div class="glow-orb glow-orb-2"></div>
	<div class="glow-orb glow-orb-3"></div>

	<!-- Cart content -->
	<div class="relative max-w-7xl mx-auto z-10">
		<!-- Header -->
		<div class="mb-12" bind:this={headerRef}>
			<a
				href="/"
				class="inline-flex items-center gap-2 text-indigo-400 hover:text-indigo-300 mb-6 transition-colors duration-300"
			>
				<IconArrowLeft size={20} />
				<span class="font-semibold">Continue Shopping</span>
			</a>

			<div class="flex items-center gap-4 mb-4">
				<div
					class="w-16 h-16 rounded-2xl bg-gradient-to-br from-purple-500 to-pink-500 p-0.5 shadow-lg shadow-purple-500/50"
				>
					<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
						<IconShoppingCart size={32} class="text-purple-400" />
					</div>
				</div>
				<div>
					<h1
						class="text-5xl md:text-6xl font-heading font-bold bg-gradient-to-r from-purple-300 via-pink-300 to-rose-300 bg-clip-text text-transparent"
					>
						Shopping Cart
					</h1>
					<p class="text-xl text-slate-300 mt-2">
						{$cartItemCount}
						{$cartItemCount === 1 ? 'item' : 'items'} in your cart
					</p>
				</div>
			</div>
		</div>

		{#if $cartStore.items.length === 0}
			<!-- Empty cart state -->
			<div class="empty-cart-card relative">
				<div
					class="absolute -inset-1 bg-gradient-to-r from-slate-500/20 to-slate-700/20 rounded-3xl opacity-30 blur-2xl"
				></div>
				<div
					class="relative bg-slate-900/95 backdrop-blur-xl border border-slate-500/30 rounded-3xl p-16 text-center shadow-2xl"
				>
					<div
						class="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-slate-600 to-slate-800 p-0.5"
					>
						<div class="w-full h-full bg-slate-900 rounded-full flex items-center justify-center">
							<IconShoppingCart size={48} class="text-slate-400" />
						</div>
					</div>
					<h2 class="text-3xl font-bold text-white mb-4">Your cart is empty</h2>
					<p class="text-slate-400 mb-8 text-lg">
						Start adding some amazing products to get started!
					</p>
					<a
						href="/"
						class="inline-block px-8 py-4 bg-gradient-to-r from-purple-500 via-pink-500 to-rose-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-purple-500/50 transition-all duration-300 transform hover:scale-105"
					>
						Browse Products
					</a>
				</div>
			</div>
		{:else}
			<!-- Cart items grid -->
			<div class="grid lg:grid-cols-3 gap-8">
				<!-- Cart items list -->
				<div class="lg:col-span-2 space-y-4" bind:this={cartItemsRef}>
					{#each $cartStore.items as item (item.id + (item.interval || ''))}
						<div class="cart-item-card relative">
							<div
								class="absolute -inset-1 bg-gradient-to-r from-indigo-500/20 via-purple-500/20 to-pink-500/20 rounded-2xl opacity-50 blur-xl"
							></div>
							<div
								class="relative bg-slate-900/95 backdrop-blur-xl border border-indigo-500/30 rounded-2xl p-6 shadow-xl"
							>
								<div class="flex gap-6">
									<!-- Product image placeholder -->
									<div
										class="flex-shrink-0 w-24 h-24 rounded-xl bg-gradient-to-br from-indigo-500 to-purple-600 p-0.5"
									>
										<div
											class="w-full h-full bg-slate-900 rounded-xl flex items-center justify-center"
										>
											{#if item.type === 'membership'}
												<IconShieldCheck size={40} class="text-indigo-400" />
											{:else if item.type === 'course'}
												<IconCreditCard size={40} class="text-purple-400" />
											{:else}
												<IconShoppingCart size={40} class="text-pink-400" />
											{/if}
										</div>
									</div>

									<!-- Product details -->
									<div class="flex-1 min-w-0">
										<div class="flex items-start justify-between gap-4 mb-2">
											<div class="flex-1">
												<h3 class="text-xl font-bold text-white mb-1">{item.name}</h3>
												<p class="text-sm text-slate-400">{item.description}</p>
												{#if item.interval}
													<span
														class="inline-block mt-2 px-3 py-1 bg-purple-500/20 text-purple-300 text-xs font-semibold rounded-full border border-purple-500/30"
													>
														{item.interval === 'monthly' ? 'Monthly' : 'Yearly'}
													</span>
												{/if}
											</div>
											<button
												onclick={() => removeItem(item.id, item.interval)}
												class="p-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-all duration-300"
												aria-label="Remove item"
											>
												<IconTrash size={20} />
											</button>
										</div>

										<div class="flex items-center justify-between mt-4">
											<!-- Quantity controls -->
											<div class="flex items-center gap-3">
												<span class="text-sm text-slate-400 font-semibold">Quantity:</span>
												<div class="flex items-center gap-2">
													<button
														onclick={() =>
															updateQuantity(item.id, item.quantity - 1, item.interval)}
														class="w-8 h-8 flex items-center justify-center bg-slate-800 hover:bg-indigo-500/20 text-white rounded-lg transition-all duration-300"
														disabled={item.quantity <= 1}
													>
														<IconMinus size={16} />
													</button>
													<span class="w-12 text-center font-bold text-white text-lg"
														>{item.quantity}</span
													>
													<button
														onclick={() =>
															updateQuantity(item.id, item.quantity + 1, item.interval)}
														class="w-8 h-8 flex items-center justify-center bg-slate-800 hover:bg-indigo-500/20 text-white rounded-lg transition-all duration-300"
													>
														<IconPlus size={16} />
													</button>
												</div>
											</div>

											<!-- Price -->
											<div class="text-right">
												<p class="text-2xl font-bold text-white">
													{formatPrice(item.price * item.quantity)}
												</p>
												{#if item.quantity > 1}
													<p class="text-sm text-slate-400">{formatPrice(item.price)} each</p>
												{/if}
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>

				<!-- Order summary -->
				<div class="lg:col-span-1">
					<div class="sticky top-24" bind:this={summaryRef}>
						<div class="summary-card relative">
							<div
								class="absolute -inset-1 bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 rounded-3xl opacity-30 blur-2xl"
							></div>
							<div
								class="relative bg-slate-900/95 backdrop-blur-xl border border-emerald-500/30 rounded-3xl p-8 shadow-2xl"
							>
								<h2 class="text-2xl font-heading font-bold text-white mb-6">Order Summary</h2>

								<!-- Coupon Code Section -->
								<div class="mb-6">
									<label for="coupon" class="block text-sm font-semibold text-slate-300 mb-2">
										Have a coupon code?
									</label>

									{#if appliedCoupon}
										<!-- Applied Coupon Display -->
										<div
											class="flex items-center justify-between p-4 bg-emerald-500/10 border border-emerald-500/30 rounded-xl mb-4"
										>
											<div class="flex items-center gap-3">
												<div
													class="w-10 h-10 rounded-lg bg-emerald-500/20 flex items-center justify-center"
												>
													<IconTicket size={20} class="text-emerald-400" />
												</div>
												<div>
													<p class="font-bold text-emerald-400">{appliedCoupon.code}</p>
													<p class="text-xs text-slate-400">
														{appliedCoupon.type === 'percentage'
															? `${appliedCoupon.discount}% off`
															: `$${appliedCoupon.discount} off`}
													</p>
												</div>
											</div>
											<button
												onclick={removeCouponCode}
												class="p-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-all"
												aria-label="Remove coupon"
											>
												<IconX size={20} />
											</button>
										</div>
									{:else}
										<!-- Coupon Input Field -->
										<div class="flex gap-2">
											<div class="flex-1 relative">
												<input
													type="text"
													id="coupon"
													bind:value={couponCode}
													onkeypress={(e) => e.key === 'Enter' && applyCouponCode()}
													placeholder="Enter code"
													class="w-full px-4 py-3 bg-slate-800 border border-slate-600 rounded-xl text-white placeholder-slate-400 focus:outline-none focus:border-emerald-500 transition-colors uppercase"
													disabled={applyingCoupon}
												/>
												{#if applyingCoupon}
													<div class="absolute right-3 top-1/2 -translate-y-1/2">
														<div
															class="w-5 h-5 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin"
														></div>
													</div>
												{/if}
											</div>
											<button
												onclick={applyCouponCode}
												disabled={applyingCoupon || !couponCode.trim()}
												class="px-6 py-3 bg-emerald-500 hover:bg-emerald-600 text-white font-semibold rounded-xl transition-all disabled:opacity-50 disabled:cursor-not-allowed"
											>
												Apply
											</button>
										</div>
										{#if couponError}
											<p class="mt-2 text-sm text-red-400">{couponError}</p>
										{/if}
									{/if}
								</div>

								<div class="space-y-4 mb-6">
									<div class="flex justify-between text-slate-300">
										<span>Subtotal</span>
										<span class="font-semibold">{formatPrice($cartTotal)}</span>
									</div>

									{#if appliedCoupon && discountAmount > 0}
										<div class="flex justify-between text-emerald-400">
											<span class="flex items-center gap-2">
												<IconTicket size={16} />
												Discount ({appliedCoupon.code})
											</span>
											<span class="font-semibold">-{formatPrice(discountAmount)}</span>
										</div>
									{/if}

									<div class="flex justify-between text-slate-300">
										<span>Tax</span>
										<span class="font-semibold">{formatPrice(0)}</span>
									</div>
									<div
										class="h-px bg-gradient-to-r from-transparent via-emerald-500/50 to-transparent"
									></div>
									<div class="flex justify-between text-white text-xl font-bold">
										<span>Total</span>
										<span>{formatPrice(finalTotal)}</span>
									</div>

									{#if appliedCoupon && discountAmount > 0}
										<p class="text-sm text-emerald-400 text-center">
											You saved {formatPrice(discountAmount)}!
										</p>
									{/if}
								</div>

								<button
									onclick={handleCheckout}
									disabled={isCheckingOut || $cartStore.items.length === 0}
									class="w-full relative px-6 py-4 bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 text-white font-heading font-bold rounded-xl overflow-hidden group transition-all duration-300 hover:shadow-lg hover:shadow-emerald-500/50 mb-4 disabled:opacity-50 disabled:cursor-not-allowed"
								>
									<span class="relative z-10 flex items-center justify-center gap-2">
										{#if isCheckingOut}
											<IconLoader size={24} class="animate-spin" />
											<span>Processing...</span>
										{:else}
											<IconCreditCard size={24} />
											<span>Proceed to Checkout</span>
										{/if}
									</span>
									<div
										class="absolute inset-0 bg-white/20 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"
									></div>
								</button>

								{#if checkoutError}
									<p class="text-red-400 text-sm text-center mb-4">{checkoutError}</p>
								{/if}

								<div class="flex items-center justify-center gap-2 text-emerald-400 text-sm">
									<IconShieldCheck size={16} />
									<span>Secure checkout guaranteed</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Animated gradient background */
	.gradient-bg {
		background: linear-gradient(
			135deg,
			#0f172a 0%,
			#1e1b4b 20%,
			#312e81 40%,
			#1e1b4b 60%,
			#0f172a 80%,
			#1e1b4b 100%
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
			linear-gradient(rgba(139, 92, 246, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(139, 92, 246, 0.03) 1px, transparent 1px);
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
		right: -100px;
		background: radial-gradient(circle, #8b5cf6, transparent 70%);
		animation: float1 25s ease-in-out infinite;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		bottom: -100px;
		left: -100px;
		background: radial-gradient(circle, #ec4899, transparent 70%);
		animation: float2 30s ease-in-out infinite;
	}

	.glow-orb-3 {
		width: 450px;
		height: 450px;
		top: 50%;
		left: 50%;
		background: radial-gradient(circle, #14b8a6, transparent 70%);
		animation: float3 35s ease-in-out infinite;
	}

	@keyframes float1 {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(-100px, -50px) scale(1.1);
		}
		66% {
			transform: translate(50px, -100px) scale(0.9);
		}
	}

	@keyframes float2 {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(100px, 50px) scale(1.1);
		}
		66% {
			transform: translate(-50px, 100px) scale(0.9);
		}
	}

	@keyframes float3 {
		0%,
		100% {
			transform: translate(-50%, -50%) scale(1);
		}
		50% {
			transform: translate(-50%, -50%) scale(1.2);
		}
	}

	/* Responsive adjustments */
	@media (max-width: 640px) {
		.glow-orb {
			filter: blur(60px);
			opacity: 0.3;
		}

		.glow-orb-1,
		.glow-orb-2,
		.glow-orb-3 {
			width: 300px;
			height: 300px;
		}
	}
</style>
