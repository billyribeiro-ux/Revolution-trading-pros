<script lang="ts">
	/**
	 * Sparkline - Tiny inline trend chart
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Pure SVG sparkline with stroked polyline + soft area fill underneath.
	 * Color inherits from `currentColor` so it adapts to the parent text color.
	 *
	 * @version 1.0.0
	 */

	interface Props {
		data: number[];
		width?: number;
		height?: number;
		stroke?: string;
	}

	let { data, width = 80, height = 24, stroke = 'currentColor' }: Props = $props();

	let normalized = $derived.by(() => {
		if (!data || data.length < 2) {
			return { line: '', area: '' };
		}
		const min = Math.min(...data);
		const max = Math.max(...data);
		const range = max - min || 1;
		const lastIndex = data.length - 1;
		const points = data.map((value, index) => {
			const x = (index / lastIndex) * width;
			const y = height - ((value - min) / range) * (height - 2) - 1;
			return `${x.toFixed(2)},${y.toFixed(2)}`;
		});
		const line = points.join(' ');
		const area = `0,${height} ${line} ${width},${height}`;
		return { line, area };
	});
</script>

{#if data && data.length > 1}
	<svg
		{width}
		{height}
		viewBox="0 0 {width} {height}"
		preserveAspectRatio="none"
		role="img"
		aria-label="Sparkline trend"
		class="sparkline"
	>
		<polygon points={normalized.area} fill={stroke} fill-opacity="0.12" stroke="none" />
		<polyline
			points={normalized.line}
			fill="none"
			stroke={stroke}
			stroke-width="1.5"
			stroke-linecap="round"
			stroke-linejoin="round"
		/>
	</svg>
{/if}

<style>
	.sparkline {
		display: block;
		overflow: visible;
	}
</style>
