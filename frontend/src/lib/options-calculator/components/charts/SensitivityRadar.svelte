<script lang="ts">
	import { curveLinearClosed, lineRadial } from 'd3-shape';
	import { Tween, prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { getCalculator } from '../../state/context.js';

	const calc = getCalculator();

	const MAX_SIZE = 420;
	const GRID_LEVELS = [0.2, 0.4, 0.6, 0.8, 1.0];
	const ICONS = ['S', 'σ', 'T', 'r', 'q'];

	/** GSAP's `back.out(1.4)`: overshoots a little, then settles. */
	function backOut(t: number) {
		const u = t - 1;
		return u * u * (2.4 * u + 1.4) + 1;
	}

	let containerWidth = $state(0);
	/** The radar is square: as wide as the container, up to `MAX_SIZE`. */
	const size = $derived(Math.min(containerWidth, MAX_SIZE));

	const maxSens = $derived(Math.max(...calc.sensitivityData.map((d) => d.sensitivity), 0.01));

	/** Each axis' sensitivity as a fraction of the largest, tweened so the polygon morphs. */
	const reach = Tween.of(() => calc.sensitivityData.map((d) => d.sensitivity / maxSens), {
		duration: () => (prefersReducedMotion.current ? 0 : 400),
		easing: cubicOut
	});

	/** Grid and axis geometry; `null` until the container has been measured. */
	const chart = $derived.by(() => {
		const data = calc.sensitivityData;
		const radius = size / 2 - 60;
		if (data.length === 0 || radius <= 0) return null;

		const angleSlice = (2 * Math.PI) / data.length;
		const angleOf = (i: number) => i * angleSlice - Math.PI / 2;

		return {
			radius,
			angleSlice,
			grid: GRID_LEVELS.map((level) => {
				const r = radius * level;
				return {
					level,
					r,
					points: Array.from({ length: data.length + 1 }, (_, i) => {
						const angle = angleOf(i);
						return `${r * Math.cos(angle)},${r * Math.sin(angle)}`;
					}).join(' ')
				};
			}),
			axes: data.map((d, i) => {
				const angle = angleOf(i);
				const labelR = radius + 20;
				return {
					parameter: d.parameter,
					label: d.label,
					icon: ICONS[i] ?? '',
					title: `${d.label}: A 1% change causes a ${(d.sensitivity * 100).toFixed(1)}% change in option price`,
					cos: Math.cos(angle),
					sin: Math.sin(angle),
					labelX: labelR * Math.cos(angle),
					labelY: labelR * Math.sin(angle)
				};
			})
		};
	});

	/** The (tweened) sensitivity polygon. */
	const polygon = $derived.by(() => {
		if (!chart) return null;
		const { radius, angleSlice } = chart;
		return lineRadial<number>()
			.angle((_, i) => i * angleSlice)
			.radius((d) => d * radius)
			.curve(curveLinearClosed)(reach.current);
	});

	/** Where each (tweened) value sits on its axis. */
	const vertices = $derived.by(() => {
		if (!chart) return [];
		const { axes, radius } = chart;
		return axes.map((axis, i) => {
			const r = (reach.current[i] ?? 0) * radius;
			return { parameter: axis.parameter, title: axis.title, x: r * axis.cos, y: r * axis.sin };
		});
	});

	const description = $derived(
		`Sensitivity radar: ${calc.sensitivityData
			.map((d) => `${d.label} ${d.sensitivity.toFixed(3)}`)
			.join(', ')}`
	);
</script>

<div class="flex flex-col gap-3">
	<div class="text-xs" style="color: var(--calc-text-muted); font-family: var(--calc-font-body);">
		Sensitivity shows how a 1% change in each parameter affects the option price (elasticity).
	</div>

	<div
		class="relative w-full flex justify-center"
		style="min-height: 300px;"
		bind:clientWidth={containerWidth}
	>
		<svg width={size} height={size} role="img" aria-label={description}>
			{#if chart}
				<g transform="translate({size / 2},{size / 2})">
					<!-- Grid rings -->
					{#each chart.grid as ring (ring.level)}
						<polygon
							points={ring.points}
							fill="none"
							stroke="var(--calc-border)"
							stroke-width="0.5"
							opacity="0.5"
						/>
						<text
							x="4"
							y={-ring.r - 2}
							fill="var(--calc-text-muted)"
							font-size="8px"
							font-family="var(--calc-font-mono)">{(maxSens * ring.level).toFixed(2)}</text
						>
					{/each}

					<!-- Axis lines and labels -->
					{#each chart.axes as axis (axis.parameter)}
						<line
							x1="0"
							y1="0"
							x2={chart.radius * axis.cos}
							y2={chart.radius * axis.sin}
							stroke="var(--calc-border)"
							stroke-width="1"
							opacity="0.4"
						/>
						<text
							x={axis.labelX}
							y={axis.labelY}
							text-anchor="middle"
							dominant-baseline="middle"
							fill="var(--calc-text-secondary)"
							font-size="10px"
							font-family="var(--calc-font-body)">{axis.label}</text
						>
						<text
							x={axis.labelX}
							y={axis.labelY + 14}
							text-anchor="middle"
							fill="var(--calc-accent)"
							font-size="11px"
							font-family="var(--calc-font-mono)"
							font-weight="600">{axis.icon}</text
						>
					{/each}

					<!-- Sensitivity polygon: grows in on mount, then morphs between inputs -->
					<path
						class="radar-shape"
						d={polygon}
						fill="var(--calc-accent)"
						fill-opacity="0.15"
						stroke="var(--calc-accent)"
						stroke-width="2"
						in:scale={{
							duration: prefersReducedMotion.current ? 0 : 800,
							easing: backOut,
							start: 0,
							opacity: 0
						}}
					/>

					<!-- Data points, each with a native tooltip on a wider hover target -->
					{#each vertices as vertex (vertex.parameter)}
						<circle
							cx={vertex.x}
							cy={vertex.y}
							r="4"
							fill="var(--calc-accent)"
							stroke="var(--calc-bg)"
							stroke-width="2"
						/>
					{/each}
					{#each vertices as vertex (vertex.parameter)}
						<circle cx={vertex.x} cy={vertex.y} r="16" fill="transparent" cursor="pointer">
							<title>{vertex.title}</title>
						</circle>
					{/each}
				</g>
			{/if}
		</svg>
	</div>

	<!-- Legend -->
	<div class="flex flex-wrap gap-3 justify-center">
		{#each calc.sensitivityData as item (item.parameter)}
			<div
				class="flex items-center gap-1.5 text-xs"
				style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
			>
				<span class="w-2 h-2 rounded-full" style="background: var(--calc-accent);"></span>
				{item.label}: {item.sensitivity.toFixed(3)}
			</div>
		{/each}
	</div>
</div>

<style>
	/* Scale about the polygon's own centre, as the entrance animation always has. */
	.radar-shape {
		transform-box: fill-box;
		transform-origin: center;
	}
</style>
