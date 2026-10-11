<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { curveMonotoneX, line } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import type { Axis, AxisDomain } from 'd3-axis';
	import type { Attachment } from 'svelte/attachments';
	import IconLoader2 from '@tabler/icons-svelte-runes/icons/loader-2';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import { formatCurrency, formatPercent } from '../../utils/formatters.js';
	import { getCalculator } from '../../state/context.js';

	const calc = getCalculator();
	const uid = $props.id();

	/** Chart height in px; the width follows the container (`bind:clientWidth`). */
	const HEIGHT = 300;
	const margin = { top: 20, right: 100, bottom: 40, left: 60 };
	const innerH = HEIGHT - margin.top - margin.bottom;
	/** SVG stays responsive with at most this many sample paths drawn. */
	const MAX_DRAWN_PATHS = 200;
	const HIST_BINS = 30;
	const HIST_WIDTH = 50;

	let width = $state(0);

	const stats = $derived(calc.monteCarloResult?.stats);

	/** Scales and path geometry for the latest run; `null` until there is one and it can be drawn. */
	const chart = $derived.by(() => {
		const result = calc.monteCarloResult;
		const innerW = width - margin.left - margin.right;
		const lastTime = result?.timePoints.at(-1);
		if (!result || result.paths.length === 0 || lastTime === undefined || innerW <= 0) {
			return null;
		}

		const { paths, timePoints } = result;
		let minPrice = Infinity;
		let maxPrice = -Infinity;
		for (const path of paths) {
			minPrice = Math.min(minPrice, path.minPrice);
			maxPrice = Math.max(maxPrice, path.maxPrice);
		}
		const yPad = (maxPrice - minPrice) * 0.05;
		const yLow = minPrice - yPad;
		const yHigh = maxPrice + yPad;

		const x = scaleLinear().domain([0, lastTime]).range([0, innerW]);
		const y = scaleLinear().domain([yLow, yHigh]).range([innerH, 0]);
		const pathLine = line<number>()
			.x((_, i) => x(timePoints[i]))
			.y((d) => y(d))
			.curve(curveMonotoneX);

		// Percentile paths by final price.
		const sorted = [...paths].sort((a, b) => a.finalPrice - b.finalPrice);
		const percentiles = [
			{
				label: '5th %ile',
				color: 'var(--calc-put)',
				path: sorted[Math.floor(paths.length * 0.05)]
			},
			{
				label: 'Median',
				color: 'var(--calc-accent)',
				path: sorted[Math.floor(paths.length * 0.5)]
			},
			{
				label: '95th %ile',
				color: 'var(--calc-call)',
				path: sorted[Math.floor(paths.length * 0.95)]
			}
		];

		// Final-price histogram along the right edge.
		const binSize = (yHigh - yLow) / HIST_BINS;
		const counts = Array.from({ length: HIST_BINS }, () => 0);
		for (const path of paths) {
			const bin = Math.min(Math.floor((path.finalPrice - yLow) / binSize), HIST_BINS - 1);
			if (bin >= 0) counts[bin]++;
		}
		const histX = scaleLinear()
			.domain([0, Math.max(...counts)])
			.range([0, HIST_WIDTH]);

		return {
			innerW,
			x,
			y,
			samples: paths.slice(0, MAX_DRAWN_PATHS).map((path) => ({
				id: path.id,
				finalPrice: path.finalPrice,
				d: pathLine(path.prices)
			})),
			highlights: percentiles.map(({ label, color, path }) => ({
				label,
				color,
				finalPrice: path.finalPrice,
				d: pathLine(path.prices)
			})),
			histogram: counts.map((count, bin) => {
				const binLow = yLow + bin * binSize;
				return {
					bin,
					mid: binLow + binSize / 2,
					y: y(binLow + binSize),
					width: histX(count),
					height: Math.max(1, y(binLow) - y(binLow + binSize))
				};
			})
		};
	});

	/** Colour by where a price ends relative to the strike. */
	const sideColor = (price: number) =>
		price > calc.strikePrice ? 'var(--calc-call)' : 'var(--calc-put)';

	/** Lets d3 draw an axis into the `<g>`; re-runs whenever the generator (its scale) changes. */
	function axis<Domain extends AxisDomain>(generator: Axis<Domain>): Attachment<SVGGElement> {
		return (g) => {
			select(g).call(generator);
		};
	}
</script>

