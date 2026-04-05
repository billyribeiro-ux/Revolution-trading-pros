<!--
	Revenue Breakdown Component
	═══════════════════════════════════════════════════════════════════════════
	
	Advanced revenue analytics with MRR, ARR, churn, and expansion metrics.
-->

<script lang="ts">
	import { Icon, IconMinus, IconTrendingDown, IconTrendingUp } from '$lib/icons';

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

	function getTrend(change: number, inverse: boolean = false): 'positive' | 'negative' | 'neutral' {
		if (change > 0) return inverse ? 'negative' : 'positive';
		if (change < 0) return inverse ? 'positive' : 'negative';
		return 'neutral';
	}
</script>

<div class="revenue-breakdown">
	<div class="breakdown-header">
		<h3 class="breakdown-title">Revenue Breakdown</h3>
		<div class="period-badge">This Month</div>
	</div>

	<!-- Primary Metrics -->
	<div class="primary-metrics">
		<div class="metric-card" data-variant="primary">
			<div class="metric-label">Monthly Recurring Revenue</div>
			<div class="metric-value">{formatCurrency(safeData.mrr)}</div>
			{#if true}
				{@const TrendIcon = getTrendIcon(safeData.mrr_change)}
				<div
					class="metric-change"
					class:positive={safeData.mrr_change > 0}
					class:negative={safeData.mrr_change < 0}
				>
					<Icon icon={TrendIcon} size={16} />
					{formatPercent(Math.abs(safeData.mrr_change))} vs last month
				</div>
			{/if}
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
			<div class="movement-item" data-trend="positive">
				<div class="movement-label">New MRR</div>
				<div class="movement-value">+{formatCurrency(safeData.new_mrr)}</div>
			</div>

			<div class="movement-item" data-trend="positive">
				<div class="movement-label">Expansion MRR</div>
				<div class="movement-value">+{formatCurrency(safeData.expansion_mrr)}</div>
			</div>

			<div class="movement-item" data-trend="negative">
				<div class="movement-label">Contraction MRR</div>
				<div class="movement-value">-{formatCurrency(safeData.contraction_mrr)}</div>
			</div>

			<div class="movement-item" data-trend="negative">
				<div class="movement-label">Churned MRR</div>
				<div class="movement-value">-{formatCurrency(safeData.churn_mrr)}</div>
			</div>

			<div
				class="movement-item"
				data-variant="net"
				data-trend={safeData.net_new_mrr > 0
					? 'positive'
					: safeData.net_new_mrr < 0
						? 'negative'
						: 'neutral'}
			>
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
				<div class="metric-value small" data-trend={getTrend(safeData.churn_rate, true)}>
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
			<div class="waterfall-bar" data-type="start">
				<div class="bar-label">Start MRR</div>
				<div class="bar" style="height: 100%">
					<span class="bar-value">{formatCurrency(safeData.mrr - safeData.net_new_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar" data-type="positive">
				<div class="bar-label">New</div>
				<div
					class="bar"
					style="height: {safeData.mrr > 0 ? (safeData.new_mrr / safeData.mrr) * 100 : 0}%"
				>
					<span class="bar-value">+{formatCurrency(safeData.new_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar" data-type="positive">
				<div class="bar-label">Expansion</div>
				<div
					class="bar"
					style="height: {safeData.mrr > 0 ? (safeData.expansion_mrr / safeData.mrr) * 100 : 0}%"
				>
					<span class="bar-value">+{formatCurrency(safeData.expansion_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar" data-type="negative">
				<div class="bar-label">Contraction</div>
				<div
					class="bar"
					style="height: {safeData.mrr > 0 ? (safeData.contraction_mrr / safeData.mrr) * 100 : 0}%"
				>
					<span class="bar-value">-{formatCurrency(safeData.contraction_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar" data-type="negative">
				<div class="bar-label">Churn</div>
				<div
					class="bar"
					style="height: {safeData.mrr > 0 ? (safeData.churn_mrr / safeData.mrr) * 100 : 0}%"
				>
					<span class="bar-value">-{formatCurrency(safeData.churn_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar" data-type="end">
				<div class="bar-label">End MRR</div>
				<div class="bar" style="height: 100%">
					<span class="bar-value">{formatCurrency(safeData.mrr)}</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.revenue-breakdown {
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.breakdown-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.breakdown-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.period-badge {
		padding-inline: var(--space-3);
		padding-block: var(--space-1);
		background-color: oklch(0.8 0.18 90 / 20%);
		color: oklch(0.8 0.18 90);
		border-radius: var(--radius-lg);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
	}

	.primary-metrics {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
		margin-block-end: var(--space-6);
		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.metric-card {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		border: 1px solid oklch(0.38 0.01 250 / 50%);

		&[data-variant='primary'] {
			border-color: oklch(0.8 0.18 90 / 30%);
			background-color: oklch(0.8 0.18 90 / 5%);
		}
	}

	.metric-label {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-2);
	}

	.metric-value {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-1);

		&.small {
			font-size: var(--text-xl);
		}
		&[data-trend='positive'] {
			color: oklch(0.7 0.18 160);
		}
		&[data-trend='negative'] {
			color: oklch(0.7 0.2 25);
		}
		&[data-trend='neutral'] {
			color: oklch(0.65 0.01 250);
		}
	}

	.metric-sublabel {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.metric-change {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);

		&.positive {
			color: oklch(0.7 0.18 160);
		}
		&.negative {
			color: oklch(0.7 0.2 25);
		}
	}

	.mrr-movement {
		margin-block-end: var(--space-6);
	}

	.section-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-4);
	}

	.movement-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-3);
		@media (min-width: 768px) {
			grid-template-columns: repeat(5, 1fr);
		}
	}

	.movement-item {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-3);
		border: 1px solid oklch(0.38 0.01 250 / 50%);

		&[data-trend='positive'] {
			border-color: oklch(0.6 0.18 160 / 30%);
			background-color: oklch(0.6 0.18 160 / 5%);
		}
		&[data-trend='negative'] {
			border-color: oklch(0.55 0.22 25 / 30%);
			background-color: oklch(0.55 0.22 25 / 5%);
		}
		&[data-variant='net'] {
			border-color: oklch(0.8 0.18 90 / 30%);
			background-color: oklch(0.8 0.18 90 / 5%);
		}
	}

	.movement-label {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-1);
	}

	.movement-value {
		font-size: var(--text-lg);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.secondary-metrics {
		margin-block-end: var(--space-6);
	}

	.metric-row {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--space-4);
	}

	.metric-item {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-3);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.waterfall-chart {
		margin-block-start: var(--space-6);
	}

	.waterfall-bars {
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		gap: var(--space-2);
		block-size: 16rem;
		background-color: oklch(0.15 0.01 250 / 30%);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
	}

	.waterfall-bar {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-2);
	}

	.bar-label {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		text-align: center;
	}

	.bar {
		inline-size: 100%;
		border-radius: var(--radius-sm) var(--radius-sm) 0 0;
		position: relative;
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-block-start: var(--space-2);
	}

	.waterfall-bar[data-type='start'] .bar,
	.waterfall-bar[data-type='end'] .bar {
		background-color: oklch(0.6 0.2 260);
	}

	.waterfall-bar[data-type='positive'] .bar {
		background-color: oklch(0.6 0.18 160);
	}

	.waterfall-bar[data-type='negative'] .bar {
		background-color: oklch(0.55 0.22 25);
	}

	.bar-value {
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}
</style>
