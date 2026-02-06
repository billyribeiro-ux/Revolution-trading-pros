<!--
/**
 * Accordion Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * ICT Level 7 - Collapsible accordion with keyboard navigation
 * 
 * Features:
 * - Full ARIA compliance (WAI-ARIA Accordion Pattern)
 * - Keyboard navigation (Arrow keys, Home, End)
 * - SSR-safe contenteditable handling
 * - Error boundary integration
 * - Proper cleanup on unmount
 */
-->

<script lang="ts">
	import { untrack } from 'svelte';
	import type { Action } from 'svelte/action';
	import { browser } from '$app/environment';
	import { IconChevronDown, IconPlus, IconMinus, IconX } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	interface AccordionItem {
		id: string;
		title: string;
		content: string;
	}

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const { block, blockId, isSelected, isEditing, onUpdate, onError }: Props = $props();

	const stateManager = getBlockStateManager();

	const openItems = $derived(stateManager.getAccordionState(blockId));
	const allowMultiple = $derived(Boolean(block.settings.allowMultiple));
	const iconStyle = $derived<'chevron' | 'plusminus'>(
		(block.settings.iconStyle as 'chevron' | 'plusminus') ?? 'chevron'
	);

	const DEFAULT_ITEM: AccordionItem = {
		id: 'acc_1',
		title: 'Section 1',
		content: 'Content for section 1'
	};
	const items = $derived<AccordionItem[]>(block.content.accordionItems ?? [DEFAULT_ITEM]);

	let editableRefs = $state<Map<string, HTMLElement>>(new Map());

	$effect(() => {
		return () => {
			editableRefs.clear();
		};
	});

	function safeExecute<T>(fn: () => T, fallback: T): T {
		try {
			return fn();
		} catch (error) {
			onError?.(error instanceof Error ? error : new Error(String(error)));
			return fallback;
		}
	}

	function updateContent(updates: Partial<BlockContent>): void {
		safeExecute(() => {
			onUpdate({ content: { ...block.content, ...updates } });
		}, undefined);
	}

	function toggleItem(itemId: string): void {
		untrack(() => {
			stateManager.toggleAccordionItem(blockId, itemId, allowMultiple);
		});
	}

	function updateItem(index: number, field: 'title' | 'content', value: string): void {
		const newItems = items.map((item, i) => (i === index ? { ...item, [field]: value } : item));
		updateContent({ accordionItems: newItems });
	}

	function addItem(): void {
		const newItems: AccordionItem[] = [
			...items,
			{
				id: `acc_${crypto.randomUUID()}`,
				title: `Section ${items.length + 1}`,
				content: ''
			}
		];
		updateContent({ accordionItems: newItems });
	}

	function removeItem(index: number): void {
		if (items.length <= 1) return;
		const newItems = items.filter((_, i) => i !== index);
		updateContent({ accordionItems: newItems });
	}

	function focusButton(itemId: string | undefined): void {
		if (!browser || !itemId) return;
		const el = document.getElementById(`accordion-btn-${blockId}-${itemId}`);
		el?.focus();
	}

	function handleKeyDown(e: KeyboardEvent, index: number): void {
		const keyActions: Record<string, () => void> = {
			ArrowDown: () => focusButton(items[index + 1]?.id),
			ArrowUp: () => focusButton(items[index - 1]?.id),
			Home: () => focusButton(items[0]?.id),
			End: () => focusButton(items[items.length - 1]?.id)
		};

		const action = keyActions[e.key];
		if (action) {
			e.preventDefault();
			action();
		}
	}

	function handlePaste(e: ClipboardEvent, target: HTMLElement): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') ?? '';
		const selection = window.getSelection();
		if (!selection?.rangeCount) return;

		const range = selection.getRangeAt(0);
		range.deleteContents();
		range.insertNode(document.createTextNode(text));
		range.collapse(false);
		selection.removeAllRanges();
		selection.addRange(range);

		target.dispatchEvent(new Event('input', { bubbles: true }));
	}

	function handleEditableInput(e: Event, index: number, field: 'title' | 'content'): void {
		const target = e.target as HTMLElement | null;
		if (!target) return;
		updateItem(index, field, target.textContent ?? '');
	}

	const editableAction: Action<HTMLElement, string> = (el, key) => {
		if (key) editableRefs.set(key, el);
		return {
			destroy() {
				if (key) editableRefs.delete(key);
			}
		};
	};
