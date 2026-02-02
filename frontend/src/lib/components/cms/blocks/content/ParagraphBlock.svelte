<!--
/**
 * Paragraph Block Component
 * ===============================================================================
 * Basic text paragraph with inline editing support
 *
 * Features:
 * - Contenteditable p element with placeholder
 * - Text alignment from settings (left, center, right, justify)
 * - Custom text color from settings
 * - Full dark mode support
 * - Paste handling (strips formatting)
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

	let props: Props = $props();

	// Derive text alignment from settings
	const textAlign = $derived(props.block.settings.textAlign || 'left');

	// Derive text color from settings (supports custom colors)
	const textColor = $derived(props.block.settings.textColor || '');

	// Build inline styles for custom settings
	const inlineStyles = $derived(() => {
		const styles: string[] = [];
		if (textAlign !== 'left') {
			styles.push(`text-align: ${textAlign}`);
		}
		if (textColor) {
			styles.push(`color: ${textColor}`);
		}
		return styles.length > 0 ? styles.join('; ') : undefined;
	});

	/**
	 * Update block content with partial updates
	 */
	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	/**
	 * Handle text input from contenteditable
	 */
	function handleTextInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	/**
	 * Handle paste - strip HTML formatting and insert plain text
	 */
	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	/**
	 * Handle keydown for special behaviors
	 */
	function handleKeydown(e: KeyboardEvent): void {
		// Prevent default Enter behavior in single-line mode
		// Allow Shift+Enter for line breaks
		if (e.key === 'Enter' && !e.shiftKey) {
			// Let parent handle block creation
		}
	}
</script>

<p
	contenteditable={props.isEditing}
	class="paragraph-block"
	class:paragraph-block--editing={props.isEditing}
	class:paragraph-block--selected={props.isSelected}
	class:paragraph-block--placeholder={!props.block.content.text}
	class:paragraph-block--align-center={textAlign === 'center'}
	class:paragraph-block--align-right={textAlign === 'right'}
	class:paragraph-block--align-justify={textAlign === 'justify'}
	style={inlineStyles()}
	oninput={handleTextInput}
	onpaste={handlePaste}
	onkeydown={handleKeydown}
	data-placeholder="Type / for commands..."
	role={props.isEditing ? 'textbox' : undefined}
	aria-label={props.isEditing ? 'Paragraph text' : undefined}
	aria-multiline={props.isEditing ? 'true' : undefined}
>
	{props.block.content.text || ''}
</p>

<style>
	/* =========================================================================
	 * Paragraph Block - Base Styles
	 * ========================================================================= */
	.paragraph-block {
		margin: 0;
		padding: 0.125rem 0;
		line-height: 1.7;
		font-size: 1rem;
		color: #1f2937;
		outline: none;
		min-height: 1.7em;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	/* =========================================================================
	 * Paragraph Block - Text Alignment Modifiers
	 * ========================================================================= */
	.paragraph-block--align-center {
		text-align: center;
	}

	.paragraph-block--align-right {
		text-align: right;
	}

	.paragraph-block--align-justify {
		text-align: justify;
	}

	/* =========================================================================
	 * Paragraph Block - State Modifiers
	 * ========================================================================= */
	.paragraph-block--editing {
		cursor: text;
	}

	.paragraph-block--selected {
		background-color: rgba(59, 130, 246, 0.04);
		border-radius: 2px;
	}

	/* =========================================================================
	 * Paragraph Block - Placeholder
	 * ========================================================================= */
	.paragraph-block--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
		font-style: normal;
	}

	/* =========================================================================
	 * Paragraph Block - Dark Mode
	 * ========================================================================= */
	:global(.dark) .paragraph-block {
		color: #e5e7eb;
	}

	:global(.dark) .paragraph-block--selected {
		background-color: rgba(59, 130, 246, 0.08);
	}

	:global(.dark) .paragraph-block--placeholder:empty::before {
		color: #6b7280;
	}
</style>
