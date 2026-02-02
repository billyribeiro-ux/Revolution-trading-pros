<!--
/**
 * List Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Ordered/unordered lists and checklists
 */
-->

<script lang="ts">
	import { IconPlus, IconX, IconCheck } from '$lib/icons';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();

	const isChecklist = $derived(props.block.type === 'checklist');
	const ListTag = $derived((props.block.content.listType === 'number' ? 'ol' : 'ul') as 'ol' | 'ul');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function addListItem(index: number): void {
		const items = [...(props.block.content.listItems || [])];
		items.splice(index + 1, 0, '');
		updateContent({ listItems: items });
	}

	function updateListItem(index: number, value: string): void {
		const items = [...(props.block.content.listItems || [])];
		items[index] = value;
		updateContent({ listItems: items });
	}

	function removeListItem(index: number): void {
		const items = [...(props.block.content.listItems || [])];
		if (items.length > 1) {
			items.splice(index, 1);
			updateContent({ listItems: items });
		}
	}

	function toggleCheckItem(itemId: string): void {
		const items =
			props.block.content.items?.map((item) =>
				item.id === itemId ? { ...item, checked: !item.checked } : item
			) || [];
		updateContent({ items });
	}

	function updateCheckItemContent(itemId: string, content: string): void {
		const items =
			props.block.content.items?.map((item) =>
				item.id === itemId ? { ...item, content } : item
			) || [];
		updateContent({ items });
	}

	function addCheckItem(): void {
		const items = [
			...(props.block.content.items || []),
			{
				id: `item_${Date.now()}`,
				content: '',
				checked: false
			}
		];
		updateContent({ items });
	}

	function removeCheckItem(itemId: string): void {
		const items = props.block.content.items?.filter((item) => item.id !== itemId) || [];
		if (items.length > 0) {
			updateContent({ items });
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}
</script>

{#if isChecklist}
	<div class="checklist-block" role="group" aria-label="Checklist">
		{#each props.block.content.items || [] as item (item.id)}
			<label class="check-item" class:checked={item.checked}>
				<input
					type="checkbox"
					checked={item.checked}
					onchange={() => toggleCheckItem(item.id)}
					disabled={!props.isEditing}
					aria-label={item.content || 'Checklist item'}
				/>
				<span class="check-icon" aria-hidden="true">
					{#if item.checked}
						<IconCheck size={14} />
					{/if}
				</span>
				<span
					contenteditable={props.isEditing}
					class="check-text editable-content"
					class:placeholder={!item.content}
					oninput={(e) =>
						updateCheckItemContent(item.id, (e.target as HTMLElement).textContent || '')}
					onpaste={handlePaste}
					data-placeholder="List item"
					role={props.isEditing ? 'textbox' : undefined}
					aria-label={props.isEditing ? 'Checklist item text' : undefined}
				>
					{item.content}
				</span>
				{#if props.isEditing && (props.block.content.items?.length || 0) > 1}
					<button
						type="button"
						class="remove-item"
						onclick={() => removeCheckItem(item.id)}
						aria-label="Remove checklist item"
					>
						<IconX size={14} aria-hidden="true" />
					</button>
				{/if}
			</label>
		{/each}
		{#if props.isEditing}
			<button
				type="button"
				class="add-item-btn"
				onclick={addCheckItem}
				aria-label="Add new checklist item"
			>
				<IconPlus size={14} aria-hidden="true" />
				Add item
			</button>
		{/if}
	</div>
{:else}
	<svelte:element this={ListTag} class="list-block">
		{#each props.block.content.listItems || [''] as item, index (index)}
			<li class="list-item">
				<span
					contenteditable={props.isEditing}
					class="list-text editable-content"
					class:placeholder={!item}
					oninput={(e) => updateListItem(index, (e.target as HTMLElement).textContent || '')}
					onkeydown={(e) => {
						if (e.key === 'Enter') {
							e.preventDefault();
							addListItem(index);
						} else if (
							e.key === 'Backspace' &&
							!item &&
							(props.block.content.listItems?.length || 0) > 1
						) {
							e.preventDefault();
							removeListItem(index);
						}
					}}
					onpaste={handlePaste}
					data-placeholder="List item"
					role={props.isEditing ? 'textbox' : undefined}
					aria-label={props.isEditing ? 'List item text' : undefined}
				>
					{item}
				</span>
				{#if props.isEditing && (props.block.content.listItems?.length || 0) > 1}
					<button
						type="button"
						class="remove-item"
						onclick={() => removeListItem(index)}
						aria-label="Remove list item"
					>
						<IconX size={14} aria-hidden="true" />
					</button>
				{/if}
			</li>
		{/each}
	</svelte:element>
	{#if props.isEditing}
		<button
			type="button"
			class="add-item-btn"
			onclick={() => addListItem((props.block.content.listItems?.length || 1) - 1)}
			aria-label="Add new list item"
		>
			<IconPlus size={14} aria-hidden="true" />
			Add item
		</button>
	{/if}
{/if}

<style>
	.list-block {
		margin: 0;
		padding-left: 1.5rem;
		color: #1f2937;
	}

	.list-item {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.list-text {
		flex: 1;
		line-height: 1.6;
		outline: none;
	}

	.checklist-block {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.check-item {
		display: flex;
		align-items: flex-start;
		gap: 0.625rem;
		cursor: pointer;
		transition: opacity 0.15s;
	}

	.check-item:hover {
		opacity: 0.8;
	}

	.check-item input[type='checkbox'] {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.check-icon {
		width: 20px;
		height: 20px;
		min-width: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px solid #d1d5db;
		border-radius: 4px;
		flex-shrink: 0;
		margin-top: 2px;
		transition: all 0.15s;
		background: white;
	}

	.check-item.checked .check-icon {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.check-text {
		flex: 1;
		line-height: 1.5;
		color: #1f2937;
		outline: none;
	}

	.check-item.checked .check-text {
		text-decoration: line-through;
		color: #9ca3af;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	.remove-item {
		display: flex;
		padding: 0.125rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #9ca3af;
		opacity: 0;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.list-item:hover .remove-item,
	.check-item:hover .remove-item {
		opacity: 1;
	}

	.remove-item:hover {
		color: #ef4444;
		background: #fee2e2;
	}

	.remove-item:focus-visible {
		opacity: 1;
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.add-item-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		margin-top: 0.5rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 6px;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		min-height: 36px;
	}

	.add-item-btn:hover {
		border-color: #3b82f6;
		color: #3b82f6;
		background: rgba(59, 130, 246, 0.05);
	}

	.add-item-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	:global(.dark) .list-block {
		color: #e5e7eb;
	}

	:global(.dark) .check-icon {
		border-color: #4b5563;
		background: #1e293b;
	}

	:global(.dark) .check-item.checked .check-icon {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	:global(.dark) .check-text {
		color: #e5e7eb;
	}

	:global(.dark) .check-item.checked .check-text {
		color: #6b7280;
	}

	:global(.dark) .remove-item:hover {
		color: #ef4444;
		background: #450a0a;
	}

	:global(.dark) .add-item-btn {
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .add-item-btn:hover {
		border-color: #60a5fa;
		color: #60a5fa;
		background: rgba(96, 165, 250, 0.1);
	}
</style>
