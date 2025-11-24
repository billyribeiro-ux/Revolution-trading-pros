<!--
	Retention Curve Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user retention over time with cohort comparison.
-->

<script lang="ts">
	import { onMount } from 'svelte';

	export let cohorts: Array<{
		name: string;
		color: string;
		data: Array<{ day: number; retention: number }>;
	}> = [];

	export let height = 300;

	let containerEl: HTMLDivElement;
	let width = 600;

	onMount(() => {
		if (containerEl) {
			width = containerEl.clientWidth;
		}
	});

	$: padding = { top: 20, right: 20, bottom: 40, left: 50 };
	$: chartWidth = width - padding.left - padding.right;
	$: chartHeight = height - padding.top - padding.bottom;
	$: maxDay = Math.max(...cohorts.flatMap((c) => c.data.map((d) => d.day)));

	// Scale functions - computed from reactive values
	function getXScale(day: number, maxDay: number, chartWidth: number): number {
		return (day / maxDay) * chartWidth;
	}

	function getYScale(retention: number, chartHeight: number): number {
		return chartHeight - (retention / 100) * chartHeight;
	}

	function generatePath(data: Array<{ day: number; retention: number }>): string {
		if (data.length === 0) return '';

		const points = data.map(
			(d) => `${getXScale(d.day, maxDay, chartWidth)},${getYScale(d.retention, chartHeight)}`
		);
		return `M ${points.join(' L ')}`;
	}

	function formatDay(day: number): string {
		if (day === 0) return 'Day 0';
		if (day === 1) return 'Day 1';
		if (day === 7) return 'Week 1';
		if (day === 30) return 'Month 1';
		if (day === 90) return 'Month 3';
		return `Day ${day}`;
	}
</script>

<div class="retention-curve" bind:this={containerEl}>
	<div class="curve-header">
		<h3 class="curve-title">Retention Curve</h3>
		<div class="curve-legend">
			{#each cohorts as cohort}
				<div class="legend-item">
					<div class="legend-color" style="background-color: {cohort.color}"></div>
					<span class="legend-label">{cohort.name}</span>
				</div>
			{/each}
		</div>
	</div>

	<svg {width} {height} class="curve-chart">
		<g transform="translate({padding.left}, {padding.top})">
			<!-- Grid lines -->
			{#each [0, 25, 50, 75, 100] as tick}
				<line
					x1="0"
					y1={getYScale(tick, chartHeight)}
					x2={chartWidth}
					y2={getYScale(tick, chartHeight)}
					stroke="#374151"
					stroke-width="1"
					stroke-dasharray="4"
				/>
				<text
					x="-10"
					y={getYScale(tick, chartHeight)}
					text-anchor="end"
					dominant-baseline="middle"
					class="axis-label"
				>
					{tick}%
				</text>
			{/each}

			<!-- X-axis labels -->
			{#each [0, 7, 14, 30, 60, 90] as day}
				{#if day <= maxDay}
					<text
						x={getXScale(day, maxDay, chartWidth)}
						y={chartHeight + 20}
						text-anchor="middle"
						class="axis-label"
					>
						{formatDay(day)}
					</text>
				{/if}
			{/each}

			<!-- Retention curves -->
			{#each cohorts as cohort}
				<path
					d={generatePath(cohort.data)}
					fill="none"
					stroke={cohort.color}
					stroke-width="3"
					class="retention-line"
				/>

				<!-- Data points -->
				{#each cohort.data as point}
					<circle
						cx={getXScale(point.day, maxDay, chartWidth)}
						cy={getYScale(point.retention, chartHeight)}
						r="4"
						fill={cohort.color}
						class="data-point"
					>
						<title>{cohort.name}: Day {point.day} - {point.retention.toFixed(1)}%</title>
					</circle>
				{/each}
			{/each}
		</g>
	</svg>

	<!-- Insights -->
	<div class="curve-insights">
		{#each cohorts as cohort}
			{@const day30 = cohort.data.find((d) => d.day === 30)}
			{@const day90 = cohort.data.find((d) => d.day === 90)}
			<div class="insight-item">
				<div class="insight-cohort" style="border-left-color: {cohort.color}">
					{cohort.name}
				</div>
				<div class="insight-metrics">
					{#if day30}
						<span class="metric">30-day: {day30.retention.toFixed(1)}%</span>
					{/if}
					{#if day90}
						<span class="metric">90-day: {day90.retention.toFixed(1)}%</span>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</div>

<style lang="postcss">
	@reference "../../../app.css";
	.retention-curve {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.curve-header {
		@apply flex items-center justify-between mb-6;
	}

	.curve-title {
		@apply text-xl font-bold text-white;
	}

	.curve-legend {
		@apply flex items-center gap-4;
	}

	.legend-item {
		@apply flex items-center gap-2;
	}

	.legend-color {
		@apply w-3 h-3 rounded-full;
	}

	.legend-label {
		@apply text-sm text-gray-400;
	}

	.curve-chart {
		@apply mb-4;
	}

	.axis-label {
		@apply text-xs fill-gray-400;
	}

	.retention-line {
		@apply transition-all;
	}

	.retention-line:hover {
		@apply stroke-[4];
	}

	.data-point {
		@apply cursor-pointer transition-all;
	}

	.data-point:hover {
		@apply scale-125;
	}

	.curve-insights {
		@apply grid grid-cols-1 md:grid-cols-2 gap-3;
	}

	.insight-item {
		@apply bg-gray-900/50 rounded-lg p-3 border border-gray-700/50;
	}

	.insight-cohort {
		@apply font-semibold text-white mb-2 pl-3 border-l-4;
	}

	.insight-metrics {
		@apply flex gap-4 text-sm text-gray-400;
	}

	.metric {
		@apply font-medium;
	}
</style>
