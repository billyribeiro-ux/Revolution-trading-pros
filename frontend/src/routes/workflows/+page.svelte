<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { workflowApi } from '$lib/api/workflow';
	import type { Workflow } from '$lib/types/workflow';

	let workflows = $state<Workflow[]>([]);
	let isLoading = $state(true);
	let filter = $state<'all' | 'active' | 'paused'>('all');

	async function loadWorkflows() {
		isLoading = true;
		try {
			workflows = await workflowApi.getWorkflows();
		} catch (error) {
			console.error('Failed to load workflows:', error);
		} finally {
			isLoading = false;
		}
	}

	function createWorkflow() {
		goto('/workflows/new');
	}

	function editWorkflow(id: number) {
		goto(`/workflows/${id}/edit`);
	}

	async function toggleStatus(workflow: Workflow) {
		const newStatus = workflow.status === 'active' ? 'paused' : 'active';
		try {
			await workflowApi.toggleWorkflowStatus(workflow.id, newStatus);
			await loadWorkflows();
		} catch (error) {
			console.error('Failed to toggle status:', error);
		}
	}

	let filteredWorkflows = $derived(
		workflows.filter((w) => {
			if (filter === 'all') return true;
			return w.status === filter;
		})
	);

	onMount(() => {
		loadWorkflows();
	});
</script>

<div class="workflows-page">
	<div class="page-header">
		<div>
			<h1>Workflows</h1>
			<p class="subtitle">Automate your business processes</p>
		</div>
		<button class="btn-primary" onclick={createWorkflow}>
			<svg
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<line x1="12" y1="5" x2="12" y2="19" />
				<line x1="5" y1="12" x2="19" y2="12" />
			</svg>
			Create Workflow
		</button>
	</div>

	<div class="filters">
		<button class:active={filter === 'all'} onclick={() => (filter = 'all')}>
			All ({workflows.length})
		</button>
		<button class:active={filter === 'active'} onclick={() => (filter = 'active')}>
			Active ({workflows.filter((w) => w.status === 'active').length})
		</button>
		<button class:active={filter === 'paused'} onclick={() => (filter = 'paused')}>
			Paused ({workflows.filter((w) => w.status === 'paused').length})
		</button>
	</div>

	{#if isLoading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading workflows...</p>
		</div>
	{:else if filteredWorkflows.length === 0}
		<div class="empty-state">
			<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#9ca3af" stroke-width="2">
				<path
					d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
				/>
				<polyline points="3.27 6.96 12 12.01 20.73 6.96" />
				<line x1="12" y1="22.08" x2="12" y2="12" />
			</svg>
			<h3>No workflows yet</h3>
			<p>Create your first workflow to automate your processes</p>
			<button class="btn-primary" onclick={createWorkflow}>Create Workflow</button>
		</div>
	{:else}
		<div class="workflows-grid">
			{#each filteredWorkflows as workflow}
				<div class="workflow-card">
					<div class="card-header">
						<h3>{workflow.name}</h3>
						<span class="status-badge" class:active={workflow.status === 'active'}>
							{workflow.status}
						</span>
					</div>

					{#if workflow.description}
						<p class="description">{workflow.description}</p>
					{/if}

					<div class="stats">
						<div class="stat">
							<span class="stat-label">Executions</span>
							<span class="stat-value">{workflow.execution_count}</span>
						</div>
						<div class="stat">
							<span class="stat-label">Success Rate</span>
							<span class="stat-value success">
								{workflow.execution_count > 0
									? ((workflow.success_count / workflow.execution_count) * 100).toFixed(1)
									: 0}%
							</span>
						</div>
					</div>

					<div class="card-actions">
						<button class="btn-secondary" onclick={() => editWorkflow(workflow.id)}> Edit </button>
						<button
							class="btn-toggle"
							class:active={workflow.status === 'active'}
							onclick={() => toggleStatus(workflow)}
						>
							{workflow.status === 'active' ? 'Pause' : 'Activate'}
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.workflows-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		font-size: 1rem;
		color: #6b7280;
		margin: 0;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.filters {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
	}

	.filters button {
		padding: 0.5rem 1rem;
		background: #f3f4f6;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filters button:hover {
		background: #e5e7eb;
	}

	.filters button.active {
		background: #3b82f6;
		color: white;
	}

	.loading,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 4rem 2rem;
		text-align: center;
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

	.empty-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
		margin: 1rem 0 0.5rem 0;
	}

	.empty-state p {
		color: #6b7280;
		margin: 0 0 1.5rem 0;
	}

	.workflows-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 1.5rem;
	}

	.workflow-card {
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.2s;
	}

	.workflow-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.card-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		background: #f3f4f6;
		color: #6b7280;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status-badge.active {
		background: #d1fae5;
		color: #065f46;
	}

	.description {
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.5;
		margin: 0 0 1rem 0;
	}

	.stats {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		margin-bottom: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.stat {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #6b7280;
		font-weight: 500;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #1f2937;
	}

	.stat-value.success {
		color: #10b981;
	}

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-secondary,
	.btn-toggle {
		flex: 1;
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary {
		background: #f3f4f6;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #e5e7eb;
	}

	.btn-toggle {
		background: #dbeafe;
		color: #1e40af;
	}

	.btn-toggle:hover {
		background: #bfdbfe;
	}

	.btn-toggle.active {
		background: #fef3c7;
		color: #92400e;
	}
</style>
