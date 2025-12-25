<script lang="ts">
	/**
	 * Membership My Account Page - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Exact replica of Simpler Trading mbmyaccount page
	 * @version 1.0.0 - Pixel Perfect
	 */

	// User data
	const user = {
		name: 'John Smith',
		email: 'john.smith@example.com',
		memberSince: 'January 15, 2024',
		avatar: 'https://www.gravatar.com/avatar/00000000000000000000000000000000?d=mp&f=y'
	};

	// Active memberships
	const memberships = [
		{ id: 1, name: 'Day Trading Room', status: 'Active', renewDate: 'January 3, 2026', price: '$197/month' },
		{ id: 2, name: 'Moxie Indicator Mastery', status: 'Active', renewDate: 'January 15, 2026', price: '$247/month' }
	];

	// Account navigation
	const accountNav = [
		{ label: 'Dashboard', href: '/dashboard/account', icon: 'dashboard' },
		{ label: 'My Orders', href: '/dashboard/account/orders', icon: 'orders' },
		{ label: 'My Subscriptions', href: '/dashboard/account/subscriptions', icon: 'subscriptions' },
		{ label: 'Coupons', href: '/dashboard/account/coupons', icon: 'coupons' },
		{ label: 'Billing Address', href: '/dashboard/account/billing-address', icon: 'address' },
		{ label: 'Payment Methods', href: '/dashboard/account/payment-methods', icon: 'payment' },
		{ label: 'Account Details', href: '/dashboard/account/edit-account', icon: 'account' },
		{ label: 'Log out', href: '/logout', icon: 'logout' }
	];
</script>

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
	<meta name="description" content="Manage your Revolution Trading Pros membership and account settings." />
</svelte:head>

