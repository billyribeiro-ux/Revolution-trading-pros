<script lang="ts">
	import type { AccountPageData } from './+page.d';

	interface Props {
		data: AccountPageData;
	}

	let { data }: Props = $props();

	// Derived user data
	let firstName = $derived(data.profile?.firstName || '');
	let lastName = $derived(data.profile?.lastName || '');
	let displayName = $derived(`${firstName} ${lastName}`.trim() || 'Member');
</script>

<svelte:head>
	<title>My Account - Revolution Trading Pros</title>
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- Dashboard Content Section -->
<section class="dashboard__content-section">
	<div class="fl-builder-content fl-builder-content-33 fl-builder-content-primary fl-builder-global-templates-locked" data-post-id="33">
		<div class="fl-row fl-row-fixed-width fl-row-bg-color fl-node-59793676724ad" data-node="59793676724ad">
			<div class="fl-row-content-wrap">
				<div class="fl-row-content fl-row-fixed-width fl-node-content">
					<div class="fl-col-group fl-node-597936767334e" data-node="597936767334e">
						<div class="fl-col fl-node-5979367673419" data-node="5979367673419">
							<div class="fl-col-content fl-node-content">
								<div class="fl-module fl-module-rich-text fl-node-59793676759ab dashboard-nav" data-node="59793676759ab">
									<div class="fl-module-content fl-node-content">
										<div class="fl-rich-text">
											<div class="woocommerce">
												<div class="woocommerce-MyAccount-content">
													<div class="woocommerce-notices-wrapper"></div>
													<div class="content-box content-box--centered">
														<div class="content-box__section">
															<p>
																Hello <strong>{displayName}</strong> (not <strong>{displayName}</strong>? <a href="/logout">Log out</a>)
															</p>
															
															<p class="u--margin-bottom-0">
																From your account dashboard you can view your <a href="/dashboard/account/orders">recent orders</a>, manage your <a href="/dashboard/account/edit-address">billing address</a>, and <a href="/dashboard/account/edit-account">edit your password and account details</a>.
															</p>
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
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 600;
		color: #0a2335;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content Section */
	.dashboard__content-section {
		padding: 0;
	}

	/* Beaver Builder Structure */
	.fl-builder-content {
		width: 100%;
	}

	.fl-row {
		width: 100%;
		position: relative;
	}

	.fl-row-content-wrap {
		position: relative;
	}

	.fl-row-content {
		margin-left: auto;
		margin-right: auto;
	}

	.fl-row-fixed-width {
		max-width: 1100px;
	}

	.fl-col-group {
		display: flex;
		flex-wrap: wrap;
	}

	.fl-col {
		flex: 1;
		min-width: 0;
	}

	.fl-col-content {
		padding: 20px;
	}

	.fl-module {
		margin-bottom: 20px;
	}

	.fl-module-content {
		position: relative;
	}

	.fl-rich-text {
		line-height: 1.6;
	}

	/* WooCommerce Content */
	.woocommerce {
		background: #fff;
		padding: 0;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.content-box {
		background: #fff;
		border: 1px solid #e5e5e5;
		margin-bottom: 20px;
	}

	.content-box--centered {
		max-width: 100%;
	}

	.content-box__section {
		padding: 24px;
	}

	.woocommerce-MyAccount-content p {
		font-size: 14px;
		line-height: 1.6;
		color: #515151;
		margin-bottom: 16px;
	}

	.woocommerce-MyAccount-content p:first-of-type {
		margin-top: 0;
	}

	.u--margin-bottom-0 {
		margin-bottom: 0 !important;
	}

	.woocommerce-MyAccount-content a {
		color: #0073aa;
		text-decoration: none;
	}

	.woocommerce-MyAccount-content a:hover {
		color: #005177;
		text-decoration: underline;
	}

	.woocommerce-MyAccount-content strong {
		font-weight: 600;
		color: #333;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 15px 20px;
		}

		.dashboard__page-title {
			font-size: 20px;
		}

		.fl-col-content {
			padding: 15px;
		}
	}
</style>
