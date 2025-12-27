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
	   100% PIXEL-PERFECT STYLES - Matching WordPress myaccount reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Header - Exact match to reference (white background) */
	.dashboard__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		flex-wrap: wrap;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	/* Page title - Exact match to reference (dark text on white) */
	.dashboard__page-title {
		font-family: 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: #333;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content - Exact match to reference */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		background-color: #efefef;
		min-width: 0;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 0;
		overflow-x: auto;
		overflow-y: hidden;
	}

	/* WooCommerce Content Box - PIXEL PERFECT match to screenshot
	   Light blue/teal info box style used in WordPress WooCommerce */
	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.content-box {
		background: #d9edf7;
		border: 1px solid #bce8f1;
		border-radius: 4px;
		overflow: hidden;
	}

	.content-box--centered {
		max-width: 100%;
	}

	.content-box__section {
		padding: 20px 25px;
		border-left: 4px solid #31708f;
	}

	.content-box__section p {
		font-size: 15px;
		line-height: 1.6;
		color: #31708f;
		margin: 0 0 10px;
		font-family: 'Open Sans', sans-serif;
	}

	.content-box__section p:last-child,
	.content-box__section .u--margin-bottom-0 {
		margin-bottom: 0;
	}

	.content-box__section a {
		color: #31708f;
		text-decoration: underline;
		font-weight: 600;
		transition: all 0.15s ease-in-out;
	}

	.content-box__section a:hover {
		color: #245269;
		text-decoration: underline;
	}

	.content-box__section strong {
		font-weight: 700;
		color: #31708f;
	}

	/* Responsive - Exact match to reference */
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 15px 20px;
		}

		.dashboard__page-title {
			font-size: 30px;
		}

		.dashboard__content-section {
			padding: 20px;
		}

		.content-box__section {
			padding: 15px 20px;
		}

		.content-box__section p {
			font-size: 14px;
		}
	}

	@media (max-width: 575px) {
		.dashboard__page-title {
			font-size: 24px;
		}
	}
</style>
