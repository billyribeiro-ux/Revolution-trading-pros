<script lang="ts">
	/**
	 * My Account Page - 100% PIXEL-PERFECT Match to WordPress Reference
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact replica of Simpler Trading Dashboard Account welcome page
	 * Reference: myaccount lines 2940-2957
	 *
	 * @version 2.1.0 - 100% Pixel Perfect with correct content-box styling
	 */
	import { user } from '$lib/stores/auth';

	// Fallback user data if not authenticated
	$: userName = $user?.name || 'Member';
</script>

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
</svelte:head>

<!-- Main Content - Exact from myaccount reference lines 2922-2967 -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<!-- WooCommerce MyAccount Content - Exact from reference lines 2942-2957 -->
			<div class="woocommerce">
				<div class="woocommerce-MyAccount-content">
					<div class="woocommerce-notices-wrapper"></div>
					<div class="content-box content-box--centered">
						<div class="content-box__section">
							<p>
								Hello <strong>{userName}</strong> (not <strong>{userName}</strong>? <a href="/dashboard/account/customer-logout">Log out</a>)
							</p>
							<p class="u--margin-bottom-0">
								From your account dashboard you can view your <a href="/dashboard/account/orders">recent orders</a>, manage your <a href="/dashboard/account/billing-address">billing address</a>, and <a href="/dashboard/account/edit-account">edit your password and account details</a>.
							</p>
						</div>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   My Account Page - Uses global styles from dashboard-globals.css
	   Only page-specific overrides are defined here
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* WooCommerce wrapper reset */
	.woocommerce-MyAccount-content {
		padding: 0;
	}
</style>
