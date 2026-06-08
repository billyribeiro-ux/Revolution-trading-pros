<!--
	Revenue Breakdown Component
	═══════════════════════════════════════════════════════════════════════════
	
	Advanced revenue analytics with MRR, ARR, churn, and expansion metrics.
-->

<script lang="ts">
	import { IconTrendingUp, IconTrendingDown, IconMinus } from '$lib/icons';

	interface RevenueData {
		mrr: number;
		mrr_change: number;
		arr: number;
		new_mrr: number;
		expansion_mrr: number;
		contraction_mrr: number;
		churn_mrr: number;
		net_new_mrr: number;
		churn_rate: number;
		ltv: number;
		arpu: number;
	}

	interface Props {
		data: RevenueData;
	}

	let { data }: Props = $props();

	// Default data to prevent undefined errors
	const safeData = $derived(
		data ?? {
			mrr: 0,
			mrr_change: 0,
			arr: 0,
			new_mrr: 0,
			expansion_mrr: 0,
			contraction_mrr: 0,
			churn_mrr: 0,
			net_new_mrr: 0,
			churn_rate: 0,
			ltv: 0,
			arpu: 0
		}
	);

	function formatCurrency(value: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(value);
	}

	function formatPercent(value: number): string {
		return value.toFixed(1) + '%';
	}

	function getTrendIcon(change: number) {
		if (change > 0) return IconTrendingUp;
		if (change < 0) return IconTrendingDown;
		return IconMinus;
	}

	function getTrendClass(change: number, inverse: boolean = false) {
		if (change > 0) return inverse ? 'metric-value--negative' : 'metric-value--positive';
		if (change < 0) return inverse ? 'metric-value--positive' : 'metric-value--negative';
		return 'metric-value--neutral';
	}

	function getMovementClass(value: number) {
		return value > 0 ? 'positive' : value < 0 ? 'negative' : undefined;
	}

	function getWaterfallHeight(value: number) {
		return `${safeData.mrr > 0 ? (value / safeData.mrr) * 100 : 0}%`;
	}
</script>

