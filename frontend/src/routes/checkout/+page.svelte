<script lang="ts">
	/**
	 * Checkout Page - WordPress Revolution Trading Style (Multi-Step)
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches the WooCommerce checkout layout from Revolution Trading:
	 * - 3-step wizard: Account → Billing → Payment
	 * - Cart sidebar showing products
	 * - Coupon toggle form
	 * - Stripe/PayPal payment integration
	 *
	 * @version 4.0.0 (WordPress-style / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { user, isAuthenticated } from '$lib/stores/auth.svelte';
	import { cartStore, getCartTotal } from '$lib/stores/cart.svelte';
	import { createCheckoutSession } from '$lib/api/cart';
	import { validateCoupon, type CouponType } from '$lib/api/coupons';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconShieldCheck from '@tabler/icons-svelte/icons/shield-check';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconTicket from '@tabler/icons-svelte/icons/ticket';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconLoader from '@tabler/icons-svelte/icons/loader';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	type CheckoutStep = 'billing' | 'payment';

	interface BillingDetails {
		firstName: string;
		lastName: string;
		email: string;
		phone: string;
		address1: string;
		address2: string;
		city: string;
		state: string;
		postcode: string;
		country: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let currentStep = $state<CheckoutStep>('billing');
	let isProcessing = $state(false);
	let paymentMethod = $state<'stripe' | 'paypal'>('stripe');

	// Billing details
	let billing = $state<BillingDetails>({
		firstName: '',
		lastName: '',
		email: '',
		phone: '',
		address1: '',
		address2: '',
		city: '',
		state: '',
		postcode: '',
		country: 'US'
	});

	// Coupon state
	let couponCode = $state('');
	let appliedCoupon = $state<{ code: string; discount: number; type: CouponType } | null>(null);
	let couponError = $state('');
	let applyingCoupon = $state(false);
	let couponFormVisible = $state(false);
	let checkoutError = $state('');

	// Form validation
	let billingErrors = $state<Partial<Record<keyof BillingDetails, string>>>({});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	let discountAmount = $derived(
		appliedCoupon
			? appliedCoupon.type === 'percentage'
				? (getCartTotal() * appliedCoupon.discount) / 100
				: appliedCoupon.discount
			: 0
	);

	let finalTotal = $derived(Math.max(0, getCartTotal() - discountAmount));

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;

		// Redirect to cart if empty
		if (cartStore.items.length === 0) {
			goto('/cart');
			return;
		}

		// Redirect to login if not authenticated
		if (!$isAuthenticated) {
			goto('/login?redirect=/checkout');
			return;
		}

		// Pre-fill billing from user data (with defensive null checks)
		if ($user) {
			const nameParts = ($user?.name || '').split(' ');
			billing.firstName = nameParts[0] || '';
			billing.lastName = nameParts.slice(1).join(' ') || '';
			billing.email = $user?.email || '';
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(price);
	}

	function getIntervalLabel(interval?: string): string {
		switch (interval) {
			case 'monthly':
				return '/ month';
			case 'quarterly':
				return 'every 3 months';
			case 'yearly':
				return '/ year';
			default:
				return '';
		}
	}

	function validateBilling(): boolean {
		const errors: Partial<Record<keyof BillingDetails, string>> = {};

		if (!billing.firstName.trim()) errors.firstName = 'First name is required';
		if (!billing.lastName.trim()) errors.lastName = 'Last name is required';
		if (!billing.email.trim()) errors.email = 'Email is required';
		if (billing.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(billing.email)) {
			errors.email = 'Please enter a valid email';
		}
		if (!billing.address1.trim()) errors.address1 = 'Address is required';
		if (!billing.city.trim()) errors.city = 'City is required';
		if (!billing.state.trim()) errors.state = 'State is required';
		if (!billing.postcode.trim()) errors.postcode = 'ZIP code is required';

		billingErrors = errors;
		return Object.keys(errors).length === 0;
	}

	function goToPayment() {
		if (validateBilling()) {
			currentStep = 'payment';
			checkoutError = '';
			window.scrollTo({ top: 0, behavior: 'smooth' });
			requestAnimationFrame(() => {
				document.getElementById('payment_stripe')?.focus();
			});
		}
	}

	function goToBilling() {
		currentStep = 'billing';
		checkoutError = '';
		window.scrollTo({ top: 0, behavior: 'smooth' });
		requestAnimationFrame(() => {
			document.getElementById('billing_first_name')?.focus();
		});
	}

	async function applyCouponCode() {
		if (!couponCode.trim()) {
			couponError = 'Please enter a coupon code';
			return;
		}

		applyingCoupon = true;
		couponError = '';

		try {
			const result = await validateCoupon(couponCode.trim().toUpperCase(), getCartTotal());

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
		} catch {
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

	async function placeOrder() {
		isProcessing = true;

		try {
			// Create checkout session with billing details
			const session = await createCheckoutSession({
				billing,
				couponCode: appliedCoupon?.code,
				provider: paymentMethod === 'paypal' ? 'paypal' : 'stripe'
			});

			// Redirect to payment processor
			if (session.url) {
				// Cart cleared on /checkout/success after payment confirmed
				window.location.href = session.url;
			}
		} catch (error) {
			console.error('Checkout error:', error);
			checkoutError =
				error instanceof Error ? error.message : 'Failed to process order. Please try again.';
		} finally {
			isProcessing = false;
		}
	}
</script>

<svelte:head>
	<title>Checkout | Revolution Trading Pros</title>
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - WordPress WooCommerce Checkout Style
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="woocommerce-checkout woocommerce-page">
	<div class="container">
		<!-- Page Header -->
		<header class="checkout-page-header">
			<a href="/cart" class="back-link">
				<IconArrowLeft size={18} />
				<span>Back to Cart</span>
			</a>
			<h1 class="page-title">Checkout</h1>
		</header>

		<!-- Checkout Steps Navigation -->
		<nav class="checkout-steps-nav">
			<ul>
				<li id="nav-checkout-account" class="completed">
					<span class="step-number"><IconCheck size={16} /></span>
					<span class="step-label">Sign In</span>
				</li>
				<li
					id="nav-checkout-billing"
					class:active={currentStep === 'billing'}
					class:completed={currentStep === 'payment'}
				>
					<span class="step-number">
						{#if currentStep === 'payment'}
							<IconCheck size={16} />
						{:else}
							2
						{/if}
					</span>
					<span class="step-label">Billing Info</span>
				</li>
				<li
					id="nav-checkout-payment"
					class:active={currentStep === 'payment'}
					class:disabled={currentStep === 'billing'}
				>
					<span class="step-number">3</span>
					<span class="step-label">Payment</span>
				</li>
			</ul>
		</nav>

		<!-- Checkout Content -->
		<div class="checkout-content">
			<!-- Main Form Area -->
			<div class="checkout-form-section">
				<form
					class="checkout woocommerce-checkout"
					onsubmit={(e: SubmitEvent) => e.preventDefault()}
				>
					<!-- Step 1: Billing -->
					<div id="checkout-billing" class="checkout-step" class:active={currentStep === 'billing'}>
						<div class="card">
							<div class="card-body">
								<h3>Billing details</h3>

								<!-- Logged in user info -->
								<div class="user-info-box">
									<IconCheck size={20} />
									<div>
										<p class="label">Signed in as:</p>
										<p class="value">{$user?.name || $user?.email}</p>
									</div>
								</div>

								<div class="billing-fields">
									<div class="form-row form-row-first">
										<label for="billing_first_name"
											>First name <abbr class="required">*</abbr></label
										>
										<input
											type="text"
											id="billing_first_name" name="billing_first_name"
											bind:value={billing.firstName}
											class="input-text"
											class:error={billingErrors.firstName}
										/>
										{#if billingErrors.firstName}
											<span class="field-error">{billingErrors.firstName}</span>
										{/if}
									</div>

									<div class="form-row form-row-last">
										<label for="billing_last_name">Last name <abbr class="required">*</abbr></label>
										<input
											type="text"
											id="billing_last_name" name="billing_last_name"
											bind:value={billing.lastName}
											class="input-text"
											class:error={billingErrors.lastName}
										/>
										{#if billingErrors.lastName}
											<span class="field-error">{billingErrors.lastName}</span>
										{/if}
									</div>

									<div class="form-row form-row-wide">
										<label for="billing_email">Email address <abbr class="required">*</abbr></label>
										<input
											type="email"
											id="billing_email" name="billing_email" autocomplete="email"
											bind:value={billing.email}
											class="input-text"
											class:error={billingErrors.email}
										/>
										{#if billingErrors.email}
											<span class="field-error">{billingErrors.email}</span>
										{/if}
									</div>

									<div class="form-row form-row-wide">
										<label for="billing_phone">Phone <span class="optional">(optional)</span></label
										>
										<input
											type="tel"
											id="billing_phone" name="billing_phone" autocomplete="tel"
											bind:value={billing.phone}
											class="input-text"
										/>
									</div>

									<div class="form-row form-row-wide">
										<label for="billing_address_1"
											>Street address <abbr class="required">*</abbr></label
										>
										<input
											type="text"
											id="billing_address_1" name="billing_address_1"
											bind:value={billing.address1}
											placeholder="House number and street name"
											class="input-text"
											class:error={billingErrors.address1}
										/>
										{#if billingErrors.address1}
											<span class="field-error">{billingErrors.address1}</span>
										{/if}
									</div>

									<div class="form-row form-row-wide">
										<label for="billing_address_2" class="screen-reader-text"
											>Apartment, suite, unit, etc.</label
										>
										<input
											type="text"
											id="billing_address_2" name="billing_address_2"
											bind:value={billing.address2}
											placeholder="Apartment, suite, unit, etc. (optional)"
											class="input-text"
										/>
									</div>

									<div class="form-row form-row-wide">
										<label for="billing_city">Town / City <abbr class="required">*</abbr></label>
										<input
											type="text"
											id="billing_city" name="billing_city"
											bind:value={billing.city}
											class="input-text"
											class:error={billingErrors.city}
										/>
										{#if billingErrors.city}
											<span class="field-error">{billingErrors.city}</span>
										{/if}
									</div>

									<div class="form-row form-row-first">
										<label for="billing_state">State <abbr class="required">*</abbr></label>
										<input
											type="text"
											id="billing_state" name="billing_state"
											bind:value={billing.state}
											class="input-text"
											class:error={billingErrors.state}
										/>
										{#if billingErrors.state}
											<span class="field-error">{billingErrors.state}</span>
										{/if}
									</div>

									<div class="form-row form-row-last">
										<label for="billing_postcode">ZIP Code <abbr class="required">*</abbr></label>
										<input
											type="text"
											id="billing_postcode" name="billing_postcode"
											bind:value={billing.postcode}
											class="input-text"
											class:error={billingErrors.postcode}
										/>
										{#if billingErrors.postcode}
											<span class="field-error">{billingErrors.postcode}</span>
										{/if}
									</div>
								</div>
							</div>

							<div class="card-footer">
								<div class="checkout-steps-actions">
									<a href="/cart" class="btn btn-default back-to-cart">
										<IconArrowLeft size={16} />
										Back to Cart
									</a>
									<button type="button" class="btn btn-orange submit-billing" onclick={goToPayment}>
										Continue to Payment
										<IconArrowRight size={16} />
									</button>
								</div>
							</div>
						</div>
					</div>

					<!-- Step 2: Payment -->
					<div id="checkout-payment" class="checkout-step" class:active={currentStep === 'payment'}>
						<div class="card">
							<div class="card-body">
								<h3>Payment Method</h3>

								{#if checkoutError}
									<div class="checkout-error" role="alert">
										<IconX size={18} />
										<span>{checkoutError}</span>
										<button
											type="button"
											onclick={() => (checkoutError = '')}
											aria-label="Dismiss error"
										>
											<IconX size={14} />
										</button>
									</div>
								{/if}

								<div class="payment-methods">
									<!-- Credit Card (Stripe) -->
									<div
										class="payment-method"
										class:selected={paymentMethod === 'stripe'}
										onclick={() => (paymentMethod = 'stripe')}
										onkeydown={(e: KeyboardEvent) => {
											if (e.key === 'Enter' || e.key === ' ') {
												e.preventDefault();
												paymentMethod = 'stripe';
											}
										}}
										role="button"
										tabindex="0"
									>
										<input
											type="radio"
											id="payment_stripe"
											name="payment_method"
											value="stripe"
											checked={paymentMethod === 'stripe'}
											onchange={() => (paymentMethod = 'stripe')}
										/>
										<label for="payment_stripe">
											<IconCreditCard size={24} />
											<span class="method-title">Credit Card</span>
											<span class="method-description"
												>Pay securely with your credit or debit card</span
											>
										</label>
										<div class="card-icons">
											<img
												src="https://js.stripe.com/v3/fingerprinted/img/visa-729c05c240c4bdb47b03ac81d9945bfe.svg"
												alt="Visa"
											/>
											<img
												src="https://js.stripe.com/v3/fingerprinted/img/mastercard-4d8844094130711885b5e41b28c9848f.svg"
												alt="Mastercard"
											/>
											<img
												src="https://js.stripe.com/v3/fingerprinted/img/amex-a49b82f46c5cd6a96a6e418a6ca1717c.svg"
												alt="Amex"
											/>
										</div>
									</div>

									<!-- PayPal - Coming Soon -->
									<div
										class="payment-method payment-method--disabled"
										title="PayPal integration coming soon"
										role="button"
										tabindex="-1"
									>
										<input
											type="radio"
											id="payment_paypal"
											name="payment_method"
											value="paypal"
											disabled
										/>
										<label for="payment_paypal">
											<svg
												xmlns="http://www.w3.org/2000/svg"
												viewBox="0 0 24 24"
												fill="currentColor"
												width="24"
												height="24"
											>
												<path
													d="M20.067 8.478c.492.88.556 2.014.3 3.327-.74 3.806-3.276 5.12-6.514 5.12h-.5a.805.805 0 00-.794.68l-.04.22-.63 3.993-.032.17a.804.804 0 01-.794.679H7.72a.483.483 0 01-.477-.558L7.418 21h1.518l.95-6.02h1.385c4.678 0 7.75-2.203 8.796-6.502zm-2.96-5.09c.762.868.983 1.81.752 3.285-.019.123-.04.24-.062.36-.735 3.773-3.089 5.446-6.956 5.446H8.957a1.01 1.01 0 00-.996.852l-.68 4.312-.472 2.987H4.907a.573.573 0 01-.567-.663l2.01-12.748a1.208 1.208 0 011.194-1.02h5.862c1.5 0 2.637.182 3.701.19z"
												/>
											</svg>
											<span class="method-title">PayPal</span>
											<span class="method-description method-description--coming-soon">
												Coming Soon
											</span>
										</label>
									</div>
								</div>

								<div class="security-notice">
									<IconLock size={18} />
									<span
										>Your payment information is encrypted and secure. We never store your card
										details.</span
									>
								</div>
							</div>

							<div class="card-footer">
								<div class="checkout-steps-actions">
									<button
										type="button"
										class="btn btn-default back-to-billing"
										onclick={goToBilling}
									>
										<IconArrowLeft size={16} />
										Back to Billing
									</button>
									<button
										type="button"
										class="btn btn-orange place-order"
										onclick={placeOrder}
										disabled={isProcessing}
									>
										{#if isProcessing}
											<IconLoader size={18} class="spin" />
											Processing...
										{:else}
											<IconShieldCheck size={18} />
											Place Order
										{/if}
									</button>
								</div>
							</div>
						</div>
					</div>
				</form>
			</div>

			<!-- Cart Sidebar -->
			<div class="checkout-cart-section">
				<div class="checkout-cart">
					<div class="card">
						<!-- Header -->
						<div class="card-header">
							<h4>Shopping Cart</h4>
							<a href="/cart" class="btn btn-xs btn-default">Edit</a>
						</div>

						<!-- Products - with thumbnails -->
						<div class="checkout-cart-contents">
							<div class="checkout-cart-products">
								{#each cartStore.items as item (item.id + (item.interval || ''))}
									<div class="product">
										<div
											class="product-image"
											style:background-image={item.thumbnail || item.image
												? `url(${item.thumbnail || item.image})`
												: undefined}
										>
											{#if !item.thumbnail && !item.image}
												<div class="product-placeholder"></div>
											{/if}
										</div>
										<div class="product-details">
											<div class="product-name">
												{item.name}
												{#if item.interval}
													<span class="product-interval">({item.interval})</span>
												{/if}
											</div>
										</div>
										<div class="product-total">
											{formatPrice(item.price)}
										</div>
									</div>
								{/each}
							</div>

							<!-- Totals -->
							<table class="cart-table">
								<tbody>
									<tr class="cart-subtotal">
										<th>Subtotal</th>
										<td>{formatPrice(getCartTotal())}</td>
									</tr>

									{#if appliedCoupon && discountAmount > 0}
										<tr class="cart-discount">
											<th>
												Discount ({appliedCoupon.code})
												<button type="button" class="remove-coupon" onclick={removeCouponCode}>
													<IconX size={14} />
												</button>
											</th>
											<td class="discount">-{formatPrice(discountAmount)}</td>
										</tr>
									{/if}

									<tr class="tax-total">
										<th>Tax</th>
										<td>Calculated at next step</td>
									</tr>
								</tbody>
							</table>

							<div class="checkout-order-total">
								<span>Total</span>
								<span class="checkout-order-total-price">{formatPrice(finalTotal)}</span>
							</div>

							<!-- Recurring totals for subscriptions -->
							{#if cartStore.items.some((i) => i.interval)}
								<table class="cart-table recurring-table">
									<tbody>
										<tr class="recurring-totals-header">
											<th colspan="2">Recurring Totals</th>
										</tr>
										{#each cartStore.items.filter((i) => i.interval) as item}
											<tr class="recurring-total">
												<td colspan="2">
													{formatPrice(item.price)}
													{getIntervalLabel(item.interval)}
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							{/if}
						</div>

						<!-- Coupon -->
						<div class="coupon-section">
							{#if !appliedCoupon}
								<button
									type="button"
									class="showcoupon"
									onclick={() => (couponFormVisible = !couponFormVisible)}
								>
									<IconTicket size={16} />
									Have a coupon? Click here to enter your code
								</button>

								{#if couponFormVisible}
									<div class="checkout_coupon">
										<label for="coupon_code" class="sr-only">Coupon code</label>
										<input
											id="coupon_code" name="coupon_code"
											type="text"
											placeholder="Coupon code"
											bind:value={couponCode}
											onkeypress={(e: KeyboardEvent) => e.key === 'Enter' && applyCouponCode()}
											disabled={applyingCoupon}
										/>
										<button
											type="button"
											class="btn btn-default"
											onclick={applyCouponCode}
											disabled={applyingCoupon}
										>
											{applyingCoupon ? '...' : 'Apply'}
										</button>
										{#if couponError}
											<p class="coupon-error">{couponError}</p>
										{/if}
									</div>
								{/if}
							{/if}
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Terms -->
		<div class="checkout-terms">
			<p>
				By completing this purchase, you agree to our
				<a href="/terms">Terms of Service</a> and <a href="/privacy">Privacy Policy</a>.
			</p>
		</div>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress WooCommerce Checkout
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES - Simpler Trading Colors (EXACT MATCH)
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--checkout-bg: #f4f4f4;
		--checkout-card-bg: #ffffff;
		--checkout-border: #dbdbdb;
		--checkout-text: #333333;
		--checkout-text-dark: #1a1a1a;
		--checkout-text-muted: #666666;
		--checkout-primary: #0984ae;
		--checkout-primary-hover: #076787;
		--checkout-orange: #f99e31;
		--checkout-orange-hover: #f88b09;
		--checkout-success: #10b981;
		--checkout-error: #dc3545;
		--checkout-transition: all 0.15s ease;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE LAYOUT - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.woocommerce-checkout {
		background: var(--checkout-bg);
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

	.checkout-page-header {
		margin-bottom: 24px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: var(--checkout-primary);
		text-decoration: none;
		font-weight: 700;
		font-size: 14px;
		margin-bottom: 12px;
		transition: var(--checkout-transition);
	}

	.back-link:hover {
		color: var(--checkout-primary-hover);
	}

	.page-title {
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: var(--checkout-text-dark);
		margin: 0;
		text-transform: uppercase;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT STEPS NAVIGATION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-steps-nav {
		margin-bottom: 30px;
	}

	.checkout-steps-nav ul {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
		background: var(--checkout-card-bg);
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
	}

	.checkout-steps-nav li {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 10px;
		padding: 16px 20px;
		border-right: 1px solid var(--checkout-border);
		color: var(--checkout-text-dark);
		font-weight: 700;
		font-size: 14px;
		transition: var(--checkout-transition);
	}

	.checkout-steps-nav li:last-child {
		border-right: none;
	}

	.checkout-steps-nav li.active {
		background: var(--checkout-primary);
		color: #ffffff;
	}

	.checkout-steps-nav li.completed {
		color: var(--checkout-success);
	}

	.checkout-steps-nav li.disabled {
		opacity: 0.5;
		color: var(--checkout-text-muted);
	}

	.step-number {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px solid currentColor;
		border-radius: 50%;
		font-size: 12px;
		font-weight: 700;
	}

	.checkout-steps-nav li.active .step-number,
	.checkout-steps-nav li.completed .step-number {
		border-color: currentColor;
		background: currentColor;
		color: #fff;
	}

	.checkout-steps-nav li.completed .step-number {
		background: var(--checkout-success);
		border-color: var(--checkout-success);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT CONTENT LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-content {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: 30px;
		align-items: start;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT STEPS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-step {
		display: none;
	}

	.checkout-step.active {
		display: block;
	}

	.card {
		background: var(--checkout-card-bg);
		border-radius: 5px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.card-body {
		padding: 30px;
	}

	.card-body h3 {
		font-size: 20px;
		font-weight: 700;
		color: var(--checkout-text-dark);
		margin: 0 0 24px;
		padding-bottom: 16px;
		border-bottom: 1px solid var(--checkout-border);
	}

	.card-footer {
		padding: 20px 30px;
		background: #f9f9f9;
		border-top: 1px solid var(--checkout-border);
	}

	/* User Info Box */
	.user-info-box {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 16px;
		background: #e8f5e9;
		border-radius: 5px;
		margin-bottom: 24px;
		color: var(--checkout-success);
	}

	.user-info-box .label {
		font-size: 13px;
		font-weight: 600;
		margin: 0 0 4px;
		opacity: 0.8;
	}

	.user-info-box .value {
		font-weight: 700;
		margin: 0;
		color: var(--checkout-text-dark);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BILLING FORM
	   ═══════════════════════════════════════════════════════════════════════════ */

	.billing-fields {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-row {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-row-wide {
		grid-column: 1 / -1;
	}

	.form-row label {
		font-size: 14px;
		font-weight: 700;
		color: var(--checkout-text-dark);
	}

	.form-row label .required {
		color: var(--checkout-error);
		text-decoration: none;
	}

	.form-row label .optional {
		font-weight: 500;
		color: var(--checkout-text-muted);
	}

	/* Input Text - 2026 Mobile-First Responsive */
	.input-text {
		width: 100%;
		padding: 12px 16px;
		border: 1px solid var(--checkout-border);
		border-radius: 5px;
		font-size: 16px; /* Prevents iOS zoom on focus */
		font-weight: 600;
		color: var(--checkout-text-dark);
		transition: var(--checkout-transition);
		min-height: 44px; /* 2026 touch target minimum */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.input-text:focus {
		outline: none;
		border-color: var(--checkout-primary);
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.1);
	}

	.input-text.error {
		border-color: var(--checkout-error);
	}

	.field-error {
		font-size: 13px;
		color: var(--checkout-error);
	}

	.screen-reader-text {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		border: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAYMENT METHODS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.payment-methods {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-bottom: 24px;
	}

	/* Payment Method - 2026 Mobile-First with 44px touch target */
	.payment-method {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 20px;
		min-height: 44px;
		border: 2px solid var(--checkout-border);
		border-radius: 8px;
		cursor: pointer;
		transition: var(--checkout-transition);
		touch-action: manipulation;
	}

	.payment-method:hover {
		border-color: var(--checkout-primary);
	}

	.payment-method.selected {
		border-color: var(--checkout-primary);
		background: rgba(9, 132, 174, 0.05);
	}

	.payment-method--disabled {
		opacity: 0.6;
		cursor: not-allowed;
		background: var(--checkout-bg);
	}

	.payment-method--disabled:hover {
		border-color: var(--checkout-border);
	}

	.payment-method--disabled label {
		cursor: not-allowed;
	}

	.method-description--coming-soon {
		display: inline-block;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.payment-method input[type='radio'] {
		margin-top: 4px;
	}

	.payment-method label {
		flex: 1;
		cursor: pointer;
	}

	.payment-method .method-title {
		display: block;
		font-weight: 600;
		font-size: 16px;
		color: var(--checkout-text);
		margin-bottom: 4px;
	}

	.payment-method .method-description {
		display: block;
		font-size: 13px;
		color: var(--checkout-text-muted);
	}

	.payment-method .card-icons {
		display: flex;
		gap: 8px;
	}

	.payment-method .card-icons img {
		height: 24px;
	}

	.security-notice {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 16px;
		background: #f0f9ff;
		border-radius: 5px;
		color: var(--checkout-primary);
		font-size: 13px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Checkout Actions - 2026 Mobile-First with safe area insets */
	.checkout-steps-actions {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	@media (min-width: 640px) {
		.checkout-steps-actions {
			flex-direction: row;
			justify-content: space-between;
			gap: 16px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Buttons - 2026 Mobile-First with touch targets */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px 24px;
		font-size: 16px; /* Prevents iOS zoom */
		font-weight: 700;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		transition: var(--checkout-transition);
		text-decoration: none;
		min-height: 48px; /* Enhanced touch target for primary actions */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
		width: 100%;
	}

	@media (min-width: 640px) {
		.btn {
			width: auto;
		}
	}

	.btn-orange {
		background: var(--checkout-orange);
		color: #fff;
	}

	.btn-orange:hover:not(:disabled) {
		background: var(--checkout-orange-hover);
	}

	.btn-orange:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-default {
		background: var(--checkout-bg);
		color: var(--checkout-text);
		border: 1px solid var(--checkout-border);
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	.btn-xs {
		padding: 10px 16px;
		font-size: 13px;
		min-height: 44px;
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-cart-section {
		position: sticky;
		top: 24px;
	}

	.checkout-cart .card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-bottom: 1px solid var(--checkout-border);
		background: #fafafa;
	}

	.checkout-cart .card-header h4 {
		font-size: 16px;
		font-weight: 700;
		color: var(--checkout-text-dark);
		margin: 0;
	}

	.checkout-cart-contents {
		padding: 20px;
	}

	.checkout-cart-products {
		margin-bottom: 20px;
	}

	.checkout-cart-products .product {
		display: flex;
		gap: 12px;
		padding: 12px 0;
		border-bottom: 1px solid #eee;
	}

	.checkout-cart-products .product:last-child {
		border-bottom: none;
	}

	.product-image {
		width: 50px;
		height: 50px;
		border-radius: 5px;
		overflow: hidden;
		background-color: var(--checkout-bg);
		background-size: cover;
		background-position: center;
		flex-shrink: 0;
	}

	.product-placeholder {
		width: 100%;
		height: 100%;
		background: linear-gradient(135deg, #e0e0e0 0%, #f5f5f5 100%);
	}

	.product-details {
		flex: 1;
		min-width: 0;
	}

	.product-name {
		font-weight: 700;
		font-size: 14px;
		color: var(--checkout-text-dark);
		margin-bottom: 4px;
	}

	.product-interval {
		font-size: 12px;
		font-weight: 600;
		color: var(--checkout-primary);
	}

	.product-total {
		font-weight: 700;
		font-size: 14px;
		color: var(--checkout-text-dark);
	}

	/* Cart Table */
	.cart-table {
		width: 100%;
		margin-bottom: 16px;
	}

	.cart-table th,
	.cart-table td {
		padding: 10px 0;
		font-size: 14px;
	}

	.cart-table th {
		text-align: left;
		font-weight: 700;
		color: var(--checkout-text-dark);
	}

	.cart-table td {
		text-align: right;
		color: var(--checkout-text-dark);
		font-weight: 600;
	}

	.cart-table .discount {
		color: var(--checkout-success);
		font-weight: 700;
	}

	.remove-coupon {
		background: none;
		border: none;
		color: var(--checkout-error);
		cursor: pointer;
		padding: 2px;
		margin-left: 8px;
	}

	/* Order Total - Dark and Bold */
	.checkout-order-total {
		display: flex;
		justify-content: space-between;
		padding: 16px 0;
		border-top: 2px solid var(--checkout-text-dark);
		font-size: 18px;
		font-weight: 700;
		color: var(--checkout-text-dark);
	}

	/* Recurring Table */
	.recurring-table {
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid #eee;
	}

	.recurring-totals-header th {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--checkout-text-muted) !important;
	}

	.recurring-total td {
		text-align: left !important;
		font-size: 13px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COUPON SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.coupon-section {
		padding: 16px 20px;
		border-top: 1px solid var(--checkout-border);
	}

	.showcoupon {
		display: flex;
		align-items: center;
		gap: 8px;
		background: none;
		border: none;
		color: var(--checkout-primary);
		font-size: 13px;
		cursor: pointer;
		padding: 0;
	}

	.showcoupon:hover {
		text-decoration: underline;
	}

	.checkout_coupon {
		display: flex;
		gap: 8px;
		margin-top: 12px;
		flex-wrap: wrap;
	}

	/* Coupon Input - 2026 Mobile-First Responsive */
	.checkout_coupon input {
		flex: 1;
		min-width: 120px;
		padding: 12px 14px;
		border: 1px solid var(--checkout-border);
		border-radius: 5px;
		font-size: 16px; /* Prevents iOS zoom */
		text-transform: uppercase;
		min-height: 44px; /* 2026 touch target minimum */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.checkout_coupon input:focus {
		outline: none;
		border-color: var(--checkout-primary);
	}

	.coupon-error {
		width: 100%;
		color: var(--checkout-error);
		font-size: 12px;
		margin: 8px 0 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT TERMS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-terms {
		margin-top: 30px;
		text-align: center;
	}

	.checkout-terms p {
		font-size: 13px;
		color: var(--checkout-text-muted);
		margin: 0;
	}

	.checkout-terms a {
		color: var(--checkout-primary);
		text-decoration: none;
	}

	.checkout-terms a:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.checkout-content {
			grid-template-columns: 1fr;
		}

		.checkout-cart-section {
			position: static;
			order: -1;
		}
	}

	@media screen and (max-width: 768px) {
		.checkout-steps-nav ul {
			flex-direction: column;
		}

		.checkout-steps-nav li {
			border-right: none;
			border-bottom: 1px solid var(--checkout-border);
		}

		.checkout-steps-nav li:last-child {
			border-bottom: none;
		}

		.billing-fields {
			grid-template-columns: 1fr;
		}

		.form-row-first,
		.form-row-last {
			grid-column: auto;
		}

		.checkout-steps-actions {
			flex-direction: column;
		}

		.card-body {
			padding: 20px;
		}

		.card-footer {
			padding: 16px 20px;
		}

		.page-title {
			font-size: 28px;
		}
	}

	@media screen and (max-width: 480px) {
		.woocommerce-checkout {
			padding: 20px 0;
		}

		.checkout-cart-contents {
			padding: 16px;
		}

		.coupon-section {
			padding: 12px 16px;
		}

		.checkout-order-total {
			font-size: 16px;
		}
	}

	@media screen and (max-width: 360px) {
		.container {
			padding: 0 12px;
		}

		.card-body {
			padding: 16px;
		}

		.card-footer {
			padding: 12px 16px;
		}

		.page-title {
			font-size: 24px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT ERROR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-error {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 5px;
		color: #dc2626;
		margin-bottom: 20px;
	}

	.checkout-error span {
		flex: 1;
		font-size: 14px;
		font-weight: 500;
	}

	.checkout-error button {
		background: none;
		border: none;
		color: #dc2626;
		cursor: pointer;
		padding: 4px;
		opacity: 0.7;
	}

	.checkout-error button:hover {
		opacity: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	.btn:focus-visible {
		outline: 2px solid var(--checkout-primary);
		outline-offset: 2px;
	}

	.input-text:focus-visible {
		outline: none; /* Already has box-shadow focus */
	}

	.payment-method:focus-within {
		border-color: var(--checkout-primary);
	}

	.showcoupon:focus-visible {
		outline: 2px solid var(--checkout-primary);
		outline-offset: 2px;
	}
</style>
