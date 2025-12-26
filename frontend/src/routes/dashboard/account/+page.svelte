<script lang="ts">
	/**
	 * My Account Page - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact replica of Simpler Trading Dashboard Account welcome page
	 * Reference: DashboardAccount lines 2940-2967
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// User data - would come from auth/store in production
	const user = {
		name: 'Zack Stambowski',
		email: 'welberribeitodrums@gmail.com'
	};
</script>

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
</svelte:head>

<!-- Main Content - Exact from DashboardAccount reference -->
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
								Hello <strong>{user.name}</strong> (not <strong>{user.name}</strong>? <a href="/dashboard/account/customer-logout">Log out</a>)
							</p>
							<p class="u--margin-bottom-0">
								From your account dashboard you can view your <a href="/dashboard/account/orders">recent orders</a>, manage your <a href="/dashboard/account/edit-address">billing address</a>, and <a href="/dashboard/account/edit-account">edit your password and account details</a>.
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
	   100% PIXEL-PERFECT STYLES - Matching DashboardAccount reference
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

	/* Dashboard Content - Exact match to reference dashboard-globals.css lines 258-280 */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
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

	.dashboard__content-section + .dashboard__content-section {
		border-top: 1px solid #dbdbdb;
	}

	/* WooCommerce Content Box */
	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.content-box {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
	}

	.content-box--centered {
		max-width: 800px;
	}

	.content-box__section {
		padding: 30px;
	}

	.content-box__section p {
		font-size: 15px;
		line-height: 1.7;
		color: #333;
		margin: 0 0 15px;
		font-family: 'Open Sans', sans-serif;
	}

	.content-box__section p:last-child,
	.content-box__section .u--margin-bottom-0 {
		margin-bottom: 0;
	}

	.content-box__section a {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
		transition: all 0.15s ease-in-out;
	}

	.content-box__section a:hover {
		color: #065a75;
		text-decoration: underline;
	}

	.content-box__section strong {
		font-weight: 700;
	}

	/* Responsive - Exact match to reference */
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 15px 20px;
		}

		.dashboard__page-title {
			font-size: 30px;
		}

		/* Mobile: content-section padding reduced, NOT dashboard__content */
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

	@media (max-width: 575px) {
		.dashboard__page-title {
			font-size: 24px;
		}
	}
</style>
