<script lang="ts">
	/**
	 * Non-Member Cart/Checkout Page - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Exact replica of SimplerNonMemberCart reference
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// Form state
	let showRegister = $state(false);
	let loginUsername = $state('');
	let loginPassword = $state('');
	let rememberMe = $state(false);
	let registerFirstName = $state('');
	let registerLastName = $state('');
	let registerEmail = $state('');
	let registerPassword = $state('');

	// Cart data - matching reference
	const cartItem = {
		name: "Tr3ndy Trading (Monthly)",
		price: 247.00,
		image: "https://cdn.simplertrading.com/2021/08/22135621/tr3ndy-trading-monthly.jpg"
	};

	function toggleForms() {
		showRegister = !showRegister;
	}

	function handleLogin(e: Event) {
		e.preventDefault();
		console.log('Logging in...');
	}

	function handleRegister(e: Event) {
		e.preventDefault();
		console.log('Registering...');
	}
</script>

<svelte:head>
	<title>Checkout | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Breadcrumbs - Exact from reference -->
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			<li class="item-home"><a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a></li>
			<li class="separator separator-home"> / </li>
			<li class="item-current item-32"><strong class="breadcrumb-current breadcrumb-32"> Checkout</strong></li>
		</ul>
	</div>
</nav>

<!-- Page Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">

		<section class="wl-page-content">
			<div class="container-fluid">
				<div class="woocommerce">
					<div class="woocommerce-notices-wrapper"></div>

					<div class="container-fluid">
						<div class="row">
							<!-- LEFT COLUMN: Login/Register Forms -->
							<div class="col-xs-12 col-sm-7 col-md-8 order-2 order-sm-1">

								<!-- Checkout Navigation -->
								<nav class="checkout-nav hidden-xs">
									<ul>
										<li id="nav-checkout-account" class="active"><span>Sign In / Register</span></li>
										<li id="nav-checkout-billing" class="disabled"><span>Billing Info</span></li>
										<li id="nav-checkout-payment" class="disabled"><span>Payment</span></li>
									</ul>
								</nav>

								<div class="checkout-steps">
									<div id="checkout-account" class="checkout-step active">

										<!-- Login Form -->
										<div class="checkout-account-login" class:is-visible={!showRegister}>
											<div class="card">
												<div class="card-body">
													<div class="checkout-account-login-left">
														<h3>Sign in</h3>
														<form id="form-login" name="login" onsubmit={handleLogin}>
															<div class="form-message" role="alert"></div>

															<div class="form-group required">
																<label for="login_username">Email Address</label>
																<input type="text" class="form-control" name="username" id="login_username" bind:value={loginUsername} />
																<div class="help-block input-errors">Email Address is required.</div>
															</div>

															<div class="form-group required">
																<label for="login_password">Password</label>
																<input class="form-control" type="password" name="password" id="login_password" bind:value={loginPassword} />
																<div class="help-block input-errors">Password is required.</div>
															</div>

															<div class="form-group">
																<div class="checkbox">
																	<label class="display" for="login_remember">Remember me</label>
																	<input name="remember" type="checkbox" id="login_remember" value="forever" bind:checked={rememberMe} />
																</div>
															</div>

															<div class="form-group">
																<input type="submit" class="btn btn-lg btn-block btn-orange font-weight-bold" name="login" value="Sign In" />
															</div>

															<div class="text-center font-size-sm">
																<a href="/dashboard/account/lost-password">Forgot your password?</a>
															</div>
														</form>
													</div>

													<div class="checkout-account-login-right text-center">
														<h3>New to Simpler Trading?</h3>
														<p>You will need an account to purchase our classes and other products.</p>
														<button type="button" class="toggle-account-forms btn btn-lg btn-block btn-orange font-weight-bold" onclick={toggleForms}>Register for an Account</button>
													</div>
												</div>
											</div>
										</div>

										<!-- Register Form -->
										<div class="checkout-account-register" class:is-visible={showRegister}>
											<form id="form-register" name="register" onsubmit={handleRegister}>
												<div class="card">
													<div class="card-body">
														<div class="row">
															<div class="col-xs-12">
																<h3>Register for an Account</h3>
																<div class="form-message" role="alert"></div>

																<div class="row">
																	<div class="col-xs-12 col-sm-6">
																		<div class="form-group required">
																			<label for="register_first_name">First Name</label>
																			<input type="text" class="form-control" name="first_name" id="register_first_name" bind:value={registerFirstName} />
																			<div class="help-block input-errors">First Name is required.</div>
																		</div>
																	</div>

																	<div class="col-xs-12 col-sm-6">
																		<div class="form-group required">
																			<label for="register_last_name">Last Name</label>
																			<input type="text" class="form-control" name="last_name" id="register_last_name" bind:value={registerLastName} />
																			<div class="help-block input-errors">Last Name is required.</div>
																		</div>
																	</div>
																</div>

																<div class="form-group required">
																	<label for="register_email">Email Address</label>
																	<input type="text" class="form-control" name="email" id="register_email" bind:value={registerEmail} />
																	<div class="help-block input-errors">Email Address is required.</div>
																</div>

																<div class="form-group required">
																	<label for="register_password">Password</label>
																	<input class="form-control" type="password" name="password" id="register_password" bind:value={registerPassword} />
																	<div class="help-block input-errors">Password is required.</div>
																</div>
															</div>
														</div>
													</div>

													<div class="card-footer">
														<div class="row flex-grid mod-flex-items-center">
															<div class="col-xs-12 col-sm-6 col-sm-push-6 col-md-4 col-md-push-8 flex-grid-item rm-m-bot">
																<div class="form-group rm-m-sm rm-m-md rm-m-lg">
																	<input type="submit" class="btn btn-lg btn-block btn-orange font-weight-bold" name="register" value="Register" />
																</div>
															</div>
															<div class="col-xs-12 col-sm-6 col-sm-pull-6 col-md-4 col-md-pull-4 text-right flex-grid-item rm-m-bot">
																<div class="form-group rm-m">
																	<button type="button" class="toggle-account-forms btn btn-block btn-lg btn-default" onclick={toggleForms}>
																		<span class="fa fa-angle-left"></span>&nbsp;&nbsp;Back to Sign In
																	</button>
																</div>
															</div>
														</div>
													</div>
												</div>
											</form>
										</div>

									</div>
								</div>
							</div>

							<!-- RIGHT COLUMN: Cart Sidebar -->
							<div class="col-xs-12 col-sm-5 col-md-4 order-1 order-sm-2">
								<div class="checkout-cart">
									<div class="card">

										<div id="order_review" class="woocommerce-checkout-review-order">
											<div class="woocommerce-checkout-review-order-table">
												<div class="card-header hidden-xs">
													<h4 class="font-weight-bold">Shopping Cart</h4>
													<a href="/cart" class="btn btn-xs btn-default">Edit</a>
												</div>

												<div class="checkout-cart-contents">
													<div class="checkout-cart-products">
														<div class="product">
															<div class="product-image" style="background-image: url({cartItem.image});">
																<img class="default-background" width="50px" height="50px" alt="" />
															</div>
															<div class="product-name">
																{cartItem.name}
															</div>
															<div class="product-total">
																<span class="woocommerce-Price-amount amount"><bdi><span class="woocommerce-Price-currencySymbol">$</span>{cartItem.price.toFixed(2)}</bdi></span>
															</div>
														</div>
													</div>

													<!-- Tax Table -->
													<div class="cart-table-wrapper">
														<table class="cart-table">
															<tbody>
																<tr class="tax-total">
																	<th>Tax</th>
																	<td><span class="woocommerce-Price-amount amount"><bdi><span class="woocommerce-Price-currencySymbol">$</span>0.00</bdi></span></td>
																</tr>
															</tbody>
														</table>
													</div>

													<!-- Order Total -->
													<div class="checkout-order-total">
														<div>Total</div>
														<div class="checkout-order-total-price">
															<span class="woocommerce-Price-amount amount"><bdi><span class="woocommerce-Price-currencySymbol">$</span>{cartItem.price.toFixed(2)}</bdi></span>
															<del class="checkout-order-total-price-regular"><span class="woocommerce-Price-amount amount"><bdi><span class="woocommerce-Price-currencySymbol">$</span>{cartItem.price.toFixed(2)}</bdi></span></del>
														</div>
													</div>

													<!-- Recurring Totals -->
													<div class="cart-table-wrapper">
														<table class="cart-table">
															<tbody>
																<tr class="recurring-totals">
																	<th colspan="2">Recurring Totals</th>
																</tr>
																<tr class="cart-subtotal recurring-total">
																	<th rowspan="1">Subtotal</th>
																	<td data-title="Subtotal"><span class="woocommerce-Price-amount amount"><span class="woocommerce-Price-currencySymbol">$</span>{cartItem.price.toFixed(2)}</span> / month</td>
																</tr>
																<tr class="tax-total recurring-total">
																	<th>Tax</th>
																	<td data-title="Tax"><span class="woocommerce-Price-amount amount"><span class="woocommerce-Price-currencySymbol">$</span>0.00</span> / month</td>
																</tr>
																<tr class="order-total recurring-total">
																	<th rowspan="1">Recurring Total</th>
																	<td data-title="Recurring Total">
																		<strong><span class="woocommerce-Price-amount amount"><span class="woocommerce-Price-currencySymbol">$</span>{cartItem.price.toFixed(2)}</span></strong> / month
																		<div class="first-payment-date"><small>First renewal: January 9, 2026</small></div>
																	</td>
																</tr>
															</tbody>
														</table>
														<div class="recurring_details" style="font-size: 14px;">
															<br>
														</div>
													</div>
												</div>
											</div>
										</div>

										<!-- Coupon Section -->
										<div class="checkout-cart-contents">
											<div class="woocommerce-form-coupon-toggle">
												<div class="woocommerce-info">
													Have a coupon? <a href="#" class="showcoupon">Click here to enter your code</a>
												</div>
											</div>

											<form class="checkout_coupon woocommerce-form-coupon" method="post" style="display:none">
												<p>If you have a coupon code, please apply it below.</p>
												<p class="form-row form-row-first">
													<label for="coupon_code" class="screen-reader-text">Coupon:</label>
													<input type="text" name="coupon_code" class="input-text" placeholder="Coupon code" id="coupon_code" value="" />
												</p>
												<p class="form-row form-row-last">
													<button type="submit" class="button" name="apply_coupon" value="Apply coupon">Apply coupon</button>
												</p>
												<div class="clear"></div>
											</form>
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

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerNonMemberCart reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(body) {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		margin: 0;
		padding: 0;
		background: #f4f4f4;
	}

	/* Breadcrumbs - Exact from reference */
	.breadcrumbs {
		background-color: #f5f5f5;
		border-bottom: 1px solid #e5e5e5;
	}

	.breadcrumbs .container-fluid {
		padding-top: 10px;
		padding-bottom: 10px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		font-size: 13px;
	}

	.breadcrumbs a {
		color: #666;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		color: #0984ae;
	}

	.breadcrumbs .separator {
		margin: 0 8px;
		color: #999;
	}

	.breadcrumbs strong {
		color: #333;
		font-weight: 600;
	}

	/* Container */
	.container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 15px;
		width: 100%;
	}

	/* Page */
	#page {
		background: #f4f4f4;
	}

	.wl-page-content {
		padding: 30px 0;
	}

	/* Grid System - Bootstrap-like */
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

	@media (min-width: 576px) {
		.col-sm-5 { flex: 0 0 41.666667%; max-width: 41.666667%; }
		.col-sm-6 { flex: 0 0 50%; max-width: 50%; }
		.col-sm-7 { flex: 0 0 58.333333%; max-width: 58.333333%; }
		.col-sm-push-6 { left: 50%; position: relative; }
		.col-sm-pull-6 { right: 50%; position: relative; }
		.order-sm-1 { order: 1; }
		.order-sm-2 { order: 2; }
	}

	@media (min-width: 768px) {
		.col-md-4 { flex: 0 0 33.333333%; max-width: 33.333333%; }
		.col-md-8 { flex: 0 0 66.666667%; max-width: 66.666667%; }
		.col-md-push-8 { left: 66.666667%; position: relative; }
		.col-md-pull-4 { right: 33.333333%; position: relative; }
	}

	.order-1 { order: 1; }
	.order-2 { order: 2; }

	/* Checkout Navigation */
	.checkout-nav {
		background: #fff;
		border-radius: 8px;
		margin-bottom: 20px;
		padding: 0;
		overflow: hidden;
	}

	.checkout-nav ul {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.checkout-nav li {
		flex: 1;
		text-align: center;
		padding: 15px 20px;
		font-size: 14px;
		font-weight: 600;
		color: #999;
		border-right: 1px solid #ededed;
		position: relative;
	}

	.checkout-nav li:last-child {
		border-right: none;
	}

	.checkout-nav li.active {
		color: #333;
		background: #fff;
	}

	.checkout-nav li.active::after {
		content: '';
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: #0984ae;
	}

	.checkout-nav li.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.hidden-xs {
		display: block;
	}

	@media (max-width: 575px) {
		.hidden-xs {
			display: none !important;
		}
	}

	/* Cards */
	.card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		margin-bottom: 20px;
		overflow: hidden;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 15px 20px;
		background: #f9f9f9;
		border-bottom: 1px solid #ededed;
	}

	.card-header h4 {
		margin: 0;
		font-size: 16px;
		font-weight: 700;
	}

	.card-body {
		padding: 25px;
	}

	.card-footer {
		padding: 20px 25px;
		background: #f9f9f9;
		border-top: 1px solid #ededed;
	}

	/* Login Form Structure */
	.checkout-account-login {
		display: none;
	}

	.checkout-account-login.is-visible {
		display: block;
	}

	.checkout-account-login .card-body {
		display: flex;
		gap: 40px;
	}

	.checkout-account-login-left {
		flex: 1;
		padding-right: 40px;
		border-right: 1px solid #ededed;
	}

	.checkout-account-login-right {
		flex: 0 0 280px;
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	@media (max-width: 767px) {
		.checkout-account-login .card-body {
			flex-direction: column;
			gap: 30px;
		}

		.checkout-account-login-left {
			padding-right: 0;
			border-right: none;
			border-bottom: 1px solid #ededed;
			padding-bottom: 30px;
		}

		.checkout-account-login-right {
			flex: none;
		}
	}

	/* Register Form */
	.checkout-account-register {
		display: none;
	}

	.checkout-account-register.is-visible {
		display: block;
	}

	/* Headings */
	h3 {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Form Elements */
	.form-group {
		margin-bottom: 20px;
	}

	.form-group label {
		display: block;
		font-size: 14px;
		font-weight: 600;
		color: #333;
		margin-bottom: 8px;
	}

	.form-control {
		width: 100%;
		padding: 12px 14px;
		border: 1px solid #dbdbdb;
		border-radius: 5px;
		font-size: 14px;
		box-sizing: border-box;
		transition: border-color 0.15s ease;
	}

	.form-control:focus {
		outline: none;
		border-color: #0984ae;
	}

	.help-block {
		display: none;
		font-size: 12px;
		color: #dc3545;
		margin-top: 5px;
	}

	.form-group.has-error .help-block {
		display: block;
	}

	.form-group.has-error .form-control {
		border-color: #dc3545;
	}

	/* Checkbox */
	.checkbox {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.checkbox input[type="checkbox"] {
		width: 16px;
		height: 16px;
		order: -1;
	}

	.checkbox label {
		margin-bottom: 0;
		font-weight: 400;
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

	.btn-lg {
		padding: 16px 28px;
		font-size: 15px;
	}

	.btn-xs {
		padding: 8px 12px;
		font-size: 12px;
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
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	.font-weight-bold {
		font-weight: 700;
	}

	/* Text utilities */
	.text-center {
		text-align: center;
	}

	.text-right {
		text-align: right;
	}

	.font-size-sm {
		font-size: 13px;
	}

	.font-size-sm a {
		color: #0984ae;
		text-decoration: none;
	}

	.font-size-sm a:hover {
		text-decoration: underline;
	}

	/* Flex grid utilities */
	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid-item {
		flex: 0 0 auto;
	}

	.rm-m-bot { margin-bottom: 0; }
	.rm-m { margin: 0; }
	.rm-m-sm, .rm-m-md, .rm-m-lg { margin: 0; }

	/* Cart Sidebar */
	.checkout-cart {
		position: sticky;
		top: 20px;
	}

	/* Product in cart */
	.checkout-cart-products {
		padding: 0 20px;
	}

	.product {
		display: flex;
		align-items: center;
		padding: 15px 0;
		border-bottom: 1px solid #ededed;
	}

	.product:last-child {
		border-bottom: none;
	}

	.product-image {
		width: 50px;
		height: 50px;
		border-radius: 4px;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		flex-shrink: 0;
	}

	.product-image img {
		display: none;
	}

	.product-name {
		flex: 1;
		padding: 0 15px;
		font-size: 14px;
		font-weight: 600;
		color: #333;
	}

	.product-total {
		font-size: 14px;
		font-weight: 700;
		color: #333;
	}

	/* Cart Contents */
	.checkout-cart-contents {
		padding: 15px 20px;
	}

	/* Cart Table */
	.cart-table-wrapper {
		margin: 15px 0;
	}

	.cart-table {
		width: 100%;
		border-collapse: collapse;
	}

	.cart-table th,
	.cart-table td {
		padding: 10px 0;
		font-size: 13px;
		vertical-align: top;
	}

	.cart-table th {
		text-align: left;
		color: #666;
		font-weight: 600;
	}

	.cart-table td {
		text-align: right;
		color: #333;
	}

	.cart-table .recurring-totals th {
		font-weight: 700;
		color: #333;
		padding-top: 15px;
		border-top: 1px solid #ededed;
	}

	.cart-table .recurring-total th,
	.cart-table .recurring-total td {
		font-size: 12px;
	}

	.cart-table .order-total th,
	.cart-table .order-total td {
		font-weight: 700;
	}

	/* Order Total */
	.checkout-order-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 15px 0;
		border-top: 2px solid #333;
		margin-top: 15px;
		font-size: 16px;
		font-weight: 700;
		color: #333;
	}

	.checkout-order-total-price {
		text-align: right;
	}

	.checkout-order-total-price del {
		display: block;
		font-size: 12px;
		color: #999;
		font-weight: 400;
		text-decoration: line-through;
	}

	/* Price Amount */
	.woocommerce-Price-amount {
		font-weight: 600;
	}

	.woocommerce-Price-currencySymbol {
		font-weight: 600;
	}

	/* First payment date */
	.first-payment-date {
		margin-top: 5px;
	}

	.first-payment-date small {
		font-size: 11px;
		color: #666;
		font-weight: 400;
	}

	/* Coupon Section */
	.woocommerce-form-coupon-toggle {
		border-top: 1px solid #ededed;
		padding-top: 15px;
	}

	.woocommerce-info {
		font-size: 14px;
		color: #666;
	}

	.woocommerce-info a {
		color: #0984ae;
		text-decoration: none;
	}

	.woocommerce-info a:hover {
		text-decoration: underline;
	}

	.woocommerce-form-coupon {
		margin-top: 15px;
		padding: 15px;
		background: #f9f9f9;
		border-radius: 5px;
	}

	.woocommerce-form-coupon p {
		margin: 0 0 10px;
		font-size: 13px;
		color: #666;
	}

	.woocommerce-form-coupon .input-text {
		width: 100%;
		padding: 10px 12px;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		font-size: 14px;
		margin-bottom: 10px;
	}

	.woocommerce-form-coupon .button {
		background: #333;
		color: #fff;
		padding: 10px 20px;
		border: none;
		border-radius: 4px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
	}

	.woocommerce-form-coupon .button:hover {
		background: #222;
	}

	/* Responsive */
	@media (max-width: 575px) {
		.checkout-cart {
			position: static;
			margin-bottom: 20px;
		}

		.wl-page-content {
			padding: 20px 0;
		}
	}
</style>
