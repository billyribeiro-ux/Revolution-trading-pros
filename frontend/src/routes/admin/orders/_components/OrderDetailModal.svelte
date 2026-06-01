<script lang="ts">
	import { IconX } from '$lib/icons';

	interface Order {
		id: number;
		order_number: string;
		status: string;
	}

	// Shape derived from the template reads. Required fields reflect what the
	// template unconditionally accesses (and what the backend always returns
	// on a successful detail fetch); optional fields are template-gated with
	// {#if orderDetail.field}.
	interface OrderItem {
		name: string;
		quantity: number;
		unit_price: number;
		total: number;
	}
	interface OrderDetail {
		status: string;
		total: number;
		currency?: string;
		subtotal: number;
		discount: number;
		created_at: string;
		completed_at?: string;
		billing_name?: string;
		billing_email?: string;
		coupon_code?: string;
		items?: OrderItem[];
	}

	interface Props {
		selectedOrder: Order;
		orderDetail: OrderDetail | null;
		loadingDetail: boolean;
		formatCurrency: (amount: number, currency?: string) => string;
		formatDate: (dateString: string) => string;
		getStatusColor: (status: string) => string;
		onclose: () => void;
	}

	let {
		selectedOrder,
		orderDetail,
		loadingDetail,
		formatCurrency,
		formatDate,
		getStatusColor,
		onclose
	}: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e) => {
		if (e.target === e.currentTarget) onclose();
	}}
	onkeydown={(e) => e.key === 'Escape' && onclose()}
	role="dialog"
	tabindex="-1"
	aria-modal="true"
>
	<div class="modal-content" role="document">
		<div class="modal-header">
			<h2>Order #{selectedOrder.order_number}</h2>
			<button class="close-btn" onclick={onclose}>
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
							<!--
								FIX-2026-04-26 (P2-4): defensive guard. Some legacy orders
								may return without a populated `items` array; the previous
								`{#each orderDetail.items}` would throw on undefined.
							-->
							{#each orderDetail.items ?? [] as item, i (i)}
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
			<button class="btn-secondary" onclick={onclose}>Close</button>
		</div>
	</div>
</div>

<style>
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

	.loading-state {
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

	@media (max-width: 767.98px) {
		.order-detail-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
