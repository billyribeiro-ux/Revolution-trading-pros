<!--
/**
 * Checklist Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Interactive checklist with full keyboard navigation, progress tracking,
 * and customizable styling. Supports adding, removing, and toggling items
 * with complete ARIA accessibility.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { IconPlus, IconX, IconCheck, IconCircleCheck, IconProgress } from '$lib/icons';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	interface ChecklistItem {
		id: string;
		text: string;
		checked: boolean;
	}

	interface ChecklistContent {
		items?: ChecklistItem[];
		showProgress?: boolean;
		strikethrough?: boolean;
		title?: string;
	}

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// =========================================================================
	// Props & State
	// =========================================================================

	const props: Props = $props();

	// Derived content values with defaults
	const content = $derived(props.block.content as ChecklistContent);
	const items = $derived(content.items || []);
	const showProgress = $derived(content.showProgress ?? true);
	const strikethrough = $derived(content.strikethrough ?? true);
	const title = $derived(content.title || '');

	// Progress calculations
	const totalItems = $derived(items.length);
	const completedItems = $derived(items.filter((item) => item.checked).length);
	const progressPercent = $derived(totalItems > 0 ? Math.round((completedItems / totalItems) * 100) : 0);

	// Generate unique IDs for ARIA
	const listId = $derived(`checklist-${props.blockId}`);
	const titleId = $derived(`checklist-title-${props.blockId}`);
	const progressId = $derived(`checklist-progress-${props.blockId}`);

	// Track which item is being focused for keyboard navigation
	let focusedItemIndex = $state<number | null>(null);

	// =========================================================================
	// Content Update Handlers
	// =========================================================================

	/**
	 * Updates block content with partial changes
	 */
	function updateContent(updates: Partial<ChecklistContent>): void {
		// Cast to handle custom ChecklistContent type alongside BlockContent
		props.onUpdate({ content: { ...props.block.content, ...updates } as BlockContent });
	}

	/**
	 * Adds a new item to the checklist
	 */
	function addItem(): void {
		const newItem: ChecklistItem = {
			id: crypto.randomUUID(),
			text: '',
			checked: false
		};
		const newItems = [...items, newItem];
		updateContent({ items: newItems });

		// Focus the new item after render
		requestAnimationFrame(() => {
			const newIndex = newItems.length - 1;
			focusItemInput(newIndex);
		});
	}

	/**
	 * Adds a new item after a specific index
	 */
	function addItemAfter(index: number): void {
		const newItem: ChecklistItem = {
			id: crypto.randomUUID(),
			text: '',
			checked: false
		};
		const newItems = [...items];
		newItems.splice(index + 1, 0, newItem);
		updateContent({ items: newItems });

		// Focus the new item after render
		requestAnimationFrame(() => {
			focusItemInput(index + 1);
		});
	}

	/**
	 * Removes an item from the checklist
	 */
	function removeItem(id: string): void {
		const itemIndex = items.findIndex((item) => item.id === id);
		const newItems = items.filter((item) => item.id !== id);

		// Only remove if at least one item remains or allow empty list
		if (newItems.length >= 0) {
			updateContent({ items: newItems });

			// Focus previous item or next item if available
			if (newItems.length > 0) {
				requestAnimationFrame(() => {
					const focusIndex = Math.min(itemIndex, newItems.length - 1);
					focusItemInput(focusIndex);
				});
			}
		}
	}

	/**
	 * Toggles the checked state of an item
	 */
	function toggleItem(id: string): void {
		const newItems = items.map((item) =>
			item.id === id ? { ...item, checked: !item.checked } : item
		);
		updateContent({ items: newItems });
	}

	/**
	 * Updates the text content of an item
	 */
	function updateItemText(id: string, text: string): void {
		const newItems = items.map((item) => (item.id === id ? { ...item, text } : item));
		updateContent({ items: newItems });
	}

	/**
	 * Updates the checklist title
	 */
	function updateTitle(text: string): void {
		updateContent({ title: text });
	}

	/**
	 * Toggles the progress indicator visibility
	 */
	function toggleShowProgress(): void {
		updateContent({ showProgress: !showProgress });
	}

	/**
	 * Toggles strikethrough styling for completed items
	 */
	function toggleStrikethrough(): void {
		updateContent({ strikethrough: !strikethrough });
	}

	// =========================================================================
	// Event Handlers
	// =========================================================================

	/**
	 * Handles input changes for item text
	 */
	function handleItemInput(e: Event, id: string): void {
		const target = e.target as HTMLElement;
		updateItemText(id, target.textContent || '');
	}

	/**
	 * Handles title input changes
	 */
	function handleTitleInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateTitle(target.textContent || '');
	}

	/**
	 * Handles paste events to strip formatting
	 */
	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	/**
	 * Handles keyboard navigation and actions within items
	 */
	function handleItemKeyDown(e: KeyboardEvent, id: string, index: number): void {
		const target = e.target as HTMLElement;
		const item = items.find((i) => i.id === id);

		switch (e.key) {
			case 'Enter':
				e.preventDefault();
				// Add new item after current
				addItemAfter(index);
				break;

			case 'Backspace':
				// Remove item if text is empty and there are other items
				if (!item?.text && items.length > 1) {
					e.preventDefault();
					removeItem(id);
				}
				break;

			case 'ArrowUp':
				if (e.altKey || index === 0) return;
				e.preventDefault();
				focusItemInput(index - 1);
				break;

			case 'ArrowDown':
				if (e.altKey || index === items.length - 1) return;
				e.preventDefault();
				focusItemInput(index + 1);
				break;

			case 'Tab':
				// Allow default tab behavior but track focus
				focusedItemIndex = e.shiftKey
					? Math.max(0, index - 1)
					: Math.min(items.length - 1, index + 1);
				break;
		}
	}

	/**
	 * Handles checkbox keyboard interaction (Space/Enter to toggle)
	 */
	function handleCheckboxKeyDown(e: KeyboardEvent, id: string): void {
		if (e.key === ' ' || e.key === 'Enter') {
			e.preventDefault();
			toggleItem(id);
		}
	}

	/**
	 * Focuses the input field of an item at the given index
	 */
	function focusItemInput(index: number): void {
		const input = document.querySelector(
			`[data-item-index="${index}"][data-block="${props.blockId}"]`
		) as HTMLElement;
		if (input) {
			input.focus();
			// Move cursor to end of text
			const selection = window.getSelection();
			const range = document.createRange();
			range.selectNodeContents(input);
			range.collapse(false);
			selection?.removeAllRanges();
			selection?.addRange(range);
		}
		focusedItemIndex = index;
	}

	/**
	 * Handles focus on item input
	 */
	function handleItemFocus(index: number): void {
		focusedItemIndex = index;
	}

	/**
	 * Handles blur on item input
	 */
	function handleItemBlur(): void {
		focusedItemIndex = null;
	}
