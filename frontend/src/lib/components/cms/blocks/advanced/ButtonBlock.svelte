<!--
/**
 * Button Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Single CTA button with multiple style variants, sizes, and full-width option
 * Production-ready with accessibility, dark mode, and responsive design
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	type ButtonStyle = 'primary' | 'secondary' | 'outline' | 'ghost';
	type ButtonSize = 'small' | 'medium' | 'large';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// =========================================================================
	// Props
	// =========================================================================

	let props: Props = $props();

	// =========================================================================
	// Derived Values
	// =========================================================================

	let buttonText = $derived(props.block.content.buttonText || 'Click Here');
	let buttonUrl = $derived(props.block.content.buttonUrl || '#');
	let buttonStyle = $derived((props.block.content.buttonStyle as ButtonStyle) || 'primary');
	let buttonSize = $derived((props.block.content.buttonSize as ButtonSize) || 'medium');
	let fullWidth = $derived(props.block.settings.fullWidth || false);

	let sanitizedUrl = $derived(sanitizeURL(buttonUrl) || '#');

	// =========================================================================
	// Handlers
	// =========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function handleClick(e: MouseEvent): void {
		if (props.isEditing) {
			e.preventDefault();
		}
	}
</script>

<div
	class="button-block"
	class:full-width={fullWidth}
	role="navigation"
	aria-label="Call to action"
>
	{#if props.isEditing}
		<!-- Edit Mode: Inline Editing with contenteditable span -->
		<a
			href={sanitizedUrl}
			class="btn btn-{buttonStyle} btn-{buttonSize}"
			class:full-width={fullWidth}
			onclick={handleClick}
			role="button"
		>
			<span
				contenteditable="true"
				class="btn-text"
				oninput={(e) => updateContent({ buttonText: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{buttonText}</span>
		</a>
	{:else}
		<!-- View Mode: Actual Link -->
		<a
			href={sanitizedUrl}
			class="btn btn-{buttonStyle} btn-{buttonSize}"
			class:full-width={fullWidth}
		>
			<span class="btn-text">{buttonText}</span>
		</a>
	{/if}
</div>

<!-- Settings Panel (Edit Mode) -->
{#if props.isEditing && props.isSelected}
	<div class="button-settings">
		<div class="settings-row">
			<label class="setting-field">
				<span>URL:</span>
				<input
					type="url"
					placeholder="https://..."
					value={buttonUrl}
					oninput={(e) => updateContent({ buttonUrl: (e.target as HTMLInputElement).value })}
				/>
			</label>
		</div>

		<div class="settings-row">
			<label class="setting-field">
				<span>Style:</span>
				<select
					value={buttonStyle}
					onchange={(e) => updateContent({ buttonStyle: (e.target as HTMLSelectElement).value as ButtonStyle })}
				>
					<option value="primary">Primary</option>
					<option value="secondary">Secondary</option>
					<option value="outline">Outline</option>
					<option value="ghost">Ghost</option>
				</select>
			</label>

			<label class="setting-field">
				<span>Size:</span>
				<select
					value={buttonSize}
					onchange={(e) => updateContent({ buttonSize: (e.target as HTMLSelectElement).value as ButtonSize })}
				>
					<option value="small">Small</option>
					<option value="medium">Medium</option>
					<option value="large">Large</option>
				</select>
			</label>
		</div>

		<div class="settings-row">
			<label class="setting-checkbox">
				<input
					type="checkbox"
					checked={fullWidth}
					onchange={(e) => updateSettings({ fullWidth: (e.target as HTMLInputElement).checked })}
				/>
				<span>Full Width</span>
			</label>
		</div>
	</div>
{/if}

<style>
	/* Container */
	.button-block {
		display: flex;
		justify-content: flex-start;
	}

	.button-block.full-width {
		display: block;
	}

	/* Base Button Styles */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 8px;
		border: 2px solid transparent;
		cursor: pointer;
		transition: all 0.2s ease;
		outline: none;
		white-space: nowrap;
	}

	.btn.full-width {
		width: 100%;
	}

	.btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Size Variants */
	.btn-small {
		padding: 0.5rem 1rem;
		font-size: 0.8125rem;
		border-radius: 6px;
	}

	.btn-medium {
		padding: 0.75rem 1.5rem;
		font-size: 0.9375rem;
	}

	.btn-large {
		padding: 1rem 2rem;
		font-size: 1.0625rem;
		border-radius: 10px;
	}

	/* Primary Variant - Blue #3b82f6 */
	.btn-primary {
		background: #3b82f6;
		color: white;
		border-color: #3b82f6;
	}

	.btn-primary:hover {
		background: #2563eb;
		border-color: #2563eb;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
	}

	.btn-primary:active {
		transform: translateY(0);
	}

	/* Secondary Variant - Gray #6b7280 */
	.btn-secondary {
		background: #6b7280;
		color: white;
		border-color: #6b7280;
	}

	.btn-secondary:hover {
		background: #4b5563;
		border-color: #4b5563;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(107, 114, 128, 0.4);
	}

	.btn-secondary:active {
		transform: translateY(0);
	}

	/* Outline Variant - Transparent with blue border */
	.btn-outline {
		background: transparent;
		color: #3b82f6;
		border-color: #3b82f6;
	}

	.btn-outline:hover {
		background: #3b82f6;
		color: white;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
	}

	.btn-outline:active {
		transform: translateY(0);
	}

	/* Ghost Variant - Transparent with blue text */
	.btn-ghost {
		background: transparent;
		color: #3b82f6;
		border-color: transparent;
	}

	.btn-ghost:hover {
		background: rgba(59, 130, 246, 0.1);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
	}

	.btn-ghost:active {
		transform: translateY(0);
	}

	/* Button Text (for contenteditable) */
	.btn-text {
		outline: none;
	}

	/* Settings Panel */
	.button-settings {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: 1rem;
		padding: 1rem;
		background: #f8fafc;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
	}

	.settings-row {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		align-items: center;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #475569;
	}

	.setting-field input[type="url"] {
		flex: 1;
		min-width: 200px;
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.setting-field select {
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.setting-checkbox {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		color: #475569;
		cursor: pointer;
	}

	.setting-checkbox input {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	/* Dark Mode */
	:global(.dark) .btn-primary {
		background: #3b82f6;
		border-color: #3b82f6;
	}

	:global(.dark) .btn-primary:hover {
		background: #60a5fa;
		border-color: #60a5fa;
	}

	:global(.dark) .btn-secondary {
		background: #6b7280;
		border-color: #6b7280;
	}

	:global(.dark) .btn-secondary:hover {
		background: #9ca3af;
		border-color: #9ca3af;
	}

	:global(.dark) .btn-outline {
		color: #60a5fa;
		border-color: #60a5fa;
	}

	:global(.dark) .btn-outline:hover {
		background: #60a5fa;
		color: #0f172a;
	}

	:global(.dark) .btn-ghost {
		color: #60a5fa;
	}

	:global(.dark) .btn-ghost:hover {
		background: rgba(96, 165, 250, 0.15);
	}

	:global(.dark) .button-settings {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .setting-field,
	:global(.dark) .setting-checkbox {
		color: #94a3b8;
	}

	:global(.dark) .setting-field input,
	:global(.dark) .setting-field select {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}
</style>