</script>

<div class="accordion-block" role="region" aria-label="Accordion">
	{#each items as item, index (item.id)}
		{@const isOpen = openItems.has(item.id)}
		{@const titleKey = `title-${item.id}`}
		{@const contentKey = `content-${item.id}`}
		<div class="accordion-item" class:open={isOpen}>
			<div class="accordion-header-wrapper">
				<button
					type="button"
					class="accordion-header"
					id="accordion-btn-{blockId}-{item.id}"
					aria-expanded={isOpen}
					aria-controls="accordion-panel-{blockId}-{item.id}"
					onclick={() => toggleItem(item.id)}
					onkeydown={(e) => handleKeyDown(e, index)}
				>
					{#if isEditing && browser}
						<span
							use:editableAction={titleKey}
							contenteditable="true"
							class="accordion-title editable-content"
							role="textbox"
							tabindex={0}
							aria-label="Edit section title: {item.title}"
							onclick={(e) => e.stopPropagation()}
							onkeydown={(e) => e.stopPropagation()}
							oninput={(e) => handleEditableInput(e, index, 'title')}
							onpaste={(e) => handlePaste(e, e.currentTarget)}>{item.title}</span
						>
					{:else}
						<span class="accordion-title">{item.title}</span>
					{/if}

					<span
						class="accordion-icon"
						class:rotated={isOpen && iconStyle === 'chevron'}
						aria-hidden="true"
					>
						{#if iconStyle === 'plusminus'}
							{#if isOpen}
								<IconMinus size={18} />
							{:else}
								<IconPlus size={18} />
							{/if}
						{:else}
							<IconChevronDown size={18} />
						{/if}
					</span>
				</button>

				{#if isEditing && items.length > 1}
					<button
						type="button"
						class="remove-item-btn"
						onclick={() => removeItem(index)}
						aria-label="Remove section: {item.title}"
					>
						<IconX size={14} />
					</button>
				{/if}
			</div>

			<div
				id="accordion-panel-{blockId}-{item.id}"
				class="accordion-panel"
				role="region"
				aria-labelledby="accordion-btn-{blockId}-{item.id}"
				hidden={!isOpen}
			>
				{#if isEditing && browser}
					<div
						use:editableAction={contentKey}
						contenteditable="true"
						class="accordion-content editable-content"
						role="textbox"
						aria-label="Edit section content"
						oninput={(e) => handleEditableInput(e, index, 'content')}
						onpaste={(e) => handlePaste(e, e.currentTarget)}
					>
						{item.content}
					</div>
				{:else}
					<div class="accordion-content">{item.content}</div>
				{/if}
			</div>
		</div>
	{/each}

	{#if isEditing}
		<button type="button" class="add-item-btn" onclick={addItem}>
			<IconPlus size={14} aria-hidden="true" />
			Add Section
		</button>

		{#if isSelected}
			<div class="accordion-settings" role="group" aria-label="Accordion settings">
				<label class="setting-checkbox">
					<input
						type="checkbox"
						checked={allowMultiple}
						onchange={(e) =>
							onUpdate({
								settings: {
									...block.settings,
									allowMultiple: (e.target as HTMLInputElement).checked
								}
							})}
					/>
					<span>Allow multiple sections open</span>
				</label>

				<fieldset class="setting-radio">
					<legend class="setting-label">Icon style:</legend>
					<label>
						<input
							type="radio"
							name="icon-style-{blockId}"
							value="chevron"
							checked={iconStyle === 'chevron'}
							onchange={() =>
								onUpdate({
									settings: { ...block.settings, iconStyle: 'chevron' }
								})}
						/>
						<span>Chevron</span>
					</label>
					<label>
						<input
							type="radio"
							name="icon-style-{blockId}"
							value="plusminus"
							checked={iconStyle === 'plusminus'}
							onchange={() =>
								onUpdate({
									settings: { ...block.settings, iconStyle: 'plusminus' }
								})}
						/>
						<span>Plus/Minus</span>
					</label>
				</fieldset>
			</div>
		{/if}
	{/if}
</div>

<style>
	.accordion-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}

	.accordion-item {
		border-bottom: 1px solid #e5e7eb;
	}

	.accordion-item:last-of-type {
		border-bottom: none;
	}

	.accordion-header-wrapper {
		display: flex;
		align-items: center;
		background: #f9fafb;
		transition: background 0.15s;
	}

	.accordion-header-wrapper:hover {
		background: #f3f4f6;
	}

	.accordion-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		width: 100%;
		padding: 1.25rem 1.5rem;
		background: transparent;
		border: none;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 500;
		color: #1f2937;
		text-align: left;
		position: relative;
	}

	.accordion-header:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
		z-index: 1;
	}

	.accordion-title {
		flex: 1;
		outline: none;
	}

	.accordion-icon {
		display: flex;
		color: #6b7280;
		transition: transform 0.2s;
		flex-shrink: 0;
	}

	.accordion-icon.rotated {
		transform: rotate(180deg);
	}

	.remove-item-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.375rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s;
		flex-shrink: 0;
		opacity: 0;
	}

	.remove-item-btn:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	.accordion-panel {
		overflow: hidden;
		animation: slideDown 0.2s ease-out;
	}

	.accordion-panel[hidden] {
		display: none;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			max-height: 0;
		}
		to {
			opacity: 1;
			max-height: 500px;
		}
	}

	.accordion-content {
		padding: 1.25rem 1.5rem;
		background: white;
		line-height: 1.7;
		color: #374151;
		outline: none;
	}

	.add-item-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		margin: 1rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 6px;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-item-btn:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.add-item-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.accordion-settings {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 1rem 1.5rem;
		background: #f9fafb;
		border-top: 1px solid #e5e7eb;
	}

	.setting-checkbox,
	.setting-radio {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
		color: #374151;
	}

	.setting-radio {
		flex-wrap: wrap;
	}

	.setting-checkbox input,
	.setting-radio input {
		cursor: pointer;
	}

	.setting-label {
		font-weight: 500;
		min-width: 100%;
	}

	.setting-radio label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.setting-radio legend {
		font-weight: 500;
		width: 100%;
		margin-bottom: 0.5rem;
	}

	fieldset.setting-radio {
		border: none;
		padding: 0;
		margin: 0;
	}

	/* Dark Mode */
	:global(.dark) .accordion-block {
		border-color: #374151;
	}

	:global(.dark) .accordion-item {
		border-color: #374151;
	}

	:global(.dark) .accordion-header-wrapper {
		background: #1e293b;
	}

	:global(.dark) .accordion-header-wrapper:hover {
		background: #334155;
	}

	:global(.dark) .accordion-header {
		color: #f9fafb;
	}

	:global(.dark) .accordion-content {
		background: #111827;
		color: #e5e7eb;
	}

	:global(.dark) .accordion-settings {
		background: #1e293b;
		border-color: #374151;
	}

	:global(.dark) .setting-checkbox,
	:global(.dark) .setting-radio {
		color: #e5e7eb;
	}

	:global(.dark) .add-item-btn {
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .add-item-btn:hover {
		border-color: #60a5fa;
		color: #60a5fa;
	}

	:global(.dark) .remove-item-btn:hover {
		background: #450a0a;
		color: #fca5a5;
	}

	/* Mobile */
	@media (max-width: 640px) {
		.accordion-header {
			padding: 1rem;
		}

		.accordion-content {
			padding: 1rem;
		}
	}
</style>
