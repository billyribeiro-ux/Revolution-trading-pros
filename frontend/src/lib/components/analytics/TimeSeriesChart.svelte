<script lang="ts">
	/**
	 * TimeSeriesChart - Enterprise Time Series Visualization
	 *
	 * Displays time series data with support for multiple metrics,
	 * tooltips, and responsive design.
	 */
	import type { Attachment } from 'svelte/attachments';

	interface DataPoint {
		date: string;
		value: number;
		label?: string;
	}

	interface Props {
		data?: DataPoint[];
		title?: string;
		color?: string;
		fillOpacity?: number;
		showArea?: boolean;
		showPoints?: boolean;
		height?: number;
		formatValue?: (val: number) => string;
		formatDate?: (date: string) => string;
	}

	let {
		data = [],
		title = '',
		color = '#3B82F6',
		fillOpacity = 0.1,
		showArea = true,
		showPoints = false,
		height = 200,
		formatValue = (v) => v.toLocaleString(),
		formatDate = (d) => new Date(d).toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
	}: Props = $props();

	let containerEl: HTMLDivElement | null = null;
	let width = $state(600);
	let hoveredIndex: number | null = $state(null);
	let tooltipX = $state(0);
	let tooltipY = $state(0);

	// Padding
	const padding = { top: 20, right: 20, bottom: 30, left: 50 };

	// Calculate chart dimensions
	let chartWidth = $derived(width - padding.left - padding.right);
	let chartHeight = $derived(height - padding.top - padding.bottom);

	// Calculate scales
	let values = $derived(data.map((d) => d.value));
	let minValue = $derived(Math.min(...values, 0));
	let maxValue = $derived(values.length === 0 ? 0 : Math.max(...values, 0));
	let valueRange = $derived(maxValue - minValue || 1);

	// Scale functions
	function scaleX(index: number): number {
		return padding.left + (index / (data.length - 1 || 1)) * chartWidth;
	}

	function scaleY(value: number): number {
		return padding.top + chartHeight - ((value - minValue) / valueRange) * chartHeight;
	}

	// Generate SVG path for line
	let linePath = $derived(
		data
			.map((d, i) => {
				const x = scaleX(i);
				const y = scaleY(d.value);
				return i === 0 ? `M ${x} ${y}` : `L ${x} ${y}`;
			})
			.join(' ')
	);

	// Generate SVG path for area
	let areaPath = $derived(
		data.length > 1
			? linePath +
					` L ${scaleX(data.length - 1)} ${padding.top + chartHeight}` +
					` L ${padding.left} ${padding.top + chartHeight}` +
					` Z`
			: ''
	);

	// Y-axis ticks
	let yTicks = $derived(
		[0, 0.25, 0.5, 0.75, 1].map((t) => ({
			value: minValue + t * valueRange,
			y: scaleY(minValue + t * valueRange)
		}))
	);

	// X-axis labels (show ~5 labels)
	let xLabels = $derived(
		data.filter((_, i) => i % Math.ceil(data.length / 5) === 0 || i === data.length - 1)
	);

	const captureContainer: Attachment<HTMLDivElement> = (element) => {
		containerEl = element;

		const resize = () => {
			width = element.clientWidth || 600;
		};

		resize();

		const observer = new ResizeObserver(resize);
		observer.observe(element);

		return () => {
			observer.disconnect();
			if (containerEl === element) containerEl = null;
		};
	};

	// Handle mouse events
	function handleMouseMove(event: MouseEvent) {
		if (!containerEl) return;

		const rect = containerEl.getBoundingClientRect();
		const x = event.clientX - rect.left - padding.left;
		const index = Math.round((x / chartWidth) * (data.length - 1));

		if (index >= 0 && index < data.length) {
			hoveredIndex = index;
			tooltipX = scaleX(index);
			tooltipY = scaleY(data[index].value);
		}
	}

	function handleMouseLeave() {
		hoveredIndex = null;
	}