<div class="revenue-breakdown">
	<div class="breakdown-header">
		<h3 class="breakdown-title">Revenue Breakdown</h3>
		<div class="period-badge">This Month</div>
	</div>

	<!-- Primary Metrics -->
	<div class="primary-metrics">
		<div class="metric-card primary">
			<div class="metric-label">Monthly Recurring Revenue</div>
			<div class="metric-value">{formatCurrency(safeData.mrr)}</div>
			{const TrendIcon = getTrendIcon(safeData.mrr_change)}
			<div
				class={[
					'metric-change',
					{
						positive: safeData.mrr_change > 0,
						negative: safeData.mrr_change < 0
					}
				]}
			>
				<TrendIcon size={16} />
				{formatPercent(Math.abs(safeData.mrr_change))} vs last month
			</div>
		</div>

		<div class="metric-card">
			<div class="metric-label">Annual Recurring Revenue</div>
			<div class="metric-value">{formatCurrency(safeData.arr)}</div>
			<div class="metric-sublabel">{formatCurrency(safeData.mrr)} × 12</div>
		</div>
	</div>

	<!-- MRR Movement -->
	<div class="mrr-movement">
		<h4 class="section-title">MRR Movement</h4>
		<div class="movement-grid">
			<div class="movement-item positive">
				<div class="movement-label">New MRR</div>
				<div class="movement-value">+{formatCurrency(safeData.new_mrr)}</div>
			</div>

			<div class="movement-item positive">
				<div class="movement-label">Expansion MRR</div>
				<div class="movement-value">+{formatCurrency(safeData.expansion_mrr)}</div>
			</div>

			<div class="movement-item negative">
				<div class="movement-label">Contraction MRR</div>
				<div class="movement-value">-{formatCurrency(safeData.contraction_mrr)}</div>
			</div>

			<div class="movement-item negative">
				<div class="movement-label">Churned MRR</div>
				<div class="movement-value">-{formatCurrency(safeData.churn_mrr)}</div>
			</div>

			<div class={['movement-item', 'net', getMovementClass(safeData.net_new_mrr)]}>
				<div class="movement-label">Net New MRR</div>
				<div class="movement-value">
					{safeData.net_new_mrr > 0 ? '+' : ''}{formatCurrency(safeData.net_new_mrr)}
				</div>
			</div>
		</div>
	</div>

	<!-- Secondary Metrics -->
	<div class="secondary-metrics">
		<div class="metric-row">
			<div class="metric-item">
				<div class="metric-label">Churn Rate</div>
				<div class={['metric-value', 'small', getTrendClass(safeData.churn_rate, true)]}>
					{formatPercent(safeData.churn_rate)}
				</div>
			</div>

			<div class="metric-item">
				<div class="metric-label">Customer LTV</div>
				<div class="metric-value small">{formatCurrency(safeData.ltv)}</div>
			</div>

			<div class="metric-item">
				<div class="metric-label">ARPU</div>
				<div class="metric-value small">{formatCurrency(safeData.arpu)}</div>
			</div>
		</div>
	</div>

	<!-- Waterfall Chart -->
	<div class="waterfall-chart">
		<h4 class="section-title">MRR Waterfall</h4>
		<div class="waterfall-bars">
			<div class="waterfall-bar start">
				<div class="bar-label">Start MRR</div>
				<div class="bar" style:height="100%">
					<span class="bar-value">{formatCurrency(safeData.mrr - safeData.net_new_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar positive">
				<div class="bar-label">New</div>
				<div class="bar" style:height={getWaterfallHeight(safeData.new_mrr)}>
					<span class="bar-value">+{formatCurrency(safeData.new_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar positive">
				<div class="bar-label">Expansion</div>
				<div class="bar" style:height={getWaterfallHeight(safeData.expansion_mrr)}>
					<span class="bar-value">+{formatCurrency(safeData.expansion_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar negative">
				<div class="bar-label">Contraction</div>
				<div class="bar" style:height={getWaterfallHeight(safeData.contraction_mrr)}>
					<span class="bar-value">-{formatCurrency(safeData.contraction_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar negative">
				<div class="bar-label">Churn</div>
				<div class="bar" style:height={getWaterfallHeight(safeData.churn_mrr)}>
					<span class="bar-value">-{formatCurrency(safeData.churn_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar end">
				<div class="bar-label">End MRR</div>
				<div class="bar" style:height="100%">
					<span class="bar-value">{formatCurrency(safeData.mrr)}</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.revenue-breakdown {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.breakdown-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.breakdown-title {
		margin: 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 700;
		color: #ffffff;
	}

	.period-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 0.5rem;
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
		font-size: 0.875rem;
		font-weight: 500;
		white-space: nowrap;
	}

	.primary-metrics {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.metric-card {
		padding: 1rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.metric-card.primary {
		border-color: rgba(234, 179, 8, 0.3);
		background: rgba(234, 179, 8, 0.05);
	}

	.metric-label {
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.metric-value {
		margin-bottom: 0.25rem;
		font-size: 1.875rem;
		line-height: 2.25rem;
		font-weight: 700;
		color: #ffffff;
	}

	.metric-value.small {
		font-size: 1.25rem;
		line-height: 1.75rem;
	}

	.metric-value--positive {
		color: #4ade80;
	}

	.metric-value--negative {
		color: #f87171;
	}

	.metric-value--neutral {
		color: #9ca3af;
	}

	.metric-sublabel {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.metric-change {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.metric-change.positive {
		color: #4ade80;
	}

	.metric-change.negative {
		color: #f87171;
	}

	.mrr-movement {
		margin-bottom: 1.5rem;
	}

	.section-title {
		margin: 0 0 1rem;
		font-size: 1.125rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #ffffff;
	}

	.movement-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.75rem;
	}

	.movement-item {
		padding: 0.75rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.movement-item.positive {
		border-color: rgba(34, 197, 94, 0.3);
		background: rgba(34, 197, 94, 0.05);
	}

	.movement-item.negative {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.05);
	}

	.movement-item.net {
		border-color: rgba(234, 179, 8, 0.3);
		background: rgba(234, 179, 8, 0.05);
	}

	.movement-label {
		margin-bottom: 0.25rem;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.movement-value {
		font-size: 1.125rem;
		line-height: 1.75rem;
		font-weight: 700;
		color: #ffffff;
	}

	.secondary-metrics {
		margin-bottom: 1.5rem;
	}

	.metric-row {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 1rem;
	}

	.metric-item {
		padding: 0.75rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.waterfall-chart {
		margin-top: 1.5rem;
	}

	.waterfall-bars {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		gap: 0.5rem;
		height: 16rem;
		padding: 1rem;
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.3);
	}

	.waterfall-bar {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: center;
		justify-content: flex-end;
		gap: 0.5rem;
		min-width: 0;
	}

	.bar-label {
		font-size: 0.75rem;
		color: #9ca3af;
		text-align: center;
	}

	.bar {
		position: relative;
		display: flex;
		align-items: flex-start;
		justify-content: center;
		width: 100%;
		min-height: 0.25rem;
		padding-top: 0.5rem;
		border-radius: 0.25rem 0.25rem 0 0;
	}

	.waterfall-bar.start .bar,
	.waterfall-bar.end .bar {
		background: #3b82f6;
	}

	.waterfall-bar.positive .bar {
		background: #22c55e;
	}

	.waterfall-bar.negative .bar {
		background: #ef4444;
	}

	.bar-value {
		font-size: 0.75rem;
		font-weight: 600;
		color: #ffffff;
		white-space: nowrap;
	}

	@media (min-width: 768px) {
		.primary-metrics {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.movement-grid {
			grid-template-columns: repeat(5, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.metric-row {
			grid-template-columns: 1fr;
		}

		.waterfall-bars {
			overflow-x: auto;
		}

		.waterfall-bar {
			min-width: 4rem;
		}
	}
</style>
