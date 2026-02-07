<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear, scaleSequential } from 'd3-scale';
	import { interpolateRdYlGn } from 'd3-scale-chromatic';
	import { axisBottom, axisLeft } from 'd3-axis';
	import { greeksMatrix } from '../../engine/greeks.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { HeatmapGreek } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let svgEl: SVGSVGElement | undefined = $state();
	let tooltipEl: HTMLDivElement | undefined = $state();
	let width = $state(600);
	let height = $state(350);

	const margin = { top: 30, right: 80, bottom: 50, left: 70 };

	const GREEK_OPTIONS: { value: HeatmapGreek; label: string }[] = [
		{ value: 'delta', label: 'Delta' },
		{ value: 'gamma', label: 'Gamma' },
		{ value: 'theta', label: 'Theta' },
		{ value: 'vega', label: 'Vega' },
		{ value: 'rho', label: 'Rho' },
		{ value: 'charm', label: 'Charm' },
		{ value: 'vanna', label: 'Vanna' },
		{ value: 'volga', label: 'Volga' },
	];

	$effect(() => {
		if (!containerEl) return;
		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				width = entry.contentRect.width;
				height = Math.max(280, Math.min(400, entry.contentRect.height));
			}
		});
		observer.observe(containerEl);
		return () => observer.disconnect();
	});

	$effect(() => {
		if (!svgEl) return;

		const spotPrice = calc.spotPrice;
		const optionType = calc.optionType;
		const greekName = calc.heatmapGreek;
		const inputs = calc.inputs;

		const innerW = width - margin.left - margin.right;
		const innerH = height - margin.top - margin.bottom;
		if (innerW <= 0 || innerH <= 0) return;

		// Generate strike range: ±30% around spot
		const strikeCount = 20;
		const strikeLow = spotPrice * 0.7;
		const strikeHigh = spotPrice * 1.3;
		const strikeStep = (strikeHigh - strikeLow) / strikeCount;
		const strikes = Array.from({ length: strikeCount + 1 }, (_, i) => strikeLow + i * strikeStep);

		// Generate expiry range: 1 day to current expiry
		const expiryCount = 15;
		const expiryMin = 1 / 365;
		const expiryMax = Math.max(inputs.timeToExpiry, 0.01);
		const expiryStep = (expiryMax - expiryMin) / expiryCount;
		const expiries = Array.from({ length: expiryCount + 1 }, (_, i) => expiryMin + i * expiryStep);

		// Calculate matrix
		const matrixData = greeksMatrix(inputs, optionType, greekName, strikes, expiries);

		const svg = select(svgEl);
		svg.selectAll('*').remove();

		const g = svg
			.attr('width', width)
			.attr('height', height)
			.append('g')
			.attr('transform', `translate(${margin.left},${margin.top})`);

		// Scales
		const x = scaleLinear().domain([strikeLow, strikeHigh]).range([0, innerW]);
		const y = scaleLinear().domain([expiryMin, expiryMax]).range([innerH, 0]);

		// Color scale
		const values = matrixData.map((d) => d.value);
		const minVal = Math.min(...values);
		const maxVal = Math.max(...values);
		const absMax = Math.max(Math.abs(minVal), Math.abs(maxVal)) || 1;

		const colorScale = scaleSequential(interpolateRdYlGn).domain([-absMax, absMax]);

		// Cell dimensions
		const cellW = innerW / strikeCount;
		const cellH = innerH / expiryCount;

		// Draw cells
		for (const cell of matrixData) {
			const cx = x(cell.strike) - cellW / 2;
			const cy = y(cell.expiry) - cellH / 2;

			g.append('rect')
				.attr('x', cx)
				.attr('y', cy)
				.attr('width', cellW + 1)
				.attr('height', cellH + 1)
				.attr('fill', colorScale(cell.value))
				.attr('opacity', 0.85)
				.on('mouseenter', (event: MouseEvent) => {
					select(event.currentTarget as SVGRectElement).attr('stroke', 'var(--calc-text)').attr('stroke-width', 2);
					if (tooltipEl) {
						tooltipEl.style.display = 'block';
						tooltipEl.style.left = `${event.offsetX + 12}px`;
						tooltipEl.style.top = `${event.offsetY - 40}px`;
						tooltipEl.innerHTML = `
							<div style="font-family: var(--calc-font-mono); font-size: 11px;">
								<div style="color: var(--calc-text-secondary);">Strike: $${cell.strike.toFixed(1)}</div>
								<div style="color: var(--calc-text-secondary);">Expiry: ${(cell.expiry * 365).toFixed(0)}d</div>
								<div style="color: var(--calc-text); font-weight: 600;">${greekName}: ${cell.value.toFixed(6)}</div>
							</div>
						`;
					}
				})
				.on('mouseleave', (event: MouseEvent) => {
					select(event.currentTarget as SVGRectElement).attr('stroke', 'none');
					if (tooltipEl) tooltipEl.style.display = 'none';
				});
		}

		// Current position marker
		const currentX = x(inputs.strikePrice);
		const currentY = y(inputs.timeToExpiry);
		if (currentX >= 0 && currentX <= innerW && currentY >= 0 && currentY <= innerH) {
			g.append('circle')
				.attr('cx', currentX)
				.attr('cy', currentY)
				.attr('r', 6)
				.attr('fill', 'none')
				.attr('stroke', 'var(--calc-text)')
				.attr('stroke-width', 2.5);
			g.append('circle')
				.attr('cx', currentX)
				.attr('cy', currentY)
				.attr('r', 3)
				.attr('fill', 'var(--calc-accent)');
		}

		// Axes
		const xAxisGen = axisBottom(x).ticks(8).tickFormat((d) => `$${Number(d).toFixed(0)}`);
		const yAxisGen = axisLeft(y).ticks(6).tickFormat((d) => `${(Number(d) * 365).toFixed(0)}d`);

		g.append('g')
			.attr('transform', `translate(0,${innerH})`)
			.call(xAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)')
			.attr('font-size', '10px');

		g.append('g')
			.call(yAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)')
			.attr('font-size', '10px');

		// Axis labels
		g.append('text')
			.attr('x', innerW / 2)
			.attr('y', innerH + 38)
			.attr('text-anchor', 'middle')
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '11px')
			.attr('font-family', 'var(--calc-font-body)')
			.text('Strike Price');

		g.append('text')
			.attr('transform', 'rotate(-90)')
			.attr('x', -innerH / 2)
			.attr('y', -50)
			.attr('text-anchor', 'middle')
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '11px')
			.attr('font-family', 'var(--calc-font-body)')
			.text('Days to Expiry');

		// Color legend
		const legendW = 15;
		const legendH = innerH;
		const legendX = innerW + 20;
		const legendSteps = 50;

		for (let i = 0; i < legendSteps; i++) {
			const val = -absMax + (2 * absMax * i) / legendSteps;
			const ly = legendH - (legendH * i) / legendSteps;
			g.append('rect')
				.attr('x', legendX)
				.attr('y', ly - legendH / legendSteps)
				.attr('width', legendW)
				.attr('height', legendH / legendSteps + 1)
				.attr('fill', colorScale(val));
		}

		g.append('text')
			.attr('x', legendX + legendW + 4)
			.attr('y', 10)
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '9px')
			.attr('font-family', 'var(--calc-font-mono)')
			.text(`+${absMax.toFixed(4)}`);

		g.append('text')
			.attr('x', legendX + legendW + 4)
			.attr('y', legendH)
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '9px')
			.attr('font-family', 'var(--calc-font-mono)')
			.text(`-${absMax.toFixed(4)}`);
	});
</script>

<div class="flex flex-col gap-3">
	<!-- Greek selector -->
	<div class="flex items-center gap-2 flex-wrap">
		{#each GREEK_OPTIONS as opt (opt.value)}
			<button
				onclick={() => (calc.heatmapGreek = opt.value)}
				class="text-xs px-2.5 py-1 rounded-lg transition-all cursor-pointer"
				style={calc.heatmapGreek === opt.value
					? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent);'
					: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
			>
				{opt.label}
			</button>
		{/each}
	</div>

	<!-- Heatmap -->
	<div bind:this={containerEl} class="relative w-full" style="min-height: 280px;">
		<svg bind:this={svgEl}></svg>
		<div
			bind:this={tooltipEl}
			class="absolute pointer-events-none rounded-lg px-2.5 py-1.5 z-10"
			style="display: none; background: var(--calc-glass); backdrop-filter: blur(12px); border: 1px solid var(--calc-glass-border); box-shadow: var(--calc-shadow-md);"
		></div>
	</div>
</div>
