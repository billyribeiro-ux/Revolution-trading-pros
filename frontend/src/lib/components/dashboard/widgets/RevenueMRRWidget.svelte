<script lang="ts">
	interface Props {
		data: any;
		config?: {
			period?: string;
			show_chart?: boolean;
			currency?: string;
			show_growth?: boolean;
			format?: 'compact' | 'full';
		};
	}

	let { data, config = {} }: Props = $props();

	let currentMRR = $derived(data?.current_mrr || 0);
	let growthPercentage = $derived(data?.growth_percentage || 0);
	let isPositive = $derived(growthPercentage >= 0);
	let currency = $derived(config.currency || '$');
	let showChart = $derived(config.show_chart !== false);
	let showGrowth = $derived(config.show_growth !== false);

	function formatNumber(num: number): string {
		if (config.format === 'compact' && num >= 1000) {
			return (num / 1000).toFixed(1) + 'K';
		}
		return num.toLocaleString();
	}
</script>

<div class="revenue-mrr">
	<div class="mrr-value">
		<span class="currency">{currency}</span>
		<span class="amount">{formatNumber(currentMRR)}</span>
	</div>

	{#if showGrowth}
		<div class="growth-indicator" class:positive={isPositive} class:negative={!isPositive}>
			<span class="arrow">{isPositive ? '↑' : '↓'}</span>
			<span class="percentage">{Math.abs(growthPercentage).toFixed(1)}%</span>
		</div>
	{/if}

	{#if showChart && data?.chart_data}
		<div class="mini-chart">
			{#each data.chart_data as point}
				<div class="chart-bar" style="height: {(point.revenue / currentMRR) * 100}%"></div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.revenue-mrr {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.mrr-value {
		display: flex;
		align-items: baseline;
		gap: 0.25rem;
	}

	.currency {
		font-size: 1.5rem;
		color: #6b7280;
		font-weight: 500;
	}

	.amount {
		font-size: 2.5rem;
		font-weight: 700;
		color: #1f2937;
	}

	.growth-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.growth-indicator.positive {
		color: #10b981;
	}

	.growth-indicator.negative {
		color: #ef4444;
	}

	.mini-chart {
		display: flex;
		align-items: flex-end;
		gap: 4px;
		height: 60px;
		margin-top: 0.5rem;
	}

	.chart-bar {
		flex: 1;
		background: linear-gradient(to top, #3b82f6, #60a5fa);
		border-radius: 2px 2px 0 0;
		min-height: 4px;
	}
</style>
