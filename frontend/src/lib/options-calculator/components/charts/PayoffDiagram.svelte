<script lang="ts">
	import { select, pointer } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { line as d3line, area as d3area, curveMonotoneX } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import { extent, bisector as d3bisector } from 'd3-array';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { PayoffPoint } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let svgEl: SVGSVGElement | undefined = $state();
	let tooltipEl: HTMLDivElement | undefined = $state();
	let width = $state(600);
	let height = $state(350);

	const margin = { top: 20, right: 30, bottom: 40, left: 60 };

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
		if (!svgEl || !calc.payoffData || calc.payoffData.length === 0) return;

		const data = calc.payoffData;
		const spotPrice = calc.spotPrice;
		const strikePrice = calc.strikePrice;
		const breakevens = calc.strategyBreakevens;
		const optionType = calc.optionType;
		const oneSD = calc.probabilities.oneSDRange;

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

		// Scales
		const xExtent = extent(data, (d: PayoffPoint) => d.underlyingPrice) as [number, number];
		const yExtent = extent(data, (d: PayoffPoint) => d.profit) as [number, number];
		const yPadding = (yExtent[1] - yExtent[0]) * 0.1 || 5;

		const x = scaleLinear().domain(xExtent).range([0, innerW]);
		const y = scaleLinear()
			.domain([yExtent[0] - yPadding, yExtent[1] + yPadding])
			.range([innerH, 0]);

		// 1σ shading
		if (oneSD[0] > xExtent[0] && oneSD[1] < xExtent[1]) {
			g.append('rect')
				.attr('x', x(oneSD[0]))
				.attr('y', 0)
				.attr('width', x(oneSD[1]) - x(oneSD[0]))
				.attr('height', innerH)
				.attr('fill', 'var(--calc-accent)')
				.attr('opacity', 0.04);
		}

		// Profit/loss area fills
		const areaProfit = d3area<PayoffPoint>()
			.x((d) => x(d.underlyingPrice))
			.y0(y(0))
			.y1((d) => (d.profit >= 0 ? y(d.profit) : y(0)))
			.curve(curveMonotoneX);

		const areaLoss = d3area<PayoffPoint>()
			.x((d) => x(d.underlyingPrice))
			.y0((d) => (d.profit < 0 ? y(d.profit) : y(0)))
			.y1(y(0))
			.curve(curveMonotoneX);

		g.append('path').datum(data).attr('d', areaProfit).attr('fill', 'var(--calc-profit)').attr('opacity', 0.12);
		g.append('path').datum(data).attr('d', areaLoss).attr('fill', 'var(--calc-loss)').attr('opacity', 0.12);

		// Zero line
		if (y(0) >= 0 && y(0) <= innerH) {
			g.append('line')
				.attr('x1', 0)
				.attr('x2', innerW)
				.attr('y1', y(0))
				.attr('y2', y(0))
				.attr('stroke', 'var(--calc-text-muted)')
				.attr('stroke-width', 1)
				.attr('stroke-dasharray', '4,4')
				.attr('opacity', 0.5);
		}

		// Strike price line
		if (strikePrice >= xExtent[0] && strikePrice <= xExtent[1]) {
			g.append('line')
				.attr('x1', x(strikePrice))
				.attr('x2', x(strikePrice))
				.attr('y1', 0)
				.attr('y2', innerH)
				.attr('stroke', 'var(--calc-accent)')
				.attr('stroke-width', 1)
				.attr('stroke-dasharray', '6,3')
				.attr('opacity', 0.6);

			g.append('text')
				.attr('x', x(strikePrice))
				.attr('y', -6)
				.attr('text-anchor', 'middle')
				.attr('fill', 'var(--calc-accent)')
				.attr('font-size', '10px')
				.attr('font-family', 'var(--calc-font-mono)')
				.text(`K=${strikePrice.toFixed(0)}`);
		}

		// Spot price line
		if (spotPrice >= xExtent[0] && spotPrice <= xExtent[1]) {
			g.append('line')
				.attr('x1', x(spotPrice))
				.attr('x2', x(spotPrice))
				.attr('y1', 0)
				.attr('y2', innerH)
				.attr('stroke', 'var(--calc-text-secondary)')
				.attr('stroke-width', 1)
				.attr('stroke-dasharray', '2,2')
				.attr('opacity', 0.5);
		}

		// Payoff line
		const payoffLine = d3line<PayoffPoint>()
			.x((d) => x(d.underlyingPrice))
			.y((d) => y(d.profit))
			.curve(curveMonotoneX);

		g.append('path')
			.datum(data)
			.attr('d', payoffLine)
			.attr('fill', 'none')
			.attr('stroke', optionType === 'call' ? 'var(--calc-call)' : 'var(--calc-put)')
			.attr('stroke-width', 2.5)
			.attr('stroke-linejoin', 'round');

		// Breakeven dots
		for (const be of breakevens) {
			if (be >= xExtent[0] && be <= xExtent[1]) {
				g.append('circle')
					.attr('cx', x(be))
					.attr('cy', y(0))
					.attr('r', 5)
					.attr('fill', 'var(--calc-warning)')
					.attr('stroke', 'var(--calc-bg)')
					.attr('stroke-width', 2);

				g.append('text')
					.attr('x', x(be))
					.attr('y', y(0) + 16)
					.attr('text-anchor', 'middle')
					.attr('fill', 'var(--calc-warning)')
					.attr('font-size', '10px')
					.attr('font-family', 'var(--calc-font-mono)')
					.text(`BE $${be.toFixed(1)}`);
			}
		}

		// Axes
		const xAxisGen = axisBottom(x).ticks(8).tickFormat((d) => `$${d}`);
		const yAxisGen = axisLeft(y).ticks(6).tickFormat((d) => `$${d}`);

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

		// Hover crosshair
		const crosshairGroup = g.append('g').style('display', 'none');
		const crosshairLineV = crosshairGroup
			.append('line')
			.attr('y1', 0)
			.attr('y2', innerH)
			.attr('stroke', 'var(--calc-text-muted)')
			.attr('stroke-width', 1)
			.attr('stroke-dasharray', '3,3');
		const crosshairLineH = crosshairGroup
			.append('line')
			.attr('x1', 0)
			.attr('x2', innerW)
			.attr('stroke', 'var(--calc-text-muted)')
			.attr('stroke-width', 1)
			.attr('stroke-dasharray', '3,3');
		const crosshairDot = crosshairGroup
			.append('circle')
			.attr('r', 4)
			.attr('fill', optionType === 'call' ? 'var(--calc-call)' : 'var(--calc-put)')
			.attr('stroke', 'var(--calc-bg)')
			.attr('stroke-width', 2);

		const dataBisector = d3bisector<PayoffPoint, number>((d) => d.underlyingPrice).left;

		g.append('rect')
			.attr('width', innerW)
			.attr('height', innerH)
			.attr('fill', 'transparent')
			.on('mouseenter', () => {
				crosshairGroup.style('display', null);
				if (tooltipEl) tooltipEl.style.display = 'block';
			})
			.on('mouseleave', () => {
				crosshairGroup.style('display', 'none');
				if (tooltipEl) tooltipEl.style.display = 'none';
			})
			.on('mousemove', (event: MouseEvent) => {
				const [mx] = pointer(event);
				const xVal = x.invert(mx);
				const idx = dataBisector(data, xVal, 1);
				const d0 = data[idx - 1];
				const d1 = data[idx];
				if (!d0 || !d1) return;
				const d = xVal - d0.underlyingPrice > d1.underlyingPrice - xVal ? d1 : d0;

				const cx = x(d.underlyingPrice);
				const cy = y(d.profit);

				crosshairLineV.attr('x1', cx).attr('x2', cx);
				crosshairLineH.attr('y1', cy).attr('y2', cy);
				crosshairDot.attr('cx', cx).attr('cy', cy);

				if (tooltipEl) {
					tooltipEl.style.left = `${cx + margin.left + 12}px`;
					tooltipEl.style.top = `${cy + margin.top - 20}px`;
					tooltipEl.innerHTML = `
						<div style="font-family: var(--calc-font-mono); font-size: 11px;">
							<div style="color: var(--calc-text-secondary);">Price: $${d.underlyingPrice.toFixed(2)}</div>
							<div style="color: ${d.profit >= 0 ? 'var(--calc-profit)' : 'var(--calc-loss)'};">P&L: ${d.profit >= 0 ? '+' : ''}$${d.profit.toFixed(2)}</div>
						</div>
					`;
				}
			});
	});
</script>

<div bind:this={containerEl} class="relative w-full" style="min-height: 280px;">
	<svg bind:this={svgEl}></svg>
	<div
		bind:this={tooltipEl}
		class="absolute pointer-events-none rounded-lg px-2.5 py-1.5 z-10"
		style="display: none; background: var(--calc-glass); backdrop-filter: blur(12px); border: 1px solid var(--calc-glass-border); box-shadow: var(--calc-shadow-md);"
	></div>
</div>
