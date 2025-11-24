<script lang="ts">
	import type { WorkflowEdge as Edge, WorkflowNode } from '$lib/types/workflow';

	export let edge: Edge;
	export let nodes: WorkflowNode[];

	$: fromNode = nodes.find((n) => n.id === edge.from_node_id);
	$: toNode = nodes.find((n) => n.id === edge.to_node_id);

	$: path = fromNode && toNode ? calculatePath(fromNode, toNode) : '';

	function calculatePath(from: WorkflowNode, to: WorkflowNode): string {
		const startX = from.position_x + 200; // Node width
		const startY = from.position_y + 40; // Half node height
		const endX = to.position_x;
		const endY = to.position_y + 40;

		const midX = (startX + endX) / 2;

		return `M ${startX} ${startY} C ${midX} ${startY}, ${midX} ${endY}, ${endX} ${endY}`;
	}

	function getEdgeColor(conditionType: string): string {
		const colors: Record<string, string> = {
			always: '#3b82f6',
			if_true: '#10b981',
			if_false: '#ef4444',
			parallel: '#8b5cf6'
		};
		return colors[conditionType] || '#6b7280';
	}

	$: edgeColor = getEdgeColor(edge.condition_type);
</script>

{#if fromNode && toNode}
	<svg class="workflow-edge">
		<defs>
			<marker
				id="arrowhead-{edge.id}"
				markerWidth="10"
				markerHeight="10"
				refX="9"
				refY="3"
				orient="auto"
			>
				<polygon points="0 0, 10 3, 0 6" fill={edgeColor} />
			</marker>
		</defs>

		<path
			d={path}
			stroke={edgeColor}
			stroke-width="2"
			fill="none"
			marker-end="url(#arrowhead-{edge.id})"
			class="edge-path"
		/>

		{#if edge.label}
			{@const midX = (fromNode.position_x + 200 + toNode.position_x) / 2}
			{@const midY = (fromNode.position_y + 40 + toNode.position_y + 40) / 2}
			<text x={midX} y={midY} text-anchor="middle" class="edge-label" fill={edgeColor}>
				{edge.label}
			</text>
		{/if}
	</svg>
{/if}

<style>
	.workflow-edge {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
		z-index: 1;
	}

	.edge-path {
		transition: stroke 0.2s;
	}

	.edge-path:hover {
		stroke-width: 3;
	}

	.edge-label {
		font-size: 0.75rem;
		font-weight: 600;
		pointer-events: none;
	}
</style>
