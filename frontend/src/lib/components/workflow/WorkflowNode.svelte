<script lang="ts">
	import type { WorkflowNode as Node } from '$lib/types/workflow';

	interface Props {
		node: Node;
		selected?: boolean;
		onselect?: () => void;
		onmove?: (pos: { x: number; y: number }) => void;
		ondelete?: () => void;
		onstartConnection?: () => void;
		onendConnection?: () => void;
	}

	let {
		node,
		selected = false,
		onselect,
		onmove,
		ondelete,
		onstartConnection,
		onendConnection
	}: Props = $props();

	let isDragging = $state(false);
	let dragStart = $state({ x: 0, y: 0 });

	function handleMouseDown(e: MouseEvent) {
		if (e.button === 0) {
			isDragging = true;
			dragStart = {
				x: e.clientX - node.position_x,
				y: e.clientY - node.position_y
			};
			onselect?.();
			e.stopPropagation();
		}
	}

	function handleMouseMove(e: MouseEvent) {
		if (isDragging) {
			onmove?.({
				x: e.clientX - dragStart.x,
				y: e.clientY - dragStart.y
			});
		}
	}

	function handleMouseUp() {
		isDragging = false;
	}

	function getNodeIcon(type: string): string {
		const icons: Record<string, string> = {
			trigger: '⚡',
			condition: '❓',
			action: '⚙️',
			delay: '⏱️',
			branch: '🔀',
			parallel: '⫸',
			merge: '⊕',
			end: '🏁'
		};
		return icons[type] || '📦';
	}

	function getNodeColor(type: string): string {
		const colors: Record<string, string> = {
			trigger: '#10b981',
			condition: '#f59e0b',
			action: '#3b82f6',
			delay: '#8b5cf6',
			branch: '#ec4899',
			parallel: '#06b6d4',
			merge: '#6366f1',
			end: '#ef4444'
		};
		return colors[type] || '#6b7280';
	}

	let nodeColor = $derived(getNodeColor(node.node_type));
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={handleMouseUp} />

<div
	class="workflow-node"
	class:selected
	class:dragging={isDragging}
	style="
    left: {node.position_x}px;
    top: {node.position_y}px;
    border-color: {nodeColor};
  "
	onmousedown={handleMouseDown}
	role="button"
	tabindex="0"
>
	<div class="node-header" style="background: {nodeColor};">
		<span class="node-icon">{getNodeIcon(node.node_type)}</span>
		<span class="node-type">{node.node_type}</span>
	</div>

	<div class="node-body">
		<div class="node-title">
			{node.config.title || node.config.action_type || 'Untitled'}
		</div>
		{#if node.config.description}
			<div class="node-description">{node.config.description}</div>
		{/if}
	</div>

	<div class="node-actions">
		<button
			class="action-btn"
			onclick={(e: MouseEvent) => {
				e.stopPropagation();
				ondelete?.();
			}}
			title="Delete node"
		>
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path
					d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
				/>
			</svg>
		</button>
	</div>

	<!-- Connection points -->
	<div
		class="connection-point input"
		onmouseup={() => onendConnection?.()}
		role="button"
		tabindex="0"
	></div>
	<div
		class="connection-point output"
		onmousedown={(e: MouseEvent) => {
			e.stopPropagation();
			onstartConnection?.();
		}}
		role="button"
		tabindex="0"
	></div>
</div>

<style>
	.workflow-node {
		position: absolute;
		width: 200px;
		background: white;
		border: 2px solid;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		cursor: move;
		transition:
			box-shadow 0.2s,
			transform 0.2s;
		z-index: 10;
	}

	.workflow-node:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.workflow-node.selected {
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.3);
		z-index: 20;
	}

	.workflow-node.dragging {
		opacity: 0.8;
		cursor: grabbing;
	}

	.node-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		color: white;
		font-weight: 600;
		font-size: 0.875rem;
		border-radius: 6px 6px 0 0;
		text-transform: capitalize;
	}

	.node-icon {
		font-size: 1.125rem;
	}

	.node-body {
		padding: 0.75rem;
	}

	.node-title {
		font-weight: 600;
		font-size: 0.875rem;
		color: #1f2937;
		margin-bottom: 0.25rem;
	}

	.node-description {
		font-size: 0.75rem;
		color: #6b7280;
		line-height: 1.4;
	}

	.node-actions {
		position: absolute;
		top: -12px;
		right: -12px;
		display: none;
	}

	.workflow-node:hover .node-actions {
		display: block;
	}

	.action-btn {
		width: 24px;
		height: 24px;
		padding: 0;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 50%;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.action-btn:hover {
		background: #fee2e2;
		border-color: #fecaca;
		color: #dc2626;
	}

	.connection-point {
		position: absolute;
		width: 12px;
		height: 12px;
		background: white;
		border: 2px solid #3b82f6;
		border-radius: 50%;
		cursor: pointer;
		transition: all 0.2s;
	}

	.connection-point:hover {
		width: 16px;
		height: 16px;
		margin: -2px;
		background: #3b82f6;
	}

	.connection-point.input {
		top: 50%;
		left: -6px;
		transform: translateY(-50%);
	}

	.connection-point.output {
		top: 50%;
		right: -6px;
		transform: translateY(-50%);
	}
</style>
