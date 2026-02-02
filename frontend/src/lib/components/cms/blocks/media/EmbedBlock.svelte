<!--
/**
 * Embed Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Embed external content (YouTube, Vimeo, Twitter, etc.)
 */
-->

<script lang="ts">
	import { IconBrandYoutube, IconBrandVimeo, IconBrandTwitter, IconCode, IconLink } from '$lib/icons';
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

	let embedUrl = $derived(props.block.content.embedUrl || '');
	let embedType = $derived(props.block.content.embedType || 'custom');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function detectEmbedType(url: string): string {
		if (url.includes('youtube.com') || url.includes('youtu.be')) return 'youtube';
		if (url.includes('vimeo.com')) return 'vimeo';
		if (url.includes('twitter.com') || url.includes('x.com')) return 'twitter';
		if (url.includes('instagram.com')) return 'instagram';
		if (url.includes('tiktok.com')) return 'tiktok';
		if (url.includes('soundcloud.com')) return 'soundcloud';
		if (url.includes('spotify.com')) return 'spotify';
		return 'custom';
	}

	function getEmbedUrl(url: string, type: string): string {
		if (!url) return '';
		
		switch (type) {
			case 'youtube': {
				const videoId = url.match(/(?:youtu\.be\/|youtube\.com\/(?:embed\/|v\/|watch\?v=|watch\?.+&v=))([^&\n?#]+)/)?.[1];
				return videoId ? `https://www.youtube.com/embed/${videoId}` : '';
			}
			case 'vimeo': {
				const vimeoId = url.match(/vimeo\.com\/(\d+)/)?.[1];
				return vimeoId ? `https://player.vimeo.com/video/${vimeoId}` : '';
			}
			default:
				return url;
		}
	}

	function handleUrlChange(url: string): void {
		const type = detectEmbedType(url) as 'youtube' | 'vimeo' | 'twitter' | 'instagram' | 'tiktok' | 'soundcloud' | 'spotify' | 'custom';
		updateContent({ embedUrl: url, embedType: type });
	}

	function getTypeIcon(type: string) {
		switch (type) {
			case 'youtube': return IconBrandYoutube;
			case 'vimeo': return IconBrandVimeo;
			case 'twitter': return IconBrandTwitter;
			default: return IconCode;
		}
	}

	let TypeIcon = $derived(getTypeIcon(embedType));
	let processedUrl = $derived(getEmbedUrl(embedUrl, embedType));
</script>

<div class="embed-block" role="region" aria-label="Embedded content">
	{#if props.isEditing}
		<div class="embed-editor">
			<div class="embed-header">
				<TypeIcon size={20} aria-hidden="true" />
				<span class="embed-type">{embedType.charAt(0).toUpperCase() + embedType.slice(1)}</span>
			</div>
			<div class="embed-input-wrapper">
				<IconLink size={18} aria-hidden="true" />
				<input
					type="url"
					placeholder="Paste YouTube, Vimeo, or other embed URL..."
					value={embedUrl}
					oninput={(e) => handleUrlChange((e.target as HTMLInputElement).value)}
				/>
			</div>
			{#if processedUrl && (embedType === 'youtube' || embedType === 'vimeo')}
				<div class="embed-preview">
					<iframe
						src={processedUrl}
						title="Embed preview"
						frameborder="0"
						allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
						allowfullscreen
					></iframe>
				</div>
			{:else if embedUrl && !processedUrl}
				<div class="embed-error">
					<p>Could not process this URL. Make sure it's a valid embed link.</p>
				</div>
			{/if}
		</div>
	{:else if processedUrl && (embedType === 'youtube' || embedType === 'vimeo')}
		<div class="embed-container">
			<iframe
				src={processedUrl}
				title="Embedded content"
				frameborder="0"
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
				allowfullscreen
			></iframe>
		</div>
	{:else if embedUrl}
		<div class="embed-fallback">
			<a href={sanitizeURL(embedUrl)} target="_blank" rel="noopener noreferrer">
				<TypeIcon size={24} aria-hidden="true" />
				<span>View embedded content</span>
			</a>
		</div>
	{:else}
		<div class="embed-empty">
			<IconCode size={32} aria-hidden="true" />
			<p>No embed URL provided</p>
		</div>
	{/if}
</div>

<style>
	.embed-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}

	.embed-editor {
		padding: 1rem;
		background: #f8fafc;
	}

	.embed-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		color: #64748b;
	}

	.embed-type {
		font-size: 0.875rem;
		font-weight: 500;
	}

	.embed-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		color: #94a3b8;
	}

	.embed-input-wrapper input {
		flex: 1;
		border: none;
		font-size: 0.9375rem;
		outline: none;
	}

	.embed-input-wrapper:focus-within {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.embed-preview,
	.embed-container {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		margin-top: 1rem;
		background: #0f172a;
		border-radius: 8px;
		overflow: hidden;
	}

	.embed-container {
		margin-top: 0;
		border-radius: 0;
	}

	.embed-preview iframe,
	.embed-container iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.embed-error {
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: #fef2f2;
		border-radius: 8px;
		color: #dc2626;
		font-size: 0.875rem;
	}

	.embed-error p { margin: 0; }

	.embed-fallback {
		padding: 2rem;
		background: #f8fafc;
		text-align: center;
	}

	.embed-fallback a {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		border-radius: 8px;
		color: white;
		text-decoration: none;
		font-weight: 500;
		transition: background 0.15s;
	}

	.embed-fallback a:hover { background: #2563eb; }

	.embed-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 3rem;
		color: #94a3b8;
		text-align: center;
	}

	.embed-empty p { margin: 0; font-size: 0.9375rem; }

	:global(.dark) .embed-block { border-color: #334155; }
	:global(.dark) .embed-editor { background: #1e293b; }
	:global(.dark) .embed-header { color: #94a3b8; }
	:global(.dark) .embed-input-wrapper { background: #0f172a; border-color: #475569; }
	:global(.dark) .embed-input-wrapper input { background: transparent; color: #e2e8f0; }
	:global(.dark) .embed-error { background: #450a0a; color: #fca5a5; }
	:global(.dark) .embed-fallback { background: #1e293b; }
	:global(.dark) .embed-empty { color: #64748b; }
</style>
