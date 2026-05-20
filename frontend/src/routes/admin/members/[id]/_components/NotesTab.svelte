<script lang="ts">
	import { IconFileText, IconPlus, IconTrash } from '$lib/icons';
	import type { NoteItem } from './helpers';
	import { formatDateTime } from './helpers';

	interface Props {
		notes: NoteItem[];
		onAddNote: () => void;
	}

	let { notes, onAddNote }: Props = $props();
</script>

<div class="panel">
	<div class="panel-header">
		<h3>Internal Notes</h3>
		<button class="btn-primary small" onclick={onAddNote}>
			<IconPlus size={16} />
			Add Note
		</button>
	</div>
	{#if notes.length === 0}
		<div class="empty-state">
			<IconFileText size={48} stroke={1} />
			<h4>No Notes</h4>
			<p>Add internal notes about this member</p>
		</div>
	{:else}
		<div class="notes-list">
			{#each notes as note (note.id)}
				<div class="note-item">
					<div class="note-content">{note.content}</div>
					<div class="note-meta">
						<span class="note-author">{note.author}</span>
						<span class="note-date">{formatDateTime(note.created_at)}</span>
					</div>
					<button class="note-delete">
						<IconTrash size={14} />
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.panel {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.panel-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h4 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.notes-list {
		padding: 1rem;
	}

	.note-item {
		position: relative;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 10px;
		margin-bottom: 0.75rem;
	}

	.note-content {
		font-size: 0.875rem;
		color: #cbd5e1;
		line-height: 1.5;
		margin-bottom: 0.5rem;
	}

	.note-meta {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.note-delete {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0.25rem;
		opacity: 0;
		transition: all 0.2s;
	}

	.note-item:hover .note-delete {
		opacity: 1;
	}

	.note-delete:hover {
		color: #ef4444;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-primary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}
</style>
