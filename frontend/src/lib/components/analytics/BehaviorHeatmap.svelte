<!--
	Behavior Heatmap Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user behavior patterns with heatmap representation
	for click tracking, scroll depth, and engagement zones.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { IconClick, IconEye, IconChartBar } from '@tabler/icons-svelte';

	interface DataPoint {
		x: number;
		y: number;
		intensity: number;
		type: 'click' | 'hover' | 'scroll';
	}

	interface Props {
		data?: DataPoint[];
		width?: number;
		height?: number;
		showLegend?: boolean;
	}

	let { data = [], width = 800, height = 600, showLegend = true }: Props = $props();

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
			const color =
				point.type === 'click'
					? '255, 193, 7'
					: point.type === 'hover'
						? '96, 165, 250'
						: '16, 185, 129';

			gradient.addColorStop(0, `rgba(${color}, ${point.intensity})`);
			gradient.addColorStop(0.5, `rgba(${color}, ${point.intensity * 0.5})`);
			gradient.addColorStop(1, `rgba(${color}, 0)`);

			ctx!.fillStyle = gradient;
			ctx!.fillRect(0, 0, width, height);
		});
	}

	$effect(() => {
		if (ctx && data) {
			renderHeatmap();
		}
	});
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
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.heatmap-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.heatmap-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: white;
	}

	.heatmap-legend {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.legend-color {
		width: 1rem;
		height: 1rem;
		border-radius: 0.25rem;
	}

	.legend-color.click {
		background-color: #fbbf24;
	}

	.legend-color.hover {
		background-color: #60a5fa;
	}

	.legend-color.scroll {
		background-color: #4ade80;
	}

	.heatmap-canvas-wrapper {
		position: relative;
		overflow: hidden;
		border-radius: 0.5rem;
		background-color: rgba(17, 24, 39, 0.5);
		margin-bottom: 1rem;
	}

	.heatmap-canvas {
		width: 100%;
		height: auto;
	}

	.heatmap-stats {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
</style>
