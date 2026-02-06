<!--
/**
 * CTA Block Component (Advanced)
 * ═══════════════════════════════════════════════════════════════════════════
 * Call-to-action section with heading, description, and dual button support
 * Features primary and optional secondary buttons with customizable styling
 *
 * @version 1.0.0
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

	type Alignment = 'left' | 'center' | 'right';

	interface ButtonConfig {
		text: string;
		url: string;
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

	// =========================================================================
	// Derived Values
	// =========================================================================

	let ctaHeading = $derived(props.block.content.ctaHeading || 'Ready to Transform Your Trading?');
	let ctaDescription = $derived(
		props.block.content.ctaDescription ||
			'Join thousands of successful traders who have elevated their game with our proven strategies and tools.'
	);
	let ctaPrimaryButton = $derived<ButtonConfig>(
		(props.block.content.ctaPrimaryButton as ButtonConfig) || {
			text: 'Get Started',
			url: '#'
		}
	);
	let ctaSecondaryButton = $derived<ButtonConfig | null>(
		(props.block.content.ctaSecondaryButton as ButtonConfig) || null
	);

	// Settings
	let alignment = $derived<Alignment>((props.block.settings.alignment as Alignment) || 'center');
	let backgroundColor = $derived(props.block.settings.backgroundColor || '#3b82f6');

	// Computed styles
	let containerStyle = $derived(`background-color: ${backgroundColor};`);
	let textAlignClass = $derived(`align-${alignment}`);

	// =========================================================================
	// Handlers
	// =========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function updatePrimaryButton(updates: Partial<ButtonConfig>): void {
		const current = ctaPrimaryButton;
		updateContent({
			ctaPrimaryButton: { ...current, ...updates }
		});
	}

	function updateSecondaryButton(updates: Partial<ButtonConfig>): void {
		const current = ctaSecondaryButton || { text: '', url: '' };
		updateContent({
			ctaSecondaryButton: { ...current, ...updates }
		});
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function handleButtonClick(e: MouseEvent, _url: string): void {
		if (props.isEditing) {
			e.preventDefault();
		}
	}
</script>

<section class="cta-block {textAlignClass}" style={containerStyle} aria-label="Call to action">
	<div class="cta-content">
		<!-- Heading -->
		{#if props.isEditing}
			<h2
				contenteditable="true"
				class="cta-heading"
				oninput={(e) => updateContent({ ctaHeading: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>
				{ctaHeading}
			</h2>
		{:else}
			<h2 class="cta-heading">{ctaHeading}</h2>
		{/if}

		<!-- Description -->
		{#if props.isEditing}
			<p
				contenteditable="true"
				class="cta-description"
				oninput={(e) =>
					updateContent({ ctaDescription: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>
				{ctaDescription}
			</p>
		{:else}
			<p class="cta-description">{ctaDescription}</p>
		{/if}

		<!-- Buttons -->
		<div class="cta-buttons">
			<!-- Primary Button -->
			{#if !props.isEditing}
				<a
					href={sanitizeURL(ctaPrimaryButton.url) || '#'}
					class="cta-btn cta-btn-primary"
					onclick={(e) => handleButtonClick(e, ctaPrimaryButton.url)}
				>
					{ctaPrimaryButton.text}
				</a>
			{:else}
				<span class="cta-btn cta-btn-primary cta-btn-preview">
					{ctaPrimaryButton.text || 'Primary Button'}
				</span>
			{/if}

			<!-- Secondary Button -->
			{#if ctaSecondaryButton && ctaSecondaryButton.text}
				{#if !props.isEditing}
					<a
						href={sanitizeURL(ctaSecondaryButton.url) || '#'}
						class="cta-btn cta-btn-secondary"
						onclick={(e) => handleButtonClick(e, ctaSecondaryButton?.url || '#')}
					>
						{ctaSecondaryButton.text}
					</a>
				{:else}
					<span class="cta-btn cta-btn-secondary cta-btn-preview">
						{ctaSecondaryButton.text}
					</span>
				{/if}
			{/if}
		</div>
	</div>

	<!-- Settings Panel (Edit Mode) -->
	{#if props.isEditing && props.isSelected}
		<div class="cta-settings">
			<div class="settings-section">
				<h4 class="settings-title">Primary Button</h4>
				<div class="settings-row">
					<label class="setting-field">
						<span>Text:</span>
						<input
							type="text"
							placeholder="Button text"
							value={ctaPrimaryButton.text}
							oninput={(e) => updatePrimaryButton({ text: (e.target as HTMLInputElement).value })}
						/>
					</label>
					<label class="setting-field">
						<span>URL:</span>
						<input
							type="url"
							placeholder="https://..."
							value={ctaPrimaryButton.url}
							oninput={(e) => updatePrimaryButton({ url: (e.target as HTMLInputElement).value })}
						/>
					</label>
				</div>
			</div>

			<div class="settings-section">
				<h4 class="settings-title">Secondary Button (Optional)</h4>
				<div class="settings-row">
					<label class="setting-field">
						<span>Text:</span>
						<input
							type="text"
							placeholder="Leave empty to hide"
							value={ctaSecondaryButton?.text || ''}
							oninput={(e) => updateSecondaryButton({ text: (e.target as HTMLInputElement).value })}
						/>
					</label>
					<label class="setting-field">
						<span>URL:</span>
						<input
							type="url"
							placeholder="https://..."
							value={ctaSecondaryButton?.url || ''}
							oninput={(e) => updateSecondaryButton({ url: (e.target as HTMLInputElement).value })}
						/>
					</label>
				</div>
			</div>

			<div class="settings-section">
				<h4 class="settings-title">Layout</h4>
				<div class="settings-row">
					<label class="setting-field">
						<span>Alignment:</span>
						<select
							value={alignment}
							onchange={(e) =>
								updateSettings({ alignment: (e.target as HTMLSelectElement).value as Alignment })}
						>
							<option value="left">Left</option>
							<option value="center">Center</option>
							<option value="right">Right</option>
						</select>
					</label>

					<label class="setting-field">
						<span>Background:</span>
						<input
							type="color"
							value={backgroundColor}
							oninput={(e) =>
								updateSettings({ backgroundColor: (e.target as HTMLInputElement).value })}
							class="color-picker"
						/>
						<input
							type="text"
							value={backgroundColor}
							placeholder="#3b82f6"
							oninput={(e) =>
								updateSettings({ backgroundColor: (e.target as HTMLInputElement).value })}
							class="color-input"
						/>
					</label>
				</div>
			</div>
		</div>
	{/if}
</section>

<style>
	/* =========================================================================
	   Base Styles
	   ========================================================================= */

	.cta-block {
		padding: 4rem 2rem;
		border-radius: 16px;
		color: white;
		position: relative;
	}

	.cta-content {
		max-width: 720px;
		margin: 0 auto;
	}

	/* Alignment Variants */
	.cta-block.align-left .cta-content {
		margin-left: 0;
		text-align: left;
	}

	.cta-block.align-center .cta-content {
		text-align: center;
	}

	.cta-block.align-right .cta-content {
		margin-right: 0;
		margin-left: auto;
		text-align: right;
	}

	/* =========================================================================
	   Typography
	   ========================================================================= */

	.cta-heading {
		margin: 0 0 1.25rem;
		font-size: 2.25rem;
		font-weight: 800;
		line-height: 1.2;
		outline: none;
	}

	.cta-description {
		margin: 0 0 2.5rem;
		font-size: 1.125rem;
		line-height: 1.7;
		opacity: 0.9;
		outline: none;
	}

	/* =========================================================================
	   Buttons Container
	   ========================================================================= */

	.cta-buttons {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.align-left .cta-buttons {
		justify-content: flex-start;
	}

	.align-center .cta-buttons {
		justify-content: center;
	}

	.align-right .cta-buttons {
		justify-content: flex-end;
	}

	/* =========================================================================
	   Button Base Styles
	   ========================================================================= */

	.cta-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 1rem 2rem;
		font-size: 1.0625rem;
		font-weight: 700;
		text-decoration: none;
		border-radius: 12px;
		cursor: pointer;
		transition:
			transform 0.2s ease,
			box-shadow 0.2s ease,
			background-color 0.2s ease,
			border-color 0.2s ease;
		white-space: nowrap;
	}

	/* Primary Button - Solid white background, dark text */
	.cta-btn-primary {
		background-color: white;
		color: #1e293b;
		border: 2px solid white;
	}

	.cta-btn-primary:hover {
		transform: translateY(-3px);
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.25);
	}

	.cta-btn-primary:active {
		transform: translateY(-1px);
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
	}

	/* Secondary Button - Transparent with white border */
	.cta-btn-secondary {
		background-color: transparent;
		color: white;
		border: 2px solid rgba(255, 255, 255, 0.7);
	}

	.cta-btn-secondary:hover {
		transform: translateY(-3px);
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
		border-color: white;
		background-color: rgba(255, 255, 255, 0.1);
	}

	.cta-btn-secondary:active {
		transform: translateY(-1px);
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.15);
	}

	/* Preview state in edit mode */
	.cta-btn-preview {
		cursor: default;
	}

	.cta-btn-preview:hover {
		transform: none;
		box-shadow: none;
	}

	/* Focus styles for accessibility */
	.cta-btn:focus-visible {
		outline: 2px solid white;
		outline-offset: 4px;
	}

	/* =========================================================================
	   Settings Panel
	   ========================================================================= */

	.cta-settings {
		margin-top: 2rem;
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.settings-section {
		margin-bottom: 1.25rem;
	}

	.settings-section:last-child {
		margin-bottom: 0;
	}

	.settings-title {
		margin: 0 0 0.75rem;
		font-size: 0.8125rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		opacity: 0.85;
	}

	.settings-row {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		flex: 1;
		min-width: 200px;
	}

	.setting-field span {
		flex-shrink: 0;
		opacity: 0.9;
	}

	.setting-field input[type='text'],
	.setting-field input[type='url'] {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.1);
		color: white;
		font-size: 0.875rem;
	}

	.setting-field input[type='text']::placeholder,
	.setting-field input[type='url']::placeholder {
		color: rgba(255, 255, 255, 0.5);
	}

	.setting-field input[type='text']:focus,
	.setting-field input[type='url']:focus {
		outline: none;
		border-color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.15);
	}

	.setting-field select {
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.1);
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.setting-field select:focus {
		outline: none;
		border-color: rgba(255, 255, 255, 0.6);
	}

	.setting-field select option {
		background: #1e293b;
		color: white;
	}

	/* Color Picker */
	.color-picker {
		width: 40px;
		height: 34px;
		padding: 2px;
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 6px;
		cursor: pointer;
		background: transparent;
	}

	.color-picker::-webkit-color-swatch-wrapper {
		padding: 0;
	}

	.color-picker::-webkit-color-swatch {
		border: none;
		border-radius: 4px;
	}

	.color-input {
		width: 90px !important;
		flex: 0 0 auto !important;
		font-family: monospace;
	}

	/* =========================================================================
	   Responsive Design
	   ========================================================================= */

	@media (max-width: 640px) {
		.cta-block {
			padding: 3rem 1.5rem;
		}

		.cta-heading {
			font-size: 1.75rem;
		}

		.cta-description {
			font-size: 1rem;
			margin-bottom: 2rem;
		}

		/* Stack buttons vertically on mobile */
		.cta-buttons {
			flex-direction: column;
			align-items: stretch;
		}

		.align-center .cta-buttons,
		.align-left .cta-buttons,
		.align-right .cta-buttons {
			justify-content: center;
		}

		.cta-btn {
			width: 100%;
			text-align: center;
		}

		/* Settings panel adjustments */
		.settings-row {
			flex-direction: column;
		}

		.setting-field {
			min-width: 100%;
		}
	}

	/* =========================================================================
	   Dark Mode Adjustments
	   ========================================================================= */

	:global(.dark) .cta-settings {
		background: rgba(0, 0, 0, 0.3);
		border-color: rgba(255, 255, 255, 0.15);
	}

	:global(.dark) .setting-field input,
	:global(.dark) .setting-field select {
		background: rgba(0, 0, 0, 0.3);
		border-color: rgba(255, 255, 255, 0.2);
	}

	:global(.dark) .setting-field input:focus,
	:global(.dark) .setting-field select:focus {
		background: rgba(0, 0, 0, 0.4);
		border-color: rgba(255, 255, 255, 0.4);
	}
</style>
