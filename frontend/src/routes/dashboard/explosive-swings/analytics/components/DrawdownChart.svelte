<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * DrawdownChart Component - Drawdown Visualization
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays drawdown over time as an area chart
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
	let width = $state(400);
	let hoveredIndex: number | null = $state(null);
	let tooltipX = $state(0);
	let tooltipY = $state(0);

	// Chart dimensions
	const height = 200;
	const padding = { top: 20, right: 20, bottom: 40, left: 50 };

	// Derived dimensions
	const chartWidth = $derived(width - padding.left - padding.right);
	const chartHeight = $derived(height - padding.top - padding.bottom);

	// Calculate scales - drawdown is always negative or zero
	const values = $derived(data.map((d) => d.drawdown_percent));
	const minValue = $derived(Math.min(...values, 0));
	const maxValue = 0; // Drawdown is at most 0

	// Scale functions
	function scaleX(index: number): number {
		return padding.left + (index / Math.max(data.length - 1, 1)) * chartWidth;
	}

	function scaleY(value: number): number {
		const range = Math.abs(minValue) || 1;
		return padding.top + (Math.abs(value) / range) * chartHeight;
	}

	// Generate SVG path for area
	const areaPath = $derived(
		data.length > 0
			? data
					.map((d, i) => {
						const x = scaleX(i);
						const y = scaleY(d.drawdown_percent);
						return i === 0 ? `M ${x} ${padding.top}` : `L ${x} ${padding.top}`;
					})
					.join(' ') +
				' ' +
				data
					.map((d, i) => {
						const x = scaleX(i);
						const y = scaleY(d.drawdown_percent);
						return `L ${x} ${y}`;
					})
					.reverse()
					.join(' ') +
				' Z'
			: ''
	);

	// Line path (bottom edge of area)
	const linePath = $derived(
		data.length > 0
			? data
					.map((d, i) => {
						const x = scaleX(i);
						const y = scaleY(d.drawdown_percent);
						return i === 0 ? `M ${x} ${y}` : `L ${x} ${y}`;
					})
					.join(' ')
			: ''
	);

	// Max drawdown stats
	const maxDrawdown = $derived(Math.min(...values, 0));
	const maxDrawdownIndex = $derived(values.indexOf(maxDrawdown));
	const maxDrawdownPoint = $derived(data[maxDrawdownIndex]);

	// Y-axis ticks
	const yTicks = $derived(
		[0, 0.25, 0.5, 0.75, 1].map((t) => ({
			value: minValue * t,
			y: padding.top + t * chartHeight
		}))
	);

	// Format functions
	function formatPercent(value: number): string {
		return `${value.toFixed(1)}%`;
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

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
			tooltipY = scaleY(data[index].drawdown_percent);
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

<div class="drawdown-chart" role="region" aria-label="Drawdown chart">
	<div class="chart-header">
		<h3 class="chart-title">Drawdown</h3>
		{#if data.length > 0}
			<span class="max-dd">Max: {formatPercent(maxDrawdown)}</span>
		{/if}
	</div>

	{#if isLoading}
		<div class="skeleton-chart">
			<div class="skel-area"></div>
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<p>No drawdown data available</p>
		</div>
	{:else}
		<div
			bind:this={containerEl}
			class="chart-container"
			onmousemove={handleMouseMove}
			onmouseleave={handleMouseLeave}
			role="img"
			aria-label="Drawdown area chart"
		>
			<svg {width} {height} class="chart-svg">
				<defs>
					<linearGradient id="drawdownGradient" x1="0%" y1="0%" x2="0%" y2="100%">
						<stop offset="0%" stop-color="var(--color-loss)" stop-opacity="0.1" />
						<stop offset="100%" stop-color="var(--color-loss)" stop-opacity="0.4" />
					</linearGradient>
				</defs>

				<!-- Y-axis labels -->
				{#each yTicks as tick}
					<line
						x1={padding.left}
						y1={tick.y}
						x2={width - padding.right}
						y2={tick.y}
						stroke="#E5E7EB"
						stroke-dasharray="4 4"
					/>
					<text
						x={padding.left - 8}
						y={tick.y}
						text-anchor="end"
						dominant-baseline="middle"
						class="axis-label"
					>
						{formatPercent(tick.value)}
					</text>
				{/each}

				<!-- Area fill -->
				{#if areaPath}
					<path d={areaPath} fill="url(#drawdownGradient)" />
				{/if}

				<!-- Line -->
				{#if linePath}
					<path
						d={linePath}
						fill="none"
						stroke="var(--color-loss)"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				{/if}

				<!-- Max drawdown marker -->
				{#if maxDrawdownPoint}
					<circle
						cx={scaleX(maxDrawdownIndex)}
						cy={scaleY(maxDrawdown)}
						r="4"
						fill="var(--color-loss)"
						stroke="white"
						stroke-width="2"
					/>
				{/if}

				<!-- Hover indicator -->
				{#if hoveredIndex !== null}
					<line
						x1={tooltipX}
						y1={padding.top}
						x2={tooltipX}
						y2={padding.top + chartHeight}
						stroke="var(--color-loss)"
						stroke-width="1"
						stroke-dasharray="4 4"
						opacity="0.5"
					/>
					<circle
						cx={tooltipX}
						cy={tooltipY}
						r="5"
						fill="var(--color-loss)"
						stroke="white"
						stroke-width="2"
					/>
				{/if}
			</svg>

			<!-- Tooltip -->
			{#if hoveredIndex !== null}
				{@const point = data[hoveredIndex]}
				<div
					class="tooltip"
					style="left: {tooltipX}px; top: {tooltipY + 15}px"
				>
					<div class="tooltip-value">{formatPercent(point.drawdown_percent)}</div>
					<div class="tooltip-date">{formatDate(point.date)}</div>
				</div>
			{/if}
		</div>

		<!-- Summary -->
		<div class="summary-row">
			<div class="summary-stat">
				<span class="stat-label">Max Drawdown</span>
				<span class="stat-value loss">{formatPercent(maxDrawdown)}</span>
			</div>
			{#if maxDrawdownPoint}
				<div class="summary-stat">
					<span class="stat-label">Max DD Date</span>
					<span class="stat-value">{formatDate(maxDrawdownPoint.date)}</span>
				</div>
			{/if}
			<div class="summary-stat">
				<span class="stat-label">Current DD</span>
				<span class="stat-value" class:loss={data[data.length - 1]?.drawdown_percent < 0}>
					{formatPercent(data[data.length - 1]?.drawdown_percent ?? 0)}
				</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.drawdown-chart {
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

	.max-dd {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-loss);
		font-variant-numeric: tabular-nums;
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

	.tooltip {
		position: absolute;
		pointer-events: none;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 8px;
		padding: 8px 12px;
		box-shadow: var(--shadow-md);
		transform: translateX(-50%);
		z-index: 10;
		text-align: center;
	}

	.tooltip-value {
		font-size: 14px;
		font-weight: 700;
		color: var(--color-loss);
		font-variant-numeric: tabular-nums;
	}

	.tooltip-date {
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
		font-size: 14px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		color: var(--color-text-primary);
	}

	.stat-value.loss {
		color: var(--color-loss);
	}

	/* Skeleton */
	.skeleton-chart {
		height: 200px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.skel-area {
		width: 100%;
		height: 140px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	.empty-state {
		height: 160px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
		font-size: 14px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	@media (max-width: 640px) {
		.drawdown-chart {
			padding: 16px;
		}

		.summary-row {
			flex-direction: column;
			gap: 12px;
		}
	}
</style>
