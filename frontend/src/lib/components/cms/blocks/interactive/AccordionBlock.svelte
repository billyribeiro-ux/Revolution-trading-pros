<!--
/**
 * Accordion Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Collapsible accordion with multiple items and keyboard navigation
 */
-->

<script lang="ts">
	import { IconChevronDown, IconPlus, IconMinus, IconX } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();
	const stateManager = getBlockStateManager();

	let openItems = $derived(stateManager.getAccordionState(props.blockId));
	let allowMultiple = $derived(props.block.settings.allowMultiple || false);
	let iconStyle = $derived((props.block.settings.iconStyle as 'chevron' | 'plusminus') || 'chevron');

	let items = $derived(
		props.block.content.accordionItems || [
			{ id: 'acc_1', title: 'Section 1', content: 'Content for section 1' }
		]
	);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function toggleItem(itemId: string): void {
		stateManager.toggleAccordionItem(props.blockId, itemId, allowMultiple);
	}

	function updateItem(index: number, field: 'title' | 'content', value: string): void {
		const newItems = items.map((item, i) => (i === index ? { ...item, [field]: value } : item));
		updateContent({ accordionItems: newItems });
	}

	function addItem(): void {
		const newItems = [
			...items,
			{
				id: `acc_${Date.now()}`,
				title: `Section ${items.length + 1}`,
				content: ''
			}
		];
		updateContent({ accordionItems: newItems });
	}

	function removeItem(index: number): void {
		if (items.length > 1) {
			const newItems = items.filter((_, i) => i !== index);
			updateContent({ accordionItems: newItems });
		}
	}

	function handleKeyDown(e: KeyboardEvent, index: number): void {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			const next = items[index + 1];
			if (next) {
				document.getElementById(`accordion-btn-${props.blockId}-${next.id}`)?.focus();
			}
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			const prev = items[index - 1];
			if (prev) {
				document.getElementById(`accordion-btn-${props.blockId}-${prev.id}`)?.focus();
			}
		} else if (e.key === 'Home') {
			e.preventDefault();
			document.getElementById(`accordion-btn-${props.blockId}-${items[0].id}`)?.focus();
		} else if (e.key === 'End') {
			e.preventDefault();
			document.getElementById(`accordion-btn-${props.blockId}-${items[items.length - 1].id}`)?.focus();
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}
</script>