{#snippet stat(label: string, value: string, color: string, note = '')}
	<div
		class="flex flex-col gap-0.5 rounded-lg px-3 py-2"
		style="background: var(--calc-surface); border: 1px solid var(--calc-border);"
	>
		<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);"
			>{label}</span
		>
		<span class="text-sm font-semibold" style:color style:font-family="var(--calc-font-mono)"
			>{value}</span
		>
		{#if note}
			<span class="text-[9px]" style="color: var(--calc-text-muted);">{note}</span>
		{/if}
	</div>
{/snippet}

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-3 flex-wrap">
		<button
			type="button"
			onclick={() => calc.runMC()}
			disabled={calc.isMonteCarloRunning}
			class="flex items-center gap-1.5 text-xs font-medium px-3 py-1.5 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer disabled:opacity-50"
			style="background: var(--calc-accent); color: white;"
		>
			{#if calc.isMonteCarloRunning}
				<IconLoader2 size={12} class="motion-safe:animate-spin" aria-hidden="true" />
				Running...
			{:else}
				<IconPlayerPlay size={12} aria-hidden="true" />
				Run Simulation
			{/if}
		</button>

		<div class="flex items-center gap-2">
			<label
				for="{uid}-paths"
				class="text-[10px]"
				style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">Paths:</label
			>
			<!-- The path count is the next run's config; `calc.runMC()` reads it when it starts. -->
			<input
				id="{uid}-paths"
				type="range"
				min="100"
				max="10000"
				step="100"
				bind:value={calc.monteCarloConfig.numPaths}
				class="calc-slider w-24"
			/>
			<span
				class="text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
				>{calc.monteCarloConfig.numPaths.toLocaleString()}</span
			>
		</div>

		{#if calc.monteCarloResult}
			<span
				class="text-[10px]"
				style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
			>
				{calc.monteCarloResult.computeTimeMs.toFixed(0)}ms
			</span>
		{/if}
	</div>

	<!-- Chart -->
	<div class="relative w-full" style="min-height: 300px;" bind:clientWidth={width}>
		{#if !calc.monteCarloResult}
			<div class="flex items-center justify-center h-64" style="color: var(--calc-text-muted);">
				<p class="text-sm">Click "Run Simulation" to generate Monte Carlo paths</p>
			</div>
		{:else}
			<svg
				{width}
				height={HEIGHT}
				role="img"
				aria-label="Monte Carlo simulation: {calc.monteCarloResult.paths.length.toLocaleString()} simulated price paths to expiration"
			>
				{#if chart}
					<g transform="translate({margin.left},{margin.top})">
						<!-- Sample paths -->
						{#each chart.samples as sample (sample.id)}
							<path
								d={sample.d}
								fill="none"
								stroke={sideColor(sample.finalPrice)}
								stroke-width="0.6"
								opacity="0.08"
							/>
						{/each}

						<!-- Percentile paths -->
						{#each chart.highlights as highlight (highlight.label)}
							<path
								d={highlight.d}
								fill="none"
								stroke={highlight.color}
								stroke-width="2"
								opacity="0.9"
							/>
							<text
								x={chart.innerW + 4}
								y={chart.y(highlight.finalPrice)}
								fill={highlight.color}
								font-size="9px"
								font-family="var(--calc-font-mono)"
								dominant-baseline="middle"
								>{highlight.label} ${highlight.finalPrice.toFixed(1)}</text
							>
						{/each}

						<!-- Strike price line -->
						<line
							x1="0"
							x2={chart.innerW}
							y1={chart.y(calc.strikePrice)}
							y2={chart.y(calc.strikePrice)}
							stroke="var(--calc-warning)"
							stroke-width="1"
							stroke-dasharray="6,3"
							opacity="0.7"
						/>
						<text
							x={chart.innerW + 4}
							y={chart.y(calc.strikePrice)}
							fill="var(--calc-warning)"
							font-size="9px"
							font-family="var(--calc-font-mono)"
							dominant-baseline="middle">K=${calc.strikePrice.toFixed(0)}</text
						>

						<!-- Spot price starting dot -->
						<circle cx={chart.x(0)} cy={chart.y(calc.spotPrice)} r="4" fill="var(--calc-text)" />

						<!-- Axes -->
						<g
							class="axis"
							transform="translate(0,{innerH})"
							{@attach axis(
								axisBottom(chart.x)
									.ticks(6)
									.tickFormat((d) => `${(Number(d) * 365).toFixed(0)}d`)
							)}
						></g>
						<g
							class="axis"
							{@attach axis(
								axisLeft(chart.y)
									.ticks(6)
									.tickFormat((d) => `$${Number(d).toFixed(0)}`)
							)}
						></g>

						<!-- Final price histogram on the right edge -->
						{#each chart.histogram as bar (bar.bin)}
							<rect
								x={chart.innerW + 2}
								y={bar.y}
								width={bar.width}
								height={bar.height}
								fill={sideColor(bar.mid)}
								opacity="0.4"
							/>
						{/each}
					</g>
				{/if}
			</svg>
		{/if}
	</div>

	<!-- Stats -->
	{#if stats}
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
			{@render stat('Mean Price', formatCurrency(stats.meanFinalPrice), 'var(--calc-text)')}
			{@render stat('Std Dev', formatCurrency(stats.stdDev), 'var(--calc-text)')}
			{@render stat(
				'MC Call',
				formatCurrency(stats.expectedPayoffCall),
				'var(--calc-call)',
				`BS: ${formatCurrency(stats.bsCallPrice)}`
			)}
			{@render stat(
				'P(Above K)',
				formatPercent(stats.probabilityAboveStrike),
				'var(--calc-accent)'
			)}
		</div>
	{/if}
</div>

<style>
	/* d3-axis writes `font-size`/`font-family` presentation attributes; CSS takes precedence. */
	.axis {
		color: var(--calc-text-muted);
		font-family: var(--calc-font-mono);
		font-size: 10px;
	}
</style>
