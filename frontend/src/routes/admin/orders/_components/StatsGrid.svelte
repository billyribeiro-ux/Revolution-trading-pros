<script lang="ts">
	import { IconShoppingCart, IconCurrencyDollar, IconCheck, IconClock } from '#lib/icons/index.js';

	interface OrderStats {
		total_orders: number;
		completed_orders: number;
		pending_orders: number;
		refunded_orders: number;
		total_revenue: number;
		revenue_this_month: number;
		average_order_value: number;
	}

	interface Props {
		/** null while loading: the grid still renders (with placeholders) so the page never shifts. */
		stats: OrderStats | null;
		formatCurrency: (amount: number, currency?: string) => string;
	}

	let { stats, formatCurrency }: Props = $props();
</script>

<div class="stats-grid">
	<div class="stat-card gradient-purple">
		<div class="stat-icon">
			<IconShoppingCart size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Total Orders</div>
			<div class="stat-value">{stats ? stats.total_orders.toLocaleString() : '—'}</div>
		</div>
	</div>

	<div class="stat-card gradient-emerald">
		<div class="stat-icon">
			<IconCheck size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Completed</div>
			<div class="stat-value">{stats ? stats.completed_orders.toLocaleString() : '—'}</div>
		</div>
	</div>

	<div class="stat-card gradient-gold">
		<div class="stat-icon">
			<IconCurrencyDollar size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Total Revenue</div>
			<div class="stat-value">{stats ? formatCurrency(stats.total_revenue) : '—'}</div>
			<div class="stat-change neutral">
				{stats ? formatCurrency(stats.revenue_this_month) : '—'} this month
			</div>
		</div>
	</div>

	<div class="stat-card gradient-blue">
		<div class="stat-icon">
			<IconClock size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Avg Order Value</div>
			<div class="stat-value">{stats ? formatCurrency(stats.average_order_value) : '—'}</div>
		</div>
	</div>
</div>

<style>
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
	}

	.stat-card.gradient-purple {
		border-color: rgba(230, 184, 0, 0.3);
	}
	.stat-card.gradient-emerald {
		border-color: rgba(63, 185, 80, 0.3);
	}
	.stat-card.gradient-gold {
		border-color: rgba(210, 153, 34, 0.3);
	}
	.stat-card.gradient-blue {
		border-color: rgba(88, 166, 255, 0.3);
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
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis);
	}
	.gradient-gold .stat-icon {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis);
	}
	.gradient-blue .stat-icon {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
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

	@media (max-width: 1023.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767.98px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
