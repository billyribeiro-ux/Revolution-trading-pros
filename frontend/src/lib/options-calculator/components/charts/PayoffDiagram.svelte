<script lang="ts">
	import { pointer, select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { area, curveMonotoneX, line } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import { bisector, extent } from 'd3-array';
	import type { Axis, AxisDomain } from 'd3-axis';
	import type { Attachment } from 'svelte/attachments';
	import { getCalculator } from '../../state/context.js';
	import type { PayoffPoint } from '../../engine/types.js';

	const calc = getCalculator();

	/** Chart height in px; the width follows the container (`bind:clientWidth`). */
	const HEIGHT = 350;
	const margin = { top: 20, right: 30, bottom: 40, left: 60 };
	const innerH = HEIGHT - margin.top - margin.bottom;

	let width = $state(0);
	/** Underlying price under the pointer while the plot is hovered. */
	let hoverPrice = $state<number | null>(null);

	const bisectPrice = bisector<PayoffPoint, number>((d) => d.underlyingPrice).left;

	const seriesColor = $derived(calc.optionType === 'call' ? 'var(--calc-call)' : 'var(--calc-put)');

	/** Scales and path geometry; `null` until the container has been measured. */
	const chart = $derived.by(() => {
		const data = calc.payoffData;
		const innerW = width - margin.left - margin.right;
		const [minPrice, maxPrice] = extent(data, (d) => d.underlyingPrice);
		const [minProfit, maxProfit] = extent(data, (d) => d.profit);
		if (
			innerW <= 0 ||
			minPrice === undefined ||
			maxPrice === undefined ||
			minProfit === undefined ||
			maxProfit === undefined
		) {
			return null;
		}

		const yPadding = (maxProfit - minProfit) * 0.1 || 5;
		const x = scaleLinear().domain([minPrice, maxPrice]).range([0, innerW]);
		const y = scaleLinear()
			.domain([minProfit - yPadding, maxProfit + yPadding])
			.range([innerH, 0]);
		const zeroY = y(0);
		const inDomain = (price: number) => price >= minPrice && price <= maxPrice;
		const [sdLow, sdHigh] = calc.probabilities.oneSDRange;

		return {
			data,
			innerW,
			x,
			y,
			zeroY,
			sigmaBand:
				sdLow > minPrice && sdHigh < maxPrice ? { x: x(sdLow), width: x(sdHigh) - x(sdLow) } : null,
			profitArea: area<PayoffPoint>()
				.x((d) => x(d.underlyingPrice))
				.y0(zeroY)
				.y1((d) => (d.profit >= 0 ? y(d.profit) : zeroY))
				.curve(curveMonotoneX)(data),
			lossArea: area<PayoffPoint>()
				.x((d) => x(d.underlyingPrice))
				.y0((d) => (d.profit < 0 ? y(d.profit) : zeroY))
				.y1(zeroY)
				.curve(curveMonotoneX)(data),
			payoffLine: line<PayoffPoint>()
				.x((d) => x(d.underlyingPrice))
				.y((d) => y(d.profit))
				.curve(curveMonotoneX)(data),
			strikeX: inDomain(calc.strikePrice) ? x(calc.strikePrice) : null,
			spotX: inDomain(calc.spotPrice) ? x(calc.spotPrice) : null,
			// A curve that touches zero at a sample reports the same breakeven twice.
			breakevens: [...new Set(calc.strategyBreakevens)].filter(inDomain)
		};
	});

	/** The sample nearest the hovered price, with its plot coordinates. */
	const hovered = $derived.by(() => {
		if (!chart || hoverPrice === null) return null;
		const { data, x, y } = chart;
		const index = bisectPrice(data, hoverPrice, 1);
		const before = data[index - 1];
		const after = data[index];
		if (!before) return null;
		const point =
			after && hoverPrice - before.underlyingPrice > after.underlyingPrice - hoverPrice
				? after
				: before;
		return { point, cx: x(point.underlyingPrice), cy: y(point.profit) };
	});

	const description = $derived(
		chart?.breakevens.length
			? `Payoff diagram: profit and loss at expiration by underlying price. Breakeven at ${chart.breakevens
					.map((be) => `$${be.toFixed(2)}`)
					.join(', ')}.`
			: 'Payoff diagram: profit and loss at expiration by underlying price.'
	);

	function trackPointer(event: MouseEvent) {
		if (!chart) return;
		const [mx] = pointer(event);
		hoverPrice = chart.x.invert(mx);
	}

	/** Lets d3 draw an axis into the `<g>`; re-runs whenever the generator (its scale) changes. */
	function axis<Domain extends AxisDomain>(generator: Axis<Domain>): Attachment<SVGGElement> {
		return (g) => {
			select(g).call(generator);
		};
	}
</script>

<div class="relative w-full" style="min-height: 280px;" bind:clientWidth={width}>
	<svg {width} height={HEIGHT} role="img" aria-label={description}>
		{#if chart}
			<g transform="translate({margin.left},{margin.top})">
				<!-- 1σ shading -->
				{#if chart.sigmaBand}
					<rect
						x={chart.sigmaBand.x}
						y="0"
						width={chart.sigmaBand.width}
						height={innerH}
						fill="var(--calc-accent)"
						opacity="0.04"
					/>
				{/if}

				<!-- Profit/loss area fills -->
				<path d={chart.profitArea} fill="var(--calc-profit)" opacity="0.12" />
				<path d={chart.lossArea} fill="var(--calc-loss)" opacity="0.12" />

				<!-- Zero line -->
				{#if chart.zeroY >= 0 && chart.zeroY <= innerH}
					<line
						x1="0"
						x2={chart.innerW}
						y1={chart.zeroY}
						y2={chart.zeroY}
						stroke="var(--calc-text-muted)"
						stroke-width="1"
						stroke-dasharray="4,4"
						opacity="0.5"
					/>
				{/if}

				<!-- Strike price line -->
				{#if chart.strikeX !== null}
					<line
						x1={chart.strikeX}
						x2={chart.strikeX}
						y1="0"
						y2={innerH}
						stroke="var(--calc-accent)"
						stroke-width="1"
						stroke-dasharray="6,3"
						opacity="0.6"
					/>
					<text
						x={chart.strikeX}
						y="-6"
						text-anchor="middle"
						fill="var(--calc-accent)"
						font-size="10px"
						font-family="var(--calc-font-mono)">K={calc.strikePrice.toFixed(0)}</text
					>
				{/if}

				<!-- Spot price line -->
				{#if chart.spotX !== null}
					<line
						x1={chart.spotX}
						x2={chart.spotX}
						y1="0"
						y2={innerH}
						stroke="var(--calc-text-secondary)"
						stroke-width="1"
						stroke-dasharray="2,2"
						opacity="0.5"
					/>
				{/if}

				<!-- Payoff line -->
				<path
					d={chart.payoffLine}
					fill="none"
					stroke={seriesColor}
					stroke-width="2.5"
					stroke-linejoin="round"
				/>

				<!-- Breakeven dots -->
				{#each chart.breakevens as be (be)}
					<circle
						cx={chart.x(be)}
						cy={chart.zeroY}
						r="5"
						fill="var(--calc-warning)"
						stroke="var(--calc-bg)"
						stroke-width="2"
					/>
					<text
						x={chart.x(be)}
						y={chart.zeroY + 16}
						text-anchor="middle"
						fill="var(--calc-warning)"
						font-size="10px"
						font-family="var(--calc-font-mono)">BE ${be.toFixed(1)}</text
					>
				{/each}

				<!-- Axes -->
				<g
					class="axis"
					transform="translate(0,{innerH})"
					{@attach axis(
						axisBottom(chart.x)
							.ticks(8)
							.tickFormat((d) => `$${d}`)
					)}
				></g>
				<g
					class="axis"
					{@attach axis(
						axisLeft(chart.y)
							.ticks(6)
							.tickFormat((d) => `$${d}`)
					)}
				></g>

				<!-- Hover crosshair -->
				{#if hovered}
					<g>
						<line
							x1={hovered.cx}
							x2={hovered.cx}
							y1="0"
							y2={innerH}
							stroke="var(--calc-text-muted)"
							stroke-width="1"
							stroke-dasharray="3,3"
						/>
						<line
							x1="0"
							x2={chart.innerW}
							y1={hovered.cy}
							y2={hovered.cy}
							stroke="var(--calc-text-muted)"
							stroke-width="1"
							stroke-dasharray="3,3"
						/>
						<circle
							cx={hovered.cx}
							cy={hovered.cy}
							r="4"
							fill={seriesColor}
							stroke="var(--calc-bg)"
							stroke-width="2"
						/>
					</g>
				{/if}

				<!-- Pointer capture surface (the chart's text alternative is the svg's label) -->
				<rect
					width={chart.innerW}
					height={innerH}
					fill="transparent"
					aria-hidden="true"
					onmousemove={trackPointer}
					onmouseleave={() => (hoverPrice = null)}
				/>
			</g>
		{/if}
	</svg>

	{#if hovered}
		<div
			class="absolute pointer-events-none rounded-lg px-2.5 py-1.5 z-10"
			style="background: var(--calc-glass); backdrop-filter: blur(12px); border: 1px solid var(--calc-glass-border); box-shadow: var(--calc-shadow-md);"
			style:left="{hovered.cx + margin.left + 12}px"
			style:top="{hovered.cy + margin.top - 20}px"
		>
			<div style="font-family: var(--calc-font-mono); font-size: 11px;">
				<div style="color: var(--calc-text-secondary);">
					Price: ${hovered.point.underlyingPrice.toFixed(2)}
				</div>
				<div style:color={hovered.point.profit >= 0 ? 'var(--calc-profit)' : 'var(--calc-loss)'}>
					P&amp;L: {hovered.point.profit >= 0 ? '+' : ''}${hovered.point.profit.toFixed(2)}
				</div>
			</div>
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
