<!--
/**
 * Toggle Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Collapsible content section with toggle header
 */
-->

<script lang="ts">
	import { IconChevronDown } from '$lib/icons';
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

	const props: Props = $props();
	const stateManager = getBlockStateManager();

	let defaultOpen = $derived(props.block.settings.defaultOpen || false);
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

		<span class="toggle-icon" class:rotated={isOpen} aria-hidden="true">
			<IconChevronDown size={20} />
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
