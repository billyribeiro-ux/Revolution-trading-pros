<script lang="ts">
	/**
	 * Shopping Cart Page - WordPress PIXEL-PERFECT REPLICA
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Exact conversion from WordPress /dashboard file (cart page)
	 * @version 7.0.0 (WordPress IDENTICAL / December 2025)
	 */

	import { browser } from '$app/environment';
	import { onMount } from 'svelte';

	// State
	let cartItems = $state([
		{
			id: '88765154a2216f5a8806f524e7fb22a1',
			productId: '1318641',
			name: 'Mastering the Trade Room (1 Month Trial)',
			price: 247.00,
			image: 'https://cdn.simplertrading.com/2020/11/01132413/Store-Card-MasteringTheTrade-Room.jpg',
			interval: 'monthly',
			trialPrice: 247.00
		}
	]);

	let couponCode = $state('');
	let cartTotal = $state(247.00);
	let recurringTotal = $state(247.00);
	let taxTotal = $state(0.00);

	function removeItem(itemId: string) {
		if (confirm('Remove this item?')) {
			cartItems = cartItems.filter(item => item.id !== itemId);
		}
	}

	function clearCart() {
		if (confirm('Clear All Items?')) {
			cartItems = [];
		}
	}

	function updateCart() {
		// Update cart logic
	}

	function proceedToCheckout() {
		window.location.href = '/checkout';
	}
</script>

<svelte:head>
	<title>Cart - Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Cart" />
	<link rel="stylesheet" href="https://www.simplertrading.com/wp-content/plugins/woocommerce/assets/css/woocommerce.css" />
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css" />
</svelte:head>

