<script lang="ts">
	import type { DashboardWidget } from '$lib/types/dashboard';

	interface Props {
		widget: DashboardWidget;
		ondragstart?: () => void;
		ondragend?: () => void;
		onlayoutchange?: (event: CustomEvent<any>) => void;
	}

	let { widget, ondragstart, ondragend, onlayoutchange }: Props = $props();

	function handleDragStart() {
		ondragstart?.();
	}

	function handleDragEnd() {
		ondragend?.();
	}

	function handleLayoutChange(detail: any) {
		onlayoutchange?.(new CustomEvent('layoutchange', { detail }));
	}
</script>

<div
	class="widget-card"
	role="button"
	tabindex="0"
	draggable="true"
	ondragstart={handleDragStart}
	ondragend={handleDragEnd}
	style="
		grid-column: span {widget.width};
		grid-row: span {widget.height};
	"
>
	<div class="widget-header">
		<h3>{widget.title}</h3>
		<div class="widget-actions">
			<button class="drag-handle" title="Drag to reorder">⋮⋮</button>
		</div>
	</div>
	<div class="widget-content">
		{#if widget.data}
			<pre>{JSON.stringify(widget.data, null, 2)}</pre>
		{:else}
			<p class="no-data">No data available</p>
		{/if}
	</div>
</div>

<style>
	.widget-card {
		background: white;
		border-radius: 8px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		transition: box-shadow 0.2s;
	}

	.widget-card:hover {
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}

	.widget-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.widget-header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #111827;
	}

	.widget-actions {
		display: flex;
		gap: 0.5rem;
	}

	.drag-handle {
		background: none;
		border: none;
		cursor: grab;
		padding: 0.25rem;
		color: #6b7280;
		font-size: 1rem;
		line-height: 1;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	.widget-content {
		flex: 1;
		padding: 1rem;
		overflow: auto;
	}

	.no-data {
		color: #9ca3af;
		text-align: center;
		padding: 2rem;
	}

	pre {
		margin: 0;
		font-size: 0.875rem;
		color: #374151;
		white-space: pre-wrap;
		word-wrap: break-word;
	}
</style>