<div class="accordion-block" role="region" aria-label="Accordion">
	{#each items as item, index (item.id)}
		{@const isOpen = openItems.has(item.id)}
		<div class="accordion-item" class:open={isOpen}>
			<div class="accordion-header-wrapper">
				{#if props.isEditing}
					<!-- When editing, use separate elements for toggle and editable title -->
					<div class="accordion-header editing">
						<span
							contenteditable="true"
							class="accordion-title editable-content"
							role="textbox"
							aria-label="Section title"
							oninput={(e) => updateItem(index, 'title', (e.target as HTMLElement).textContent || '')}
							onpaste={handlePaste}
						>
							{item.title}
						</span>
						<button
							type="button"
							class="accordion-toggle-btn"
							id="accordion-btn-{props.blockId}-{item.id}"
							aria-expanded={isOpen}
							aria-controls="accordion-panel-{props.blockId}-{item.id}"
							aria-label="{isOpen ? 'Collapse' : 'Expand'} section"
							onclick={() => toggleItem(item.id)}
							onkeydown={(e) => handleKeyDown(e, index)}
						>
							<span class="accordion-icon" class:rotated={isOpen && iconStyle === 'chevron'} aria-hidden="true">
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
					</div>
				{:else}
					<!-- When not editing, use a single button for the entire header -->
					<button
						type="button"
						class="accordion-header"
						id="accordion-btn-{props.blockId}-{item.id}"
						aria-expanded={isOpen}
						aria-controls="accordion-panel-{props.blockId}-{item.id}"
						onclick={() => toggleItem(item.id)}
						onkeydown={(e) => handleKeyDown(e, index)}
					>
						<span class="accordion-title">{item.title}</span>
						<span class="accordion-icon" class:rotated={isOpen && iconStyle === 'chevron'} aria-hidden="true">
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
				{/if}

				{#if props.isEditing && items.length > 1}
					<button
						type="button"
						class="remove-item-btn"
						onclick={() => removeItem(index)}
						aria-label="Remove section"
					>
						<IconX size={14} />
					</button>
				{/if}
			</div>

			<div
				id="accordion-panel-{props.blockId}-{item.id}"
				class="accordion-panel"
				role="region"
				aria-labelledby="accordion-btn-{props.blockId}-{item.id}"
				hidden={!isOpen}
			>
				{#if props.isEditing}
					<div
						contenteditable="true"
						class="accordion-content editable-content"
						role="textbox"
						aria-label="Section content"
						oninput={(e) => updateItem(index, 'content', (e.target as HTMLElement).textContent || '')}
						onpaste={handlePaste}
					>
						{item.content}
					</div>
				{:else}
					<div class="accordion-content">{item.content}</div>
				{/if}
			</div>
		</div>
	{/each}

	{#if props.isEditing}
		<button type="button" class="add-item-btn" onclick={addItem}>
			<IconPlus size={14} aria-hidden="true" />
			Add Section
		</button>

		{#if props.isSelected}
			<div class="accordion-settings">
				<label class="setting-checkbox">
					<input
						type="checkbox"
						checked={allowMultiple}
						onchange={(e) =>
							props.onUpdate({
								settings: {
									...props.block.settings,
									allowMultiple: (e.target as HTMLInputElement).checked
								}
							})}
					/>
					<span>Allow multiple sections open</span>
				</label>

				<div class="setting-radio">
					<span class="setting-label">Icon style:</span>
					<label>
						<input
							type="radio"
							name="icon-style-{props.blockId}"
							value="chevron"
							checked={iconStyle === 'chevron'}
							onchange={() =>
								props.onUpdate({
									settings: { ...props.block.settings, iconStyle: 'chevron' }
								})}
						/>
						<span>Chevron</span>
					</label>
					<label>
						<input
							type="radio"
							name="icon-style-{props.blockId}"
							value="plusminus"
							checked={iconStyle === 'plusminus'}
							onchange={() =>
								props.onUpdate({
									settings: { ...props.block.settings, iconStyle: 'plusminus' }
								})}
						/>
						<span>Plus/Minus</span>
					</label>
				</div>
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
		position: relative;
	}

	.accordion-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		flex: 1;
		padding: 1.25rem 1.5rem;
		background: #f9fafb;
		border: none;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 500;
		color: #1f2937;
		text-align: left;
		transition: background 0.15s;
		position: relative;
	}

	.accordion-header:hover {
		background: #f3f4f6;
	}

	.accordion-header:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
		z-index: 1;
	}

	/* Editing mode: header is a div with separate toggle button */
	.accordion-header.editing {
		cursor: default;
	}

	.accordion-toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.accordion-toggle-btn:hover {
		background: rgba(59, 130, 246, 0.1);
		color: #3b82f6;
	}

	.accordion-toggle-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
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
		margin-right: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s;
		flex-shrink: 0;
		opacity: 0;
	}

	.accordion-header-wrapper:hover .remove-item-btn {
		opacity: 1;
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

	/* Dark Mode */
	:global(.dark) .accordion-block {
		border-color: #374151;
	}

	:global(.dark) .accordion-item {
		border-color: #374151;
	}

	:global(.dark) .accordion-header {
		background: #1e293b;
		color: #f9fafb;
	}

	:global(.dark) .accordion-header:hover {
		background: #334155;
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

	:global(.dark) .accordion-toggle-btn {
		color: #94a3b8;
	}

	:global(.dark) .accordion-toggle-btn:hover {
		background: rgba(96, 165, 250, 0.1);
		color: #60a5fa;
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
