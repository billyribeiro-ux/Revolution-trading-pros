<script lang="ts">
	import type { NodeType } from '$lib/types/workflow';

	interface NodeTemplate {
		type: NodeType;
		name: string;
		description: string;
		icon: string;
		color: string;
	}

	const nodeTemplates: NodeTemplate[] = [
		{
			type: 'trigger',
			name: 'Trigger',
			description: 'Start workflow on event',
			icon: '⚡',
			color: '#10b981'
		},
		{
			type: 'condition',
			name: 'Condition',
			description: 'Check if/else logic',
			icon: '❓',
			color: '#f59e0b'
		},
		{
			type: 'action',
			name: 'Action',
			description: 'Perform an action',
			icon: '⚙️',
			color: '#E6B800'
		},
		{
			type: 'delay',
			name: 'Delay',
			description: 'Wait for time period',
			icon: '⏱️',
			color: '#B38F00'
		},
		{
			type: 'branch',
			name: 'Branch',
			description: 'Split into paths',
			icon: '🔀',
			color: '#ec4899'
		},
		{
			type: 'parallel',
			name: 'Parallel',
			description: 'Run simultaneously',
			icon: '⫸',
			color: '#06b6d4'
		}
	];

	function handleDragStart(e: DragEvent, nodeType: NodeType) {
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'copy';
			e.dataTransfer.setData('nodeType', nodeType);
		}
	}
</script>

<div class="node-palette">
	<div class="palette-header">
		<h3>Workflow Nodes</h3>
		<p class="subtitle">Drag nodes to canvas</p>
	</div>

	<div class="node-list">
		{#each nodeTemplates as template}
			<div
				class="node-template"
				draggable="true"
				ondragstart={(e: DragEvent) => handleDragStart(e, template.type)}
				role="button"
				tabindex="0"
				style="border-left-color: {template.color};"
			>
				<div class="template-icon" style="background: {template.color}20; color: {template.color};">
					{template.icon}
				</div>
				<div class="template-info">
					<div class="template-name">{template.name}</div>
					<div class="template-description">{template.description}</div>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.node-palette {
		width: 280px;
		background: white;
		border-right: 1px solid #e5e7eb;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.palette-header {
		padding: 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.palette-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.node-list {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.node-template {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-left: 3px solid;
		border-radius: 8px;
		margin-bottom: 0.75rem;
		cursor: grab;
		transition: all 0.2s;
	}

	.node-template:hover {
		background: white;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transform: translateX(4px);
	}

	.node-template:active {
		cursor: grabbing;
	}

	.template-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.template-info {
		flex: 1;
		min-width: 0;
	}

	.template-name {
		font-weight: 600;
		font-size: 0.875rem;
		color: #1f2937;
		margin-bottom: 0.125rem;
	}

	.template-description {
		font-size: 0.75rem;
		color: #6b7280;
		line-height: 1.4;
	}
</style>
