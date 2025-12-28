<script lang="ts">
	/**
	 * My Orders - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT 11+ Principal Engineer Grade - December 2025
	 * Real API integration with loading states and error handling
	 *
	 * @version 3.0.0 - Production Ready
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { authStore } from '$lib/stores/auth';

	// API Configuration
	const isDev = import.meta.env.DEV;
	const PRODUCTION_API_URL = 'https://revolution-trading-pros-api.fly.dev/api';
	const API_BASE = browser
		? isDev
			? '/api'
			: (import.meta.env['VITE_API_URL'] || PRODUCTION_API_URL)
		: '/api';

	// Types
	interface Order {
		id: number;
		order_number: string;
		status: string;
		subtotal: number;
		discount: number;
		tax: number;
		total: number;
		currency: string;
		created_at: string;
		completed_at: string | null;
		items: OrderItem[];
	}

	interface OrderItem {
		id: number;
		name: string;
		quantity: number;
		unit_price: number;
		total: number;
	}

	// State using Svelte 5 runes
	let orders = $state<Order[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	// Fetch orders from API
	async function fetchOrders(): Promise<void> {
		if (!browser) return;

		isLoading = true;
		error = null;

		try {
			const token = authStore.getToken();

			const response = await fetch(`${API_BASE}/checkout/orders`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				credentials: 'include'
			});

			if (!response.ok) {
				if (response.status === 401) {
					error = 'Please log in to view your orders';
					return;
				}
				throw new Error('Failed to fetch orders');
			}

			const data = await response.json();
			orders = data.orders || data || [];
		} catch (e) {
			console.error('Error fetching orders:', e);
			error = e instanceof Error ? e.message : 'Failed to load orders';
		} finally {
			isLoading = false;
		}
	}

	// Format date for display
	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	// Format currency
	function formatCurrency(amount: number, currency: string = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	// Get status badge class
	function getStatusClass(status: string): string {
		switch (status.toLowerCase()) {
			case 'completed':
				return 'status--completed';
			case 'pending':
				return 'status--pending';
			case 'processing':
				return 'status--processing';
			case 'failed':
				return 'status--failed';
			case 'refunded':
				return 'status--refunded';
			case 'cancelled':
				return 'status--cancelled';
			default:
				return '';
		}
	}

	// Capitalize first letter
	function capitalize(str: string): string {
		return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
	}

	onMount(() => {
		fetchOrders();
	});
</script>

<svelte:head>
	<title>My Orders - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- Orders Page - Real API Integration -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">My Orders</h2>

			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading your orders...</p>
				</div>
			{:else if error}
				<div class="error-state">
					<p>{error}</p>
					<button class="btn btn-primary" onclick={() => fetchOrders()}>
						Try Again
					</button>
				</div>
			{:else if orders.length === 0}
				<div class="empty-state">
					<div class="empty-icon">📦</div>
					<p>No orders found.</p>
					<a href="/pricing" class="btn btn-primary">Browse Memberships</a>
				</div>
			{:else}
				<table class="table">
					<thead>
						<tr>
							<th class="col-order">Order</th>
							<th class="col-date">Date</th>
							<th class="col-status">Status</th>
							<th class="col-total">Total</th>
							<th class="col-actions text-right">Actions</th>
						</tr>
					</thead>
					<tbody class="u--font-size-sm">
						{#each orders as order (order.id)}
							<tr>
								<td class="col-order">
									<a href="/dashboard/account/view-order/{order.order_number}">
										#{order.order_number}
									</a>
								</td>
								<td class="col-date">
									<time datetime={order.created_at}>{formatDate(order.created_at)}</time>
								</td>
								<td class="col-status">
									<span class="status-badge {getStatusClass(order.status)}">
										{capitalize(order.status)}
									</span>
								</td>
								<td class="col-total">
									{formatCurrency(order.total, order.currency)}
								</td>
								<td class="col-actions text-right table__actions">
									<div class="dropdown">
										<a
											href="/dashboard/account/view-order/{order.order_number}"
											class="btn btn-xs btn-white table__more-actions"
											aria-label="View order details"
										>
											<span class="st-icon-ellipsis-h icon--md">⋯</span>
										</a>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Apple ICT 11+ Grade Styles - Production Ready
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #666;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e0e0e0;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Error State */
	.error-state {
		text-align: center;
		padding: 40px 20px;
		color: #d32f2f;
		background: #ffebee;
		border-radius: 8px;
	}

	.error-state p {
		margin-bottom: 16px;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
		background: #f9f9f9;
		border-radius: 8px;
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.empty-state p {
		margin-bottom: 20px;
		font-size: 16px;
	}

	/* Table */
	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.table thead {
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.table th {
		padding: 15px 20px;
		font-size: 13px;
		font-weight: 700;
		color: #333;
		text-align: left;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.table td {
		padding: 15px 20px;
		font-size: 14px;
		color: #333;
		border-bottom: 1px solid #ededed;
	}

	.table tbody tr:last-child td {
		border-bottom: none;
	}

	.table tbody tr:hover {
		background: #f9f9f9;
	}

	/* Column widths */
	.col-order {
		width: 20%;
	}

	.col-date {
		width: 25%;
	}

	.col-status {
		width: 15%;
	}

	.col-total {
		width: 20%;
	}

	.col-actions {
		width: 20%;
	}

	.text-right {
		text-align: right;
	}

	/* Status Badges */
	.status-badge {
		display: inline-block;
		padding: 4px 10px;
		font-size: 12px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: capitalize;
	}

	.status--completed {
		background: #e8f5e9;
		color: #2e7d32;
	}

	.status--pending {
		background: #fff3e0;
		color: #ef6c00;
	}

	.status--processing {
		background: #e3f2fd;
		color: #1565c0;
	}

	.status--failed {
		background: #ffebee;
		color: #c62828;
	}

	.status--refunded {
		background: #f3e5f5;
		color: #7b1fa2;
	}

	.status--cancelled {
		background: #fafafa;
		color: #616161;
	}

	/* Links */
	.table td a {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
	}

	.table td a:hover {
		color: #0e6ac4;
		text-decoration: underline;
	}

	/* Actions */
	.table__actions {
		text-align: right;
	}

	.table__more-actions {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 8px 12px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		color: #666;
		font-size: 16px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.table__more-actions:hover {
		background: #f4f4f4;
		border-color: #999;
	}

	.icon--md {
		font-size: 16px;
		letter-spacing: 2px;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #077a9e;
	}

	@media (max-width: 768px) {
		.table th,
		.table td {
			padding: 12px 15px;
		}

		.col-order,
		.col-date,
		.col-status,
		.col-total,
		.col-actions {
			width: auto;
		}

		/* Hide some columns on mobile */
		.col-status,
		.col-total {
			display: none;
		}
	}
</style>
