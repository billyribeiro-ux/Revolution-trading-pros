<!--
	Retention Curve Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user retention over time with cohort comparison.
-->

<script lang="ts">
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

	let width = $state(600);

	const padding = { top: 20, right: 20, bottom: 40, left: 50 };
	let chartWidth = $derived(width - padding.left - padding.right);
	let chartHeight = $derived(height - padding.top - padding.bottom);
	let maxDay = $derived.by(() => {
		const days = cohorts.flatMap((c) => c.data.map((d) => d.day));
		return days.length > 0 ? Math.max(...days) : 1;
	});

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

	function measureContainer(node: HTMLDivElement) {
		const updateWidth = () => {
			width = node.clientWidth;
		};

		updateWidth();

		const observer = new ResizeObserver(updateWidth);
		observer.observe(node);

		return () => {
			observer.disconnect();
		};
	}
</script>

<div class="retention-curve" {@attach measureContainer}>
	<div class="curve-header">
		<h3 class="curve-title">Retention Curve</h3>
		<div class="curve-legend">
			{#each cohorts as cohort (cohort.name ?? cohort)}
				<div class="legend-item">
					<div class="legend-color" style:background-color={cohort.color}></div>
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
			{#each cohorts as cohort (cohort.name ?? cohort)}
				<path
					d={generatePath(cohort.data)}
					fill="none"
					stroke={cohort.color}
					stroke-width="3"
					class="retention-line"
				/>

				<!-- Data points -->
				{#each cohort.data as point (point.day ?? point)}
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
		{#each cohorts as cohort (cohort.name ?? cohort)}
			{const day30 = cohort.data.find((d) => d.day === 30)}
			{const day90 = cohort.data.find((d) => d.day === 90)}
			<div class="insight-item">
				<div class="insight-cohort" style:border-left-color={cohort.color}>
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
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.curve-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.curve-title {
		margin: 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 700;
		color: #ffffff;
	}

	.curve-legend {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.legend-color {
		width: 0.75rem;
		height: 0.75rem;
		border-radius: 999px;
	}

	.legend-label {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.curve-chart {
		margin-bottom: 1rem;
	}

	.axis-label {
		fill: #9ca3af;
		font-size: 0.75rem;
	}

	.retention-line {
		transition: stroke-width 0.2s ease;
	}

	.retention-line:hover {
		stroke-width: 4;
	}

	.data-point {
		cursor: pointer;
		transition: transform 0.2s ease;
		transform-origin: center;
	}

	.data-point:hover {
		transform: scale(1.25);
	}

	.curve-insights {
		display: grid;
		grid-template-columns: 1fr;
		gap: 0.75rem;
	}

	.insight-item {
		padding: 0.75rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.insight-cohort {
		margin-bottom: 0.5rem;
		padding-left: 0.75rem;
		border-left: 4px solid;
		color: #ffffff;
		font-weight: 600;
	}

	.insight-metrics {
		display: flex;
		gap: 1rem;
		color: #9ca3af;
		font-size: 0.875rem;
	}

	.metric {
		font-weight: 500;
	}

	@media (min-width: 768px) {
		.curve-insights {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.curve-header {
			align-items: flex-start;
			flex-direction: column;
		}
	}
</style>
