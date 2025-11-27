<script lang="ts">
	/**
	 * TimeSeriesChart - Enterprise Time Series Visualization
	 *
	 * Displays time series data with support for multiple metrics,
	 * tooltips, and responsive design.
	 */
	import { onMount, onDestroy } from 'svelte';

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

	let containerEl: HTMLDivElement;
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
	let maxValue = $derived(Math.max(...values));
	let valueRange = $derived(maxValue - minValue || 1);

	// Scale functions
	function scaleX(index: number): number {
		return padding.left + (index / (data.length - 1 || 1)) * chartWidth;
	}

	function scaleY(value: number): number {
		return padding.top + chartHeight - ((value - minValue) / valueRange) * chartHeight;
	}

	// Generate SVG path for line
	let linePath = $derived(data
		.map((d, i) => {
			const x = scaleX(i);
			const y = scaleY(d.value);
			return i === 0 ? `M ${x} ${y}` : `L ${x} ${y}`;
		})
		.join(' '));

	// Generate SVG path for area
	let areaPath = $derived(
		linePath +
		` L ${scaleX(data.length - 1)} ${padding.top + chartHeight}` +
		` L ${padding.left} ${padding.top + chartHeight}` +
		` Z`);

	// Y-axis ticks
	let yTicks = $derived([0, 0.25, 0.5, 0.75, 1].map((t) => ({
		value: minValue + t * valueRange,
		y: scaleY(minValue + t * valueRange)
	})));

	// X-axis labels (show ~5 labels)
	let xLabels = $derived(data.filter((_, i) => i % Math.ceil(data.length / 5) === 0 || i === data.length - 1));

	// Handle resize
	function handleResize() {
		if (containerEl) {
			width = containerEl.clientWidth;
		}
	}

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

	onMount(() => {
		handleResize();
		window.addEventListener('resize', handleResize);
	});

	onDestroy(() => {
		window.removeEventListener('resize', handleResize);
	});
</script>

<div class="bg-white rounded-xl border border-gray-200 p-4">
	{#if title}
		<h3 class="text-lg font-semibold text-gray-900 mb-4">{title}</h3>
	{/if}

	<div
		bind:this={containerEl}
		class="relative"
		onmousemove={handleMouseMove}
		onmouseleave={handleMouseLeave}
		role="img"
		aria-label="Time series chart"
	>
		<svg {width} {height} class="overflow-visible">
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

			<!-- Y-axis labels -->
			{#each yTicks as tick}
				<text
					x={padding.left - 8}
					y={tick.y}
					text-anchor="end"
					dominant-baseline="middle"
					class="text-xs fill-gray-500"
				>
					{formatValue(tick.value)}
				</text>
			{/each}

			<!-- X-axis labels -->
			{#each xLabels as item, i}
				{@const index = data.indexOf(item)}
				<text x={scaleX(index)} y={height - 8} text-anchor="middle" class="text-xs fill-gray-500">
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
				{#each data as point, i}
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
				class="absolute pointer-events-none bg-gray-900 text-white px-3 py-2 rounded-lg text-sm shadow-lg z-10 transform -translate-x-1/2"
				style="left: {tooltipX}px; top: {tooltipY - 50}px"
			>
				<div class="font-medium">{formatValue(point.value)}</div>
				<div class="text-gray-400 text-xs">{formatDate(point.date)}</div>
			</div>
		{/if}
	</div>
</div>
