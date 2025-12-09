<script lang="ts">
	/**
	 * NonMemberCheckout Component - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Displays the checkout page for non-authenticated users
	 * Shows Sign In/Register forms alongside the cart summary
	 * Identical to the Simpler Trading non-member checkout experience
	 *
	 * @version 1.0.0 (Svelte 5 / December 2025)
	 */

	import { goto } from '$app/navigation';
	import { cartStore, cartTotal, cartItemCount } from '$lib/stores/cart';
	import { login, register } from '$lib/api/auth';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconShieldCheck from '@tabler/icons-svelte/icons/shield-check';
	import IconTicket from '@tabler/icons-svelte/icons/ticket';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let activeForm = $state<'login' | 'register'>('login');
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

	let hasSubscriptions = $derived($cartStore.items.some((i) => i.interval));

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
				return '/ 3 months';
			case 'yearly':
				return '/ year';
			default:
				return '';
		}
	}

	function toggleForm() {
		activeForm = activeForm === 'login' ? 'register' : 'login';
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
			await login(loginEmail, loginPassword, rememberMe);
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
				password: registerPassword
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
     TEMPLATE - Non-Member Checkout (Simpler Trading Style)
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="nonmember-checkout">
	<div class="container">
		<!-- Page Header -->
		<header class="checkout-page-header">
			<a href="/" class="back-link">
				<IconArrowLeft size={18} />
				<span>Continue Shopping</span>
			</a>
			<h1 class="page-title">Checkout</h1>
		</header>

		<!-- Step Navigation -->
		<nav class="checkout-nav">
			<ul>
				<li class="active">
					<span class="step-label">Sign In / Register</span>
				</li>
				<li class="disabled">
					<span class="step-label">Billing Info</span>
				</li>
				<li class="disabled">
					<span class="step-label">Payment</span>
				</li>
			</ul>
		</nav>

		<!-- Checkout Content -->
		<div class="checkout-content">
			<!-- Forms Section -->
			<div class="checkout-forms-section">
				<!-- Login Form -->
				{#if activeForm === 'login'}
					<div class="checkout-account-login">
						<div class="card">
							<div class="card-body">
								<div class="login-split">
									<div class="login-left">
										<h3>Sign in</h3>
										<form onsubmit={handleLogin}>
											{#if errorMessage}
												<div class="form-message error" role="alert">
													{errorMessage}
												</div>
											{/if}

											<div class="form-group">
												<label for="login_email">Email Address</label>
												<input
													type="email"
													id="login_email"
													class="form-control"
													bind:value={loginEmail}
													disabled={isSubmitting}
													required
												/>
											</div>

											<div class="form-group">
												<label for="login_password">Password</label>
												<input
													type="password"
													id="login_password"
													class="form-control"
													bind:value={loginPassword}
													disabled={isSubmitting}
													required
												/>
											</div>

											<div class="form-group checkbox-group">
												<label class="checkbox-label">
													<input
														type="checkbox"
														bind:checked={rememberMe}
														disabled={isSubmitting}
													/>
													<span>Remember me</span>
												</label>
											</div>

											<div class="form-group">
												<button
													type="submit"
													class="btn btn-lg btn-block btn-orange"
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

									<div class="login-right">
										<h3>New to Revolution Trading?</h3>
										<p>You will need an account to purchase our classes and other products.</p>
										<button
											type="button"
											class="btn btn-lg btn-block btn-orange"
											onclick={toggleForm}
										>
											Register for an Account
										</button>
									</div>
								</div>
							</div>
						</div>
					</div>
				{:else}
					<!-- Register Form -->
					<div class="checkout-account-register">
						<form onsubmit={handleRegister}>
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
											<label for="register_first_name"
												>First Name <abbr class="required">*</abbr></label
											>
											<input
												type="text"
												id="register_first_name"
												class="form-control"
												bind:value={registerFirstName}
												disabled={isSubmitting}
												required
											/>
										</div>

										<div class="form-group form-group-half">
											<label for="register_last_name"
												>Last Name <abbr class="required">*</abbr></label
											>
											<input
												type="text"
												id="register_last_name"
												class="form-control"
												bind:value={registerLastName}
												disabled={isSubmitting}
												required
											/>
										</div>
									</div>

									<div class="form-group">
										<label for="register_email">Email Address <abbr class="required">*</abbr></label
										>
										<input
											type="email"
											id="register_email"
											class="form-control"
											bind:value={registerEmail}
											disabled={isSubmitting}
											required
										/>
									</div>

									<div class="form-group">
										<label for="register_password">Password <abbr class="required">*</abbr></label>
										<input
											type="password"
											id="register_password"
											class="form-control"
											bind:value={registerPassword}
											disabled={isSubmitting}
											required
										/>
									</div>
								</div>

								<div class="card-footer">
									<div class="register-actions">
										<button
											type="button"
											class="btn btn-lg btn-default"
											onclick={toggleForm}
											disabled={isSubmitting}
										>
											<IconArrowLeft size={16} />
											Back to Sign In
										</button>
										<button
											type="submit"
											class="btn btn-lg btn-orange"
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

						<!-- Products -->
						<div class="checkout-cart-contents">
							<div class="checkout-cart-products">
								{#each $cartStore.items as item (item.id + (item.interval || ''))}
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

							<!-- Tax row -->
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

							<!-- Total -->
							<div class="checkout-order-total">
								<div class="total-label">Total</div>
								<div class="checkout-order-total-price">
									{formatPrice($cartTotal)}
								</div>
							</div>

							<!-- Recurring totals for subscriptions -->
							{#if hasSubscriptions}
								<div class="cart-table-wrapper">
									<table class="cart-table">
										<tbody>
											<tr class="recurring-totals">
												<th colspan="2">Recurring Totals</th>
											</tr>
											{#each $cartStore.items.filter((i) => i.interval) as item}
												<tr class="cart-subtotal recurring-total">
													<td colspan="2">
														{formatPrice(item.price)}
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
							<div class="coupon-toggle">
								<IconTicket size={16} />
								<button
									type="button"
									class="showcoupon"
									onclick={() => (couponFormVisible = !couponFormVisible)}
								>
									Have a coupon? Click here to enter your code
								</button>
							</div>

							{#if couponFormVisible}
								<form class="checkout-coupon" onsubmit={(e) => e.preventDefault()}>
									<p>If you have a coupon code, please apply it below.</p>
									<div class="coupon-input-row">
										<input
											type="text"
											placeholder="Coupon code"
											bind:value={couponCode}
											class="input-text"
										/>
										<button type="submit" class="btn btn-default"> Apply </button>
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

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Non-Member Checkout (Simpler Trading Style)
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--checkout-bg: #f4f4f4;
		--checkout-card-bg: #fff;
		--checkout-border: #dbdbdb;
		--checkout-text: #333;
		--checkout-text-dark: #1a1a1a;
		--checkout-text-muted: #666;
		--checkout-primary: #0984ae;
		--checkout-orange: #f99e31;
		--checkout-orange-hover: #f88b09;
		--checkout-success: #10b981;
		--checkout-error: #ef4444;
		--checkout-transition: all 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.nonmember-checkout {
		min-height: 100vh;
		background: var(--checkout-bg);
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
		color: #076787;
	}

	.page-title {
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: var(--checkout-text-dark);
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STEP NAVIGATION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-nav {
		margin-bottom: 30px;
	}

	.checkout-nav ul {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
		background: var(--checkout-card-bg);
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
	}

	.checkout-nav li {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 16px 20px;
		border-right: 1px solid var(--checkout-border);
		color: var(--checkout-text-dark);
		font-weight: 700;
		transition: var(--checkout-transition);
	}

	.checkout-nav li:last-child {
		border-right: none;
	}

	.checkout-nav li.active {
		background: var(--checkout-primary);
		color: #fff;
	}

	.checkout-nav li.disabled {
		opacity: 0.5;
		color: var(--checkout-text-muted);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CHECKOUT CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-content {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: 30px;
		align-items: start;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARDS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.card {
		background: var(--checkout-card-bg);
		border-radius: 5px;
		box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
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
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-bottom: 1px solid var(--checkout-border);
	}

	.card-header h4 {
		font-size: 16px;
		font-weight: 700;
		color: var(--checkout-text-dark);
		margin: 0;
	}

	.card-footer {
		padding: 20px 30px;
		background: #f9f9f9;
		border-top: 1px solid var(--checkout-border);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOGIN SPLIT LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.login-split {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 40px;
	}

	.login-left {
		padding-right: 40px;
		border-right: 1px solid var(--checkout-border);
	}

	.login-right {
		text-align: center;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.login-right h3 {
		margin-bottom: 16px;
	}

	.login-right p {
		color: var(--checkout-text-muted);
		margin-bottom: 24px;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORMS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 700;
		color: var(--checkout-text-dark);
		margin-bottom: 6px;
	}

	.form-group label .required {
		color: var(--checkout-error);
		text-decoration: none;
	}

	.form-control {
		width: 100%;
		padding: 12px 16px;
		border: 1px solid var(--checkout-border);
		border-radius: 5px;
		font-size: 15px;
		font-weight: 500;
		color: var(--checkout-text-dark);
		transition: var(--checkout-transition);
	}

	.form-control:focus {
		outline: none;
		border-color: var(--checkout-primary);
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.1);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.checkbox-group {
		display: flex;
		align-items: center;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		font-weight: 500 !important;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
	}

	.form-message {
		padding: 12px 16px;
		border-radius: 5px;
		margin-bottom: 16px;
		font-weight: 600;
	}

	.form-message.error {
		background: #fef2f2;
		color: var(--checkout-error);
		border: 1px solid #fecaca;
	}

	.form-message.success {
		background: #ecfdf5;
		color: var(--checkout-success);
		border: 1px solid #a7f3d0;
	}

	.forgot-password-link {
		color: var(--checkout-primary);
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
	}

	.forgot-password-link:hover {
		text-decoration: underline;
	}

	.register-actions {
		display: flex;
		justify-content: space-between;
		gap: 16px;
	}

	.text-center {
		text-align: center;
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
		transition: var(--checkout-transition);
		text-decoration: none;
	}

	.btn-lg {
		padding: 14px 28px;
		font-size: 16px;
	}

	.btn-block {
		width: 100%;
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
		color: var(--checkout-text-dark);
		border: 1px solid var(--checkout-border);
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CART SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.checkout-cart-section {
		position: sticky;
		top: 24px;
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
		align-items: center;
	}

	.checkout-cart-products .product:last-child {
		border-bottom: none;
	}

	.product-image {
		width: 50px;
		height: 50px;
		border-radius: 5px;
		background: var(--checkout-bg);
		background-size: cover;
		background-position: center;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--checkout-text-muted);
		flex-shrink: 0;
	}

	.product-details {
		flex: 1;
		min-width: 0;
	}

	.product-name {
		font-weight: 700;
		font-size: 14px;
		color: var(--checkout-text-dark);
	}

	.product-interval {
		font-weight: 500;
		font-size: 12px;
		color: var(--checkout-primary);
		margin-left: 4px;
	}

	.product-total {
		font-weight: 700;
		font-size: 14px;
		color: var(--checkout-text-dark);
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
		color: var(--checkout-text-dark);
	}

	.cart-table td {
		text-align: right;
		color: var(--checkout-text-dark);
	}

	.recurring-totals th {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--checkout-text-muted) !important;
		padding-top: 16px;
	}

	.recurring-total td {
		text-align: left !important;
		font-size: 13px;
	}

	/* Order Total */
	.checkout-order-total {
		display: flex;
		justify-content: space-between;
		padding: 16px 0;
		border-top: 2px solid var(--checkout-text-dark);
		margin-top: 16px;
	}

	.total-label {
		font-size: 18px;
		font-weight: 700;
		color: var(--checkout-text-dark);
	}

	.checkout-order-total-price {
		font-size: 18px;
		font-weight: 700;
		color: var(--checkout-text-dark);
	}

	/* Coupon Section */
	.coupon-section {
		padding: 16px 20px;
		border-top: 1px solid var(--checkout-border);
	}

	.coupon-toggle {
		display: flex;
		align-items: center;
		gap: 8px;
		color: var(--checkout-primary);
	}

	.showcoupon {
		background: none;
		border: none;
		color: var(--checkout-primary);
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		padding: 0;
	}

	.showcoupon:hover {
		text-decoration: underline;
	}

	.checkout-coupon {
		margin-top: 16px;
	}

	.checkout-coupon p {
		font-size: 13px;
		color: var(--checkout-text-muted);
		margin-bottom: 12px;
		font-weight: 500;
	}

	.coupon-input-row {
		display: flex;
		gap: 8px;
	}

	.coupon-input-row .input-text {
		flex: 1;
		padding: 10px 12px;
		border: 1px solid var(--checkout-border);
		border-radius: 5px;
		font-size: 13px;
		text-transform: uppercase;
		font-weight: 600;
	}

	.coupon-input-row .input-text:focus {
		outline: none;
		border-color: var(--checkout-primary);
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

		.login-split {
			grid-template-columns: 1fr;
			gap: 30px;
		}

		.login-left {
			padding-right: 0;
			border-right: none;
			border-bottom: 1px solid var(--checkout-border);
			padding-bottom: 30px;
		}
	}

	@media screen and (max-width: 768px) {
		.checkout-nav ul {
			flex-direction: column;
		}

		.checkout-nav li {
			border-right: none;
			border-bottom: 1px solid var(--checkout-border);
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
		.nonmember-checkout {
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
