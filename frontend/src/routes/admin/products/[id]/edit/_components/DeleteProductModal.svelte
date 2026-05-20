<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): delete-confirmation modal.
	 * Self-contained: parent only toggles `open` and passes `onConfirm`/`onCancel`.
	 */
	import { IconTrash } from '$lib/icons';

	interface Props {
		open: boolean;
		productName: string;
		deleting: boolean;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let { open, productName, deleting, onConfirm, onCancel }: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={onCancel}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onCancel()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && onCancel()}
			role="document"
			tabindex="-1"
		>
			<h3>Delete Product?</h3>
			<p>
				Are you sure you want to delete <strong>{productName}</strong>? This action cannot be
				undone.
			</p>
			<div class="modal-actions">
				<button class="btn-secondary" onclick={onCancel}>Cancel</button>
				<button class="btn-danger" onclick={onConfirm} disabled={deleting}>
					{#if deleting}
						<div class="btn-spinner danger"></div>
						Deleting...
					{:else}
						<IconTrash size={18} />
						Delete Product
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 2rem;
	}

	.modal {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 16px;
		padding: 2rem;
		max-width: 480px;
		width: 100%;
	}

	.modal h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.modal p {
		color: #94a3b8;
		font-size: 0.9375rem;
		line-height: 1.5;
		margin-bottom: 1.5rem;
	}

	.modal strong {
		color: #f1f5f9;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	.btn-secondary,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		color: #cbd5e1;
	}

	.btn-secondary:hover {
		background: rgba(30, 41, 59, 0.8);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.btn-danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-danger:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.btn-spinner.danger {
		border-color: rgba(248, 113, 113, 0.3);
		border-top-color: #f87171;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
