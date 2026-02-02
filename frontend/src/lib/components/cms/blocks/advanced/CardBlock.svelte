<!--
/**
 * Card Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Image card with title, description, and CTA button
 */
-->

<script lang="ts">
	import { IconPhoto, IconExternalLink } from '$lib/icons';
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

	let props: Props = $props();

	let imageUrl = $derived(props.block.content.cardImage || '');
	let title = $derived(props.block.content.cardTitle || 'Card Title');
	let description = $derived(props.block.content.cardDescription || '');
	let buttonText = $derived(props.block.content.cardButtonText || 'Learn More');
	let buttonUrl = $derived(props.block.content.cardButtonUrl || '#');
	let newTab = $derived(props.block.settings.cardNewTab || false);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div class="card-block" role="article">
	<div class="card-image">
		{#if imageUrl}
			<img src={sanitizeURL(imageUrl)} alt={title} loading="lazy" />
		{:else}
			<div class="image-placeholder">
				<IconPhoto size={32} aria-hidden="true" />
			</div>
		{/if}
		{#if props.isEditing}
			<input
				type="url"
				class="image-input"
				placeholder="Image URL..."
				value={imageUrl}
				oninput={(e) => updateContent({ cardImage: (e.target as HTMLInputElement).value })}
			/>
		{/if}
	</div>

	<div class="card-content">
		{#if props.isEditing}
			<h3
				contenteditable="true"
				class="card-title"
				oninput={(e) => updateContent({ cardTitle: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{title}</h3>
			<p
				contenteditable="true"
				class="card-description"
				data-placeholder="Add a description..."
				oninput={(e) => updateContent({ cardDescription: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{description}</p>
		{:else}
			<h3 class="card-title">{title}</h3>
			{#if description}
				<p class="card-description">{description}</p>
			{/if}
		{/if}

		{#if props.isEditing}
			<div class="button-edit">
				<input type="text" placeholder="Button text" value={buttonText} oninput={(e) => updateContent({ cardButtonText: (e.target as HTMLInputElement).value })} />
				<input type="url" placeholder="Button URL" value={buttonUrl} oninput={(e) => updateContent({ cardButtonUrl: (e.target as HTMLInputElement).value })} />
				<label class="new-tab-check">
					<input type="checkbox" checked={newTab} onchange={(e) => props.onUpdate({ settings: { ...props.block.settings, cardNewTab: (e.target as HTMLInputElement).checked } })} />
					New tab
				</label>
			</div>
		{:else}
			<a
				href={sanitizeURL(buttonUrl) || '#'}
				class="card-button"
				target={newTab ? '_blank' : undefined}
				rel={newTab ? 'noopener noreferrer' : undefined}
			>
				{buttonText}
				{#if newTab}<IconExternalLink size={14} aria-hidden="true" />{/if}
			</a>
		{/if}
	</div>
</div>

<style>
	.card-block {
		border: 1px solid #e5e7eb; border-radius: 16px; overflow: hidden;
		background: white; transition: box-shadow 0.2s;
	}
	.card-block:hover { box-shadow: 0 8px 24px rgba(0,0,0,0.08); }

	.card-image { position: relative; aspect-ratio: 16/9; background: #f3f4f6; overflow: hidden; }
	.card-image img { width: 100%; height: 100%; object-fit: cover; }
	.image-placeholder { display: flex; align-items: center; justify-content: center; height: 100%; color: #9ca3af; }
	.image-input {
		position: absolute; bottom: 0; left: 0; right: 0;
		padding: 0.5rem; border: none; background: rgba(0,0,0,0.7);
		color: white; font-size: 0.8125rem;
	}
	.image-input::placeholder { color: rgba(255,255,255,0.6); }

	.card-content { padding: 1.5rem; }
	.card-title {
		margin: 0 0 0.75rem; font-size: 1.25rem; font-weight: 700; color: #0f172a;
		outline: none;
	}
	.card-description {
		margin: 0 0 1.25rem; font-size: 0.9375rem; line-height: 1.6; color: #64748b;
		outline: none;
	}
	.card-description:empty::before {
		content: attr(data-placeholder); color: #9ca3af;
	}

	.card-button {
		display: inline-flex; align-items: center; gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6; border-radius: 8px;
		color: white; font-weight: 600; font-size: 0.9375rem;
		text-decoration: none; transition: background 0.15s;
	}
	.card-button:hover { background: #2563eb; }

	.button-edit { display: flex; flex-direction: column; gap: 0.5rem; }
	.button-edit input {
		padding: 0.5rem 0.75rem; border: 1px solid #d1d5db; border-radius: 6px; font-size: 0.875rem;
	}
	.new-tab-check { display: flex; align-items: center; gap: 0.375rem; font-size: 0.8125rem; color: #6b7280; }

	:global(.dark) .card-block { background: #1e293b; border-color: #334155; }
	:global(.dark) .card-image { background: #0f172a; }
	:global(.dark) .image-placeholder { color: #475569; }
	:global(.dark) .card-title { color: #f8fafc; }
	:global(.dark) .card-description { color: #94a3b8; }
	:global(.dark) .button-edit input { background: #0f172a; border-color: #475569; color: #e2e8f0; }
</style>
