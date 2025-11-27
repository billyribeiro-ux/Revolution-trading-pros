<script lang="ts">
	import { onMount } from 'svelte';
	import { dashboardStore } from '$lib/stores/dashboard';
	import WidgetCard from './WidgetCard.svelte';
	import type { DashboardWidget } from '$lib/types/dashboard';

	interface Props {
		dashboardType?: 'admin' | 'user';
	}

	let { dashboardType = 'user' }: Props = $props();

	let gridElement: HTMLDivElement;
	let isDragging = $state(false);
	let draggedWidget: DashboardWidget | null = $state(null);

	onMount(async () => {
		await dashboardStore.loadDashboard(dashboardType);
	});

	function handleDragStart(widget: DashboardWidget) {
		isDragging = true;
		draggedWidget = widget;
	}

	function handleDragEnd() {
		isDragging = false;
		draggedWidget = null;
	}

	async function handleLayoutChange(widget: DashboardWidget, newLayout: any) {
		await dashboardStore.updateWidgetLayout(widget.id, newLayout);
	}

	let widgets = $derived($dashboardStore.widgets);
	let gridColumns = $derived($dashboardStore.currentDashboard?.grid_columns || 12);
	let gridGap = $derived($dashboardStore.currentDashboard?.grid_gap || 16);
</script>

<div class="dashboard-container">
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
			class="dashboard-grid"
			style="
        --grid-columns: {gridColumns};
        --grid-gap: {gridGap}px;
      "
		>
			{#each widgets as widget (widget.id)}
				{#if widget.is_visible}
					<WidgetCard
						{widget}
						ondragstart={() => handleDragStart(widget)}
						ondragend={handleDragEnd}
						onlayoutchange={(e) => handleLayoutChange(widget, e.detail)}
					/>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<style>
	.dashboard-container {
		width: 100%;
		height: 100%;
		padding: 1.5rem;
		background: #f8f9fa;
	}

	.dashboard-grid {
		display: grid;
		grid-template-columns: repeat(var(--grid-columns), 1fr);
		gap: var(--grid-gap);
		width: 100%;
		min-height: 100vh;
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
	}

	button:hover {
		background: #2563eb;
	}
</style>
