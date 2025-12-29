<script lang="ts">
	import { workflowCanvas } from '$lib/stores/workflow';
	import type { WorkflowNode } from '$lib/types/workflow';

	interface Props {
		node: WorkflowNode;
	}

	let { node }: Props = $props();

	// Config state initialized with empty object
	let config = $state({} as typeof node.config);

	// Sync node.config prop to config state
	$effect(() => {
		config = { ...node.config };
	});

	function handleUpdate() {
		workflowCanvas.updateNode(node.id, { config });
	}

	function handleClose() {
		workflowCanvas.clearSelection();
	}
</script>

<div class="node-properties">
	<div class="properties-header">
		<h3>Node Properties</h3>
		<button class="close-btn" onclick={handleClose} aria-label="Close properties panel">
			<svg
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M18 6L6 18M6 6l12 12" />
			</svg>
		</button>
	</div>

	<div class="properties-body">
		<div class="form-group">
			<label for="node-title">Title</label>
			<input
				id="node-title"
				type="text"
				bind:value={config['title']}
				onblur={handleUpdate}
				placeholder="Enter node title"
			/>
		</div>

		<div class="form-group">
			<label for="node-description">Description</label>
			<textarea
				id="node-description"
				bind:value={config['description']}
				onblur={handleUpdate}
				placeholder="Enter description"
				rows="3"
			></textarea>
		</div>

		{#if node.node_type === 'action'}
			<div class="form-group">
				<label for="action-type">Action Type</label>
				<select id="action-type" bind:value={config['action_type']} onchange={handleUpdate}>
					<option value="">Select action...</option>
					<option value="add_tag">Add Tag</option>
					<option value="remove_tag">Remove Tag</option>
					<option value="send_email">Send Email</option>
					<option value="create_deal">Create Deal</option>
					<option value="create_task">Create Task</option>
					<option value="send_notification">Send Notification</option>
					<option value="http_request">HTTP Request</option>
				</select>
			</div>
		{/if}

		{#if node.node_type === 'delay'}
			<div class="form-group">
				<label for="delay-seconds">Delay (seconds)</label>
				<input
					id="delay-seconds"
					type="number"
					bind:value={config['delay_seconds']}
					onblur={handleUpdate}
					min="0"
				/>
			</div>
		{/if}

		{#if node.node_type === 'condition'}
			<div class="form-group">
				<label for="condition-logic">Condition Logic</label>
				<select id="condition-logic" bind:value={config['logic']} onchange={handleUpdate}>
					<option value="AND">AND (All must be true)</option>
					<option value="OR">OR (Any must be true)</option>
				</select>
			</div>
		{/if}
	</div>
</div>

<style>
	.node-properties {
		width: 320px;
		background: white;
		border-left: 1px solid #e5e7eb;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.properties-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.5rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.properties-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.close-btn {
		padding: 0.25rem;
		background: none;
		border: none;
		color: #6b7280;
		cursor: pointer;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.close-btn:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.properties-body {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	input[type='text'],
	input[type='number'],
	select,
	textarea {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
		transition: border-color 0.2s;
	}

	input:focus,
	select:focus,
	textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	textarea {
		resize: vertical;
		font-family: inherit;
	}
</style>
