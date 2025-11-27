<!--
	Revenue Breakdown Component
	═══════════════════════════════════════════════════════════════════════════
	
	Advanced revenue analytics with MRR, ARR, churn, and expansion metrics.
-->

<script lang="ts">
	import { IconTrendingUp, IconTrendingDown, IconMinus } from '@tabler/icons-svelte';

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

	function getTrendColor(change: number, inverse: boolean = false) {
		if (change > 0) return inverse ? 'text-red-400' : 'text-green-400';
		if (change < 0) return inverse ? 'text-green-400' : 'text-red-400';
		return 'text-gray-400';
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
			<div class="metric-value">{formatCurrency(data.mrr)}</div>
			{#if true}
			{@const TrendIcon = getTrendIcon(data.mrr_change)}
			<div
				class="metric-change"
				class:positive={data.mrr_change > 0}
				class:negative={data.mrr_change < 0}
			>
				<TrendIcon size={16} />
				{formatPercent(Math.abs(data.mrr_change))} vs last month
			</div>
			{/if}
		</div>

		<div class="metric-card">
			<div class="metric-label">Annual Recurring Revenue</div>
			<div class="metric-value">{formatCurrency(data.arr)}</div>
			<div class="metric-sublabel">{formatCurrency(data.mrr)} × 12</div>
		</div>
	</div>

	<!-- MRR Movement -->
	<div class="mrr-movement">
		<h4 class="section-title">MRR Movement</h4>
		<div class="movement-grid">
			<div class="movement-item positive">
				<div class="movement-label">New MRR</div>
				<div class="movement-value">+{formatCurrency(data.new_mrr)}</div>
			</div>

			<div class="movement-item positive">
				<div class="movement-label">Expansion MRR</div>
				<div class="movement-value">+{formatCurrency(data.expansion_mrr)}</div>
			</div>

			<div class="movement-item negative">
				<div class="movement-label">Contraction MRR</div>
				<div class="movement-value">-{formatCurrency(data.contraction_mrr)}</div>
			</div>

			<div class="movement-item negative">
				<div class="movement-label">Churned MRR</div>
				<div class="movement-value">-{formatCurrency(data.churn_mrr)}</div>
			</div>

			<div
				class="movement-item net {data.net_new_mrr > 0
					? 'positive'
					: data.net_new_mrr < 0
						? 'negative'
						: ''}"
			>
				<div class="movement-label">Net New MRR</div>
				<div class="movement-value">
					{data.net_new_mrr > 0 ? '+' : ''}{formatCurrency(data.net_new_mrr)}
				</div>
			</div>
		</div>
	</div>

	<!-- Secondary Metrics -->
	<div class="secondary-metrics">
		<div class="metric-row">
			<div class="metric-item">
				<div class="metric-label">Churn Rate</div>
				<div class="metric-value small {getTrendColor(data.churn_rate, true)}">
					{formatPercent(data.churn_rate)}
				</div>
			</div>

			<div class="metric-item">
				<div class="metric-label">Customer LTV</div>
				<div class="metric-value small">{formatCurrency(data.ltv)}</div>
			</div>

			<div class="metric-item">
				<div class="metric-label">ARPU</div>
				<div class="metric-value small">{formatCurrency(data.arpu)}</div>
			</div>
		</div>
	</div>

	<!-- Waterfall Chart -->
	<div class="waterfall-chart">
		<h4 class="section-title">MRR Waterfall</h4>
		<div class="waterfall-bars">
			<div class="waterfall-bar start">
				<div class="bar-label">Start MRR</div>
				<div class="bar" style="height: 100%">
					<span class="bar-value">{formatCurrency(data.mrr - data.net_new_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar positive">
				<div class="bar-label">New</div>
				<div class="bar" style="height: {(data.new_mrr / data.mrr) * 100}%">
					<span class="bar-value">+{formatCurrency(data.new_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar positive">
				<div class="bar-label">Expansion</div>
				<div class="bar" style="height: {(data.expansion_mrr / data.mrr) * 100}%">
					<span class="bar-value">+{formatCurrency(data.expansion_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar negative">
				<div class="bar-label">Contraction</div>
				<div class="bar" style="height: {(data.contraction_mrr / data.mrr) * 100}%">
					<span class="bar-value">-{formatCurrency(data.contraction_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar negative">
				<div class="bar-label">Churn</div>
				<div class="bar" style="height: {(data.churn_mrr / data.mrr) * 100}%">
					<span class="bar-value">-{formatCurrency(data.churn_mrr)}</span>
				</div>
			</div>

			<div class="waterfall-bar end">
				<div class="bar-label">End MRR</div>
				<div class="bar" style="height: 100%">
					<span class="bar-value">{formatCurrency(data.mrr)}</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style lang="postcss">
	@reference "../../../app.css";
	.revenue-breakdown {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.breakdown-header {
		@apply flex items-center justify-between mb-6;
	}

	.breakdown-title {
		@apply text-xl font-bold text-white;
	}

	.period-badge {
		@apply px-3 py-1 bg-yellow-500/20 text-yellow-400 rounded-lg text-sm font-medium;
	}

	.primary-metrics {
		@apply grid grid-cols-1 md:grid-cols-2 gap-4 mb-6;
	}

	.metric-card {
		@apply bg-gray-900/50 rounded-lg p-4 border border-gray-700/50;
	}

	.metric-card.primary {
		@apply border-yellow-500/30 bg-yellow-500/5;
	}

	.metric-label {
		@apply text-sm text-gray-400 mb-2;
	}

	.metric-value {
		@apply text-3xl font-bold text-white mb-1;
	}

	.metric-value.small {
		@apply text-xl;
	}

	.metric-sublabel {
		@apply text-xs text-gray-500;
	}

	.metric-change {
		@apply flex items-center gap-1 text-sm font-medium;
	}

	.metric-change.positive {
		@apply text-green-400;
	}

	.metric-change.negative {
		@apply text-red-400;
	}

	.mrr-movement {
		@apply mb-6;
	}

	.section-title {
		@apply text-lg font-semibold text-white mb-4;
	}

	.movement-grid {
		@apply grid grid-cols-2 md:grid-cols-5 gap-3;
	}

	.movement-item {
		@apply bg-gray-900/50 rounded-lg p-3 border border-gray-700/50;
	}

	.movement-item.positive {
		@apply border-green-500/30 bg-green-500/5;
	}

	.movement-item.negative {
		@apply border-red-500/30 bg-red-500/5;
	}

	.movement-item.net {
		@apply border-yellow-500/30 bg-yellow-500/5;
	}

	.movement-label {
		@apply text-xs text-gray-400 mb-1;
	}

	.movement-value {
		@apply text-lg font-bold text-white;
	}

	.secondary-metrics {
		@apply mb-6;
	}

	.metric-row {
		@apply grid grid-cols-3 gap-4;
	}

	.metric-item {
		@apply bg-gray-900/50 rounded-lg p-3 border border-gray-700/50;
	}

	.waterfall-chart {
		@apply mt-6;
	}

	.waterfall-bars {
		@apply flex items-end justify-between gap-2 h-64 bg-gray-900/30 rounded-lg p-4;
	}

	.waterfall-bar {
		@apply flex-1 flex flex-col items-center justify-end gap-2;
	}

	.bar-label {
		@apply text-xs text-gray-400 text-center;
	}

	.bar {
		@apply w-full rounded-t relative flex items-start justify-center pt-2;
	}

	.waterfall-bar.start .bar,
	.waterfall-bar.end .bar {
		@apply bg-blue-500;
	}

	.waterfall-bar.positive .bar {
		@apply bg-green-500;
	}

	.waterfall-bar.negative .bar {
		@apply bg-red-500;
	}

	.bar-value {
		@apply text-xs font-semibold text-white;
	}
</style>
