<!--
/**
 * Pull Quote Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Large decorative quote with customizable styling and attribution.
 * Features decorative quotation marks, multiple alignment options,
 * border styles, and accent color customization.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { IconColorPicker } from '$lib/icons';
	import type { Block } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	interface PullQuoteContent {
		text?: string;
		attribution?: string;
		source?: string;
		alignment?: 'left' | 'center' | 'right';
		borderStyle?: 'left-bar' | 'top-bottom' | 'none';
		accentColor?: string;
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

	let props: Props = $props();

	// Generate unique IDs for ARIA labelling
	const quoteId = $derived(`pullquote-${props.blockId}`);
	const captionId = $derived(`pullquote-caption-${props.blockId}`);

	// Derived content values with defaults
	const content = $derived(props.block.content as PullQuoteContent);
	const quoteText = $derived(content.text || '');
	const attribution = $derived(content.attribution || '');
	const source = $derived(content.source || '');
	const alignment = $derived(content.alignment || 'center');
	const borderStyle = $derived(content.borderStyle || 'top-bottom');
	const accentColor = $derived(content.accentColor || '#3b82f6');

	// State for color picker visibility
	let showColorPicker = $state(false);

	// Preset color options for quick selection
	const presetColors = [
		'#3b82f6', // Blue
		'#10b981', // Green
		'#f59e0b', // Amber
		'#ef4444', // Red
		'#8b5cf6', // Purple
		'#ec4899', // Pink
		'#06b6d4', // Cyan
		'#64748b', // Slate
		'#1f2937', // Dark gray
		'#059669' // Emerald
	];

	// =========================================================================
	// Content Update Handlers
	// =========================================================================

	/**
	 * Updates block content with partial changes
	 */
	function updateContent(updates: Partial<PullQuoteContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	/**
	 * Handles quote text input changes
	 */
	function handleQuoteInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	/**
	 * Handles attribution input changes
	 */
	function handleAttributionInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ attribution: target.textContent || '' });
	}

	/**
	 * Handles source input changes
	 */
	function handleSourceInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ source: target.textContent || '' });
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
	 * Handles keyboard navigation for editable fields
	 */
	function handleKeyDown(e: KeyboardEvent, field: 'quote' | 'attribution' | 'source'): void {
		// Prevent default enter behavior to avoid line breaks in single-line fields
		if (e.key === 'Enter' && field !== 'quote') {
			e.preventDefault();
			// Move focus to next field
			const nextField = field === 'attribution' ? 'source' : null;
			if (nextField) {
				const nextElement = document.querySelector(
					`[data-field="${nextField}"][data-block="${props.blockId}"]`
				) as HTMLElement;
				nextElement?.focus();
			}
		}
	}

	// =========================================================================
	// Style Option Handlers
	// =========================================================================

	/**
	 * Changes the quote alignment
	 */
	function setAlignment(newAlignment: 'left' | 'center' | 'right'): void {
		updateContent({ alignment: newAlignment });
	}

	/**
	 * Changes the border style
	 */
	function setBorderStyle(newStyle: 'left-bar' | 'top-bottom' | 'none'): void {
		updateContent({ borderStyle: newStyle });
	}

	/**
	 * Sets accent color from preset or picker
	 */
	function setAccentColor(color: string): void {
		updateContent({ accentColor: color });
	}

	/**
	 * Handles custom color input
	 */
	function handleColorInput(e: Event): void {
		const target = e.target as HTMLInputElement;
		setAccentColor(target.value);
	}

	/**
	 * Toggles color picker visibility
	 */
	function toggleColorPicker(): void {
		showColorPicker = !showColorPicker;
	}

	/**
	 * Closes color picker when clicking outside
	 */
	function handleClickOutside(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (!target.closest('.color-picker-container')) {
			showColorPicker = false;
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<figure
	class="pullquote-block"
	class:align-left={alignment === 'left'}
	class:align-center={alignment === 'center'}
	class:align-right={alignment === 'right'}
	class:border-left-bar={borderStyle === 'left-bar'}
	class:border-top-bottom={borderStyle === 'top-bottom'}
	class:border-none={borderStyle === 'none'}
	aria-labelledby={captionId}
	style:--accent-color={accentColor}
>
	<!-- Decorative Opening Quote Mark -->
	<div class="quote-mark quote-mark-open" aria-hidden="true">
		<span class="quote-symbol">"</span>
	</div>

	<!-- Quote Text -->
	<blockquote
		id={quoteId}
		contenteditable={props.isEditing}
		class="pullquote-text editable-content"
		class:placeholder={!quoteText}
		oninput={handleQuoteInput}
		onpaste={handlePaste}
		onkeydown={(e) => handleKeyDown(e, 'quote')}
		data-placeholder="Enter a memorable quote..."
		role={props.isEditing ? 'textbox' : undefined}
		aria-label={props.isEditing ? 'Quote text' : undefined}
		aria-multiline="true"
	>
		{quoteText}
	</blockquote>

	<!-- Decorative Closing Quote Mark -->
	<div class="quote-mark quote-mark-close" aria-hidden="true">
		<span class="quote-symbol">"</span>
	</div>

	<!-- Attribution & Source -->
	{#if attribution || source || props.isEditing}
		<figcaption id={captionId} class="pullquote-caption">
			<span class="attribution-wrapper">
				<span class="em-dash" aria-hidden="true">—</span>
				<cite
					contenteditable={props.isEditing}
					class="pullquote-attribution editable-content"
					class:placeholder={!attribution}
					oninput={handleAttributionInput}
					onpaste={handlePaste}
					onkeydown={(e) => handleKeyDown(e, 'attribution')}
					data-placeholder="Author name"
					data-field="attribution"
					data-block={props.blockId}
					role={props.isEditing ? 'textbox' : undefined}
					aria-label={props.isEditing ? 'Quote author' : undefined}
				>
					{attribution}
				</cite>
				{#if source || props.isEditing}
					<span class="source-separator" aria-hidden="true">,</span>
					<span
						contenteditable={props.isEditing}
						class="pullquote-source editable-content"
						class:placeholder={!source}
						oninput={handleSourceInput}
						onpaste={handlePaste}
						onkeydown={(e) => handleKeyDown(e, 'source')}
						data-placeholder="Publication"
						data-field="source"
						data-block={props.blockId}
						role={props.isEditing ? 'textbox' : undefined}
						aria-label={props.isEditing ? 'Quote source' : undefined}
					>
						{source}
					</span>
				{/if}
			</span>
		</figcaption>
	{/if}
</figure>

<!-- Style Controls (visible when editing and selected) -->
{#if props.isEditing && props.isSelected}
	<div class="pullquote-controls" role="toolbar" aria-label="Quote styling options">
		<!-- Alignment Controls -->
		<div class="control-group" role="group" aria-label="Text alignment">
			<span class="control-label">Align:</span>
			<button
				type="button"
				class="control-btn"
				class:active={alignment === 'left'}
				onclick={() => setAlignment('left')}
				aria-pressed={alignment === 'left'}
				title="Align left"
			>
				Left
			</button>
			<button
				type="button"
				class="control-btn"
				class:active={alignment === 'center'}
				onclick={() => setAlignment('center')}
				aria-pressed={alignment === 'center'}
				title="Align center"
			>
				Center
			</button>
			<button
				type="button"
				class="control-btn"
				class:active={alignment === 'right'}
				onclick={() => setAlignment('right')}
				aria-pressed={alignment === 'right'}
				title="Align right"
			>
				Right
			</button>
		</div>

		<!-- Border Style Controls -->
		<div class="control-group" role="group" aria-label="Border style">
			<span class="control-label">Border:</span>
			<button
				type="button"
				class="control-btn"
				class:active={borderStyle === 'left-bar'}
				onclick={() => setBorderStyle('left-bar')}
				aria-pressed={borderStyle === 'left-bar'}
				title="Left bar border"
			>
				Left Bar
			</button>
			<button
				type="button"
				class="control-btn"
				class:active={borderStyle === 'top-bottom'}
				onclick={() => setBorderStyle('top-bottom')}
				aria-pressed={borderStyle === 'top-bottom'}
				title="Top and bottom borders"
			>
				Top/Bottom
			</button>
			<button
				type="button"
				class="control-btn"
				class:active={borderStyle === 'none'}
				onclick={() => setBorderStyle('none')}
				aria-pressed={borderStyle === 'none'}
				title="No border"
			>
				None
			</button>
		</div>

		<!-- Accent Color Picker -->
		<div class="control-group color-picker-container" role="group" aria-label="Accent color">
			<span class="control-label">Color:</span>
			<button
				type="button"
				class="color-trigger"
				onclick={toggleColorPicker}
				aria-expanded={showColorPicker}
				aria-haspopup="true"
				title="Choose accent color"
			>
				<span class="color-preview" style:background-color={accentColor}></span>
				<IconColorPicker size={14} aria-hidden="true" />
			</button>

			{#if showColorPicker}
				<div class="color-picker-dropdown" role="listbox" aria-label="Color presets">
					<div class="preset-colors">
						{#each presetColors as color}
							<button
								type="button"
								class="preset-color"
								class:selected={accentColor === color}
								style:background-color={color}
								onclick={() => setAccentColor(color)}
								aria-pressed={accentColor === color}
								title={color}
							>
								<span class="sr-only">Select color {color}</span>
							</button>
						{/each}
					</div>
					<div class="custom-color">
						<label for="custom-color-input">Custom:</label>
						<input
							type="color"
							id="custom-color-input"
							value={accentColor}
							oninput={handleColorInput}
						/>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	/* =========================================================================
	 * Base Styles
	 * ========================================================================= */

	.pullquote-block {
		position: relative;
		margin: 2rem 0;
		padding: 2rem 2.5rem;
		transition: all 0.2s ease;
	}

	/* =========================================================================
	 * Alignment Variants
	 * ========================================================================= */

	.align-left {
		text-align: left;
	}

	.align-center {
		text-align: center;
	}

	.align-right {
		text-align: right;
	}

	/* =========================================================================
	 * Border Style Variants
	 * ========================================================================= */

	.border-left-bar {
		border-left: 4px solid var(--accent-color, #3b82f6);
		padding-left: 2rem;
	}

	.border-top-bottom {
		border-top: 2px solid var(--accent-color, #3b82f6);
		border-bottom: 2px solid var(--accent-color, #3b82f6);
	}

	.border-none {
		border: none;
	}

	/* =========================================================================
	 * Quote Marks
	 * ========================================================================= */

	.quote-mark {
		display: flex;
		justify-content: center;
		color: var(--accent-color, #3b82f6);
		opacity: 0.25;
		line-height: 1;
		user-select: none;
	}

	.quote-symbol {
		font-family: Georgia, 'Times New Roman', serif;
		font-size: 4rem;
		font-weight: bold;
	}

	.quote-mark-open {
		margin-bottom: 0.5rem;
	}

	.align-left .quote-mark {
		justify-content: flex-start;
	}

	.align-right .quote-mark {
		justify-content: flex-end;
	}

	.quote-mark-close {
		margin-top: 0.5rem;
		transform: rotate(180deg);
	}

	/* =========================================================================
	 * Quote Text
	 * ========================================================================= */

	.pullquote-text {
		font-size: 1.5rem;
		font-weight: 500;
		line-height: 1.6;
		color: #1f2937;
		margin: 0;
		padding: 0;
		outline: none;
		font-style: italic;
		min-height: 1.6em;
	}

	.pullquote-text:focus {
		outline: none;
	}

	/* =========================================================================
	 * Attribution & Source
	 * ========================================================================= */

	.pullquote-caption {
		margin-top: 1.25rem;
		font-size: 0.9375rem;
		color: #6b7280;
	}

	.attribution-wrapper {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		flex-wrap: wrap;
	}

	.em-dash {
		font-weight: 500;
		color: var(--accent-color, #3b82f6);
	}

	.pullquote-attribution {
		font-weight: 600;
		color: #374151;
		font-style: normal;
		outline: none;
	}

	.source-separator {
		color: #9ca3af;
	}

	.pullquote-source {
		font-style: italic;
		color: #6b7280;
		outline: none;
	}

	/* =========================================================================
	 * Placeholder Styles
	 * ========================================================================= */

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
		font-style: italic;
	}

	.pullquote-attribution.placeholder:empty::before,
	.pullquote-source.placeholder:empty::before {
		font-style: italic;
		font-weight: normal;
	}

	/* =========================================================================
	 * Style Controls
	 * ========================================================================= */

	.pullquote-controls {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
	}

	.control-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.control-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.control-btn {
		padding: 0.375rem 0.625rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #64748b;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.control-btn:hover {
		color: #3b82f6;
		border-color: #93c5fd;
		background: #eff6ff;
	}

	.control-btn.active {
		color: white;
		background: #3b82f6;
		border-color: #3b82f6;
	}

	.control-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* =========================================================================
	 * Color Picker
	 * ========================================================================= */

	.color-picker-container {
		position: relative;
	}

	.color-trigger {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.color-trigger:hover {
		border-color: #93c5fd;
	}

	.color-preview {
		width: 18px;
		height: 18px;
		border-radius: 3px;
		border: 1px solid rgba(0, 0, 0, 0.1);
	}

	.color-picker-dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		margin-top: 0.5rem;
		padding: 0.75rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		box-shadow:
			0 10px 25px -5px rgba(0, 0, 0, 0.1),
			0 8px 10px -6px rgba(0, 0, 0, 0.1);
		z-index: 50;
	}

	.preset-colors {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 0.375rem;
		margin-bottom: 0.75rem;
	}

	.preset-color {
		width: 28px;
		height: 28px;
		border: 2px solid transparent;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.preset-color:hover {
		transform: scale(1.1);
	}

	.preset-color.selected {
		border-color: #1f2937;
		box-shadow:
			0 0 0 2px white,
			0 0 0 4px #1f2937;
	}

	.preset-color:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.custom-color {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid #e2e8f0;
	}

	.custom-color label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #64748b;
	}

	.custom-color input[type='color'] {
		width: 40px;
		height: 28px;
		padding: 0;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		cursor: pointer;
	}

	/* Screen reader only */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* =========================================================================
	 * Dark Mode
	 * ========================================================================= */

	:global(.dark) .pullquote-block {
		background: rgba(30, 41, 59, 0.3);
	}

	:global(.dark) .pullquote-text {
		color: #f1f5f9;
	}

	:global(.dark) .pullquote-attribution {
		color: #e2e8f0;
	}

	:global(.dark) .pullquote-source,
	:global(.dark) .pullquote-caption {
		color: #94a3b8;
	}

	:global(.dark) .source-separator {
		color: #64748b;
	}

	:global(.dark) .quote-mark {
		opacity: 0.2;
	}

	:global(.dark) .pullquote-controls {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .control-label {
		color: #94a3b8;
	}

	:global(.dark) .control-btn {
		color: #94a3b8;
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .control-btn:hover {
		color: #60a5fa;
		border-color: #3b82f6;
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .control-btn.active {
		color: white;
		background: #3b82f6;
		border-color: #3b82f6;
	}

	:global(.dark) .color-trigger {
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .color-trigger:hover {
		border-color: #3b82f6;
	}

	:global(.dark) .color-picker-dropdown {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .custom-color {
		border-color: #334155;
	}

	:global(.dark) .custom-color label {
		color: #94a3b8;
	}

	:global(.dark) .custom-color input[type='color'] {
		border-color: #334155;
	}

	:global(.dark) .editable-content.placeholder:empty::before {
		color: #64748b;
	}

	/* =========================================================================
	 * Responsive Design
	 * ========================================================================= */

	@media (max-width: 640px) {
		.pullquote-block {
			padding: 1.5rem 1rem;
		}

		.pullquote-text {
			font-size: 1.25rem;
		}

		.quote-symbol {
			font-size: 3rem;
		}

		.pullquote-controls {
			flex-direction: column;
			gap: 0.75rem;
		}

		.control-group {
			width: 100%;
			justify-content: space-between;
		}
	}
</style>