<!-- WordPress EXACT Structure -->
<div class="page-template-default page page-id-31 logged-in woocommerce-account woocommerce-page">

	<!-- Breadcrumbs -->
	<nav id="breadcrumbs" class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home"><a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a></li>
				<li class="separator separator-home"> / </li>
				<li class="item-current"><strong class="breadcrumb-current">My Account</strong></li>
			</ul>
		</div>
	</nav>

	<!-- Page Content -->
	<div id="page" class="hfeed site grid-parent">
		<div id="content" class="site-content">

			<!-- Page Header -->
			<header class="page-header-section">
				<div class="container-fluid">
					<h1 class="page-title">My Account</h1>
				</div>
			</header>

			<!-- Account Content -->
			<section class="wl-page-content">
				<div class="container-fluid">
					<div class="woocommerce">
						<div class="woocommerce-MyAccount-content">
							<div class="row">
								<!-- Sidebar Navigation -->
								<div class="col-xs-12 col-md-3">
									<div class="account-sidebar">
										<!-- User Card -->
										<div class="user-card">
											<div class="user-avatar">
												<img src={user.avatar} alt={user.name} />
											</div>
											<div class="user-info">
												<h4 class="user-name">{user.name}</h4>
												<p class="user-email">{user.email}</p>
												<p class="user-member-since">Member since {user.memberSince}</p>
											</div>
										</div>

										<!-- Navigation -->
										<nav class="account-nav">
											<ul class="woocommerce-MyAccount-navigation">
												{#each accountNav as item}
													<li class="woocommerce-MyAccount-navigation-link">
														<a href={item.href}>
															<span class="nav-icon nav-icon-{item.icon}"></span>
															{item.label}
														</a>
													</li>
												{/each}
											</ul>
										</nav>
									</div>
								</div>

								<!-- Main Content -->
								<div class="col-xs-12 col-md-9">
									<div class="account-content">
										<!-- Welcome Section -->
										<div class="welcome-section">
											<p>Hello <strong>{user.name}</strong> (not {user.name}? <a href="/logout">Log out</a>)</p>
											<p>From your account dashboard you can view your <a href="/dashboard/account/orders">recent orders</a>, manage your <a href="/dashboard/account/billing-address">shipping and billing addresses</a>, and <a href="/dashboard/account/edit-account">edit your password and account details</a>.</p>
										</div>

										<!-- Active Memberships -->
										<div class="memberships-section">
											<h3 class="section-title">Active Memberships</h3>
											<div class="memberships-grid">
												{#each memberships as membership}
													<div class="membership-card">
														<div class="membership-header">
															<span class="status-badge status-active">{membership.status}</span>
														</div>
														<h4 class="membership-name">{membership.name}</h4>
														<div class="membership-details">
															<div class="detail-row">
																<span class="detail-label">Price:</span>
																<span class="detail-value">{membership.price}</span>
															</div>
															<div class="detail-row">
																<span class="detail-label">Next Renewal:</span>
																<span class="detail-value">{membership.renewDate}</span>
															</div>
														</div>
														<div class="membership-actions">
															<a href="/dashboard/account/view-subscription/{membership.id}" class="btn btn-sm btn-default">Manage</a>
														</div>
													</div>
												{/each}
											</div>
										</div>

										<!-- Quick Actions -->
										<div class="quick-actions-section">
											<h3 class="section-title">Quick Actions</h3>
											<div class="quick-actions-grid">
												<a href="/dashboard" class="quick-action-card">
													<span class="action-icon">
														<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg>
													</span>
													<span class="action-label">Go to Dashboard</span>
												</a>
												<a href="/dashboard/account/subscriptions" class="quick-action-card">
													<span class="action-icon">
														<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M21 18v1c0 1.1-.9 2-2 2H5c-1.11 0-2-.9-2-2V5c0-1.1.89-2 2-2h14c1.1 0 2 .9 2 2v1h-9c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h9zm-9-2h10V8H12v8zm4-2.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"/></svg>
													</span>
													<span class="action-label">View Subscriptions</span>
												</a>
												<a href="/dashboard/account/orders" class="quick-action-card">
													<span class="action-icon">
														<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-2 10H7v-2h10v2zm0-4H7V7h10v2z"/></svg>
													</span>
													<span class="action-label">View Orders</span>
												</a>
												<a href="/dashboard/account/edit-account" class="quick-action-card">
													<span class="action-icon">
														<svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>
													</span>
													<span class="action-label">Edit Account</span>
												</a>
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
	   100% PIXEL-PERFECT STYLES - Matching Simpler Trading
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

	.breadcrumbs a:hover {
		text-decoration: underline;
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
	.page-header-section {
		padding: 40px 0 30px;
	}

	.page-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0;
		text-transform: uppercase;
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
		.col-md-3 {
			flex: 0 0 25%;
			max-width: 25%;
		}

		.col-md-9 {
			flex: 0 0 75%;
			max-width: 75%;
		}
	}

	/* Account Sidebar */
	.account-sidebar {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		overflow: hidden;
	}

	.user-card {
		padding: 24px;
		text-align: center;
		border-bottom: 1px solid #ededed;
	}

	.user-avatar {
		width: 80px;
		height: 80px;
		margin: 0 auto 16px;
		border-radius: 50%;
		overflow: hidden;
		border: 3px solid #0984ae;
	}

	.user-avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.user-name {
		font-size: 18px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 4px;
	}

	.user-email {
		font-size: 13px;
		color: #666;
		margin: 0 0 4px;
	}

	.user-member-since {
		font-size: 12px;
		color: #999;
		margin: 0;
	}

	/* Account Navigation */
	.account-nav {
		padding: 16px 0;
	}

	.woocommerce-MyAccount-navigation {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.woocommerce-MyAccount-navigation-link {
		margin: 0;
	}

	.woocommerce-MyAccount-navigation-link a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 24px;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: all 0.15s ease;
	}

	.woocommerce-MyAccount-navigation-link a:hover {
		background: #f4f4f4;
		color: #0984ae;
	}

	.nav-icon {
		width: 20px;
		height: 20px;
		background: #999;
		border-radius: 4px;
	}

	/* Account Content */
	.account-content {
		padding-left: 20px;
	}

	/* Welcome Section */
	.welcome-section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		margin-bottom: 24px;
	}

	.welcome-section p {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 12px;
	}

	.welcome-section p:last-child {
		margin-bottom: 0;
	}

	.welcome-section a {
		color: #0984ae;
		text-decoration: none;
	}

	.welcome-section a:hover {
		text-decoration: underline;
	}

	/* Section Title */
	.section-title {
		font-size: 18px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 20px;
		padding-bottom: 12px;
		border-bottom: 2px solid #0984ae;
	}

	/* Memberships Section */
	.memberships-section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		margin-bottom: 24px;
	}

	.memberships-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}

	.membership-card {
		background: #f9f9f9;
		border-radius: 8px;
		padding: 20px;
		border: 1px solid #ededed;
	}

	.membership-header {
		margin-bottom: 12px;
	}

	.status-badge {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 25px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
	}

	.status-active {
		background: #d4edda;
		color: #155724;
	}

	.membership-name {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 16px;
	}

	.membership-details {
		margin-bottom: 16px;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
		font-size: 13px;
	}

	.detail-label {
		color: #666;
	}

	.detail-value {
		color: #1a1a1a;
		font-weight: 600;
	}

	.membership-actions {
		display: flex;
		gap: 8px;
	}

	/* Quick Actions */
	.quick-actions-section {
		background: #fff;
		border-radius: 8px;
		padding: 24px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.quick-actions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 16px;
	}

	.quick-action-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		padding: 24px;
		background: #f9f9f9;
		border-radius: 8px;
		border: 1px solid #ededed;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.quick-action-card:hover {
		background: #f4f4f4;
		border-color: #0984ae;
		transform: translateY(-2px);
	}

	.action-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: #0984ae;
		border-radius: 12px;
		color: #fff;
	}

	.action-label {
		font-size: 14px;
		font-weight: 600;
		color: #1a1a1a;
		text-align: center;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 5px;
		border: none;
		cursor: pointer;
		transition: all 0.15s ease;
		text-decoration: none;
	}

	.btn-sm {
		padding: 8px 16px;
		font-size: 13px;
	}

	.btn-default {
		background: #f4f4f4;
		color: #1a1a1a;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
		border-color: #999;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.account-content {
			padding-left: 0;
			margin-top: 24px;
		}

		.page-title {
			font-size: 28px;
		}
	}
</style>
