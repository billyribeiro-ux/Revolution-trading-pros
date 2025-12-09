<script lang="ts">
	/**
	 * Dashboard - My Orders Page
	 * Shows user's order history with status and details
	 * WordPress Revolution Trading Exact Match
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAuthenticated, authStore } from '$lib/stores/auth';
	import IconPackage from '@tabler/icons-svelte/icons/package';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconTruck from '@tabler/icons-svelte/icons/truck';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';

	// Redirect if not authenticated
	onMount(() => {
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/orders', { replaceState: true });
		}
	});

	// Order status types
	type OrderStatus = 'completed' | 'processing' | 'pending' | 'cancelled' | 'refunded';

	interface OrderItem {
		name: string;
		quantity: number;
		price: number;
	}

	interface Order {
		id: string;
		orderNumber: string;
		date: string;
		status: OrderStatus;
		total: number;
		items: OrderItem[];
	}

	// Sample orders data (would come from API)
	const orders: Order[] = [
		{
			id: 'ord_1',
			orderNumber: '#12847',
			date: 'December 5, 2025',
			status: 'completed',
			total: 497,
			items: [
				{ name: 'Mastering the Trade - Monthly', quantity: 1, price: 497 }
			]
		},
		{
			id: 'ord_2',
			orderNumber: '#12832',
			date: 'November 28, 2025',
			status: 'completed',
			total: 197,
			items: [
				{ name: 'Small Accounts Room - Monthly', quantity: 1, price: 197 }
			]
		},
		{
			id: 'ord_3',
			orderNumber: '#12801',
			date: 'November 15, 2025',
			status: 'completed',
			total: 97,
			items: [
				{ name: 'Explosive Swings Alerts - Monthly', quantity: 1, price: 97 }
			]
		},
		{
			id: 'ord_4',
			orderNumber: '#12756',
			date: 'October 20, 2025',
			status: 'refunded',
			total: 297,
			items: [
				{ name: 'Options Mastery Course', quantity: 1, price: 297 }
			]
		}
	];

	function getStatusInfo(status: OrderStatus) {
		const statusMap = {
			completed: { label: 'Completed', class: 'status--completed', icon: IconCheck },
			processing: { label: 'Processing', class: 'status--processing', icon: IconClock },
			pending: { label: 'Pending', class: 'status--pending', icon: IconClock },
			cancelled: { label: 'Cancelled', class: 'status--cancelled', icon: IconX },
			refunded: { label: 'Refunded', class: 'status--refunded', icon: IconX }
		};
		return statusMap[status];
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}
</script>

<svelte:head>
	<title>My Orders | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if $isAuthenticated}
	<!-- Dashboard Header -->
	<header class="dashboard__header">
		<div class="dashboard__header-content">
			<h1 class="dashboard__page-title">Orders</h1>
			<p class="dashboard__page-subtitle">View your order history and download invoices</p>
		</div>
	</header>

	<!-- Dashboard Content -->
	<div class="dashboard__content">
		<div class="dashboard__content-main">
			{#if orders.length > 0}
				<!-- Orders Table -->
				<div class="orders-table-wrapper">
					<table class="orders-table">
						<thead>
							<tr>
								<th>Order</th>
								<th>Date</th>
								<th>Status</th>
								<th>Total</th>
								<th>Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each orders as order (order.id)}
								{@const statusInfo = getStatusInfo(order.status)}
								<tr>
									<td class="order-number">
										<IconPackage size={18} class="order-icon" />
										<span>{order.orderNumber}</span>
									</td>
									<td class="order-date">{order.date}</td>
									<td>
										<span class="order-status {statusInfo.class}">
											<svelte:component this={statusInfo.icon} size={14} />
											{statusInfo.label}
										</span>
									</td>
									<td class="order-total">{formatCurrency(order.total)}</td>
									<td class="order-actions">
										<button class="action-btn" title="View Order">
											<IconEye size={18} />
										</button>
										{#if order.status === 'completed'}
											<button class="action-btn" title="Download Invoice">
												<IconDownload size={18} />
											</button>
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				<!-- Mobile Cards View -->
				<div class="orders-cards">
					{#each orders as order (order.id)}
						{@const statusInfo = getStatusInfo(order.status)}
						<div class="order-card">
							<div class="order-card__header">
								<div class="order-card__number">
									<IconPackage size={20} />
									<span>{order.orderNumber}</span>
								</div>
								<span class="order-status {statusInfo.class}">
									<svelte:component this={statusInfo.icon} size={14} />
									{statusInfo.label}
								</span>
							</div>
							<div class="order-card__body">
								<div class="order-card__items">
									{#each order.items as item}
										<p>{item.name}</p>
									{/each}
								</div>
								<div class="order-card__meta">
									<span class="order-card__date">{order.date}</span>
									<span class="order-card__total">{formatCurrency(order.total)}</span>
								</div>
							</div>
							<div class="order-card__actions">
								<button class="view-order-btn">
									View Order
									<IconChevronRight size={16} />
								</button>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<!-- Empty State -->
				<div class="empty-state">
					<IconPackage size={64} class="empty-icon" />
					<h3>No orders yet</h3>
					<p>When you make a purchase, your orders will appear here.</p>
					<a href="/live-trading-rooms" class="browse-btn">Browse Trading Rooms</a>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="loading-state">
		<p>Redirecting to login...</p>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		padding: 30px;
		border-bottom: 1px solid #ededed;
	}

	.dashboard__page-title {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 8px;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
	}

	.dashboard__page-subtitle {
		font-size: 15px;
		color: #666;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT
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
	   ORDERS TABLE (Desktop)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.orders-table-wrapper {
		overflow-x: auto;
		border: 1px solid #ededed;
		border-radius: 8px;
	}

	.orders-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.orders-table th {
		background: #f8f8f8;
		padding: 14px 16px;
		text-align: left;
		font-weight: 600;
		color: #333;
		border-bottom: 1px solid #ededed;
		white-space: nowrap;
	}

	.orders-table td {
		padding: 16px;
		border-bottom: 1px solid #ededed;
		color: #333;
	}

	.orders-table tr:last-child td {
		border-bottom: none;
	}

	.orders-table tr:hover {
		background: #fafafa;
	}

	.order-number {
		display: flex;
		align-items: center;
		gap: 8px;
		font-weight: 600;
		color: #1e73be;
	}

	:global(.order-icon) {
		color: #666;
	}

	.order-date {
		color: #666;
	}

	.order-total {
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ORDER STATUS BADGES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.order-status {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status--completed {
		background: #dcfce7;
		color: #166534;
	}

	.status--processing {
		background: #fef3c7;
		color: #92400e;
	}

	.status--pending {
		background: #e0e7ff;
		color: #3730a3;
	}

	.status--cancelled {
		background: #fee2e2;
		color: #991b1b;
	}

	.status--refunded {
		background: #f3f4f6;
		color: #4b5563;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ORDER ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.order-actions {
		display: flex;
		gap: 8px;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #f4f4f4;
		border: 1px solid #dbdbdb;
		border-radius: 6px;
		color: #666;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn:hover {
		background: #0984ae;
		border-color: #0984ae;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ORDERS CARDS (Mobile)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.orders-cards {
		display: none;
		flex-direction: column;
		gap: 16px;
	}

	.order-card {
		background: #fff;
		border: 1px solid #ededed;
		border-radius: 8px;
		overflow: hidden;
	}

	.order-card__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px;
		background: #f8f8f8;
		border-bottom: 1px solid #ededed;
	}

	.order-card__number {
		display: flex;
		align-items: center;
		gap: 8px;
		font-weight: 600;
		color: #1e73be;
	}

	.order-card__body {
		padding: 16px;
	}

	.order-card__items p {
		margin: 0 0 8px;
		color: #333;
		font-weight: 500;
	}

	.order-card__meta {
		display: flex;
		justify-content: space-between;
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid #ededed;
	}

	.order-card__date {
		color: #666;
		font-size: 13px;
	}

	.order-card__total {
		font-weight: 600;
		color: #333;
	}

	.order-card__actions {
		padding: 16px;
		border-top: 1px solid #ededed;
	}

	.view-order-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 12px;
		background: #0984ae;
		border: none;
		border-radius: 6px;
		color: #fff;
		font-weight: 600;
		font-size: 14px;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.view-order-btn:hover {
		background: #076787;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
	}

	:global(.empty-icon) {
		color: #ccc;
		margin-bottom: 20px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.empty-state p {
		color: #666;
		margin: 0 0 24px;
	}

	.browse-btn {
		display: inline-block;
		padding: 12px 24px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 6px;
		font-weight: 600;
		transition: background 0.15s ease;
	}

	.browse-btn:hover {
		background: #076787;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		color: #666;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 20px;
		}

		.dashboard__page-title {
			font-size: 26px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.orders-table-wrapper {
			display: none;
		}

		.orders-cards {
			display: flex;
		}
	}
</style>
