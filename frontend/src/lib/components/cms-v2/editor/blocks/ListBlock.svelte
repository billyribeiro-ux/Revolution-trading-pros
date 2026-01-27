<!--
	List Block - Bullet or numbered list
	═══════════════════════════════════════════════════════════════════════════════

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconTrash from '@tabler/icons-svelte/icons/trash';

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	let data = $derived(block.data as { type?: string; items?: string[] });
	let listType = $derived(data.type ?? 'bullet');
	let items = $derived(data.items ?? ['']);

	function handleTypeChange(newType: string) {
		onUpdate?.({ ...data, type: newType });
	}

	function handleItemChange(index: number, value: string) {
		const newItems = [...items];
		newItems[index] = value;
		onUpdate?.({ ...data, items: newItems });
	}

	function addItem() {
		onUpdate?.({ ...data, items: [...items, ''] });
	}

	function removeItem(index: number) {
		if (items.length <= 1) return;
		const newItems = items.filter((_, i) => i !== index);
		onUpdate?.({ ...data, items: newItems });
	}

	function handleKeyDown(e: KeyboardEvent, index: number) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			addItem();
		} else if (e.key === 'Backspace' && items[index] === '' && items.length > 1) {
			e.preventDefault();
			removeItem(index);
		}
	}
</script>

<div class="list-block">
	{#if !readonly}
		<div class="type-selector">
			<button
				type="button"
				class="type-btn"
				class:active={listType === 'bullet'}
				onclick={() => handleTypeChange('bullet')}
			>
				• Bullet
			</button>
			<button
				type="button"
				class="type-btn"
				class:active={listType === 'numbered'}
				onclick={() => handleTypeChange('numbered')}
			>
				1. Numbered
			</button>
		</div>
	{/if}

	{#if listType === 'bullet'}
		<ul class="list list-bullet">
			{#each items as item, index}
				<li class="list-item">
					{#if readonly}
						{item}
					{:else}
						<input
							type="text"
							class="item-input"
							value={item}
							oninput={(e) => handleItemChange(index, (e.target as HTMLInputElement).value)}
							onkeydown={(e) => handleKeyDown(e, index)}
							placeholder="List item..."
						/>
						{#if items.length > 1}
							<button
								type="button"
								class="remove-btn"
								onclick={() => removeItem(index)}
								aria-label="Remove item"
							>
								<IconTrash size={12} />
							</button>
						{/if}
					{/if}
				</li>
			{/each}
		</ul>
	{:else}
		<ol class="list list-numbered">
			{#each items as item, index}
				<li class="list-item">
					{#if readonly}
						{item}
					{:else}
						<input
							type="text"
							class="item-input"
							value={item}
							oninput={(e) => handleItemChange(index, (e.target as HTMLInputElement).value)}
							onkeydown={(e) => handleKeyDown(e, index)}
							placeholder="List item..."
						/>
						{#if items.length > 1}
							<button
								type="button"
								class="remove-btn"
								onclick={() => removeItem(index)}
								aria-label="Remove item"
							>
								<IconTrash size={12} />
							</button>
						{/if}
					{/if}
				</li>
			{/each}
		</ol>
	{/if}

	{#if !readonly}
		<button type="button" class="add-btn" onclick={addItem}>
			<IconPlus size={14} />
			Add item
		</button>
	{/if}
</div>

<style>
	.list-block {
		width: 100%;
	}

	.type-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.75rem;
	}

	.type-btn {
		padding: 0.25rem 0.625rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid transparent;
		border-radius: 0.25rem;
		color: #64748b;
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.type-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #94a3b8;
	}

	.type-btn.active {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.3);
		color: #818cf8;
	}

	.list {
		margin: 0;
		padding-left: 1.5rem;
	}

	.list-bullet {
		list-style-type: disc;
	}

	.list-numbered {
		list-style-type: decimal;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.375rem;
		color: #f1f5f9;
	}

	.item-input {
		flex: 1;
		padding: 0.375rem 0.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 1px solid rgba(51, 65, 85, 0.3);
		border-radius: 0.25rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.item-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.remove-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		color: #64748b;
		cursor: pointer;
		opacity: 0;
		transition: all 0.15s;
	}

	.list-item:hover .remove-btn {
		opacity: 1;
	}

	.remove-btn:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.add-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		margin-top: 0.5rem;
		margin-left: 1.5rem;
		padding: 0.375rem 0.625rem;
		background: transparent;
		border: 1px dashed rgba(99, 102, 241, 0.3);
		border-radius: 0.25rem;
		color: #64748b;
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-btn:hover {
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(99, 102, 241, 0.05);
		color: #818cf8;
	}
</style>
