<!--
/**
 * Heading Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Configurable heading levels (H1-H6)
 */
-->

<script lang="ts">
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

	const level = $derived((props.block.settings.level || 2) as 1 | 2 | 3 | 4 | 5 | 6);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handleTextInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function setLevel(newLevel: 1 | 2 | 3 | 4 | 5 | 6): void {
		props.onUpdate({
			settings: { ...props.block.settings, level: newLevel }
		});
	}
</script>

<div class="heading-block-wrapper">
	{#if props.isEditing && props.isSelected}
		<div class="heading-level-selector" role="toolbar" aria-label="Heading level">
			{#each [1, 2, 3, 4, 5, 6] as lvl}
				<button
					type="button"
					class="level-btn"
					class:active={level === lvl}
					onclick={() => setLevel(lvl as 1 | 2 | 3 | 4 | 5 | 6)}
					title="Heading {lvl}"
					aria-label="Heading {lvl}"
					aria-pressed={level === lvl}
				>
					H{lvl}
				</button>
			{/each}
		</div>
	{/if}

	<svelte:element
		this={`h${level}`}
		contenteditable={props.isEditing}
		class="heading-content heading-{level} editable-content"
		class:placeholder={!props.block.content.text}
		oninput={handleTextInput}
		onpaste={handlePaste}
		data-placeholder="Heading {level}"
		id={props.block.settings.anchor || undefined}
		role={props.isEditing ? 'textbox' : undefined}
		aria-label={props.isEditing ? `Heading ${level} text` : undefined}
		aria-level={level}
	>
		{props.block.content.text || ''}
	</svelte:element>
</div>

<style>
	.heading-block-wrapper {
		position: relative;
	}

	.heading-level-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
		padding: 0.375rem;
		background: #f5f5f5;
		border-radius: 8px;
		width: fit-content;
	}

	.level-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 28px;
		padding: 0 0.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.level-btn:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.level-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.level-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.heading-content {
		margin: 0;
		font-weight: 600;
		color: #1f2937;
		outline: none;
	}

	.heading-1 {
		font-size: 2.25rem;
		font-weight: 700;
	}
	.heading-2 {
		font-size: 1.875rem;
	}
	.heading-3 {
		font-size: 1.5rem;
	}
	.heading-4 {
		font-size: 1.25rem;
	}
	.heading-5 {
		font-size: 1.125rem;
	}
	.heading-6 {
		font-size: 1rem;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	:global(.dark) .heading-level-selector {
		background: #1e293b;
	}

	:global(.dark) .level-btn {
		background: #334155;
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .level-btn:hover {
		background: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .level-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	:global(.dark) .heading-content {
		color: #f1f5f9;
	}
</style>
