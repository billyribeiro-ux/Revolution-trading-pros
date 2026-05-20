<script lang="ts">
	import type { Workflow } from '$lib/types/workflow';

	interface Props {
		workflow?: Workflow | null;
		isSaving?: boolean;
		ontest?: () => void;
		onsave?: () => void;
	}

	let { workflow = null, isSaving = false, ontest, onsave }: Props = $props();
</script>

<div class="workflow-toolbar">
	<div class="toolbar-left">
		<h1 class="workflow-name">{workflow?.name || 'New Workflow'}</h1>
		{#if workflow}
			<span class="workflow-status" class:active={workflow.status === 'active'}>
				{workflow.status}
			</span>
		{/if}
	</div>

	<div class="toolbar-right">
		<button class="btn-secondary" onclick={() => ontest?.()} aria-label="Test workflow">
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<polygon points="5 3 19 12 5 21 5 3" />
			</svg>
			Test Run
		</button>

		<button
			class="btn-primary"
			onclick={() => onsave?.()}
			disabled={isSaving}
			aria-label="Save workflow"
		>
			{#if isSaving}
				<div class="btn-spinner"></div>
				Saving...
			{:else}
				<svg
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
					<polyline points="17 21 17 13 7 13 7 21" />
					<polyline points="7 3 7 8 15 8" />
				</svg>
				Save
			{/if}
		</button>
	</div>
</div>

<style>
	.workflow-toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.5rem;
		background: white;
		border-bottom: 1px solid #e5e7eb;
	}

	.toolbar-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.workflow-name {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.workflow-status {
		padding: 0.25rem 0.75rem;
		background: #f3f4f6;
		color: #6b7280;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.workflow-status.active {
		background: #d1fae5;
		color: #065f46;
	}

	.toolbar-right {
		display: flex;
		gap: 0.75rem;
	}

	.btn-secondary,
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
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

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