</script>

<div
	class="checklist-block"
	class:has-progress={showProgress && totalItems > 0}
	role="region"
	aria-labelledby={title ? titleId : undefined}
>
	<!-- Optional Title -->
	{#if title || props.isEditing}
		<div class="checklist-header">
			<h4
				id={titleId}
				contenteditable={props.isEditing}
				class="checklist-title editable-content"
				class:placeholder={!title}
				oninput={handleTitleInput}
				onpaste={handlePaste}
				data-placeholder="Checklist title (optional)"
				role={props.isEditing ? 'textbox' : undefined}
				aria-label={props.isEditing ? 'Checklist title' : undefined}
			>
				{title}
			</h4>
		</div>
	{/if}

	<!-- Progress Indicator -->
	{#if showProgress && totalItems > 0}
		<div class="progress-section" id={progressId} aria-live="polite">
			<div class="progress-header">
				<span class="progress-text">
					<IconProgress size={16} aria-hidden="true" />
					<span class="progress-count">{completedItems} of {totalItems} completed</span>
				</span>
				<span class="progress-percent">{progressPercent}%</span>
			</div>
			<div
				class="progress-bar"
				role="progressbar"
				aria-valuenow={progressPercent}
				aria-valuemin={0}
				aria-valuemax={100}
				aria-label="Checklist completion progress"
			>
				<div
					class="progress-fill"
					class:complete={progressPercent === 100}
					style:width="{progressPercent}%"
				></div>
			</div>
		</div>
	{/if}

	<!-- Checklist Items -->
	<ul id={listId} class="checklist-items" role="list" aria-describedby={showProgress ? progressId : undefined}>
		{#each items as item, index (item.id)}
			<li
				class="checklist-item"
				class:checked={item.checked}
				class:strikethrough={strikethrough && item.checked}
				class:focused={focusedItemIndex === index}
				role="listitem"
			>
				<!-- Custom Checkbox -->
				<button
					type="button"
					class="checkbox"
					class:checked={item.checked}
					onclick={() => toggleItem(item.id)}
					onkeydown={(e) => handleCheckboxKeyDown(e, item.id)}
					disabled={!props.isEditing}
					role="checkbox"
					aria-checked={item.checked}
					aria-label={item.checked ? 'Mark as incomplete' : 'Mark as complete'}
					tabindex={props.isEditing ? 0 : -1}
				>
					{#if item.checked}
						<IconCheck size={14} aria-hidden="true" />
					{/if}
				</button>

				<!-- Item Text -->
				<span
					contenteditable={props.isEditing}
					class="item-text editable-content"
					class:placeholder={!item.text}
					oninput={(e) => handleItemInput(e, item.id)}
					onpaste={handlePaste}
					onkeydown={(e) => handleItemKeyDown(e, item.id, index)}
					onfocus={() => handleItemFocus(index)}
					onblur={handleItemBlur}
					data-placeholder="New item..."
					data-item-index={index}
					data-block={props.blockId}
					role={props.isEditing ? 'textbox' : undefined}
					aria-label={props.isEditing ? `Item ${index + 1} text` : undefined}
				>
					{item.text}
				</span>

				<!-- Remove Button -->
				{#if props.isEditing}
					<button
						type="button"
						class="remove-btn"
						onclick={() => removeItem(item.id)}
						aria-label="Remove item"
						title="Remove item"
					>
						<IconX size={14} aria-hidden="true" />
					</button>
				{/if}
			</li>
		{/each}
	</ul>

	<!-- Add Item Button -->
	{#if props.isEditing}
		<button type="button" class="add-item-btn" onclick={addItem} aria-label="Add new checklist item">
			<IconPlus size={16} aria-hidden="true" />
			<span>Add item</span>
		</button>
	{/if}

	<!-- Empty State -->
	{#if items.length === 0 && !props.isEditing}
		<div class="empty-state" role="status">
			<IconCircleCheck size={32} aria-hidden="true" />
			<p>No items in this checklist</p>
		</div>
	{/if}
</div>

<!-- Style Controls (visible when editing and selected) -->
{#if props.isEditing && props.isSelected}
	<div class="checklist-controls" role="toolbar" aria-label="Checklist options">
		<label class="control-toggle">
			<input
				type="checkbox"
				checked={showProgress}
				onchange={toggleShowProgress}
				aria-describedby="progress-toggle-desc"
			/>
			<span class="toggle-switch"></span>
			<span class="toggle-label" id="progress-toggle-desc">Show progress</span>
		</label>

		<label class="control-toggle">
			<input
				type="checkbox"
				checked={strikethrough}
				onchange={toggleStrikethrough}
				aria-describedby="strikethrough-toggle-desc"
			/>
			<span class="toggle-switch"></span>
			<span class="toggle-label" id="strikethrough-toggle-desc">Strikethrough completed</span>
		</label>
	</div>
{/if}

<style>
	/* =========================================================================
	 * Base Styles
	 * ========================================================================= */

	.checklist-block {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	/* =========================================================================
	 * Header & Title
	 * ========================================================================= */

	.checklist-header {
		margin-bottom: 0.25rem;
	}

	.checklist-title {
		margin: 0;
		font-size: 1.0625rem;
		font-weight: 600;
		color: #1f2937;
		outline: none;
		min-height: 1.5em;
	}

	.checklist-title.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		font-weight: normal;
	}

	/* =========================================================================
	 * Progress Section
	 * ========================================================================= */

	.progress-section {
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border-radius: 8px;
		margin-bottom: 0.25rem;
	}

	.progress-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.progress-text {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #64748b;
	}

	.progress-count {
		color: #374151;
	}

	.progress-percent {
		font-size: 0.875rem;
		font-weight: 600;
		color: #3b82f6;
	}

	.progress-bar {
		height: 6px;
		background: #e2e8f0;
		border-radius: 999px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #60a5fa);
		border-radius: 999px;
		transition: width 0.3s ease;
	}

	.progress-fill.complete {
		background: linear-gradient(90deg, #10b981, #34d399);
	}

	/* =========================================================================
	 * Checklist Items
	 * ========================================================================= */

	.checklist-items {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		margin: 0;
		padding: 0;
		list-style: none;
	}

	.checklist-item {
		display: flex;
		align-items: flex-start;
		gap: 0.625rem;
		padding: 0.5rem 0.625rem;
		border-radius: 6px;
		transition: background-color 0.15s;
	}

	.checklist-item:hover {
		background: #f8fafc;
	}

	.checklist-item.focused {
		background: #eff6ff;
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
	}

	/* =========================================================================
	 * Checkbox
	 * ========================================================================= */

	.checkbox {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 22px;
		height: 22px;
		min-width: 22px;
		margin-top: 1px;
		padding: 0;
		background: white;
		border: 2px solid #d1d5db;
		border-radius: 5px;
		cursor: pointer;
		transition: all 0.15s;
		color: transparent;
	}

	.checkbox:hover:not(:disabled) {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.checkbox:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.checkbox.checked {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.checkbox:disabled {
		cursor: default;
	}

	/* =========================================================================
	 * Item Text
	 * ========================================================================= */

	.item-text {
		flex: 1;
		font-size: 0.9375rem;
		line-height: 1.5;
		color: #374151;
		outline: none;
		min-height: 1.5em;
		word-break: break-word;
	}

	.item-text.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
	}

	.checklist-item.strikethrough .item-text {
		text-decoration: line-through;
		color: #9ca3af;
	}

	.checklist-item.checked:not(.strikethrough) .item-text {
		color: #6b7280;
	}

	/* =========================================================================
	 * Remove Button
	 * ========================================================================= */

	.remove-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		padding: 0;
		margin-top: 1px;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #9ca3af;
		opacity: 0;
		transition: all 0.15s;
	}

	.checklist-item:hover .remove-btn,
	.checklist-item.focused .remove-btn {
		opacity: 1;
	}

	.remove-btn:hover {
		color: #ef4444;
		background: #fee2e2;
	}

	.remove-btn:focus-visible {
		opacity: 1;
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* =========================================================================
	 * Add Item Button
	 * ========================================================================= */

	.add-item-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		margin-top: 0.25rem;
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
		background: rgba(59, 130, 246, 0.05);
	}

	.add-item-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* =========================================================================
	 * Empty State
	 * ========================================================================= */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		color: #9ca3af;
		text-align: center;
	}

	.empty-state p {
		margin: 0.75rem 0 0;
		font-size: 0.875rem;
	}

	/* =========================================================================
	 * Controls
	 * ========================================================================= */

	.checklist-controls {
		display: flex;
		flex-wrap: wrap;
		gap: 1.25rem;
		margin-top: 1rem;
		padding: 0.875rem 1rem;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
	}

	.control-toggle {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		cursor: pointer;
	}

	.control-toggle input {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle-switch {
		position: relative;
		width: 36px;
		height: 20px;
		background: #d1d5db;
		border-radius: 999px;
		transition: background-color 0.2s;
	}

	.toggle-switch::after {
		content: '';
		position: absolute;
		top: 2px;
		left: 2px;
		width: 16px;
		height: 16px;
		background: white;
		border-radius: 50%;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
		transition: transform 0.2s;
	}

	.control-toggle input:checked + .toggle-switch {
		background: #3b82f6;
	}

	.control-toggle input:checked + .toggle-switch::after {
		transform: translateX(16px);
	}

	.control-toggle input:focus-visible + .toggle-switch {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.toggle-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	/* =========================================================================
	 * Dark Mode
	 * ========================================================================= */

	:global(.dark) .checklist-title {
		color: #f1f5f9;
	}

	:global(.dark) .checklist-title.placeholder:empty::before {
		color: #64748b;
	}

	:global(.dark) .progress-section {
		background: rgba(30, 41, 59, 0.5);
	}

	:global(.dark) .progress-text {
		color: #94a3b8;
	}

	:global(.dark) .progress-count {
		color: #e2e8f0;
	}

	:global(.dark) .progress-percent {
		color: #60a5fa;
	}

	:global(.dark) .progress-bar {
		background: #334155;
	}

	:global(.dark) .checklist-item:hover {
		background: rgba(30, 41, 59, 0.5);
	}

	:global(.dark) .checklist-item.focused {
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .checkbox {
		background: #1e293b;
		border-color: #475569;
	}

	:global(.dark) .checkbox:hover:not(:disabled) {
		border-color: #60a5fa;
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .checkbox.checked {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	:global(.dark) .item-text {
		color: #e2e8f0;
	}

	:global(.dark) .item-text.placeholder:empty::before {
		color: #64748b;
	}

	:global(.dark) .checklist-item.strikethrough .item-text {
		color: #64748b;
	}

	:global(.dark) .checklist-item.checked:not(.strikethrough) .item-text {
		color: #94a3b8;
	}

	:global(.dark) .remove-btn {
		color: #64748b;
	}

	:global(.dark) .remove-btn:hover {
		color: #ef4444;
		background: rgba(239, 68, 68, 0.15);
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

	:global(.dark) .empty-state {
		color: #64748b;
	}

	:global(.dark) .checklist-controls {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .toggle-switch {
		background: #475569;
	}

	:global(.dark) .control-toggle input:checked + .toggle-switch {
		background: #3b82f6;
	}

	:global(.dark) .toggle-label {
		color: #e2e8f0;
	}

	/* =========================================================================
	 * Responsive Design
	 * ========================================================================= */

	@media (max-width: 640px) {
		.checklist-item {
			padding: 0.625rem 0.5rem;
		}

		.progress-section {
			padding: 0.625rem 0.75rem;
		}

		.progress-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.25rem;
		}

		.checklist-controls {
			flex-direction: column;
			gap: 0.875rem;
		}

		.remove-btn {
			opacity: 1;
		}
	}
</style>
