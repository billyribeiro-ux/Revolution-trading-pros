<!--
	Behavior Heatmap Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user behavior patterns with heatmap representation
	for click tracking, scroll depth, and engagement zones.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { IconClick, IconEye, IconChartBar } from '@tabler/icons-svelte';

	export let data: Array<{
		x: number;
		y: number;
		intensity: number;
		type: 'click' | 'hover' | 'scroll';
	}> = [];
	export let width: number = 800;
	export let height: number = 600;
	export let showLegend: boolean = true;

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;

	onMount(() => {
		if (canvas) {
			ctx = canvas.getContext('2d');
			renderHeatmap();
		}
	});

	function renderHeatmap() {
		if (!ctx) return;

		// Clear canvas
		ctx.clearRect(0, 0, width, height);

		// Create gradient for each data point
		data.forEach((point) => {
			const gradient = ctx!.createRadialGradient(
				point.x,
				point.y,
				0,
				point.x,
				point.y,
				50 * point.intensity
			);

			// Color based on type
			const color = point.type === 'click' ? '255, 193, 7' : point.type === 'hover' ? '96, 165, 250' : '16, 185, 129';

			gradient.addColorStop(0, `rgba(${color}, ${point.intensity})`);
			gradient.addColorStop(0.5, `rgba(${color}, ${point.intensity * 0.5})`);
			gradient.addColorStop(1, `rgba(${color}, 0)`);

			ctx!.fillStyle = gradient;
			ctx!.fillRect(0, 0, width, height);
		});
	}

	$: if (ctx && data) {
		renderHeatmap();
	}
</script>

<div class="heatmap-container">
	<div class="heatmap-header">
		<h3 class="heatmap-title">
			<IconChartBar size={20} />
			Behavior Heatmap
		</h3>
		{#if showLegend}
			<div class="heatmap-legend">
				<div class="legend-item">
					<div class="legend-color click"></div>
					<span>Clicks</span>
				</div>
				<div class="legend-item">
					<div class="legend-color hover"></div>
					<span>Hovers</span>
				</div>
				<div class="legend-item">
					<div class="legend-color scroll"></div>
					<span>Scrolls</span>
				</div>
			</div>
		{/if}
	</div>

	<div class="heatmap-canvas-wrapper">
		<canvas bind:this={canvas} {width} {height} class="heatmap-canvas"></canvas>
	</div>

	<div class="heatmap-stats">
		<div class="stat">
			<IconClick size={18} />
			<span>{data.filter((d) => d.type === 'click').length} Clicks</span>
		</div>
		<div class="stat">
			<IconEye size={18} />
			<span>{data.filter((d) => d.type === 'hover').length} Hovers</span>
		</div>
	</div>
</div>

<style lang="postcss">
	.heatmap-container {
		@apply bg-gray-800/50 rounded-xl p-6 border border-gray-700/50;
	}

	.heatmap-header {
		@apply flex items-center justify-between mb-4;
	}

	.heatmap-title {
		@apply flex items-center gap-2 text-lg font-semibold text-white;
	}

	.heatmap-legend {
		@apply flex items-center gap-4;
	}

	.legend-item {
		@apply flex items-center gap-2 text-sm text-gray-400;
	}

	.legend-color {
		@apply w-4 h-4 rounded;
	}

	.legend-color.click {
		@apply bg-yellow-400;
	}

	.legend-color.hover {
		@apply bg-blue-400;
	}

	.legend-color.scroll {
		@apply bg-green-400;
	}

	.heatmap-canvas-wrapper {
		@apply relative overflow-hidden rounded-lg bg-gray-900/50 mb-4;
	}

	.heatmap-canvas {
		@apply w-full h-auto;
	}

	.heatmap-stats {
		@apply flex items-center gap-6 text-sm text-gray-400;
	}

	.stat {
		@apply flex items-center gap-2;
	}
</style>
