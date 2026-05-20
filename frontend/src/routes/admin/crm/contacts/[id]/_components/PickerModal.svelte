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
		onClose: () => void;
		onSelect: (id: string) => void;
	}

	let { open, kind, items, onClose, onSelect }: Props = $props();

	const titleId = $derived(`${kind === 'tag' ? 'add-tag' : 'add-list'}-title`);
	const title = $derived(kind === 'tag' ? 'Add Tag' : 'Add to List');
	const emptyText = $derived(kind === 'tag' ? 'No tags available' : 'No lists available');
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={(e: MouseEvent) => e.target === e.currentTarget && onClose()}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby={titleId}
		tabindex="-1"
	>
		<div class="modal" role="document">
			<div class="modal-header">
				<h3 id={titleId}>{title}</h3>
				<button class="modal-close" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if items.length === 0}
					<p class="empty-text">{emptyText}</p>
				{:else}
					<div class="picker-options">
						{#each items as item (item.id)}
							<button class="picker-option" onclick={() => onSelect(item.id)}>
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
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #334155;
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
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.empty-text {
		color: #64748b;
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
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
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
