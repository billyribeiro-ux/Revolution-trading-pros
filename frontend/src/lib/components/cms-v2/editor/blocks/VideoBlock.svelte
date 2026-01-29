<!--
	Video Block - Video embed
	═══════════════════════════════════════════════════════════════════════════════

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconBrandYoutube from '@tabler/icons-svelte/icons/brand-youtube';

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	let data = $derived(
		block.data as {
			assetId?: string;
			url?: string;
			embedUrl?: string;
			caption?: string;
		}
	);

	let videoUrl = $derived(data.embedUrl || data.url || '');
	let hasVideo = $derived(!!videoUrl || !!data.assetId);

	// Parse YouTube URL to embed format
	let embedSrc = $derived.by(() => {
		if (!videoUrl) return '';

		// YouTube
		const youtubeMatch = videoUrl.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&\s]+)/);
		if (youtubeMatch) {
			return `https://www.youtube.com/embed/${youtubeMatch[1]}`;
		}

		// Vimeo
		const vimeoMatch = videoUrl.match(/vimeo\.com\/(\d+)/);
		if (vimeoMatch) {
			return `https://player.vimeo.com/video/${vimeoMatch[1]}`;
		}

		// Already an embed URL
		if (videoUrl.includes('embed')) {
			return videoUrl;
		}

		return videoUrl;
	});

	function handleUrlChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, url: target.value });
	}

	function handleCaptionChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, caption: target.value });
	}
</script>

<div class="video-block">
	{#if hasVideo && embedSrc}
		<div class="video-wrapper">
			<iframe
				src={embedSrc}
				title={data.caption || 'Video'}
				frameborder="0"
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
				allowfullscreen
			></iframe>
		</div>
		{#if data.caption}
			<p class="video-caption">{data.caption}</p>
		{/if}
		{#if !readonly}
			<div class="video-controls">
				<input
					type="text"
					class="control-input"
					value={data.caption ?? ''}
					oninput={handleCaptionChange}
					placeholder="Add a caption..."
				/>
			</div>
		{/if}
	{:else}
		<div class="video-placeholder">
			<div class="placeholder-icon">
				<IconVideo size={32} />
			</div>
			<p class="placeholder-text">Add a video</p>
			{#if !readonly}
				<div class="placeholder-actions">
					<input
						type="text"
						class="url-input"
						placeholder="Paste YouTube or Vimeo URL..."
						oninput={handleUrlChange}
					/>
					<div class="supported-platforms">
						<IconBrandYoutube size={16} />
						<span>YouTube & Vimeo supported</span>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.video-block {
		width: 100%;
	}

	.video-wrapper {
		position: relative;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		height: 0;
		overflow: hidden;
		border-radius: 0.5rem;
		background: rgba(0, 0, 0, 0.3);
	}

	.video-wrapper iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border-radius: 0.5rem;
	}

	.video-caption {
		margin: 0.5rem 0 0;
		font-size: 0.8125rem;
		color: #94a3b8;
		text-align: center;
		font-style: italic;
	}

	.video-controls {
		margin-top: 0.75rem;
	}

	.control-input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #f1f5f9;
		font-size: 0.8125rem;
	}

	.control-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* Placeholder */
	.video-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 2.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 2px dashed rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		text-align: center;
	}

	.placeholder-icon {
		color: #475569;
	}

	.placeholder-text {
		margin: 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.placeholder-actions {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		max-width: 320px;
	}

	.url-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #f1f5f9;
		font-size: 0.8125rem;
		text-align: center;
	}

	.url-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.supported-platforms {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #64748b;
	}
</style>
