<script lang="ts">
	import { IconLink } from '$lib/icons';
	import type { RoomResource } from '$lib/api/room-resources';

	type Props = {
		resource: RoomResource;
		newFileUrl: string;
		isSaving: boolean;
		onClose: () => void;
		onReplace: () => void;
	};

	let {
		resource,
		newFileUrl = $bindable(),
		isSaving,
		onClose,
		onReplace
	}: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e: MouseEvent) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e: KeyboardEvent) => {
		if (e.key === 'Escape') onClose();
	}}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<div class="modal" role="document">
		<div class="modal-header">
			<h2>Replace Resource</h2>
			<button class="modal-close" onclick={onClose}>&times;</button>
		</div>
		<div class="modal-body">
			<div class="replace-info">
				<p><strong>Current:</strong> {resource.title}</p>
				<p class="text-muted">All metadata will be kept. Only the file URL will be replaced.</p>
			</div>
			<div class="form-group">
				<label for="new-file-url">New File URL</label>
				<!-- svelte-ignore a11y_autofocus -->
				<input
					type="url"
					id="new-file-url"
					name="new-file-url"
					bind:value={newFileUrl}
					placeholder="https://..."
					autofocus
				/>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}>Cancel</button>
			<button class="btn-primary" onclick={onReplace} disabled={isSaving || !newFileUrl}>
				{#if isSaving}
					<span class="spinner-small"></span>
					Replacing...
				{:else}
					<IconLink size={16} />
					Replace
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
		background: #1e293b;
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
		color: #f1f5f9;
	}

	.modal-close {
		background: transparent;
		border: none;
		font-size: 1.5rem;
		color: #64748b;
		cursor: pointer;
		line-height: 1;
	}

	.modal-close:hover {
		color: #f1f5f9;
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

	/* Form */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		margin-bottom: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	/* Replace-specific */
	.replace-info {
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		padding: 1rem;
		margin-bottom: 1.5rem;
	}

	.replace-info p {
		margin: 0.25rem 0;
		color: #cbd5e1;
	}

	.replace-info strong {
		color: #f1f5f9;
	}

	.text-muted {
		color: #64748b;
		font-size: 0.875rem;
	}

	/* Buttons */
	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
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

		.form-group input {
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
