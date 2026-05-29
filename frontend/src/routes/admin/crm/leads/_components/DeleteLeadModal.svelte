<script lang="ts">
	import { IconTrash, IconX } from '$lib/icons';

	interface DeletingLead {
		full_name: string;
	}

	interface Props {
		lead: DeletingLead;
		onClose: () => void;
		onConfirm: () => void;
	}

	let { lead, onClose, onConfirm }: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e) => e.key === 'Escape' && onClose()}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<div class="modal modal-small" role="document">
		<div class="modal-header">
			<h3>
				<IconTrash size={20} />
				Delete Lead
			</h3>
			<button class="modal-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		</div>
		<div class="modal-body">
			<p class="confirm-text">
				Are you sure you want to delete <strong>{lead.full_name}</strong>? This action cannot be
				undone.
			</p>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}> Cancel </button>
			<button class="btn-danger" onclick={onConfirm}>
				<IconTrash size={18} />
				Delete Lead
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

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}
</style>
