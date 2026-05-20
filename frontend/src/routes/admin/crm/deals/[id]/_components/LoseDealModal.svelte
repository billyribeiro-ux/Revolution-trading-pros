<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { Deal } from '$lib/crm/types';
	import { formatCurrency } from './helpers';

	interface Props {
		deal: Deal;
		lostReason: string;
		processingAction: boolean;
		onUpdateReason: (value: string) => void;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let { deal, lostReason, processingAction, onUpdateReason, onConfirm, onCancel }: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={onCancel}
	onkeydown={(e) => e.key === 'Escape' && onCancel()}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal"
		onclick={(e) => e.stopPropagation()}
		onkeydown={(e) => e.stopPropagation()}
		role="document"
	>
		<div class="modal-header danger">
			<IconX size={24} />
			<h3>Mark Deal as Lost</h3>
			<button class="modal-close" onclick={onCancel}>
				<IconX size={20} />
			</button>
		</div>
		<div class="modal-body">
			<div class="deal-summary">
				<p class="summary-name">{deal.name}</p>
				<p class="summary-value">{formatCurrency(deal.amount)}</p>
			</div>
			<div class="form-group">
				<label for="lost-reason">Lost Reason <span class="required">*</span></label>
				<textarea
					id="lost-reason"
					value={lostReason}
					oninput={(e) => onUpdateReason(e.currentTarget.value)}
					placeholder="Why was this deal lost? (required)"
					rows="4"
					required
				></textarea>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onCancel} disabled={processingAction}>
				Cancel
			</button>
			<button
				class="btn-danger"
				onclick={onConfirm}
				disabled={processingAction || !lostReason.trim()}
			>
				{#if processingAction}
					<div class="btn-spinner"></div>
				{:else}
					<IconX size={18} />
				{/if}
				Mark as Lost
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
		max-width: 480px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header.danger {
		background: rgba(239, 68, 68, 0.1);
	}

	.modal-header.danger :global(svg) {
		color: #f87171;
	}

	.modal-header h3 {
		flex: 1;
		margin: 0;
		font-size: 1.1rem;
		color: white;
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.deal-summary {
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
		margin-bottom: 20px;
	}

	.summary-name {
		margin: 0 0 4px;
		font-weight: 600;
		color: white;
	}

	.summary-value {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #4ade80;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group label {
		font-size: 0.85rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.form-group .required {
		color: #f87171;
	}

	.form-group textarea {
		padding: 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #e2e8f0;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #1e293b;
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

	.btn-danger:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}

	.btn-danger:disabled {
		opacity: 0.5;
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

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
