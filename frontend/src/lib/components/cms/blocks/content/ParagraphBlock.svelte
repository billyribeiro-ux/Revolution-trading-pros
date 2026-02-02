<!--
/**
 * Paragraph Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Simple text paragraph with inline editing
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
</script>

<p
	contenteditable={props.isEditing}
	class="paragraph-content editable-content"
	class:placeholder={!props.block.content.text}
	oninput={handleTextInput}
	onpaste={handlePaste}
	data-placeholder="Start writing or type / for commands..."
	role={props.isEditing ? 'textbox' : undefined}
	aria-label={props.isEditing ? 'Paragraph text' : undefined}
	aria-multiline={props.isEditing ? 'true' : undefined}
>
	{props.block.content.text || ''}
</p>

<style>
	.paragraph-content {
		margin: 0;
		line-height: 1.7;
		font-size: 1rem;
		color: #1f2937;
		outline: none;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	:global(.dark) .paragraph-content {
		color: #e5e7eb;
	}

	:global(.dark) .editable-content.placeholder:empty::before {
		color: #6b7280;
	}
</style>
