<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * MonthlyReturnsChart Component - Monthly Returns Bar Chart
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays monthly trading returns as a bar chart
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5
	 */
	import { onMount, onDestroy } from 'svelte';
	import type { MonthlyPerformance } from '../analytics.state.svelte';

	interface Props {
		data: MonthlyPerformance[];
		isLoading?: boolean;
	}

	const { data, isLoading = false }: Props = $props();

	// DOM reference for responsive width measurement (not reactive)
	// svelte-ignore non_reactive_update
	let containerEl: HTMLDivElement;
	let width = $state(400);
	let hoveredIndex: number | null = $state(null);

	// Chart dimensions
	const height = 220;
	const padding = { top: 20, right: 20, bottom: 50, left: 50 };

	// Derived dimensions
	const chartWidth = $derived(width - padding.left - padding.right);
	const chartHeight = $derived(height - padding.top - padding.bottom);

	// Calculate scales
	const values = $derived(data.map((d) => d.pnl_percent));
	const maxAbsValue = $derived(
		Math.max(Math.abs(Math.min(...values, 0)), Math.max(...values, 0)) || 10
	);

	// Bar width
	const barWidth = $derived(Math.max(20, chartWidth / Math.max(data.length, 1) - 8));

	// Scale functions
	function scaleX(index: number): number {
		const totalWidth = data.length * (barWidth + 8);
		const offset = (chartWidth - totalWidth) / 2;
		return padding.left + offset + index * (barWidth + 8) + barWidth / 2;
	}

	function scaleY(value: number): number {
		const normalized = value / maxAbsValue;
		return padding.top + chartHeight / 2 - (normalized * chartHeight) / 2;
	}

	// Zero line position
	const zeroY = $derived(padding.top + chartHeight / 2);

	// Format functions
	function formatPercent(value: number): string {
		return `${value >= 0 ? '+' : ''}${value.toFixed(1)}%`;
	}

	function formatMonth(monthName: string): string {
		return `${monthName.slice(0, 3)}`;
	}

	function formatFullMonth(monthName: string, year: number): string {
		return `${monthName} ${year}`;
	}

	// Get bar color based on value
	function getBarColor(value: number): string {
		return value >= 0 ? 'var(--color-profit)' : 'var(--color-loss)';
	}

	// Handle resize
	function handleResize() {
		if (containerEl) {
			width = containerEl.clientWidth;
		}
	}

	onMount(() => {
		handleResize();
		window.addEventListener('resize', handleResize);
	});

	onDestroy(() => {
		window.removeEventListener('resize', handleResize);
	});
</script>

