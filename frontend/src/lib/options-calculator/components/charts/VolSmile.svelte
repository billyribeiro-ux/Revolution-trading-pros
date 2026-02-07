<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { line as d3line, area as d3area, curveMonotoneX } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { VolSmilePoint } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let svgEl: SVGSVGElement | undefined = $state();
	let width = $state(600);
	let height = $state(350);
	let skew = $state(-0.15);
	let kurtosis = $state(0.10);

	const margin = { top: 20, right: 30, bottom: 50, left: 60 };

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

	let smileData = $derived.by<VolSmilePoint[]>(() => {
		const spot = calc.spotPrice;
		const atmVol = calc.volatility;
		const sk = skew;
		const kurt = kurtosis;
		const points: VolSmilePoint[] = [];

		for (let i = 0; i <= 40; i++) {
			const ratio = 0.7 + (i / 40) * 0.6;
			const strike = spot * ratio;
			const m = ratio - 1;
			const iv = atmVol + sk * m + kurt * m * m;
			points.push({
				strike,
				impliedVol: Math.max(0.01, iv),
				moneyness: m,
				strikeRatio: ratio,
			});
		}
		return points;
	});

	$effect(() => {
		if (!svgEl || smileData.length === 0) return;

		const data = smileData;
		const spot = calc.spotPrice;
		const atmVol = calc.volatility;

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

		const xDomain = [data[0].strike, data[data.length - 1].strike];
		const yMin = Math.min(...data.map((d) => d.impliedVol)) * 0.9;
		const yMax = Math.max(...data.map((d) => d.impliedVol)) * 1.1;

		const x = scaleLinear().domain(xDomain).range([0, innerW]);
		const y = scaleLinear().domain([yMin, yMax]).range([innerH, 0]);

		// Shaded area under the smile
		const areaGen = d3area<VolSmilePoint>()
			.x((d) => x(d.strike))
			.y0(innerH)
			.y1((d) => y(d.impliedVol))
			.curve(curveMonotoneX);

		g.append('path')
			.datum(data)
			.attr('d', areaGen)
			.attr('fill', 'var(--calc-accent)')
			.attr('opacity', 0.06);

		// Vol smile line
		const lineGen = d3line<VolSmilePoint>()
			.x((d) => x(d.strike))
			.y((d) => y(d.impliedVol))
			.curve(curveMonotoneX);

		g.append('path')
			.datum(data)
			.attr('d', lineGen)
			.attr('fill', 'none')
			.attr('stroke', 'var(--calc-accent)')
			.attr('stroke-width', 2.5);

		// ATM marker
		g.append('line')
			.attr('x1', x(spot)).attr('x2', x(spot))
			.attr('y1', 0).attr('y2', innerH)
			.attr('stroke', 'var(--calc-text-muted)')
			.attr('stroke-width', 1).attr('stroke-dasharray', '4,4').attr('opacity', 0.5);

		g.append('circle')
			.attr('cx', x(spot)).attr('cy', y(atmVol))
			.attr('r', 5).attr('fill', 'var(--calc-accent)')
			.attr('stroke', 'var(--calc-bg)').attr('stroke-width', 2);

		g.append('text')
			.attr('x', x(spot) + 8).attr('y', y(atmVol) - 8)
			.attr('fill', 'var(--calc-accent)')
			.attr('font-size', '10px').attr('font-family', 'var(--calc-font-mono)')
			.text(`ATM ${(atmVol * 100).toFixed(1)}%`);

		// Current strike marker
		const currentStrike = calc.strikePrice;
		if (currentStrike >= xDomain[0] && currentStrike <= xDomain[1]) {
			const currentIV = data.reduce((closest, d) =>
				Math.abs(d.strike - currentStrike) < Math.abs(closest.strike - currentStrike) ? d : closest,
			).impliedVol;

			g.append('circle')
				.attr('cx', x(currentStrike)).attr('cy', y(currentIV))
				.attr('r', 6).attr('fill', 'none')
				.attr('stroke', 'var(--calc-warning)').attr('stroke-width', 2);

			g.append('text')
				.attr('x', x(currentStrike) + 8).attr('y', y(currentIV) + 4)
				.attr('fill', 'var(--calc-warning)')
				.attr('font-size', '9px').attr('font-family', 'var(--calc-font-mono)')
				.text(`K=$${currentStrike.toFixed(0)} IV=${(currentIV * 100).toFixed(1)}%`);
		}

		// Axes
		const xAxisGen = axisBottom(x).ticks(8).tickFormat((d) => `$${Number(d).toFixed(0)}`);
		const yAxisGen = axisLeft(y).ticks(6).tickFormat((d) => `${(Number(d) * 100).toFixed(0)}%`);

		g.append('g')
			.attr('transform', `translate(0,${innerH})`)
			.call(xAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)').attr('font-size', '10px');

		g.append('g')
			.call(yAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)').attr('font-size', '10px');

		// Axis labels
		g.append('text')
			.attr('x', innerW / 2).attr('y', innerH + 38)
			.attr('text-anchor', 'middle')
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '11px').attr('font-family', 'var(--calc-font-body)')
			.text('Strike Price');

		g.append('text')
			.attr('transform', 'rotate(-90)')
			.attr('x', -innerH / 2).attr('y', -45)
			.attr('text-anchor', 'middle')
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '11px').attr('font-family', 'var(--calc-font-body)')
			.text('Implied Volatility');
	});
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-4 flex-wrap">
		<div class="flex items-center gap-2">
			<span class="text-[10px]" style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">Skew:</span>
			<input type="range" min="-0.5" max="0.2" step="0.01" bind:value={skew} class="calc-slider w-20" />
			<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{skew.toFixed(2)}</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="text-[10px]" style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">Kurtosis:</span>
			<input type="range" min="0" max="0.5" step="0.01" bind:value={kurtosis} class="calc-slider w-20" />
			<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">{kurtosis.toFixed(2)}</span>
		</div>
		<span class="text-[10px] italic" style="color: var(--calc-text-muted);">
			Synthetic smile: IV(K) = ATM + skew*(K/S-1) + kurt*(K/S-1)²
		</span>
	</div>

	<!-- Chart -->
	<div bind:this={containerEl} class="relative w-full" style="min-height: 280px;">
		<svg bind:this={svgEl}></svg>
	</div>
</div>
