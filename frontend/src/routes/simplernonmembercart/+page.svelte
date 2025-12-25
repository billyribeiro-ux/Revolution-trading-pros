<script lang="ts">
	/**
	 * Non-Member Cart Page - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Cart page for non-members with login prompt
	 * @version 1.0.0 - Pixel Perfect
	 */

	// Cart state
	let cartItems = $state([
		{
			id: '88765154a2216f5a8806f524e7fb22a1',
			productId: '1318641',
			name: 'Day Trading Room (1 Month Trial)',
			price: 247.00,
			image: 'https://cdn.simplertrading.com/2020/11/01132413/Store-Card-MasteringTheTrade-Room.jpg',
			interval: 'monthly',
			trialPrice: 247.00
		}
	]);

	let cartTotal = $state(247.00);
	let email = $state('');
	let password = $state('');

	function removeItem(itemId: string) {
		if (confirm('Remove this item?')) {
			cartItems = cartItems.filter(item => item.id !== itemId);
		}
	}

	function proceedAsGuest() {
		window.location.href = '/checkout?guest=true';
	}

	function login(e: Event) {
		e.preventDefault();
		// Handle login
		console.log('Logging in...');
	}
</script>

<svelte:head>
	<title>Shopping Cart | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- WordPress EXACT Structure -->
