<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { area, curveMonotoneX, line } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import type { Axis, AxisDomain } from 'd3-axis';
	import type { Attachment } from 'svelte/attachments';
	import { getCalculator } from '../../state/context.js';
	import type { VolSmilePoint } from '../../engine/types.js';

	const calc = getCalculator();
	const uid = $props.id();

	/** Chart height in px; the width follows the container (`bind:clientWidth`). */
	const HEIGHT = 350;
	const margin = { top: 20, right: 30, bottom: 50, left: 60 };
	const innerH = HEIGHT - margin.top - margin.bottom;

	let width = $state(0);
	let skew = $state(-0.15);
	let kurtosis = $state(0.1);

	/** Synthetic smile: IV(K) = ATM + skew·m + kurtosis·m², m = K/S − 1, over 70%–130% of spot. */
	const smileData = $derived.by<VolSmilePoint[]>(() => {
		const spot = calc.spotPrice;
		const atmVol = calc.volatility;
		const points: VolSmilePoint[] = [];

		for (let i = 0; i <= 40; i++) {
			const ratio = 0.7 + (i / 40) * 0.6;
			const m = ratio - 1;
			points.push({
				strike: spot * ratio,
				impliedVol: Math.max(0.01, atmVol + skew * m + kurtosis * m * m),
				moneyness: m,
				strikeRatio: ratio
			});
		}
		return points;
	});

	/** Scales and path geometry; `null` until the container has been measured. */
	const chart = $derived.by(() => {
		const data = smileData;
		const first = data.at(0);
		const last = data.at(-1);
		const innerW = width - margin.left - margin.right;
		if (!first || !last || innerW <= 0) return null;

		const vols = data.map((d) => d.impliedVol);
		const x = scaleLinear().domain([first.strike, last.strike]).range([0, innerW]);
		const y = scaleLinear()
			.domain([Math.min(...vols) * 0.9, Math.max(...vols) * 1.1])
			.range([innerH, 0]);

		// The current strike sits on the smile at its nearest sampled strike.
		const strike = calc.strikePrice;
		const nearest = data.reduce((closest, d) =>
			Math.abs(d.strike - strike) < Math.abs(closest.strike - strike) ? d : closest
		);

		return {
			innerW,
			x,
			y,
			smileArea: area<VolSmilePoint>()
				.x((d) => x(d.strike))
				.y0(innerH)
				.y1((d) => y(d.impliedVol))
				.curve(curveMonotoneX)(data),
			smileLine: line<VolSmilePoint>()
				.x((d) => x(d.strike))
				.y((d) => y(d.impliedVol))
				.curve(curveMonotoneX)(data),
			atm: { x: x(calc.spotPrice), y: y(calc.volatility) },
			current:
				strike >= first.strike && strike <= last.strike
					? { x: x(strike), y: y(nearest.impliedVol), iv: nearest.impliedVol }
					: null
		};
	});

	const atmLabel = $derived(`${(calc.volatility * 100).toFixed(1)}%`);

	/** Lets d3 draw an axis into the `<g>`; re-runs whenever the generator (its scale) changes. */
	function axis<Domain extends AxisDomain>(generator: Axis<Domain>): Attachment<SVGGElement> {
		return (g) => {
			select(g).call(generator);
		};
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-4 flex-wrap">
		<div class="flex items-center gap-2">
			<label
				for="{uid}-skew"
				class="text-[10px]"
				style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">Skew:</label
			>
			<input
				id="{uid}-skew"
				type="range"
				min="-0.5"
				max="0.2"
				step="0.01"
				bind:value={skew}
				aria-label="Volatility skew"
				class="calc-slider w-20"
			/>
			<span
				class="text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
				>{skew.toFixed(2)}</span
			>
		</div>
		<div class="flex items-center gap-2">
			<label
				for="{uid}-kurtosis"
				class="text-[10px]"
				style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">Kurtosis:</label
			>
			<input
				id="{uid}-kurtosis"
				type="range"
				min="0"
				max="0.5"
				step="0.01"
				bind:value={kurtosis}
				aria-label="Volatility kurtosis"
				class="calc-slider w-20"
			/>
			<span
				class="text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
				>{kurtosis.toFixed(2)}</span
			>
		</div>
		<span class="text-[10px] italic" style="color: var(--calc-text-muted);">
			Synthetic smile: IV(K) = ATM + skew*(K/S-1) + kurt*(K/S-1)²
		</span>
	</div>

	<!-- Chart -->
	<div class="relative w-full" style="min-height: 280px;" bind:clientWidth={width}>
		<svg
			{width}
			height={HEIGHT}
			role="img"
			aria-label="Volatility smile: implied volatility by strike price, at-the-money {atmLabel}"
		>
			{#if chart}
				<g transform="translate({margin.left},{margin.top})">
					<!-- Shaded area under the smile -->
					<path d={chart.smileArea} fill="var(--calc-accent)" opacity="0.06" />

					<!-- Vol smile line -->
					<path d={chart.smileLine} fill="none" stroke="var(--calc-accent)" stroke-width="2.5" />

					<!-- ATM marker -->
					<line
						x1={chart.atm.x}
						x2={chart.atm.x}
						y1="0"
						y2={innerH}
						stroke="var(--calc-text-muted)"
						stroke-width="1"
						stroke-dasharray="4,4"
						opacity="0.5"
					/>
					<circle
						cx={chart.atm.x}
						cy={chart.atm.y}
						r="5"
						fill="var(--calc-accent)"
						stroke="var(--calc-bg)"
						stroke-width="2"
					/>
					<text
						x={chart.atm.x + 8}
						y={chart.atm.y - 8}
						fill="var(--calc-accent)"
						font-size="10px"
						font-family="var(--calc-font-mono)">ATM {atmLabel}</text
					>

					<!-- Current strike marker -->
					{#if chart.current}
						<circle
							cx={chart.current.x}
							cy={chart.current.y}
							r="6"
							fill="none"
							stroke="var(--calc-warning)"
							stroke-width="2"
						/>
						<text
							x={chart.current.x + 8}
							y={chart.current.y + 4}
							fill="var(--calc-warning)"
							font-size="9px"
							font-family="var(--calc-font-mono)"
							>K=${calc.strikePrice.toFixed(0)} IV={(chart.current.iv * 100).toFixed(1)}%</text
						>
					{/if}

					<!-- Axes -->
					<g
						class="axis"
						transform="translate(0,{innerH})"
						{@attach axis(
							axisBottom(chart.x)
								.ticks(8)
								.tickFormat((d) => `$${Number(d).toFixed(0)}`)
						)}
					></g>
					<g
						class="axis"
						{@attach axis(
							axisLeft(chart.y)
								.ticks(6)
								.tickFormat((d) => `${(Number(d) * 100).toFixed(0)}%`)
						)}
					></g>

					<!-- Axis labels -->
					<text
						x={chart.innerW / 2}
						y={innerH + 38}
						text-anchor="middle"
						fill="var(--calc-text-muted)"
						font-size="11px"
						font-family="var(--calc-font-body)">Strike Price</text
					>
					<text
						transform="rotate(-90)"
						x={-innerH / 2}
						y="-45"
						text-anchor="middle"
						fill="var(--calc-text-muted)"
						font-size="11px"
						font-family="var(--calc-font-body)">Implied Volatility</text
					>
				</g>
			{/if}
		</svg>
	</div>
</div>

<style>
	/* d3-axis writes `font-size`/`font-family` presentation attributes; CSS takes precedence. */
	.axis {
		color: var(--calc-text-muted);
		font-family: var(--calc-font-mono);
		font-size: 10px;
	}
</style>
