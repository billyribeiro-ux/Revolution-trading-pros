<script lang="ts">
	import { onMount } from 'svelte';
	import { dashboardStore } from '$lib/stores/dashboard';
	import DraggableWidget from './DraggableWidget.svelte';
	import type { DashboardWidget } from '$lib/types/dashboard';

	interface Props {
		dashboardType?: 'admin' | 'user';
	}

	let { dashboardType = 'user' }: Props = $props();

	let gridElement: HTMLDivElement;
	let isDragging = $state(false);
	let draggedWidget: DashboardWidget | null = $state(null);
	let dropZones: Map<string, HTMLElement> = new Map();

	onMount(async () => {
		await dashboardStore.loadDashboard(dashboardType);
	});

	function handleDragStart(event: CustomEvent<{ widget: DashboardWidget }>) {
		isDragging = true;
		draggedWidget = event.detail.widget;
	}

	function handleDragEnd() {
		isDragging = false;
		draggedWidget = null;
	}

	async function handleDrop(event: CustomEvent<{ widget: DashboardWidget; x: number; y: number }>) {
		const { widget, x, y } = event.detail;

		await dashboardStore.updateWidgetLayout(widget.id, {
			x,
			y,
			width: widget.width,
			height: widget.height
		});
	}

	async function handleResize(
		event: CustomEvent<{ widget: DashboardWidget; width: number; height: number }>
	) {
		const { widget, width, height } = event.detail;

		await dashboardStore.updateWidgetLayout(widget.id, {
			x: widget.position_x,
			y: widget.position_y,
			width,
			height
		});
	}

	let widgets = $derived($dashboardStore.widgets);
	let gridColumns = $derived($dashboardStore.currentDashboard?.grid_columns || 12);
	let gridGap = $derived($dashboardStore.currentDashboard?.grid_gap || 16);
	let gridRowHeight = $derived($dashboardStore.currentDashboard?.grid_row_height || 80);
</script>

<div class="drag-drop-container">
	{#if $dashboardStore.isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading dashboard...</p>
		</div>
	{:else if $dashboardStore.error}
		<div class="error-state">
			<p class="error-message">{$dashboardStore.error}</p>
			<button onclick={() => dashboardStore.clearError()}>Dismiss</button>
		</div>
	{:else}
		<div
			bind:this={gridElement}
			class="drag-drop-grid"
			class:dragging={isDragging}
			style="
        --grid-columns: {gridColumns};
        --grid-gap: {gridGap}px;
        --grid-row-height: {gridRowHeight}px;
      "
		>
			{#each widgets as widget (widget.id)}
				{#if widget.is_visible}
					<DraggableWidget
						{widget}
						{gridColumns}
						{gridRowHeight}
						ondragstart={handleDragStart}
						ondragend={handleDragEnd}
						ondrop={handleDrop}
						onresize={handleResize}
					/>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<style>
	.drag-drop-container {
		width: 100%;
		height: 100%;
		padding: 1.5rem;
		background: #f8f9fa;
	}

	.drag-drop-grid {
		display: grid;
		grid-template-columns: repeat(var(--grid-columns), 1fr);
		gap: var(--grid-gap);
		width: 100%;
		min-height: calc(100vh - 3rem);
		position: relative;
	}

	.drag-drop-grid.dragging {
		cursor: grabbing;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		gap: 1rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid #e0e0e0;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-state {
		padding: 2rem;
		background: #fee;
		border: 1px solid #fcc;
		border-radius: 8px;
		text-align: center;
	}

	.error-message {
		color: #c00;
		margin-bottom: 1rem;
	}

	button {
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
	}

	button:hover {
		background: #2563eb;
	}
</style>
