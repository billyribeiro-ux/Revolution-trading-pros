<!--
/**
 * Heading Block Component
 * ===============================================================================
 * Dynamic heading levels (H1-H6) with inline editing
 *
 * Features:
 * - Configurable heading level (1-6) from settings
 * - Auto-generated anchor slug from text content
 * - Level selector toolbar when editing and selected
 * - Contenteditable with placeholder
 * - Full dark mode support
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

	// Derive the heading level from settings (default to H2)
	const level = $derived((props.block.settings.level || 2) as 1 | 2 | 3 | 4 | 5 | 6);

	// Derive text alignment from settings
	const textAlign = $derived(props.block.settings.textAlign || 'left');

	// Derive text color from settings
	const textColor = $derived(props.block.settings.textColor || '');

	/**
	 * Generate a URL-safe anchor slug from text
	 * - Converts to lowercase
	 * - Replaces spaces with hyphens
	 * - Removes non-alphanumeric characters (except hyphens)
	 * - Removes consecutive hyphens
	 */
	const anchorSlug = $derived.by(() => {
		const text = props.block.content.text || '';
		if (!text.trim()) return '';

		return text
			.toLowerCase()
			.trim()
			.replace(/\s+/g, '-') // Replace spaces with hyphens
			.replace(/[^\w\-]+/g, '') // Remove non-word chars (except hyphens)
			.replace(/\-\-+/g, '-') // Replace multiple hyphens with single
			.replace(/^-+/, '') // Trim hyphens from start
			.replace(/-+$/, ''); // Trim hyphens from end
	});

	// Use custom anchor from settings if provided, otherwise use auto-generated
	const anchor = $derived(props.block.settings.anchor || anchorSlug);

	// Build inline styles for custom settings
	const inlineStyles = $derived.by(() => {
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
	 * Set the heading level
	 */
	function setLevel(newLevel: 1 | 2 | 3 | 4 | 5 | 6): void {
		props.onUpdate({
			settings: { ...props.block.settings, level: newLevel }
		});
	}

	/**
	 * Handle keydown - prevent Enter from creating new line
	 */
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === 'Enter') {
			e.preventDefault();
			// Let parent handle block creation
		}
	}

	// Available heading levels for the toolbar
	const headingLevels: Array<1 | 2 | 3 | 4 | 5 | 6> = [1, 2, 3, 4, 5, 6];
</script>

<div class="heading-block">
	<!-- Level Selector Toolbar (shown when editing and selected) -->
	{#if props.isEditing && props.isSelected}
		<div class="heading-block__toolbar" role="toolbar" aria-label="Heading level selector">
			{#each headingLevels as lvl}
				<button
					type="button"
					class="heading-block__level-btn"
					class:heading-block__level-btn--active={level === lvl}
					onclick={() => setLevel(lvl)}
					title="Heading {lvl}"
					aria-label="Heading level {lvl}"
					aria-pressed={level === lvl}
				>
					H{lvl}
				</button>
			{/each}
		</div>
	{/if}

	<!-- Dynamic Heading Element -->
	<svelte:element
		this={`h${level}`}
		contenteditable={props.isEditing}
		class="heading-block__content heading-block__content--level-{level}"
		class:heading-block__content--editing={props.isEditing}
		class:heading-block__content--selected={props.isSelected}
		class:heading-block__content--placeholder={!props.block.content.text}
		style={inlineStyles}
		oninput={handleTextInput}
		onpaste={handlePaste}
		onkeydown={handleKeydown}
		data-placeholder="Heading {level}"
		id={anchor || undefined}
		role={props.isEditing ? 'textbox' : 'heading'}
		aria-label={props.isEditing ? `Heading ${level} text` : undefined}
		aria-level={level}
	>
		{props.block.content.text || ''}
	</svelte:element>
</div>

<style>
	/* =========================================================================
	 * Heading Block - Container
	 * ========================================================================= */
	.heading-block {
		position: relative;
	}

	/* =========================================================================
	 * Heading Block - Level Selector Toolbar
	 * ========================================================================= */
	.heading-block__toolbar {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
		padding: 0.375rem;
		background: #f5f5f5;
		border-radius: 8px;
		width: fit-content;
	}

	.heading-block__level-btn {
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
		color: #666666;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.heading-block__level-btn:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.heading-block__level-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.heading-block__level-btn--active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.heading-block__level-btn--active:hover {
		background: #2563eb;
		border-color: #2563eb;
		color: white;
	}

	/* =========================================================================
	 * Heading Block - Content Base Styles
	 * ========================================================================= */
	.heading-block__content {
		margin: 0;
		font-weight: 600;
		color: #1f2937;
		outline: none;
		line-height: 1.3;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	/* =========================================================================
	 * Heading Block - Level-Specific Styles
	 * ========================================================================= */
	.heading-block__content--level-1 {
		font-size: 2.25rem;
		font-weight: 700;
		letter-spacing: -0.025em;
	}

	.heading-block__content--level-2 {
		font-size: 1.875rem;
		font-weight: 600;
		letter-spacing: -0.02em;
	}

	.heading-block__content--level-3 {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.heading-block__content--level-4 {
		font-size: 1.25rem;
		font-weight: 600;
	}

	.heading-block__content--level-5 {
		font-size: 1.125rem;
		font-weight: 600;
	}

	.heading-block__content--level-6 {
		font-size: 1rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* =========================================================================
	 * Heading Block - State Modifiers
	 * ========================================================================= */
	.heading-block__content--editing {
		cursor: text;
	}

	.heading-block__content--selected {
		background-color: rgba(59, 130, 246, 0.04);
		border-radius: 2px;
	}

	/* =========================================================================
	 * Heading Block - Placeholder
	 * ========================================================================= */
	.heading-block__content--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
		font-weight: 400;
	}

	/* =========================================================================
	 * Heading Block - Dark Mode
	 * ========================================================================= */
	:global(.dark) .heading-block__toolbar {
		background: #1e293b;
	}

	:global(.dark) .heading-block__level-btn {
		background: #334155;
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .heading-block__level-btn:hover {
		background: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .heading-block__level-btn--active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	:global(.dark) .heading-block__level-btn--active:hover {
		background: #2563eb;
		border-color: #2563eb;
	}

	:global(.dark) .heading-block__content {
		color: #f1f5f9;
	}

	:global(.dark) .heading-block__content--selected {
		background-color: rgba(59, 130, 246, 0.08);
	}

	:global(.dark) .heading-block__content--placeholder:empty::before {
		color: #64748b;
	}
</style>
