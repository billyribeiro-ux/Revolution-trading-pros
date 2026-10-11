<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { area, curveMonotoneX, line } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import type { Axis, AxisDomain } from 'd3-axis';
	import type { Attachment } from 'svelte/attachments';
	import { formatCurrency } from '../../utils/formatters.js';
	import { getCalculator } from '../../state/context.js';
	import type { TimeMachineSnapshot } from '../../engine/types.js';

	const calc = getCalculator();

	/** Chart height in px; the width follows the container (`bind:clientWidth`). */
	const HEIGHT = 350;
	const margin = { top: 20, right: 30, bottom: 50, left: 60 };
	const innerH = HEIGHT - margin.top - margin.bottom;
	/** Days-to-expiry milestones marked on the chart. */
	const KEY_DTES = [45, 30, 21, 14, 7, 1];

	let width = $state(0);
	let showBoth = $state(true);

	const isCall = $derived(calc.optionType === 'call');

	/** Scales and path geometry; `null` until the container has been measured. */
	const chart = $derived.by(() => {
		const snapshots = calc.timeMachineSnapshots;
		const last = snapshots.at(-1);
		const innerW = width - margin.left - margin.right;
		if (!last || innerW <= 0) return null;

		const totalDays = last.daysFromNow;
		const maxPrice = Math.max(...snapshots.map((s) => Math.max(s.callPrice, s.putPrice))) * 1.1;
		const x = scaleLinear().domain([0, totalDays]).range([0, innerW]);
		const y = scaleLinear().domain([0, maxPrice]).range([innerH, 0]);

		const intrinsic = calc.currentIntrinsic;
		const intrinsicY = y(intrinsic);
		const accelDay = totalDays - 45;

		return {
			innerW,
			x,
			y,
			intrinsic,
			intrinsicY,
			// Extrinsic value of the selected option type, shaded above its intrinsic value.
			extrinsicArea: area<TimeMachineSnapshot>()
				.x((d) => x(d.daysFromNow))
				.y0(intrinsicY)
				.y1((d) => y(Math.max(isCall ? d.callPrice : d.putPrice, intrinsic)))
				.curve(curveMonotoneX)(snapshots),
			callLine: line<TimeMachineSnapshot>()
				.x((d) => x(d.daysFromNow))
				.y((d) => y(d.callPrice))
				.curve(curveMonotoneX)(snapshots),
			putLine: line<TimeMachineSnapshot>()
				.x((d) => x(d.daysFromNow))
				.y((d) => y(d.putPrice))
				.curve(curveMonotoneX)(snapshots),
			dteMarkers: KEY_DTES.filter((dte) => dte > 0 && dte < totalDays).map((dte) => ({
				dte,
				x: x(totalDays - dte)
			})),
			// The theta acceleration zone starts ~45 DTE.
			accelX: accelDay > 0 && accelDay < totalDays * 0.8 ? x(accelDay) : null
		};
	});

	/** Lets d3 draw an axis into the `<g>`; re-runs whenever the generator (its scale) changes. */
	function axis<Domain extends AxisDomain>(generator: Axis<Domain>): Attachment<SVGGElement> {
		return (g) => {
			select(g).call(generator);
		};
	}
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-3">
		<button
			type="button"
			onclick={() => (showBoth = !showBoth)}
			class="text-xs px-2.5 py-1 rounded-lg transition-all cursor-pointer"
			style:background={showBoth ? 'var(--calc-accent-glow)' : 'var(--calc-surface)'}
			style:color={showBoth ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
			style:border="1px solid {showBoth ? 'var(--calc-accent)' : 'var(--calc-border)'}"
		>
			{showBoth ? 'Both' : isCall ? 'Call Only' : 'Put Only'}
		</button>

		{#if calc.currentTimeMachineSnapshot}
			<span
				class="text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
			>
				Daily θ: {formatCurrency(calc.currentGreeks.first.theta)}
			</span>
		{/if}
	</div>

	<!-- Chart -->
	<div class="relative w-full" style="min-height: 280px;" bind:clientWidth={width}>
		<svg
			{width}
			height={HEIGHT}
			role="img"
			aria-label="Theta decay: call and put value from today to expiration"
		>
			{#if chart}
				<g transform="translate({margin.left},{margin.top})">
					<path
						d={chart.extrinsicArea}
						fill={isCall ? 'var(--calc-call)' : 'var(--calc-put)'}
						opacity="0.08"
					/>

					<!-- Intrinsic value line -->
					<line
						x1="0"
						x2={chart.innerW}
						y1={chart.intrinsicY}
						y2={chart.intrinsicY}
						stroke="var(--calc-text-muted)"
						stroke-width="1"
						stroke-dasharray="4,4"
						opacity="0.5"
					/>
					<text
						x={chart.innerW - 4}
						y={chart.intrinsicY - 6}
						text-anchor="end"
						fill="var(--calc-text-muted)"
						font-size="9px"
						font-family="var(--calc-font-mono)">Intrinsic ${chart.intrinsic.toFixed(2)}</text
					>

					<!-- Call / put decay lines -->
					{#if showBoth || isCall}
						<path
							d={chart.callLine}
							fill="none"
							stroke="var(--calc-call)"
							stroke-width={isCall ? 2.5 : 1.5}
							opacity={isCall ? 1 : 0.5}
						/>
					{/if}
					{#if showBoth || !isCall}
						<path
							d={chart.putLine}
							fill="none"
							stroke="var(--calc-put)"
							stroke-width={isCall ? 1.5 : 2.5}
							opacity={isCall ? 0.5 : 1}
						/>
					{/if}

					<!-- Key DTE markers -->
					{#each chart.dteMarkers as marker (marker.dte)}
						<line
							x1={marker.x}
							x2={marker.x}
							y1="0"
							y2={innerH}
							stroke="var(--calc-border)"
							stroke-width="1"
							stroke-dasharray="2,4"
							opacity="0.4"
						/>
						<text
							x={marker.x}
							y="-6"
							text-anchor="middle"
							fill="var(--calc-text-muted)"
							font-size="8px"
							font-family="var(--calc-font-mono)">{marker.dte}d</text
						>
					{/each}

					<!-- Theta acceleration zone -->
					{#if chart.accelX !== null}
						<rect
							x={chart.accelX}
							y="0"
							width={chart.innerW - chart.accelX}
							height={innerH}
							fill="var(--calc-warning)"
							opacity="0.03"
						/>
						<text
							x={chart.accelX + 4}
							y="14"
							fill="var(--calc-warning)"
							font-size="9px"
							font-family="var(--calc-font-body)"
							opacity="0.7">Theta acceleration zone</text
						>
					{/if}

					<!-- Current time machine position -->
					{#if calc.timeMachineDay > 0}
						{const dayX = $derived(chart.x(calc.timeMachineDay))}
						<line
							x1={dayX}
							x2={dayX}
							y1="0"
							y2={innerH}
							stroke="var(--calc-accent)"
							stroke-width="2"
							opacity="0.8"
						/>
						<text
							x={dayX + 4}
							y={innerH - 4}
							fill="var(--calc-accent)"
							font-size="9px"
							font-family="var(--calc-font-mono)">Day {calc.timeMachineDay}</text
						>
					{/if}

					<!-- Axes -->
					<g
						class="axis"
						transform="translate(0,{innerH})"
						{@attach axis(
							axisBottom(chart.x)
								.ticks(8)
								.tickFormat((d) => `${Number(d).toFixed(0)}d`)
						)}
					></g>
					<g
						class="axis"
						{@attach axis(
							axisLeft(chart.y)
								.ticks(6)
								.tickFormat((d) => `$${Number(d).toFixed(1)}`)
						)}
					></g>

					<text
						x={chart.innerW / 2}
						y={innerH + 38}
						text-anchor="middle"
						fill="var(--calc-text-muted)"
						font-size="11px"
						font-family="var(--calc-font-body)">Days from Now</text
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
