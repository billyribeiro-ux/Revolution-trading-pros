<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { line as d3line, area as d3area, curveMonotoneX } from 'd3-shape';
	import { axisBottom, axisLeft } from 'd3-axis';
	import { formatCurrency } from '../../utils/formatters.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let svgEl: SVGSVGElement | undefined = $state();
	let width = $state(600);
	let height = $state(350);
	let showBoth = $state(true);

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

	$effect(() => {
		const snapshots = calc.timeMachineSnapshots;
		if (!svgEl || snapshots.length === 0) return;

		const optionType = calc.optionType;
		const currentDay = calc.timeMachineDay;
		const intrinsicVal = calc.currentIntrinsic;

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

		const totalDays = snapshots[snapshots.length - 1].daysFromNow;
		const maxPrice = Math.max(
			...snapshots.map((s) => Math.max(s.callPrice, s.putPrice)),
		) * 1.1;

		const x = scaleLinear().domain([0, totalDays]).range([0, innerW]);
		const y = scaleLinear().domain([0, maxPrice]).range([innerH, 0]);

		// Extrinsic value shading for current option type
		const priceAccessor = (s: (typeof snapshots)[0]) =>
			optionType === 'call' ? s.callPrice : s.putPrice;

		const extrinsicArea = d3area<(typeof snapshots)[0]>()
			.x((d) => x(d.daysFromNow))
			.y0(y(intrinsicVal))
			.y1((d) => y(Math.max(priceAccessor(d), intrinsicVal)))
			.curve(curveMonotoneX);

		g.append('path')
			.datum(snapshots)
			.attr('d', extrinsicArea)
			.attr('fill', optionType === 'call' ? 'var(--calc-call)' : 'var(--calc-put)')
			.attr('opacity', 0.08);

		// Intrinsic value line
		g.append('line')
			.attr('x1', 0).attr('x2', innerW)
			.attr('y1', y(intrinsicVal)).attr('y2', y(intrinsicVal))
			.attr('stroke', 'var(--calc-text-muted)')
			.attr('stroke-width', 1).attr('stroke-dasharray', '4,4').attr('opacity', 0.5);

		g.append('text')
			.attr('x', innerW - 4).attr('y', y(intrinsicVal) - 6)
			.attr('text-anchor', 'end')
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '9px').attr('font-family', 'var(--calc-font-mono)')
			.text(`Intrinsic $${intrinsicVal.toFixed(2)}`);

		// Call decay line
		const callLine = d3line<(typeof snapshots)[0]>()
			.x((d) => x(d.daysFromNow))
			.y((d) => y(d.callPrice))
			.curve(curveMonotoneX);

		if (showBoth || optionType === 'call') {
			g.append('path')
				.datum(snapshots)
				.attr('d', callLine)
				.attr('fill', 'none')
				.attr('stroke', 'var(--calc-call)')
				.attr('stroke-width', optionType === 'call' ? 2.5 : 1.5)
				.attr('opacity', optionType === 'call' ? 1 : 0.5);
		}

		// Put decay line
		const putLine = d3line<(typeof snapshots)[0]>()
			.x((d) => x(d.daysFromNow))
			.y((d) => y(d.putPrice))
			.curve(curveMonotoneX);

		if (showBoth || optionType === 'put') {
			g.append('path')
				.datum(snapshots)
				.attr('d', putLine)
				.attr('fill', 'none')
				.attr('stroke', 'var(--calc-put)')
				.attr('stroke-width', optionType === 'put' ? 2.5 : 1.5)
				.attr('opacity', optionType === 'put' ? 1 : 0.5);
		}

		// Key DTE markers
		const keyDTEs = [45, 30, 21, 14, 7, 1];
		for (const dte of keyDTEs) {
			const dayFromNow = totalDays - dte;
			if (dayFromNow > 0 && dayFromNow < totalDays) {
				g.append('line')
					.attr('x1', x(dayFromNow)).attr('x2', x(dayFromNow))
					.attr('y1', 0).attr('y2', innerH)
					.attr('stroke', 'var(--calc-border)')
					.attr('stroke-width', 1).attr('stroke-dasharray', '2,4').attr('opacity', 0.4);

				g.append('text')
					.attr('x', x(dayFromNow)).attr('y', -6)
					.attr('text-anchor', 'middle')
					.attr('fill', 'var(--calc-text-muted)')
					.attr('font-size', '8px').attr('font-family', 'var(--calc-font-mono)')
					.text(`${dte}d`);
			}
		}

		// Theta acceleration zone annotation (~45 DTE)
		const accelDay = totalDays - 45;
		if (accelDay > 0 && accelDay < totalDays * 0.8) {
			g.append('rect')
				.attr('x', x(accelDay))
				.attr('y', 0)
				.attr('width', innerW - x(accelDay))
				.attr('height', innerH)
				.attr('fill', 'var(--calc-warning)')
				.attr('opacity', 0.03);

			g.append('text')
				.attr('x', x(accelDay) + 4).attr('y', 14)
				.attr('fill', 'var(--calc-warning)')
				.attr('font-size', '9px').attr('font-family', 'var(--calc-font-body)')
				.attr('opacity', 0.7)
				.text('Theta acceleration zone');
		}

		// Current time machine position
		if (currentDay > 0) {
			g.append('line')
				.attr('x1', x(currentDay)).attr('x2', x(currentDay))
				.attr('y1', 0).attr('y2', innerH)
				.attr('stroke', 'var(--calc-accent)')
				.attr('stroke-width', 2).attr('opacity', 0.8);

			g.append('text')
				.attr('x', x(currentDay) + 4).attr('y', innerH - 4)
				.attr('fill', 'var(--calc-accent)')
				.attr('font-size', '9px').attr('font-family', 'var(--calc-font-mono)')
				.text(`Day ${currentDay}`);
		}

		// Axes
		const xAxisGen = axisBottom(x).ticks(8).tickFormat((d) => `${Number(d).toFixed(0)}d`);
		const yAxisGen = axisLeft(y).ticks(6).tickFormat((d) => `$${Number(d).toFixed(1)}`);

		g.append('g')
			.attr('transform', `translate(0,${innerH})`)
			.call(xAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)').attr('font-size', '10px');

		g.append('g')
			.call(yAxisGen)
			.attr('color', 'var(--calc-text-muted)')
			.attr('font-family', 'var(--calc-font-mono)').attr('font-size', '10px');

		g.append('text')
			.attr('x', innerW / 2).attr('y', innerH + 38)
			.attr('text-anchor', 'middle')
			.attr('fill', 'var(--calc-text-muted)')
			.attr('font-size', '11px').attr('font-family', 'var(--calc-font-body)')
			.text('Days from Now');
	});
</script>

<div class="flex flex-col gap-3">
	<!-- Controls -->
	<div class="flex items-center gap-3">
		<button
			onclick={() => (showBoth = !showBoth)}
			class="text-xs px-2.5 py-1 rounded-lg transition-all cursor-pointer"
			style={showBoth
				? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent);'
				: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
		>
			{showBoth ? 'Both' : calc.optionType === 'call' ? 'Call Only' : 'Put Only'}
		</button>

		{#if calc.currentTimeMachineSnapshot}
			<span class="text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">
				Daily \u03b8: {formatCurrency(calc.currentGreeks.first.theta)}
			</span>
		{/if}
	</div>

	<!-- Chart -->
	<div bind:this={containerEl} class="relative w-full" style="min-height: 280px;">
		<svg bind:this={svgEl}></svg>
	</div>
</div>
