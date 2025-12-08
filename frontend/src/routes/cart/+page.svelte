<script lang="ts">
	/**
	 * Shopping Cart Page - WordPress Revolution Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches the WooCommerce cart layout from Revolution Trading
	 * Uses existing cart store for state management
	 *
	 * @version 4.0.0 (WordPress-style / December 2025)
	 */

	import { goto } from '$app/navigation';
	import { cartStore, cartItemCount, cartTotal } from '$lib/stores/cart';
	import { validateCoupon, type CouponType } from '$lib/api/coupons';
	import { isAuthenticated } from '$lib/stores/auth';
	import {
		IconTrash,
		IconPlus,
		IconMinus,
		IconShoppingCart,
		IconArrowLeft,
		IconArrowRight,
		IconTicket,
		IconX,
		IconShieldCheck
	} from '@tabler/icons-svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let couponCode = $state('');
	let appliedCoupon = $state<{ code: string; discount: number; type: CouponType } | null>(null);
	let couponError = $state('');
	let applyingCoupon = $state(false);
	let couponFormVisible = $state(false);

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

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

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

	function getIntervalLabel(interval?: string): string {
		switch (interval) {
			case 'monthly': return '/ month';
			case 'quarterly': return '/ 3 months';
			case 'yearly': return '/ year';
			default: return '';
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
				couponFormVisible = false;
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
</script>

<svelte:head>
	<title>Cart | Revolution Trading Pros</title>
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - WordPress WooCommerce Cart Style
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="woocommerce-cart woocommerce-page">
	<div class="container">
		<!-- Page Header -->
		<header class="cart-page-header">
			<a href="/" class="back-link">
				<IconArrowLeft size={18} />
				<span>Continue Shopping</span>
			</a>
			<h1 class="page-title">Shopping Cart</h1>
			<p class="cart-count">{$cartItemCount} {$cartItemCount === 1 ? 'item' : 'items'}</p>
		</header>

		{#if $cartStore.items.length === 0}
			<!-- Empty Cart State -->
			<div class="cart-empty">
				<div class="cart-empty-icon">
					<IconShoppingCart size={64} />
				</div>
				<h2>Your cart is empty</h2>
				<p>Looks like you haven't added any products to your cart yet.</p>
				<a href="/live-trading-rooms" class="btn btn-orange">
					Browse Trading Rooms
					<IconArrowRight size={18} />
				</a>
			</div>
		{:else}
			<!-- Cart Content -->
			<div class="cart-content">
				<!-- Cart Items Table -->
				<div class="cart-items-section">
					<form class="woocommerce-cart-form">
						<table class="shop_table cart" cellspacing="0">
							<thead>
								<tr>
									<th class="product-remove">&nbsp;</th>
									<th class="product-thumbnail">&nbsp;</th>
									<th class="product-name">Product</th>
									<th class="product-price">Price</th>
									<th class="product-quantity">Quantity</th>
									<th class="product-subtotal">Subtotal</th>
								</tr>
							</thead>
							<tbody>
								{#each $cartStore.items as item (item.id + (item.interval || ''))}
									<tr class="cart_item">
										<!-- Remove -->
										<td class="product-remove">
											<button
												type="button"
												class="remove"
												aria-label="Remove this item"
												onclick={() => removeItem(item.id, item.interval)}
											>
												<IconX size={18} />
											</button>
										</td>

										<!-- Thumbnail -->
										<td class="product-thumbnail">
											<div class="product-image" style:background-image={item.image ? `url(${item.image})` : undefined}>
												{#if !item.image}
													<IconShoppingCart size={24} />
												{/if}
											</div>
										</td>

										<!-- Name -->
										<td class="product-name" data-title="Product">
											<span class="product-title">{item.name}</span>
											{#if item.interval}
												<span class="subscription-interval">
													Billed {item.interval}
												</span>
											{/if}
										</td>

										<!-- Price -->
										<td class="product-price" data-title="Price">
											<span class="woocommerce-Price-amount amount">
												{formatPrice(item.price)}
											</span>
											{#if item.interval}
												<span class="interval-label">{getIntervalLabel(item.interval)}</span>
											{/if}
										</td>

										<!-- Quantity -->
										<td class="product-quantity" data-title="Quantity">
											<div class="quantity-controls">
												<button
													type="button"
													class="qty-btn minus"
													onclick={() => updateQuantity(item.id, item.quantity - 1, item.interval)}
													disabled={item.quantity <= 1}
												>
													<IconMinus size={14} />
												</button>
												<input
													type="number"
													class="qty"
													value={item.quantity}
													min="1"
													readonly
												/>
												<button
													type="button"
													class="qty-btn plus"
													onclick={() => updateQuantity(item.id, item.quantity + 1, item.interval)}
												>
													<IconPlus size={14} />
												</button>
											</div>
										</td>

										<!-- Subtotal -->
										<td class="product-subtotal" data-title="Subtotal">
											<span class="woocommerce-Price-amount amount">
												{formatPrice(item.price * item.quantity)}
											</span>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</form>
				</div>

				<!-- Cart Totals Sidebar -->
				<div class="cart-totals-section">
					<div class="cart_totals">
						<h2>Cart totals</h2>

						<table class="shop_table shop_table_responsive" cellspacing="0">
							<tbody>
								<tr class="cart-subtotal">
									<th>Subtotal</th>
									<td data-title="Subtotal">
										<span class="woocommerce-Price-amount amount">{formatPrice($cartTotal)}</span>
									</td>
								</tr>

								{#if appliedCoupon && discountAmount > 0}
									<tr class="cart-discount coupon-{appliedCoupon.code.toLowerCase()}">
										<th>Coupon: {appliedCoupon.code}</th>
										<td data-title="Coupon">
											<span class="discount-amount">-{formatPrice(discountAmount)}</span>
											<button type="button" class="remove-coupon" onclick={removeCouponCode}>
												[Remove]
											</button>
										</td>
									</tr>
								{/if}

								<tr class="tax-total">
									<th>Tax</th>
									<td data-title="Tax">
										<span class="woocommerce-Price-amount amount">Calculated at checkout</span>
									</td>
								</tr>

								<tr class="order-total">
									<th>Total</th>
									<td data-title="Total">
										<strong>
											<span class="woocommerce-Price-amount amount">{formatPrice(finalTotal)}</span>
										</strong>
									</td>
								</tr>

								{#if $cartStore.items.some(i => i.interval)}
									<tr class="recurring-totals">
										<th colspan="2">
											<span class="recurring-label">Recurring Totals</span>
										</th>
									</tr>
									{#each $cartStore.items.filter(i => i.interval) as item}
										<tr class="recurring-total">
											<td colspan="2">
												<span class="recurring-amount">
													{formatPrice(item.price)} {getIntervalLabel(item.interval)}
												</span>
											</td>
										</tr>
									{/each}
								{/if}
							</tbody>
						</table>

						<!-- Coupon Section -->
						<div class="coupon-section">
							{#if !appliedCoupon}
								<div class="woocommerce-form-coupon-toggle">
									<button
										type="button"
										class="showcoupon"
										onclick={() => couponFormVisible = !couponFormVisible}
									>
										<IconTicket size={16} />
										Have a coupon? Click here to enter your code
									</button>
								</div>

								{#if couponFormVisible}
									<div class="checkout_coupon woocommerce-form-coupon">
										<p class="form-row form-row-first">
											<input
												type="text"
												name="coupon_code"
												class="input-text"
												placeholder="Coupon code"
												bind:value={couponCode}
												onkeypress={(e) => e.key === 'Enter' && applyCouponCode()}
												disabled={applyingCoupon}
											/>
										</p>
										<p class="form-row form-row-last">
											<button
												type="button"
												class="btn btn-default"
												onclick={applyCouponCode}
												disabled={applyingCoupon}
											>
												{applyingCoupon ? 'Applying...' : 'Apply coupon'}
											</button>
										</p>
										{#if couponError}
											<p class="coupon-error">{couponError}</p>
										{/if}
									</div>
								{/if}
							{/if}
						</div>

						<!-- Proceed to Checkout -->
						<div class="wc-proceed-to-checkout">
							<button
								type="button"
								class="checkout-button btn btn-orange btn-block"
								onclick={proceedToCheckout}
							>
								Proceed to checkout
								<IconArrowRight size={18} />
							</button>
						</div>

						<!-- Security Badge -->
						<div class="security-badge">
							<IconShieldCheck size={20} />
							<span>Secure checkout guaranteed</span>
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress WooCommerce Cart
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--cart-bg: #f4f4f4;
		--cart-card-bg: #fff;
		--cart-border: #dbdbdb;
		--cart-text: #333;
		--cart-text-muted: #666;
		--cart-primary: #0984ae;
		--cart-orange: #f99e31;
		--cart-orange-hover: #f88b09;
		--cart-success: #10b981;
		--cart-error: #ef4444;
		--cart-transition: all 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.woocommerce-cart {
		min-height: 100vh;
		background: var(--cart-bg);
		padding: 40px 0;
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
		color: var(--cart-primary);
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
		margin-bottom: 16px;
		transition: var(--cart-transition);
	}

	.back-link:hover {
		color: #076787;
	}

	.page-title {
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: var(--cart-text);
		margin: 0 0 8px;
	}

	.cart-count {
		color: var(--cart-text-muted);
		font-size: 16px;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY CART
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cart-empty {
		background: var(--cart-card-bg);
		border-radius: 5px;
		padding: 80px 40px;
		text-align: center;
		box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
	}

	.cart-empty-icon {
		width: 120px;
		height: 120px;
		margin: 0 auto 24px;
		background: var(--cart-bg);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--cart-text-muted);
	}

	.cart-empty h2 {
		font-size: 28px;
		font-weight: 700;
		color: var(--cart-text);
		margin: 0 0 12px;
	}

	.cart-empty p {
		color: var(--cart-text-muted);
		font-size: 16px;
		margin: 0 0 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART CONTENT LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cart-content {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: 30px;
		align-items: start;
	}

	.cart-items-section {
		background: var(--cart-card-bg);
		border-radius: 5px;
		box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART TABLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.shop_table.cart {
		width: 100%;
		border-collapse: collapse;
	}

	.shop_table.cart thead th {
		background: #f9f9f9;
		padding: 16px 12px;
		text-align: left;
		font-size: 13px;
		font-weight: 600;
		color: var(--cart-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-bottom: 1px solid var(--cart-border);
	}

	.shop_table.cart tbody tr {
		border-bottom: 1px solid var(--cart-border);
	}

	.shop_table.cart tbody tr:last-child {
		border-bottom: none;
	}

	.shop_table.cart tbody td {
		padding: 20px 12px;
		vertical-align: middle;
	}

	/* Product Remove */
	.product-remove {
		width: 40px;
		text-align: center;
	}

	.product-remove .remove {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid var(--cart-border);
		border-radius: 4px;
		color: var(--cart-text-muted);
		cursor: pointer;
		transition: var(--cart-transition);
	}

	.product-remove .remove:hover {
		background: var(--cart-error);
		border-color: var(--cart-error);
		color: #fff;
	}

	/* Product Thumbnail */
	.product-thumbnail {
		width: 80px;
	}

	.product-image {
		width: 60px;
		height: 60px;
		background: var(--cart-bg);
		background-size: cover;
		background-position: center;
		border-radius: 5px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--cart-text-muted);
	}

	/* Product Name */
	.product-name .product-title {
		display: block;
		font-weight: 600;
		color: var(--cart-text);
		font-size: 15px;
	}

	.product-name .subscription-interval {
		display: block;
		font-size: 12px;
		color: var(--cart-primary);
		margin-top: 4px;
	}

	/* Product Price */
	.product-price .interval-label {
		display: block;
		font-size: 12px;
		color: var(--cart-text-muted);
	}

	/* Quantity Controls */
	.quantity-controls {
		display: inline-flex;
		align-items: center;
		border: 1px solid var(--cart-border);
		border-radius: 5px;
		overflow: hidden;
	}

	.qty-btn {
		width: 32px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f9f9f9;
		border: none;
		cursor: pointer;
		color: var(--cart-text);
		transition: var(--cart-transition);
	}

	.qty-btn:hover:not(:disabled) {
		background: var(--cart-primary);
		color: #fff;
	}

	.qty-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.quantity-controls .qty {
		width: 50px;
		height: 36px;
		border: none;
		border-left: 1px solid var(--cart-border);
		border-right: 1px solid var(--cart-border);
		text-align: center;
		font-size: 14px;
		font-weight: 600;
		color: var(--cart-text);
		-moz-appearance: textfield;
	}

	.quantity-controls .qty::-webkit-outer-spin-button,
	.quantity-controls .qty::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	/* Product Subtotal */
	.product-subtotal .amount {
		font-weight: 700;
		font-size: 16px;
		color: var(--cart-text);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART TOTALS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cart-totals-section {
		position: sticky;
		top: 24px;
	}

	.cart_totals {
		background: var(--cart-card-bg);
		border-radius: 5px;
		padding: 24px;
		box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
	}

	.cart_totals h2 {
		font-size: 20px;
		font-weight: 700;
		color: var(--cart-text);
		margin: 0 0 20px;
		padding-bottom: 16px;
		border-bottom: 1px solid var(--cart-border);
	}

	.cart_totals .shop_table {
		width: 100%;
		margin-bottom: 20px;
	}

	.cart_totals .shop_table th,
	.cart_totals .shop_table td {
		padding: 12px 0;
		border-bottom: 1px solid #eee;
	}

	.cart_totals .shop_table th {
		text-align: left;
		font-weight: 500;
		color: var(--cart-text);
	}

	.cart_totals .shop_table td {
		text-align: right;
		color: var(--cart-text);
	}

	.cart_totals .order-total th,
	.cart_totals .order-total td {
		font-size: 18px;
		font-weight: 700;
		border-bottom: none;
		padding-top: 16px;
	}

	.cart_totals .discount-amount {
		color: var(--cart-success);
	}

	.cart_totals .remove-coupon {
		display: block;
		background: none;
		border: none;
		color: var(--cart-error);
		font-size: 12px;
		cursor: pointer;
		padding: 0;
		margin-top: 4px;
	}

	.recurring-totals th {
		font-size: 14px;
		color: var(--cart-text-muted) !important;
		padding-top: 16px !important;
	}

	.recurring-label {
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		font-size: 11px;
	}

	.recurring-total td {
		font-size: 14px;
		padding: 8px 0 !important;
		text-align: left !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COUPON SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.coupon-section {
		margin-bottom: 20px;
	}

	.showcoupon {
		display: flex;
		align-items: center;
		gap: 8px;
		background: none;
		border: none;
		color: var(--cart-primary);
		font-size: 14px;
		cursor: pointer;
		padding: 0;
		transition: var(--cart-transition);
	}

	.showcoupon:hover {
		color: #076787;
	}

	.checkout_coupon {
		margin-top: 16px;
		padding: 16px;
		background: #f9f9f9;
		border-radius: 5px;
	}

	.checkout_coupon .input-text {
		width: 100%;
		padding: 12px 16px;
		border: 1px solid var(--cart-border);
		border-radius: 5px;
		font-size: 14px;
		margin-bottom: 12px;
		text-transform: uppercase;
	}

	.checkout_coupon .input-text:focus {
		outline: none;
		border-color: var(--cart-primary);
	}

	.coupon-error {
		color: var(--cart-error);
		font-size: 13px;
		margin-top: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTONS
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
		transition: var(--cart-transition);
		text-decoration: none;
	}

	.btn-orange {
		background: var(--cart-orange);
		color: #fff;
	}

	.btn-orange:hover {
		background: var(--cart-orange-hover);
	}

	.btn-default {
		background: var(--cart-bg);
		color: var(--cart-text);
		border: 1px solid var(--cart-border);
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	.btn-block {
		width: 100%;
	}

	.checkout-button {
		padding: 16px 24px;
		font-size: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECURITY BADGE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.security-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid #eee;
		color: var(--cart-success);
		font-size: 13px;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.cart-content {
			grid-template-columns: 1fr;
		}

		.cart-totals-section {
			position: static;
		}
	}

	@media screen and (max-width: 768px) {
		.shop_table.cart thead {
			display: none;
		}

		.shop_table.cart tbody tr {
			display: grid;
			grid-template-columns: auto 1fr;
			gap: 12px;
			padding: 16px;
		}

		.shop_table.cart tbody td {
			padding: 0;
		}

		.product-remove {
			grid-column: 2;
			grid-row: 1;
			justify-self: end;
		}

		.product-thumbnail {
			grid-column: 1;
			grid-row: 1 / 4;
		}

		.product-name {
			grid-column: 2;
			grid-row: 1;
		}

		.product-price {
			grid-column: 2;
			grid-row: 2;
		}

		.product-quantity {
			grid-column: 1 / 3;
			grid-row: 4;
		}

		.product-subtotal {
			grid-column: 2;
			grid-row: 3;
		}

		.shop_table.cart tbody td::before {
			content: attr(data-title) ": ";
			font-weight: 600;
			margin-right: 8px;
		}

		.product-remove::before,
		.product-thumbnail::before {
			display: none;
		}

		.page-title {
			font-size: 28px;
		}
	}

	@media screen and (max-width: 480px) {
		.woocommerce-cart {
			padding: 20px 0;
		}

		.cart_totals {
			padding: 16px;
		}

		.cart-empty {
			padding: 40px 20px;
		}
	}
</style>
