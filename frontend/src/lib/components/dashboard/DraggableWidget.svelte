<script lang="ts">
	import type { DashboardWidget } from '$lib/types/dashboard';
	import { dashboardStore } from '$lib/stores/dashboard';

	// Widget components
	import SystemHealthWidget from './widgets/SystemHealthWidget.svelte';
	import RevenueMRRWidget from './widgets/RevenueMRRWidget.svelte';
	import UserGrowthWidget from './widgets/UserGrowthWidget.svelte';
	import RecentActivityWidget from './widgets/RecentActivityWidget.svelte';
	import GenericWidget from './widgets/GenericWidget.svelte';

	interface Props {
		widget: DashboardWidget;
		gridColumns?: number;
		gridRowHeight?: number;
		ondragstart?: (data: { widget: DashboardWidget }) => void;
		ondragend?: () => void;
		ondrop?: (data: { widget: DashboardWidget; x: number; y: number }) => void;
		onresize?: (data: { widget: DashboardWidget; width: number; height: number }) => void;
	}

	let { widget, gridColumns = 12, gridRowHeight = 80, ondragstart, ondragend, ondrop, onresize }: Props = $props();

	let isRefreshing = $state(false);
	let isDragging = $state(false);
	let isResizing = $state(false);
	let resizeHandle: 'se' | 'e' | 's' | null = $state(null);
	let startX = $state(0);
	let startY = $state(0);
	let startWidth = $state(0);
	let startHeight = $state(0);

	async function handleRefresh() {
		isRefreshing = true;
		await dashboardStore.refreshWidget(widget.id);
		isRefreshing = false;
	}

	async function handleRemove() {
		if (confirm(`Remove widget "${widget.title}"?`)) {
			await dashboardStore.removeWidget(widget.id);
		}
	}

	function handleDragStart(event: DragEvent) {
		isDragging = true;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
			event.dataTransfer.setData('text/plain', widget.id);
		}
		ondragstart?.({ widget });
	}

	function handleDragEnd() {
		isDragging = false;
		ondragend?.();
	}

	function handleResizeStart(event: MouseEvent, handle: 'se' | 'e' | 's') {
		event.preventDefault();
		event.stopPropagation();

		isResizing = true;
		resizeHandle = handle;
		startX = event.clientX;
		startY = event.clientY;
		startWidth = widget.width;
		startHeight = widget.height;

		document.addEventListener('mousemove', handleResizeMove);
		document.addEventListener('mouseup', handleResizeEnd);
	}

	function handleResizeMove(event: MouseEvent) {
		if (!isResizing || !resizeHandle) return;

		const deltaX = event.clientX - startX;
		const deltaY = event.clientY - startY;

		// Calculate grid units moved
		const cellWidth = (window.innerWidth - 48) / gridColumns; // Approximate
		const gridDeltaX = Math.round(deltaX / cellWidth);
		const gridDeltaY = Math.round(deltaY / gridRowHeight);

		let newWidth = startWidth;
		let newHeight = startHeight;

		if (resizeHandle === 'se' || resizeHandle === 'e') {
			newWidth = Math.max(2, startWidth + gridDeltaX);
			newWidth = Math.min(gridColumns, newWidth);
		}

		if (resizeHandle === 'se' || resizeHandle === 's') {
			newHeight = Math.max(2, startHeight + gridDeltaY);
		}

		// Update widget dimensions temporarily (visual feedback)
		widget.width = newWidth;
		widget.height = newHeight;
	}

	function handleResizeEnd() {
		if (isResizing) {
			onresize?.({
				widget,
				width: widget.width,
				height: widget.height
			});
		}

		isResizing = false;
		resizeHandle = null;
		document.removeEventListener('mousemove', handleResizeMove);
		document.removeEventListener('mouseup', handleResizeEnd);
	}

	function getWidgetComponent(type: string) {
		switch (type) {
			case 'system_health':
				return SystemHealthWidget;
			case 'revenue_mrr':
				return RevenueMRRWidget;
			case 'user_growth':
				return UserGrowthWidget;
			case 'recent_activity':
				return RecentActivityWidget;
			default:
				return GenericWidget;
		}
	}

	let WidgetComponent = $derived(getWidgetComponent(widget.widget_type));
	let gridColumn = $derived(`${widget.position_x + 1} / span ${widget.width}`);
	let gridRow = $derived(`${widget.position_y + 1} / span ${widget.height}`);
