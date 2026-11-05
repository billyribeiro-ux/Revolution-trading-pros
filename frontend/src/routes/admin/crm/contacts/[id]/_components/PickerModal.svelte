<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import IconList from '@tabler/icons-svelte-runes/icons/list';

	/**
	 * Generic single-select picker modal used for both
	 * "Add Tag" and "Add to List" — the two original modals
	 * had identical structure (overlay + header + a list of
	 * pickable rows). Discriminated by the `kind` prop so the
	 * caller still gets a tight prop surface.
	 */
	interface Item {
		id: string;
		name: string;
	}

	interface Props {
		open: boolean;
		kind: 'tag' | 'list';
		items: Item[];
		busy: boolean;
		loading: boolean;
		error: string;
		search: string;
		page: number;
		pages: number;
		total: number;
		onSearch: () => void;
		onPage: (page: number) => void;
		onClose: () => void;
		onSelect: (id: string) => void;
	}

	let {
		open,
		kind,
		items,
		busy,
		loading,
		error,
		search = $bindable(),
		page,
		pages,
		total,
		onSearch,
		onPage,
		onClose,
		onSelect
	}: Props = $props();

	const titleId = $derived(`${kind === 'tag' ? 'add-tag' : 'add-list'}-title`);
	const title = $derived(kind === 'tag' ? 'Add Tag' : 'Add to List');
	const emptyText = $derived(kind === 'tag' ? 'No tags available' : 'No lists available');
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={(e: MouseEvent) => !busy && e.target === e.currentTarget && onClose()}
		onkeydown={(e: KeyboardEvent) => !busy && e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby={titleId}
		tabindex="-1"
	>
		<div class="modal" role="document">
			<div class="modal-header">
				<h3 id={titleId}>{title}</h3>
				<button class="modal-close" onclick={onClose} disabled={busy} aria-label="Close picker">
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<form
					onsubmit={(event) => {
						event.preventDefault();
						onSearch();
					}}
				>
					<label for={`${kind}-search`}>Search {kind}s</label><input
						id={`${kind}-search`}
						bind:value={search}
						disabled={busy || loading}
					/><button disabled={busy || loading}>Search</button>
				</form>
				{#if loading}<p role="status">Loading catalog…</p>{:else if error}<p role="alert">
						{error}
					</p>
					<button onclick={onSearch}>Retry catalog</button>{:else if items.length === 0}
					<p class="empty-text">{emptyText}</p>
				{:else}
					<div class="picker-options">
						{#each items as item (item.id)}
							<button class="picker-option" disabled={busy} onclick={() => onSelect(item.id)}>
								{#if kind === 'tag'}
									<IconTag size={16} />
								{:else}
									<IconList size={16} />
								{/if}
								{item.name}
							</button>
						{/each}
					</div>
				{/if}
				<nav aria-label={`${kind} catalog pages`}>
					<button disabled={busy || loading || page <= 1} onclick={() => onPage(page - 1)}
						>Previous</button
					><span>Page {page} of {pages}; {total} records</span><button
						disabled={busy || loading || page >= pages}
						onclick={() => onPage(page + 1)}>Next</button
					>
				</nav>
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

	.empty-text {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0;
	}

	.picker-options {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.picker-option {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 16px;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.picker-option:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}
</style>
