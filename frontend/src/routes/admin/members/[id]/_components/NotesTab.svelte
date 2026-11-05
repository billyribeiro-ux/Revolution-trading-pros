<script lang="ts">
	import { IconFileText, IconPlus, IconTrash } from '#lib/icons/index.js';
	import type { MemberProfile } from '#lib/api/member-profile.js';
	import { recordedInstant } from './profile-display.js';
	import HistoryControls from './HistoryControls.svelte';
	let {
		result,
		loading,
		error,
		busy = false,
		onPage,
		onPerPage,
		onRefresh,
		onAddNote,
		onDeleteNote
	}: {
		result: MemberProfile['notes'] | null;
		loading: boolean;
		error: string;
		busy?: boolean;
		onPage: (page: number) => void;
		onPerPage: (size: number) => void;
		onRefresh: () => void;
		onAddNote: () => void;
		onDeleteNote: (id: string) => void;
	} = $props();
</script>

<section class="panel" aria-label="Recorded internal notes">
	<div class="panel-header">
		<h3>Internal Notes</h3>
		<button onclick={onAddNote} disabled={busy}><IconPlus size={16} />Add Note</button>
	</div>
	<div class="content">
		<p class="hint">Saved internal notes and actual recorded authors.</p>
		{#if result && !loading && !error}{#if result.notes.length === 0}<div class="empty-state">
					<IconFileText size={48} />
					<h4>No saved notes on this page</h4>
				</div>{:else}<div class="notes-list">
					{#each result.notes as note (note.id)}<article class="note-item">
							<p class="note-content">{note.content}</p>
							<div class="note-meta">
								<span
									>{note.created_by_name ??
										(note.created_by
											? `Author #${note.created_by} (name not recorded)`
											: 'Author not recorded')}</span
								><span>{recordedInstant(note.created_at)}</span>
							</div>
							<button
								class="note-delete"
								aria-label={`Delete note ${note.id}`}
								disabled={busy}
								onclick={() => onDeleteNote(note.id)}><IconTrash size={14} /></button
							>
						</article>{/each}
				</div>{/if}{/if}
	</div>
	<HistoryControls
		label="notes"
		pagination={result?.pagination ?? null}
		{loading}
		{error}
		{busy}
		{onPage}
		{onPerPage}
		{onRefresh}
	/>
</section>

<style>
	.panel {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 16px;
		overflow: hidden;
	}
	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}
	h3 {
		font-size: 1rem;
		margin: 0;
		color: var(--text-primary, #f0f6fc);
	}
	button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.6rem 0.9rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 9px;
		cursor: pointer;
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	.content {
		padding: 1.25rem 1.5rem;
	}
	.hint,
	.empty-state {
		color: var(--text-secondary, #8b949e);
		line-height: 1.6;
	}
	.empty-state {
		padding: 2rem;
		text-align: center;
	}
	h4 {
		color: var(--text-primary, #f0f6fc);
	}
	.notes-list {
		display: grid;
		gap: 1rem;
	}
	.note-item {
		position: relative;
		padding: 1rem 3rem 1rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 10px;
	}
	.note-content {
		white-space: pre-wrap;
		overflow-wrap: anywhere;
		line-height: 1.6;
	}
	.note-meta {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		font-size: 0.8rem;
		color: var(--text-secondary, #8b949e);
	}
	.note-delete {
		position: absolute;
		right: 0.75rem;
		top: 0.75rem;
		padding: 0.4rem;
	}
</style>
