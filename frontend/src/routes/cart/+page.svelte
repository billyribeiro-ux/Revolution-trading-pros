<script lang="ts">
	/**
	 * Shopping Cart Page - Revolution Trading Custom Cart System
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Custom cart system with pixel-perfect styling (no WooCommerce dependency)
	 * @version 8.0.0 (Custom RTP Cart System / January 2026)
	 */

	import { goto } from '$app/navigation';
	import { cartStore, cartItemCount, cartTotal } from '$lib/stores/cart.svelte';
	import { validateCoupon, type CouponType } from '$lib/api/coupons';
	import { isAuthenticated } from '$lib/stores/auth.svelte';
	import NonMemberCheckout from '$lib/components/cart/NonMemberCheckout.svelte';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconShoppingCart from '@tabler/icons-svelte-runes/icons/shopping-cart';
	import IconShieldCheck from '@tabler/icons-svelte-runes/icons/shield-check';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let couponCode = $state('');
	let appliedCoupon = $state<{ code: string; discount: number; type: CouponType } | null>(null);
	let couponError = $state('');
	let applyingCoupon = $state(false);
	// CSRF token for form security (replaces WordPress nonce)
	let csrfToken = $state(crypto.randomUUID());

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	let discountAmount = $derived(
		appliedCoupon
			? appliedCoupon.type === 'percentage'
				? ($cartTotal * appliedCoupon.discount) / 100
				: appliedCoupon.discount
			: 0
	);

	let finalTotal = $derived(Math.max(0, $cartTotal - discountAmount));

	let hasSubscriptions = $derived($cartStore.items.some((i) => i.interval));

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function removeItem(itemId: string, interval?: 'monthly' | 'quarterly' | 'yearly' | 'lifetime') {
		cartStore.removeItem(itemId, interval);
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(price);
	}

	function getIntervalText(interval?: string): string {
		switch (interval) {
			case 'monthly':
				return 'every month';
			case 'quarterly':
				return 'every 3 months';
			case 'yearly':
				return 'every year';
			default:
				return '';
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
				cartStore.applyCoupon(appliedCoupon.code, appliedCoupon.discount);
				couponCode = '';
			} else {
				couponError = result.message || 'Invalid coupon code';
			}
		} catch (error) {
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

	function proceedToCheckout() {
		if (!$isAuthenticated) {
			goto('/login?redirect=/checkout');
			return;
		}
		goto('/checkout');
	}

	function clearCart() {
		if (confirm('Clear All Items?')) {
			cartStore.clearCart();
		}
	}
</script>

<svelte:head>
	<title>Cart | Revolution Trading Pros</title>
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Simpler Trading Cart Style (EXACT STRUCTURE)
     ═══════════════════════════════════════════════════════════════════════════ -->

<!-- Show NonMemberCheckout for unauthenticated users with items in cart -->
{#if !$isAuthenticated && $cartStore.items.length > 0}
	<NonMemberCheckout />
{:else}
	<div class="rtp-cart rtp-page">
		<div class="container">
			<!-- Page Header -->
			<header class="cart-page-header">
				<a href="/" class="back-link">
					<IconArrowLeft size={14} stroke={2.5} />
					<span>Continue Shopping</span>
				</a>
				<h1 class="page-title">CART</h1>
				<p class="cart-count">
					{$cartItemCount}
					{$cartItemCount === 1 ? 'item' : 'items'} in your cart
				</p>
			</header>

			{#if $cartStore.items.length === 0}
				<!-- Empty Cart State -->
				<div class="cart-empty">
					<div class="card">
						<div class="card-body">
							<div class="cart-empty-icon">
								<IconShoppingCart size={64} />
							</div>
							<h2>Your cart is empty</h2>
							<p>Looks like you haven't added any products to your cart yet.</p>
							<a href="/live-trading-rooms" class="btn btn-lg btn-orange">
								Browse Trading Rooms
								<IconArrowRight size={16} />
							</a>
						</div>
					</div>
				</div>
			{:else}
				<!-- Cart Form - Revolution Trading Structure -->
				<form class="rtp-cart-form" onsubmit={(e: SubmitEvent) => e.preventDefault()}>
					<!-- CSRF Token for Security -->
					<input type="hidden" name="_csrf_token" value={csrfToken} />
					<div class="rtp-cart-form__contents">
						<div class="row">
							<!-- Products Column -->
							<div class="col-xs-12 col-sm-7 col-md-8">
								<div class="products" data-products-view="list">
									<div class="flex-grid">
										{#each $cartStore.items as item (item.id + (item.interval || ''))}
											<article class="product flex-grid-item cart_item">
												<div class="card">
													<figure class="card-media">
														<div
															class="card-image"
															style:background-image={item.thumbnail || item.image
																? `url(${item.thumbnail || item.image})`
																: undefined}
														>
															{#if !item.thumbnail && !item.image}
																<IconShoppingCart size={32} />
															{/if}
														</div>
													</figure>

													<section class="card-body">
														<h4 class="card-title">
															{item.name}&nbsp;
														</h4>
														{#if item.interval}
															<div class="card-description">
																<span class="membership-tagline">
																	<span class="bold">Subscription:</span>
																	<span class="rtp-price-amount amount">
																		{formatPrice(item.price)}
																	</span>
																	<span class="subscription-details">
																		{getIntervalText(item.interval)}</span
																	>
																</span>
															</div>
														{/if}
														<div class="product-quantity" data-title="Quantity">
															<input type="hidden" name="cart[qty]" value="1" />
														</div>
													</section>

													<footer class="card-footer">
														<div class="product-add-to-cart product-remove">
															<button
																type="button"
																class="btn btn-xs btn-default"
																aria-label="Remove this item"
																onclick={() => removeItem(item.id, item.interval)}
															>
																<IconX size={13} stroke={2} />&nbsp;Remove
															</button>
														</div>
														<div class="product-price">
															<span class="rtp-price-amount-wrap">
																<span class="rtp-price-amount amount">
																	{formatPrice(item.price)}
																</span>
															</span>
														</div>
													</footer>
												</div>
											</article>
										{/each}
									</div>
								</div>

								<div class="clear-cart">
									<button type="button" class="btn btn-lg btn-default" onclick={clearCart}>
										Clear Cart
									</button>
								</div>
							</div>

							<!-- Cart Totals Sidebar -->
							<div class="col-xs-12 col-sm-5 col-md-4">
								<div class="cart-collaterals">
									<div class="card">
										<div class="card-body">
											<div class="cart_totals">
												<!-- Order Total -->
												<div class="order-total">
													<div>Total</div>
													<div class="order-total-price">
														{#if appliedCoupon && discountAmount > 0}
															<span class="original-price">{formatPrice($cartTotal)}</span>
														{/if}
														{formatPrice(finalTotal)}
													</div>
												</div>

												<!-- Applied Coupon -->
												{#if appliedCoupon && discountAmount > 0}
													<div class="coupon-applied">
														<span class="coupon-label">Coupon: {appliedCoupon.code}</span>
														<span class="coupon-discount">-{formatPrice(discountAmount)}</span>
														<button type="button" class="remove-coupon" onclick={removeCouponCode}>
															<IconX size={14} stroke={2} />
														</button>
													</div>
												{/if}

												<!-- Recurring Totals for Subscriptions -->
												{#if hasSubscriptions}
													<div class="cart-table-wrapper">
														<table class="cart-table">
															<tbody>
																<tr class="recurring-totals">
																	<th colspan="2">Recurring Totals</th>
																</tr>
																{#each $cartStore.items.filter((i) => i.interval) as item}
																	<tr class="cart-subtotal recurring-total">
																		<th rowspan="1">Subtotal</th>
																		<td data-title="Subtotal">
																			<span class="rtp-price-amount amount">
																				{formatPrice(item.price)}
																			</span>
																			{getIntervalText(item.interval)}
																		</td>
																	</tr>
																{/each}
																<tr class="tax-total recurring-total">
																	<th>Tax</th>
																	<td data-title="Tax">Calculated at checkout</td>
																</tr>
															</tbody>
														</table>
													</div>
												{/if}

												<!-- Proceed to Checkout -->
												<div class="cart-proceed-to-checkout">
													<button
														type="button"
														class="btn btn-lg btn-block btn-orange"
														onclick={proceedToCheckout}
													>
														Proceed to checkout
													</button>
												</div>

												<p class="tax-disclaimer">* Plus applicable taxes</p>

												<!-- Coupon Input -->
												<div class="cart-coupon">
													<div class="input-group">
														<input
															type="text"
															name="coupon_code"
															class="form-control input-sm"
															id="coupon_code"
															bind:value={couponCode}
															placeholder="Coupon code"
															disabled={applyingCoupon}
														/>
														<span class="input-group-btn">
															<button
																type="button"
																class="btn btn-sm btn-blue"
																onclick={applyCouponCode}
																disabled={applyingCoupon}
															>
																{applyingCoupon ? '...' : 'Apply'}
															</button>
														</span>
													</div>
													{#if couponError}
														<p class="coupon-error">{couponError}</p>
													{/if}
												</div>

												<!-- Security Badge -->
												<div class="security-badge">
													<IconShieldCheck size={13} />
													<span>Secure checkout guaranteed</span>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</form>
			{/if}
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Revolution Trading Custom Cart System (EXACT MATCH)
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES - Revolution Trading Colors
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--st-bg: #f4f4f4;
		--st-card-bg: #ffffff;
		--st-border: #dbdbdb;
		--st-text: #333333;
		--st-text-dark: #1a1a1a;
		--st-text-muted: #666666;
		--st-primary: #0984ae;
		--st-primary-hover: #076787;
		--st-orange: #f99e31;
		--st-orange-hover: #f88b09;
		--st-blue: #0984ae;
		--st-success: #10b981;
		--st-error: #dc3545;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE LAYOUT - Revolution Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.rtp-cart {
		background: var(--st-bg);
		padding: 40px 0;
		font-family:
			'Open Sans',
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto,
			sans-serif;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cart-page-header {
		margin-bottom: 30px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: var(--st-primary);
		text-decoration: none;
		font-weight: 700;
		font-size: 14px;
		margin-bottom: 12px;
		transition: color 0.15s ease;
	}

	.back-link:hover {
		color: var(--st-primary-hover);
	}

	.page-title {
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: var(--st-text-dark);
		margin: 0 0 8px;
		text-transform: uppercase;
	}

	.cart-count {
		color: var(--st-text-muted);
		font-size: 16px;
		font-weight: 500;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY CART
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cart-empty .card {
		background: var(--st-card-bg);
		border-radius: 5px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.cart-empty .card-body {
		padding: 80px 40px;
		text-align: center;
	}

	.cart-empty-icon {
		width: 120px;
		height: 120px;
		margin: 0 auto 24px;
		background: var(--st-bg);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--st-text-muted);
	}

	.cart-empty h2 {
		font-size: 28px;
		font-weight: 700;
		color: var(--st-text-dark);
		margin: 0 0 12px;
	}

	.cart-empty p {
		color: var(--st-text-muted);
		font-size: 16px;
		margin: 0 0 24px;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART FORM LAYOUT - Revolution Trading Row Structure
	   ═══════════════════════════════════════════════════════════════════════════ */

	.rtp-cart-form__contents {
		display: block;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-xs-12 {
		padding: 0 15px;
	}

	.col-sm-7 {
		flex: 0 0 58.333333%;
		max-width: 58.333333%;
	}

	.col-md-8 {
		flex: 0 0 66.666667%;
		max-width: 66.666667%;
	}

	.col-sm-5 {
		flex: 0 0 41.666667%;
		max-width: 41.666667%;
	}

	.col-md-4 {
		flex: 0 0 33.333333%;
		max-width: 33.333333%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRODUCTS - Card Layout (Simpler Trading Style)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.products {
		margin-bottom: 20px;
	}

	.flex-grid {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.product.cart_item {
		display: block;
	}

	.product .card {
		background: var(--st-card-bg);
		border-radius: 5px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		display: grid;
		grid-template-columns: 120px 1fr auto;
		overflow: hidden;
	}

	/* Card Media - Product Image */
	.card-media {
		margin: 0;
		grid-row: 1 / 3;
	}

	.card-image {
		width: 120px;
		height: 100%;
		min-height: 120px;
		background: var(--st-bg);
		background-size: cover;
		background-position: center;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--st-text-muted);
	}

	/* Card Body - Product Details */
	.card-body {
		padding: 20px;
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	.card-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--st-text-dark);
		margin: 0 0 8px;
	}

	.card-description {
		font-size: 14px;
		color: var(--st-text-muted);
	}

	.membership-tagline {
		display: block;
	}

	.membership-tagline .bold {
		font-weight: 700;
		color: var(--st-text-dark);
	}

	.subscription-details {
		color: var(--st-text-muted);
	}

	.product-quantity {
		display: none;
	}

	/* Card Footer - Remove & Price */
	.card-footer {
		padding: 20px;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		justify-content: center;
		gap: 12px;
		border-left: 1px solid #eeeeee;
	}

	.product-remove .btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
	}

	.product-price {
		font-size: 18px;
		font-weight: 700;
		color: var(--st-text-dark);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CLEAR CART
	   ═══════════════════════════════════════════════════════════════════════════ */

	.clear-cart {
		margin-top: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART TOTALS SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cart-collaterals {
		position: sticky;
		top: 24px;
	}

	.cart-collaterals .card {
		background: var(--st-card-bg);
		border-radius: 5px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.cart-collaterals .card-body {
		padding: 24px;
	}

	/* Cart totals container */
	.cart_totals {
		display: block;
	}

	/* Order Total - Simpler Trading Style */
	.order-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 16px;
		border-bottom: 2px solid var(--st-text-dark);
		font-size: 20px;
		font-weight: 700;
		color: var(--st-text-dark);
	}

	.order-total-price {
		font-size: 24px;
	}

	.order-total-price .original-price {
		font-size: 16px;
		text-decoration: line-through;
		color: var(--st-text-muted);
		margin-right: 8px;
	}

	/* Coupon Applied */
	.coupon-applied {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 12px;
		padding: 10px;
		background: #ecfdf5;
		border-radius: 5px;
		font-size: 14px;
	}

	.coupon-label {
		flex: 1;
		font-weight: 600;
		color: var(--st-text-dark);
	}

	.coupon-discount {
		font-weight: 700;
		color: var(--st-success);
	}

	.remove-coupon {
		background: none;
		border: none;
		color: var(--st-error);
		cursor: pointer;
		padding: 4px;
	}

	/* Recurring Totals Table */
	.cart-table-wrapper {
		margin-top: 20px;
		padding-top: 16px;
		border-top: 1px solid #eeeeee;
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
	}

	.cart-table {
		width: 100%;
		min-width: 280px;
	}

	.cart-table th,
	.cart-table td {
		padding: 10px 0;
		font-size: 14px;
	}

	.cart-table th {
		text-align: left;
		font-weight: 700;
		color: var(--st-text-dark);
	}

	.cart-table td {
		text-align: right;
		color: var(--st-text-dark);
		font-weight: 600;
	}

	.recurring-totals th {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--st-text-muted) !important;
		font-weight: 600;
	}

	.tax-total td {
		color: var(--st-text-muted);
		font-weight: 500;
	}

	/* Proceed to Checkout */
	.cart-proceed-to-checkout {
		margin-top: 20px;
	}

	.tax-disclaimer {
		margin-top: 12px;
		font-size: 12px;
		color: var(--st-text-muted);
		text-align: center;
		font-weight: 500;
	}

	/* Coupon Input */
	.cart-coupon {
		margin-top: 20px;
		padding-top: 20px;
		border-top: 1px solid #eeeeee;
	}

	.input-group {
		display: flex;
		gap: 0;
	}

	.input-group .form-control {
		flex: 1;
		padding: 12px 14px;
		border: 1px solid var(--st-border);
		border-radius: 5px 0 0 5px;
		font-size: 16px;
		text-transform: uppercase;
		font-weight: 600;
		min-height: 44px;
	}

	.input-group .form-control:focus {
		outline: none;
		border-color: var(--st-primary);
	}

	.input-group-btn .btn {
		border-radius: 0 5px 5px 0;
	}

	.coupon-error {
		margin-top: 8px;
		font-size: 13px;
		color: var(--st-error);
		font-weight: 600;
	}

	/* Security Badge */
	.security-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		margin-top: 20px;
		padding-top: 16px;
		border-top: 1px solid #eeeeee;
		color: var(--st-success);
		font-size: 13px;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTONS - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 700;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
		text-decoration: none;
	}

	.btn-lg {
		padding: 14px 28px;
		font-size: 16px;
	}

	.btn-sm {
		padding: 10px 16px;
		font-size: 14px;
	}

	.btn-xs {
		padding: 10px 16px;
		font-size: 13px;
		min-height: 44px;
	}

	.btn-block {
		width: 100%;
	}

	.btn-orange {
		background: var(--st-orange);
		color: #ffffff;
	}

	.btn-orange:hover:not(:disabled) {
		background: var(--st-orange-hover);
	}

	.btn-orange:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-blue {
		background: var(--st-blue);
		color: #ffffff;
	}

	.btn-blue:hover:not(:disabled) {
		background: var(--st-primary-hover);
	}

	.btn-default {
		background: var(--st-bg);
		color: var(--st-text-dark);
		border: 1px solid var(--st-border);
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	/* RTP Price Styling */
	.rtp-price-amount {
		font-weight: 700;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.col-sm-7,
		.col-md-8 {
			flex: 0 0 100%;
			max-width: 100%;
			margin-bottom: 24px;
		}

		.col-sm-5,
		.col-md-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.cart-collaterals {
			position: static;
		}
	}

	@media screen and (max-width: 768px) {
		.product .card {
			grid-template-columns: 80px 1fr;
		}

		.card-footer {
			grid-column: 1 / -1;
			flex-direction: row;
			justify-content: space-between;
			border-left: none;
			border-top: 1px solid #eeeeee;
		}

		.card-image {
			width: 80px;
			min-height: 80px;
		}

		.page-title {
			font-size: 28px;
		}
	}

	@media screen and (max-width: 480px) {
		.rtp-cart {
			padding: 20px 0;
		}

		.cart-collaterals .card-body {
			padding: 16px;
		}

		.cart-empty .card-body {
			padding: 40px 20px;
		}

		.product .card {
			grid-template-columns: 1fr;
		}

		.card-media {
			grid-row: auto;
		}

		.card-image {
			width: 100%;
			height: 160px;
		}

		.card-footer {
			flex-direction: row;
			justify-content: space-between;
		}
	}
</style>
