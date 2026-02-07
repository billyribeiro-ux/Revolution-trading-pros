<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { line as d3line, curveMonotoneX } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import { Loader2, Play } from '@lucide/svelte';
	import { formatCurrency, formatPercent } from '../../utils/formatters.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { MonteCarloPath } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let svgEl: SVGSVGElement | undefined = $state();
	let width = $state(600);
	let height = $state(350);
	let pathCount = $state(2000);

	const margin = { top: 20, right: 100, bottom: 40, left: 60 };

	$effect(() => {
		if (!containerEl) return;
		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				width = entry.contentRect.width;
				height = Math.max(300, Math.min(420, entry.contentRect.height));
			}
		});
		observer.observe(containerEl);
		return () => observer.disconnect();
	});

	function handleRun() {
		calc.monteCarloConfig = { numPaths: pathCount, numSteps: 100 };
		calc.runMC();
	}

	$effect(() => {
		const result = calc.monteCarloResult;
		if (!svgEl || !result || result.paths.length === 0) return;

		const innerW = width - margin.left - margin.right;
		const innerH = height - margin.top - margin.bottom;
		if (innerW <= 0 || innerH <= 0) return;

		const svg = select(svgEl);
		svg.selectAll('*').remove();

		const g = svg
			.attr('width', width)
			.attr('height', height)
			.append('g')
			.attr('transform', `translate(${margin.left},${margin.top})`);

		const timePoints = result.timePoints;
		const paths = result.paths;

		// Find price range across all paths
		let minPrice = Infinity;
		let maxPrice = -Infinity;
		for (const path of paths) {
			minPrice = Math.min(minPrice, path.minPrice);
			maxPrice = Math.max(maxPrice, path.maxPrice);
		}
		const yPad = (maxPrice - minPrice) * 0.05;

		const x = scaleLinear().domain([0, timePoints[timePoints.length - 1]]).range([0, innerW]);
		const y = scaleLinear().domain([minPrice - yPad, maxPrice + yPad]).range([innerH, 0]);

		// Draw paths (limit to 200 for SVG performance)
		const displayPaths = paths.slice(0, Math.min(200, paths.length));
		const pathGen = d3line<number>()
			.x((_, i) => x(timePoints[i]))
			.y((d) => y(d))
			.curve(curveMonotoneX);

		for (const path of displayPaths) {
			g.append('path')
				.datum(path.prices)
				.attr('d', pathGen)
				.attr('fill', 'none')
				.attr('stroke', path.finalPrice > calc.strikePrice ? 'var(--calc-call)' : 'var(--calc-put)')
				.attr('stroke-width', 0.6)
				.attr('opacity', 0.08);
		}

		// Highlight percentile paths
		const sortedByFinal = [...paths].sort((a, b) => a.finalPrice - b.finalPrice);
		const p5Path = sortedByFinal[Math.floor(paths.length * 0.05)];
		const medianPath = sortedByFinal[Math.floor(paths.length * 0.5)];
		const p95Path = sortedByFinal[Math.floor(paths.length * 0.95)];

		const highlights: { path: MonteCarloPath; color: string; label: string }[] = [
			{ path: p5Path, color: 'var(--calc-put)', label: '5th %ile' },
			{ path: medianPath, color: 'var(--calc-accent)', label: 'Median' },
			{ path: p95Path, color: 'var(--calc-call)', label: '95th %ile' },
		];

		for (const hl of highlights) {
			g.append('path')
				.datum(hl.path.prices)
				.attr('d', pathGen)
				.attr('fill', 'none')
				.attr('stroke', hl.color)
				.attr('stroke-width', 2)
				.attr('opacity', 0.9);

			g.append('text')
				.attr('x', innerW + 4)
				.attr('y', y(hl.path.finalPrice))
				.attr('fill', hl.color)
				.attr('font-size', '9px')
				.attr('font-family', 'var(--calc-font-mono)')
				.attr('dominant-baseline', 'middle')
				.text(`${hl.label} $${hl.path.finalPrice.toFixed(1)}`);
		}

		// Strike price line
		g.append('line')
			.attr('x1', 0).attr('x2', innerW)
			.attr('y1', y(calc.strikePrice)).attr('y2', y(calc.strikePrice))
			.attr('stroke', 'var(--calc-warning)')
			.attr('stroke-width', 1).attr('stroke-dasharray', '6,3').attr('opacity', 0.7);

		g.append('text')
			.attr('x', innerW + 4).attr('y', y(calc.strikePrice))
			.attr('fill', 'var(--calc-warning)')
			.attr('font-size', '9px').attr('font-family', 'var(--calc-font-mono)')
			.attr('dominant-baseline', 'middle')
			.text(`K=$${calc.strikePrice.toFixed(0)}`);

		// Spot price starting dot
		g.append('circle')
			.attr('cx', x(0)).attr('cy', y(calc.spotPrice))
			.attr('r', 4).attr('fill', 'var(--calc-text)');

		// Axes
		const xAxisGen = axisBottom(x).ticks(6).tickFormat((d) => `${(Number(d) * 365).toFixed(0)}d`);
		const yAxisGen = axisLeft(y).ticks(6).tickFormat((d) => `$${Number(d).toFixed(0)}`);

		g.append('g')
			.attr('transform', `translate(0,${innerH})`)
			.call(xAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)').attr('font-size', '10px');

		g.append('g')
			.call(yAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)').attr('font-size', '10px');

		// Final price histogram on right edge
		const histBins = 30;
		const histW = 50;
		const binSize = (maxPrice + yPad - (minPrice - yPad)) / histBins;
		const bins = new Array(histBins).fill(0);
		for (const fp of paths.map((p) => p.finalPrice)) {
			const idx = Math.min(Math.floor((fp - (minPrice - yPad)) / binSize), histBins - 1);
			if (idx >= 0) bins[idx]++;
		}
		const maxBin = Math.max(...bins);
		const histX = scaleLinear().domain([0, maxBin]).range([0, histW]);

		for (let i = 0; i < histBins; i++) {
			const binY = minPrice - yPad + i * binSize;
			const barW = histX(bins[i]);
			g.append('rect')
				.attr('x', innerW + 2)
				.attr('y', y(binY + binSize))
				.attr('width', barW)
				.attr('height', Math.max(1, y(binY) - y(binY + binSize)))
				.attr('fill', binY + binSize / 2 > calc.strikePrice ? 'var(--calc-call)' : 'var(--calc-put)')
				.attr('opacity', 0.4);
		}
	});
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-3 flex-wrap">
		<button
			onclick={handleRun}
			disabled={calc.isMonteCarloRunning}
			class="flex items-center gap-1.5 text-xs font-medium px-3 py-1.5 rounded-lg transition-colors cursor-pointer disabled:opacity-50"
			style="background: var(--calc-accent); color: white;"
		>
			{#if calc.isMonteCarloRunning}
				<Loader2 size={12} class="animate-spin" />
				Running...
			{:else}
				<Play size={12} />
				Run Simulation
			{/if}
		</button>

		<div class="flex items-center gap-2">
			<span class="text-[10px]" style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">Paths:</span>
			<input
				type="range"
				min="100"
				max="10000"
				step="100"
				bind:value={pathCount}
				class="calc-slider w-24"
			/>
			<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{pathCount.toLocaleString()}</span>
		</div>

		{#if calc.monteCarloResult}
			<span class="text-[10px]" style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);">
				{calc.monteCarloResult.computeTimeMs.toFixed(0)}ms
			</span>
		{/if}
	</div>

	<!-- Chart -->
	<div bind:this={containerEl} class="relative w-full" style="min-height: 300px;">
		{#if !calc.monteCarloResult}
			<div class="flex items-center justify-center h-64" style="color: var(--calc-text-muted);">
				<p class="text-sm">Click "Run Simulation" to generate Monte Carlo paths</p>
			</div>
		{:else}
			<svg bind:this={svgEl}></svg>
		{/if}
	</div>

	<!-- Stats -->
	{#if calc.monteCarloResult}
		{@const s = calc.monteCarloResult.stats}
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
			<div class="flex flex-col gap-0.5 rounded-lg px-3 py-2" style="background: var(--calc-surface); border: 1px solid var(--calc-border);">
				<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">Mean Price</span>
				<span class="text-sm font-semibold" style="color: var(--calc-text); font-family: var(--calc-font-mono);">{formatCurrency(s.meanFinalPrice)}</span>
			</div>
			<div class="flex flex-col gap-0.5 rounded-lg px-3 py-2" style="background: var(--calc-surface); border: 1px solid var(--calc-border);">
				<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">Std Dev</span>
				<span class="text-sm font-semibold" style="color: var(--calc-text); font-family: var(--calc-font-mono);">{formatCurrency(s.stdDev)}</span>
			</div>
			<div class="flex flex-col gap-0.5 rounded-lg px-3 py-2" style="background: var(--calc-surface); border: 1px solid var(--calc-border);">
				<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">MC Call</span>
				<span class="text-sm font-semibold" style="color: var(--calc-call); font-family: var(--calc-font-mono);">{formatCurrency(s.expectedPayoffCall)}</span>
				<span class="text-[9px]" style="color: var(--calc-text-muted);">BS: {formatCurrency(s.bsCallPrice)}</span>
			</div>
			<div class="flex flex-col gap-0.5 rounded-lg px-3 py-2" style="background: var(--calc-surface); border: 1px solid var(--calc-border);">
				<span class="text-[10px] uppercase tracking-wider" style="color: var(--calc-text-muted);">P(Above K)</span>
				<span class="text-sm font-semibold" style="color: var(--calc-accent); font-family: var(--calc-font-mono);">{formatPercent(s.probabilityAboveStrike)}</span>
			</div>
		</div>
	{/if}
</div>