</script>

<div
	class="draggable-widget"
	class:dragging={isDragging}
	class:resizing={isResizing}
	style="
    grid-column: {gridColumn};
    grid-row: {gridRow};
  "
	draggable="true"
	role="button"
	tabindex="0"
	aria-label="Draggable widget: {widget.title}"
	ondragstart={handleDragStart}
	ondragend={handleDragEnd}
>
	<div class="widget-header">
		<div class="drag-handle" title="Drag to move">
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="9" cy="5" r="1" /><circle cx="9" cy="12" r="1" /><circle cx="9" cy="19" r="1" />
				<circle cx="15" cy="5" r="1" /><circle cx="15" cy="12" r="1" /><circle
					cx="15"
					cy="19"
					r="1"
				/>
			</svg>
		</div>
		<h3 class="widget-title">{widget.title}</h3>
		<div class="widget-actions">
			<button
				class="action-btn"
				onclick={handleRefresh}
				disabled={isRefreshing}
				title="Refresh"
				aria-label="Refresh widget"
			>
				<svg
					class:spin={isRefreshing}
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"
					/>
				</svg>
			</button>
			<button class="action-btn" onclick={handleRemove} title="Remove" aria-label="Remove widget">
				<svg
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M18 6L6 18M6 6l12 12" />
				</svg>
			</button>
		</div>
	</div>

	<div class="widget-content">
		<WidgetComponent data={widget.data} config={widget.config} />
	</div>

	<!-- Resize handles -->
	<div
		class="resize-handle resize-e"
		role="button"
		tabindex="0"
		aria-label="Resize widget horizontally"
		onmousedown={(e) => handleResizeStart(e, 'e')}
	></div>
	<div
		class="resize-handle resize-s"
		role="button"
		tabindex="0"
		aria-label="Resize widget vertically"
		onmousedown={(e) => handleResizeStart(e, 's')}
	></div>
	<div
		class="resize-handle resize-se"
		role="button"
		tabindex="0"
		aria-label="Resize widget diagonally"
		onmousedown={(e) => handleResizeStart(e, 'se')}
	></div>
</div>

<style>
	.draggable-widget {
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		transition:
			box-shadow 0.2s,
			transform 0.2s;
		position: relative;
		cursor: grab;
	}

	.draggable-widget:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.draggable-widget.dragging {
		opacity: 0.5;
		cursor: grabbing;
		transform: scale(1.02);
	}

	.draggable-widget.resizing {
		transition: none;
	}

	.widget-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
		background: #fafafa;
		cursor: grab;
	}

	.draggable-widget.dragging .widget-header {
		cursor: grabbing;
	}

	.drag-handle {
		color: #9ca3af;
		display: flex;
		align-items: center;
		cursor: grab;
	}

	.widget-title {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
		flex: 1;
	}

	.widget-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		background: none;
		border: none;
		padding: 0.25rem;
		cursor: pointer;
		color: #6b7280;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #e5e7eb;
		color: #1f2937;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.widget-content {
		flex: 1;
		padding: 1.25rem;
		overflow: auto;
	}

	/* Resize handles */
	.resize-handle {
		position: absolute;
		background: #3b82f6;
		opacity: 0;
		transition: opacity 0.2s;
		z-index: 10;
	}

	.draggable-widget:hover .resize-handle {
		opacity: 0.6;
	}

	.resize-handle:hover {
		opacity: 1 !important;
	}

	.resize-e {
		right: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 4px;
		height: 40%;
		cursor: ew-resize;
		border-radius: 2px 0 0 2px;
	}

	.resize-s {
		bottom: 0;
		left: 50%;
		transform: translateX(-50%);
		height: 4px;
		width: 40%;
		cursor: ns-resize;
		border-radius: 2px 2px 0 0;
	}

	.resize-se {
		right: 0;
		bottom: 0;
		width: 16px;
		height: 16px;
		cursor: nwse-resize;
		border-radius: 12px 0 0 0;
	}

	.spin {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
