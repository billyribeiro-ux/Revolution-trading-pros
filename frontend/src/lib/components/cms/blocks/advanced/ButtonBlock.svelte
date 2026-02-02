<!--
/**
 * Button Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Single CTA button with multiple variants, sizes, and icon support
 * Production-ready with accessibility, dark mode, and responsive design
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import {
		IconExternalLink,
		IconArrowRight,
		IconArrowLeft,
		IconDownload,
		IconPlay,
		IconMail,
		IconPhone,
		IconShoppingCart,
		IconRocket
	} from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'ghost';
	type ButtonSize = 'small' | 'medium' | 'large';
	type IconPosition = 'left' | 'right' | 'none';
	type IconType = 'arrow-right' | 'arrow-left' | 'external' | 'download' | 'play' | 'mail' | 'phone' | 'cart' | 'rocket' | 'none';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	type IconComponent = any;

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

	const props: Props = $props();

	// =========================================================================
	// Derived Values
	// =========================================================================

	let buttonText = $derived(props.block.content.buttonText || 'Click Here');
	let buttonUrl = $derived(props.block.content.buttonUrl || '#');
	let variant = $derived((props.block.content.buttonVariant as ButtonVariant) || 'primary');
	let size = $derived((props.block.content.buttonSize as ButtonSize) || 'medium');
	let iconType = $derived((props.block.content.buttonIcon as IconType) || 'none');
	let iconPosition = $derived((props.block.content.buttonIconPosition as IconPosition) || 'right');
	let target = $derived((props.block.settings.buttonTarget as '_blank' | '_self') || '_self');
	let fullWidth = $derived(props.block.settings.buttonFullWidth || false);
	let disabled = $derived(props.block.settings.buttonDisabled || false);

	let sanitizedUrl = $derived(sanitizeURL(buttonUrl) || '#');
	let isExternal = $derived(target === '_blank');

	// =========================================================================
	// Icon Configuration
	// =========================================================================

	const ICON_MAP: Record<IconType, IconComponent | null> = {
		'arrow-right': IconArrowRight,
		'arrow-left': IconArrowLeft,
		'external': IconExternalLink,
		'download': IconDownload,
		'play': IconPlay,
		'mail': IconMail,
		'phone': IconPhone,
		'cart': IconShoppingCart,
		'rocket': IconRocket,
		'none': null
	};

	let CurrentIcon = $derived(ICON_MAP[iconType]);
	let iconSize = $derived(size === 'small' ? 14 : size === 'large' ? 20 : 16);

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
		if (props.isEditing || disabled) {
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
		<!-- Edit Mode: Inline Editing -->
		<span
			class="btn btn-{variant} btn-{size}"
			class:full-width={fullWidth}
			class:disabled={disabled}
			role="button"
			tabindex="0"
		>
			{#if CurrentIcon && iconPosition === 'left'}
				<span class="btn-icon" aria-hidden="true">
					<svelte:component this={CurrentIcon} size={iconSize} />
				</span>
			{/if}

			<span
				contenteditable="true"
				class="btn-text"
				oninput={(e) => updateContent({ buttonText: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{buttonText}</span>

			{#if CurrentIcon && iconPosition === 'right'}
				<span class="btn-icon" aria-hidden="true">
					<svelte:component this={CurrentIcon} size={iconSize} />
				</span>
			{/if}
		</span>
	{:else}
		<!-- View Mode: Actual Link/Button -->
		<a
			href={disabled ? undefined : sanitizedUrl}
			class="btn btn-{variant} btn-{size}"
			class:full-width={fullWidth}
			class:disabled={disabled}
			target={isExternal ? '_blank' : undefined}
			rel={isExternal ? 'noopener noreferrer' : undefined}
			aria-disabled={disabled}
			onclick={handleClick}
		>
			{#if CurrentIcon && iconPosition === 'left'}
				<span class="btn-icon" aria-hidden="true">
					<svelte:component this={CurrentIcon} size={iconSize} />
				</span>
			{/if}

			<span class="btn-text">{buttonText}</span>

			{#if CurrentIcon && iconPosition === 'right'}
				<span class="btn-icon" aria-hidden="true">
					<svelte:component this={CurrentIcon} size={iconSize} />
				</span>
			{/if}

			{#if isExternal && iconType !== 'external'}
				<span class="btn-external-indicator" aria-hidden="true">
					<IconExternalLink size={iconSize - 2} />
				</span>
			{/if}
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
				<span>Variant:</span>
				<select
					value={variant}
					onchange={(e) => updateContent({ buttonVariant: (e.target as HTMLSelectElement).value as ButtonVariant })}
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
					value={size}
					onchange={(e) => updateContent({ buttonSize: (e.target as HTMLSelectElement).value as ButtonSize })}
				>
					<option value="small">Small</option>
					<option value="medium">Medium</option>
					<option value="large">Large</option>
				</select>
			</label>
		</div>

		<div class="settings-row">
			<label class="setting-field">
				<span>Icon:</span>
				<select
					value={iconType}
					onchange={(e) => updateContent({ buttonIcon: (e.target as HTMLSelectElement).value as IconType })}
				>
					<option value="none">None</option>
					<option value="arrow-right">Arrow Right</option>
					<option value="arrow-left">Arrow Left</option>
					<option value="external">External Link</option>
					<option value="download">Download</option>
					<option value="play">Play</option>
					<option value="mail">Email</option>
					<option value="phone">Phone</option>
					<option value="cart">Cart</option>
					<option value="rocket">Rocket</option>
				</select>
			</label>

			{#if iconType !== 'none'}
				<label class="setting-field">
					<span>Icon Position:</span>
					<select
						value={iconPosition}
						onchange={(e) => updateContent({ buttonIconPosition: (e.target as HTMLSelectElement).value as IconPosition })}
					>
						<option value="left">Left</option>
						<option value="right">Right</option>
					</select>
				</label>
			{/if}
		</div>

		<div class="settings-row">
			<label class="setting-field">
				<span>Target:</span>
				<select
					value={target}
					onchange={(e) => updateSettings({ buttonTarget: (e.target as HTMLSelectElement).value as '_blank' | '_self' })}
				>
					<option value="_self">Same Window</option>
					<option value="_blank">New Tab</option>
				</select>
			</label>

			<label class="setting-checkbox">
				<input
					type="checkbox"
					checked={fullWidth}
					onchange={(e) => updateSettings({ buttonFullWidth: (e.target as HTMLInputElement).checked })}
				/>
				<span>Full Width</span>
			</label>

			<label class="setting-checkbox">
				<input
					type="checkbox"
					checked={disabled}
					onchange={(e) => updateSettings({ buttonDisabled: (e.target as HTMLInputElement).checked })}
				/>
				<span>Disabled</span>
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

	/* Primary Variant */
	.btn-primary {
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: white;
		border-color: transparent;
	}

	.btn-primary:hover:not(.disabled) {
		background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
	}

	.btn-primary:active:not(.disabled) {
		transform: translateY(0);
	}

	/* Secondary Variant */
	.btn-secondary {
		background: #1e293b;
		color: white;
		border-color: transparent;
	}

	.btn-secondary:hover:not(.disabled) {
		background: #334155;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(30, 41, 59, 0.4);
	}

	/* Outline Variant */
	.btn-outline {
		background: transparent;
		color: #3b82f6;
		border-color: #3b82f6;
	}

	.btn-outline:hover:not(.disabled) {
		background: #3b82f6;
		color: white;
	}

	/* Ghost Variant */
	.btn-ghost {
		background: transparent;
		color: #3b82f6;
		border-color: transparent;
	}

	.btn-ghost:hover:not(.disabled) {
		background: rgba(59, 130, 246, 0.1);
	}

	/* Disabled State */
	.btn.disabled {
		opacity: 0.5;
		cursor: not-allowed;
		pointer-events: none;
	}

	/* Icon */
	.btn-icon {
		display: inline-flex;
		align-items: center;
		flex-shrink: 0;
	}

	.btn-external-indicator {
		display: inline-flex;
		align-items: center;
		opacity: 0.7;
		margin-left: 0.25rem;
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
	:global(.dark) .btn-secondary {
		background: #475569;
	}

	:global(.dark) .btn-secondary:hover:not(.disabled) {
		background: #64748b;
	}

	:global(.dark) .btn-outline {
		color: #60a5fa;
		border-color: #60a5fa;
	}

	:global(.dark) .btn-outline:hover:not(.disabled) {
		background: #60a5fa;
		color: #0f172a;
	}

	:global(.dark) .btn-ghost {
		color: #60a5fa;
	}

	:global(.dark) .btn-ghost:hover:not(.disabled) {
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
