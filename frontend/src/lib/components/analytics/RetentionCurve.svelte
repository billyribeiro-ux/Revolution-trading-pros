<!--
	Retention Curve Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user retention over time with cohort comparison.
-->

<script lang="ts">
	import { onMount } from 'svelte';

	interface Cohort {
		name: string;
		color: string;
		data: Array<{ day: number; retention: number }>;
	}

	interface Props {
		cohorts?: Cohort[];
		height?: number;
	}

	let { cohorts = [], height = 300 }: Props = $props();

	let containerEl: HTMLDivElement;
	let width = $state(600);

	onMount(() => {
		if (containerEl) {
			width = containerEl.clientWidth;
		}
	});

	const padding = { top: 20, right: 20, bottom: 40, left: 50 };
	let chartWidth = $derived(width - padding.left - padding.right);
	let chartHeight = $derived(height - padding.top - padding.bottom);
	let maxDay = $derived(Math.max(...cohorts.flatMap((c) => c.data.map((d) => d.day))));

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
			{#each cohorts as cohort (cohort.name)}
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
			{#each [0, 25, 50, 75, 100] as tick (tick)}
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
			{#each [0, 7, 14, 30, 60, 90] as day (day)}
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
			{#each cohorts as cohort (cohort.name)}
				<path
					d={generatePath(cohort.data)}
					fill="none"
					stroke={cohort.color}
					stroke-width="3"
					class="retention-line"
				/>

				<!-- Data points -->
				{#each cohort.data as point (point.day)}
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
		{#each cohorts as cohort (cohort.name)}
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

<style>
	.retention-curve {
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.curve-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.curve-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.curve-legend {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.legend-color {
		inline-size: 0.75rem;
		block-size: 0.75rem;
		border-radius: 9999px;
	}

	.legend-label {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
	}

	.curve-chart {
		margin-block-end: var(--space-4);
	}

	.axis-label {
		font-size: var(--text-xs);
		fill: oklch(0.65 0.01 250);
	}

	.retention-line {
		transition: stroke-width var(--duration-fast) var(--ease-default);
		&:hover {
			stroke-width: 4;
		}
	}

	.data-point {
		cursor: pointer;
		transition: transform var(--duration-fast) var(--ease-default);
		&:hover {
			transform: scale(1.25);
		}
	}

	.curve-insights {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-3);
		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.insight-item {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-3);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.insight-cohort {
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-2);
		padding-inline-start: var(--space-3);
		border-inline-start: 4px solid;
	}

	.insight-metrics {
		display: flex;
		gap: var(--space-4);
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
	}

	.metric {
		font-weight: var(--weight-medium);
	}
</style>
