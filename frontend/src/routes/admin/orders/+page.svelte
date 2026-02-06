<script lang="ts">
	/**
	 * Admin Orders Management Page
	 * ICT 7 Fix: Complete admin orders dashboard
	 * Apple Principal Engineer Grade - February 2026
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import {
		IconShoppingCart,
		IconCurrencyDollar,
		IconCheck,
		IconClock,
		IconRefresh,
		IconSearch,
		IconFilter,
		IconDownload,
		IconChevronLeft,
		IconChevronRight,
		IconExternalLink,
		IconX,
		IconAlertTriangle
	} from '$lib/icons';
	import { toastStore } from '$lib/stores/toast.svelte';

	// Types
	interface Order {
		id: number;
		order_number: string;
		status: string;
		total: number;
		currency: string;
		user_email: string;
		user_name: string | null;
		payment_provider: string | null;
		item_count: number;
		created_at: string;
		completed_at: string | null;
	}

	interface OrderStats {
		total_orders: number;
		completed_orders: number;
		pending_orders: number;
		refunded_orders: number;
		total_revenue: number;
		revenue_this_month: number;
		average_order_value: number;
	}

	interface Pagination {
		page: number;
		per_page: number;
		total: number;
		total_pages: number;
	}

	// State
	let orders = $state<Order[]>([]);
	let stats = $state<OrderStats | null>(null);
	let pagination = $state<Pagination | null>(null);
	let loading = $state(true);
	let searchQuery = $state('');
	let statusFilter = $state('');
	let showFilters = $state(false);
	let selectedOrder = $state<Order | null>(null);
	let showDetailModal = $state(false);
	let orderDetail = $state<any>(null);
	let loadingDetail = $state(false);
	let error = $state('');

	// Fetch orders from API
	async function loadOrders(page = 1) {
		loading = true;
		error = '';
		try {
			const params = new URLSearchParams();
			params.set('page', page.toString());
			params.set('per_page', '25');
			if (statusFilter) params.set('status', statusFilter);
			if (searchQuery) params.set('search', searchQuery);

			const response = await fetch(`/api/admin/orders?${params}`);
			if (!response.ok) {
				throw new Error('Failed to load orders');
			}
			const data = await response.json();
			orders = data.data || [];
			stats = data.stats || null;
			pagination = data.pagination || null;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load orders';
			console.error('Error loading orders:', err);
		} finally {
			loading = false;
		}
	}

	// Fetch order details
	async function loadOrderDetail(orderId: number) {
		loadingDetail = true;
		try {
			const response = await fetch(`/api/admin/orders/${orderId}`);
			if (!response.ok) throw new Error('Failed to load order details');
			const data = await response.json();
			orderDetail = data.data;
		} catch (err) {
			toastStore.error('Failed to load order details');
		} finally {
			loadingDetail = false;
		}
	}

	// Handle search
	function handleSearch() {
		loadOrders(1);
	}

	// Handle filter change
	function handleStatusFilter(status: string) {
		statusFilter = status;
		loadOrders(1);
	}

	// Open order detail
	function openOrderDetail(order: Order) {
		selectedOrder = order;
		showDetailModal = true;
		loadOrderDetail(order.id);
	}

	// Format currency
	function formatCurrency(amount: number, currency = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency
		}).format(amount);
	}

	// Format date
	function formatDate(dateString: string): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// Get status color
	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed':
				return 'status-completed';
			case 'pending':
				return 'status-pending';
			case 'refunded':
			case 'partial_refund':
				return 'status-refunded';
			case 'failed':
				return 'status-failed';
			default:
				return 'status-default';
		}
	}

	// Export orders
	async function handleExport() {
		try {
			const params = new URLSearchParams();
			if (statusFilter) params.set('status', statusFilter);
			if (searchQuery) params.set('search', searchQuery);
			params.set('format', 'csv');

			const response = await fetch(`/api/admin/orders/export?${params}`);
			if (!response.ok) throw new Error('Export failed');

			const blob = await response.blob();
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `orders-export-${new Date().toISOString().split('T')[0]}.csv`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
			toastStore.success('Orders exported successfully');
		} catch {
			toastStore.error('Failed to export orders');
		}
	}

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (browser) loadOrders();
	});
</script>

<svelte:head>
	<title>Orders Management | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-orders">
	<!-- Background Effects -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
	</div>

	<div class="admin-page-container">
		<!-- Error Banner -->
		{#if error}
			<div class="error-banner">
				<IconAlertTriangle size={20} />
				<span>{error}</span>
				<button onclick={() => loadOrders()}>Retry</button>
			</div>
		{/if}

		<!-- Header -->
		<header class="page-header">
			<h1>Orders Management</h1>
			<p class="subtitle">View and manage all customer transactions</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={() => loadOrders()}>
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-secondary" onclick={handleExport}>
					<IconDownload size={18} />
					Export
				</button>
			</div>
		</header>

		<!-- Stats Grid -->
		{#if stats}
			<div class="stats-grid">
				<div class="stat-card gradient-purple">
					<div class="stat-icon">
						<IconShoppingCart size={28} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Total Orders</div>
						<div class="stat-value">{stats.total_orders.toLocaleString()}</div>
					</div>
				</div>

				<div class="stat-card gradient-emerald">
					<div class="stat-icon">
						<IconCheck size={28} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Completed</div>
						<div class="stat-value">{stats.completed_orders.toLocaleString()}</div>
					</div>
				</div>

				<div class="stat-card gradient-gold">
					<div class="stat-icon">
						<IconCurrencyDollar size={28} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Total Revenue</div>
						<div class="stat-value">{formatCurrency(stats.total_revenue)}</div>
						<div class="stat-change neutral">
							{formatCurrency(stats.revenue_this_month)} this month
						</div>
					</div>
				</div>

				<div class="stat-card gradient-blue">
					<div class="stat-icon">
						<IconClock size={28} />
					</div>
					<div class="stat-content">
						<div class="stat-label">Avg Order Value</div>
						<div class="stat-value">{formatCurrency(stats.average_order_value)}</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Toolbar -->
		<div class="toolbar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					type="text"
					placeholder="Search by order #, email, or name..."
					bind:value={searchQuery}
					onkeydown={(e) => e.key === 'Enter' && handleSearch()}
				/>
			</div>

			<div class="toolbar-actions">
				<button
					class="filter-toggle"
					class:active={showFilters}
					onclick={() => (showFilters = !showFilters)}
				>
					<IconFilter size={18} />
					Filters
				</button>
			</div>
		</div>

		<!-- Filters Panel -->
		{#if showFilters}
			<div class="filters-panel">
				<div class="filter-group">
					<label for="status-filter">Status</label>
					<select
						id="status-filter"
						bind:value={statusFilter}
						onchange={() => handleStatusFilter(statusFilter)}
					>
						<option value="">All Statuses</option>
						<option value="pending">Pending</option>
						<option value="completed">Completed</option>
						<option value="refunded">Refunded</option>
						<option value="partial_refund">Partial Refund</option>
						<option value="failed">Failed</option>
					</select>
				</div>

				<button
					class="clear-filters"
					onclick={() => {
						statusFilter = '';
						searchQuery = '';
						loadOrders(1);
					}}
				>
					Clear All
				</button>
			</div>
		{/if}

		<!-- Orders Table -->
		<div class="orders-table-container">
			{#if loading}
				<div class="loading-state">
					<div class="loader"></div>
					<p>Loading orders...</p>
				</div>
			{:else if orders.length === 0}
				<div class="empty-state">
					<IconShoppingCart size={64} stroke={1} />
					<h3>No orders found</h3>
					<p>Try adjusting your filters or search query</p>
				</div>
			{:else}
				<table class="orders-table">
					<thead>
						<tr>
							<th>Order #</th>
							<th>Customer</th>
							<th>Status</th>
							<th>Items</th>
							<th>Total</th>
							<th>Payment</th>
							<th>Date</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each orders as order}
							<tr>
								<td>
									<span class="order-number">{order.order_number}</span>
								</td>
								<td>
									<div class="customer-info">
										<div class="customer-name">
											{order.user_name || 'Guest'}
										</div>
										<div class="customer-email">{order.user_email}</div>
									</div>
								</td>
								<td>
									<span class="status-badge {getStatusColor(order.status)}">
										{order.status.replace('_', ' ')}
									</span>
								</td>
								<td>
									<span class="item-count"
										>{order.item_count} item{order.item_count !== 1 ? 's' : ''}</span
									>
								</td>
								<td>
									<span class="order-total">{formatCurrency(order.total, order.currency)}</span>
								</td>
								<td>
									<span class="payment-provider">{order.payment_provider || 'N/A'}</span>
								</td>
								<td>
									<span class="order-date">{formatDate(order.created_at)}</span>
								</td>
								<td>
									<button class="action-btn" onclick={() => openOrderDetail(order)}>
										<IconExternalLink size={16} />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				<!-- Pagination -->
				{#if pagination}
					<div class="pagination">
						<div class="pagination-info">
							Showing {(pagination.page - 1) * pagination.per_page + 1} to {Math.min(
								pagination.page * pagination.per_page,
								pagination.total
							)} of {pagination.total} orders
						</div>
						<div class="pagination-controls">
							<button
								class="page-btn"
								disabled={pagination.page === 1}
								onclick={() => loadOrders(pagination!.page - 1)}
							>
								<IconChevronLeft size={18} />
							</button>
							<span class="page-indicator">
								Page {pagination.page} of {pagination.total_pages}
							</span>
							<button
								class="page-btn"
								disabled={pagination.page === pagination.total_pages}
								onclick={() => loadOrders(pagination!.page + 1)}
							>
								<IconChevronRight size={18} />
							</button>
						</div>
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>

<!-- Order Detail Modal -->
{#if showDetailModal && selectedOrder}
	<div
		class="modal-overlay"
		onclick={() => (showDetailModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showDetailModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal-content"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Order #{selectedOrder.order_number}</h2>
				<button class="close-btn" onclick={() => (showDetailModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if loadingDetail}
					<div class="loading-state">
						<div class="loader"></div>
						<p>Loading order details...</p>
					</div>
				{:else if orderDetail}
					<div class="order-detail-grid">
						<div class="detail-section">
							<h4>Order Information</h4>
							<div class="detail-row">
								<span class="label">Status:</span>
								<span class="status-badge {getStatusColor(orderDetail.status)}">
									{orderDetail.status}
								</span>
							</div>
							<div class="detail-row">
								<span class="label">Total:</span>
								<span class="value">{formatCurrency(orderDetail.total, orderDetail.currency)}</span>
							</div>
							<div class="detail-row">
								<span class="label">Subtotal:</span>
								<span class="value">{formatCurrency(orderDetail.subtotal)}</span>
							</div>
							{#if orderDetail.discount > 0}
								<div class="detail-row">
									<span class="label">Discount:</span>
									<span class="value discount">-{formatCurrency(orderDetail.discount)}</span>
								</div>
							{/if}
							<div class="detail-row">
								<span class="label">Created:</span>
								<span class="value">{formatDate(orderDetail.created_at)}</span>
							</div>
							{#if orderDetail.completed_at}
								<div class="detail-row">
									<span class="label">Completed:</span>
									<span class="value">{formatDate(orderDetail.completed_at)}</span>
								</div>
							{/if}
						</div>

						<div class="detail-section">
							<h4>Customer</h4>
							<div class="detail-row">
								<span class="label">Name:</span>
								<span class="value">{orderDetail.billing_name || 'N/A'}</span>
							</div>
							<div class="detail-row">
								<span class="label">Email:</span>
								<span class="value">{orderDetail.billing_email || 'N/A'}</span>
							</div>
							{#if orderDetail.coupon_code}
								<div class="detail-row">
									<span class="label">Coupon:</span>
									<span class="value coupon">{orderDetail.coupon_code}</span>
								</div>
							{/if}
						</div>
					</div>

					<div class="order-items-section">
						<h4>Order Items</h4>
						<table class="items-table">
							<thead>
								<tr>
									<th>Item</th>
									<th>Qty</th>
									<th>Price</th>
									<th>Total</th>
								</tr>
							</thead>
							<tbody>
								{#each orderDetail.items as item}
									<tr>
										<td>{item.name}</td>
										<td>{item.quantity}</td>
										<td>{formatCurrency(item.unit_price)}</td>
										<td>{formatCurrency(item.total)}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showDetailModal = false)}>Close</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Admin Orders Page Styles - ICT 7 */
	.admin-orders {
		min-height: calc(100vh - 70px - 4rem);
		background: var(--admin-bg, linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%));
		color: white;
		position: relative;
		overflow: hidden;
		margin: -2rem;
		padding: 2rem;
	}

	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-2 {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, var(--primary-600), #1e293b);
		animation: float 25s ease-in-out infinite reverse;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	/* Header */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 800;
		color: var(--admin-text-primary, #f8fafc);
		margin: 0 0 0.5rem;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(22, 27, 34, 0.8);
		border-radius: 12px;
		padding: 1.5rem;
		position: relative;
		overflow: hidden;
		border: 1px solid var(--border-muted);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		backdrop-filter: blur(10px);
	}

	.stat-card.gradient-purple {
		border-color: rgba(230, 184, 0, 0.3);
	}
	.stat-card.gradient-emerald {
		border-color: rgba(16, 185, 129, 0.3);
	}
	.stat-card.gradient-gold {
		border-color: rgba(251, 191, 36, 0.3);
	}
	.stat-card.gradient-blue {
		border-color: rgba(59, 130, 246, 0.3);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.gradient-purple .stat-icon {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.gradient-emerald .stat-icon {
		background: rgba(16, 185, 129, 0.15);
		color: var(--success-emphasis);
	}
	.gradient-gold .stat-icon {
		background: rgba(251, 191, 36, 0.15);
		color: var(--warning-emphasis);
	}
	.gradient-blue .stat-icon {
		background: rgba(59, 130, 246, 0.15);
		color: #3b82f6;
	}

	.stat-label {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 800;
		color: var(--admin-text-primary);
	}

	.stat-change.neutral {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.search-box {
		flex: 1;
		max-width: 400px;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		color: var(--text-secondary);
		backdrop-filter: blur(10px);
	}

	.search-box:focus-within {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: var(--text-primary);
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary);
	}

	.toolbar-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Filters */
	.filter-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		color: var(--text-primary);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-toggle:hover,
	.filter-toggle.active {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.filters-panel {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		margin-bottom: 1rem;
		backdrop-filter: blur(10px);
	}

	.filter-group {
		flex: 1;
	}

	.filter-group label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.filter-group select {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(22, 27, 34, 0.95);
		border: 1px solid var(--border-default);
		border-radius: 12px;
		color: var(--text-primary);
		font-size: 0.875rem;
		cursor: pointer;
	}

	.clear-filters {
		padding: 0.625rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		color: var(--text-primary);
		font-weight: 500;
		cursor: pointer;
		align-self: flex-end;
	}

	/* Table */
	.orders-table-container {
		background: rgba(22, 27, 34, 0.85);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		overflow: hidden;
		backdrop-filter: blur(10px);
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-tertiary);
	}

	.loader {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.orders-table {
		width: 100%;
		border-collapse: collapse;
	}

	.orders-table thead {
		background: rgba(13, 17, 23, 0.6);
	}

	.orders-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.orders-table tbody tr {
		border-top: 1px solid var(--border-muted);
		transition: background 0.2s;
	}

	.orders-table tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	.orders-table td {
		padding: 1rem 1.5rem;
		color: var(--text-primary);
	}

	.order-number {
		font-weight: 600;
		color: var(--primary-400);
	}

	.customer-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}
	.customer-name {
		font-weight: 600;
	}
	.customer-email {
		font-size: 0.8125rem;
		color: var(--text-tertiary);
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.status-completed {
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}
	.status-badge.status-pending {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.status-badge.status-refunded {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}
	.status-badge.status-failed {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}
	.status-badge.status-default {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.order-total {
		font-weight: 700;
		color: var(--primary-400);
	}
	.payment-provider {
		color: var(--text-tertiary);
		font-size: 0.875rem;
	}
	.order-date {
		font-size: 0.875rem;
		color: var(--text-tertiary);
	}

	.action-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid var(--border-default);
		border-radius: 6px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--border-muted);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: var(--text-tertiary);
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.page-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid var(--border-default);
		border-radius: 12px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	/* Buttons */
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: 1px solid rgba(100, 116, 139, 0.3);
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 12px;
		color: var(--error-emphasis);
		margin-bottom: 1.5rem;
	}

	.error-banner span {
		flex: 1;
		font-weight: 500;
	}

	.error-banner button {
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: var(--error-emphasis);
		font-weight: 600;
		cursor: pointer;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: linear-gradient(145deg, var(--bg-elevated) 0%, var(--bg-base) 100%);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 700px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.5);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}
	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.3);
	}

	/* Order Detail */
	.order-detail-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.detail-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 1rem;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--border-muted);
	}

	.detail-row .label {
		color: var(--text-tertiary);
	}
	.detail-row .value {
		font-weight: 500;
		color: var(--text-primary);
	}
	.detail-row .value.discount {
		color: #10b981;
	}
	.detail-row .value.coupon {
		color: var(--primary-500);
		font-family: monospace;
	}

	.order-items-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 1rem;
	}

	.items-table {
		width: 100%;
		border-collapse: collapse;
	}

	.items-table th,
	.items-table td {
		padding: 0.75rem;
		text-align: left;
		border-bottom: 1px solid var(--border-muted);
	}

	.items-table th {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-tertiary);
		text-transform: uppercase;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.admin-page-container {
			padding: 1rem;
		}
		.stats-grid {
			grid-template-columns: 1fr;
		}
		.toolbar {
			flex-direction: column;
		}
		.search-box {
			max-width: 100%;
		}
		.orders-table-container {
			overflow-x: auto;
		}
		.orders-table {
			min-width: 700px;
		}
		.order-detail-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