<div class="page-template-default page woocommerce-cart woocommerce-page non-member">

	<!-- Breadcrumbs -->
	<nav id="breadcrumbs" class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home"><a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a></li>
				<li class="separator separator-home"> / </li>
				<li class="item-current"><strong class="breadcrumb-current">Cart</strong></li>
			</ul>
		</div>
	</nav>

	<!-- Page Content -->
	<div id="page" class="hfeed site grid-parent">
		<div id="content" class="site-content">

			<!-- Cart Page Header -->
			<header class="cart-page-header">
				<div class="container-fluid">
					<h1 class="cart-page-title">Shopping Cart</h1>
					<h6>({cartItems.length} Products in Cart)</h6>
				</div>
			</header>

			<!-- Cart Content -->
			<section class="wl-page-content">
				<div class="container-fluid">
					<div class="woocommerce">
						<div class="row">
							<!-- Products Column -->
							<div class="col-xs-12 col-md-7">
								<div class="products" data-products-view="list">
									{#each cartItems as item}
										<article class="product cart_item">
											<div class="card">
												<figure class="card-media">
													<div class="card-image" style="background-image: url({item.image})"></div>
												</figure>

												<section class="card-body">
													<h4 class="card-title">{item.name}</h4>
													<div class="card-description">
														<span class="membership-tagline">
															<span class="bold">Subscription:</span>
															<span class="price">${item.price.toFixed(2)}</span>
															<span class="interval"> / month with 1 month trial for </span>
															<span class="price">${item.trialPrice.toFixed(2)}</span>
														</span>
													</div>
												</section>

												<footer class="card-footer">
													<button
														type="button"
														class="btn btn-xs btn-default remove-btn"
														onclick={() => removeItem(item.id)}
													>
														<svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
														Remove
													</button>
													<div class="product-price">
														<span class="price">${item.price.toFixed(2)}</span>
													</div>
												</footer>
											</div>
										</article>
									{/each}
								</div>

								<a href="/products" class="continue-shopping">
									<svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/></svg>
									Continue Shopping
								</a>
							</div>

							<!-- Sidebar -->
							<div class="col-xs-12 col-md-5">
								<div class="cart-sidebar">
									<!-- Cart Totals -->
									<div class="cart-totals card">
										<div class="card-body">
											<div class="order-total">
												<span>Total</span>
												<span class="total-price">${cartTotal.toFixed(2)}</span>
											</div>

											<p class="tax-note">* Plus applicable taxes</p>

											<!-- Login/Register Section -->
											<div class="login-section">
												<h4>Already have an account?</h4>
												<form onsubmit={login}>
													<div class="form-group">
														<label for="email">Email Address</label>
														<input type="email" id="email" class="form-control" bind:value={email} placeholder="Email address" required />
													</div>
													<div class="form-group">
														<label for="password">Password</label>
														<input type="password" id="password" class="form-control" bind:value={password} placeholder="Password" required />
													</div>
													<button type="submit" class="btn btn-block btn-blue">Login & Checkout</button>
												</form>
												<a href="/forgot-password" class="forgot-link">Forgot password?</a>
											</div>

											<div class="divider">
												<span>OR</span>
											</div>

											<!-- Guest Checkout -->
											<div class="guest-section">
												<h4>New to Revolution Trading?</h4>
												<p>Create an account during checkout to track orders and access your purchases.</p>
												<button type="button" class="btn btn-block btn-orange" onclick={proceedAsGuest}>
													Proceed as Guest
												</button>
											</div>
										</div>
									</div>

									<!-- Trust Badges -->
									<div class="trust-badges card">
										<div class="card-body">
											<div class="badge-item">
												<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/></svg>
												<span>Secure Checkout</span>
											</div>
											<div class="badge-item">
												<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4v-6h16v6zm0-10H4V6h16v2z"/></svg>
												<span>All Major Cards Accepted</span>
											</div>
											<div class="badge-item">
												<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/></svg>
												<span>30-Day Money Back</span>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</section>

		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching Simpler Trading Non-Member Cart
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(body) {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		margin: 0;
		padding: 0;
		background: #f4f4f4;
	}

	/* Breadcrumbs */
	.breadcrumbs {
		background: #f4f4f4;
		padding: 12px 0;
		border-bottom: 1px solid #dbdbdb;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.breadcrumbs li {
		font-size: 14px;
		color: #666;
	}

	.breadcrumbs a {
		color: #0984ae;
		text-decoration: none;
	}

	.separator {
		margin: 0 8px;
	}

	/* Container */
	.container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* Page Header */
	.cart-page-header {
		padding: 40px 0 30px;
	}

	.cart-page-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px;
		text-transform: uppercase;
	}

	.cart-page-header h6 {
		color: #666;
		font-size: 16px;
		font-weight: 500;
		margin: 0;
	}

	/* Grid */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-xs-12 {
		padding: 0 15px;
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 768px) {
		.col-md-7 {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}

		.col-md-5 {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}
	}

	/* Products */
	.products {
		display: flex;
		flex-direction: column;
		gap: 16px;
		margin-bottom: 20px;
	}

	.product.cart_item .card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		display: grid;
		grid-template-columns: 120px 1fr auto;
		overflow: hidden;
	}

	.card-media {
		margin: 0;
		grid-row: 1 / 3;
	}

	.card-image {
		width: 120px;
		height: 100%;
		min-height: 120px;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
	}

	.card-body {
		padding: 20px;
	}

	.card-title {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px;
	}

	.card-description {
		font-size: 14px;
		color: #666;
	}

	.bold {
		font-weight: 700;
		color: #1a1a1a;
	}

	.price {
		font-weight: 600;
		color: #1a1a1a;
	}

	.card-footer {
		padding: 20px;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		justify-content: center;
		gap: 12px;
		border-left: 1px solid #ededed;
	}

	.product-price {
		font-size: 18px;
		font-weight: 700;
		color: #1a1a1a;
	}

	.remove-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
	}

	.continue-shopping {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		margin-top: 16px;
	}

	.continue-shopping:hover {
		text-decoration: underline;
	}

	/* Cart Sidebar */
	.cart-sidebar {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.card-body {
		padding: 24px;
	}

	/* Cart Totals */
	.order-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 16px;
		border-bottom: 2px solid #1a1a1a;
		font-size: 20px;
		font-weight: 700;
		color: #1a1a1a;
	}

	.total-price {
		font-size: 24px;
	}

	.tax-note {
		font-size: 12px;
		color: #666;
		text-align: center;
		margin: 12px 0 24px;
	}

	/* Login Section */
	.login-section h4,
	.guest-section h4 {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 16px;
	}

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: #333;
		margin-bottom: 6px;
	}

	.form-control {
		width: 100%;
		padding: 12px 14px;
		border: 1px solid #dbdbdb;
		border-radius: 5px;
		font-size: 14px;
		box-sizing: border-box;
	}

	.form-control:focus {
		outline: none;
		border-color: #0984ae;
	}

	.forgot-link {
		display: block;
		text-align: center;
		margin-top: 12px;
		color: #0984ae;
		text-decoration: none;
		font-size: 13px;
	}

	.forgot-link:hover {
		text-decoration: underline;
	}

	/* Divider */
	.divider {
		display: flex;
		align-items: center;
		margin: 24px 0;
	}

	.divider::before,
	.divider::after {
		content: '';
		flex: 1;
		height: 1px;
		background: #dbdbdb;
	}

	.divider span {
		padding: 0 16px;
		color: #666;
		font-size: 12px;
		font-weight: 600;
	}

	/* Guest Section */
	.guest-section p {
		font-size: 14px;
		color: #666;
		margin: 0 0 16px;
		line-height: 1.5;
	}

	/* Trust Badges */
	.trust-badges .card-body {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.badge-item {
		display: flex;
		align-items: center;
		gap: 12px;
		color: #666;
		font-size: 14px;
	}

	.badge-item svg {
		color: #0984ae;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px 24px;
		font-size: 14px;
		font-weight: 700;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
		text-decoration: none;
	}

	.btn-block {
		width: 100%;
	}

	.btn-xs {
		padding: 8px 12px;
		font-size: 13px;
	}

	.btn-blue {
		background: #0984ae;
		color: #fff;
	}

	.btn-blue:hover {
		background: #076787;
	}

	.btn-orange {
		background: #f99e31;
		color: #fff;
	}

	.btn-orange:hover {
		background: #f88b09;
	}

	.btn-default {
		background: #f4f4f4;
		color: #1a1a1a;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.product.cart_item .card {
			grid-template-columns: 80px 1fr;
		}

		.card-footer {
			grid-column: 1 / -1;
			flex-direction: row;
			justify-content: space-between;
			border-left: none;
			border-top: 1px solid #ededed;
		}

		.card-image {
			width: 80px;
			min-height: 80px;
		}

		.cart-sidebar {
			margin-top: 24px;
		}
	}
</style>
