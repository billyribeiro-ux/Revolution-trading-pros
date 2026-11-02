<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';

	interface Props {
		open: boolean;
		content: string;
		onClose: () => void;
		onSave: () => void;
	}

	let { open, content = $bindable(), onClose, onSave }: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={(e: MouseEvent) => e.target === e.currentTarget && onClose()}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="add-note-title"
		tabindex="-1"
	>
		<div class="modal" role="document">
			<div class="modal-header">
				<h3 id="add-note-title">Add Note</h3>
				<button class="modal-close" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<textarea id="addnotemodal-content" name="addnotemodal_content"
					class="note-input"
					placeholder="Write your note here..."
					bind:value={content}
					rows="4"></textarea>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose}>Cancel</button>
				<button class="btn-primary" onclick={onSave} disabled={!content.trim()}>
					<IconCheck size={18} />
					Save Note
				</button>
			</div>
		</div>
	</div>
{/if}

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
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.modal-header h3 {
		margin: 0;
		font-size: 1.1rem;
		color: white;
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
		border-top: 1px solid var(--border-default, #30363d);
	}

	.note-input {
		width: 100%;
		padding: 12px;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
		min-height: 100px;
	}

	.note-input:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: var(--primary-500);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		color: var(--text-primary, #f0f6fc);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: var(--bg-elevated, #161b22);
		border-color: var(--border-default, #30363d);
	}
</style>
