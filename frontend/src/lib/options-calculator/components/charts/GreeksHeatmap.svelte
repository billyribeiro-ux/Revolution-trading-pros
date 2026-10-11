<script lang="ts">
	import { pointer, select } from 'd3-selection';
	import { scaleLinear, scaleSequential } from 'd3-scale';
	import { interpolateRdYlGn } from 'd3-scale-chromatic';
	import { axisBottom, axisLeft } from 'd3-axis';
	import type { Axis, AxisDomain } from 'd3-axis';
	import type { Attachment } from 'svelte/attachments';
	import { greeksMatrix } from '../../engine/greeks.js';
	import { getCalculator } from '../../state/context.js';
	import type { HeatmapGreek } from '../../engine/types.js';

	const calc = getCalculator();

	/** Chart height in px; the width follows the container (`bind:clientWidth`). */
	const HEIGHT = 350;
	const margin = { top: 30, right: 80, bottom: 50, left: 70 };
	const innerH = HEIGHT - margin.top - margin.bottom;

	const STRIKE_COUNT = 20;
	const EXPIRY_COUNT = 15;
	const EXPIRY_MIN = 1 / 365;
	const LEGEND_WIDTH = 15;
	/** The colour legend is a stack of this many bands, the first at the bottom. */
	const LEGEND_STEPS = 50;

	const GREEK_OPTIONS: { value: HeatmapGreek; label: string }[] = [
		{ value: 'delta', label: 'Delta' },
		{ value: 'gamma', label: 'Gamma' },
		{ value: 'theta', label: 'Theta' },
		{ value: 'vega', label: 'Vega' },
		{ value: 'rho', label: 'Rho' },
		{ value: 'charm', label: 'Charm' },
		{ value: 'vanna', label: 'Vanna' },
		{ value: 'volga', label: 'Volga' }
	];

	let width = $state(0);
	/** The hovered cell and where its tooltip sits (container px). */
	let hover = $state<{ key: string; left: number; top: number } | null>(null);

	/** The greek over a strike × expiry grid: ±30% around spot, 1 day to the current expiry. */
	const matrix = $derived.by(() => {
		const inputs = calc.inputs;
		const strikeLow = calc.spotPrice * 0.7;
		const strikeHigh = calc.spotPrice * 1.3;
		const strikeStep = (strikeHigh - strikeLow) / STRIKE_COUNT;
		const strikes = Array.from({ length: STRIKE_COUNT + 1 }, (_, i) => strikeLow + i * strikeStep);

		const expiryMax = Math.max(inputs.timeToExpiry, 0.01);
		const expiryStep = (expiryMax - EXPIRY_MIN) / EXPIRY_COUNT;
		const expiries = Array.from(
			{ length: EXPIRY_COUNT + 1 },
			(_, i) => EXPIRY_MIN + i * expiryStep
		);

		const values = greeksMatrix(inputs, calc.optionType, calc.heatmapGreek, strikes, expiries);
		const greeks = values.map((d) => d.value);
		const absMax = Math.max(Math.abs(Math.min(...greeks)), Math.abs(Math.max(...greeks))) || 1;
		const color = scaleSequential(interpolateRdYlGn).domain([-absMax, absMax]);

		return {
			strikeLow,
			strikeHigh,
			expiryMax,
			absMax,
			// `greeksMatrix` is strike-major, so the grid position (strike, expiry) is the cell's identity.
			cells: values.map((cell, i) => ({
				...cell,
				key: `${Math.floor(i / expiries.length)}:${i % expiries.length}`,
				fill: color(cell.value)
			})),
			legend: Array.from({ length: LEGEND_STEPS }, (_, step) => ({
				step,
				fill: color(-absMax + (2 * absMax * step) / LEGEND_STEPS)
			}))
		};
	});

	/** Scales for the measured width; `null` until the container has been measured. */
	const chart = $derived.by(() => {
		const innerW = width - margin.left - margin.right;
		if (innerW <= 0) return null;

		const { strikeLow, strikeHigh, expiryMax } = matrix;
		const x = scaleLinear().domain([strikeLow, strikeHigh]).range([0, innerW]);
		const y = scaleLinear().domain([EXPIRY_MIN, expiryMax]).range([innerH, 0]);
		const markerX = x(calc.strikePrice);
		const markerY = y(calc.timeToExpiry);

		return {
			innerW,
			x,
			y,
			cellW: innerW / STRIKE_COUNT,
			cellH: innerH / EXPIRY_COUNT,
			marker:
				markerX >= 0 && markerX <= innerW && markerY >= 0 && markerY <= innerH
					? { x: markerX, y: markerY }
					: null
		};
	});

	const hoveredCell = $derived.by(() => {
		const key = hover?.key;
		return key === undefined ? undefined : matrix.cells.find((cell) => cell.key === key);
	});

	const greekLabel = $derived(
		GREEK_OPTIONS.find((opt) => opt.value === calc.heatmapGreek)?.label ?? calc.heatmapGreek
	);

	function showCell(key: string, event: MouseEvent) {
		const [px, py] = pointer(event);
		hover = { key, left: px + margin.left + 12, top: py + margin.top - 40 };
	}

	/** Lets d3 draw an axis into the `<g>`; re-runs whenever the generator (its scale) changes. */
	function axis<Domain extends AxisDomain>(generator: Axis<Domain>): Attachment<SVGGElement> {
		return (g) => {
			select(g).call(generator);
		};
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Greek selector -->
	<div class="flex items-center gap-2 flex-wrap">
		{#each GREEK_OPTIONS as opt (opt.value)}
			{const selected = $derived(calc.heatmapGreek === opt.value)}
			<button
				type="button"
				onclick={() => (calc.heatmapGreek = opt.value)}
				aria-pressed={selected}
				class="text-xs px-2.5 py-1 rounded-lg transition-all cursor-pointer"
				style:background={selected ? 'var(--calc-accent-glow)' : 'var(--calc-surface)'}
				style:color={selected ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
				style:border="1px solid {selected ? 'var(--calc-accent)' : 'var(--calc-border)'}"
			>
				{opt.label}
			</button>
		{/each}
	</div>

	<!-- Heatmap -->
	<div class="relative w-full" style="min-height: 280px;" bind:clientWidth={width}>
		<svg
			{width}
			height={HEIGHT}
			role="img"
			aria-label="{greekLabel} heatmap by strike price and days to expiry"
		>
			{#if chart}
				<g transform="translate({margin.left},{margin.top})">
					<!-- Cells -->
					{#each matrix.cells as cell (cell.key)}
						{const active = $derived(hover?.key === cell.key)}
						<rect
							x={chart.x(cell.strike) - chart.cellW / 2}
							y={chart.y(cell.expiry) - chart.cellH / 2}
							width={chart.cellW + 1}
							height={chart.cellH + 1}
							fill={cell.fill}
							opacity="0.85"
							stroke={active ? 'var(--calc-text)' : undefined}
							stroke-width={active ? 2 : undefined}
							aria-hidden="true"
							onmouseenter={(event) => showCell(cell.key, event)}
							onmouseleave={() => (hover = null)}
						/>
					{/each}

					<!-- Current position marker -->
					{#if chart.marker}
						<circle
							cx={chart.marker.x}
							cy={chart.marker.y}
							r="6"
							fill="none"
							stroke="var(--calc-text)"
							stroke-width="2.5"
						/>
						<circle cx={chart.marker.x} cy={chart.marker.y} r="3" fill="var(--calc-accent)" />
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
								.tickFormat((d) => `${(Number(d) * 365).toFixed(0)}d`)
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
						y="-50"
						text-anchor="middle"
						fill="var(--calc-text-muted)"
						font-size="11px"
						font-family="var(--calc-font-body)">Days to Expiry</text
					>

					<!-- Colour legend -->
					{#each matrix.legend as band (band.step)}
						<rect
							x={chart.innerW + 20}
							y={innerH - (innerH * (band.step + 1)) / LEGEND_STEPS}
							width={LEGEND_WIDTH}
							height={innerH / LEGEND_STEPS + 1}
							fill={band.fill}
						/>
					{/each}
					<text
						x={chart.innerW + 20 + LEGEND_WIDTH + 4}
						y="10"
						fill="var(--calc-text-muted)"
						font-size="9px"
						font-family="var(--calc-font-mono)">+{matrix.absMax.toFixed(4)}</text
					>
					<text
						x={chart.innerW + 20 + LEGEND_WIDTH + 4}
						y={innerH}
						fill="var(--calc-text-muted)"
						font-size="9px"
						font-family="var(--calc-font-mono)">-{matrix.absMax.toFixed(4)}</text
					>
				</g>
			{/if}
		</svg>

		{#if hover && hoveredCell}
			<div
				class="absolute pointer-events-none rounded-lg px-2.5 py-1.5 z-10"
				style="background: var(--calc-glass); backdrop-filter: blur(12px); border: 1px solid var(--calc-glass-border); box-shadow: var(--calc-shadow-md);"
				style:left="{hover.left}px"
				style:top="{hover.top}px"
			>
				<div style="font-family: var(--calc-font-mono); font-size: 11px;">
					<div style="color: var(--calc-text-secondary);">
						Strike: ${hoveredCell.strike.toFixed(1)}
					</div>
					<div style="color: var(--calc-text-secondary);">
						Expiry: {(hoveredCell.expiry * 365).toFixed(0)}d
					</div>
					<div style="color: var(--calc-text); font-weight: 600;">
						{calc.heatmapGreek}: {hoveredCell.value.toFixed(6)}
					</div>
				</div>
			</div>
		{/if}
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
