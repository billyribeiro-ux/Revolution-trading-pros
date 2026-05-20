<script lang="ts">
	import IconNotes from '@tabler/icons-svelte-runes/icons/notes';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import type { Note } from './types';
	import { formatDateTime } from './helpers';

	interface Props {
		notes: Note[];
		onAddNote: () => void;
	}

	let { notes, onAddNote }: Props = $props();
</script>

<div class="notes-section">
	<div class="notes-header">
		<button class="btn-primary" onclick={onAddNote}>
			<IconPlus size={18} />
			Add Note
		</button>
	</div>
	{#if notes.length === 0}
		<div class="empty-state">
			<IconNotes size={48} />
			<h3>No notes yet</h3>
			<p>Add notes to keep track of important information about this contact</p>
		</div>
	{:else}
		<div class="notes-list">
			{#each notes as note (note.id)}
				<div class="note-item">
					<div class="note-header">
						<span class="note-author">{note.created_by?.name || 'Unknown'}</span>
						<span class="note-date">{formatDateTime(note.created_at)}</span>
					</div>
					<p class="note-content">{note.content}</p>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.notes-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.notes-header {
		margin-bottom: 20px;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
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
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.notes-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.note-item {
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
	}

	.note-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 8px;
	}

	.note-author {
		font-weight: 600;
		color: #e2e8f0;
		font-size: 0.875rem;
	}

	.note-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.note-content {
		margin: 0;
		color: #94a3b8;
		font-size: 0.9rem;
		line-height: 1.6;
		white-space: pre-wrap;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		color: white;
		font-size: 1.1rem;
	}

	.empty-state p {
		margin: 0;
		color: #64748b;
		font-size: 0.9rem;
	}
</style>
