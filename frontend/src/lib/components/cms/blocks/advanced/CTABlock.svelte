<!--
/**
 * CTA Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Call-to-action section with title, description, and button
 */
-->

<script lang="ts">
	import { IconExternalLink } from '$lib/icons';
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

	let title = $derived(props.block.content.ctaTitle || 'Ready to Get Started?');
	let description = $derived(props.block.content.ctaDescription || 'Join thousands of traders who are already benefiting from our platform.');
	let buttonText = $derived(props.block.content.ctaButtonText || 'Start Now');
	let buttonUrl = $derived(props.block.content.ctaButtonUrl || '#');
	let style = $derived((props.block.content.ctaStyle as 'default' | 'gradient' | 'minimal') || 'default');
	let newTab = $derived(props.block.settings.ctaNewTab || false);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<section class="cta-block style-{style}" role="region" aria-label="Call to action">
	<div class="cta-content">
		{#if props.isEditing}
			<h2
				contenteditable="true"
				class="cta-title"
				oninput={(e) => updateContent({ ctaTitle: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{title}</h2>
			<p
				contenteditable="true"
				class="cta-description"
				oninput={(e) => updateContent({ ctaDescription: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{description}</p>
		{:else}
			<h2 class="cta-title">{title}</h2>
			<p class="cta-description">{description}</p>
		{/if}

		{#if props.isEditing}
			<div class="button-edit">
				<input type="text" placeholder="Button text" value={buttonText} oninput={(e) => updateContent({ ctaButtonText: (e.target as HTMLInputElement).value })} />
				<input type="url" placeholder="Button URL" value={buttonUrl} oninput={(e) => updateContent({ ctaButtonUrl: (e.target as HTMLInputElement).value })} />
			</div>
		{:else}
			<a
				href={sanitizeURL(buttonUrl) || '#'}
				class="cta-button"
				target={newTab ? '_blank' : undefined}
				rel={newTab ? 'noopener noreferrer' : undefined}
			>
				{buttonText}
				{#if newTab}<IconExternalLink size={16} aria-hidden="true" />{/if}
			</a>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="cta-settings">
			<label class="setting-field">
				<span>Style:</span>
				<select value={style} onchange={(e) => updateContent({ ctaStyle: (e.target as HTMLSelectElement).value as 'default' | 'gradient' | 'minimal' })}>
					<option value="default">Default</option>
					<option value="gradient">Gradient</option>
					<option value="minimal">Minimal</option>
				</select>
			</label>
			<label class="setting-checkbox">
				<input type="checkbox" checked={newTab} onchange={(e) => props.onUpdate({ settings: { ...props.block.settings, ctaNewTab: (e.target as HTMLInputElement).checked } })} />
				<span>Open in new tab</span>
			</label>
		</div>
	{/if}
</section>

<style>
	.cta-block {
		padding: 3rem 2rem;
		border-radius: 16px;
		text-align: center;
	}

	.cta-block.style-default {
		background: #3b82f6;
		color: white;
	}

	.cta-block.style-gradient {
		background: linear-gradient(135deg, #8b5cf6 0%, #3b82f6 50%, #06b6d4 100%);
		color: white;
	}

	.cta-block.style-minimal {
		background: #f8fafc;
		border: 2px solid #e2e8f0;
		color: #1e293b;
	}

	.cta-content { max-width: 600px; margin: 0 auto; }

	.cta-title {
		margin: 0 0 1rem; font-size: 2rem; font-weight: 800;
		outline: none;
	}

	.cta-description {
		margin: 0 0 2rem; font-size: 1.125rem; line-height: 1.6;
		opacity: 0.9; outline: none;
	}

	.style-minimal .cta-description { color: #64748b; }

	.cta-button {
		display: inline-flex; align-items: center; gap: 0.5rem;
		padding: 1rem 2.5rem;
		font-size: 1.125rem; font-weight: 700;
		border-radius: 12px; text-decoration: none;
		transition: all 0.2s;
	}

	.style-default .cta-button,
	.style-gradient .cta-button {
		background: white;
		color: #3b82f6;
	}

	.style-default .cta-button:hover,
	.style-gradient .cta-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(0,0,0,0.2);
	}

	.style-minimal .cta-button {
		background: #3b82f6;
		color: white;
	}

	.style-minimal .cta-button:hover {
		background: #2563eb;
	}

	.button-edit {
		display: flex; gap: 0.75rem; justify-content: center; flex-wrap: wrap;
	}
	.button-edit input {
		padding: 0.75rem 1rem;
		border: 2px solid rgba(255,255,255,0.3); border-radius: 8px;
		background: rgba(255,255,255,0.1);
		color: inherit; font-size: 1rem;
	}
	.style-minimal .button-edit input {
		border-color: #d1d5db;
		background: white;
	}

	.cta-settings {
		display: flex; gap: 1.5rem; justify-content: center; flex-wrap: wrap;
		margin-top: 1.5rem; padding-top: 1.5rem;
		border-top: 1px solid rgba(255,255,255,0.2);
	}
	.style-minimal .cta-settings { border-color: #e2e8f0; }

	.setting-field { display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem; }
	.setting-field select {
		padding: 0.375rem 0.5rem; border-radius: 6px;
		border: 1px solid rgba(255,255,255,0.3);
		background: rgba(255,255,255,0.1); color: inherit;
	}
	.style-minimal .setting-field select { border-color: #d1d5db; background: white; }

	.setting-checkbox { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; }

	:global(.dark) .cta-block.style-minimal { background: #1e293b; border-color: #334155; color: #f8fafc; }
	:global(.dark) .style-minimal .cta-description { color: #94a3b8; }
	:global(.dark) .style-minimal .button-edit input { background: #0f172a; border-color: #475569; }
</style>
