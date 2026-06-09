<script lang="ts">
	import { IconReceipt, IconDownload, IconExternalLink } from '$lib/icons';
	import type { Order } from '$lib/api/members';
	import { formatCurrency, formatDate } from './helpers';

	interface Props {
		orders: Order[] | undefined;
	}

	let { orders }: Props = $props();
</script>

<div class="panel">
	<div class="panel-header">
		<h3>Order History</h3>
		<button class="btn-secondary small">
			<IconDownload size={16} />
			Export
		</button>
	</div>
	{#if !orders || orders.length === 0}
		<div class="empty-state">
			<IconReceipt size={48} stroke={1} />
			<h4>No Orders</h4>
			<p>This member has no order history</p>
		</div>
	{:else}
		<table class="orders-table">
			<thead>
				<tr>
					<th>Order #</th>
					<th>Date</th>
					<th>Status</th>
					<th>Total</th>
					<th>Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each orders as order (order.id)}
					<tr>
						<td class="order-number">{order.number}</td>
						<td>{formatDate(order.created_at)}</td>
						<td>
							<span
								class={[
									'status-badge',
									order.status === 'completed' ? 'status-completed' : 'status-pending'
								]}
							>
								{order.status}
							</span>
						</td>
						<td class="order-total">{formatCurrency(order.total)}</td>
						<td>
							<button class="btn-icon small">
								<IconExternalLink size={16} />
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>

<style>
	.panel {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.panel-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h4 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.orders-table {
		width: 100%;
		border-collapse: collapse;
	}

	.orders-table thead {
		background: rgba(15, 23, 42, 0.6);
	}

	.orders-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.orders-table tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.orders-table td {
		padding: 1rem 1.5rem;
		color: #cbd5e1;
	}

	.order-number {
		font-weight: 600;
		color: var(--primary-400);
	}

	.order-total {
		font-weight: 600;
		color: #34d399;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
		text-transform: capitalize;
	}

	.status-completed {
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
		border-color: rgba(16, 185, 129, 0.35);
	}

	.status-pending {
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
		border-color: rgba(234, 179, 8, 0.35);
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
		font-size: 0.875rem;
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	.btn-secondary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.btn-icon.small {
		width: 28px;
		height: 28px;
	}
</style>
