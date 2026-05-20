<script lang="ts">
	import { IconArrowRight, IconX } from '$lib/icons';

	interface ConvertingLead {
		full_name: string;
	}

	interface Props {
		lead: ConvertingLead;
		onClose: () => void;
		onConfirm: () => void;
	}

	let { lead, onClose, onConfirm }: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={onClose}
	onkeydown={(e) => e.key === 'Escape' && onClose()}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal modal-small"
		onclick={(e) => e.stopPropagation()}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="document"
	>
		<div class="modal-header">
			<h3>
				<IconArrowRight size={20} />
				Convert to Contact
			</h3>
			<button class="modal-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		</div>
		<div class="modal-body">
			<p class="confirm-text">
				Convert <strong>{lead.full_name}</strong> to a contact? The lead will be moved to your contacts
				list with all associated data.
			</p>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}> Cancel </button>
			<button class="btn-primary" onclick={onConfirm}>
				<IconArrowRight size={18} />
				Convert Lead
			</button>
		</div>
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		width: 100%;
		max-width: 560px;
		max-height: 90vh;
		overflow-y: auto;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
	}

	.modal-small {
		max-width: 420px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0;
		font-size: 1.1rem;
		font-weight: 600;
		color: white;
	}

	.modal-header h3 :global(svg) {
		color: var(--primary-500);
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: color 0.2s;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.confirm-text {
		margin: 0;
		font-size: 0.95rem;
		color: #94a3b8;
		line-height: 1.6;
	}

	.confirm-text strong {
		color: white;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
