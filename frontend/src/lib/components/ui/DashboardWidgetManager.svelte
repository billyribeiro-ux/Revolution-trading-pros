<script lang="ts">
	/**
	 * DashboardWidgetManager Component
	 * Manages draggable dashboard widgets
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';

	interface Widget {
		id: string;
		title: string;
		component: Snippet;
		width?: 1 | 2 | 3 | 4;
		height?: 1 | 2;
		visible?: boolean;
	}

	interface Props {
		widgets: Widget[];
		columns?: number;
		gap?: number;
		onReorder?: (widgets: Widget[]) => void;
		onToggleVisibility?: (widgetId: string, visible: boolean) => void;
	}

	const { widgets, columns = 4, gap = 16, onReorder, onToggleVisibility }: Props = $props();

	let draggedWidget = $state<string | null>(null);

	function handleDragStart(widgetId: string) {
		draggedWidget = widgetId;
	}

	function handleDragEnd() {
		draggedWidget = null;
	}

	function handleDrop(targetId: string) {
		if (!draggedWidget || draggedWidget === targetId) return;

		const newWidgets = [...widgets];
		const draggedIndex = newWidgets.findIndex(w => w.id === draggedWidget);
		const targetIndex = newWidgets.findIndex(w => w.id === targetId);

		const [removed] = newWidgets.splice(draggedIndex, 1);
		newWidgets.splice(targetIndex, 0, removed);

		onReorder?.(newWidgets);
		draggedWidget = null;
	}

	function toggleWidget(widgetId: string) {
		const widget = widgets.find(w => w.id === widgetId);
		if (widget) {
			onToggleVisibility?.(widgetId, !widget.visible);
		}
	}

	const visibleWidgets = $derived(widgets.filter(w => w.visible !== false));
</script>

<div class="widget-manager" style:--columns={columns} style:--gap="{gap}px">
	<div class="widget-grid">
		{#each visibleWidgets as widget (widget.id)}
			<div
				class="widget-container"
				class:dragging={draggedWidget === widget.id}
				style:--width={widget.width ?? 1}
				style:--height={widget.height ?? 1}
				draggable="true"
				ondragstart={() => handleDragStart(widget.id)}
				ondragend={handleDragEnd}
				ondragover={(e) => e.preventDefault()}
				ondrop={() => handleDrop(widget.id)}
				role="button"
				tabindex="0"
			>
				<div class="widget-header">
					<h3 class="widget-title">{widget.title}</h3>
					<button
						type="button"
						class="widget-action"
						onclick={() => toggleWidget(widget.id)}
						aria-label="Hide widget"
					>
						×
					</button>
				</div>
				<div class="widget-content">
					{@render widget.component()}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.widget-manager {
		width: 100%;
	}

	.widget-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns), 1fr);
		gap: var(--gap);
	}

	.widget-container {
		grid-column: span var(--width);
		grid-row: span var(--height);
		background: var(--color-bg-card, #ffffff);
		border: 1px solid var(--color-border-default, #e5e7eb);
		border-radius: var(--radius-lg, 0.5rem);
		overflow: hidden;
		transition: box-shadow 0.15s, transform 0.15s;
	}

	.widget-container:hover {
		box-shadow: var(--shadow-md, 0 4px 6px -1px rgba(0, 0, 0, 0.1));
	}

	.widget-container.dragging {
		opacity: 0.5;
		transform: scale(0.98);
	}

	.widget-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--color-border-default, #e5e7eb);
		background: var(--color-bg-secondary, #f9fafb);
		cursor: grab;
	}

	.widget-header:active {
		cursor: grabbing;
	}

	.widget-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin: 0;
		color: var(--color-text-primary, #111827);
	}

	.widget-action {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: var(--radius-sm, 0.25rem);
		color: var(--color-text-muted, #9ca3af);
		cursor: pointer;
		transition: background 0.15s, color 0.15s;
	}

	.widget-action:hover {
		background: var(--color-bg-tertiary, #e5e7eb);
		color: var(--color-text-primary, #111827);
	}

	.widget-content {
		padding: 1rem;
		min-height: 100px;
	}

	/* Mobile responsive */
	@media (max-width: 768px) {
		.widget-grid {
			grid-template-columns: 1fr;
		}

		.widget-container {
			grid-column: span 1 !important;
		}
	}

	@media (min-width: 768px) and (max-width: 1024px) {
		.widget-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
