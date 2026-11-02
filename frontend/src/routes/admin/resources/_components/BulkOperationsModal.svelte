<script lang="ts">
	import { IconAlertCircle } from '#lib/icons/index.js';
	import type { AccessLevel } from '#lib/api/room-resources.js';

	type AccessLevelOption = { id: AccessLevel; name: string; color: string };

	type BulkAction = 'publish' | 'unpublish' | 'feature' | 'unfeature' | 'access' | 'delete';

	type Props = {
		selectedCount: number;
		bulkAction: BulkAction;
		bulkAccessLevel: AccessLevel;
		accessLevels: AccessLevelOption[];
		isSaving: boolean;
		onClose: () => void;
		onConfirm: () => void;
	};

	let {
		selectedCount,
		bulkAction = $bindable(),
		bulkAccessLevel = $bindable(),
		accessLevels,
		isSaving,
		onClose,
		onConfirm
	}: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e: MouseEvent) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<div class="modal" role="document">
		<div class="modal-header">
			<h2>Bulk Operations</h2>
			<button class="modal-close" onclick={onClose}>&times;</button>
		</div>
		<div class="modal-body">
			<p class="bulk-info">
				Apply action to <strong>{selectedCount}</strong> selected resources:
			</p>

			<div class="form-group">
				<label for="bulk-action">Action</label>
				<select id="bulk-action" bind:value={bulkAction}>
					<option value="publish">Publish All</option>
					<option value="unpublish">Unpublish All</option>
					<option value="feature">Mark as Featured</option>
					<option value="unfeature">Remove Featured</option>
					<option value="access">Set Access Level</option>
					<option value="delete">Delete All</option>
				</select>
			</div>

			{#if bulkAction === 'access'}
				<div class="form-group">
					<label for="bulk-access">Access Level</label>
					<select id="bulk-access" bind:value={bulkAccessLevel}>
						{#each accessLevels as level (level.id)}
							<option value={level.id}>{level.name}</option>
						{/each}
					</select>
				</div>
			{/if}

			{#if bulkAction === 'delete'}
				<div class="alert alert-error">
					<IconAlertCircle size={18} />
					This will permanently delete {selectedCount} resources. This cannot be undone.
				</div>
			{/if}
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}>Cancel</button>
			<button
				class={['btn-primary', { 'btn-danger': bulkAction === 'delete' }]}
				onclick={onConfirm}
				disabled={isSaving}
			>
				{#if isSaving}
					<span class="spinner-small"></span>
					Processing...
				{:else if bulkAction === 'delete'}
					Delete {selectedCount} Resources
				{:else}
					Apply to {selectedCount} Resources
				{/if}
			</button>
		</div>
	</div>
</div>

<style>
	/* Modal shell */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: var(--bg-elevated, #161b22);
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.2);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		color: var(--text-primary, #f0f6fc);
	}

	.modal-close {
		background: transparent;
		border: none;
		font-size: 1.5rem;
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
		line-height: 1;
	}

	.modal-close:hover {
		color: var(--text-primary, #f0f6fc);
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(230, 184, 0, 0.2);
	}

	/* Bulk info */
	.bulk-info {
		color: var(--text-secondary, #8b949e);
		margin-bottom: 1.5rem;
	}

	.bulk-info strong {
		color: var(--text-primary, #f0f6fc);
	}

	/* Form */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		font-weight: 500;
	}

	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
	}

	.form-group select:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	/* Alert (delete warning) */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
	}

	.alert-error {
		background: rgba(248, 81, 73, 0.15);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	/* Buttons */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: var(--primary-500);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}


	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-primary.btn-danger {
		background: var(--error-base, #da3633);
	}


	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-secondary, #8b949e);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--text-primary, #f0f6fc);
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Mobile */
	@media (max-width: 639px) {
		.modal-overlay {
			padding: 0.5rem;
			padding-top: calc(0.5rem + env(safe-area-inset-top, 0px));
			padding-bottom: calc(0.5rem + env(safe-area-inset-bottom, 0px));
		}

		.modal {
			max-width: 100%;
			max-height: calc(
				100vh - 1rem - env(safe-area-inset-top, 0px) - env(safe-area-inset-bottom, 0px)
			);
		}

		.modal-header {
			padding: 1rem;
		}

		.modal-header h2 {
			font-size: 1.125rem;
		}

		.modal-body {
			padding: 1rem;
		}

		.modal-footer {
			padding: 0.75rem 1rem;
			flex-direction: column;
			gap: 0.5rem;
		}

		.modal-footer button {
			width: 100%;
			min-height: 44px;
		}

		.form-group label {
			font-size: 0.8125rem;
		}

		.form-group select {
			min-height: 44px;
			font-size: 0.9375rem;
			padding: 0.625rem 0.875rem;
		}

		.btn-primary,
		.btn-secondary {
			min-height: 44px;
			padding: 0.625rem 1rem;
			font-size: 0.875rem;
		}
	}
</style>
