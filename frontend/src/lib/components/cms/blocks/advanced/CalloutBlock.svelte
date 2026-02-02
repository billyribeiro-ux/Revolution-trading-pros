<!--
/**
 * Callout Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Highlighted information box with different types (info, success, warning, error, tip)
 * Production-ready with accessibility, dark mode, and dismissible option
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import {
		IconInfoCircle,
		IconCircleCheck,
		IconAlertTriangle,
		IconCircleX,
		IconBulb,
		IconX
	} from '$lib/icons';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	type CalloutType = 'info' | 'success' | 'warning' | 'error' | 'tip';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	type IconComponent = any;

	interface CalloutConfig {
		icon: IconComponent;
		label: string;
		ariaRole: 'note' | 'alert';
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

	const props: Props = $props();

	let isDismissed = $state(false);

	// =========================================================================
	// Derived Values
	// =========================================================================

	let calloutType = $derived(
		(props.block.content.calloutType as CalloutType) || 'info'
	);
	let title = $derived(props.block.content.calloutTitle || '');
	let content = $derived(props.block.content.calloutContent || 'Enter your callout message here...');
	let dismissible = $derived(props.block.settings.calloutDismissible || false);

	// =========================================================================
	// Configuration
	// =========================================================================

	const CALLOUT_CONFIG: Record<CalloutType, CalloutConfig> = {
		info: {
			icon: IconInfoCircle,
			label: 'Information',
			ariaRole: 'note'
		},
		success: {
			icon: IconCircleCheck,
			label: 'Success',
			ariaRole: 'note'
		},
		warning: {
			icon: IconAlertTriangle,
			label: 'Warning',
			ariaRole: 'alert'
		},
		error: {
			icon: IconCircleX,
			label: 'Error',
			ariaRole: 'alert'
		},
		tip: {
			icon: IconBulb,
			label: 'Tip',
			ariaRole: 'note'
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
		class="callout-block callout-{calloutType}"
		class:dismissed={isDismissed}
		role={currentConfig.ariaRole}
		aria-label="{currentConfig.label} callout"
	>
		<!-- Icon -->
		<div class="callout-icon" aria-hidden="true">
			<svelte:component this={currentConfig.icon} size={24} />
		</div>

		<!-- Content -->
		<div class="callout-content">
			{#if props.isEditing}
				{#if title || props.isSelected}
					<strong
						contenteditable="true"
						class="callout-title"
						data-placeholder="Add title (optional)..."
						oninput={(e) => updateContent({ calloutTitle: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
					>{title}</strong>
				{/if}
				<p
					contenteditable="true"
					class="callout-text"
					oninput={(e) => updateContent({ calloutContent: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}
				>{content}</p>
			{:else}
				{#if title}
					<strong class="callout-title">{title}</strong>
				{/if}
				<p class="callout-text">{content}</p>
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
					onchange={(e) => updateContent({ calloutType: (e.target as HTMLSelectElement).value as CalloutType })}
				>
					<option value="info">Info (Blue)</option>
					<option value="success">Success (Green)</option>
					<option value="warning">Warning (Yellow)</option>
					<option value="error">Error (Red)</option>
					<option value="tip">Tip (Purple)</option>
				</select>
			</label>

			<label class="setting-checkbox">
				<input
					type="checkbox"
					checked={dismissible}
					onchange={(e) => updateSettings({ calloutDismissible: (e.target as HTMLInputElement).checked })}
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
		border-left: 4px solid;
		position: relative;
		transition: opacity 0.2s ease, transform 0.2s ease;
	}

	.callout-block.dismissed {
		opacity: 0.5;
		pointer-events: none;
	}

	/* Type Variants - Light Mode */
	.callout-info {
		background-color: #eff6ff;
		border-left-color: #3b82f6;
		color: #1e40af;
	}

	.callout-success {
		background-color: #f0fdf4;
		border-left-color: #22c55e;
		color: #166534;
	}

	.callout-warning {
		background-color: #fefce8;
		border-left-color: #eab308;
		color: #854d0e;
	}

	.callout-error {
		background-color: #fef2f2;
		border-left-color: #ef4444;
		color: #991b1b;
	}

	.callout-tip {
		background-color: #faf5ff;
		border-left-color: #a855f7;
		color: #6b21a8;
	}

	/* Icon */
	.callout-icon {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-top: 0.125rem;
	}

	.callout-info .callout-icon { color: #3b82f6; }
	.callout-success .callout-icon { color: #22c55e; }
	.callout-warning .callout-icon { color: #eab308; }
	.callout-error .callout-icon { color: #ef4444; }
	.callout-tip .callout-icon { color: #a855f7; }

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

	.callout-text {
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
	:global(.dark) .callout-info {
		background-color: rgba(59, 130, 246, 0.15);
		color: #93c5fd;
	}

	:global(.dark) .callout-success {
		background-color: rgba(34, 197, 94, 0.15);
		color: #86efac;
	}

	:global(.dark) .callout-warning {
		background-color: rgba(234, 179, 8, 0.15);
		color: #fde047;
	}

	:global(.dark) .callout-error {
		background-color: rgba(239, 68, 68, 0.15);
		color: #fca5a5;
	}

	:global(.dark) .callout-tip {
		background-color: rgba(168, 85, 247, 0.15);
		color: #d8b4fe;
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
