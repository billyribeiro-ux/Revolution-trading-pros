<script lang="ts">
	import { select } from 'd3-selection';
	import { scaleLinear } from 'd3-scale';
	import { lineRadial, curveLinearClosed } from 'd3-shape';
	import gsap from 'gsap';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let containerEl: HTMLDivElement | undefined = $state();
	let svgEl: SVGSVGElement | undefined = $state();
	let size = $state(350);

	$effect(() => {
		if (!containerEl) return;
		const observer = new ResizeObserver((entries) => {
			for (const entry of entries) {
				size = Math.min(entry.contentRect.width, 420);
			}
		});
		observer.observe(containerEl);
		return () => observer.disconnect();
	});

	$effect(() => {
		const data = calc.sensitivityData;
		if (!svgEl || data.length === 0) return;

		const svg = select(svgEl);
		svg.selectAll('*').remove();

		const cx = size / 2;
		const cy = size / 2;
		const radius = size / 2 - 60;
		const numAxes = data.length;
		const angleSlice = (2 * Math.PI) / numAxes;

		const g = svg
			.attr('width', size)
			.attr('height', size)
			.append('g')
			.attr('transform', `translate(${cx},${cy})`);

		// Max sensitivity for scale
		const maxSens = Math.max(...data.map((d) => d.sensitivity), 0.01);
		const rScale = scaleLinear().domain([0, maxSens]).range([0, radius]);

		// Grid circles
		const gridLevels = [0.2, 0.4, 0.6, 0.8, 1.0];
		for (const level of gridLevels) {
			const r = radius * level;
			const points: string[] = [];
			for (let i = 0; i <= numAxes; i++) {
				const angle = i * angleSlice - Math.PI / 2;
				points.push(`${r * Math.cos(angle)},${r * Math.sin(angle)}`);
			}
			g.append('polygon')
				.attr('points', points.join(' '))
				.attr('fill', 'none')
				.attr('stroke', 'var(--calc-border)')
				.attr('stroke-width', 0.5)
				.attr('opacity', 0.5);

			g.append('text')
				.attr('x', 4)
				.attr('y', -r - 2)
				.attr('fill', 'var(--calc-text-muted)')
				.attr('font-size', '8px')
				.attr('font-family', 'var(--calc-font-mono)')
				.text((maxSens * level).toFixed(2));
		}

		// Axis lines and labels
		const ICONS = ['S', 'σ', 'T', 'r', 'q'];
		for (let i = 0; i < numAxes; i++) {
			const angle = i * angleSlice - Math.PI / 2;
			const lineX = radius * Math.cos(angle);
			const lineY = radius * Math.sin(angle);

			g.append('line')
				.attr('x1', 0).attr('y1', 0)
				.attr('x2', lineX).attr('y2', lineY)
				.attr('stroke', 'var(--calc-border)')
				.attr('stroke-width', 1)
				.attr('opacity', 0.4);

			const labelR = radius + 20;
			const labelX = labelR * Math.cos(angle);
			const labelY = labelR * Math.sin(angle);

			g.append('text')
				.attr('x', labelX)
				.attr('y', labelY)
				.attr('text-anchor', 'middle')
				.attr('dominant-baseline', 'middle')
				.attr('fill', 'var(--calc-text-secondary)')
				.attr('font-size', '10px')
				.attr('font-family', 'var(--calc-font-body)')
				.text(data[i].label);

			g.append('text')
				.attr('x', labelX)
				.attr('y', labelY + 14)
				.attr('text-anchor', 'middle')
				.attr('fill', 'var(--calc-accent)')
				.attr('font-size', '11px')
				.attr('font-family', 'var(--calc-font-mono)')
				.attr('font-weight', '600')
				.text(ICONS[i] ?? '');
		}

		// Sensitivity polygon
		const radarLine = lineRadial<number>()
			.angle((_, i) => i * angleSlice)
			.radius((d) => rScale(d))
			.curve(curveLinearClosed);

		const sensValues = data.map((d) => d.sensitivity);

		// Filled polygon
		g.append('path')
			.datum(sensValues)
			.attr('d', radarLine)
			.attr('fill', 'var(--calc-accent)')
			.attr('fill-opacity', 0.15)
			.attr('stroke', 'var(--calc-accent)')
			.attr('stroke-width', 2)
			.attr('opacity', 0);

		// Animate polygon entrance
		const polygonPath = g.select('path:last-child');
		gsap.fromTo(
			polygonPath.node(),
			{ opacity: 0, scale: 0, transformOrigin: 'center center' },
			{ opacity: 1, scale: 1, duration: 0.8, ease: 'back.out(1.4)' },
		);

		// Data points
		for (let i = 0; i < numAxes; i++) {
			const angle = i * angleSlice - Math.PI / 2;
			const r = rScale(data[i].sensitivity);
			const dotX = r * Math.cos(angle);
			const dotY = r * Math.sin(angle);

			g.append('circle')
				.attr('cx', dotX)
				.attr('cy', dotY)
				.attr('r', 4)
				.attr('fill', 'var(--calc-accent)')
				.attr('stroke', 'var(--calc-bg)')
				.attr('stroke-width', 2);
		}

		// Hover areas for tooltips
		for (let i = 0; i < numAxes; i++) {
			const angle = i * angleSlice - Math.PI / 2;
			const r = rScale(data[i].sensitivity);
			const dotX = r * Math.cos(angle);
			const dotY = r * Math.sin(angle);

			g.append('circle')
				.attr('cx', dotX)
				.attr('cy', dotY)
				.attr('r', 16)
				.attr('fill', 'transparent')
				.attr('cursor', 'pointer')
				.append('title')
				.text(`${data[i].label}: A 1% change causes a ${(data[i].sensitivity * 100).toFixed(1)}% change in option price`);
		}
	});
</script>

<div class="flex flex-col gap-3">
	<div class="text-xs" style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">
		Sensitivity shows how a 1% change in each parameter affects the option price (elasticity).
	</div>

	<div bind:this={containerEl} class="relative w-full flex justify-center" style="min-height: 300px;">
		<svg bind:this={svgEl}></svg>
	</div>

	<!-- Legend -->
	<div class="flex flex-wrap gap-3 justify-center">
		{#each calc.sensitivityData as item (item.parameter)}
			<div class="flex items-center gap-1.5 text-xs" style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);">
				<span class="w-2 h-2 rounded-full" style="background: var(--calc-accent);"></span>
				{item.label}: {item.sensitivity.toFixed(3)}
			</div>
		{/each}
	</div>
</div>