<!-- WordPress EXACT Structure -->
<div class="page-template-default page page-id-31 logged-in wp-custom-logo woocommerce-cart woocommerce-page">
	
	<!-- Header -->
	<header id="masthead" class="site-header">
		<div class="inside-header grid-container grid-parent">
			<div class="site-logo">
				<a href="/" title="Revolution Trading Pros" rel="home">
					<img class="header-image" alt="Revolution Trading Pros" src="https://cdn.simplertrading.com/2018/11/08120448/Trademark-Simpler-Logo-White.png" width="901" height="80" />
				</a>
			</div>
		</div>
	</header>

	<!-- Navigation -->
	<nav id="site-navigation" class="main-navigation">
		<div class="inside-navigation grid-container grid-parent">
			<div class="site-logo sticky-logo navigation-logo">
				<a href="/" title="Revolution Trading Pros" rel="home">
					<img src="https://cdn.simplertrading.com/2018/11/08120448/Trademark-Simpler-Logo-White.png" alt="Revolution Trading Pros" />
				</a>
			</div>
			<button class="menu-toggle" aria-controls="primary-menu" aria-expanded="false">
				<span class="gp-icon icon-menu-bars">
					<svg viewBox="0 0 512 512" width="1em" height="1em">
						<path d="M0 96c0-13.255 10.745-24 24-24h464c13.255 0 24 10.745 24 24s-10.745 24-24 24H24c-13.255 0-24-10.745-24-24zm0 160c0-13.255 10.745-24 24-24h464c13.255 0 24 10.745 24 24s-10.745 24-24 24H24c-13.255 0-24-10.745-24-24zm0 160c0-13.255 10.745-24 24-24h464c13.255 0 24 10.745 24 24s-10.745 24-24 24H24c-13.255 0-24-10.745-24-24z" />
					</svg>
				</span>
				<span class="mobile-menu">Menu</span>
			</button>
			<div id="primary-menu" class="main-nav">
				<ul id="menu-header-menu-2023" class="menu sf-menu">
					<li class="menu-item"><a href="/live-trading-rooms">Live Trading Rooms</a></li>
					<li class="menu-item"><a href="/courses">Learning</a></li>
					<li class="menu-item"><a href="/indicators">Tools</a></li>
					<li class="menu-item"><a href="/resources">Resources</a></li>
					<li class="menu-item"><a href="/about">About Us</a></li>
					<li class="menu-item"><a href="/dashboard">Dashboard</a></li>
					<li class="menu-item"><a href="tel:5122668659"><span class="fa fa-phone"></span> (512) 266-8659</a></li>
					<li class="menu-item menu-item-cart">
						<a href="/cart">
							<span class="cart-link-icon">
								<span class="fa fa-shopping-cart"></span>
								<span class="cart-items-count">{cartItems.length}</span>
							</span>
							<span class="cart-link-text">View My Cart</span>
						</a>
					</li>
				</ul>
			</div>
		</div>
	</nav>

	<!-- Breadcrumbs -->
	<nav id="breadcrumbs" class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home"><a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a></li>
				<li class="separator separator-home"> / </li>
				<li class="item-current item-31"><strong class="breadcrumb-current breadcrumb-31"> Cart</strong></li>
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
						<div class="woocommerce-notices-wrapper"></div>
						
						<form class="woocommerce-cart-form" action="/cart" method="post" onsubmit={(e) => e.preventDefault()}>
							<div class="woocommerce-cart-form__contents">
								<div class="row">
									<!-- Products Column -->
									<div class="col-xs-12 col-sm-7 col-md-8">
										<div class="wc_coupon_message_wrap" style="padding: 10px 0 10px;">
											<div class="no_wc_coupon_message"></div>
										</div>
										
										<div class="products" data-products-view="list">
											<div class="flex-grid">
												{#each cartItems as item}
													<!-- Product Card -->
													<article class="product flex-grid-item cart_item">
														<div class="card st{item.productId}">
															<figure class="card-media">
																<a href="/cart" class="card-image" style="background-image: url({item.image});">
																	<img class="default-background" width="540px" height="256px" alt={item.name} />
																</a>
															</figure>

															<section class="card-body">
																<h4 class="card-title">
																	{item.name}&nbsp;
																</h4>
																<div class="card-description">
																	<span class="membership-tagline">
																		<span class="bold">Subscription:</span> 
																		<span class="woocommerce-Price-amount amount">
																			<span class="woocommerce-Price-currencySymbol">$</span>{item.price.toFixed(2)}
																		</span>
																		<span class="subscription-details"> / month with 1 month trial for 
																			<span class="woocommerce-Price-amount amount">
																				<span class="woocommerce-Price-currencySymbol">$</span>{item.trialPrice.toFixed(2)}
																			</span>
																		</span>
																	</span>
																</div>
																<div class="product-quantity" data-title="Quantity">
																	<input type="hidden" name="cart[{item.id}][qty]" value="1" />
																</div>
															</section>

															<footer class="card-footer">
																<div class="product-add-to-cart product-remove">
																	<button 
																		type="button"
																		class="btn btn-xs btn-default" 
																		aria-label="Remove this item"
																		onclick={() => removeItem(item.id)}
																	>
																		<span class="fa fa-times"></span>&nbsp;Remove
																	</button>
																</div>
																<div class="product-price">
																	<span class="woocommerce-Price-amount-wrap sale-true">
																		<del class="del-regular-price product-price-regular">
																			<span class="woocommerce-Price-amount amount">
																				<span class="woocommerce-Price-currencySymbol">$</span>{item.price.toFixed(2)}
																			</span>
																		</del>
																		<ins class="">
																			<span class="woocommerce-Price-amount amount">
																				<span class="woocommerce-Price-currencySymbol">$</span>{item.price.toFixed(2)}
																			</span>
																		</ins>
																	</span>
																</div>
															</footer>
														</div>
													</article>
												{/each}

												<button type="submit" class="button update-cart" name="update_cart" value="Update cart" onclick={updateCart}>Update cart</button>
											</div>
										</div>

										<div class="clear-cart">
											<input type="button" onclick={clearCart} class="btn btn-lg btn-default" name="clear-cart" value="Clear Cart" />
										</div>

										<!-- Coupons List -->
										<div id="coupons_list" style="display: none;">
											<h3>Available Coupons (click on a coupon to use it)</h3>
											<div id="sc-cc">
												<div id="all_coupon_container" class="sc-coupons-list"></div>
											</div>
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
															<div class="order-total-price">&#036;{cartTotal.toFixed(2)}</div>
														</div>

														<!-- Recurring Totals Table -->
														<div class="cart-table-wrapper">
															<table class="cart-table">
																<tbody>
																	<tr class="recurring-totals">
																		<th colspan="2">Recurring Totals</th>
																	</tr>
																	<tr class="cart-subtotal recurring-total">
																		<th rowspan="1">Subtotal</th>
																		<td data-title="Subtotal">
																			<span class="woocommerce-Price-amount amount">
																				<span class="woocommerce-Price-currencySymbol">&#036;</span>{recurringTotal.toFixed(2)}
																			</span> / month
																		</td>
																	</tr>
																	<tr class="tax-total recurring-total">
																		<th>Tax</th>
																		<td data-title="Tax">
																			<span class="woocommerce-Price-amount amount">
																				<span class="woocommerce-Price-currencySymbol">&#036;</span>{taxTotal.toFixed(2)}
																			</span> / month
																		</td>
																	</tr>
																	<tr class="order-total recurring-total">
																		<th rowspan="1">Recurring Total</th>
																		<td data-title="Recurring Total">
																			<strong>
																				<span class="woocommerce-Price-amount amount">
																					<span class="woocommerce-Price-currencySymbol">&#036;</span>{recurringTotal.toFixed(2)}
																				</span>
																			</strong> / month
																			<div class="first-payment-date">
																				<small>First renewal: January 21, 2026</small>
																			</div>
																		</td>
																	</tr>
																</tbody>
															</table>
															<div class="recurring_details" style="font-size: 14px;"></div>
														</div>

														<!-- Proceed to Checkout -->
														<div class="cart-proceed-to-checkout">
															<div id="wc-stripe-payment-request-wrapper" style="clear:both;padding-top:1.5em;display:none;">
																<div id="wc-stripe-payment-request-button"></div>
															</div>
															<p id="wc-stripe-payment-request-button-separator" style="margin-top:1.5em;text-align:center;display:none;">&mdash; OR &mdash;</p>
															<button type="button" onclick={proceedToCheckout} class="btn btn-lg btn-block btn-orange">
																Proceed to checkout
															</button>
														</div>

														<p class="tax-disclaimer">* Plus applicable taxes</p>

														<!-- Coupon Input -->
														<div class="cart-coupon">
															<div class="input-group">
																<input type="text" name="coupon_code" class="form-control input-sm" id="coupon_code" bind:value={couponCode} placeholder="Coupon code" />
																<span class="input-group-btn">
																	<input type="submit" class="btn btn-sm btn-blue" name="apply_coupon" value="Apply" />
																</span>
															</div>
														</div>

														<input type="hidden" id="_wpnonce" name="_wpnonce" value="31d89a8fba" />
														<input type="hidden" name="_wp_http_referer" value="/cart" />
													</div>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>
						</form>

						<style>
							.coupon-q421jcsp, .coupon-q421jc10x, .coupon-q421jcrafp, .coupon-q421jcm10x, .coupon-q421jcra, .coupon-q421jcsamqt, .coupon-q421jcsps {
								display: none;
							}
						</style>
					</div>
				</div>
			</section>

		</div>
	</div>

	<!-- Loading Container -->
	<div class="loading-container loading-global">
		<div class="loading">
			<span class="loading-icon"></span>
			<span class="loading-message"></span>
		</div>
	</div>

	<!-- Footer -->
	<div class="site-footer">
		<footer class="fl-builder-content fl-builder-content-36965 fl-builder-global-templates-locked" data-post-id="36965">
			<div class="fl-row fl-row-full-width fl-row-bg-color fl-node-59adb3c86fcbf footer-section">
				<div class="fl-row-content-wrap">
					<div class="fl-row-content fl-row-fixed-width fl-node-content">
						<div class="fl-col-group fl-node-5b6c661a8e03f fl-col-group-equal-height fl-col-group-align-top fl-col-group-custom-width">
							<!-- Footer Logo Column -->
							<div class="fl-col fl-node-5b6c661a8e13a fl-col-small">
								<div class="fl-col-content fl-node-content">
									<div id="footer-logo" class="fl-module fl-module-html fl-node-59adb48fc0c6a">
										<div class="fl-module-content fl-node-content">
											<div class="fl-html">
												<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 216 20" style="enable-background:new 0 0 216 20;" width="216" height="20">
													<style type="text/css">.st0{fill:#FFFFFF;}</style>
													<g>
														<path class="st0" d="M34.2,13.6c0,1.4-0.5,2.6-1.6,3.4c-1,0.8-2.5,1.2-4.4,1.2c-1.7,0-3.2-0.3-4.6-0.9v-3.2c1.1,0.5,2,0.8,2.7,1s1.4,0.3,2,0.3c0.7,0,1.3-0.1,1.7-0.4c0.4-0.3,0.6-0.7,0.6-1.3c0-0.3-0.1-0.6-0.2-0.8c-0.2-0.2-0.4-0.5-0.8-0.7c-0.3-0.2-1-0.6-2.1-1.1c-1-0.4-1.7-0.9-2.2-1.3c-0.5-0.4-0.9-0.9-1.2-1.5c-0.3-0.5-0.4-1.2-0.4-2c0-1.4,0.5-2.5,1.4-3.3c0.9-0.8,2.3-1.2,4-1.2c0.8,0,1.6,0.1,2.4,0.3c0.7,0.2,1.5,0.5,2.4,0.8l-1.1,2.6c-0.8-0.3-1.6-0.6-2.1-0.7c-0.5-0.1-1.1-0.2-1.6-0.2c-0.6,0-1.1,0.1-1.5,0.4c-0.3,0.3-0.5,0.7-0.5,1.2c0,0.3,0,0.5,0.2,0.8c0.1,0.2,0.3,0.4,0.6,0.6c0.3,0.2,1,0.6,2.1,1.1c1.5,0.7,2.5,1.4,3.1,2.1C33.9,11.6,34.2,12.5,34.2,13.6z"/>
													</g>
												</svg>
											</div>
										</div>
									</div>
									<div class="fl-module fl-module-rich-text fl-node-5b6c66888faf0">
										<div class="fl-module-content fl-node-content">
											<div class="fl-rich-text">
												<p>
													<span style="font-size: 14px;"><a href="mailto:support@revolutiontradingpros.com">support@revolutiontradingpros.com</a></span><br />
													<span style="font-size: 14px;">8911 North Capital of Texas Hwy<br />Suite 4200 #1005</span><br />
													<span style="font-size: 14px;">Austin, TX 78759</span><br />
													<span style="font-size: 14px; margin-bottom: 0;">512-266-8659</span>
												</p>
											</div>
										</div>
									</div>
								</div>
							</div>

							<!-- Learn More Column -->
							<div class="fl-col fl-node-5b6c661a8e176 fl-col-small">
								<div class="fl-col-content fl-node-content">
									<div id="learn-more--footer" class="fl-module fl-module-rich-text fl-node-5b6c66514654a">
										<div class="fl-module-content fl-node-content">
											<div class="fl-rich-text">
												<div class="footer-margins">
													<p><strong style="font-size: 17px;">LEARN MORE</strong></p>
													<p><a href="/join"><span style="font-size: 14px;">Memberships</span></a></p>
													<p><a href="/daily-videos"><span style="font-size: 14px;">Daily Videos</span></a></p>
													<p><a href="/support"><span style="font-size: 14px;">Support</span></a></p>
													<p><a href="/partners"><span style="font-size: 14px;">Partners</span></a></p>
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>

							<!-- About Column -->
							<div class="fl-col fl-node-5b6c661a8e1b2 fl-col-small">
								<div class="fl-col-content fl-node-content">
									<div id="about--footer" class="fl-module fl-module-rich-text fl-node-5b6c693409ba7">
										<div class="fl-module-content fl-node-content">
											<div class="fl-rich-text">
												<p><strong style="font-size: 17px;">ABOUT</strong></p>
												<p><a href="/about"><span style="font-size: 14px;">About Revolution™</span></a></p>
												<p><a href="/traders"><span style="font-size: 14px;">Our Traders</span></a></p>
												<p><a href="/store"><span style="font-size: 14px;">Store</span></a></p>
												<p><a href="/blog"><span style="font-size: 14px;">Blog</span></a></p>
											</div>
										</div>
									</div>
								</div>
							</div>

							<!-- Legal Column -->
							<div class="fl-col fl-node-5b6c661a8e1ec fl-col-small">
								<div class="fl-col-content fl-node-content">
									<div id="legal--footer" class="fl-module fl-module-rich-text fl-node-5b6c6953e8ba2">
										<div class="fl-module-content fl-node-content">
											<div class="fl-rich-text">
												<p><strong style="font-size: 17px;">LEGAL</strong></p>
												<p><a href="/privacy-policy"><span style="font-size: 14px;">Privacy Policy</span></a></p>
												<p><a href="/terms-and-conditions"><span style="font-size: 14px;">Terms & Conditions</span></a></p>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Footer Disclaimer -->
			<div class="fl-row fl-row-full-width fl-row-bg-color fl-node-59adb3c86fa31">
				<div class="fl-row-content-wrap">
					<div class="fl-row-content fl-row-fixed-width fl-node-content">
						<div class="fl-col-group fl-node-59adb3c86fadc">
							<div class="fl-col fl-node-59adb3c86fb7e">
								<div class="fl-col-content fl-node-content">
									<div class="fl-module fl-module-rich-text fl-node-59adb3c86fc1f">
										<div class="fl-module-content fl-node-content">
											<div class="fl-rich-text">
												<p><span style="font-weight: 400;">The information contained on this website is solely for educational purposes, and does not constitute investment advice. The risk of trading in securities markets can be substantial. You must review and agree to our Disclaimers and Terms and Conditions before using this site.</span></p>
												<p><span style="font-weight: 400;">© 2025 <strong>Revolution Trading Pros</strong> ALL RIGHTS RESERVED. Reproduction without Permission Prohibited</span></p>
											</div>
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</footer>
	</div>

	<!-- Back to Top -->
	<a title="Scroll back to top" rel="nofollow" href="#" class="generate-back-to-top" style="opacity:0;visibility:hidden;">
		<span class="screen-reader-text">Scroll back to top</span>
		<span class="gp-icon icon-arrow">
			<svg viewBox="0 0 330 512" width="1em" height="1em">
				<path d="M305.913 197.085c0 2.266-1.133 4.815-2.833 6.514L171.087 335.593c-1.7 1.7-4.249 2.832-6.515 2.832s-4.815-1.133-6.515-2.832L26.064 203.599c-1.7-1.7-2.832-4.248-2.832-6.514s1.132-4.816 2.832-6.515l14.162-14.163c1.7-1.699 3.966-2.832 6.515-2.832 2.266 0 4.815 1.133 6.515 2.832l111.316 111.317 111.316-111.317c1.7-1.699 4.249-2.832 6.515-2.832s4.815 1.133 6.515 2.832l14.162 14.163c1.7 1.7 2.833 4.249 2.833 6.515z" fill-rule="nonzero"/>
			</svg>
		</span>
	</a>
</div>

<style>
	/* WordPress EXACT CSS - Pixel Perfect Match */
	:global(body) {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		margin: 0;
		padding: 0;
		background: #f4f4f4;
	}

	/* Header */
	.site-header {
		background: #0a2335;
		padding: 20px 0;
	}

	.inside-header {
		max-width: 1300px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.site-logo img {
		max-width: 200px;
		height: auto;
	}

	/* Navigation */
	.main-navigation {
		background-color: #fff;
		box-shadow: 0 2px 2px -2px rgba(0,0,0,.2);
		position: sticky;
		top: 0;
		z-index: 1000;
	}

	.inside-navigation {
		max-width: 1300px !important;
		margin: 0 auto;
		padding: 0 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.navigation-logo img {
		max-width: 150px;
		height: auto;
		filter: invert(10%) sepia(70%) saturate(730%) hue-rotate(165deg) brightness(20%) contrast(96%);
	}

	.menu-toggle {
		display: none;
		background: none;
		border: none;
		padding: 20px;
		cursor: pointer;
		color: #0a2335;
	}

	.main-nav ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.main-nav li {
		margin: 0;
	}

	.main-nav a {
		display: block;
		padding: 0 19px;
		line-height: 72px;
		color: #0a2335;
		text-decoration: none;
		font-weight: 700;
		font-size: 14px;
		text-transform: capitalize;
		transition: all 0.2s linear;
	}

	.main-nav a:hover {
		background: #0e6ac4 !important;
		color: #ffffff !important;
	}

	.menu-item-cart .cart-items-count {
		background: #f99e31;
		color: #fff;
		border-radius: 50%;
		padding: 2px 6px;
		font-size: 11px;
		font-weight: 700;
		margin-left: 4px;
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
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
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

	/* Page Content */
	.site-content {
		min-height: 60vh;
	}

	.container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* Cart Page Header */
	.cart-page-header {
		padding: 40px 0 30px;
	}

	.cart-page-title {
		font-family: 'Open Sans Condensed', sans-serif;
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

	/* Cart Form */
	.woocommerce-cart-form__contents {
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

	/* Products */
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
		background: #ffffff;
		border-radius: 5px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
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
		background: #f4f4f4;
		background-size: cover;
		background-position: center;
		display: block;
	}

	.card-image img {
		display: none;
	}

	.card-body {
		padding: 20px;
		display: flex;
		flex-direction: column;
		justify-content: center;
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

	.membership-tagline .bold {
		font-weight: 700;
		color: #1a1a1a;
	}

	.card-footer {
		padding: 20px;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		justify-content: center;
		gap: 12px;
		border-left: 1px solid #eeeeee;
	}

	.product-price {
		font-size: 18px;
		font-weight: 700;
		color: #1a1a1a;
	}

	.product-price del {
		text-decoration: line-through;
		color: #999;
		font-size: 14px;
		margin-right: 8px;
	}

	.update-cart {
		margin-top: 16px;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		font-weight: 700;
	}

	.clear-cart {
		margin-top: 16px;
	}

	/* Cart Totals */
	.cart-collaterals {
		position: sticky;
		top: 24px;
	}

	.cart-collaterals .card {
		background: #ffffff;
		border-radius: 5px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.cart-collaterals .card-body {
		padding: 24px;
	}

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

	.order-total-price {
		font-size: 24px;
	}

	.cart-table-wrapper {
		margin-top: 20px;
		padding-top: 16px;
		border-top: 1px solid #eeeeee;
	}

	.cart-table {
		width: 100%;
	}

	.cart-table th,
	.cart-table td {
		padding: 10px 0;
		font-size: 14px;
	}

	.cart-table th {
		text-align: left;
		font-weight: 700;
		color: #1a1a1a;
	}

	.cart-table td {
		text-align: right;
		color: #1a1a1a;
		font-weight: 600;
	}

	.recurring-totals th {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #666 !important;
		font-weight: 600;
	}

	.first-payment-date {
		margin-top: 4px;
	}

	.first-payment-date small {
		font-size: 12px;
		color: #666;
		font-weight: 400;
	}

	.cart-proceed-to-checkout {
		margin-top: 20px;
	}

	.tax-disclaimer {
		margin-top: 12px;
		font-size: 12px;
		color: #666;
		text-align: center;
		font-weight: 500;
	}

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
		border: 1px solid #dbdbdb;
		border-radius: 5px 0 0 5px;
		font-size: 16px;
		text-transform: uppercase;
		font-weight: 600;
	}

	.input-group-btn .btn {
		border-radius: 0 5px 5px 0;
	}

	/* Buttons */
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
		padding: 8px 12px;
		font-size: 13px;
	}

	.btn-block {
		width: 100%;
	}

	.btn-orange {
		background: #f99e31;
		color: #ffffff;
	}

	.btn-orange:hover {
		background: #f88b09;
	}

	.btn-blue {
		background: #0984ae;
		color: #ffffff;
	}

	.btn-blue:hover {
		background: #076787;
	}

	.btn-default {
		background: #f4f4f4;
		color: #1a1a1a;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
	}

	/* Footer */
	.site-footer {
		background: #0a2335;
		color: #ffffff;
		padding: 60px 0 20px;
		margin-top: 80px;
	}

	.footer-section {
		margin-bottom: 40px;
	}

	.fl-row-content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.fl-col-group {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 40px;
	}

	.fl-rich-text p {
		margin: 0 0 12px;
		line-height: 1.6;
	}

	.fl-rich-text a {
		color: #ffffff;
		text-decoration: none;
	}

	.fl-rich-text a:hover {
		text-decoration: underline;
	}

	.footer-margins {
		margin: 0;
	}

	/* Loading */
	.loading-container {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0,0,0,0.5);
		z-index: 9999;
		align-items: center;
		justify-content: center;
	}

	.loading-icon {
		width: 40px;
		height: 40px;
		border: 4px solid #f3f3f3;
		border-top: 4px solid #0984ae;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}

	/* Back to Top */
	.generate-back-to-top {
		position: fixed;
		bottom: 20px;
		right: 20px;
		background: #0984ae;
		color: #fff;
		width: 50px;
		height: 50px;
		border-radius: 5px;
		display: flex;
		align-items: center;
		justify-content: center;
		text-decoration: none;
		transition: opacity 0.3s, visibility 0.3s;
		z-index: 999;
	}

	.screen-reader-text {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		overflow: hidden;
		clip: rect(0,0,0,0);
		white-space: nowrap;
		border: 0;
	}

	/* Responsive */
	@media screen and (max-width: 980px) {
		.col-sm-7, .col-md-8 {
			flex: 0 0 100%;
			max-width: 100%;
			margin-bottom: 24px;
		}

		.col-sm-5, .col-md-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.cart-collaterals {
			position: static;
		}

		.fl-col-group {
			grid-template-columns: repeat(2, 1fr);
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

		.menu-toggle {
			display: block;
		}

		.main-nav {
			display: none;
		}

		.fl-col-group {
			grid-template-columns: 1fr;
		}
	}

	@media screen and (max-width: 480px) {
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
	}
</style>
