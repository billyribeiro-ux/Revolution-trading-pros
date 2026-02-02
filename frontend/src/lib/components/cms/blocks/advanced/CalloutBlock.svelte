<!--
/**
 * Callout Block Component
 * ============================================================================
 * Highlighted information box with different types (info, success, warning, error)
 * Features emoji icons, dismissible option, and dark mode support
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { IconX } from '$lib/icons';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	type CalloutType = 'info' | 'success' | 'warning' | 'error';

	interface CalloutConfig {
		emoji: string;
		label: string;
		bg: string;
		border: string;
		text: string;
		darkBg: string;
		darkBorder: string;
		darkText: string;
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

	let isDismissed = $state(false);

	// =========================================================================
	// Derived Values
	// =========================================================================

	let calloutType = $derived((props.block.settings.type as CalloutType) || 'info');
	let title = $derived(props.block.content.title || '');
	let description = $derived(props.block.content.description || 'Enter your callout message here...');
	let dismissible = $derived(props.block.settings.dismissible || false);

	// =========================================================================
	// Configuration
	// =========================================================================

	const CALLOUT_CONFIG: Record<CalloutType, CalloutConfig> = {
		info: {
			emoji: '\u2139\uFE0F',
			label: 'Information',
			bg: '#eff6ff',
			border: '#3b82f6',
			text: '#1e3a8a',
			darkBg: 'rgba(59, 130, 246, 0.15)',
			darkBorder: '#3b82f6',
			darkText: '#93c5fd'
		},
		success: {
			emoji: '\u2713',
			label: 'Success',
			bg: '#f0fdf4',
			border: '#10b981',
			text: '#065f46',
			darkBg: 'rgba(16, 185, 129, 0.15)',
			darkBorder: '#10b981',
			darkText: '#6ee7b7'
		},
		warning: {
			emoji: '\u26A0\uFE0F',
			label: 'Warning',
			bg: '#fffbeb',
			border: '#f59e0b',
			text: '#92400e',
			darkBg: 'rgba(245, 158, 11, 0.15)',
			darkBorder: '#f59e0b',
			darkText: '#fcd34d'
		},
		error: {
			emoji: '\u2715',
			label: 'Error',
			bg: '#fef2f2',
			border: '#ef4444',
			text: '#991b1b',
			darkBg: 'rgba(239, 68, 68, 0.15)',
			darkBorder: '#ef4444',
			darkText: '#fca5a5'
		}
	};

	let currentConfig = $derived(CALLOUT_CONFIG[calloutType]);

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

	function handleDismiss(): void {
		isDismissed = true;
	}

	function handleKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleDismiss();
		}
	}
</script>

{#if !isDismissed || props.isEditing}
	<div
		class="callout-block"
		class:dismissed={isDismissed}
		class:callout-info={calloutType === 'info'}
		class:callout-success={calloutType === 'success'}
		class:callout-warning={calloutType === 'warning'}
		class:callout-error={calloutType === 'error'}
		role="alert"
		aria-label="{currentConfig.label} callout"
		style="
			--callout-bg: {currentConfig.bg};
			--callout-border: {currentConfig.border};
			--callout-text: {currentConfig.text};
			--callout-dark-bg: {currentConfig.darkBg};
			--callout-dark-border: {currentConfig.darkBorder};
			--callout-dark-text: {currentConfig.darkText};
		"
	>
		<!-- Emoji Icon -->
		<div class="callout-icon" aria-hidden="true">
			{currentConfig.emoji}
		</div>

		<!-- Content -->
		<div class="callout-content">
			{#if props.isEditing}
				{#if title || props.isSelected}
					<strong
						contenteditable="true"
						class="callout-title"
						data-placeholder="Add title (optional)..."
						oninput={(e) => updateContent({ title: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
					>{title}</strong>
				{/if}
				<p
					contenteditable="true"
					class="callout-description"
					oninput={(e) => updateContent({ description: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}
				>{description}</p>
			{:else}
				{#if title}
					<strong class="callout-title">{title}</strong>
				{/if}
				<p class="callout-description">{description}</p>
			{/if}
		</div>

		<!-- Dismiss Button -->
		{#if dismissible && !props.isEditing}
			<button
				type="button"
				class="callout-dismiss"
				aria-label="Dismiss callout"
				onclick={handleDismiss}
				onkeydown={handleKeyDown}
			>
				<IconX size={18} aria-hidden="true" />
			</button>
		{/if}
	</div>

	<!-- Settings Panel (Edit Mode) -->
	{#if props.isEditing && props.isSelected}
		<div class="callout-settings">
			<label class="setting-field">
				<span>Type:</span>
				<select
					value={calloutType}
					onchange={(e) => updateSettings({ type: (e.target as HTMLSelectElement).value as CalloutType })}
				>
					<option value="info">Info (Blue)</option>
					<option value="success">Success (Green)</option>
					<option value="warning">Warning (Amber)</option>
					<option value="error">Error (Red)</option>
				</select>
			</label>

			<label class="setting-checkbox">
				<input
					type="checkbox"
					checked={dismissible}
					onchange={(e) => updateSettings({ dismissible: (e.target as HTMLInputElement).checked })}
				/>
				<span>Dismissible</span>
			</label>
		</div>
	{/if}
{/if}

<style>
	/* Base Styles */
	.callout-block {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-radius: 12px;
		border-left: 4px solid var(--callout-border);
		background-color: var(--callout-bg);
		color: var(--callout-text);
		position: relative;
		transition: opacity 0.2s ease, transform 0.2s ease;
	}

	.callout-block.dismissed {
		opacity: 0.5;
		pointer-events: none;
	}

	/* Emoji Icon */
	.callout-icon {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
		line-height: 1;
		margin-top: 0.125rem;
	}

	/* Content */
	.callout-content {
		flex: 1;
		min-width: 0;
	}

	.callout-title {
		display: block;
		font-weight: 700;
		font-size: 1rem;
		margin-bottom: 0.375rem;
		outline: none;
	}

	.callout-title:empty::before {
		content: attr(data-placeholder);
		opacity: 0.5;
	}

	.callout-description {
		margin: 0;
		font-size: 0.9375rem;
		line-height: 1.6;
		outline: none;
	}

	/* Dismiss Button */
	.callout-dismiss {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border: none;
		border-radius: 6px;
		background: transparent;
		color: currentColor;
		cursor: pointer;
		opacity: 0.6;
		transition: opacity 0.15s, background-color 0.15s;
	}

	.callout-dismiss:hover {
		opacity: 1;
		background-color: rgba(0, 0, 0, 0.1);
	}

	.callout-dismiss:focus-visible {
		outline: 2px solid currentColor;
		outline-offset: 2px;
	}

	/* Settings Panel */
	.callout-settings {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
		margin-top: 1rem;
		padding: 1rem;
		background: #f8fafc;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #475569;
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
	:global(.dark) .callout-block {
		background-color: var(--callout-dark-bg);
		border-left-color: var(--callout-dark-border);
		color: var(--callout-dark-text);
	}

	:global(.dark) .callout-dismiss:hover {
		background-color: rgba(255, 255, 255, 0.1);
	}

	:global(.dark) .callout-settings {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .setting-field,
	:global(.dark) .setting-checkbox {
		color: #94a3b8;
	}

	:global(.dark) .setting-field select {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}
</style>
