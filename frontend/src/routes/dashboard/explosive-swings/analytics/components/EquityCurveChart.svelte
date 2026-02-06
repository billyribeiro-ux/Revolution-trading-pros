<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * EquityCurveChart Component - Cumulative P&L Line Chart
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays cumulative P&L over time as a line chart using D3
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5
	 */
	import { onMount, onDestroy } from 'svelte';
	import type { EquityPoint } from '../analytics.state.svelte';

	interface Props {
		data: EquityPoint[];
		isLoading?: boolean;
	}

	const { data, isLoading = false }: Props = $props();

	// DOM reference for responsive width measurement (not reactive)
	// svelte-ignore non_reactive_update
	let containerEl: HTMLDivElement;
	let width = $state(600);
	let hoveredIndex: number | null = $state(null);
	let tooltipX = $state(0);
	let tooltipY = $state(0);

	// Chart dimensions
	const height = 280;
	const padding = { top: 20, right: 20, bottom: 40, left: 60 };

	// Derived dimensions
	const chartWidth = $derived(width - padding.left - padding.right);
	const chartHeight = $derived(height - padding.top - padding.bottom);

	// Calculate scales
	const values = $derived(data.map((d) => d.equity_percent));
	const minValue = $derived(Math.min(...values, 0));
	const maxValue = $derived(Math.max(...values, 0));
	const valueRange = $derived(maxValue - minValue || 1);

	// Scale functions
	function scaleX(index: number): number {
		return padding.left + (index / Math.max(data.length - 1, 1)) * chartWidth;
	}

	function scaleY(value: number): number {
		return padding.top + chartHeight - ((value - minValue) / valueRange) * chartHeight;
	}

	// Zero line position
	const zeroY = $derived(scaleY(0));

	// Generate SVG path for line
	const linePath = $derived(
		data.length > 0
			? data
					.map((d, i) => {
						const x = scaleX(i);
						const y = scaleY(d.equity_percent);
						return i === 0 ? `M ${x} ${y}` : `L ${x} ${y}`;
					})
					.join(' ')
			: ''
	);

	// Generate SVG path for area (gradient fill)
	const areaPath = $derived(
		data.length > 0
			? linePath + ` L ${scaleX(data.length - 1)} ${zeroY}` + ` L ${padding.left} ${zeroY}` + ` Z`
			: ''
	);

	// Y-axis ticks
	const yTicks = $derived(
		[0, 0.25, 0.5, 0.75, 1].map((t) => ({
			value: minValue + t * valueRange,
			y: scaleY(minValue + t * valueRange)
		}))
	);

	// X-axis labels (show ~5 labels)
	const xLabels = $derived.by(() => {
		if (data.length === 0) return [];
		const step = Math.ceil(data.length / 5);
		return data.filter((_, i) => i % step === 0 || i === data.length - 1);
	});

	// Format functions
	function formatValue(value: number): string {
		return `${value >= 0 ? '+' : ''}${value.toFixed(1)}%`;
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	// Determine line color based on final value
	const lineColor = $derived(
		data.length > 0 && data[data.length - 1].equity_percent >= 0 ? '#10B981' : '#EF4444'
	);

	// Handle resize
	function handleResize() {
		if (containerEl) {
			width = containerEl.clientWidth;
		}
	}

	// Handle mouse events
	function handleMouseMove(event: MouseEvent) {
		if (!containerEl || data.length === 0) return;

		const rect = containerEl.getBoundingClientRect();
		const x = event.clientX - rect.left - padding.left;
		const index = Math.round((x / chartWidth) * (data.length - 1));

		if (index >= 0 && index < data.length) {
			hoveredIndex = index;
			tooltipX = scaleX(index);
			tooltipY = scaleY(data[index].equity_percent);
		}
	}

	function handleMouseLeave() {
		hoveredIndex = null;
	}

	onMount(() => {
		handleResize();
		window.addEventListener('resize', handleResize);
	});

	onDestroy(() => {
		window.removeEventListener('resize', handleResize);
	});
</script>

<div class="equity-curve-chart" role="region" aria-label="Equity curve chart">
	<div class="chart-header">
		<h3 class="chart-title">Equity Curve</h3>
		{#if data.length > 0}
			<span
				class="chart-value"
				class:profit={data[data.length - 1]?.equity_percent >= 0}
				class:loss={data[data.length - 1]?.equity_percent < 0}
			>
				{formatValue(data[data.length - 1]?.equity_percent ?? 0)}
			</span>
		{/if}
	</div>

	{#if isLoading}
		<div class="skeleton-chart">
			<div class="skel-line"></div>
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<p>No equity data available</p>
		</div>
	{:else}
		<div
			bind:this={containerEl}
			class="chart-container"
			onmousemove={handleMouseMove}
			onmouseleave={handleMouseLeave}
			role="img"
			aria-label="Equity curve line chart"
		>
			<svg {width} {height} class="chart-svg">
				<defs>
					<linearGradient id="equityGradient" x1="0%" y1="0%" x2="0%" y2="100%">
						<stop offset="0%" stop-color={lineColor} stop-opacity="0.2" />
						<stop offset="100%" stop-color={lineColor} stop-opacity="0" />
					</linearGradient>
				</defs>

				<!-- Grid lines -->
				{#each yTicks as tick}
					<line
						x1={padding.left}
						y1={tick.y}
						x2={width - padding.right}
						y2={tick.y}
						stroke="#E5E7EB"
						stroke-dasharray="4 4"
					/>
				{/each}

				<!-- Zero line -->
				<line
					x1={padding.left}
					y1={zeroY}
					x2={width - padding.right}
					y2={zeroY}
					stroke="#9CA3AF"
					stroke-width="1"
				/>

				<!-- Y-axis labels -->
				{#each yTicks as tick}
					<text
						x={padding.left - 8}
						y={tick.y}
						text-anchor="end"
						dominant-baseline="middle"
						class="axis-label"
					>
						{formatValue(tick.value)}
					</text>
				{/each}

				<!-- X-axis labels -->
				{#each xLabels as item}
					{@const index = data.indexOf(item)}
					<text x={scaleX(index)} y={height - 8} text-anchor="middle" class="axis-label">
						{formatDate(item.date)}
					</text>
				{/each}

				<!-- Area fill -->
				{#if areaPath}
					<path d={areaPath} fill="url(#equityGradient)" />
				{/if}

				<!-- Line -->
				{#if linePath}
					<path
						d={linePath}
						fill="none"
						stroke={lineColor}
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				{/if}

				<!-- Data points (only show on hover area) -->
				{#if hoveredIndex !== null}
					<line
						x1={tooltipX}
						y1={padding.top}
						x2={tooltipX}
						y2={padding.top + chartHeight}
						stroke={lineColor}
						stroke-width="1"
						stroke-dasharray="4 4"
						opacity="0.5"
					/>
					<circle
						cx={tooltipX}
						cy={tooltipY}
						r="5"
						fill={lineColor}
						stroke="white"
						stroke-width="2"
					/>
				{/if}
			</svg>

			<!-- Tooltip -->
			{#if hoveredIndex !== null}
				{@const point = data[hoveredIndex]}
				<div class="tooltip" style="left: {tooltipX}px; top: {tooltipY - 60}px">
					<div
						class="tooltip-value"
						class:profit={point.equity_percent >= 0}
						class:loss={point.equity_percent < 0}
					>
						{formatValue(point.equity_percent)}
					</div>
					<div class="tooltip-date">{formatDate(point.date)}</div>
					<div class="tooltip-trades">{point.trade_count} trades</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.equity-curve-chart {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.chart-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0;
	}

	.chart-value {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.chart-value.profit {
		color: var(--color-profit);
	}

	.chart-value.loss {
		color: var(--color-loss);
	}

	.chart-container {
		position: relative;
	}

	.chart-svg {
		overflow: visible;
	}

	.axis-label {
		font-size: 11px;
		fill: var(--color-text-tertiary);
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
	}

	.tooltip-value {
		font-size: 16px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		margin-bottom: 2px;
	}

	.tooltip-value.profit {
		color: var(--color-profit);
	}

	.tooltip-value.loss {
		color: var(--color-loss);
	}

	.tooltip-date {
		font-size: 12px;
		color: var(--color-text-secondary);
	}

	.tooltip-trades {
		font-size: 11px;
		color: var(--color-text-tertiary);
	}

	.skeleton-chart {
		height: 280px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.skel-line {
		width: 100%;
		height: 200px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	.empty-state {
		height: 200px;
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
		.equity-curve-chart {
			padding: 16px;
		}
	}
</style>
