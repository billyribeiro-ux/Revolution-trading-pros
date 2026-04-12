<!--
/**
 * Quote Block Component
 * ===============================================================================
 * Blockquote with optional citation and pullquote variant
 *
 * Features:
 * - Standard quote with blue left border
 * - Pullquote variant with decorative quote icon
 * - Quote text (contenteditable)
 * - Optional citation/attribution
 * - Full dark mode support
 */
-->

<script lang="ts">
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';
	import { Icon, IconQuote } from '$lib/icons';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let { block, blockId, isSelected, isEditing, onUpdate, onError }: Props = $props();

	// Determine if this is a pullquote (based on block type)
	const isPullQuote = $derived(block.type === 'pullquote');

	// Derive text alignment from settings
	const textAlign = $derived(block.settings.textAlign || (isPullQuote ? 'center' : 'left'));

	// Derive text color from settings
	const textColor = $derived(block.settings.textColor || '');

	// Build inline styles for quote text
	const quoteStyles = $derived.by(() => {
		const styles: string[] = [];
		if (!isPullQuote && textAlign !== 'left') {
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
		onUpdate({ content: { ...block.content, ...updates } });
	}

	/**
	 * Handle quote text input
	 */
	function handleTextInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	/**
	 * Handle citation input (stored in html field for compatibility)
	 */
	function handleCiteInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ html: target.textContent || '' });
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
	 * Handle keydown - manage Enter key behavior
	 */
	function handleKeydown(e: KeyboardEvent): void {
		// Allow Shift+Enter for line breaks within the quote
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
		}
	}
</script>

