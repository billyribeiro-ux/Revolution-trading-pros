<script lang="ts">
	/**
	 * Dashboard - My Account Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact match of Simpler Trading's WooCommerce My Account page.
	 * Shows greeting and links to account sections.
	 *
	 * @version 4.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const userName = $derived($user?.name || $user?.email?.split('@')[0] || 'Guest');

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account', { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleLogout(): void {
		authStore.logout();
		goto('/', { replaceState: true });
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<!-- WooCommerce: .woocommerce-MyAccount-content -->
			<div class="woocommerce-MyAccount-content">
				<!-- Content Box - Simpler Trading Style -->
				<div class="content-box content-box--centered">
					<div class="content-box__section">
						<p>
							Hello <strong>{userName}</strong> (not <strong>{userName}</strong>? <button class="link-button" onclick={handleLogout}>Log out</button>)
						</p>

						<p class="u--margin-bottom-0">
							From your account dashboard you can view your <a href="/dashboard/orders">recent orders</a>, manage your <a href="/dashboard/addresses">billing address</a>, and <a href="/dashboard/account/edit">edit your password and account details</a>.
						</p>
					</div>
				</div>
			</div>
		</section>
	</div>

	<!-- Sidebar - WordPress: .dashboard__content-sidebar -->
	<aside class="dashboard__content-sidebar">
		<nav class="dashboard__nav-secondary">
			<ul class="woocommerce-MyAccount-navigation">
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--orders">
					<a href="/dashboard/orders">My Orders</a>
				</li>
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--subscriptions">
					<a href="/dashboard/subscriptions">My Subscriptions</a>
				</li>
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--coupons">
					<a href="/dashboard/coupons">Coupons</a>
				</li>
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--edit-address">
					<a href="/dashboard/addresses">Billing Address</a>
				</li>
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--payment-methods">
					<a href="/dashboard/payment-methods">Payment Methods</a>
				</li>
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--edit-account">
					<a href="/dashboard/account/edit">Account Details</a>
				</li>
				<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--logout">
					<button onclick={handleLogout}>Log out</button>
				</li>
			</ul>
		</nav>
	</aside>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		gap: 30px;
		padding: 30px;
		background: #f4f4f4;
		min-height: calc(100vh - 80px);
	}

	.dashboard__content-main {
		flex: 1;
		min-width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT BOX - WooCommerce Style (Simpler Trading EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		background: #fff;
		border-radius: 4px;
		padding: 30px;
	}

	.woocommerce-MyAccount-content {
		max-width: 800px;
	}

	.content-box {
		background: #fafafa;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
	}

	.content-box--centered {
		text-align: left;
	}

	.content-box__section {
		padding: 30px;
	}

	.content-box__section p {
		color: #333;
		font-size: 16px;
		line-height: 1.7;
		margin: 0 0 15px;
	}

	.content-box__section p:last-child {
		margin-bottom: 0;
	}

	.content-box__section strong {
		font-weight: 700;
	}

	.content-box__section a {
		color: #0984ae;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.content-box__section a:hover {
		color: #076787;
		text-decoration: underline;
	}

	.link-button {
		background: none;
		border: none;
		padding: 0;
		color: #0984ae;
		font-size: inherit;
		font-family: inherit;
		cursor: pointer;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.link-button:hover {
		color: #076787;
		text-decoration: underline;
	}

	.u--margin-bottom-0 {
		margin-bottom: 0 !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - WooCommerce MyAccount Navigation
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-sidebar {
		width: 240px;
		flex-shrink: 0;
	}

	.dashboard__nav-secondary {
		background: #fff;
		border-radius: 4px;
		overflow: hidden;
	}

	.woocommerce-MyAccount-navigation {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.woocommerce-MyAccount-navigation-link {
		border-bottom: 1px solid #e5e5e5;
	}

	.woocommerce-MyAccount-navigation-link:last-child {
		border-bottom: none;
	}

	.woocommerce-MyAccount-navigation-link a,
	.woocommerce-MyAccount-navigation-link button {
		display: block;
		width: 100%;
		padding: 14px 20px;
		color: #333;
		font-size: 14px;
		font-weight: 400;
		text-decoration: none;
		text-align: left;
		background: none;
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.woocommerce-MyAccount-navigation-link a:hover,
	.woocommerce-MyAccount-navigation-link button:hover {
		background: #f4f4f4;
		color: #0984ae;
	}

	.woocommerce-MyAccount-navigation-link.is-active a {
		background: #0984ae;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 991px) {
		.dashboard__content {
			flex-direction: column-reverse;
		}

		.dashboard__content-sidebar {
			width: 100%;
		}

		.woocommerce-MyAccount-navigation {
			display: flex;
			flex-wrap: wrap;
			gap: 2px;
		}

		.woocommerce-MyAccount-navigation-link {
			border-bottom: none;
		}

		.woocommerce-MyAccount-navigation-link a,
		.woocommerce-MyAccount-navigation-link button {
			padding: 10px 16px;
			border: 1px solid #e5e5e5;
			border-radius: 4px;
		}
	}

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.dashboard__content-section {
			padding: 20px;
		}

		.content-box__section {
			padding: 20px;
		}

		.content-box__section p {
			font-size: 14px;
		}
	}
</style>
