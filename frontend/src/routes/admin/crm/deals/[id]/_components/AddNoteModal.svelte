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

	let { newNoteContent, processingAction, onUpdateContent, onConfirm, onCancel }: Props = $props();
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
					rows="6"></textarea>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={onCancel} disabled={processingAction}> Cancel </button>
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 20px;
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.modal-header :global(svg:first-child) {
		color: var(--warning-emphasis, #d29922);
	}

	.modal-header h3 {
		flex: 1;
		margin: 0;
		font-size: 1.1rem;
		color: #ffffff;
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
	}

	.modal-close:hover {
		color: #ffffff;
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
		color: var(--text-primary, #f0f6fc);
	}

	.form-group textarea {
		padding: 12px;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500, #e6b800);
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: var(--bg-elevated, #161b22);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
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
