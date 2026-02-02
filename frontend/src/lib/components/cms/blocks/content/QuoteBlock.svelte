<!--
/**
 * Quote Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Blockquote with optional attribution
 */
-->

<script lang="ts">
	import { IconQuote } from '$lib/icons';
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

	const isPullQuote = $derived(props.block.type === 'pullquote');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handleTextInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	function handleCiteInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ html: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}
</script>

{#if isPullQuote}
	<figure class="pullquote-block" role="figure">
		<div class="pullquote-icon" aria-hidden="true">
			<IconQuote size={32} />
		</div>
		<blockquote
			contenteditable={props.isEditing}
			class="pullquote-text editable-content"
			class:placeholder={!props.block.content.text}
			oninput={handleTextInput}
			onpaste={handlePaste}
			data-placeholder="Add a notable quote..."
			role={props.isEditing ? 'textbox' : undefined}
			aria-label={props.isEditing ? 'Quote text' : undefined}
		>
			{props.block.content.text || ''}
		</blockquote>
	</figure>
{:else}
	<blockquote class="quote-block" role="blockquote">
		<div
			contenteditable={props.isEditing}
			class="quote-text editable-content"
			class:placeholder={!props.block.content.text}
			oninput={handleTextInput}
			onpaste={handlePaste}
			data-placeholder="Write a quote..."
			role={props.isEditing ? 'textbox' : undefined}
			aria-label={props.isEditing ? 'Quote text' : undefined}
		>
			{props.block.content.text || ''}
		</div>
		{#if props.block.content.html || props.isEditing}
			<cite
				contenteditable={props.isEditing}
				class="quote-cite editable-content"
				class:placeholder={!props.block.content.html}
				oninput={handleCiteInput}
				onpaste={handlePaste}
				data-placeholder="Author name"
				role={props.isEditing ? 'textbox' : undefined}
				aria-label={props.isEditing ? 'Quote author' : undefined}
			>
				{props.block.content.html || ''}
			</cite>
		{/if}
	</blockquote>
{/if}

<style>
	/* Standard Quote */
	.quote-block {
		border-left: 4px solid #3b82f6;
		padding-left: 1.5rem;
		margin: 0;
	}

	.quote-text {
		font-size: 1.125rem;
		line-height: 1.7;
		font-style: italic;
		color: #374151;
		outline: none;
	}

	.quote-cite {
		display: block;
		margin-top: 0.75rem;
		font-size: 0.875rem;
		color: #6b7280;
		font-style: normal;
		outline: none;
	}

	.quote-cite::before {
		content: '— ';
	}

	/* Pull Quote */
	.pullquote-block {
		position: relative;
		margin: 0;
		padding: 2rem;
		text-align: center;
		border-top: 2px solid #e5e7eb;
		border-bottom: 2px solid #e5e7eb;
	}

	.pullquote-icon {
		display: flex;
		justify-content: center;
		margin-bottom: 1rem;
		color: #3b82f6;
		opacity: 0.3;
	}

	.pullquote-text {
		font-size: 1.5rem;
		font-weight: 500;
		line-height: 1.5;
		color: #1f2937;
		margin: 0;
		outline: none;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	/* Dark Mode */
	:global(.dark) .quote-block {
		border-left-color: #60a5fa;
	}

	:global(.dark) .quote-text {
		color: #d1d5db;
	}

	:global(.dark) .quote-cite {
		color: #9ca3af;
	}

	:global(.dark) .pullquote-block {
		border-color: #374151;
	}

	:global(.dark) .pullquote-text {
		color: #f1f5f9;
	}

	:global(.dark) .pullquote-icon {
		color: #60a5fa;
	}
</style>
