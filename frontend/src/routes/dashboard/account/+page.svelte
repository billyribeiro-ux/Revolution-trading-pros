<script lang="ts">
	import type { AccountPageData } from './+page.d';

	interface Props {
		data: AccountPageData;
	}

	let { data }: Props = $props();

	/**
	 * Display name with fallback chain:
	 * 1. Profile firstName + lastName
	 * 2. Auth user.name from parent layout
	 * 3. Email username formatted
	 * 4. "Member" as last resort
	 */
	let displayName = $derived(
		(() => {
			// Try profile first/last name
			const firstName = data.profile?.firstName || '';
			const lastName = data.profile?.lastName || '';
			const fullName = `${firstName} ${lastName}`.trim();
			if (fullName) return fullName;
			
			// Try parent layout's user.name (from auth)
			if (data.user?.name) return data.user.name;
			
			// Try email from profile or user
			const email = data.profile?.email || data.user?.email || '';
			if (email) {
				return email.split('@')[0]
					.replace(/[._]/g, ' ')
					.split(' ')
					.map((w: string) => w.charAt(0).toUpperCase() + w.slice(1))
					.join(' ');
			}
			
			return 'Member';
		})()
	);
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

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<div class="account-container">
				<div class="account-row">
					<div class="account-content">
						<div class="woocommerce">
							<div class="woocommerce-MyAccount-content">
								<div class="woocommerce-notices-wrapper"></div>
								<div class="content-box content-box--centered">
									<div class="content-box__section">
										<p>
											Hello <strong>{displayName}</strong> (not <strong>{displayName}</strong>? <a href="/logout">Log out</a>)
										</p>
										<p class="account-text--no-margin">
											From your account dashboard you can view your <a href="/dashboard/account/orders">recent orders</a>, manage your <a href="/dashboard/account/edit-address">billing address</a>, and <a href="/dashboard/account/edit-account">edit your password and account details</a>.
										</p>
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
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-family: 'Montserrat', sans-serif;
		font-size: 24px;
		font-weight: 600;
		color: #0a2335;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content Wrapper */
	.dashboard__content {
		background: #f5f5f5;
		min-height: calc(100vh - 60px);
		padding: 30px;
	}

	.dashboard__content-main {
		max-width: 1200px;
		margin: 0 auto;
	}

	.dashboard__content-section {
		padding: 0;
	}

	/* Account Container - Semantic naming */
	.account-container {
		width: 100%;
		font-family: 'Montserrat', sans-serif;
		background-color: #ffffff;
	}

	.account-row {
		max-width: 1100px;
		margin: 0 auto;
	}

	.account-content {
		line-height: 1.6;
	}

	.account-text--no-margin {
		margin-bottom: 0 !important;
	}

	/* WooCommerce Content */
	.woocommerce {
		background: #fff;
		padding: 0;
		font-family: 'Montserrat', sans-serif;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.woocommerce-notices-wrapper {
		margin: 0;
	}

	/* Content Box - EXACT MATCH to dashboard.8f78208b.css */
	.content-box {
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.15);
	}

	.content-box--centered {
		max-width: 100%;
		margin: 0 auto;
	}

	.content-box__section {
		padding: 20px;
	}

	@media screen and (min-width: 1024px) {
		.content-box__section {
			padding: 40px;
		}
	}

	.woocommerce-MyAccount-content p {
		font-family: 'Montserrat', sans-serif;
		font-size: 15px;
		line-height: 1.7;
		color: #333333;
		margin: 0 0 15px 0;
	}

	.woocommerce-MyAccount-content p:first-of-type {
		margin-top: 0;
	}

	.woocommerce-MyAccount-content p:last-child {
		margin-bottom: 0;
	}


	.woocommerce-MyAccount-content a {
		color: #0984ae;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.woocommerce-MyAccount-content a:hover {
		color: #076787;
		text-decoration: underline;
	}

	.woocommerce-MyAccount-content strong {
		font-family: 'Montserrat', sans-serif;
		font-weight: 700;
		color: #0a2335;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 15px 20px;
		}

		.dashboard__page-title {
			font-size: 20px;
		}

		.woocommerce-MyAccount-content p {
			font-size: 14px;
		}
	}
</style>
