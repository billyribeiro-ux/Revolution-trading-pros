<script lang="ts">
	/**
	 * Dashboard - My Subscriptions Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/account/subscriptions
	 * Shows user's active and past subscriptions
	 *
	 * @version 1.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account/subscriptions', { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA (would come from API)
	// ═══════════════════════════════════════════════════════════════════════════

	type SubscriptionStatus = 'active' | 'pending-cancellation' | 'cancelled' | 'on-hold' | 'expired';

	interface Subscription {
		id: string;
		status: SubscriptionStatus;
		statusLabel: string;
		product: string;
		nextPayment: string | null;
		paymentMethod?: string;
		total: number;
		recurring: boolean;
		viewUrl: string;
	}

	// Sample subscriptions data matching screenshot
	const subscriptions: Subscription[] = [
		{
			id: '2176655',
			status: 'pending-cancellation',
			statusLabel: 'Pending Cancellation',
			product: 'Mastering the Trade Room (1 Month Trial)',
			nextPayment: null,
			total: 197.00,
			recurring: false,
			viewUrl: '/dashboard/account/view-subscription/2176655'
		},
		{
			id: '2173015',
			status: 'cancelled',
			statusLabel: 'Cancelled',
			product: 'Moxie Indicator™ Mastery Monthly (Trial)',
			nextPayment: null,
			total: 247.00,
			recurring: false,
			viewUrl: '/dashboard/account/view-subscription/2173015'
		},
		{
			id: '2132733',
			status: 'cancelled',
			statusLabel: 'Cancelled',
			product: 'Mastering the Trade Room (1 Month Trial)',
			nextPayment: null,
			total: 247.00,
			recurring: true,
			viewUrl: '/dashboard/account/view-subscription/2132733'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number): string {
		return `$${amount.toFixed(2)}`;
	}

	function getStatusClass(status: SubscriptionStatus): string {
		switch (status) {
			case 'active':
				return 'status--active';
			case 'pending-cancellation':
				return 'status--pending';
			case 'cancelled':
				return 'status--cancelled';
			case 'on-hold':
				return 'status--on-hold';
			case 'expired':
				return 'status--expired';
			default:
				return '';
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Subscriptions | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<h1 class="dashboard__page-title">My Account</h1>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Subscriptions Table -->
		<div class="subscriptions-table-wrapper">
			<table class="subscriptions-table">
				<thead>
					<tr>
						<th>Subscription</th>
						<th>Status</th>
						<th>Product</th>
						<th>Next Payment</th>
						<th>Total</th>
						<th></th>
					</tr>
				</thead>
				<tbody>
					{#each subscriptions as sub (sub.id)}
						<tr>
							<td class="col-subscription">
								<a href={sub.viewUrl} class="subscription-link">#{sub.id}</a>
							</td>
							<td class="col-status">
								<span class="status-badge {getStatusClass(sub.status)}">
									{sub.statusLabel.toUpperCase()}
								</span>
							</td>
							<td class="col-product">{sub.product}</td>
							<td class="col-next-payment">{sub.nextPayment || '-'}</td>
							<td class="col-total">
								{formatCurrency(sub.total)}{#if sub.recurring} / month{/if}
							</td>
							<td class="col-actions">
								<a href={sub.viewUrl} class="btn-view">View</a>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Empty State -->
		{#if subscriptions.length === 0}
			<div class="empty-state">
				<p>You have no active subscriptions.</p>
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBSCRIPTIONS TABLE - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.subscriptions-table-wrapper {
		border: 1px solid #e9ebed;
		border-radius: 4px;
		overflow: hidden;
	}

	.subscriptions-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.subscriptions-table thead {
		background: #f8f9fa;
	}

	.subscriptions-table th {
		padding: 12px 16px;
		text-align: left;
		font-weight: 600;
		color: #333;
		border-bottom: 1px solid #e9ebed;
	}

	.subscriptions-table td {
		padding: 16px;
		border-bottom: 1px solid #e9ebed;
		color: #333;
		vertical-align: middle;
	}

	.subscriptions-table tr:last-child td {
		border-bottom: none;
	}

	/* Column styles */
	.col-subscription {
		width: 15%;
	}

	.col-status {
		width: 18%;
	}

	.col-product {
		width: 30%;
	}

	.col-next-payment {
		width: 15%;
	}

	.col-total {
		width: 12%;
	}

	.col-actions {
		width: 10%;
		text-align: right;
	}

	/* Subscription link */
	.subscription-link {
		color: #1e73be;
		text-decoration: none;
		font-weight: 400;
	}

	.subscription-link:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATUS BADGES - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.status-badge {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 600;
		border-radius: 3px;
		text-transform: uppercase;
		letter-spacing: 0.3px;
		white-space: nowrap;
	}

	.status--active {
		background: #d4edda;
		color: #155724;
	}

	.status--pending {
		background: #d1ecf1;
		color: #0c5460;
	}

	.status--cancelled {
		background: #f8d7da;
		color: #721c24;
	}

	.status--on-hold {
		background: #fff3cd;
		color: #856404;
	}

	.status--expired {
		background: #e2e3e5;
		color: #383d41;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIEW BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn-view {
		display: inline-block;
		padding: 6px 14px;
		background: #fff;
		border: 1px solid #ddd;
		color: #333;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.btn-view:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		padding: 40px;
		text-align: center;
		color: #666;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1024px) {
		.subscriptions-table-wrapper {
			overflow-x: auto;
		}

		.subscriptions-table {
			min-width: 700px;
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

		.subscriptions-table th,
		.subscriptions-table td {
			padding: 12px;
		}
	}

	@media screen and (max-width: 480px) {
		.subscriptions-table {
			font-size: 13px;
		}

		.subscriptions-table th,
		.subscriptions-table td {
			padding: 10px 8px;
		}

		.status-badge {
			font-size: 10px;
			padding: 3px 6px;
		}
	}
</style>
