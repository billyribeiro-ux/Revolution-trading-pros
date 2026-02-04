<script lang="ts">
	/**
	 * NonMemberCheckout Component - Revolution Trading Custom Cart System
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Custom cart system for non-member checkout with pixel-perfect styling
	 * (no WooCommerce dependency)
	 *
	 * Reference: Custom RTP Cart System
	 *
	 * @version 3.0.0 (Custom RTP Cart System / January 2026)
	 */

	import { goto } from '$app/navigation';
	import { cartStore, getCartTotal } from '$lib/stores/cart.svelte';
	import { login, register } from '$lib/api/auth';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconTicket from '@tabler/icons-svelte-runes/icons/ticket';
	import IconShoppingCart from '@tabler/icons-svelte-runes/icons/shopping-cart';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let showRegisterForm = $state(false);
	let isSubmitting = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');

	// Login form
	let loginEmail = $state('');
	let loginPassword = $state('');
	let rememberMe = $state(false);

	// Register form
	let registerFirstName = $state('');
	let registerLastName = $state('');
	let registerEmail = $state('');
	let registerPassword = $state('');

	// Coupon state
	let couponCode = $state('');
	let couponFormVisible = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	let hasSubscriptions = $derived(cartStore.items.some((i) => i.interval));

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function getIntervalLabel(interval?: string): string {
		switch (interval) {
			case 'monthly':
				return '/ month';
			case 'quarterly':
				return '/ 3 months';
			case 'yearly':
				return '/ year';
			default:
				return '';
		}
	}

	function toggleForms() {
		showRegisterForm = !showRegisterForm;
		errorMessage = '';
		successMessage = '';
	}

	async function handleLogin(e: Event) {
		e.preventDefault();
		if (!loginEmail.trim() || !loginPassword.trim()) {
			errorMessage = 'Please enter your email and password';
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			await login({
				email: loginEmail,
				password: loginPassword,
				remember: rememberMe
			});
			goto('/checkout');
		} catch (error) {
			errorMessage = error instanceof Error ? error.message : 'Login failed. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}

	async function handleRegister(e: Event) {
		e.preventDefault();
		if (
			!registerFirstName.trim() ||
			!registerLastName.trim() ||
			!registerEmail.trim() ||
			!registerPassword.trim()
		) {
			errorMessage = 'Please fill in all required fields';
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			await register({
				name: `${registerFirstName.trim()} ${registerLastName.trim()}`,
				email: registerEmail.trim(),
				password: registerPassword,
				password_confirmation: registerPassword
			});
			successMessage = 'Account created successfully! Redirecting to checkout...';
			setTimeout(() => goto('/checkout'), 1500);
		} catch (error) {
			errorMessage =
				error instanceof Error ? error.message : 'Registration failed. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Simpler Trading NonMember Checkout (EXACT STRUCTURE)
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="rtp-checkout rtp-page checkout-nonmember">
	<div class="container">
		<!-- Page Header -->
		<header class="checkout-page-header">
			<a href="/" class="back-link">
				<IconArrowLeft size={18} />
				<span>Continue Shopping</span>
			</a>
			<h1 class="page-title">Checkout</h1>
		</header>

		<!-- Step Navigation - Simpler Trading Style -->
		<nav class="checkout-nav hidden-xs">
			<ul>
				<li id="nav-checkout-account" class="active">
					<span>Sign In / Register</span>
				</li>
				<li id="nav-checkout-billing" class="disabled">
					<span>Billing Info</span>
				</li>
				<li id="nav-checkout-payment" class="disabled">
					<span>Payment</span>
				</li>
			</ul>
		</nav>

		<!-- Checkout Content -->
		<div class="checkout-steps">
			<!-- Account Step -->
			<div id="checkout-account" class="checkout-step active">
				<div class="checkout-step-content">
					<!-- Login/Register Forms - LEFT SIDE -->
					{#if !showRegisterForm}
						<!-- Login Form - Simpler Trading Style -->
						<div class="checkout-account-login is-visible">
							<div class="card">
								<div class="card-body">
									<div class="checkout-account-login-left">
										<h3>Sign in</h3>
										<form id="form-login" name="login" method="post" onsubmit={handleLogin}>
											{#if errorMessage}
												<div class="form-message error" role="alert">
													{errorMessage}
												</div>
											{/if}

											<div class="form-group required">
												<label for="login_username">Email Address</label>
												<input
													type="text"
													class="form-control"
													name="username"
													id="login_username"
													bind:value={loginEmail}
													disabled={isSubmitting}
												/>
											</div>

											<div class="form-group required">
												<label for="login_password">Password</label>
												<input
													type="password"
													class="form-control"
													name="password"
													id="login_password"
													bind:value={loginPassword}
													disabled={isSubmitting}
												/>
											</div>

											<div class="form-group form-check">
												<label class="form-check-label">
													<input
														type="checkbox"
														class="form-check-input"
														bind:checked={rememberMe}
														disabled={isSubmitting}
													/>
													<span>Remember me</span>
												</label>
											</div>

											<div class="form-group">
												<button
													type="submit"
													class="btn btn-lg btn-block btn-orange font-weight-bold"
													disabled={isSubmitting}
												>
													{isSubmitting ? 'Signing In...' : 'Sign In'}
												</button>
											</div>

											<div class="text-center">
												<a href="/forgot-password" class="forgot-password-link"
													>Forgot your password?</a
												>
											</div>
										</form>
									</div>

									<div class="checkout-account-login-right text-center">
										<h3>New to Revolution Trading?</h3>
										<p>You will need an account to purchase our classes and other products.</p>
										<button
											type="button"
											class="toggle-account-forms btn btn-lg btn-block btn-orange font-weight-bold"
											onclick={toggleForms}
										>
											Register for an Account
										</button>
									</div>
								</div>
							</div>
						</div>
					{:else}
						<!-- Register Form - Simpler Trading Style -->
						<div class="checkout-account-register is-visible">
							<form id="form-register" name="register" method="post" onsubmit={handleRegister}>
								<div class="card">
									<div class="card-body">
										<h3>Register for an Account</h3>

										{#if errorMessage}
											<div class="form-message error" role="alert">
												{errorMessage}
											</div>
										{/if}

										{#if successMessage}
											<div class="form-message success" role="alert">
												{successMessage}
											</div>
										{/if}

										<div class="form-row">
											<div class="form-group form-group-half">
												<label for="reg_billing_first_name"
													>First Name <abbr class="required" title="required">*</abbr></label
												>
												<input
													type="text"
													class="form-control"
													name="billing_first_name"
													id="reg_billing_first_name"
													bind:value={registerFirstName}
													disabled={isSubmitting}
												/>
											</div>

											<div class="form-group form-group-half">
												<label for="reg_billing_last_name"
													>Last Name <abbr class="required" title="required">*</abbr></label
												>
												<input
													type="text"
													class="form-control"
													name="billing_last_name"
													id="reg_billing_last_name"
													bind:value={registerLastName}
													disabled={isSubmitting}
												/>
											</div>
										</div>

										<div class="form-group">
											<label for="reg_email"
												>Email Address <abbr class="required" title="required">*</abbr></label
											>
											<input
												type="email"
												class="form-control"
												name="email"
												id="reg_email"
												bind:value={registerEmail}
												disabled={isSubmitting}
											/>
										</div>

										<div class="form-group">
											<label for="reg_password"
												>Password <abbr class="required" title="required">*</abbr></label
											>
											<input
												type="password"
												class="form-control"
												name="password"
												id="reg_password"
												bind:value={registerPassword}
												disabled={isSubmitting}
											/>
										</div>
									</div>

									<div class="card-footer">
										<div class="register-actions">
											<button
												type="button"
												class="toggle-account-forms btn btn-lg btn-default"
												onclick={toggleForms}
												disabled={isSubmitting}
											>
												<IconArrowLeft size={16} />
												Back to Sign In
											</button>
											<button
												type="submit"
												class="btn btn-lg btn-orange font-weight-bold"
												disabled={isSubmitting}
											>
												{isSubmitting ? 'Creating Account...' : 'Register'}
											</button>
										</div>
									</div>
								</div>
							</form>
						</div>
					{/if}

					<!-- Cart Sidebar - RIGHT SIDE - Revolution Trading Style -->
					<div class="checkout-cart">
						<div id="order_review" class="rtp-checkout-review-order">
							<div class="rtp-checkout-review-order-table">
								<div class="card">
									<div class="card-header">
										<h4>Shopping Cart</h4>
										<a href="/cart" class="btn btn-xs btn-default">Edit</a>
									</div>

									<div class="checkout-cart-contents">
										<!-- Products -->
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
															<IconShoppingCart size={20} />
														{/if}
													</div>
													<div class="product-name">
														{item.name}
														{#if item.interval}
															&nbsp;<span class="product-interval">({item.interval})</span>
														{/if}
													</div>
													<div class="product-total">
														<span class="rtp-price-amount amount">
															<bdi
																><span class="rtp-price-currency">$</span>{item.price.toFixed(
																	2
																)}</bdi
															>
														</span>
													</div>
												</div>
											{/each}
										</div>

										<!-- Tax Row -->
										<div class="cart-table-wrapper">
											<table class="cart-table">
												<tbody>
													<tr class="tax-total">
														<th>Tax</th>
														<td>Calculated at checkout</td>
													</tr>
												</tbody>
											</table>
										</div>

										<!-- Order Total -->
										<div class="checkout-order-total">
											<div>Total</div>
											<div class="checkout-order-total-price">
												<span class="rtp-price-amount amount">
													<bdi><span class="rtp-price-currency">$</span>{getCartTotal().toFixed(2)}</bdi
													>
												</span>
											</div>
										</div>

										<!-- Recurring Totals for Subscriptions -->
										{#if hasSubscriptions}
											<div class="cart-table-wrapper">
												<table class="rtp-cart-table">
													<tbody>
														<tr class="rtp-recurring-totals">
															<th colspan="2">Recurring Totals</th>
														</tr>
														{#each cartStore.items.filter((i) => i.interval) as item}
															<tr class="rtp-cart-subtotal rtp-recurring-total">
																<td colspan="2">
																	<span class="rtp-price-amount amount">
																		<bdi
																			><span class="rtp-price-currency">$</span>{item.price.toFixed(
																				2
																			)}</bdi
																		>
																	</span>
																	{getIntervalLabel(item.interval)}
																</td>
															</tr>
														{/each}
													</tbody>
												</table>
											</div>
										{/if}
									</div>

									<!-- Coupon Section -->
									<div class="coupon-section">
										<div class="rtp-form-coupon-toggle">
											<div class="rtp-info">
												<IconTicket size={16} />
												<button
													type="button"
													class="showcoupon"
													onclick={() => (couponFormVisible = !couponFormVisible)}
												>
													Have a coupon? Click here to enter your code
												</button>
											</div>
										</div>

										{#if couponFormVisible}
											<form
												class="rtp-checkout-coupon rtp-form-coupon"
												onsubmit={(e: SubmitEvent) => e.preventDefault()}
											>
												<p>If you have a coupon code, please apply it below.</p>
												<div class="coupon-input-row">
													<input
														type="text"
														name="coupon_code"
														class="input-text"
														placeholder="Coupon code"
														bind:value={couponCode}
													/>
													<button type="submit" class="btn btn-default"> Apply coupon </button>
												</div>
											</form>
										{/if}
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading NonMember Checkout (EXACT MATCH)
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES - Simpler Trading Colors
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
		--st-success: #10b981;
		--st-error: #dc3545;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE LAYOUT - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-nonmember {
		/* ICT11+ Fix: Removed min-height: 100vh - let parent flex container handle height */
		flex: 1;
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

	.checkout-page-header {
		margin-bottom: 24px;
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
		margin: 0;
		text-transform: uppercase;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT NAV - Simpler Trading Step Navigation
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-nav {
		margin-bottom: 30px;
	}

	.checkout-nav ul {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
		background: var(--st-card-bg);
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.checkout-nav li {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 16px 20px;
		border-right: 1px solid var(--st-border);
		font-weight: 700;
		font-size: 14px;
		color: var(--st-text-dark);
		transition: all 0.15s ease;
	}

	.checkout-nav li:last-child {
		border-right: none;
	}

	.checkout-nav li.active {
		background: var(--st-primary);
		color: #ffffff;
	}

	.checkout-nav li.disabled {
		opacity: 0.5;
		color: var(--st-text-muted);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT STEPS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-steps {
		position: relative;
	}

	.checkout-step {
		display: none;
	}

	.checkout-step.active {
		display: block;
	}

	.checkout-step-content {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: 30px;
		align-items: start;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARDS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.card {
		background: var(--st-card-bg);
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
		color: var(--st-text-dark);
		margin: 0 0 24px;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-bottom: 1px solid var(--st-border);
		background: #fafafa;
	}

	.card-header h4 {
		font-size: 16px;
		font-weight: 700;
		color: var(--st-text-dark);
		margin: 0;
	}

	.card-footer {
		padding: 20px 30px;
		background: #f9f9f9;
		border-top: 1px solid var(--st-border);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOGIN FORM - checkout-account-login (Simpler Trading Style)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-account-login .card-body {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 40px;
	}

	.checkout-account-login-left {
		padding-right: 40px;
		border-right: 1px solid var(--st-border);
	}

	.checkout-account-login-right {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
	}

	.checkout-account-login-right h3 {
		margin-bottom: 16px;
	}

	.checkout-account-login-right p {
		color: var(--st-text-muted);
		margin-bottom: 24px;
		font-weight: 500;
		line-height: 1.6;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REGISTER FORM - checkout-account-register
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-account-register .card-body h3 {
		margin-bottom: 24px;
	}

	.register-actions {
		display: flex;
		justify-content: space-between;
		gap: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORMS - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 700;
		color: var(--st-text-dark);
		margin-bottom: 8px;
	}

	.form-group label .required {
		color: var(--st-error);
		text-decoration: none;
	}

	/* Form Control - 2026 Mobile-First Responsive */
	.form-control {
		width: 100%;
		padding: 12px 16px;
		border: 1px solid var(--st-border);
		border-radius: 5px;
		font-size: 16px; /* Prevents iOS zoom on focus */
		font-weight: 500;
		color: var(--st-text-dark);
		background: #ffffff;
		transition: all 0.15s ease;
		min-height: 44px; /* 2026 touch target minimum */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.form-control:focus {
		outline: none;
		border-color: var(--st-primary);
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.15);
	}

	/* Form Row - 2026 Mobile-First (stack on mobile) */
	.form-row {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	@media (min-width: 640px) {
		.form-row {
			display: grid;
			grid-template-columns: 1fr 1fr;
		}
	}

	.form-check {
		display: flex;
		align-items: center;
	}

	.form-check-label {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		font-weight: 500 !important;
	}

	/* Checkbox - 2026 Mobile-First with touch target */
	.form-check-input {
		width: 24px;
		height: 24px;
		min-width: 24px;
		cursor: pointer;
		touch-action: manipulation;
	}

	.form-message {
		padding: 12px 16px;
		border-radius: 5px;
		margin-bottom: 16px;
		font-weight: 600;
		font-size: 14px;
	}

	.form-message.error {
		background: #fef2f2;
		color: var(--st-error);
		border: 1px solid #fecaca;
	}

	.form-message.success {
		background: #ecfdf5;
		color: var(--st-success);
		border: 1px solid #a7f3d0;
	}

	.forgot-password-link {
		color: var(--st-primary);
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
	}

	.forgot-password-link:hover {
		text-decoration: underline;
	}

	.text-center {
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTONS - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Button - 2026 Mobile-First with touch targets */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 16px; /* Prevents iOS zoom */
		font-weight: 700;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
		text-decoration: none;
		min-height: 44px; /* 2026 touch target minimum */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.btn-lg {
		padding: 14px 28px;
		font-size: 16px;
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

	.btn-default {
		background: var(--st-bg);
		color: var(--st-text-dark);
		border: 1px solid var(--st-border);
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.font-weight-bold {
		font-weight: 700;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART SIDEBAR - checkout-cart (Simpler Trading Style)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-cart {
		position: sticky;
		top: 24px;
	}

	.checkout-cart-contents {
		padding: 20px;
	}

	.checkout-cart-products {
		margin-bottom: 16px;
	}

	.checkout-cart-products .product {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 0;
		border-bottom: 1px solid #eeeeee;
	}

	.checkout-cart-products .product:last-child {
		border-bottom: none;
	}

	.product-image {
		width: 50px;
		height: 50px;
		border-radius: 5px;
		background: var(--st-bg);
		background-size: cover;
		background-position: center;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--st-text-muted);
		flex-shrink: 0;
	}

	.product-name {
		flex: 1;
		font-weight: 700;
		font-size: 14px;
		color: var(--st-text-dark);
	}

	.product-interval {
		font-weight: 600;
		font-size: 12px;
		color: var(--st-primary);
	}

	.product-total {
		font-weight: 700;
		font-size: 14px;
		color: var(--st-text-dark);
	}

	/* Cart Table */
	.cart-table-wrapper {
		margin-top: 16px;
	}

	.cart-table {
		width: 100%;
	}

	.cart-table th,
	.cart-table td {
		padding: 10px 0;
		font-size: 14px;
		font-weight: 600;
	}

	.cart-table th {
		text-align: left;
		color: var(--st-text-dark);
	}

	.cart-table td {
		text-align: right;
		color: var(--st-text-dark);
	}

	.cart-table .tax-total td {
		color: var(--st-text-muted);
		font-weight: 500;
	}

	.rtp-recurring-totals th {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--st-text-muted) !important;
		padding-top: 16px;
	}

	.rtp-recurring-total td {
		text-align: left !important;
		font-size: 13px;
	}

	/* Order Total - Simpler Trading Style */
	.checkout-order-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 0;
		border-top: 2px solid var(--st-text-dark);
		margin-top: 16px;
		font-size: 18px;
		font-weight: 700;
		color: var(--st-text-dark);
	}

	.checkout-order-total-price {
		font-size: 20px;
		font-weight: 700;
	}

	/* RTP Price Styling */
	.rtp-price-amount {
		font-weight: 700;
	}

	.rtp-price-currency {
		font-weight: 700;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COUPON SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.coupon-section {
		padding: 16px 20px;
		border-top: 1px solid var(--st-border);
	}

	.rtp-info {
		display: flex;
		align-items: center;
		gap: 8px;
		color: var(--st-primary);
	}

	.showcoupon {
		background: none;
		border: none;
		color: var(--st-primary);
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		padding: 0;
		text-align: left;
	}

	.showcoupon:hover {
		text-decoration: underline;
	}

	.rtp-checkout-coupon {
		margin-top: 16px;
	}

	.rtp-checkout-coupon p {
		font-size: 13px;
		color: var(--st-text-muted);
		margin-bottom: 12px;
		font-weight: 500;
	}

	.coupon-input-row {
		display: flex;
		gap: 8px;
	}

	/* Coupon Input - 2026 Mobile-First Responsive */
	.coupon-input-row .input-text {
		flex: 1;
		padding: 12px 14px;
		border: 1px solid var(--st-border);
		border-radius: 5px;
		font-size: 16px; /* Prevents iOS zoom */
		text-transform: uppercase;
		font-weight: 600;
		min-height: 44px; /* 2026 touch target minimum */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	.coupon-input-row .input-text:focus {
		outline: none;
		border-color: var(--st-primary);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.checkout-step-content {
			grid-template-columns: 1fr;
		}

		.checkout-cart {
			position: static;
			order: -1;
		}

		.checkout-account-login .card-body {
			grid-template-columns: 1fr;
			gap: 30px;
		}

		.checkout-account-login-left {
			padding-right: 0;
			border-right: none;
			border-bottom: 1px solid var(--st-border);
			padding-bottom: 30px;
		}
	}

	@media screen and (max-width: 768px) {
		.checkout-nav ul {
			flex-direction: column;
		}

		.checkout-nav li {
			border-right: none;
			border-bottom: 1px solid var(--st-border);
		}

		.checkout-nav li:last-child {
			border-bottom: none;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.register-actions {
			flex-direction: column;
		}

		.page-title {
			font-size: 28px;
		}
	}

	@media screen and (max-width: 480px) {
		.checkout-nonmember {
			padding: 20px 0;
		}

		.card-body {
			padding: 20px;
		}

		.card-footer {
			padding: 16px 20px;
		}
	}
</style>
