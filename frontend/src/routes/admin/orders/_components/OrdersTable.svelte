<script lang="ts">
	import {
		IconShoppingCart,
		IconChevronLeft,
		IconChevronRight,
		IconExternalLink
	} from '$lib/icons';

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

	interface Pagination {
		page: number;
		per_page: number;
		total: number;
		total_pages: number;
	}

	interface Props {
		orders: Order[];
		loading: boolean;
		pagination: Pagination | null;
		formatCurrency: (amount: number, currency?: string) => string;
		formatDate: (dateString: string) => string;
		getStatusColor: (status: string) => string;
		onpage: (page: number) => void;
		onopenDetail: (order: Order) => void;
	}

	let {
		orders,
		loading,
		pagination,
		formatCurrency,
		formatDate,
		getStatusColor,
		onpage,
		onopenDetail
	}: Props = $props();
</script>

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
				{#each orders as order (order.id)}
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
							<button class="action-btn" onclick={() => onopenDetail(order)}>
								<IconExternalLink size={16} />
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>

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
						onclick={() => pagination && onpage(pagination.page - 1)}
					>
						<IconChevronLeft size={18} />
					</button>
					<span class="page-indicator">
						Page {pagination.page} of {pagination.total_pages}
					</span>
					<button
						class="page-btn"
						disabled={pagination.page === pagination.total_pages}
						onclick={() => pagination && onpage(pagination.page + 1)}
					>
						<IconChevronRight size={18} />
					</button>
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
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

	@media (max-width: 767.98px) {
		.orders-table-container {
			overflow-x: auto;
		}
		.orders-table {
			min-width: 700px;
		}
	}
</style>
