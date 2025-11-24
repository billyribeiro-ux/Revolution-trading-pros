<script lang="ts">
	import { onMount } from 'svelte';
	import WorkflowCanvas from './WorkflowCanvas.svelte';
	import NodePalette from './NodePalette.svelte';
	import NodeProperties from './NodeProperties.svelte';
	import WorkflowToolbar from './WorkflowToolbar.svelte';
	import { workflowCanvas } from '$lib/stores/workflow';
	import { workflowApi } from '$lib/api/workflow';
	import type { Workflow } from '$lib/types/workflow';

	export let workflowId: number | null = null;

	let workflow: Workflow | null = null;
	let isLoading = true;
	let isSaving = false;

	async function loadWorkflow() {
		if (!workflowId) {
			isLoading = false;
			return;
		}

		try {
			workflow = await workflowApi.getWorkflow(workflowId);
			const nodes = await workflowApi.getWorkflowNodes(workflowId);
			const edges = await workflowApi.getWorkflowEdges(workflowId);
			workflowCanvas.loadWorkflow(nodes, edges);
		} catch (error) {
			console.error('Failed to load workflow:', error);
		} finally {
			isLoading = false;
		}
	}

	async function saveWorkflow() {
		if (!workflowId) return;

		isSaving = true;
		try {
			// Save nodes and edges
			for (const node of $workflowCanvas.nodes) {
				if (node.id < 1000000000000) {
					// Existing node
					await workflowApi.updateNode(workflowId, node.id, node);
				} else {
					// New node
					await workflowApi.createNode(workflowId, node);
				}
			}

			for (const edge of $workflowCanvas.edges) {
				if (edge.id < 1000000000000) {
					// Existing edge - no update needed
				} else {
					// New edge
					await workflowApi.createEdge(workflowId, edge);
				}
			}

			alert('Workflow saved successfully!');
		} catch (error) {
			console.error('Failed to save workflow:', error);
			alert('Failed to save workflow');
		} finally {
			isSaving = false;
		}
	}

	async function testWorkflow() {
		if (!workflowId) return;

		try {
			const run = await workflowApi.executeWorkflow(workflowId, {
				test: true,
				timestamp: new Date().toISOString()
			});
			alert(`Workflow execution started! Run ID: ${run.id}`);
		} catch (error) {
			console.error('Failed to execute workflow:', error);
			alert('Failed to execute workflow');
		}
	}

	onMount(() => {
		loadWorkflow();
	});
</script>

<div class="workflow-builder">
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading workflow...</p>
		</div>
	{:else}
		<WorkflowToolbar {workflow} {isSaving} on:save={saveWorkflow} on:test={testWorkflow} />

		<div class="builder-content">
			<NodePalette />

			<div class="canvas-container">
				<WorkflowCanvas workflowId={workflowId || 0} />
			</div>

			{#if $workflowCanvas.selectedNode}
				<NodeProperties node={$workflowCanvas.selectedNode} />
			{/if}
		</div>
	{/if}
</div>

<style>
	.workflow-builder {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: #f9fafb;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100vh;
		gap: 1rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid #e5e7eb;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.builder-content {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.canvas-container {
		flex: 1;
		position: relative;
	}
</style>
