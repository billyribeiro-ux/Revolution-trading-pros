<!--
/**
 * Toggle Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Collapsible content section with toggle header
 */
-->

<script lang="ts">
	import { IconChevronDown, IconPlus, IconMinus } from '$lib/icons';
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

	let defaultOpen = $derived(props.block.settings.defaultOpen || false);
	let iconStyle = $derived<'chevron' | 'plusminus'>(props.block.settings.iconStyle || 'chevron');
	let isOpen = $derived(stateManager.getToggleState(props.blockId, defaultOpen));

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function toggle(): void {
		stateManager.toggleToggle(props.blockId);
	}

	function handleTitleInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ toggleTitle: target.textContent || '' });
	}

	function handleContentInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ toggleContent: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	let title = $derived(props.block.content.toggleTitle || 'Click to expand');
	let content = $derived(props.block.content.toggleContent || '');
</script>

<div class="toggle-block" class:open={isOpen}>
	<button
		type="button"
		class="toggle-header"
		aria-expanded={isOpen}
		aria-controls="toggle-content-{props.blockId}"
		onclick={toggle}
	>
		{#if props.isEditing}
			<span
				contenteditable="true"
				class="toggle-title editable-content"
				role="textbox"
				tabindex="0"
				aria-label="Toggle title"
				onclick={(e) => e.stopPropagation()}
				oninput={handleTitleInput}
				onpaste={handlePaste}
			>
				{title}
			</span>
		{:else}
			<span class="toggle-title">{title}</span>
		{/if}

		<span class="toggle-icon" class:rotated={isOpen && iconStyle === 'chevron'} aria-hidden="true">
			{#if iconStyle === 'plusminus'}
				{#if isOpen}
					<IconMinus size={20} />
				{:else}
					<IconPlus size={20} />
				{/if}
			{:else}
				<IconChevronDown size={20} />
			{/if}
		</span>
	</button>

	<div
		id="toggle-content-{props.blockId}"
		class="toggle-content-wrapper"
		hidden={!isOpen}
	>
		{#if props.isEditing}
			<div
				contenteditable="true"
				class="toggle-content editable-content"
				class:placeholder={!content}
				role="textbox"
				aria-label="Toggle content"
				data-placeholder="Add hidden content..."
				oninput={handleContentInput}
				onpaste={handlePaste}
			>
				{content}
			</div>
		{:else}
			<div class="toggle-content">{content}</div>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="toggle-settings">
			<label class="setting-checkbox">
				<input
					type="checkbox"
					checked={defaultOpen}
					onchange={(e) =>
						props.onUpdate({
							settings: {
								...props.block.settings,
								defaultOpen: (e.target as HTMLInputElement).checked
							}
						})}
				/>
				<span>Open by default</span>
			</label>

			<label class="setting-field">
				<span>Icon style:</span>
				<select
					value={iconStyle}
					onchange={(e) =>
						props.onUpdate({
							settings: {
								...props.block.settings,
								iconStyle: (e.target as HTMLSelectElement).value as 'chevron' | 'plusminus'
							}
						})}
					aria-label="Toggle icon style"
				>
					<option value="chevron">Chevron</option>
					<option value="plusminus">Plus/Minus</option>
				</select>
			</label>
		</div>
	{/if}
</div>

<style>
	.toggle-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}

	.toggle-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		width: 100%;
		padding: 1rem 1.25rem;
		background: #f9fafb;
		border: none;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 500;
		color: #1f2937;
		text-align: left;
		transition: background 0.15s;
	}

	.toggle-header:hover {
		background: #f3f4f6;
	}

	.toggle-header:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
		z-index: 1;
	}

	.toggle-title {
		flex: 1;
		outline: none;
	}

	.toggle-icon {
		display: flex;
		color: #6b7280;
		transition: transform 0.2s ease;
		flex-shrink: 0;
	}

	.toggle-icon.rotated {
		transform: rotate(180deg);
	}

	.toggle-content-wrapper {
		animation: slideDown 0.2s ease-out;
	}

	.toggle-content-wrapper[hidden] {
		display: none;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.toggle-content {
		padding: 1.25rem;
		background: white;
		line-height: 1.7;
		color: #374151;
		outline: none;
		min-height: 80px;
		border-top: 1px solid #e5e7eb;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	.toggle-settings {
		padding: 1rem 1.25rem;
		background: #f9fafb;
		border-top: 1px solid #e5e7eb;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.setting-checkbox {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #374151;
		cursor: pointer;
	}

	.setting-checkbox input {
		cursor: pointer;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
		color: #374151;
	}

	.setting-field span {
		font-weight: 500;
	}

	.setting-field select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.setting-field select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	/* Dark Mode */
	:global(.dark) .toggle-block {
		border-color: #374151;
	}

	:global(.dark) .toggle-header {
		background: #1e293b;
		color: #f9fafb;
	}

	:global(.dark) .toggle-header:hover {
		background: #334155;
	}

	:global(.dark) .toggle-content {
		background: #111827;
		color: #e5e7eb;
		border-color: #374151;
	}

	:global(.dark) .toggle-settings {
		background: #1e293b;
		border-color: #374151;
	}

	:global(.dark) .setting-checkbox {
		color: #e5e7eb;
	}

	:global(.dark) .setting-field {
		color: #e5e7eb;
	}

	:global(.dark) .setting-field select {
		background: #1e293b;
		border-color: #475569;
		color: #e2e8f0;
	}

	/* Mobile */
	@media (max-width: 640px) {
		.toggle-header {
			padding: 0.875rem 1rem;
		}

		.toggle-content {
			padding: 1rem;
		}
	}
</style>
