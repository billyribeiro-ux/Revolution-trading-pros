<script lang="ts">
	/**
	 * EnterpriseChart Component
	 * Wrapper for chart libraries with consistent styling
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';

	type ChartType = 'line' | 'area' | 'bar' | 'candlestick' | 'pie' | 'donut';

	interface DataPoint {
		x: number | string | Date;
		y: number;
	}

	interface Series {
		name: string;
		data: DataPoint[] | number[];
		color?: string;
	}

	interface Props {
		type?: ChartType;
		series: Series[];
		height?: number | string;
		width?: number | string;
		title?: string;
		loading?: boolean;
		className?: string;
	}

	let props: Props = $props();

	// Derived values with defaults
	let type = $derived(props.type ?? 'line');
	let series = $derived(props.series);
	let height = $derived(props.height ?? 300);
	let width = $derived(props.width ?? '100%');
	let title = $derived(props.title);
	let loading = $derived(props.loading ?? false);
	let className = $derived(props.className ?? '');

	let containerRef = $state<HTMLDivElement | null>(null);
	let chartInstance = $state<unknown>(null);

	onMount(() => {
		if (browser && containerRef) {
			initChart();
		}
	});

	onDestroy(() => {
		if (chartInstance) {
			// Cleanup chart instance
			chartInstance = null;
		}
	});

	function initChart() {
		// Placeholder for chart initialization
		// In production, integrate with lightweight-charts, ApexCharts, or Chart.js
		console.log('[EnterpriseChart] Chart initialized with type:', type);
	}

	$effect(() => {
		if (browser && chartInstance && series) {
			// Update chart data
			console.log('[EnterpriseChart] Data updated');
		}
	});
</script>

<div
	class="enterprise-chart {className}"
	style:height={typeof height === 'number' ? `${height}px` : height}
	style:width={typeof width === 'number' ? `${width}px` : width}
>
	{#if title}
		<h3 class="chart-title">{title}</h3>
	{/if}

	{#if loading}
		<div class="chart-loading">
			<div class="loading-spinner"></div>
			<span>Loading chart...</span>
		</div>
	{:else}
		<div bind:this={containerRef} class="chart-container">
			<!-- Chart renders here -->
			<div class="chart-placeholder">
				<span class="placeholder-icon">📊</span>
				<span class="placeholder-text">{type} chart - {series.length} series</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.enterprise-chart {
		position: relative;
		background: var(--color-bg-card, #ffffff);
		border: 1px solid var(--color-border-default, #e5e7eb);
		border-radius: var(--radius-lg, 0.5rem);
		overflow: hidden;
	}

	.chart-title {
		padding: 1rem 1rem 0;
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-text-primary, #111827);
	}

	.chart-container {
		width: 100%;
		height: 100%;
		min-height: 200px;
	}

	.chart-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: 0.75rem;
		color: var(--color-text-secondary, #6b7280);
	}

	.loading-spinner {
		width: 24px;
		height: 24px;
		border: 2px solid var(--color-border-default, #e5e7eb);
		border-top-color: var(--color-primary, #6366f1);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.chart-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: 0.5rem;
		color: var(--color-text-muted, #9ca3af);
	}

	.placeholder-icon {
		font-size: 2rem;
	}

	.placeholder-text {
		font-size: 0.875rem;
	}
</style>
