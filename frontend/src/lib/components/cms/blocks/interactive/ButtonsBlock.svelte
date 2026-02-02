<!--
/**
 * Buttons Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Group of styled buttons with multiple layouts
 */
-->

<script lang="ts">
	import { IconPlus, IconX, IconExternalLink } from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
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

	interface ButtonItem {
		id: string;
		text: string;
		url: string;
		style: 'primary' | 'secondary' | 'outline' | 'ghost';
		newTab: boolean;
	}

	let buttons = $derived<ButtonItem[]>(
		(props.block.content.buttonItems?.map(item => ({
			id: item.id,
			text: item.text,
			url: item.url,
			style: (item.style || 'primary') as 'primary' | 'secondary' | 'outline' | 'ghost',
			newTab: item.newTab || false
		})) || [
			{ id: 'btn_1', text: 'Learn More', url: '#', style: 'primary' as const, newTab: false }
		])
	);

	let layout = $derived((props.block.settings.buttonLayout as 'row' | 'column' | 'wrap') || 'row');
	let alignment = $derived((props.block.settings.buttonAlignment as 'left' | 'center' | 'right') || 'left');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateButton(index: number, updates: Partial<ButtonItem>): void {
		const newButtons = buttons.map((btn, i) => (i === index ? { ...btn, ...updates } : btn));
		updateContent({ buttonItems: newButtons });
	}

	function addButton(): void {
		const newButtons = [
			...buttons,
			{ id: `btn_${Date.now()}`, text: 'Button', url: '#', style: 'secondary' as const, newTab: false }
		];
		updateContent({ buttonItems: newButtons });
	}

	function removeButton(index: number): void {
		if (buttons.length > 1) {
			updateContent({ buttonItems: buttons.filter((_, i) => i !== index) });
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div class="buttons-block layout-{layout} align-{alignment}" role="group" aria-label="Action buttons">
	<div class="buttons-container">
		{#each buttons as button, index (button.id)}
			<div class="button-wrapper">
				{#if props.isEditing}
					<span
						contenteditable="true"
						class="btn btn-{button.style} editable-btn"
						role="textbox"
						oninput={(e) => updateButton(index, { text: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
					>{button.text}</span>
					{#if buttons.length > 1}
						<button type="button" class="remove-btn" onclick={() => removeButton(index)} aria-label="Remove">
							<IconX size={12} />
						</button>
					{/if}
				{:else}
					<a
						href={sanitizeURL(button.url) || '#'}
						class="btn btn-{button.style}"
						target={button.newTab ? '_blank' : undefined}
						rel={button.newTab ? 'noopener noreferrer' : undefined}
					>
						{button.text}
						{#if button.newTab}<IconExternalLink size={14} aria-hidden="true" />{/if}
					</a>
				{/if}
			</div>
		{/each}
		{#if props.isEditing}
			<button type="button" class="add-btn" onclick={addButton} aria-label="Add button">
				<IconPlus size={14} />
			</button>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="buttons-settings">
			{#each buttons as button, index (button.id)}
				<div class="button-settings">
					<span class="setting-title">Button {index + 1}</span>
					<label class="setting-field">
						<span>URL:</span>
						<input type="url" value={button.url} oninput={(e) => updateButton(index, { url: (e.target as HTMLInputElement).value })} />
					</label>
					<label class="setting-field">
						<span>Style:</span>
						<select value={button.style} onchange={(e) => updateButton(index, { style: (e.target as HTMLSelectElement).value as ButtonItem['style'] })}>
							<option value="primary">Primary</option>
							<option value="secondary">Secondary</option>
							<option value="outline">Outline</option>
							<option value="ghost">Ghost</option>
						</select>
					</label>
					<label class="setting-checkbox">
						<input type="checkbox" checked={button.newTab} onchange={(e) => updateButton(index, { newTab: (e.target as HTMLInputElement).checked })} />
						<span>New tab</span>
					</label>
				</div>
			{/each}
			<div class="layout-settings">
				<label class="setting-field">
					<span>Layout:</span>
					<select value={layout} onchange={(e) => props.onUpdate({ settings: { ...props.block.settings, buttonLayout: (e.target as HTMLSelectElement).value as 'row' | 'column' | 'wrap' } })}>
						<option value="row">Row</option>
						<option value="column">Column</option>
						<option value="wrap">Wrap</option>
					</select>
				</label>
				<label class="setting-field">
					<span>Align:</span>
					<select value={alignment} onchange={(e) => props.onUpdate({ settings: { ...props.block.settings, buttonAlignment: (e.target as HTMLSelectElement).value as 'left' | 'center' | 'right' } })}>
						<option value="left">Left</option>
						<option value="center">Center</option>
						<option value="right">Right</option>
					</select>
				</label>
			</div>
		</div>
	{/if}
</div>

<style>
	.buttons-block { width: 100%; }
	.buttons-container { display: flex; gap: 0.75rem; }
	.layout-row .buttons-container { flex-direction: row; }
	.layout-column .buttons-container { flex-direction: column; align-items: flex-start; }
	.layout-wrap .buttons-container { flex-wrap: wrap; }
	.align-left .buttons-container { justify-content: flex-start; }
	.align-center .buttons-container { justify-content: center; }
	.align-right .buttons-container { justify-content: flex-end; }
	.button-wrapper { position: relative; display: inline-flex; }

	.btn {
		display: inline-flex; align-items: center; gap: 0.5rem;
		padding: 0.75rem 1.5rem; border-radius: 8px;
		font-size: 0.9375rem; font-weight: 500;
		text-decoration: none; cursor: pointer;
		transition: all 0.15s; outline: none;
	}
	.btn-primary { background: #3b82f6; color: white; border: none; }
	.btn-primary:hover { background: #2563eb; }
	.btn-secondary { background: #e5e7eb; color: #1f2937; border: none; }
	.btn-secondary:hover { background: #d1d5db; }
	.btn-outline { background: transparent; color: #3b82f6; border: 2px solid #3b82f6; }
	.btn-outline:hover { background: #3b82f6; color: white; }
	.btn-ghost { background: transparent; color: #6b7280; border: none; }
	.btn-ghost:hover { background: #f3f4f6; color: #1f2937; }
	.btn:focus-visible { outline: 2px solid #3b82f6; outline-offset: 2px; }
	.editable-btn { min-width: 60px; }

	.remove-btn {
		position: absolute; top: -8px; right: -8px;
		width: 20px; height: 20px;
		display: flex; align-items: center; justify-content: center;
		background: #fee2e2; border: none; border-radius: 50%;
		color: #dc2626; cursor: pointer; opacity: 0; transition: opacity 0.15s;
	}
	.button-wrapper:hover .remove-btn { opacity: 1; }
	.add-btn {
		display: flex; align-items: center; justify-content: center;
		width: 40px; height: 40px;
		background: transparent; border: 2px dashed #d1d5db; border-radius: 8px;
		color: #9ca3af; cursor: pointer; transition: all 0.15s;
	}
	.add-btn:hover { border-color: #3b82f6; color: #3b82f6; }

	.buttons-settings {
		display: flex; flex-direction: column; gap: 1rem;
		margin-top: 1rem; padding: 1rem;
		background: #f9fafb; border-radius: 8px;
	}
	.button-settings {
		display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: center;
		padding-bottom: 0.75rem; border-bottom: 1px solid #e5e7eb;
	}
	.button-settings:last-of-type { border-bottom: none; padding-bottom: 0; }
	.setting-title { font-weight: 600; font-size: 0.875rem; min-width: 70px; }
	.setting-field { display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem; }
	.setting-field input, .setting-field select {
		padding: 0.375rem 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; font-size: 0.875rem;
	}
	.setting-field input[type="url"] { width: 150px; }
	.setting-checkbox { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; }
	.layout-settings { display: flex; gap: 1rem; flex-wrap: wrap; padding-top: 0.75rem; border-top: 1px solid #e5e7eb; }

	:global(.dark) .btn-primary { background: #2563eb; }
	:global(.dark) .btn-primary:hover { background: #1d4ed8; }
	:global(.dark) .btn-secondary { background: #374151; color: #f9fafb; }
	:global(.dark) .btn-secondary:hover { background: #4b5563; }
	:global(.dark) .btn-outline { color: #60a5fa; border-color: #60a5fa; }
	:global(.dark) .btn-outline:hover { background: #60a5fa; color: #0f172a; }
	:global(.dark) .btn-ghost { color: #94a3b8; }
	:global(.dark) .btn-ghost:hover { background: #1e293b; color: #f9fafb; }
	:global(.dark) .buttons-settings { background: #1e293b; }
	:global(.dark) .button-settings { border-color: #374151; }
	:global(.dark) .setting-field input, :global(.dark) .setting-field select { background: #0f172a; border-color: #475569; color: #e2e8f0; }
</style>
