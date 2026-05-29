<script lang="ts">
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconNotes from '@tabler/icons-svelte-runes/icons/notes';

	interface Props {
		newNoteContent: string;
		processingAction: boolean;
		onUpdateContent: (value: string) => void;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let {
		newNoteContent,
		processingAction,
		onUpdateContent,
		onConfirm,
		onCancel
	}: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e) => {
		if (e.target === e.currentTarget) onCancel();
	}}
	onkeydown={(e) => e.key === 'Escape' && onCancel()}
	role="dialog"
	aria-modal="true"
	tabindex="-1"
>
	<div class="modal" role="document">
		<div class="modal-header">
			<IconNotes size={24} />
			<h3>Add Note</h3>
			<button class="modal-close" onclick={onCancel}>
				<IconX size={20} />
			</button>
		</div>
		<div class="modal-body">
			<div class="form-group">
				<label for="note-content">Note Content</label>
				<textarea
					id="note-content"
					value={newNoteContent}
					oninput={(e) => onUpdateContent(e.currentTarget.value)}
					placeholder="Write your note here..."
					rows="6"
				></textarea>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onCancel} disabled={processingAction}>
				Cancel
			</button>
			<button
				class="btn-primary"
				onclick={onConfirm}
				disabled={processingAction || !newNoteContent.trim()}
			>
				{#if processingAction}
					<div class="btn-spinner"></div>
				{:else}
					<IconCheck size={18} />
				{/if}
				Save Note
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

	.modal-header :global(svg:first-child) {
		color: #f97316;
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

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
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
