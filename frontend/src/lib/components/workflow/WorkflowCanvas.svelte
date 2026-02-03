<script lang="ts">
	import { onMount } from 'svelte';
	import { workflowCanvas } from '$lib/stores/workflow.svelte';
	import WorkflowNode from './WorkflowNode.svelte';
	import WorkflowEdge from './WorkflowEdge.svelte';
	import type { NodeType } from '$lib/types/workflow';

	interface Props {
		workflowId: number;
	}

	let { workflowId }: Props = $props();

	let canvasElement: HTMLDivElement;
	let isPanning = $state(false);
	let startPan = $state({ x: 0, y: 0 });
	let isConnecting = $state(false);
	let connectionStart: number | null = $state(null);
	let mousePos = $state({ x: 0, y: 0 });

	// Local derived from getters
	const pan = $derived(workflowCanvas.pan);
	const zoom = $derived(workflowCanvas.zoom);
	const nodes = $derived(workflowCanvas.nodes);
	const edges = $derived(workflowCanvas.edges);
	const selectedNode = $derived(workflowCanvas.selectedNode);

	function handleCanvasMouseDown(e: MouseEvent) {
		if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
			// Middle mouse or Shift+Left mouse for panning
			isPanning = true;
			startPan = {
				x: e.clientX - pan.x,
				y: e.clientY - pan.y
			};
			e.preventDefault();
		} else {
			workflowCanvas.clearSelection();
		}
	}

	function handleCanvasMouseMove(e: MouseEvent) {
		mousePos = { x: e.clientX, y: e.clientY };

		if (isPanning) {
			workflowCanvas.setPan({
				x: e.clientX - startPan.x,
				y: e.clientY - startPan.y
			});
		}
	}

	function handleCanvasMouseUp() {
		isPanning = false;

		if (isConnecting && connectionStart !== null) {
			// Connection ended without target
			isConnecting = false;
			connectionStart = null;
		}
	}

	function handleWheel(e: WheelEvent) {
		e.preventDefault();
		const delta = e.deltaY > 0 ? -0.1 : 0.1;
		workflowCanvas.setZoom(zoom + delta);
	}

	function handleNodeDrop(e: DragEvent) {
		e.preventDefault();

		const nodeType = e.dataTransfer?.getData('nodeType') as NodeType;
		if (!nodeType) return;

		const rect = canvasElement.getBoundingClientRect();
		const x = (e.clientX - rect.left - pan.x) / zoom;
		const y = (e.clientY - rect.top - pan.y) / zoom;

		// Create new node
		const newNode = {
			id: Date.now(), // Temporary ID
			workflow_id: workflowId,
			node_type: nodeType,
			config: {},
			position_x: Math.round(x),
			position_y: Math.round(y),
			order: nodes.length,
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		workflowCanvas.addNode(newNode);
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'copy';
		}
	}

	function startConnection(nodeId: number) {
		isConnecting = true;
		connectionStart = nodeId;
	}

	function endConnection(nodeId: number) {
		if (isConnecting && connectionStart !== null && connectionStart !== nodeId) {
			// Create edge
			const newEdge = {
				id: Date.now(),
				workflow_id: workflowId,
				from_node_id: connectionStart,
				to_node_id: nodeId,
				condition_type: 'always' as const,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString()
			};

			workflowCanvas.addEdge(newEdge);
		}

		isConnecting = false;
		connectionStart = null;
	}

	onMount(() => {
		// Load workflow data
		// workflowApi.getWorkflowNodes(workflowId).then(nodes => {
		//   workflowApi.getWorkflowEdges(workflowId).then(edges => {
		//     workflowCanvas.loadWorkflow(nodes, edges);
		//   });
		// });
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	class="workflow-canvas"
	bind:this={canvasElement}
	onmousedown={handleCanvasMouseDown}
	onmousemove={handleCanvasMouseMove}
	onmouseup={handleCanvasMouseUp}
	onwheel={handleWheel}
	ondrop={handleNodeDrop}
	ondragover={handleDragOver}
	role="application"
	tabindex="0"
>
	<div
		class="canvas-content"
		style="
      transform: translate({pan.x}px, {pan.y}px) scale({zoom});
      transform-origin: 0 0;
    "
	>
		<!-- Render edges first (behind nodes) -->
		{#each edges as edge (edge.id)}
			<WorkflowEdge {edge} {nodes} />
		{/each}

		<!-- Render nodes -->
		{#each nodes as node (node.id)}
			<WorkflowNode
				{node}
				selected={selectedNode?.id === node.id}
				onselect={() => workflowCanvas.selectNode(node)}
				onmove={(pos) => workflowCanvas.moveNode(node.id, pos)}
				ondelete={() => workflowCanvas.deleteNode(node.id)}
				onstartConnection={() => startConnection(node.id)}
				onendConnection={() => endConnection(node.id)}
			/>
		{/each}

		<!-- Connection line while dragging -->
		{#if isConnecting && connectionStart !== null}
			{@const startNode = nodes.find((n) => n.id === connectionStart)}
			{#if startNode}
				<svg class="connection-line">
					<line
						x1={startNode.position_x + 100}
						y1={startNode.position_y + 40}
						x2={(mousePos.x - pan.x) / zoom}
						y2={(mousePos.y - pan.y) / zoom}
						stroke="#3b82f6"
						stroke-width="2"
						stroke-dasharray="5,5"
					/>
				</svg>
			{/if}
		{/if}
	</div>

	<!-- Canvas controls -->
	<div class="canvas-controls">
		<button onclick={() => workflowCanvas.setZoom(zoom + 0.1)} aria-label="Zoom in">
			<svg
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" /><path d="M11 8v6M8 11h6" />
			</svg>
		</button>
		<button
			onclick={() => workflowCanvas.setZoom(zoom - 0.1)}
			aria-label="Zoom out"
		>
			<svg
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" /><path d="M8 11h6" />
			</svg>
		</button>
		<button onclick={() => workflowCanvas.setZoom(1)} aria-label="Reset zoom"> Reset </button>
		<span class="zoom-level">{Math.round(zoom * 100)}%</span>
	</div>
</div>

<style>
	.workflow-canvas {
		position: relative;
		width: 100%;
		height: 100%;
		background: #f9fafb;
		background-image:
			linear-gradient(#e5e7eb 1px, transparent 1px),
			linear-gradient(90deg, #e5e7eb 1px, transparent 1px);
		background-size: 20px 20px;
		overflow: hidden;
		cursor: grab;
	}

	.workflow-canvas:active {
		cursor: grabbing;
	}

	.canvas-content {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.connection-line {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
		z-index: 1;
	}

	.canvas-controls {
		position: absolute;
		bottom: 20px;
		right: 20px;
		display: flex;
		gap: 0.5rem;
		background: white;
		padding: 0.5rem;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.canvas-controls button {
		padding: 0.5rem;
		background: #f3f4f6;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.canvas-controls button:hover {
		background: #e5e7eb;
	}

	.zoom-level {
		display: flex;
		align-items: center;
		padding: 0 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
		font-weight: 500;
	}
</style>
