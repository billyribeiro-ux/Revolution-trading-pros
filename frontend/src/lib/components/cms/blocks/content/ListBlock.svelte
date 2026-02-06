<!--
/**
 * List Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Ordered/unordered lists and checklists with dynamic item management,
 * keyboard navigation, and progress tracking
 */
-->

<script lang="ts">
	import { IconCheck, IconPlus, IconX } from '$lib/icons';
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

	let props: Props = $props();

	// Derived state
	const isChecklist = $derived(props.block.type === 'checklist');
	const listType = $derived(props.block.content.listType || 'bullet');
	const ListTag = $derived((listType === 'number' ? 'ol' : 'ul') as 'ol' | 'ul');

	// Checklist progress calculation
	const checklistItems = $derived(props.block.content.items || []);
	const completedCount = $derived(checklistItems.filter((item) => item.checked).length);
	const totalCount = $derived(checklistItems.length);
	const progressPercent = $derived(
		totalCount > 0 ? Math.round((completedCount / totalCount) * 100) : 0
	);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	// List type toggle
	function toggleListType(): void {
		updateContent({ listType: listType === 'bullet' ? 'number' : 'bullet' });
	}

	// Regular list item management
	function addListItem(index: number): void {
		const items = [...(props.block.content.listItems || [])];
		items.splice(index + 1, 0, '');
		updateContent({ listItems: items });

		// Focus the new item after render
		requestAnimationFrame(() => {
			const newItem = document.querySelector(
				`[data-list-item-index="${index + 1}"]`
			) as HTMLElement;
			newItem?.focus();
		});
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

			// Focus previous item after removal
			requestAnimationFrame(() => {
				const prevIndex = Math.max(0, index - 1);
				const prevItem = document.querySelector(
					`[data-list-item-index="${prevIndex}"]`
				) as HTMLElement;
				prevItem?.focus();
			});
		}
	}

	function handleListKeyDown(e: KeyboardEvent, index: number): void {
		const items = props.block.content.listItems || [];
		const currentItem = items[index] || '';

		if (e.key === 'Enter') {
			e.preventDefault();
			addListItem(index);
		} else if (e.key === 'Backspace' && !currentItem && items.length > 1) {
			e.preventDefault();
			removeListItem(index);
		} else if (e.key === 'ArrowUp' && index > 0) {
			e.preventDefault();
			const prevItem = document.querySelector(
				`[data-list-item-index="${index - 1}"]`
			) as HTMLElement;
			prevItem?.focus();
		} else if (e.key === 'ArrowDown' && index < items.length - 1) {
			e.preventDefault();
			const nextItem = document.querySelector(
				`[data-list-item-index="${index + 1}"]`
			) as HTMLElement;
			nextItem?.focus();
		}
	}

	// Checklist item management
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

	function addCheckItem(afterIndex?: number): void {
		const newItem = {
			id: `item_${Date.now()}_${Math.random().toString(36).slice(2, 9)}`,
			content: '',
			checked: false
		};

		const items = [...(props.block.content.items || [])];
		if (afterIndex !== undefined) {
			items.splice(afterIndex + 1, 0, newItem);
		} else {
			items.push(newItem);
		}
		updateContent({ items });

		// Focus the new item
		requestAnimationFrame(() => {
			const newIndex = afterIndex !== undefined ? afterIndex + 1 : items.length - 1;
			const newElement = document.querySelector(
				`[data-check-item-index="${newIndex}"]`
			) as HTMLElement;
			newElement?.focus();
		});
	}

	function removeCheckItem(itemId: string, index: number): void {
		const items = props.block.content.items?.filter((item) => item.id !== itemId) || [];
		if (items.length > 0) {
			updateContent({ items });

			// Focus previous item
			requestAnimationFrame(() => {
				const prevIndex = Math.max(0, index - 1);
				const prevElement = document.querySelector(
					`[data-check-item-index="${prevIndex}"]`
				) as HTMLElement;
				prevElement?.focus();
			});
		}
	}

	function handleCheckKeyDown(e: KeyboardEvent, itemId: string, index: number): void {
		const items = props.block.content.items || [];
		const currentItem = items.find((item) => item.id === itemId);

		if (e.key === 'Enter') {
			e.preventDefault();
			addCheckItem(index);
		} else if (e.key === 'Backspace' && !currentItem?.content && items.length > 1) {
			e.preventDefault();
			removeCheckItem(itemId, index);
		} else if (e.key === 'ArrowUp' && index > 0) {
			e.preventDefault();
			const prevElement = document.querySelector(
				`[data-check-item-index="${index - 1}"]`
			) as HTMLElement;
			prevElement?.focus();
		} else if (e.key === 'ArrowDown' && index < items.length - 1) {
			e.preventDefault();
			const nextElement = document.querySelector(
				`[data-check-item-index="${index + 1}"]`
			) as HTMLElement;
			nextElement?.focus();
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}
</script>

{#if isChecklist}
	<!-- Checklist Mode -->
	<div class="list-block list-block--checklist" role="group" aria-label="Checklist">
		<!-- Progress Indicator -->
		{#if totalCount > 0}
			<div class="list-block__progress">
				<div class="list-block__progress-bar">
					<div
						class="list-block__progress-fill"
						style="width: {progressPercent}%"
						role="progressbar"
						aria-valuenow={progressPercent}
						aria-valuemin={0}
						aria-valuemax={100}
					></div>
				</div>
				<span class="list-block__progress-text">
					{completedCount}/{totalCount} completed ({progressPercent}%)
				</span>
			</div>
		{/if}

		<!-- Checklist Items -->
		<div class="list-block__items">
			{#each checklistItems as item, index (item.id)}
				<label class="list-block__check-item" class:list-block__check-item--checked={item.checked}>
					<input
						type="checkbox"
						checked={item.checked}
						onchange={() => toggleCheckItem(item.id)}
						disabled={!props.isEditing}
						class="list-block__checkbox-input"
						aria-label={item.content || 'Checklist item'}
					/>
					<span class="list-block__checkbox" aria-hidden="true">
						{#if item.checked}
							<IconCheck size={14} />
						{/if}
					</span>
					<span
						contenteditable={props.isEditing}
						class="list-block__check-text"
						class:list-block__check-text--placeholder={!item.content}
						oninput={(e) =>
							updateCheckItemContent(item.id, (e.target as HTMLElement).textContent || '')}
						onkeydown={(e) => handleCheckKeyDown(e, item.id, index)}
						onpaste={handlePaste}
						data-placeholder="List item..."
						data-check-item-index={index}
						role={props.isEditing ? 'textbox' : undefined}
						aria-label={props.isEditing ? 'Checklist item text' : undefined}
						spellcheck="true"
					>
						{item.content}
					</span>
					{#if props.isEditing && checklistItems.length > 1}
						<button
							type="button"
							class="list-block__remove-btn"
							onclick={() => removeCheckItem(item.id, index)}
							aria-label="Remove checklist item"
						>
							<IconX size={14} aria-hidden="true" />
						</button>
					{/if}
				</label>
			{/each}
		</div>

		<!-- Add Item Button -->
		{#if props.isEditing}
			<button
				type="button"
				class="list-block__add-btn"
				onclick={() => addCheckItem()}
				aria-label="Add new checklist item"
			>
				<IconPlus size={14} aria-hidden="true" />
				<span>Add item</span>
			</button>
		{/if}
	</div>
{:else}
	<!-- Regular List Mode -->
	<div
		class="list-block"
		role="group"
		aria-label={listType === 'number' ? 'Numbered list' : 'Bulleted list'}
	>
		<!-- List Type Toggle -->
		{#if props.isEditing}
			<div class="list-block__toolbar">
				<button
					type="button"
					class="list-block__toggle-btn"
					class:list-block__toggle-btn--active={listType === 'bullet'}
					onclick={toggleListType}
					aria-pressed={listType === 'bullet'}
					aria-label="Toggle bullet list"
				>
					<svg
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="9" y1="6" x2="20" y2="6"></line>
						<line x1="9" y1="12" x2="20" y2="12"></line>
						<line x1="9" y1="18" x2="20" y2="18"></line>
						<circle cx="4" cy="6" r="2" fill="currentColor"></circle>
						<circle cx="4" cy="12" r="2" fill="currentColor"></circle>
						<circle cx="4" cy="18" r="2" fill="currentColor"></circle>
					</svg>
					<span>Bullets</span>
				</button>
				<button
					type="button"
					class="list-block__toggle-btn"
					class:list-block__toggle-btn--active={listType === 'number'}
					onclick={toggleListType}
					aria-pressed={listType === 'number'}
					aria-label="Toggle numbered list"
				>
					<svg
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<line x1="10" y1="6" x2="21" y2="6"></line>
						<line x1="10" y1="12" x2="21" y2="12"></line>
						<line x1="10" y1="18" x2="21" y2="18"></line>
						<text x="4" y="7" font-size="8" fill="currentColor" stroke="none">1</text>
						<text x="4" y="13" font-size="8" fill="currentColor" stroke="none">2</text>
						<text x="4" y="19" font-size="8" fill="currentColor" stroke="none">3</text>
					</svg>
					<span>Numbers</span>
				</button>
			</div>
		{/if}

		<!-- List Items -->
		<svelte:element
			this={ListTag}
			class="list-block__list"
			class:list-block__list--numbered={listType === 'number'}
		>
			{#each props.block.content.listItems || [''] as item, index (index)}
				<li class="list-block__item">
					<span
						contenteditable={props.isEditing}
						class="list-block__item-text"
						class:list-block__item-text--placeholder={!item}
						oninput={(e) => updateListItem(index, (e.target as HTMLElement).textContent || '')}
						onkeydown={(e) => handleListKeyDown(e, index)}
						onpaste={handlePaste}
						data-placeholder="List item..."
						data-list-item-index={index}
						role={props.isEditing ? 'textbox' : undefined}
						aria-label={props.isEditing ? 'List item text' : undefined}
						spellcheck="true"
					>
						{item}
					</span>
					{#if props.isEditing && (props.block.content.listItems?.length || 0) > 1}
						<button
							type="button"
							class="list-block__remove-btn"
							onclick={() => removeListItem(index)}
							aria-label="Remove list item"
						>
							<IconX size={14} aria-hidden="true" />
						</button>
					{/if}
				</li>
			{/each}
		</svelte:element>

		<!-- Add Item Button -->
		{#if props.isEditing}
			<button
				type="button"
				class="list-block__add-btn"
				onclick={() => addListItem((props.block.content.listItems?.length || 1) - 1)}
				aria-label="Add new list item"
			>
				<IconPlus size={14} aria-hidden="true" />
				<span>Add item</span>
			</button>
			<p class="list-block__hint">Press Enter to add a new item. Backspace on empty to remove.</p>
		{/if}
	</div>
{/if}

<style>
	/* =========================================================================
	 * List Block - BEM Structure
	 * ========================================================================= */

	.list-block {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	/* Toolbar
	 * ------------------------------------------------------------------------- */
	.list-block__toolbar {
		display: flex;
		gap: 0.5rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #e5e7eb;
		margin-bottom: 0.5rem;
	}

	.list-block__toggle-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		color: #6b7280;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.list-block__toggle-btn:hover {
		background: #f3f4f6;
		border-color: #d1d5db;
		color: #374151;
	}

	.list-block__toggle-btn--active {
		background: #eff6ff;
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.list-block__toggle-btn--active:hover {
		background: #dbeafe;
	}

	.list-block__toggle-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Progress Indicator (Checklist)
	 * ------------------------------------------------------------------------- */
	.list-block__progress {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
		margin-bottom: 0.5rem;
	}

	.list-block__progress-bar {
		flex: 1;
		height: 6px;
		background: #e5e7eb;
		border-radius: 3px;
		overflow: hidden;
	}

	.list-block__progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #10b981);
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.list-block__progress-text {
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
		white-space: nowrap;
	}

	/* List Container
	 * ------------------------------------------------------------------------- */
	.list-block__list {
		margin: 0;
		padding-left: 1.5rem;
		color: #1f2937;
	}

	.list-block__list--numbered {
		list-style-type: decimal;
	}

	.list-block__items {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	/* Regular List Item
	 * ------------------------------------------------------------------------- */
	.list-block__item {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		margin-bottom: 0.375rem;
		position: relative;
	}

	.list-block__item-text {
		flex: 1;
		line-height: 1.6;
		color: #1f2937;
		outline: none;
		padding: 0.125rem 0;
		border-radius: 4px;
		transition: background-color 0.15s ease;
	}

	.list-block__item-text:focus {
		background: rgba(59, 130, 246, 0.05);
	}

	.list-block__item-text--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	/* Checklist Item
	 * ------------------------------------------------------------------------- */
	.list-block__check-item {
		display: flex;
		align-items: flex-start;
		gap: 0.625rem;
		padding: 0.375rem 0.5rem;
		border-radius: 6px;
		cursor: pointer;
		transition: background-color 0.15s ease;
	}

	.list-block__check-item:hover {
		background: #f9fafb;
	}

	.list-block__checkbox-input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.list-block__checkbox {
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
		transition: all 0.15s ease;
		background: white;
		color: white;
	}

	.list-block__check-item--checked .list-block__checkbox {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	.list-block__check-text {
		flex: 1;
		line-height: 1.5;
		color: #1f2937;
		outline: none;
		padding: 0.125rem 0;
		border-radius: 4px;
		transition: all 0.15s ease;
	}

	.list-block__check-text:focus {
		background: rgba(59, 130, 246, 0.05);
	}

	.list-block__check-item--checked .list-block__check-text {
		text-decoration: line-through;
		color: #9ca3af;
	}

	.list-block__check-text--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	/* Remove Button
	 * ------------------------------------------------------------------------- */
	.list-block__remove-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #9ca3af;
		opacity: 0;
		transition: all 0.15s ease;
		flex-shrink: 0;
	}

	.list-block__item:hover .list-block__remove-btn,
	.list-block__check-item:hover .list-block__remove-btn {
		opacity: 1;
	}

	.list-block__remove-btn:hover {
		color: #ef4444;
		background: #fee2e2;
	}

	.list-block__remove-btn:focus-visible {
		opacity: 1;
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Add Button
	 * ------------------------------------------------------------------------- */
	.list-block__add-btn {
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
		transition: all 0.15s ease;
		width: fit-content;
	}

	.list-block__add-btn:hover {
		border-color: #3b82f6;
		color: #3b82f6;
		background: rgba(59, 130, 246, 0.05);
	}

	.list-block__add-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Hint Text
	 * ------------------------------------------------------------------------- */
	.list-block__hint {
		margin: 0.5rem 0 0 0;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* =========================================================================
	 * Dark Mode
	 * ========================================================================= */
	:global(.dark) .list-block__toolbar {
		border-color: #374151;
	}

	:global(.dark) .list-block__toggle-btn {
		background: #1f2937;
		border-color: #374151;
		color: #9ca3af;
	}

	:global(.dark) .list-block__toggle-btn:hover {
		background: #374151;
		border-color: #4b5563;
		color: #e5e7eb;
	}

	:global(.dark) .list-block__toggle-btn--active {
		background: rgba(59, 130, 246, 0.1);
		border-color: #3b82f6;
		color: #60a5fa;
	}

	:global(.dark) .list-block__toggle-btn--active:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	:global(.dark) .list-block__progress {
		background: #1f2937;
	}

	:global(.dark) .list-block__progress-bar {
		background: #374151;
	}

	:global(.dark) .list-block__progress-text {
		color: #9ca3af;
	}

	:global(.dark) .list-block__list {
		color: #e5e7eb;
	}

	:global(.dark) .list-block__item-text {
		color: #e5e7eb;
	}

	:global(.dark) .list-block__item-text:focus {
		background: rgba(96, 165, 250, 0.1);
	}

	:global(.dark) .list-block__item-text--placeholder:empty::before,
	:global(.dark) .list-block__check-text--placeholder:empty::before {
		color: #6b7280;
	}

	:global(.dark) .list-block__check-item:hover {
		background: #1f2937;
	}

	:global(.dark) .list-block__checkbox {
		border-color: #4b5563;
		background: #1e293b;
	}

	:global(.dark) .list-block__check-item--checked .list-block__checkbox {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	:global(.dark) .list-block__check-text {
		color: #e5e7eb;
	}

	:global(.dark) .list-block__check-text:focus {
		background: rgba(96, 165, 250, 0.1);
	}

	:global(.dark) .list-block__check-item--checked .list-block__check-text {
		color: #6b7280;
	}

	:global(.dark) .list-block__remove-btn:hover {
		color: #ef4444;
		background: #450a0a;
	}

	:global(.dark) .list-block__add-btn {
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .list-block__add-btn:hover {
		border-color: #60a5fa;
		color: #60a5fa;
		background: rgba(96, 165, 250, 0.1);
	}

	:global(.dark) .list-block__hint {
		color: #6b7280;
	}
</style>