{#if isPullQuote}
	<!-- Pull Quote Variant -->
	<figure class="quote-block quote-block--pullquote" class:quote-block--selected={isSelected}>
		<!-- Decorative Quote Icon -->
		<div class="quote-block__icon" aria-hidden="true">
			<Icon icon={IconQuote} size={32} />
		</div>

		<!-- Quote Text -->
		<blockquote
			contenteditable={isEditing}
			class="quote-block__pullquote-text"
			class:quote-block__pullquote-text--editing={isEditing}
			class:quote-block__pullquote-text--placeholder={!block.content.text}
			style={quoteStyles}
			oninput={handleTextInput}
			onpaste={handlePaste}
			onkeydown={handleKeydown}
			data-placeholder="Add a notable quote..."
			role={isEditing ? 'textbox' : undefined}
			aria-label={isEditing ? 'Quote text' : undefined}
		>
			{block.content.text || ''}
		</blockquote>

		<!-- Citation (optional) -->
		{#if block.content.html || isEditing}
			<figcaption
				contenteditable={isEditing}
				class="quote-block__cite quote-block__cite--pullquote"
				class:quote-block__cite--placeholder={!block.content.html}
				oninput={handleCiteInput}
				onpaste={handlePaste}
				data-placeholder="Source or author"
				role={isEditing ? 'textbox' : undefined}
				aria-label={isEditing ? 'Quote attribution' : undefined}
			>
				{block.content.html || ''}
			</figcaption>
		{/if}
	</figure>
{:else}
	<!-- Standard Quote -->
	<blockquote
		class="quote-block quote-block--standard"
		class:quote-block--selected={isSelected}
		role="blockquote"
	>
		<!-- Quote Text -->
		<div
			contenteditable={isEditing}
			class="quote-block__text"
			class:quote-block__text--editing={isEditing}
			class:quote-block__text--placeholder={!block.content.text}
			style={quoteStyles}
			oninput={handleTextInput}
			onpaste={handlePaste}
			onkeydown={handleKeydown}
			data-placeholder="Write a quote..."
			role={isEditing ? 'textbox' : undefined}
			aria-label={isEditing ? 'Quote text' : undefined}
		>
			{block.content.text || ''}
		</div>

		<!-- Citation (shown when has content or editing) -->
		{#if block.content.html || isEditing}
			<cite
				contenteditable={isEditing}
				class="quote-block__cite"
				class:quote-block__cite--placeholder={!block.content.html}
				oninput={handleCiteInput}
				onpaste={handlePaste}
				data-placeholder="Author name"
				role={isEditing ? 'textbox' : undefined}
				aria-label={isEditing ? 'Quote author' : undefined}
			>
				{block.content.html || ''}
			</cite>
		{/if}
	</blockquote>
{/if}

<style>
	/* =========================================================================
	 * Quote Block - Standard Quote Styles
	 * ========================================================================= */
	.quote-block--standard {
		position: relative;
		border-left: 4px solid #3b82f6;
		padding-left: 1.5rem;
		margin: 0;
	}

	.quote-block__text {
		font-size: 1.125rem;
		line-height: 1.7;
		font-style: italic;
		color: #374151;
		outline: none;
		min-height: 1.5em;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	.quote-block__text--editing {
		cursor: text;
	}

	.quote-block__cite {
		display: block;
		margin-top: 0.75rem;
		font-size: 0.875rem;
		color: #6b7280;
		font-style: normal;
		outline: none;
	}

	.quote-block__cite::before {
		content: '\2014\00a0'; /* em dash + nbsp */
	}

	/* =========================================================================
	 * Quote Block - Pull Quote Styles
	 * ========================================================================= */
	.quote-block--pullquote {
		position: relative;
		margin: 0;
		padding: 2rem;
		text-align: center;
		border-top: 2px solid #e5e7eb;
		border-bottom: 2px solid #e5e7eb;
	}

	.quote-block__icon {
		display: flex;
		justify-content: center;
		margin-bottom: 1rem;
		color: #3b82f6;
		opacity: 0.3;
	}

	.quote-block__pullquote-text {
		font-size: 1.5rem;
		font-weight: 500;
		line-height: 1.5;
		color: #1f2937;
		margin: 0;
		outline: none;
		min-height: 1.5em;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	.quote-block__pullquote-text--editing {
		cursor: text;
	}

	.quote-block__cite--pullquote {
		margin-top: 1rem;
		text-align: center;
	}

	.quote-block__cite--pullquote::before {
		content: '';
	}

	/* =========================================================================
	 * Quote Block - State Modifiers
	 * ========================================================================= */
	.quote-block--selected {
		background-color: rgba(59, 130, 246, 0.04);
		border-radius: 2px;
	}

	/* =========================================================================
	 * Quote Block - Placeholder Styles
	 * ========================================================================= */
	.quote-block__text--placeholder:empty::before,
	.quote-block__pullquote-text--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
		font-style: normal;
	}

	.quote-block__cite--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	/* =========================================================================
	 * Quote Block - Dark Mode (Standard)
	 * ========================================================================= */
	:global(.dark) .quote-block--standard {
		border-left-color: #60a5fa;
	}

	:global(.dark) .quote-block__text {
		color: #d1d5db;
	}

	:global(.dark) .quote-block__cite {
		color: #9ca3af;
	}

	/* =========================================================================
	 * Quote Block - Dark Mode (Pull Quote)
	 * ========================================================================= */
	:global(.dark) .quote-block--pullquote {
		border-color: #374151;
	}

	:global(.dark) .quote-block__pullquote-text {
		color: #f1f5f9;
	}

	:global(.dark) .quote-block__icon {
		color: #60a5fa;
	}

	/* =========================================================================
	 * Quote Block - Dark Mode (Shared)
	 * ========================================================================= */
	:global(.dark) .quote-block--selected {
		background-color: rgba(59, 130, 246, 0.08);
	}

	:global(.dark) .quote-block__text--placeholder:empty::before,
	:global(.dark) .quote-block__pullquote-text--placeholder:empty::before,
	:global(.dark) .quote-block__cite--placeholder:empty::before {
		color: #64748b;
	}
</style>