<div class="monthly-returns-chart" role="region" aria-label="Monthly returns chart">
	<h3 class="chart-title">Monthly Returns</h3>

	{#if isLoading}
		<div class="skeleton-chart">
			<div class="skel-bars">
				{#each Array(6) as _}
					<div class="skel-bar"></div>
				{/each}
			</div>
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<p>No monthly data available</p>
		</div>
	{:else}
		<div bind:this={containerEl} class="chart-container">
			<svg {width} {height} class="chart-svg">
				<!-- Grid lines -->
				{#each [-1, -0.5, 0, 0.5, 1] as tick}
					{@const y = padding.top + chartHeight / 2 - (tick * chartHeight) / 2}
					<line
						x1={padding.left}
						y1={y}
						x2={width - padding.right}
						y2={y}
						stroke={tick === 0 ? '#9CA3AF' : '#E5E7EB'}
						stroke-dasharray={tick === 0 ? '0' : '4 4'}
					/>
					<text
						x={padding.left - 8}
						{y}
						text-anchor="end"
						dominant-baseline="middle"
						class="axis-label"
					>
						{formatPercent(tick * maxAbsValue)}
					</text>
				{/each}

				<!-- Bars -->
				{#each data as month, index}
					{@const barHeight = Math.abs(month.pnl_percent / maxAbsValue) * (chartHeight / 2)}
					{@const barY = month.pnl_percent >= 0 ? scaleY(month.pnl_percent) : zeroY}
					{@const x = scaleX(index) - barWidth / 2}

					<rect
						{x}
						y={barY}
						width={barWidth}
						height={barHeight}
						fill={getBarColor(month.pnl_percent)}
						rx="4"
						class="bar"
						class:hovered={hoveredIndex === index}
						role="img"
						aria-label="{month.month}: {formatPercent(month.pnl_percent)} ({month.pnl_percent >= 0
							? 'profit'
							: 'loss'})"
						onmouseenter={() => (hoveredIndex = index)}
						onmouseleave={() => (hoveredIndex = null)}
					/>

					<!-- X-axis labels -->
					<text x={scaleX(index)} y={height - 28} text-anchor="middle" class="axis-label">
						{formatMonth(month.month_name)}
					</text>
					<text x={scaleX(index)} y={height - 14} text-anchor="middle" class="axis-label year">
						{month.year}
					</text>
				{/each}
			</svg>

			<!-- Tooltip -->
			{#if hoveredIndex !== null}
				{@const month = data[hoveredIndex]}
				<div
					class="tooltip"
					style="left: {scaleX(hoveredIndex)}px; top: {scaleY(Math.max(month.pnl_percent, 0)) -
						60}px"
				>
					<div class="tooltip-month">{formatFullMonth(month.month_name, month.year)}</div>
					<div
						class="tooltip-value"
						class:profit={month.pnl_percent >= 0}
						class:loss={month.pnl_percent < 0}
					>
						{formatPercent(month.pnl_percent)}
					</div>
					<div class="tooltip-stats">
						{month.total_trades} trades | {month.win_rate.toFixed(0)}% WR
					</div>
				</div>
			{/if}
		</div>

		<!-- Summary Stats -->
		{#if data.length > 0}
			{@const positiveMonths = data.filter((m) => m.pnl_percent > 0).length}
			{@const negativeMonths = data.filter((m) => m.pnl_percent < 0).length}
			{@const avgReturn = data.reduce((acc, m) => acc + m.pnl_percent, 0) / data.length}

			<div class="summary-row">
				<div class="summary-stat">
					<span class="stat-label">Profitable Months</span>
					<span class="stat-value profit">{positiveMonths}</span>
				</div>
				<div class="summary-stat">
					<span class="stat-label">Losing Months</span>
					<span class="stat-value loss">{negativeMonths}</span>
				</div>
				<div class="summary-stat">
					<span class="stat-label">Avg Monthly</span>
					<span class="stat-value" class:profit={avgReturn >= 0} class:loss={avgReturn < 0}>
						{formatPercent(avgReturn)}
					</span>
				</div>
			</div>
		{/if}
	{/if}
</div>

<style>
	.monthly-returns-chart {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.chart-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0 0 16px 0;
	}

	.chart-container {
		position: relative;
	}

	.chart-svg {
		overflow: visible;
	}

	.axis-label {
		font-size: 10px;
		fill: var(--color-text-tertiary);
	}

	.axis-label.year {
		font-size: 9px;
		fill: var(--color-text-quaternary, #9ca3af);
	}

	.bar {
		transition: opacity 0.15s;
		cursor: pointer;
	}

	.bar:hover,
	.bar.hovered {
		opacity: 0.8;
	}

	.tooltip {
		position: absolute;
		pointer-events: none;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 8px;
		padding: 10px 12px;
		box-shadow: var(--shadow-md);
		transform: translateX(-50%);
		z-index: 10;
		text-align: center;
	}

	.tooltip-month {
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin-bottom: 4px;
	}

	.tooltip-value {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		margin-bottom: 4px;
	}

	.tooltip-value.profit {
		color: var(--color-profit);
	}

	.tooltip-value.loss {
		color: var(--color-loss);
	}

	.tooltip-stats {
		font-size: 11px;
		color: var(--color-text-tertiary);
	}

	/* Summary Row */
	.summary-row {
		display: flex;
		justify-content: space-around;
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid var(--color-border-subtle);
	}

	.summary-stat {
		text-align: center;
	}

	.stat-label {
		display: block;
		font-size: 10px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 4px;
	}

	.stat-value {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		color: var(--color-text-primary);
	}

	.stat-value.profit {
		color: var(--color-profit);
	}

	.stat-value.loss {
		color: var(--color-loss);
	}

	/* Skeleton */
	.skeleton-chart {
		height: 220px;
		display: flex;
		align-items: flex-end;
		justify-content: center;
		padding-bottom: 50px;
	}

	.skel-bars {
		display: flex;
		gap: 12px;
		align-items: flex-end;
	}

	.skel-bar {
		width: 30px;
		height: 60px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skel-bar:nth-child(2) {
		height: 100px;
	}
	.skel-bar:nth-child(3) {
		height: 40px;
	}
	.skel-bar:nth-child(4) {
		height: 80px;
	}
	.skel-bar:nth-child(5) {
		height: 30px;
	}
	.skel-bar:nth-child(6) {
		height: 70px;
	}

	.empty-state {
		height: 180px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
		font-size: 14px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	@media (max-width: 640px) {
		.monthly-returns-chart {
			padding: 16px;
		}

		.summary-row {
			flex-direction: column;
			gap: 12px;
		}
	}
</style>
