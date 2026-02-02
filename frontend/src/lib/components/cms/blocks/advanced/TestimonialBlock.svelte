<!--
/**
 * Testimonial Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Quote with author name, title, and photo
 */
-->

<script lang="ts">
	import { IconQuote, IconUser } from '$lib/icons';
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

	let quote = $derived(props.block.content.testimonialQuote || 'This is an amazing product that changed my life!');
	let authorName = $derived(props.block.content.testimonialAuthor || 'John Doe');
	let authorTitle = $derived(props.block.content.testimonialTitle || 'CEO, Company');
	let authorPhoto = $derived(props.block.content.testimonialPhoto || '');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<figure class="testimonial-block" role="figure" aria-label="Testimonial">
	<div class="quote-icon" aria-hidden="true">
		<IconQuote size={32} />
	</div>

	<blockquote class="testimonial-quote">
		{#if props.isEditing}
			<p
				contenteditable="true"
				oninput={(e) => updateContent({ testimonialQuote: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{quote}</p>
		{:else}
			<p>{quote}</p>
		{/if}
	</blockquote>

	<figcaption class="testimonial-author">
		<div class="author-photo">
			{#if authorPhoto}
				<img src={sanitizeURL(authorPhoto)} alt={authorName} />
			{:else}
				<div class="photo-placeholder">
					<IconUser size={24} aria-hidden="true" />
				</div>
			{/if}
		</div>
		<div class="author-info">
			{#if props.isEditing}
				<span
					contenteditable="true"
					class="author-name"
					oninput={(e) => updateContent({ testimonialAuthor: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}
				>{authorName}</span>
				<span
					contenteditable="true"
					class="author-title"
					oninput={(e) => updateContent({ testimonialTitle: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}
				>{authorTitle}</span>
			{:else}
				<span class="author-name">{authorName}</span>
				<span class="author-title">{authorTitle}</span>
			{/if}
		</div>
	</figcaption>

	{#if props.isEditing && props.isSelected}
		<div class="testimonial-settings">
			<label>
				<span>Author Photo URL:</span>
				<input
					type="url"
					placeholder="https://..."
					value={authorPhoto}
					oninput={(e) => updateContent({ testimonialPhoto: (e.target as HTMLInputElement).value })}
				/>
			</label>
		</div>
	{/if}
</figure>

<style>
	.testimonial-block {
		margin: 0; padding: 2rem;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 16px; text-align: center;
		position: relative;
	}

	.quote-icon {
		display: inline-flex;
		width: 56px; height: 56px;
		align-items: center; justify-content: center;
		background: #3b82f6; border-radius: 50%;
		color: white; margin-bottom: 1.5rem;
	}

	.testimonial-quote {
		margin: 0 0 1.5rem;
	}
	.testimonial-quote p {
		margin: 0;
		font-size: 1.25rem; font-style: italic; line-height: 1.7;
		color: #1e293b; outline: none;
	}

	.testimonial-author {
		display: flex; align-items: center; justify-content: center; gap: 1rem;
	}
	.author-photo {
		width: 56px; height: 56px;
		border-radius: 50%; overflow: hidden;
		background: #e2e8f0; flex-shrink: 0;
	}
	.author-photo img { width: 100%; height: 100%; object-fit: cover; }
	.photo-placeholder {
		width: 100%; height: 100%;
		display: flex; align-items: center; justify-content: center;
		color: #94a3b8;
	}

	.author-info { text-align: left; }
	.author-name {
		display: block; font-weight: 700; font-size: 1rem; color: #0f172a;
		outline: none;
	}
	.author-title {
		display: block; font-size: 0.875rem; color: #64748b;
		outline: none;
	}

	.testimonial-settings {
		margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid #e2e8f0;
	}
	.testimonial-settings label {
		display: flex; flex-direction: column; gap: 0.375rem;
		font-size: 0.8125rem; color: #64748b; text-align: left;
	}
	.testimonial-settings input {
		padding: 0.5rem 0.75rem; border: 1px solid #d1d5db; border-radius: 6px; font-size: 0.875rem;
	}

	:global(.dark) .testimonial-block { background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%); }
	:global(.dark) .testimonial-quote p { color: #f1f5f9; }
	:global(.dark) .author-photo { background: #334155; }
	:global(.dark) .photo-placeholder { color: #64748b; }
	:global(.dark) .author-name { color: #f8fafc; }
	:global(.dark) .author-title { color: #94a3b8; }
	:global(.dark) .testimonial-settings { border-color: #334155; }
	:global(.dark) .testimonial-settings input { background: #0f172a; border-color: #475569; color: #e2e8f0; }
</style>