</script>

<div class="time-series-chart">
	{#if title}
		<h3>{title}</h3>
	{/if}

	<div
		{@attach captureContainer}
		class="time-series-chart__plot"
		onmousemove={handleMouseMove}
		onmouseleave={handleMouseLeave}
		role="img"
		aria-label="Time series chart"
	>
		<svg aria-hidden="true" {width} {height} class="time-series-chart__svg">
			<!-- Grid lines -->
			{#each yTicks as tick (tick.value)}
				<line
					x1={padding.left}
					y1={tick.y}
					x2={width - padding.right}
					y2={tick.y}
					stroke="#E5E7EB"
					stroke-dasharray="4 4"
				/>
			{/each}

			<!-- Y-axis labels -->
			{#each yTicks as tick (tick.value)}
				<text
					x={padding.left - 8}
					y={tick.y}
					text-anchor="end"
					dominant-baseline="middle"
					class="time-series-chart__axis-label"
				>
					{formatValue(tick.value)}
				</text>
			{/each}

			<!-- X-axis labels -->
			{#each xLabels as item (item.date)}
				{@const index = data.indexOf(item)}
				<text
					x={scaleX(index)}
					y={height - 8}
					text-anchor="middle"
					class="time-series-chart__axis-label"
				>
					{formatDate(item.date)}
				</text>
			{/each}

			<!-- Area fill -->
			{#if showArea && data.length > 1}
				<path d={areaPath} fill={color} fill-opacity={fillOpacity} />
			{/if}

			<!-- Line -->
			{#if data.length > 1}
				<path
					d={linePath}
					fill="none"
					stroke={color}
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
			{/if}

			<!-- Points -->
			{#if showPoints}
				{#each data as point, i (i)}
					<circle
						cx={scaleX(i)}
						cy={scaleY(point.value)}
						r="3"
						fill={color}
						stroke="white"
						stroke-width="2"
					/>
				{/each}
			{/if}

			<!-- Hover indicator -->
			{#if hoveredIndex !== null}
				<line
					x1={tooltipX}
					y1={padding.top}
					x2={tooltipX}
					y2={padding.top + chartHeight}
					stroke={color}
					stroke-width="1"
					stroke-dasharray="4 4"
					opacity="0.5"
				/>
				<circle cx={tooltipX} cy={tooltipY} r="5" fill={color} stroke="white" stroke-width="2" />
			{/if}
		</svg>

		<!-- Tooltip -->
		{#if hoveredIndex !== null}
			{@const point = data[hoveredIndex]}
			<div
				class="time-series-chart__tooltip"
				style:left={`${tooltipX}px`}
				style:top={`${tooltipY - 50}px`}
			>
				<div class="time-series-chart__tooltip-value">{formatValue(point.value)}</div>
				<div class="time-series-chart__tooltip-date">{formatDate(point.date)}</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.time-series-chart {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		padding: 1rem;
	}

	.time-series-chart h3 {
		margin: 0 0 1rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.time-series-chart__plot {
		position: relative;
	}

	.time-series-chart__svg {
		overflow: visible;
	}

	.time-series-chart__axis-label {
		fill: #6b7280;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.time-series-chart__tooltip {
		position: absolute;
		z-index: 10;
		pointer-events: none;
		transform: translateX(-50%);
		border-radius: 0.5rem;
		background: #111827;
		box-shadow:
			0 10px 15px -3px rgba(17, 24, 39, 0.2),
			0 4px 6px -4px rgba(17, 24, 39, 0.2);
		color: #ffffff;
		font-size: 0.875rem;
		line-height: 1.25rem;
		padding: 0.5rem 0.75rem;
	}

	.time-series-chart__tooltip-value {
		font-weight: 500;
	}

	.time-series-chart__tooltip-date {
		color: #9ca3af;
		font-size: 0.75rem;
		line-height: 1rem;
	}
</style>
